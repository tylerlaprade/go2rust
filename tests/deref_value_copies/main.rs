#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use std::any::Any;
use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

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

fn format_slice<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[T]>,
    T: Display,
{
    let guard = slice.borrow();
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

fn format_slice_wrapped<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[Rc<RefCell<Option<T>>>]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| {
            let inner = v.borrow();
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

/// Runs an append, __store, for elements whose Go copy is not their Clone,
/// such as a struct of cells. When the append moves the slice to a new
/// backing array, the elements it carried over are replaced with __copy of
/// them, so the old array and the new one share nothing.
trait GoSliceAppendCopying<T> {
    fn go_append_copying(&mut self, __store: impl FnOnce(&mut Self), __copy: impl Fn(&T) -> T);
}

impl<T: Clone> GoSliceAppendCopying<T> for GoSlice<T> {
    fn go_append_copying(&mut self, __store: impl FnOnce(&mut Self), __copy: impl Fn(&T) -> T) {
        let __before = self.clone();
        let __len = self.len();
        __store(self);
        if !self.shares_backing_with(&__before) {
            for __i in 0..__len.min(self.len()) {
                let __value = __copy(&self.as_slice()[__i]);
                self.as_mut_slice()[__i] = __value;
            }
        }
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

thread_local! {
    static __GO_RECEIVER_HANDLES: std::cell::RefCell<std::vec::Vec<(usize, Box<dyn std::any::Any>)>> = std::cell::RefCell::new(std::vec::Vec::new());
}

fn go_set_receiver_handle<H: Clone + 'static>(handle: &H, address: usize) {
    __GO_RECEIVER_HANDLES.with(|__handles| {
        let mut __handles = __handles.borrow_mut();
        if __handles.len() >= 4096 {
            __handles.remove(0);
        }
        __handles.push((address, Box::new(handle.clone())));
    });
}

fn go_hint_receiver_mut<T: 'static>(handle: &std::rc::Rc<std::cell::RefCell<Option<T>>>) -> *mut T {
    let __ptr = handle.borrow_mut().as_mut().unwrap() as *mut T;
    go_set_receiver_handle(handle, __ptr as *const () as usize);
    __ptr
}

fn go_hint_receiver<T: 'static>(handle: &std::rc::Rc<std::cell::RefCell<Option<T>>>) -> *const T {
    let __ptr = handle.borrow().as_ref().unwrap() as *const T;
    go_set_receiver_handle(handle, __ptr as *const () as usize);
    __ptr
}

fn go_receiver_ptr_mut<T>(handle: &std::rc::Rc<std::cell::RefCell<Option<T>>>) -> *mut T {
    handle.borrow_mut().as_mut().unwrap() as *mut T
}

fn go_receiver_ptr<T>(handle: &std::rc::Rc<std::cell::RefCell<Option<T>>>) -> *const T {
    handle.borrow().as_ref().unwrap() as *const T
}

fn go_take_receiver_handle<T: 'static>(address: usize) -> Option<std::rc::Rc<std::cell::RefCell<Option<T>>>> {
    __GO_RECEIVER_HANDLES.with(|__handles| {
        let mut __handles = __handles.borrow_mut();
        let __at = __handles.iter().rposition(|(__address, __handle)| *__address == address && __handle.is::<std::rc::Rc<std::cell::RefCell<Option<T>>>>())?;
        // A hint is recorded just before its call's arguments run, so the
        // hints after this one belong to calls among those arguments, which
        // have all returned: any still here were never taken.
        let __handle = __handles.drain(__at..).next()?;
        __handle.1.downcast::<std::rc::Rc<std::cell::RefCell<Option<T>>>>().ok().map(|__handle| *__handle)
    })
}

fn __go_type_name(val: &dyn Any) -> &'static str {
    if val.is::<i64>() { return "int" }
    if val.is::<i32>() { return "int32" }
    if val.is::<char>() { return "int32" }
    if val.is::<i8>() { return "int8" }
    if val.is::<i16>() { return "int16" }
    if val.is::<u64>() { return "uint" }
    if val.is::<u32>() { return "uint32" }
    if val.is::<u8>() { return "uint8" }
    if val.is::<u16>() { return "uint16" }
    if val.is::<f64>() { return "float64" }
    if val.is::<f32>() { return "float32" }
    if val.is::<bool>() { return "bool" }
    if val.is::<String>() { return "string" }
    if val.is::<Vec<i64>>() { return "[]int" }
    if val.is::<Vec<i32>>() { return "[]int32" }
    if val.is::<Vec<f64>>() { return "[]float64" }
    if val.is::<Vec<String>>() { return "[]string" }
    if val.is::<Vec<bool>>() { return "[]bool" }
    if val.is::<P>() { return "main.P" }
    std::any::type_name_of_val(val)
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

#[derive(Clone)]
pub struct GoSliceElemPtr<T> {
    slice: Rc<RefCell<Option<Vec<T>>>>,
    index: usize,
    /// The element's identity when the pointer was made with the slice
    /// free to borrow; otherwise identity reads it from the slice.
    id: Option<(usize, usize)>,
}

pub struct GoSliceElemRef<T: Clone> {
    value: Option<T>,
}

pub struct GoSliceElemMutRef<T: Clone> {
    slice: Rc<RefCell<Option<Vec<T>>>>,
    index: usize,
    value: Option<T>,
}

pub trait GoArrayElemBacking<T: Clone + 'static, const N: usize> {
    fn borrow_at(&self, index: usize) -> Option<T>;
    fn assign_at(&self, index: usize, value: Option<T>);
    fn identity_at(&self, index: usize) -> (*const (), usize);
}

#[derive(Clone)]
pub struct GoDirectArrayElemBacking<T: Clone + 'static, const N: usize> {
    array: Rc<RefCell<Option<[T; N]>>>,
}

impl<T: Clone + 'static, const N: usize> GoArrayElemBacking<T, N> for GoDirectArrayElemBacking<T, N> {
    fn borrow_at(&self, index: usize) -> Option<T> {
        let guard = self.array.borrow();
        guard.as_ref().and_then(|values| values.get(index).cloned())
    }

    fn assign_at(&self, index: usize, value: Option<T>) {
        if let Some(value) = value {
            if let Some(values) = self.array.borrow_mut().as_mut() {
                values[index] = value;
            }
        }
    }

    fn identity_at(&self, index: usize) -> (*const (), usize) {
        (Rc::as_ptr(&self.array) as *const (), index)
    }
}

#[derive(Clone)]
pub struct GoNestedArrayElemBacking<T: Clone + 'static, const N: usize, const OUT: usize> {
    outer: Rc<RefCell<Option<[[T; N]; OUT]>>>,
    outer_index: usize,
}

impl<T: Clone + 'static, const N: usize, const OUT: usize> GoArrayElemBacking<T, N> for GoNestedArrayElemBacking<T, N, OUT> {
    fn borrow_at(&self, index: usize) -> Option<T> {
        let guard = self.outer.borrow();
        guard.as_ref().and_then(|values| values.get(self.outer_index)).and_then(|inner| inner.get(index)).cloned()
    }

    fn assign_at(&self, index: usize, value: Option<T>) {
        if let Some(value) = value {
            if let Some(values) = self.outer.borrow_mut().as_mut() {
                values[self.outer_index][index] = value;
            }
        }
    }

    fn identity_at(&self, index: usize) -> (*const (), usize) {
        (Rc::as_ptr(&self.outer) as *const (), self.outer_index.wrapping_mul(N).wrapping_add(index))
    }
}

#[derive(Clone)]
pub struct GoArrayElemFromElemBacking<T: Clone + 'static, const N: usize, const OUT: usize> {
    parent: GoArrayElemPtr<[T; N], OUT>,
}

impl<T: Clone + 'static, const N: usize, const OUT: usize> GoArrayElemBacking<T, N> for GoArrayElemFromElemBacking<T, N, OUT> {
    fn borrow_at(&self, index: usize) -> Option<T> {
        let inner = self.parent.borrow();
        inner.as_ref().and_then(|values| values.get(index).cloned())
    }

    fn assign_at(&self, index: usize, value: Option<T>) {
        if let Some(value) = value {
            let mut inner = self.parent.borrow_mut();
            if let Some(values) = inner.as_mut() {
                values[index] = value;
            }
        }
    }

    fn identity_at(&self, index: usize) -> (*const (), usize) {
        let (base, outer_index) = self.parent.identity();
        (base, outer_index.wrapping_mul(N).wrapping_add(index))
    }
}

#[derive(Clone)]
pub struct GoArrayElemFromGoPtrBacking<T: Clone + 'static, const N: usize> {
    parent: GoPtr<[T; N]>,
}

impl<T: Clone + 'static, const N: usize> GoArrayElemBacking<T, N> for GoArrayElemFromGoPtrBacking<T, N> {
    fn borrow_at(&self, index: usize) -> Option<T> {
        let inner = self.parent.borrow();
        inner.as_ref().and_then(|values| values.get(index).cloned())
    }

    fn assign_at(&self, index: usize, value: Option<T>) {
        if let Some(value) = value {
            self.parent.with_mut(|values| {
                values[index] = value;
            });
        }
    }

    fn identity_at(&self, index: usize) -> (*const (), usize) {
        (self.parent.addr() as *const (), index)
    }
}

#[derive(Clone)]
pub struct GoArrayElemPtr<T: Clone + 'static, const N: usize> {
    backing: Rc<dyn GoArrayElemBacking<T, N>>,
    index: usize,
}

pub struct GoArrayElemRef<T: Clone> {
    value: Option<T>,
}

pub struct GoArrayElemMutRef<T: Clone + 'static, const N: usize> {
    backing: Rc<dyn GoArrayElemBacking<T, N>>,
    index: usize,
    value: Option<T>,
}

impl<T> GoSliceElemPtr<T> {
    pub fn new(slice: Rc<RefCell<Option<Vec<T>>>>, index: usize) -> Self {
        // &s[i] panics, as indexing does, when i is not below len(s). A
        // slice already borrowed for the expression is not checked.
        if let Ok(__guard) = slice.try_borrow() {
            let __len = __guard.as_ref().map_or(0, |__values| __values.len());
            if index >= __len {
                drop(__guard);
                panic!("index out of bounds: the len is {} but the index is {}", __len, index);
            }
            // Hold the backing array the slice has now, not the variable.
            let __header = (*__guard).clone();
            drop(__guard);
            // The element's address, taken now: nothing else can lock
            // the copy, and == need not wait for it.
            let __id = __header.as_ref().map(|__slice| ((__slice.as_slice().as_ptr() as usize).wrapping_add(index.wrapping_mul(std::mem::size_of::<T>())), 0));
            return GoSliceElemPtr { slice: std::rc::Rc::new(std::cell::RefCell::new(__header)), index, id: __id };
        }
        GoSliceElemPtr { slice, index, id: None }
    }

    /// The array and element the pointer names, for == between pointers.
    pub fn identity(&self) -> (usize, usize) {
        if let Some(__id) = self.id {
            return __id;
        }
        if let Ok(__guard) = self.slice.try_borrow() {
            if let Some(__slice) = __guard.as_ref() {
                let __base = __slice.as_slice().as_ptr() as usize;
                return (__base.wrapping_add(self.index.wrapping_mul(std::mem::size_of::<T>())), 0);
            }
        }
        (Rc::as_ptr(&self.slice) as *const () as usize, self.index)
    }

    pub fn slice_handle(&self) -> Rc<RefCell<Option<Vec<T>>>> {
        self.slice.clone()
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

impl<T: Clone> GoSliceElemPtr<T> {
    pub fn borrow(&self) -> GoSliceElemRef<T> {
        let guard = self.slice.borrow();
        GoSliceElemRef {
            value: guard.as_ref().and_then(|values| values.get(self.index).cloned()),
        }
    }

    pub fn borrow_mut(&self) -> GoSliceElemMutRef<T> {
        let guard = self.slice.borrow();
        GoSliceElemMutRef {
            slice: self.slice.clone(),
            index: self.index,
            value: guard.as_ref().and_then(|values| values.get(self.index).cloned()),
        }
    }
}

impl<T: Clone + 'static, const N: usize> GoArrayElemPtr<T, N> {
    pub fn new(array: Rc<RefCell<Option<[T; N]>>>, index: usize) -> Self {
        GoArrayElemPtr {
            backing: Rc::new(GoDirectArrayElemBacking { array }),
            index,
        }
    }

    pub fn nested<const OUT: usize>(outer: Rc<RefCell<Option<[[T; N]; OUT]>>>, outer_index: usize, index: usize) -> Self {
        GoArrayElemPtr {
            backing: Rc::new(GoNestedArrayElemBacking { outer, outer_index }),
            index,
        }
    }

    pub fn from_array_elem<const OUT: usize>(parent: GoArrayElemPtr<[T; N], OUT>, index: usize) -> Self {
        GoArrayElemPtr {
            backing: Rc::new(GoArrayElemFromElemBacking { parent }),
            index,
        }
    }

    pub fn from_go_ptr(parent: GoPtr<[T; N]>, index: usize) -> Self {
        GoArrayElemPtr {
            backing: Rc::new(GoArrayElemFromGoPtrBacking { parent }),
            index,
        }
    }

    pub fn borrow(&self) -> GoArrayElemRef<T> {
        GoArrayElemRef {
            value: self.backing.borrow_at(self.index),
        }
    }

    pub fn borrow_mut(&self) -> GoArrayElemMutRef<T, N> {
        GoArrayElemMutRef {
            backing: self.backing.clone(),
            index: self.index,
            value: self.backing.borrow_at(self.index),
        }
    }

    /// The element as the guard of a cell, for code written against a
    /// Mutex-held value: sync/atomic's Load and Store on &a[i].
    #[allow(dead_code)]
    pub fn lock(&self) -> Result<GoArrayElemMutRef<T, N>, std::convert::Infallible> {
        Ok(self.borrow_mut())
    }

    pub fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut value = self.backing.borrow_at(self.index).expect("nil pointer dereference");
        let result = f(&mut value);
        self.backing.assign_at(self.index, Some(value));
        result
    }

    pub fn identity(&self) -> (*const (), usize) {
        self.backing.identity_at(self.index)
    }
}

impl<T: Clone> std::ops::Deref for GoSliceElemRef<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Clone> std::ops::Deref for GoSliceElemMutRef<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Clone> std::ops::DerefMut for GoSliceElemMutRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T: Clone> Drop for GoSliceElemMutRef<T> {
    fn drop(&mut self) {
        if let Some(value) = self.value.clone() {
            if let Some(values) = self.slice.borrow_mut().as_mut() {
                values[self.index] = value;
            }
        }
    }
}

impl<T: Clone> std::ops::Deref for GoArrayElemRef<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Clone + 'static, const N: usize> std::ops::Deref for GoArrayElemMutRef<T, N> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Clone + 'static, const N: usize> std::ops::DerefMut for GoArrayElemMutRef<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T: Clone + 'static, const N: usize> Drop for GoArrayElemMutRef<T, N> {
    fn drop(&mut self) {
        self.backing.assign_at(self.index, self.value.clone());
    }
}

pub trait GoArrayElemPtrDyn<T: 'static> {
    fn borrow_dyn(&self) -> Option<T>;
    fn assign_dyn(&self, value: Option<T>);
    fn with_mut_dyn(&self, f: &mut dyn FnMut(&mut T));
    fn identity_dyn(&self) -> (*const (), usize);
}

pub struct GoForeignArrayElemPtrDyn<T: 'static> {
    borrow: Rc<dyn Fn() -> Option<T>>,
    assign: Rc<dyn Fn(Option<T>)>,
    with_mut: Rc<dyn Fn(&mut dyn FnMut(&mut T))>,
    identity: Rc<dyn Fn() -> (*const (), usize)>,
}

impl<T: 'static> GoArrayElemPtrDyn<T> for GoForeignArrayElemPtrDyn<T> {
    fn borrow_dyn(&self) -> Option<T> {
        (self.borrow)()
    }

    fn assign_dyn(&self, value: Option<T>) {
        (self.assign)(value)
    }

    fn with_mut_dyn(&self, f: &mut dyn FnMut(&mut T)) {
        (self.with_mut)(f)
    }

    fn identity_dyn(&self) -> (*const (), usize) {
        (self.identity)()
    }
}

impl<T: Clone + 'static, const N: usize> GoArrayElemPtrDyn<T> for GoArrayElemPtr<T, N> {
    fn borrow_dyn(&self) -> Option<T> {
        (*self.borrow()).clone()
    }

    fn assign_dyn(&self, value: Option<T>) {
        *self.borrow_mut() = value;
    }

    fn with_mut_dyn(&self, f: &mut dyn FnMut(&mut T)) {
        self.with_mut(|value| f(value));
    }

    fn identity_dyn(&self) -> (*const (), usize) {
        self.identity()
    }
}

pub enum GoPtr<T: 'static> {
    Nil,
    Raw(usize),
    Local(Rc<RefCell<Option<T>>>),
    SliceElem(GoSliceElemPtr<T>),
    ArrayElem(Rc<dyn GoArrayElemPtrDyn<T>>),
}

impl<T: 'static> Clone for GoPtr<T> {
    fn clone(&self) -> Self {
        match self {
            GoPtr::Nil => GoPtr::Nil,
            GoPtr::Raw(addr) => GoPtr::Raw(*addr),
            GoPtr::Local(value) => GoPtr::Local(value.clone()),
            GoPtr::SliceElem(value) => GoPtr::SliceElem(GoSliceElemPtr { slice: value.slice.clone(), index: value.index, id: value.id }),
            GoPtr::ArrayElem(value) => GoPtr::ArrayElem(value.clone()),
        }
    }
}

impl<T: 'static> GoPtr<T> {
    pub fn nil() -> Self {
        GoPtr::Nil
    }

    pub fn raw(addr: usize) -> Self {
        if addr == 0 {
            GoPtr::Nil
        } else {
            GoPtr::Raw(addr)
        }
    }

    pub fn local(value: Rc<RefCell<Option<T>>>) -> Self {
        if value.borrow().is_none() {
            GoPtr::Nil
        } else {
            GoPtr::Local(value)
        }
    }

    pub fn slice_elem(value: GoSliceElemPtr<T>) -> Self {
        GoPtr::SliceElem(value)
    }

    pub fn slice_elem_opt(value: Option<GoSliceElemPtr<T>>) -> Self {
        match value {
            Some(value) => GoPtr::SliceElem(value),
            None => GoPtr::Nil,
        }
    }

    pub fn array_elem<const N: usize>(value: GoArrayElemPtr<T, N>) -> Self
    where
        T: Clone,
    {
        GoPtr::ArrayElem(Rc::new(value))
    }

    pub fn array_elem_opt<const N: usize>(value: Option<GoArrayElemPtr<T, N>>) -> Self
    where
        T: Clone,
    {
        match value {
            Some(value) => GoPtr::ArrayElem(Rc::new(value)),
            None => GoPtr::Nil,
        }
    }

    pub fn array_elem_foreign(
        borrow: Rc<dyn Fn() -> Option<T>>,
        assign: Rc<dyn Fn(Option<T>)>,
        with_mut: Rc<dyn Fn(&mut dyn FnMut(&mut T))>,
        identity: Rc<dyn Fn() -> (*const (), usize)>,
    ) -> Self {
        GoPtr::ArrayElem(Rc::new(GoForeignArrayElemPtrDyn {
            borrow,
            assign,
            with_mut,
            identity,
        }))
    }

    pub fn is_nil(&self) -> bool {
        match self {
            GoPtr::Nil => true,
            GoPtr::Raw(addr) => *addr == 0,
            GoPtr::Local(value) => value.borrow().is_none(),
            GoPtr::SliceElem(value) => {
                let guard = value.slice.borrow();
                guard.as_ref().and_then(|values| values.get(value.index)).is_none()
            }
            GoPtr::ArrayElem(value) => value.identity_dyn().0.is_null(),
        }
    }

    pub fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        match self {
            GoPtr::Nil => panic!("nil pointer dereference"),
            GoPtr::Raw(_) => panic!("raw unsafe pointer mutable borrow requires unsafe pointee support"),
            GoPtr::Local(slot) => {
                let mut guard = slot.borrow_mut();
                f(guard.as_mut().unwrap())
            }
            GoPtr::SliceElem(slot) => {
                let mut guard = slot.slice.borrow_mut();
                let values = guard.as_mut().expect("nil pointer dereference");
                f(values.get_mut(slot.index).expect("nil pointer dereference"))
            }
            GoPtr::ArrayElem(slot) => {
                let mut result = None;
                let mut callback = Some(f);
                slot.with_mut_dyn(&mut |value| {
                    let f = callback.take().expect("array element pointer mutable borrow called twice");
                    result = Some(f(value));
                });
                result.expect("nil pointer dereference")
            }
        }
    }

    pub fn ptr_eq(left: &Self, right: &Self) -> bool {
        match (left, right) {
            (GoPtr::Nil, GoPtr::Nil) => true,
            (GoPtr::Raw(_), _) | (_, GoPtr::Raw(_)) => left.addr() == right.addr(),
            (GoPtr::Local(left), GoPtr::Local(right)) => Rc::ptr_eq(left, right),
            (GoPtr::SliceElem(left), GoPtr::SliceElem(right)) => {
                left.identity() == right.identity()
            }
            (GoPtr::ArrayElem(left), GoPtr::ArrayElem(right)) => left.identity_dyn() == right.identity_dyn(),
            _ => false,
        }
    }

    pub fn addr(&self) -> usize {
        match self {
            GoPtr::Nil => 0,
            GoPtr::Raw(addr) => *addr,
            GoPtr::Local(value) => Rc::as_ptr(value) as usize,
            GoPtr::SliceElem(value) => {
                let (base, index) = value.identity();
                base.wrapping_add(index)
            }
            GoPtr::ArrayElem(value) => {
                let (base, index) = value.identity_dyn();
                (base as usize).wrapping_add(index)
            }
        }
    }
}

impl<T: Clone + 'static> GoPtr<T> {
    pub fn borrow(&self) -> Option<T> {
        match self {
            GoPtr::Nil => None,
            GoPtr::Raw(_) => panic!("raw unsafe pointer dereference requires unsafe pointee support"),
            GoPtr::Local(value) => (*value.borrow()).clone(),
            GoPtr::SliceElem(value) => (*value.borrow()).clone(),
            GoPtr::ArrayElem(value) => value.borrow_dyn(),
        }
    }

    pub fn assign(&self, value: Option<T>) {
        match self {
            GoPtr::Nil => panic!("nil pointer dereference"),
            GoPtr::Raw(_) => panic!("raw unsafe pointer assignment requires unsafe pointee support"),
            GoPtr::Local(slot) => *slot.borrow_mut() = value,
            GoPtr::SliceElem(slot) => *slot.borrow_mut() = value,
            GoPtr::ArrayElem(slot) => slot.assign_dyn(value),
        }
    }
}

impl<T: 'static> Default for GoPtr<T> {
    fn default() -> Self {
        GoPtr::Nil
    }
}

impl<T: 'static> std::fmt::Debug for GoPtr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_nil() {
            write!(f, "<nil>")
        } else {
            write!(f, "<ptr>")
        }
    }
}

#[derive(Debug, Clone)]
pub struct P {
    pub n: Rc<RefCell<Option<i64>>>,
    pub tags: Rc<RefCell<Option<Vec<String>>>>,
}

impl P {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.n.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.tags.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            n: __go_clone_0_0,
            tags: __go_clone_1_0,
        }
    }
}


impl Default for P {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_1_0 = Rc::new(RefCell::new(None));
        Self {
            n: __go_default_0_0,
            tags: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for P {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.n.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice(&self.tags));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone, Default)]
pub struct Box_ {
    pub inner: Rc<RefCell<Option<P>>>,
}

impl Box_ {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.inner.clone();
        Self {
            inner: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for Box_ {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.inner.borrow(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


pub trait Getter: std::fmt::Display + Any {
    fn __go_clone_box_getter(&self) -> Box<dyn Getter>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_print(&self, plus: bool) -> String { if plus { format!("{:#}", self) } else { format!("{}", self) } }
    fn __go_eq_getter(&self, other: &dyn Getter) -> bool;
    fn get(&self) -> i64;
}

impl Clone for Box<dyn Getter> {
    fn clone(&self) -> Self {
        Getter::__go_clone_box_getter(self.as_ref())
    }
}

impl P {
    pub fn inc(&mut self) {
        { let __target = self.n.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    pub fn get(&self) -> i64 {
        return (*self.n.borrow().as_ref().unwrap());
    }

    pub fn self_(&self) -> Rc<RefCell<Option<P>>> {
        if let Some(mut p) = go_take_receiver_handle::<Self>(self as *const Self as usize) {
        return p.clone();
        } else {
        Rc::new(RefCell::new(Some(self.clone())))
        }
    }
}

impl Getter for P {
    fn get(&self) -> i64 {
        P::get(self)
    }
    fn __go_clone_box_getter(&self) -> Box<dyn Getter> {
        Box::new(self.clone()) as Box<dyn Getter>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_getter(&self, other: &dyn Getter) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<P>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct PPtr(pub Rc<RefCell<Option<P>>>);

impl std::fmt::Display for PPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Getter for PPtr {
    fn get(&self) -> i64 {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        P::get(__recv)
    }
    fn __go_clone_box_getter(&self) -> Box<dyn Getter> {
        Box::new(self.clone()) as Box<dyn Getter>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => if plus { format!("&{:#}", __v) } else { format!("&{}", __v) }, None => "<nil>".to_string() }
    }
    fn __go_eq_getter(&self, other: &dyn Getter) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<PPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn bump(p: Rc<RefCell<Option<P>>>) -> Rc<RefCell<Option<P>>> {
    { let __target = (*p.borrow().as_ref().unwrap()).n.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = (*p.borrow().as_ref().unwrap()).tags.clone(); let __append_value = "x".to_string(); let mut __appended: Vec<String> = (*__append_target.borrow()).clone().unwrap_or_default(); __appended.push(__append_value); Rc::new(RefCell::new(Some(__appended))) })); *(*p.borrow().as_ref().unwrap()).tags.borrow_mut() = __slice_header; };
    { let __owned = p.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) }
}

pub fn get(p: Rc<RefCell<Option<P>>>) -> Rc<RefCell<Option<P>>> {
    Rc::new(RefCell::new(Some({ let __v = (*p.borrow().as_ref().unwrap()).__go_value_clone(); __v })))
}

/// An element read through a pointer to an array is that element: its
/// pointer method changes the array, and a copy of it is its own.
pub fn bump_first(arr: Rc<RefCell<Option<[P; 1]>>>) -> i64 {
    { let __seq = ({ let __v = (*arr.borrow().as_ref().unwrap()).clone(); __v }); __seq[(0) as usize].clone() }.inc();
    let mut x = Rc::new(RefCell::new(Some({ let __seq = ({ let __v = (*arr.borrow().as_ref().unwrap()).clone(); __v }); __seq[(0) as usize].clone() }.__go_value_clone())));
    { let new_val = 100; *(*x.borrow().as_ref().unwrap()).n.borrow_mut() = Some(new_val); };
    return (*(*x.borrow().as_ref().unwrap()).n.borrow().as_ref().unwrap());
}

fn main() {
    go_install_panic_exit();
    let mut a = Rc::new(RefCell::new(Some(P { n: Rc::new(RefCell::new(Some(1))), tags: Rc::new(RefCell::new(Some(vec!["a".to_string()]))), ..Default::default() })));
    let mut r = bump(Rc::new(RefCell::new(Some({ let __v = (*a.borrow().as_ref().unwrap()).__go_value_clone(); __v }))));
    println!("{} {} {} {}", format!("{}", (*(*a.borrow().as_ref().unwrap()).n.borrow().as_ref().unwrap())), format!("{}", (*(*r.borrow().as_ref().unwrap()).n.borrow().as_ref().unwrap())), format!("{}", format_slice_values(&(*(*a.borrow().as_ref().unwrap()).tags.borrow().as_ref().unwrap()).clone())), format!("{}", format_slice_values(&(*(*r.borrow().as_ref().unwrap()).tags.borrow().as_ref().unwrap()).clone())));

    let mut d = get(a.clone());
    { let new_val = 30; *(*d.borrow().as_ref().unwrap()).n.borrow_mut() = Some(new_val); };
    println!("{} {}", format!("{}", (*(*a.borrow().as_ref().unwrap()).n.borrow().as_ref().unwrap())), format!("{}", (*(*d.borrow().as_ref().unwrap()).n.borrow().as_ref().unwrap())));

    let mut list = Rc::new(RefCell::new(Some(Vec::<P>::new())));
    { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = list.clone(); let __append_value = { let __v = (*a.borrow().as_ref().unwrap()).__go_value_clone(); __v }; let mut __appended: Vec<P> = (*__append_target.borrow()).clone().unwrap_or_default(); __appended.go_append_copying(|__go_slice| { __go_slice.push(__append_value); }, |__go_elem| __go_elem.__go_value_clone()); Rc::new(RefCell::new(Some(__appended))) })); *list.borrow_mut() = __slice_header; };
    { let new_val = 40; *(*list.borrow().as_ref().unwrap())[(0) as usize].clone().n.borrow_mut() = Some(new_val); };
    println!("{} {}", format!("{}", (*(*a.borrow().as_ref().unwrap()).n.borrow().as_ref().unwrap())), format!("{}", (*(*list.borrow().as_ref().unwrap())[(0) as usize].clone().n.borrow().as_ref().unwrap())));

    let mut m = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<P>>>>::from([]))));
    { let __map_key = "k".to_string(); let __map_value = Rc::new(RefCell::new(Some({ let __v = (*a.borrow().as_ref().unwrap()).__go_value_clone(); __v }))); (*m.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    let mut e = Rc::new(RefCell::new(Some({ let __map_holder = m.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"k".to_string())).map(|__v| __v.borrow().as_ref().unwrap().__go_value_clone()).unwrap_or_else(|| Default::default()) })));
    { let new_val = 50; *(*e.borrow().as_ref().unwrap()).n.borrow_mut() = Some(new_val); };
    println!("{} {}", format!("{}", (*(*a.borrow().as_ref().unwrap()).n.borrow().as_ref().unwrap())), format!("{}", (*{ let __map_holder = m.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"k".to_string())).map(|__v| __v.borrow().as_ref().unwrap().__go_value_clone()).unwrap_or_else(|| Default::default()) }.n.borrow().as_ref().unwrap())));

    let mut arr = Rc::new(RefCell::new(Some([{ let __v = (*a.borrow().as_ref().unwrap()).__go_value_clone(); __v }])));
    { let new_val = 60; *(*arr.borrow().as_ref().unwrap())[(0) as usize].clone().n.borrow_mut() = Some(new_val); };
    let mut s = Rc::new(RefCell::new(Some(AnonymousStruct1 { inner: Rc::new(RefCell::new(Some({ let __v = (*a.borrow().as_ref().unwrap()).__go_value_clone(); __v }))) })));
    { let new_val = 70; *(*(*s.borrow().as_ref().unwrap()).inner.borrow().as_ref().unwrap()).n.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", (*(*a.borrow().as_ref().unwrap()).n.borrow().as_ref().unwrap())), format!("{}", (*(*arr.borrow().as_ref().unwrap())[(0) as usize].clone().n.borrow().as_ref().unwrap())), format!("{}", (*(*(*s.borrow().as_ref().unwrap()).inner.borrow().as_ref().unwrap()).n.borrow().as_ref().unwrap())));

    let mut iface: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new({ let __v = (*a.borrow().as_ref().unwrap()).__go_value_clone(); __v }) as Box<dyn Any>)));
    { let new_val = 2; *(*a.borrow().as_ref().unwrap()).n.borrow_mut() = Some(new_val); };
    println!("{} {}", format!("{}", (*({
        let val = iface.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<P>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "main.P")).__go_value_clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "main.P")
        }
    }).n.borrow().as_ref().unwrap())), format!("{}", (*(*a.borrow().as_ref().unwrap()).n.borrow().as_ref().unwrap())));

    { let __f_holder = Rc::new(RefCell::new(Some(Rc::new(move |p: Rc<RefCell<Option<P>>>| {
        { let new_val = 100; *(*p.borrow().as_ref().unwrap()).n.borrow_mut() = Some(new_val); };
    }) as Rc<dyn Fn(Rc<RefCell<Option<P>>>) -> ()>))); let __f: Rc<dyn Fn(Rc<RefCell<Option<P>>>) -> ()> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __v = (*a.borrow().as_ref().unwrap()).__go_value_clone(); __v })))) };
    println!("{}", format!("{}", (*(*a.borrow().as_ref().unwrap()).n.borrow().as_ref().unwrap())));

    let mut b = Rc::new(RefCell::new(Some(Box_ { inner: a.clone(), ..Default::default() })));
    let mut c = Rc::new(RefCell::new(Some({ let __v = (*(*b.borrow().as_ref().unwrap()).inner.borrow().as_ref().unwrap()).__go_value_clone(); __v })));
    { let new_val = 11; *(*c.borrow().as_ref().unwrap()).n.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", (*(*a.borrow().as_ref().unwrap()).n.borrow().as_ref().unwrap())), format!("{}", (*(*c.borrow().as_ref().unwrap()).n.borrow().as_ref().unwrap())), format!("{}", (*((*(*b.borrow().as_ref().unwrap()).inner.borrow_mut().as_mut().unwrap())).n.borrow().as_ref().unwrap())));

        // A selector on the dereference selects on the pointer: a pointer
        // method called on (*a) changes a.
    ({ let __v = (*a.borrow().as_ref().unwrap()).clone(); __v }).inc();
    { let __target = ((*a.borrow_mut().as_mut().unwrap())).n.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = ((*a.borrow_mut().as_mut().unwrap())).tags.clone(); let __append_value = "t".to_string(); let mut __appended: Vec<String> = (*__append_target.borrow()).clone().unwrap_or_default(); __appended.push(__append_value); Rc::new(RefCell::new(Some(__appended))) })); *((*a.borrow_mut().as_mut().unwrap())).tags.borrow_mut() = __slice_header; };
    { let __go_print_arg_1 = ({ let __v = (*a.borrow().as_ref().unwrap()).clone(); __v }).get(); let __go_print_arg_2 = Rc::new(RefCell::new(Some({ let __left = { let __recv = a.clone(); let __recv_ptr = go_hint_receiver(&__recv); let __result = unsafe { &*__recv_ptr }.self_(); __result }; let __right = a.clone(); let __both_nil = (*__left.borrow()).is_none() && (*__right.borrow()).is_none(); let __eq = __both_nil || Rc::ptr_eq(&__left, &__right) || { let __left_id = __left.borrow().as_ref().map(|__v| Rc::as_ptr(&__v.n) as *const () as usize); let __right_id = __right.borrow().as_ref().map(|__v| Rc::as_ptr(&__v.n) as *const () as usize); __left_id.is_some() && __left_id == __right_id }; __eq }))); let __go_print_value = println!("{} {} {} {}", format!("{}", (*(*a.borrow().as_ref().unwrap()).n.borrow().as_ref().unwrap())), format!("{}", __go_print_arg_1), format!("{}", format_slice_values(&(*(*a.borrow().as_ref().unwrap()).tags.borrow().as_ref().unwrap()).clone())), format!("{}", { let __v = (*__go_print_arg_2.borrow().as_ref().unwrap()).clone(); __v })); __go_print_value };

        // (*a).Self() is a itself: replacing the pointee through it changes a.
    { let new_val = P { n: Rc::new(RefCell::new(Some((*(*a.borrow().as_ref().unwrap()).n.borrow().as_ref().unwrap()) + 1 as i64))), tags: { let __slice_header = GoSliceHeader::go_slice_header(&((*a.borrow().as_ref().unwrap()).tags)); Rc::new(RefCell::new(__slice_header)) }, ..Default::default() }; *{ let __recv = a.clone(); let __recv_ptr = go_hint_receiver(&__recv); let __result = unsafe { &*__recv_ptr }.self_(); __result }.borrow_mut() = Some(new_val); };
    println!("{}", format!("{}", (*(*a.borrow().as_ref().unwrap()).n.borrow().as_ref().unwrap())));

    let mut one: Rc<RefCell<Option<[P; 1]>>> = Rc::new(RefCell::new(Some(std::array::from_fn::<_, 1, _>(|_| Default::default()))));
    { let new_val = 1; *(*one.borrow().as_ref().unwrap())[(0) as usize].clone().n.borrow_mut() = Some(new_val); };
    println!("{} {}", format!("{}", bump_first(one.clone())), format!("{}", (*(*one.borrow().as_ref().unwrap())[(0) as usize].clone().n.borrow().as_ref().unwrap())));

        // A pointer to a slice element: the value read through it is a copy,
        // and a pointer method called through it changes the element.
    let mut elems = Rc::new(RefCell::new(Some(vec![P { n: Rc::new(RefCell::new(Some(1))), ..Default::default() }])));
    let mut pe: Option<GoSliceElemPtr<P>> = Some(GoSliceElemPtr::new(elems.clone(), (0) as usize));
    println!("{} {}", format!("{}", (*(*bump(Rc::new(RefCell::new(Some({ let __v = (*pe.as_ref().unwrap().borrow().as_ref().unwrap()).__go_value_clone(); __v })))).borrow().as_ref().unwrap()).n.borrow().as_ref().unwrap())), format!("{}", (*(*elems.borrow().as_ref().unwrap())[(0) as usize].clone().n.borrow().as_ref().unwrap())));
    ({ let __v = (*pe.as_ref().unwrap().borrow().as_ref().unwrap()).clone(); __v }).inc();
    println!("{}", format!("{}", (*(*elems.borrow().as_ref().unwrap())[(0) as usize].clone().n.borrow().as_ref().unwrap())));

        // A method of an interface called through a pointer to it.
    let mut g: Rc<RefCell<Option<Box<dyn Getter>>>> = Rc::new(RefCell::new(Some(Box::new(P { n: Rc::new(RefCell::new(Some(8))), ..Default::default() }) as Box<dyn Getter>)));
    let mut pg = g.clone();
    println!("{}", format!("{}", ({ let __v = (*pg.borrow().as_ref().unwrap()).clone(); __v }).get()));
}

#[derive(Debug, Clone)]
pub struct AnonymousStruct1 {
    pub inner: Rc<RefCell<Option<P>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.inner.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        Self {
            inner: __go_clone_0_0,
        }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(P::default())));
        Self {
            inner: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.inner.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}
