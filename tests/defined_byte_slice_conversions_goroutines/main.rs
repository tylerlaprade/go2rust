#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

/// A Go slice: an offset, a length and a capacity over a backing array that
/// every slice made from it shares. Every name below that a binding uses
/// starts with __, so no package-level variable of the program shadows it.
pub struct GoSlice<T> {
    backing: std::sync::Arc<std::cell::UnsafeCell<GoSliceStore<T>>>,
    off: usize,
    len: usize,
    cap: usize,
    /// Whether this is the nil slice. A slice variable's nil is the None in
    /// its cell; this one is for slices held bare, such as the elements of
    /// a [][]byte, whose zero value is nil too.
    nil: bool,
}

/// A slice's backing array: one the slices own, or the elements of a Go
/// array held in a cell, which a[lo:hi] slices in place. The cell is kept
/// alive by the slice, so the elements do not move while it is held.
#[allow(dead_code)]
pub enum GoSliceStore<T> {
    Owned(std::vec::Vec<T>),
    Array { ptr: *mut T, len: usize, owner: Box<dyn std::any::Any> },
}

unsafe impl<T: Send> Send for GoSlice<T> {}
unsafe impl<T: Send + Sync> Sync for GoSlice<T> {}

/// The prelude's Vec and vec! are Go slices in generated code.
pub type Vec<T> = GoSlice<T>;

impl<T> Clone for GoSlice<T> {
    /// Copies the header. The copy shares the elements, as a Go slice
    /// assignment does.
    fn clone(&self) -> Self {
        GoSlice { backing: self.backing.clone(), off: self.off, len: self.len, cap: self.cap, nil: self.nil }
    }
}

impl<T> GoSlice<T> {
    pub fn new() -> Self {
        Self::from(std::vec::Vec::new())
    }

    pub fn with_capacity(__cap: usize) -> Self {
        GoSlice { backing: std::sync::Arc::from(std::cell::UnsafeCell::new(GoSliceStore::Owned(std::vec::Vec::with_capacity(__cap)))), off: 0, len: 0, cap: __cap, nil: false }
    }

    /// The nil slice: no elements, no backing array of its own.
    pub fn nil() -> Self {
        GoSlice { backing: std::sync::Arc::from(std::cell::UnsafeCell::new(GoSliceStore::Owned(std::vec::Vec::new()))), off: 0, len: 0, cap: 0, nil: true }
    }

    /// Go's s == nil.
    pub fn is_nil(&self) -> bool {
        self.nil
    }

    fn backing_ref(&self) -> &[T] {
        match unsafe { &*self.backing.get() } {
            GoSliceStore::Owned(__elems) => __elems,
            GoSliceStore::Array { ptr, len, .. } => unsafe { std::slice::from_raw_parts(*ptr, *len) },
        }
    }

    #[allow(clippy::mut_from_ref)]
    fn backing_mut(&self) -> &mut [T] {
        match unsafe { &mut *self.backing.get() } {
            GoSliceStore::Owned(__elems) => __elems,
            GoSliceStore::Array { ptr, len, .. } => unsafe { std::slice::from_raw_parts_mut(*ptr, *len) },
        }
    }

    /// The backing Vec, which can grow; None for an array's elements, which
    /// hold every element up to the capacity already.
    #[allow(clippy::mut_from_ref)]
    fn backing_vec(&self) -> Option<&mut std::vec::Vec<T>> {
        match unsafe { &mut *self.backing.get() } {
            GoSliceStore::Owned(__elems) => Some(__elems),
            GoSliceStore::Array { .. } => None,
        }
    }

    pub fn as_slice(&self) -> &[T] {
        &self.backing_ref()[self.off..self.off + self.len]
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        let (__off, __len) = (self.off, self.len);
        &mut self.backing_mut()[__off..__off + __len]
    }

    /// Go's cap(s).
    pub fn capacity(&self) -> usize {
        self.cap
    }

    /// Whether two slices share their backing array.
    pub fn shares_backing_with(&self, __other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.backing, &__other.backing)
    }

    /// Go's s[lo:hi].
    pub fn reslice(&self, __lo: usize, __hi: usize) -> Self {
        if __lo > __hi || __hi > self.cap {
            panic!("runtime error: slice bounds out of range [{}:{}] with capacity {}", __lo, __hi, self.cap);
        }
        if self.off + __hi > self.backing_ref().len() {
            panic!("slice [{}:{}] reaches past the elements GoSlice has stored", __lo, __hi);
        }
        GoSlice { backing: self.backing.clone(), off: self.off + __lo, len: __hi - __lo, cap: self.cap - __lo, nil: self.nil }
    }

    /// Go's s[lo:hi:max].
    pub fn reslice3(&self, __lo: usize, __hi: usize, __max: usize) -> Self {
        if __max > self.cap || __hi > __max {
            panic!("runtime error: slice bounds out of range [{}:{}:{}] with capacity {}", __lo, __hi, __max, self.cap);
        }
        let mut __slice = self.reslice(__lo, __hi);
        __slice.cap = __max - __lo;
        __slice
    }

    /// Go's s = s[:n] when n is at most len(s).
    pub fn truncate(&mut self, __n: usize) {
        if __n < self.len {
            self.len = __n;
        }
    }

    /// Go's s = s[:0].
    pub fn clear(&mut self) {
        self.len = 0;
    }

    #[allow(unused_variables)]
    pub fn reserve(&mut self, __additional: usize) {}

    /// Keeps the elements the predicate accepts, in order, at the front.
    pub fn retain<F: FnMut(&T) -> bool>(&mut self, mut __keep: F) {
        let mut __kept = 0;
        {
            let __window = self.as_mut_slice();
            for __index in 0..__window.len() {
                if __keep(&__window[__index]) {
                    __window.swap(__kept, __index);
                    __kept += 1;
                }
            }
        }
        self.len = __kept;
    }

    /// Moves the slice's elements out when no other slice shares them.
    fn take_unique(&mut self) -> Option<std::vec::Vec<T>> {
        let (__off, __len) = (self.off, self.len);
        let __backing = std::sync::Arc::get_mut(&mut self.backing)?;
        let mut __elems = match __backing.get_mut() {
            GoSliceStore::Owned(__elems) => std::mem::take(__elems),
            GoSliceStore::Array { .. } => return None,
        };
        __elems.truncate(__off + __len);
        __elems.drain(..__off);
        Some(__elems)
    }

    fn fresh(__elems: std::vec::Vec<T>, __cap: usize) -> Self {
        let __len = __elems.len();
        GoSlice { backing: std::sync::Arc::from(std::cell::UnsafeCell::new(GoSliceStore::Owned(__elems))), off: 0, len: __len, cap: __cap.max(__len), nil: false }
    }

    /// The capacity Go's append gives a slice that must grow to hold
    /// __needed elements.
    fn grown_cap(&self, __needed: usize) -> usize {
        let __old = self.cap;
        let __doubled = __old + __old;
        if __needed > __doubled {
            return __needed;
        }
        if __old < 256 {
            return __doubled.max(1);
        }
        let mut __grown = __old;
        while __grown < __needed {
            __grown += (__grown + 3 * 256) / 4;
        }
        __grown
    }

    /// Stores a value at index len(s) of the backing array: the in-place
    /// half of Go's append. The backing Vec has room for the capacity, so
    /// pushing onto it never moves elements other slices are reading.
    fn store_at_end(&mut self, __value: T) {
        let __at = self.off + self.len;
        let __backing = self.backing_mut();
        if __at < __backing.len() {
            __backing[__at] = __value;
        } else {
            self.backing_vec().expect("an array's slice stores within the array").push(__value);
        }
        self.len += 1;
        self.nil = false;
    }

    /// append for elements that cannot be copied: when the slice is full and
    /// shares its backing array, the value goes at the end of that array.
    fn push_without_copy(&mut self, __value: T) {
        if self.len < self.cap {
            self.store_at_end(__value);
            return;
        }
        let __cap = self.grown_cap(self.len + 1);
        if let Some(mut __elems) = self.take_unique() {
            __elems.reserve(__cap.saturating_sub(__elems.len()));
            __elems.push(__value);
            *self = Self::fresh(__elems, __cap);
            return;
        }
        self.store_at_end(__value);
        self.cap = self.len;
    }
}

impl<T: 'static> GoSlice<T> {
    /// Go's a[lo:hi:max] of an array a held in a single-threaded cell: a
    /// slice over a's own elements, so a write through either one is seen
    /// through the other.
    pub fn from_array_rc_cell<const N: usize>(__cell: &std::rc::Rc<std::cell::RefCell<Option<[T; N]>>>, __lo: usize, __hi: usize, __max: usize) -> Self {
        let __ptr = match unsafe { &mut *__cell.as_ptr() } {
            Some(__array) => __array.as_mut_ptr(),
            None => panic!("runtime error: invalid memory address or nil pointer dereference"),
        };
        Self::over_array(__ptr, N, Box::new(__cell.clone()), __lo, __hi, __max)
    }

    /// Go's a[lo:hi:max] of an array a held in a goroutine-safe cell.
    pub fn from_array_arc_cell<const N: usize>(__cell: &std::sync::Arc<std::sync::Mutex<Option<[T; N]>>>, __lo: usize, __hi: usize, __max: usize) -> Self {
        let __ptr = match __cell.lock().unwrap_or_else(|__poisoned| __poisoned.into_inner()).as_mut() {
            Some(__array) => __array.as_mut_ptr(),
            None => panic!("runtime error: invalid memory address or nil pointer dereference"),
        };
        Self::over_array(__ptr, N, Box::new(__cell.clone()), __lo, __hi, __max)
    }

    fn over_array(__ptr: *mut T, __len: usize, __owner: Box<dyn std::any::Any>, __lo: usize, __hi: usize, __max: usize) -> Self {
        if __hi > __max || __max > __len {
            panic!("runtime error: slice bounds out of range [:{}] with length {}", __hi.max(__max), __len);
        }
        if __lo > __hi {
            panic!("runtime error: slice bounds out of range [{}:{}]", __lo, __hi);
        }
        GoSlice {
            backing: std::sync::Arc::from(std::cell::UnsafeCell::new(GoSliceStore::Array { ptr: __ptr, len: __len, owner: __owner })),
            off: __lo,
            len: __hi - __lo,
            cap: __max - __lo,
            nil: false,
        }
    }
}

impl<T: Clone + Default> GoSlice<T> {
    /// Go's s[lo:hi:max] for an element type with a zero value. Up to the
    /// capacity the backing array may not have stored the elements, after an
    /// append grew it; in Go they are zero, so the zeros are stored first.
    pub fn reslice3_zeroed(&self, __lo: usize, __hi: usize, __max: usize) -> Self {
        if __hi <= self.cap && __max <= self.cap {
            let __need = self.off + __hi;
            if let Some(__backing) = self.backing_vec() {
                if __need > __backing.len() {
                    __backing.resize(__need, T::default());
                }
            }
        }
        self.reslice3(__lo, __hi, __max)
    }
}

impl<T: Clone> GoSlice<T> {
    /// Go's s = append(s, v): in place while there is capacity, else into a
    /// new backing array.
    pub fn push(&mut self, __value: T) {
        if self.len < self.cap {
            self.store_at_end(__value);
            return;
        }
        let __cap = self.grown_cap(self.len + 1);
        let mut __elems = match self.take_unique() {
            Some(__elems) => __elems,
            None => self.as_slice().to_vec(),
        };
        __elems.reserve(__cap.saturating_sub(__elems.len()));
        __elems.push(__value);
        *self = Self::fresh(__elems, __cap);
    }

    /// Go's append(s, v), leaving s as it was.
    pub fn appended(&self, __value: T) -> Self {
        let mut __result = self.clone();
        __result.push(__value);
        __result
    }

    /// Go's s = append(s, xs...).
    pub fn extend_from_slice(&mut self, __values: &[T]) {
        if self.len + __values.len() <= self.cap {
            for __value in __values {
                self.store_at_end(__value.clone());
            }
            return;
        }
        let __cap = self.grown_cap(self.len + __values.len());
        let mut __elems = match self.take_unique() {
            Some(__elems) => __elems,
            None => self.as_slice().to_vec(),
        };
        __elems.reserve(__cap.saturating_sub(__elems.len()));
        __elems.extend_from_slice(__values);
        *self = Self::fresh(__elems, __cap);
    }

    /// Go's s = append(s, xs...) from values.
    pub fn extend<I: IntoIterator<Item = T>>(&mut self, __values: I) {
        let __values: std::vec::Vec<T> = __values.into_iter().collect();
        self.extend_from_slice(&__values);
    }

    /// Removes and returns the last element: Go's v := s[len(s)-1];
    /// s = s[:len(s)-1].
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        let __last = self.as_slice()[self.len - 1].clone();
        self.len -= 1;
        Some(__last)
    }

    pub fn insert(&mut self, __index: usize, __value: T) {
        if __index > self.len {
            panic!("insertion index (is {}) should be <= len (is {})", __index, self.len);
        }
        self.push(__value);
        self.as_mut_slice()[__index..].rotate_right(1);
    }

    pub fn remove(&mut self, __index: usize) -> T {
        let __removed = self.as_slice()[__index].clone();
        self.as_mut_slice()[__index..].rotate_left(1);
        self.len -= 1;
        __removed
    }

    pub fn split_off(&mut self, __at: usize) -> Self {
        let __tail = Self::from(self.as_slice()[__at..].to_vec());
        self.len = __at;
        __tail
    }

    pub fn drain<R: std::ops::RangeBounds<usize>>(&mut self, __range: R) -> std::vec::IntoIter<T> {
        let mut __elems = self.as_slice().to_vec();
        let __drained: std::vec::Vec<T> = __elems.drain(__range).collect();
        let __cap = self.cap;
        *self = Self::fresh(__elems, __cap);
        __drained.into_iter()
    }

    pub fn resize(&mut self, __new_len: usize, __value: T) {
        if __new_len <= self.len {
            self.len = __new_len;
        } else {
            let __more = std::vec::from_elem(__value, __new_len - self.len);
            self.extend_from_slice(&__more);
        }
    }

    pub fn resize_with<F: FnMut() -> T>(&mut self, __new_len: usize, mut __make: F) {
        if __new_len <= self.len {
            self.len = __new_len;
        } else {
            let __more: std::vec::Vec<T> = (self.len..__new_len).map(|_| __make()).collect();
            self.extend_from_slice(&__more);
        }
    }

    pub fn into_std_vec(self) -> std::vec::Vec<T> {
        self.as_slice().to_vec()
    }

    pub fn into_boxed_slice(self) -> Box<[T]> {
        self.as_slice().to_vec().into_boxed_slice()
    }
}

/// The values appended by append(s, xs...) to a slice of interface values:
/// always a slice built for the append, which nothing else holds.
pub trait GoSliceBoxedValues<T> {
    fn into_boxed_values(self) -> std::vec::Vec<T>;
}

impl<T> GoSliceBoxedValues<T> for std::vec::Vec<T> {
    fn into_boxed_values(self) -> std::vec::Vec<T> {
        self
    }
}

impl<T> GoSliceBoxedValues<T> for GoSlice<T> {
    fn into_boxed_values(mut self) -> std::vec::Vec<T> {
        match self.take_unique() {
            Some(__elems) => __elems,
            None => panic!("append of interface values held by another slice"),
        }
    }
}

/// Go's append for interface values, which have no Clone. When the slice
/// is full and shares its backing array, the new element goes at the end of
/// that array instead of into a copy.
pub trait GoSliceAppendBoxed<T> {
    fn push(&mut self, __value: T);
    fn extend<I: GoSliceBoxedValues<T>>(&mut self, __values: I);
}

impl GoSliceAppendBoxed<Box<dyn std::any::Any>> for GoSlice<Box<dyn std::any::Any>> {
    fn push(&mut self, __value: Box<dyn std::any::Any>) {
        self.push_without_copy(__value);
    }
    fn extend<I: GoSliceBoxedValues<Box<dyn std::any::Any>>>(&mut self, __values: I) {
        for __value in __values.into_boxed_values() {
            self.push_without_copy(__value);
        }
    }
}

impl GoSliceAppendBoxed<Box<dyn std::any::Any + Send + Sync>> for GoSlice<Box<dyn std::any::Any + Send + Sync>> {
    fn push(&mut self, __value: Box<dyn std::any::Any + Send + Sync>) {
        self.push_without_copy(__value);
    }
    fn extend<I: GoSliceBoxedValues<Box<dyn std::any::Any + Send + Sync>>>(&mut self, __values: I) {
        for __value in __values.into_boxed_values() {
            self.push_without_copy(__value);
        }
    }
}

/// Reads the header of a slice value, whichever form the value has.
pub trait GoSliceHeader<T> {
    fn go_slice_header(&self) -> Option<GoSlice<T>>;
}

impl<T> GoSliceHeader<T> for GoSlice<T> {
    fn go_slice_header(&self) -> Option<GoSlice<T>> {
        Some(self.clone())
    }
}

impl<T> GoSliceHeader<T> for Option<GoSlice<T>> {
    fn go_slice_header(&self) -> Option<GoSlice<T>> {
        self.clone()
    }
}

impl<T> GoSliceHeader<T> for std::rc::Rc<std::cell::RefCell<Option<GoSlice<T>>>> {
    fn go_slice_header(&self) -> Option<GoSlice<T>> {
        self.borrow().clone()
    }
}

impl<T> GoSliceHeader<T> for std::sync::Arc<std::sync::Mutex<Option<GoSlice<T>>>> {
    fn go_slice_header(&self) -> Option<GoSlice<T>> {
        self.lock().unwrap().clone()
    }
}

/// Copies a Rust slice into a new Go slice.
pub trait GoSliceFromSlice<T> {
    fn to_go_slice(&self) -> GoSlice<T>;
}

impl<T: Clone> GoSliceFromSlice<T> for [T] {
    fn to_go_slice(&self) -> GoSlice<T> {
        GoSlice::from(self.to_vec())
    }
}

impl<T> Default for GoSlice<T> {
    /// A slice's zero value is nil.
    fn default() -> Self {
        Self::nil()
    }
}

impl<T> From<std::vec::Vec<T>> for GoSlice<T> {
    fn from(__elems: std::vec::Vec<T>) -> Self {
        let __len = __elems.len();
        GoSlice { backing: std::sync::Arc::from(std::cell::UnsafeCell::new(GoSliceStore::Owned(__elems))), off: 0, len: __len, cap: __len, nil: false }
    }
}

impl<T: Clone> From<&[T]> for GoSlice<T> {
    fn from(__elems: &[T]) -> Self {
        Self::from(__elems.to_vec())
    }
}

impl<T: Clone> From<&mut [T]> for GoSlice<T> {
    fn from(__elems: &mut [T]) -> Self {
        Self::from(__elems.to_vec())
    }
}

impl<T, const N: usize> From<[T; N]> for GoSlice<T> {
    fn from(__elems: [T; N]) -> Self {
        Self::from(std::vec::Vec::from(__elems))
    }
}

impl From<&str> for GoSlice<u8> {
    fn from(__text: &str) -> Self {
        Self::from(__text.as_bytes().to_vec())
    }
}

impl From<String> for GoSlice<u8> {
    fn from(__text: String) -> Self {
        Self::from(__text.into_bytes())
    }
}

impl<T: Clone> From<GoSlice<T>> for std::vec::Vec<T> {
    fn from(__slice: GoSlice<T>) -> Self {
        __slice.as_slice().to_vec()
    }
}

impl<T> std::ops::Deref for GoSlice<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> std::ops::DerefMut for GoSlice<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T> AsRef<[T]> for GoSlice<T> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> AsMut<[T]> for GoSlice<T> {
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T> std::borrow::Borrow<[T]> for GoSlice<T> {
    fn borrow(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> std::borrow::BorrowMut<[T]> for GoSlice<T> {
    fn borrow_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T> std::iter::FromIterator<T> for GoSlice<T> {
    fn from_iter<I: IntoIterator<Item = T>>(__values: I) -> Self {
        Self::from(__values.into_iter().collect::<std::vec::Vec<T>>())
    }
}

impl<T: Clone> IntoIterator for GoSlice<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    fn into_iter(mut self) -> Self::IntoIter {
        match self.take_unique() {
            Some(__elems) => __elems.into_iter(),
            None => self.as_slice().to_vec().into_iter(),
        }
    }
}

impl<'a, T> IntoIterator for &'a GoSlice<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}

impl<'a, T> IntoIterator for &'a mut GoSlice<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.as_mut_slice().iter_mut()
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for GoSlice<T> {
    fn fmt(&self, __f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_slice().fmt(__f)
    }
}

/// Go's %v of a slice: the elements' %v between brackets.
impl<T: std::fmt::Display> std::fmt::Display for GoSlice<T> {
    fn fmt(&self, __f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        __f.write_str("[")?;
        for (__index, __value) in self.as_slice().iter().enumerate() {
            if __index > 0 {
                __f.write_str(" ")?;
            }
            write!(__f, "{}", __value)?;
        }
        __f.write_str("]")
    }
}

impl<T: PartialEq> PartialEq for GoSlice<T> {
    fn eq(&self, __other: &Self) -> bool {
        self.as_slice() == __other.as_slice()
    }
}

impl<T: PartialEq> PartialEq<[T]> for GoSlice<T> {
    fn eq(&self, __other: &[T]) -> bool {
        self.as_slice() == __other
    }
}

impl<T: PartialEq> PartialEq<std::vec::Vec<T>> for GoSlice<T> {
    fn eq(&self, __other: &std::vec::Vec<T>) -> bool {
        self.as_slice() == __other.as_slice()
    }
}

impl<T: Eq> Eq for GoSlice<T> {}

impl<T: PartialOrd> PartialOrd for GoSlice<T> {
    fn partial_cmp(&self, __other: &Self) -> Option<std::cmp::Ordering> {
        self.as_slice().partial_cmp(__other.as_slice())
    }
}

impl<T: Ord> Ord for GoSlice<T> {
    fn cmp(&self, __other: &Self) -> std::cmp::Ordering {
        self.as_slice().cmp(__other.as_slice())
    }
}

impl<T: std::hash::Hash> std::hash::Hash for GoSlice<T> {
    fn hash<H: std::hash::Hasher>(&self, __state: &mut H) {
        self.as_slice().hash(__state)
    }
}

impl std::io::Write for GoSlice<u8> {
    fn write(&mut self, __bytes: &[u8]) -> std::io::Result<usize> {
        self.extend_from_slice(__bytes);
        Ok(__bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn format_slice<T, C>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[T]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| go_display_value(v)).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn format_slice_values<T>(slice: &[T]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| go_display_value(v)).collect();
    format!("[{}]", formatted.join(" "))
}

fn format_slice_wrapped<T, C>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[Arc<Mutex<Option<T>>>]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| {
            let inner = v.lock().unwrap();
            match inner.as_ref() {
                Some(value) => format!("&{}", value),
                None => "<nil>".to_string(),
            }
        }).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

thread_local! {
    static __GO_RECOVER_PAYLOAD: RefCell<Option<Box<dyn Any + Send + Sync>>> = RefCell::new(None);
}

fn go_recover() -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
    __GO_RECOVER_PAYLOAD.with(|slot| Arc::new(Mutex::new(slot.borrow_mut().take())))
}

fn go_store_panic_payload(payload: Box<dyn Any + Send>) {
    let payload = match payload.downcast::<Box<dyn Any + Send + Sync>>() {
        Ok(boxed) => {
            let mut payload = *boxed;
            loop {
                match payload.downcast::<Box<dyn Any + Send + Sync>>() {
                    Ok(boxed) => {
                        payload = *boxed;
                    }
                    Err(payload) => {
                        __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(payload));
                        return;
                    }
                }
            }
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<String>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(go_runtime_error_text(*value)) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<&'static str>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(go_runtime_error_text(value.to_string())) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<i32>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(*value) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<i64>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(*value) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(payload) => payload,
    };
    let _payload = match payload.downcast::<bool>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(*value) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(_payload) => _payload,
    };
    panic!("recover: unsupported Rust panic payload; emit panic_any with a Go any payload instead")
}

fn go_runtime_error_text(__text: String) -> String {
    if __text == "attempt to divide by zero" || __text == "attempt to calculate the remainder with a divisor of zero" {
        return "runtime error: integer divide by zero".to_string();
    }
    if let Some(__rest) = __text.strip_prefix("index out of bounds: the len is ") {
        if let Some((__len, __index)) = __rest.split_once(" but the index is ") {
            return format!("runtime error: index out of range [{}] with length {}", __index, __len);
        }
    }
    // A nil pointer's cell is empty, so reading through it unwraps None.
    if __text == "called `Option::unwrap()` on a `None` value" || __text == "nil pointer dereference" || __text == "invalid memory address or nil pointer dereference" {
        return "runtime error: invalid memory address or nil pointer dereference".to_string();
    }
    __text
}

thread_local! {
    static __GO_PANIC_HOOK_SILENCE: std::cell::Cell<usize> = std::cell::Cell::new(0);
}

fn go_silence_panic_hook() -> usize {
    static __GO_PANIC_HOOK: std::sync::Once = std::sync::Once::new();
    __GO_PANIC_HOOK.call_once(|| {
        let __previous = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |__info| {
            if __GO_PANIC_HOOK_SILENCE.with(|__depth| __depth.get()) == 0 {
                __previous(__info);
            }
        }));
    });
    __GO_PANIC_HOOK_SILENCE.with(|__depth| {
        let __previous = __depth.get();
        __depth.set(__previous + 1);
        __previous
    })
}

fn go_restore_panic_hook(__previous: usize) {
    __GO_PANIC_HOOK_SILENCE.with(|__depth| __depth.set(__previous));
}

fn go_resume_unrecovered_panic() {
    if let Some(payload) = __GO_RECOVER_PAYLOAD.with(|slot| slot.borrow_mut().take()) {
        std::panic::panic_any(payload);
    }
}

struct WaitGroup {
    count: std::sync::Arc<(std::sync::Mutex<i64>, std::sync::Condvar)>,
}

impl WaitGroup {
    fn new() -> Self {
        WaitGroup {
            count: std::sync::Arc::new((std::sync::Mutex::new(0), std::sync::Condvar::new())),
        }
    }

    fn add(&self, n: i64) {
        let (lock, _) = &*self.count;
        let mut count = lock.lock().unwrap();
        *count += n;
    }

    fn done(&self) {
        let (lock, cvar) = &*self.count;
        let mut count = lock.lock().unwrap();
        *count -= 1;
        if *count <= 0 {
            cvar.notify_all();
        }
    }

    fn wait(&self) {
        let (lock, cvar) = &*self.count;
        let mut count = lock.lock().unwrap();
        while *count > 0 {
            count = cvar.wait(count).unwrap();
        }
    }
}

impl Clone for WaitGroup {
    fn clone(&self) -> Self {
        WaitGroup {
            count: self.count.clone(),
        }
    }
}

impl Default for WaitGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for WaitGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WaitGroup")
    }
}

fn go_float_string(__value: f64, __single: bool) -> String {
    // The names are prefixed: this helper is at the crate root, where a Go
    // package variable of the same name would be a static it cannot shadow.
    if __value.is_nan() {
        return "NaN".to_string();
    }
    if __value.is_infinite() {
        return if __value < 0.0 { "-Inf".to_string() } else { "+Inf".to_string() };
    }
    let __sign = if __value.is_sign_negative() { "-" } else { "" };
    let __shortest = if __single { format!("{:e}", __value.abs() as f32) } else { format!("{:e}", __value.abs()) };
    let (__mantissa, __exponent_text) = __shortest.split_once('e').unwrap();
    let __digits: String = __mantissa.chars().filter(|__c| __c.is_ascii_digit()).collect();
    let __exponent: i32 = __exponent_text.parse().unwrap();
    if __exponent < -4 || __exponent >= 6 {
        let mut __text = String::from(__sign);
        __text.push_str(&__digits[..1]);
        if __digits.len() > 1 {
            __text.push('.');
            __text.push_str(&__digits[1..]);
        }
        __text.push('e');
        __text.push(if __exponent < 0 { '-' } else { '+' });
        __text.push_str(&format!("{:02}", __exponent.abs()));
        return __text;
    }
    if __exponent < 0 {
        return format!("{}0.{}{}", __sign, "0".repeat((-__exponent - 1) as usize), __digits);
    }
    let __whole = (__exponent + 1) as usize;
    if __digits.len() <= __whole {
        return format!("{}{}{}", __sign, __digits, "0".repeat(__whole - __digits.len()));
    }
    format!("{}{}.{}", __sign, &__digits[..__whole], &__digits[__whole..])
}

// A value inside a slice or a map prints as fmt prints it: a float through
// go_float_string, anything else through its Display. The element type may
// borrow, so it is recognized by name rather than by TypeId.
fn go_display_value<T: Display>(__value: &T) -> String {
    let __name = std::any::type_name::<T>();
    if __name == "f64" {
        // T is f64.
        return go_float_string(unsafe { *(__value as *const T as *const f64) }, false);
    }
    if __name == "f32" {
        // T is f32.
        return go_float_string(unsafe { *(__value as *const T as *const f32) } as f64, true);
    }
    __value.to_string()
}

#[derive(Debug, Clone, Default)]
pub struct Bytes(pub Arc<Mutex<Option<Vec<u8>>>>);

impl Display for Bytes {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice(&self.0))
    }
}


#[derive(Debug, Clone, Default)]
pub struct Runes(pub Arc<Mutex<Option<Vec<i32>>>>);

impl Display for Runes {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice(&self.0))
    }
}


#[derive(Debug)]
pub struct Text(pub Arc<Mutex<Option<String>>>);

impl Clone for Text {
    fn clone(&self) -> Self {
        let __value = (*self.0.lock().unwrap()).clone();
        Text(Arc::new(Mutex::new(__value)))
    }
}

impl Default for Text {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(Some(Default::default()))))
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for Text {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialOrd for Text {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ord::cmp(self, other))
    }
}

impl Eq for Text {}

impl Ord for Text {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}

impl PartialEq<String> for Text {
    fn eq(&self, other: &String) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialEq<&str> for Text {
    fn eq(&self, other: &&str) -> bool {
        self.0.lock().unwrap().as_ref().unwrap().as_str() == *other
    }
}

impl std::ops::Add for Text {
    type Output = Text;
    fn add(self, other: Self) -> Text {
        let __joined = format!("{}{}", self.0.lock().unwrap().as_ref().unwrap(), other.0.lock().unwrap().as_ref().unwrap());
        Text(Arc::new(Mutex::new(Some(__joined))))
    }
}

impl std::ops::Add<String> for Text {
    type Output = Text;
    fn add(self, other: String) -> Text {
        let __joined = format!("{}{}", self.0.lock().unwrap().as_ref().unwrap(), other);
        Text(Arc::new(Mutex::new(Some(__joined))))
    }
}

impl std::ops::Add<&str> for Text {
    type Output = Text;
    fn add(self, other: &str) -> Text {
        let __joined = format!("{}{}", self.0.lock().unwrap().as_ref().unwrap(), other);
        Text(Arc::new(Mutex::new(Some(__joined))))
    }
}


#[derive(Debug, Clone)]
pub struct Msg {
    pub body: Arc<Mutex<Option<Bytes>>>,
}

impl Msg {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.body.clone();
        Self {
            body: __go_clone_0_0,
        }
    }
}


impl Default for Msg {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Default::default())));
        Self {
            body: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Msg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.body.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Bytes {
    pub fn upper(&self) -> Arc<Mutex<Option<Bytes>>> {
        let mut out = Arc::new(Mutex::new(Some(Bytes(Arc::new(Mutex::new(Some(vec![0; ({ let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as usize])))))));
        { let __range_holder = self.0.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for (i, mut c) in __range_values.iter().copied().enumerate() {
        if { let __tmp_x = c; let __tmp_y = ('a' as u8) as u8; __tmp_x >= __tmp_y } && { let __tmp_x = c; let __tmp_y = ('z' as u8) as u8; __tmp_x <= __tmp_y } {
        { let __rhs = 32 as u8; c = c - __rhs; };
    }
        { let __idx = (i) as usize; let __rhs = c; (*{ let __named_slice = (*out.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    } }
        return out.clone();
    }
}

pub fn size(b: Arc<Mutex<Option<Bytes>>>) -> i64 {
    return { let __slice_holder = { let __named_slice = (*b.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64;
}

fn main() {
    let mut b = Arc::new(Mutex::new(Some(Bytes(Arc::new(Mutex::new(Some(("xyz".to_string()).as_bytes().to_go_slice())))))));
    let mut c: Arc<Mutex<Option<Bytes>>> = Arc::new(Mutex::new(Some(Bytes(Arc::new(Mutex::new(Some(("abc".to_string()).as_bytes().to_go_slice())))))));
    let mut d = Arc::new(Mutex::new(Some(Bytes(Arc::new(Mutex::new(Some(vec![('h' as u8), ('i' as u8)])))))));
    println!("{} {} {} {}", format!("{}", (*Arc::new(Mutex::new(Some(String::from_utf8(std::vec::Vec::from({ let __bytes = { let __named_slice = (*b.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __bytes_guard = __bytes.lock().unwrap(); let __bytes_value = __bytes_guard.as_ref().cloned().unwrap_or_default(); __bytes_value })).unwrap()))).lock().unwrap().as_ref().unwrap())), format!("{}", (*Arc::new(Mutex::new(Some(String::from_utf8(std::vec::Vec::from({ let __bytes = { let __named_slice = (*c.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __bytes_guard = __bytes.lock().unwrap(); let __bytes_value = __bytes_guard.as_ref().cloned().unwrap_or_default(); __bytes_value })).unwrap()))).lock().unwrap().as_ref().unwrap())), format!("{}", (*Arc::new(Mutex::new(Some(String::from_utf8(std::vec::Vec::from({ let __bytes = { let __named_slice = (*d.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __bytes_guard = __bytes.lock().unwrap(); let __bytes_value = __bytes_guard.as_ref().cloned().unwrap_or_default(); __bytes_value })).unwrap()))).lock().unwrap().as_ref().unwrap())), format!("{}", { let __slice_holder = { let __named_slice = (*b.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }));
    println!("{} {}", format!("{}", (*Arc::new(Mutex::new(Some(String::from_utf8(std::vec::Vec::from({ let __bytes = { let __named_slice = (*(*c.lock().unwrap().as_ref().unwrap()).upper().lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __bytes_guard = __bytes.lock().unwrap(); let __bytes_value = __bytes_guard.as_ref().cloned().unwrap_or_default(); __bytes_value })).unwrap()))).lock().unwrap().as_ref().unwrap())), format!("{}", (*Arc::new(Mutex::new(Some(String::from_utf8(std::vec::Vec::from({ let __bytes = { let __named_slice = Bytes(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*b.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__slice_guard); let __low = (1) as usize; let __high = __seq.len(); let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) })))); __named_slice.0.clone() }; let __bytes_guard = __bytes.lock().unwrap(); let __bytes_value = __bytes_guard.as_ref().cloned().unwrap_or_default(); __bytes_value })).unwrap()))).lock().unwrap().as_ref().unwrap())));
    print!("{} {}\n", format!("{}", String::from_utf8(std::vec::Vec::from({ let __bytes = { let __named_slice = (*b.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __bytes_guard = __bytes.lock().unwrap(); let __bytes_value = __bytes_guard.as_ref().cloned().unwrap_or_default(); __bytes_value })).unwrap()), format!("{}", format_slice(&{ let __named_slice = (*c.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice })));
    let mut r = Arc::new(Mutex::new(Some(Runes(Arc::new(Mutex::new(Some(("h\u{e9}llo".to_string()).chars().map(|c| c as i32).collect::<Vec<_>>())))))));
    println!("{} {}", format!("{}", { let __slice_holder = { let __named_slice = (*r.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }), format!("{}", (*Arc::new(Mutex::new(Some({ let __rune_slice_holder = { let __named_slice = (*r.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __rune_slice_guard = __rune_slice_holder.lock().unwrap(); match __rune_slice_guard.as_ref() { Some(__runes) => (*__runes).iter().map(|&c| char::from_u32(c as u32).unwrap_or('\u{FFFD}')).collect::<String>(), None => String::new() } }))).lock().unwrap().as_ref().unwrap())));
    { let new_val = Arc::new(Mutex::new(Some(Bytes(Arc::new(Mutex::new(Some(("reassigned".to_string()).as_bytes().to_go_slice()))))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *c.lock().unwrap() = __moved_val; };
    println!("{} {}", format!("{}", (*Arc::new(Mutex::new(Some(String::from_utf8(std::vec::Vec::from({ let __bytes = { let __named_slice = (*c.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __bytes_guard = __bytes.lock().unwrap(); let __bytes_value = __bytes_guard.as_ref().cloned().unwrap_or_default(); __bytes_value })).unwrap()))).lock().unwrap().as_ref().unwrap())), format!("{}", size(Arc::new(Mutex::new(Some(Bytes(Arc::new(Mutex::new(Some(("four".to_string()).as_bytes().to_go_slice()))))))))));
    let mut m = Arc::new(Mutex::new(Some(Msg { body: Arc::new(Mutex::new(Some(Bytes(Arc::new(Mutex::new(Some(("body".to_string()).as_bytes().to_go_slice()))).clone())))), ..Default::default() })));
    println!("{}", format!("{}", (*Arc::new(Mutex::new(Some(String::from_utf8(std::vec::Vec::from({ let __bytes = { let __named_slice = (*(*m.lock().unwrap().as_ref().unwrap()).body.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __bytes_guard = __bytes.lock().unwrap(); let __bytes_value = __bytes_guard.as_ref().cloned().unwrap_or_default(); __bytes_value })).unwrap()))).lock().unwrap().as_ref().unwrap())));

        // u shares raw's array until the append, which needs a new one.
    let mut raw = Arc::new(Mutex::new(Some(vec![0u8; (3) as usize])));
    {
        let _src = "raw".to_string().as_bytes().to_vec();
        let _n = std::cmp::min((*raw.lock().unwrap().as_ref().unwrap()).len(), _src.len());
        for _i in 0.._n {
            (*raw.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone();
        }
        Arc::new(Mutex::new(Some(_n as i64)))
    };
    let mut u: Arc<Mutex<Option<Bytes>>> = Arc::new(Mutex::new(Some(Bytes(Arc::new(Mutex::new(raw.lock().unwrap().clone()))))));
    { let __idx = (1) as usize; let __rhs = ('A' as u8) as u8; (*{ let __named_slice = (*u.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    { let new_val = { let __base = { let __named_slice = (*u.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __base_guard = __base.lock().unwrap(); let mut __values = __base_guard.as_ref().cloned().unwrap_or_else(Vec::new); std::mem::drop(__base_guard); __values.push(('!' as u8)); Arc::new(Mutex::new(Some(Bytes(Arc::new(Mutex::new(Some(__values))))))) }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *u.lock().unwrap() = __moved_val; };
    { let __idx = (0) as usize; let __rhs = ('R' as u8) as u8; (*{ let __named_slice = (*u.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {}", format!("{}", (*Arc::new(Mutex::new(Some(String::from_utf8(std::vec::Vec::from((*raw.lock().unwrap().as_ref().unwrap()).clone())).unwrap()))).lock().unwrap().as_ref().unwrap())), format!("{}", (*Arc::new(Mutex::new(Some(String::from_utf8(std::vec::Vec::from({ let __bytes = { let __named_slice = (*u.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __bytes_guard = __bytes.lock().unwrap(); let __bytes_value = __bytes_guard.as_ref().cloned().unwrap_or_default(); __bytes_value })).unwrap()))).lock().unwrap().as_ref().unwrap())));

        // A defined string, a nil []byte, an assigned variable and a [][]byte
        // row go into the defined type; a nil or invalid []rune converts.
    let mut name: Arc<Mutex<Option<Text>>> = Arc::new(Mutex::new(Some(Text(Arc::new(Mutex::new(Some("hey".to_string())))))));
    println!("{} {}", format!("{}", (*Arc::new(Mutex::new(Some(String::from_utf8(std::vec::Vec::from({ let __bytes = { let __named_slice = Bytes(Arc::new(Mutex::new(Some((*(*name.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()).clone().as_bytes().to_go_slice())))); __named_slice.0.clone() }; let __bytes_guard = __bytes.lock().unwrap(); let __bytes_value = __bytes_guard.as_ref().cloned().unwrap_or_default(); __bytes_value })).unwrap()))).lock().unwrap().as_ref().unwrap())), format!("{}", { let __slice_holder = { let __named_slice = Runes(Arc::new(Mutex::new(Some((*(*name.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()).clone().chars().map(|c| c as i32).collect::<Vec<_>>())))); __named_slice.0.clone() }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }));
    let mut none: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));
    let mut empty: Arc<Mutex<Option<Bytes>>> = Arc::new(Mutex::new(Some(Bytes(Arc::new(Mutex::new(none.lock().unwrap().clone()))))));
    println!("{} {} {}", format!("{}", { let __slice_holder = { let __named_slice = empty.lock().unwrap().as_ref().map(|__v| __v.0.clone()).unwrap_or_else(|| Arc::new(Mutex::new(None))); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); (*__slice_guard).is_none() }), format!("{}", { let __slice_holder = { let __named_slice = (*empty.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }), format!("{}", { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __rune_slice_holder = { let __named_slice = Runes(Arc::new(Mutex::new(None::<Vec<i32>>))); __named_slice.0.clone() }; let __rune_slice_guard = __rune_slice_holder.lock().unwrap(); match __rune_slice_guard.as_ref() { Some(__runes) => (*__runes).iter().map(|&c| char::from_u32(c as u32).unwrap_or('\u{FFFD}')).collect::<String>(), None => String::new() } }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y }));
    let mut one = Arc::new(Mutex::new(Some(vec![1 as u8])));
    { let new_val = Arc::new(Mutex::new(Some(Bytes(Arc::new(Mutex::new(one.lock().unwrap().clone())))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *u.lock().unwrap() = __moved_val; };
    { let __idx = (0) as usize; let __rhs = 2 as u8; (*{ let __named_slice = (*u.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    let mut rows = Arc::new(Mutex::new(Some(vec![vec![7 as u8]])));
    let mut row: Arc<Mutex<Option<Bytes>>> = Arc::new(Mutex::new(Some(Bytes(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = rows.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone())))))));
    { let __idx = (0) as usize; let __rhs = 8 as u8; (*{ let __named_slice = (*row.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    let mut bad = Arc::new(Mutex::new(Some(Runes(Arc::new(Mutex::new(Some(("x".to_string()).chars().map(|c| c as i32).collect::<Vec<_>>())))))));
    { let __idx = (0) as usize; let __rhs = -1 as i32; (*{ let __named_slice = (*bad.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {} {}", format!("{}", { let __seq = { let __seq_holder = one.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }), format!("{}", ({ let __seq = { let __seq_holder = rows.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() })[(0) as usize].clone()), format!("{}", { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __rune_slice_holder = { let __named_slice = (*bad.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __rune_slice_guard = __rune_slice_holder.lock().unwrap(); match __rune_slice_guard.as_ref() { Some(__runes) => (*__runes).iter().map(|&c| char::from_u32(c as u32).unwrap_or('\u{FFFD}')).collect::<String>(), None => String::new() } }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "\u{fffd}".to_string(); __tmp_x == __tmp_y }));
    let mut wg = WaitGroup::new();
    let mut out: Arc<Mutex<Option<Vec<Bytes>>>> = Arc::new(Mutex::new(Some(vec![Default::default(); (2) as usize])));
    for i in 0..(({ let __range_holder = out.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        wg.add(1);
        let out_thread = out.clone(); let wg_thread = wg.clone(); let __go_arg_0 = Arc::new(Mutex::new(Some(i as i64))); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();
        let mut __closure = move |i: Arc<Mutex<Option<i64>>>| {
            let __go_previous_panic_hook = go_silence_panic_hook();
let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
let wg_defer_captured = wg_thread.clone(); __defer_stack.push(Box::new(move || {
        wg_defer_captured.done();
    }));;
            { let __idx = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __rhs = (*Bytes(Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(format!("{}{}", format!("{}", "g".to_string()), format!("{}", { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }))))).lock().unwrap().as_ref().unwrap()).as_bytes().to_go_slice())))).upper().lock().unwrap().as_ref().unwrap()).clone(); (*out_thread.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };;
            while let Some(f) = __defer_stack.pop() {
                f();
            }
            }));
            match __go_panic_result {
                Ok(__go_value) => {
                    go_restore_panic_hook(__go_previous_panic_hook);
                    __go_value
                }
                Err(__go_panic_payload) => {
                    go_store_panic_payload(__go_panic_payload);
                    while let Some(f) = __defer_stack.pop() {
                        if let Err(__go_defer_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
                            go_store_panic_payload(__go_defer_panic);
                        }
                    }
                    go_restore_panic_hook(__go_previous_panic_hook);
                    go_resume_unrecovered_panic();
                    ()
                }
            }

        };
        __closure(__go_arg_0);
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    }
    wg.wait();
    println!("{} {}", format!("{}", (*Arc::new(Mutex::new(Some(String::from_utf8(std::vec::Vec::from({ let __bytes = { let __named_slice = { let __seq = { let __seq_holder = out.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; __named_slice.0.clone() }; let __bytes_guard = __bytes.lock().unwrap(); let __bytes_value = __bytes_guard.as_ref().cloned().unwrap_or_default(); __bytes_value })).unwrap()))).lock().unwrap().as_ref().unwrap())), format!("{}", (*Arc::new(Mutex::new(Some(String::from_utf8(std::vec::Vec::from({ let __bytes = { let __named_slice = { let __seq = { let __seq_holder = out.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }; __named_slice.0.clone() }; let __bytes_guard = __bytes.lock().unwrap(); let __bytes_value = __bytes_guard.as_ref().cloned().unwrap_or_default(); __bytes_value })).unwrap()))).lock().unwrap().as_ref().unwrap())));
}