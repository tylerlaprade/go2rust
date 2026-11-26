#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use std::any::Any;
use std::collections::BTreeMap;
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

fn go_install_panic_exit() {
    // Go lets a program call main again; the hook goes in once, so a
    // later call keeps the hook that go_silence_panic_hook wrapped around it.
    static __GO_PANIC_EXIT: std::sync::Once = std::sync::Once::new();
    __GO_PANIC_EXIT.call_once(|| {
        std::panic::set_hook(Box::new(|__info| {
            let __text = go_panic_payload_text(__info.payload());
            {
                use std::io::Write;
                std::io::stdout().flush().ok();
            }
            eprintln!("panic: {}", __text);
            std::process::exit(2);
        }));
    });
}

fn go_panic_payload_text(__payload: &dyn Any) -> String {
    if let Some(__text) = __payload.downcast_ref::<&'static str>() {
        return go_runtime_error_text(__text.to_string());
    }
    if let Some(__text) = __payload.downcast_ref::<String>() {
        return go_runtime_error_text_of_string(__text.clone());
    }
    go_panic_value_text(__payload)
}

fn go_panic_value_text(__payload: &dyn Any) -> String {
    // A boxed payload holds the Go value itself, so its string is printed
    // as it is.
    if let Some(__text) = __payload.downcast_ref::<String>() {
        return __text.clone();
    }
    if let Some(__text) = __payload.downcast_ref::<&'static str>() {
        return __text.to_string();
    }
    if let Some(__inner) = __payload.downcast_ref::<Box<dyn Any + Send + Sync>>() {
        let __text = go_panic_value_text(&**__inner);
        return __text;
    }
    if let Some(__inner) = __payload.downcast_ref::<Box<dyn Any + Send>>() {
        return go_panic_value_text(&**__inner);
    }
    if let Some(__inner) = __payload.downcast_ref::<Box<dyn Any>>() {
        return go_panic_value_text(&**__inner);
    }
    if let Some(__err) = __payload.downcast_ref::<std::sync::Arc<std::sync::Mutex<Option<std::sync::Arc<dyn std::error::Error + Send + Sync>>>>>() {
        return __err.lock().unwrap().as_ref().map(|__err| __err.to_string()).unwrap_or_else(|| String::from("nil"));
    }
    macro_rules! go_panic_display_text {
        ($($t:ty),*) => {
            $(if let Some(__value) = __payload.downcast_ref::<$t>() {
                return __value.to_string();
            })*
        };
    }
    go_panic_display_text!(i64, i32, i16, i8, u64, u32, u16, u8, usize, isize, bool, char);
    if let Some(__value) = __payload.downcast_ref::<f64>() {
        // Go prints a float in a panic as +1.500000e+000.
        if __value.is_nan() {
            return String::from("NaN");
        }
        if __value.is_infinite() {
            return String::from(if *__value > 0.0 { "+Inf" } else { "-Inf" });
        }
        let __text = format!("{:+.6e}", __value);
        if let Some((__mantissa, __exponent)) = __text.split_once('e') {
            let __exponent: i32 = __exponent.parse().unwrap_or(0);
            return format!("{}e{}{:03}", __mantissa, if __exponent < 0 { '-' } else { '+' }, __exponent.abs());
        }
        return __text;
    }
    String::from(GO_UNPRINTABLE_PANIC_VALUE)
}

const GO_UNPRINTABLE_PANIC_VALUE: &str = "(unprintable panic value)";

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

fn go_runtime_error_text_of_string(__text: String) -> String {
    // panic(s) in Go raises a String, and so does a runtime error whose text
    // Rust formats: an index out of range, or a nil pointer read through
    // expect(). The constant runtime errors arrive as a &'static str, so a
    // Go string that reads like one keeps its text.
    if __text.starts_with("index out of bounds: ") || __text == "nil pointer dereference" || __text == "invalid memory address or nil pointer dereference" {
        return go_runtime_error_text(__text);
    }
    __text
}

struct GoChannel<T> {
    tx: std::sync::Arc<std::sync::Mutex<Option<std::sync::mpsc::SyncSender<T>>>>,
    rx: std::sync::Arc<std::sync::Mutex<std::sync::mpsc::Receiver<T>>>,
    is_nil: std::sync::Arc<std::sync::atomic::AtomicBool>,
    len: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    capacity: usize,
}

impl<T> GoChannel<T> {
    fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel(0);
        GoChannel {
            tx: std::sync::Arc::new(std::sync::Mutex::new(Some(tx))),
            rx: std::sync::Arc::new(std::sync::Mutex::new(rx)),
            is_nil: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            len: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            capacity: 0,
        }
    }

    fn new_buffered(cap: usize) -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel(cap);
        GoChannel {
            tx: std::sync::Arc::new(std::sync::Mutex::new(Some(tx))),
            rx: std::sync::Arc::new(std::sync::Mutex::new(rx)),
            is_nil: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            len: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            capacity: cap,
        }
    }

    fn send(&self, val: T) {
        if self.is_nil() {
            return;
        }
        let tx = self.tx.lock().unwrap().clone();
        if let Some(tx) = tx {
            if tx.send(val).is_ok() && self.capacity > 0 {
                self.len.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        }
    }

    fn try_send(&self, val: T) -> bool {
        if self.is_nil() {
            return false;
        }
        let tx = self.tx.lock().unwrap().clone();
        if let Some(tx) = tx {
            if tx.try_send(val).is_ok() {
                if self.capacity > 0 {
                    self.len.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn recv(&self) -> Option<T> {
        if self.is_nil() {
            return None;
        }
        let value = self.rx.lock().unwrap().recv().ok();
        if value.is_some() && self.capacity > 0 {
            let _ = self.len.fetch_update(
                std::sync::atomic::Ordering::SeqCst,
                std::sync::atomic::Ordering::SeqCst,
                |__go_current| __go_current.checked_sub(1),
            );
        }
        value
    }

    fn try_recv(&self) -> Option<T> {
        if self.is_nil() {
            return None;
        }
        let value = self.rx.lock().unwrap().try_recv().ok();
        if value.is_some() && self.capacity > 0 {
            let _ = self.len.fetch_update(
                std::sync::atomic::Ordering::SeqCst,
                std::sync::atomic::Ordering::SeqCst,
                |__go_current| __go_current.checked_sub(1),
            );
        }
        value
    }

    // A select case's receive: Some(Some(v)) for a value, Some(None) when
    // the channel is closed and drained, None when nothing is ready yet.
    fn try_recv_select(&self) -> Option<Option<T>> {
        if self.is_nil() {
            return None;
        }
        match self.rx.lock().unwrap().try_recv() {
            Ok(value) => {
                if self.capacity > 0 {
                    let _ = self.len.fetch_update(
                        std::sync::atomic::Ordering::SeqCst,
                        std::sync::atomic::Ordering::SeqCst,
                        |__go_current| __go_current.checked_sub(1),
                    );
                }
                Some(Some(value))
            }
            Err(std::sync::mpsc::TryRecvError::Disconnected) => Some(None),
            Err(std::sync::mpsc::TryRecvError::Empty) => None,
        }
    }

    fn close(&self) {
        *self.tx.lock().unwrap() = None;
    }

    fn is_nil(&self) -> bool {
        self.is_nil.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn len(&self) -> usize {
        self.len.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn capacity(&self) -> usize {
        self.capacity
    }
}

impl<T> Clone for GoChannel<T> {
    fn clone(&self) -> Self {
        GoChannel {
            tx: self.tx.clone(),
            rx: self.rx.clone(),
            is_nil: self.is_nil.clone(),
            len: self.len.clone(),
            capacity: self.capacity,
        }
    }
}

impl<T> Default for GoChannel<T> {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel(0);
        GoChannel {
            tx: std::sync::Arc::new(std::sync::Mutex::new(Some(tx))),
            rx: std::sync::Arc::new(std::sync::Mutex::new(rx)),
            is_nil: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true)),
            len: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            capacity: 0,
        }
    }
}

impl<T> std::fmt::Debug for GoChannel<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GoChannel")
    }
}

impl<T> Iterator for GoChannel<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.recv()
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

#[derive(Debug, Clone)]
pub struct record {
    pub n: Arc<Mutex<Option<i64>>>,
    pub s: Arc<Mutex<Option<String>>>,
}

impl record {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.s.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            n: __go_clone_0_0,
            s: __go_clone_1_0,
        }
    }
}


impl Default for record {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            n: __go_default_0_0,
            s: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for record {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.n.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.s.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct r#box {
    pub n: Arc<Mutex<Option<i64>>>,
}

impl r#box {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            n: __go_clone_0_0,
        }
    }
}


impl Default for r#box {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            n: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for r#box {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.n.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


pub(crate) static counter: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static label: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<String>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static flag: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static text: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<String>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static spare: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<i64>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *counter.lock().unwrap() = Some(0);
    *label.lock().unwrap() = Some(String::new());
    *flag.lock().unwrap() = Some(false);
    *text.lock().unwrap() = Some(String::new());
    *spare.lock().unwrap() = Some(Vec::nil());
    *label.lock().unwrap() = Some("a".to_string());
    *text.lock().unwrap() = Some("a".to_string());
    *spare.lock().unwrap() = Some((*Arc::new(Mutex::new(Some({ let mut v: Vec<i64> = (0..(4) as usize).map(|_| 0i64).collect(); v.truncate((1) as usize); v }))).lock().unwrap().as_ref().unwrap()).clone());
}


impl record {
    pub fn twice(&self) -> i64 {
        return { let __tmp_x = (*self.n.lock().unwrap().as_ref().unwrap()); let __tmp_y = 2; __tmp_x * __tmp_y };
    }
}

impl r#box {
    pub fn inc(&mut self) -> i64 {
        { let __target = self.n.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return (*self.n.lock().unwrap().as_ref().unwrap());
    }

    pub fn total(&mut self) -> i64 {
        return { let __go_rhs_1 = Arc::new(Mutex::new(Some(self.inc()))); let __go_binary_value = { let __tmp_x = (*self.n.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*__go_rhs_1.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; __go_binary_value };
    }
}

pub fn bump_counter() -> i64 {
    { let __rhs = 5; let mut guard = counter.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    1
}

pub fn relabel() -> Arc<Mutex<Option<String>>> {
    { let new_val = "b".to_string(); *label.lock().unwrap() = Some(new_val); };
    Arc::new(Mutex::new(Some("c".to_string())))
}

pub fn new_record() -> Arc<Mutex<Option<record>>> {
    { let __rhs = 100; let mut guard = counter.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    { let new_val = "d".to_string(); *label.lock().unwrap() = Some(new_val); };
    Arc::new(Mutex::new(Some(record { n: Arc::new(Mutex::new(Some(3))), s: Arc::new(Mutex::new(Some("e".to_string()))), ..Default::default() })))
}

pub fn bump(s: Arc<Mutex<Option<Vec<i64>>>>) -> i64 {
    { let __idx = (0) as usize; let mut __seq_guard = s.lock().unwrap(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx] + 1; }
    1
}

pub fn fill(s: Arc<Mutex<Option<Vec<i64>>>>, v: Arc<Mutex<Option<i64>>>) {
    { let __idx = (1) as usize; let __rhs = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; (*s.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
}

pub fn tag(p: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    { (*p.lock().unwrap().as_mut().unwrap()).push_str(&"!".to_string()); };
    Arc::new(Mutex::new(Some("t".to_string())))
}

pub fn add_one(m: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>) -> i64 {
    { let __map_holder = m.clone(); let __key = "a".to_string(); let mut __map_guard = __map_holder.lock().unwrap(); let __map = __map_guard.as_mut().unwrap(); let __entry = __map.entry(__key).or_insert_with(|| Arc::new(Mutex::new(Some(0)))); let mut __value = __entry.lock().unwrap(); * __value = Some((*__value.as_ref().unwrap()).clone() + 1); }
    0
}

pub fn plain(i: Arc<Mutex<Option<i64>>>) -> i64 {
    return { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x * __tmp_y };
}

pub fn twice(s: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    return Arc::new(Mutex::new(Some(format!("{}{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }))));
}

pub fn raise() -> bool {
    { let new_val = true; *flag.lock().unwrap() = Some(new_val); };
    true
}

pub fn grow() -> i64 {
    { let new_val = "abcd".to_string(); *text.lock().unwrap() = Some(new_val); };
    0
}

pub fn regrow() -> i64 {
    { let new_val = { let __collection_holder = Arc::new(Mutex::new(Some({ let mut v: Vec<i64> = (0..(9) as usize).map(|_| 0i64).collect(); v.truncate((1) as usize); v }))).clone(); let __collection_guard = __collection_holder.lock().unwrap(); (*__collection_guard).clone() }; *spare.lock().unwrap() = new_val; };
    0
}

fn main() {
    go_install_panic_exit();
    __go_init_all();
        // A goroutine makes every variable a locked one.
    let mut done = GoChannel::<String>::new_buffered(1 as usize);
    let done_thread = done.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        done_thread.send("started".to_string());;;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    println!("{}", format!("{}", done.recv().unwrap_or_default()));

    let mut s = Arc::new(Mutex::new(Some(vec![1i64, 2i64])));
    let mut x = Arc::new(Mutex::new(Some({ let __go_rhs_2 = Arc::new(Mutex::new(Some(bump(s.clone())))); let __go_binary_value = {
        let __tmp_x = { let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() };
        let __tmp_y = { let __v = (*__go_rhs_2.lock().unwrap().as_ref().unwrap()).clone(); __v };
        __tmp_x + __tmp_y
    }; __go_binary_value })));
    println!("{} {} {}", format!("{}", "slice element:".to_string()), format!("{}", { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", format_slice(&s)));
    fill(
        s.clone(),
        Arc::new(Mutex::new(Some({ let __go_rhs_3 = Arc::new(Mutex::new(Some(bump(s.clone())))); let __go_binary_value = {
            let __tmp_x = { let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() };
            let __tmp_y = { let __v = (*__go_rhs_3.lock().unwrap().as_ref().unwrap()).clone(); __v };
            __tmp_x + __tmp_y
        }; __go_binary_value })))
    );
    println!("{} {}", format!("{}", "in an argument:".to_string()), format!("{}", format_slice(&s)));
    let mut y = Arc::new(Mutex::new(Some({
        let __tmp_x = { let __go_rhs_4 = Arc::new(Mutex::new(Some(bump(s.clone())))); let __go_binary_value = {
            let __tmp_x = {
                let __tmp_x = { let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() };
                let __tmp_y = 10;
                __tmp_x * __tmp_y
            };
            let __tmp_y = { let __v = (*__go_rhs_4.lock().unwrap().as_ref().unwrap()).clone(); __v };
            __tmp_x + __tmp_y
        }; __go_binary_value };
        let __tmp_y = { let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() };
        __tmp_x - __tmp_y
    })));
    println!("{} {} {}", format!("{}", "several operands:".to_string()), format!("{}", { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", format_slice(&s)));
    println!("{} {} {}", format!("{}", "comparison:".to_string()), format!("{}", { let __go_rhs_5 = Arc::new(Mutex::new(Some(bump(s.clone())))); let __go_binary_value = {
        let __tmp_x = { let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() };
        let __tmp_y = { let __v = (*__go_rhs_5.lock().unwrap().as_ref().unwrap()).clone(); __v };
        __tmp_x == __tmp_y
    }; __go_binary_value }), format!("{}", format_slice(&s)));

    let mut b = Arc::new(Mutex::new(Some(r#box { n: Arc::new(Mutex::new(Some(5))), ..Default::default() })));
    let mut z = Arc::new(Mutex::new(Some({ let __go_rhs_6 = Arc::new(Mutex::new(Some({ let __recv = b.clone(); let __recv_ptr: *mut r#box = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut r#box }; let __result = unsafe { &mut *__recv_ptr }.inc(); __result }))); let __go_binary_value = { let __tmp_x = (*{ let __field = (*b.lock().unwrap().as_ref().unwrap()).n.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*__go_rhs_6.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; __go_binary_value })));
    println!("{} {} {}", format!("{}", "field and pointer method:".to_string()), format!("{}", { let __v = (*z.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", (*{ let __field = (*b.lock().unwrap().as_ref().unwrap()).n.clone(); __field }.lock().unwrap().as_ref().unwrap())));
    println!("{} {} {}", format!("{}", "in a method:".to_string()), format!("{}", { let __recv = b.clone(); let __recv_ptr: *mut r#box = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut r#box }; let __result = unsafe { &mut *__recv_ptr }.total(); __result }), format!("{}", (*{ let __field = (*b.lock().unwrap().as_ref().unwrap()).n.clone(); __field }.lock().unwrap().as_ref().unwrap())));

    let mut n = Arc::new(Mutex::new(Some(3i64)));
    let mut pn = n.clone();
    let pn_closure_clone = pn.clone(); let mut w = Arc::new(Mutex::new(Some({ let __go_rhs_7 = Arc::new(Mutex::new(Some({ let __f_holder = Arc::new(Mutex::new(Some(Arc::new(move || -> i64 {
        { let new_val = 10; *pn_closure_clone.lock().unwrap() = Some(new_val); };
        1
    }) as Arc<dyn Fn() -> i64 + Send + Sync>))); let __f: Arc<dyn Fn() -> i64 + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }))); let __go_binary_value = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*__go_rhs_7.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; __go_binary_value })));
    println!("{} {} {}", format!("{}", "address taken:".to_string()), format!("{}", { let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }));

    let mut c = Arc::new(Mutex::new(Some(0i64)));
    let mut c_closure_clone = c.clone(); let mut next = Arc::new(Mutex::new(Some(Arc::new(move || -> i64 {
        let mut c_closure_clone = c_closure_clone.clone();
        { let mut guard = c_closure_clone.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return { let __v = (*c_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }) as Arc<dyn Fn() -> i64 + Send + Sync>)));
    println!("{} {} {}", format!("{}", "captured by a closure:".to_string()), format!("{}", { let __go_rhs_8 = Arc::new(Mutex::new(Some({
        let __f: Arc<dyn Fn() -> i64 + Send + Sync> = next.lock().unwrap().as_ref().unwrap().clone();
        (__f)(

        )
    }))); let __go_binary_value = { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*__go_rhs_8.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; __go_binary_value }), format!("{}", { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }));

    println!("{} {} {}", format!("{}", "package variable:".to_string()), format!("{}", { let __go_rhs_9 = Arc::new(Mutex::new(Some(bump_counter()))); let __go_binary_value = { let __tmp_x = (*counter.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*__go_rhs_9.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; __go_binary_value }), format!("{}", { let __v = (*counter.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    println!("{} {} {}", format!("{}", "parenthesized:".to_string()), format!("{}", { let __go_rhs_10 = (relabel()); let __go_binary_value = format!("{}{}", (*label.lock().unwrap().as_ref().unwrap()), { let __v = (*__go_rhs_10.lock().unwrap().as_ref().unwrap()).clone(); __v }); __go_binary_value }), format!("{}", { let __v = (*label.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    println!("{} {}", format!("{}", "field of a call:".to_string()), format!("{}", { let __go_rhs_11 = Arc::new(Mutex::new(Some((*(*new_record().lock().unwrap().as_ref().unwrap()).n.lock().unwrap().as_ref().unwrap())))); let __go_binary_value = { let __tmp_x = (*counter.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*__go_rhs_11.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; __go_binary_value }));
    println!("{} {}", format!("{}", "string field of a call:".to_string()), format!("{}", { let __go_rhs_12 = Arc::new(Mutex::new(Some((*(*new_record().lock().unwrap().as_ref().unwrap()).s.lock().unwrap().as_ref().unwrap()).clone()))); let __go_binary_value = format!("{}{}", (*label.lock().unwrap().as_ref().unwrap()), { let __v = (*__go_rhs_12.lock().unwrap().as_ref().unwrap()).clone(); __v }); __go_binary_value }));
    println!("{} {}", format!("{}", "method of a call's result:".to_string()), format!("{}", { let __go_rhs_13 = Arc::new(Mutex::new(Some({
        let __recv = new_record();
        let __result = (*__recv.lock().unwrap().as_ref().unwrap()).twice();
        __result
    }))); let __go_binary_value = { let __tmp_x = (*counter.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*__go_rhs_13.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; __go_binary_value }));

    let mut str = Arc::new(Mutex::new(Some("a".to_string())));
    let mut t = Arc::new(Mutex::new(Some({ let __go_rhs_14 = tag(str.clone()); let __go_binary_value = format!("{}{}", { let __v = (*str.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*__go_rhs_14.lock().unwrap().as_ref().unwrap()).clone(); __v }); __go_binary_value })));
    println!("{} {} {}", format!("{}", "string:".to_string()), format!("{}", { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*str.lock().unwrap().as_ref().unwrap()).clone(); __v }));

    let mut m = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some(1i64))))]))));
    println!("{} {} {}", format!("{}", "map element:".to_string()), format!("{}", { let __go_rhs_15 = Arc::new(Mutex::new(Some(add_one(m.clone())))); let __go_binary_value = { let __tmp_x = { let __map = { let __map_holder = m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }; let __tmp_y = { let __v = (*__go_rhs_15.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; __go_binary_value }), format!("{}", { let __map = { let __map_holder = m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));

    let mut f = Arc::new(Mutex::new(Some(1.5)));
    let mut pf = f.clone();
    let pf_closure_clone = pf.clone(); let mut g = Arc::new(Mutex::new(Some({ let __go_rhs_16 = Arc::new(Mutex::new(Some({ let __f_holder = Arc::new(Mutex::new(Some(Arc::new(move || -> f64 {
        { let new_val = 2.0; *pf_closure_clone.lock().unwrap() = Some(new_val); };
        3.0
    }) as Arc<dyn Fn() -> f64 + Send + Sync>))); let __f: Arc<dyn Fn() -> f64 + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }))); let __go_binary_value = { let __tmp_x = { let __v = (*f.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*__go_rhs_16.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; __go_binary_value })));
    println!("{} {} {}", format!("{}", "float:".to_string()), { let __text = go_float_string(({ let __v = (*g.lock().unwrap().as_ref().unwrap()).clone(); __v }) as f64, false); __text }, { let __text = go_float_string(({ let __v = (*f.lock().unwrap().as_ref().unwrap()).clone(); __v }) as f64, false); __text });

    let mut words = Arc::new(Mutex::new(Some(vec!["ab".to_string(), "cde".to_string()])));
    println!("{} {}", format!("{}", "len of a call:".to_string()), format!("{}", { let __go_rhs_17 = Arc::new(Mutex::new(Some(({ let __len = (*twice(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = words.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() })))).lock().unwrap().as_ref().unwrap()).len(); __len }) as i64))); let __go_binary_value = { let __tmp_x = (*counter.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*__go_rhs_17.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; __go_binary_value }));

    let mut yes = Arc::new(Mutex::new(Some(true)));
    println!("{} {}", format!("{}", "&& on the left:".to_string()), format!("{}", { let __go_lhs_18 = Arc::new(Mutex::new(Some(({ let __v = (*yes.lock().unwrap().as_ref().unwrap()).clone(); __v } && (*flag.lock().unwrap().as_ref().unwrap()))))); let __go_binary_value = { let __tmp_x = { let __v = (*__go_lhs_18.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = raise(); __tmp_x == __tmp_y }; __go_binary_value }));
    println!("{} {} {}", format!("{}", "len and cap on the left:".to_string()), format!("{}", { let __go_lhs_19 = Arc::new(Mutex::new(Some(({ let __len = (*text.lock().unwrap().as_ref().unwrap()).len(); __len }) as i64))); let __go_binary_value = { let __tmp_x = { let __v = (*__go_lhs_19.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = grow(); __tmp_x + __tmp_y }; __go_binary_value }), format!("{}", { let __go_lhs_20 = Arc::new(Mutex::new(Some((*spare.lock().unwrap()).as_ref().map(|__v| __v.capacity()).unwrap_or(0) as i64))); let __go_binary_value = { let __tmp_x = { let __v = (*__go_lhs_20.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = regrow(); __tmp_x + __tmp_y }; __go_binary_value }));
    let mut low = Arc::new(Mutex::new(Some(2i64)));
    let mut pl = low.clone();
    let pl_closure_clone = pl.clone(); println!("{} {}", format!("{}", "min on the left:".to_string()), format!("{}", { let __go_lhs_21 = Arc::new(Mutex::new(Some(std::cmp::min(({ let __v = (*low.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64), (5 as i64))))); let __go_binary_value = { let __tmp_x = { let __v = (*__go_lhs_21.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __f_holder = Arc::new(Mutex::new(Some(Arc::new(move || -> i64 {
        { let new_val = 0; *pl_closure_clone.lock().unwrap() = Some(new_val); };
        0
    }) as Arc<dyn Fn() -> i64 + Send + Sync>))); let __f: Arc<dyn Fn() -> i64 + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }; __tmp_x + __tmp_y }; __go_binary_value }));
    let pl_closure_clone = pl.clone(); println!("{} {}", format!("{}", "negation on the left:".to_string()), format!("{}", { let __go_rhs_22 = Arc::new(Mutex::new(Some({ let __f_holder = Arc::new(Mutex::new(Some(Arc::new(move || -> i64 {
        { let new_val = 7; *pl_closure_clone.lock().unwrap() = Some(new_val); };
        0
    }) as Arc<dyn Fn() -> i64 + Send + Sync>))); let __f: Arc<dyn Fn() -> i64 + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }))); let __go_binary_value = { let __tmp_x = -(({ let __v = (*low.lock().unwrap().as_ref().unwrap()); __v })); let __tmp_y = { let __v = (*__go_rhs_22.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; __go_binary_value }));
    let mut dst = Arc::new(Mutex::new(Some(vec![1i64])));
    println!("{} {} {}", format!("{}", "copy on the right:".to_string()), format!("{}", { let __go_rhs_23 = {
        let _src = { let __copy_src_holder = Arc::new(Mutex::new(Some(vec![5i64]))).clone(); let __copy_src_guard = __copy_src_holder.lock().unwrap(); __copy_src_guard.as_ref().map(|__s| __s.to_vec()).unwrap_or_default() };
        let _n = std::cmp::min((*dst.lock().unwrap().as_ref().unwrap()).len(), _src.len());
        for _i in 0.._n {
            (*dst.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone();
        }
        Arc::new(Mutex::new(Some(_n as i64)))
    }; let __go_binary_value = {
        let __tmp_x = { let __seq = { let __seq_holder = dst.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() };
        let __tmp_y = { let __v = (*__go_rhs_23.lock().unwrap().as_ref().unwrap()).clone(); __v };
        __tmp_x + __tmp_y
    }; __go_binary_value }), format!("{}", format_slice(&dst)));

    let mut total = Arc::new(Mutex::new(Some(0i64)));
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x < __tmp_y } {
        { let new_val = { let __tmp_x = { let __v = (*total.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = plain(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __tmp_x + __tmp_y }; *total.lock().unwrap() = Some(new_val); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    println!("{} {}", format!("{}", "plain local:".to_string()), format!("{}", { let __v = (*total.lock().unwrap().as_ref().unwrap()).clone(); __v }));

    if { let __go_rhs_24 = Arc::new(Mutex::new(Some(bump(s.clone())))); let __go_binary_value = {
        let __tmp_x = { let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() };
        let __tmp_y = { let __v = (*__go_rhs_24.lock().unwrap().as_ref().unwrap()).clone(); __v };
        __tmp_x > __tmp_y
    }; __go_binary_value } {
        println!("{} {}", format!("{}", "condition: before".to_string()), format!("{}", format_slice(&s)));
    } else {
        println!("{} {}", format!("{}", "condition: after".to_string()), format!("{}", format_slice(&s)));
    }
}

pub(crate) fn __go_init_all() {
    static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();
    __GO_INIT_ONCE.call_once(|| {
        self::__go_init_globals();
    });
}
