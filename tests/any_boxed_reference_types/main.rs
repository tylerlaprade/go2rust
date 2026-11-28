#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use std::any::Any;
use std::cell::{RefCell};
use std::cmp::Ord;
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! print {
    ($($__arg:tt)*) => {{
        std::print!($($__arg)*);
        std::io::Write::flush(&mut std::io::stdout()).ok();
    }};
}
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


// Go expands a pointer to a struct only when the pointer is the value being
// printed. Inside a struct or a slice it prints the address. That is also
// what keeps a value that points back to itself from printing forever.
fn go_format_any_pointer(nested: bool, address: *const (), is_nil: impl FnOnce() -> bool, expand: impl FnOnce() -> String) -> String {
    if !nested {
        return expand();
    }
    if is_nil() { "<nil>".to_string() } else { format!("{:p}", address) }
}
fn format_map<K: Display + Ord + Clone, V>(map: &Rc<RefCell<Option<BTreeMap<K, Rc<RefCell<Option<V>>>>>>>) -> String
where
    V: Display + Clone,
{
    format_map_nil(map, "<nil>")
}

// format_map_nil prints a value the map holds no value for as __nil_text:
// [] for a nil slice, as Go prints one.
fn format_map_nil<K: Display + Ord + Clone, V>(map: &Rc<RefCell<Option<BTreeMap<K, Rc<RefCell<Option<V>>>>>>>, __nil_text: &str) -> String
where
    V: Display + Clone,
{
    // The entries are copied out first, in key order, so a value's String
    // method that reads this map does not wait on the lock printing holds.
    let items: Option<Vec<(K, Rc<RefCell<Option<V>>>)>> = { let guard = map.borrow(); guard.as_ref().map(|m| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect()) };
    if let Some(items) = items {
        let formatted: Vec<String> = items
            .iter()
            .map(|(k, v)| {
                // The value is copied out too, so its String method may
                // read its own entry.
                let value = { let v_guard = v.borrow(); v_guard.clone() };
                if let Some(ref val) = value {
                    format!("{}:{}", go_display_value(k), go_display_value(val))
                } else {
                    format!("{}:{}", go_display_value(k), __nil_text)
                }
            })
            .collect();

        format!("map[{}]", formatted.join(" "))
    } else {
        "map[]".to_string()
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

fn format_any(value: &dyn Any) -> String {
    format_any_at(value, false)
}

// Prints a value held inside the one being printed: a struct's interface
// field, an element of a []interface{}.
fn format_any_nested(value: &dyn Any) -> String {
    format_any_at(value, true)
}

fn format_any_at(value: &dyn Any, nested: bool) -> String {
    if let Some(v) = value.downcast_ref::<i32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f64>() {
        go_float_string(*v, false)
    } else if let Some(v) = value.downcast_ref::<f32>() {
        go_float_string(*v as f64, true)
    } else if let Some(v) = value.downcast_ref::<String>() {
        v.clone()
    } else if let Some(v) = value.downcast_ref::<&str>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<bool>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i8>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i16>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<isize>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u8>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u16>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<usize>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<GoNilAny>() {
        "<nil>".to_string()
    } else if let Some(v) = value.downcast_ref::<IntList>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<Point>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<bool>>>>>>>>() {
        format_map(v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>() {
        format_map(v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Point>>>>() {
        go_format_any_pointer(nested, Rc::as_ptr(v) as *const (), || v.try_borrow().map(|__guard| __guard.is_none()).unwrap_or(false), || match (*v.borrow()).as_ref() { Some(__pointee) => format!("&{}", __pointee), None => "<nil>".to_string() })
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<String>>>>>() {
        format_slice(v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<f64>>>>>() {
        format_slice(v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<i64>>>>>() {
        format_slice(v)
    } else if let Some(v) = value.downcast_ref::<Registry>() {
        format!("{}", v)
    } else {
        panic!("go2rust: cannot print this value held in an interface: its type has no formatting case yet")
    }
}

/// A nil interface value in a slot that holds a Box<dyn Any>, such as a
/// []any element: it copies, prints and compares as nil.
#[derive(Clone, Copy, Debug, PartialEq)]
struct GoNilAny;

/// The Go func value a func-typed expression holds, or None for a nil func.
/// A copy of a func value is a clone of the same closure.
trait GoFuncValue {
    type Func: ?Sized;
    fn go_func_value(&self) -> Option<std::rc::Rc<Self::Func>>;
}

macro_rules! impl_go_func_value {
    ($(($($arg:ident),*)),* $(,)?) => { $(
        impl<R: 'static, $($arg: 'static),*> GoFuncValue for std::rc::Rc<dyn Fn($($arg),*) -> R> {
            type Func = dyn Fn($($arg),*) -> R;
            fn go_func_value(&self) -> Option<std::rc::Rc<Self::Func>> { Some(self.clone()) }
        }
        impl<R: 'static, $($arg: 'static),*> GoFuncValue for Option<std::rc::Rc<dyn Fn($($arg),*) -> R>> {
            type Func = dyn Fn($($arg),*) -> R;
            fn go_func_value(&self) -> Option<std::rc::Rc<Self::Func>> { self.clone() }
        }
        impl<R: 'static, $($arg: 'static),*> GoFuncValue for std::rc::Rc<std::cell::RefCell<Option<std::rc::Rc<dyn Fn($($arg),*) -> R>>>> {
            type Func = dyn Fn($($arg),*) -> R;
            fn go_func_value(&self) -> Option<std::rc::Rc<Self::Func>> { self.borrow().clone() }
        }
    )* };
}
impl_go_func_value!((), (A0), (A0, A1), (A0, A1, A2), (A0, A1, A2, A3), (A0, A1, A2, A3, A4), (A0, A1, A2, A3, A4, A5), (A0, A1, A2, A3, A4, A5, A6), (A0, A1, A2, A3, A4, A5, A6, A7), (A0, A1, A2, A3, A4, A5, A6, A7, A8), (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9), (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10), (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11));

fn go_any_try_clone(value: &dyn Any) -> Option<Box<dyn Any>> {
    if let Some(v) = value.downcast_ref::<i32>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<i64>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<i8>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<i16>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<u32>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<u64>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<u8>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<u16>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<usize>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<isize>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<f64>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<f32>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<&'static str>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<bool>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<char>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<String>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<GoNilAny>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<IntList>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Point>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<bool>>>>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Point>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Rc<dyn Fn() -> i64>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<String>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<f64>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<i64>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Registry>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Scale>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    None
}

fn go_any_clone(value: &dyn Any) -> Box<dyn Any> {
    go_any_try_clone(value).unwrap_or_else(|| panic!("go_any_clone: unsupported dynamic type; add typed lowering instead of cloning Box<dyn Any>"))
}

#[allow(dead_code)]
fn go_any_handle_copy(handle: &Rc<RefCell<Option<Box<dyn Any>>>>) -> Rc<RefCell<Option<Box<dyn Any>>>> {
    let __copy = match handle.borrow().as_ref() {
        None => None,
        Some(value) => match go_any_try_clone(&**value) {
            Some(copy) => Some(copy),
            None => return handle.clone(),
        },
    };
    Rc::new(RefCell::new(__copy))
}

/// Stores Go's copy of source's value in target's cell. A source that no
/// one else holds gives up its value; a value whose type has no copy comes
/// back for the caller to share instead.
#[allow(dead_code)]
fn go_any_store(target: &Rc<RefCell<Option<Box<dyn Any>>>>, source: Rc<RefCell<Option<Box<dyn Any>>>>) -> Option<Rc<RefCell<Option<Box<dyn Any>>>>> {
    if Rc::ptr_eq(target, &source) {
        return None;
    }
    let __value = if Rc::strong_count(&source) == 1 {
        source.borrow_mut().take()
    } else {
        let __copy = match source.borrow().as_ref() {
            None => None,
            Some(value) => match go_any_try_clone(&**value) {
                Some(copy) => Some(copy),
                None => return Some(source.clone()),
            },
        };
        __copy
    };
    *target.borrow_mut() = __value;
    None
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
    if val.is::<GoNilAny>() { return "<nil>" }
    if val.is::<IntList>() { return "main.IntList" }
    if val.is::<Point>() { return "main.Point" }
    if val.is::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<bool>>>>>>>>() { return "map[string]bool" }
    if val.is::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>() { return "map[string]int" }
    if val.is::<Rc<RefCell<Option<Point>>>>() { return "*main.Point" }
    if val.is::<Rc<RefCell<Option<Rc<dyn Fn() -> i64>>>>>() { return "func() int" }
    if val.is::<Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64>>>>>() { return "func(int) int" }
    if val.is::<Rc<RefCell<Option<Vec<String>>>>>() { return "[]string" }
    if val.is::<Rc<RefCell<Option<Vec<f64>>>>>() { return "[]float64" }
    if val.is::<Rc<RefCell<Option<Vec<i64>>>>>() { return "[]int" }
    if val.is::<Registry>() { return "main.Registry" }
    if val.is::<Scale>() { return "main.Scale" }
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

#[derive(Debug, Clone)]
pub struct Point {
    pub x: Rc<RefCell<Option<i64>>>,
    pub y: Rc<RefCell<Option<i64>>>,
}

impl Point {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.y.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
            y: __go_clone_0_1,
        }
    }
}


impl Default for Point {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_0_1 = Rc::new(RefCell::new(Some(0)));
        Self {
            x: __go_default_0_0,
            y: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.y.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone, Default)]
pub struct IntList(pub Rc<RefCell<Option<Vec<i64>>>>);

impl Display for IntList {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice(&self.0))
    }
}


#[derive(Debug, Clone, Default)]
pub struct Registry(pub Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>);

impl Display for Registry {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_map(&self.0))
    }
}


pub type Scale = Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64>>>>;


#[derive(Clone, Default)]
pub struct Holder {
    pub v: Rc<RefCell<Option<Box<dyn Any>>>>,
}

impl Holder {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.v.clone();
        Self {
            v: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for Holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", match self.v.borrow().as_ref() { Some(__v) => format_any_nested(__v.as_ref()), None => "<nil>".to_string() });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Clone, Default)]
pub struct Pair {
    pub left: Rc<RefCell<Option<Box<dyn Any>>>>,
    pub right: Rc<RefCell<Option<Box<dyn Any>>>>,
}

impl Pair {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.left.clone();
        let __go_clone_0_1 = self.right.clone();
        Self {
            left: __go_clone_0_0,
            right: __go_clone_0_1,
        }
    }
}

impl std::fmt::Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", match self.left.borrow().as_ref() { Some(__v) => format_any_nested(__v.as_ref()), None => "<nil>".to_string() });
        let __go_fmt_1 = format!("{}", match self.right.borrow().as_ref() { Some(__v) => format_any_nested(__v.as_ref()), None => "<nil>".to_string() });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct Shelf {
    pub items: Rc<RefCell<Option<IntList>>>,
}

impl Shelf {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.items.clone();
        Self {
            items: __go_clone_0_0,
        }
    }
}


impl Default for Shelf {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(Default::default())));
        Self {
            items: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Shelf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.items.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Holder {
    pub fn set(&mut self, v: Rc<RefCell<Option<Box<dyn Any>>>>) {
        { let new_val = go_any_handle_copy(&v); self.v = new_val; };
    }
}

/// classify reports the dynamic type of a boxed value. A pointer, slice, map
/// or function has to come back out of the interface as the same kind of thing
/// that went in.
pub fn classify(value: Rc<RefCell<Option<Box<dyn Any>>>>) -> Rc<RefCell<Option<String>>> {
    {
    let _ts_subject = value.clone();
    let _ts_guard = _ts_subject.borrow();
    let _ts_is_nil = _ts_guard.as_ref().map_or(true, |__v| (**__v).is::<GoNilAny>());
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| {
        let mut __any = __v.as_ref() as &dyn Any;
        while let Some(__boxed) = __any.downcast_ref::<Box<dyn Any>>() {
            __any = __boxed.as_ref() as &dyn Any;
        }
        __any
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<i64>()).is_some() {
        let v = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<i64>()).unwrap().clone())));
        std::mem::drop(_ts_guard);
        return Rc::new(RefCell::new(Some(format!("int {}", { let __v = (*v.borrow().as_ref().unwrap()).clone(); __v }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<String>()).is_some() {
        let v = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<String>()).unwrap().clone())));
        std::mem::drop(_ts_guard);
        return Rc::new(RefCell::new(Some(format!("{}{}", "string ".to_string(), (*v.borrow().as_ref().unwrap())))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Vec<i64>>>>>()).is_some() {
        let v = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Vec<i64>>>>>()).unwrap().clone();
        std::mem::drop(_ts_guard);
        let mut v = { let __slice_header = GoSliceHeader::go_slice_header(&v); Rc::new(RefCell::new(__slice_header)) };
        return Rc::new(RefCell::new(Some(format!("[]int len {}", ({ let __len = (*v.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Vec<String>>>>>()).is_some() {
        let v = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Vec<String>>>>>()).unwrap().clone();
        std::mem::drop(_ts_guard);
        let mut v = { let __slice_header = GoSliceHeader::go_slice_header(&v); Rc::new(RefCell::new(__slice_header)) };
        return Rc::new(RefCell::new(Some(format!("[]string len {}", ({ let __len = (*v.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>()).is_some() {
        let v = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>()).unwrap().clone();
        std::mem::drop(_ts_guard);
        return Rc::new(RefCell::new(Some(format!("map len {}", ({ let __len = (*v.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Point>>>>()).is_some() {
        let v = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Point>>>>()).unwrap().clone();
        std::mem::drop(_ts_guard);
        return Rc::new(RefCell::new(Some(format!("*Point {}", (*(*v.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Point>()).is_some() {
        let v = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<Point>()).unwrap().__go_value_clone())));
        std::mem::drop(_ts_guard);
        return Rc::new(RefCell::new(Some(format!("Point {}", (*(*v.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<IntList>()).is_some() {
        let v = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<IntList>()).unwrap().clone())));
        std::mem::drop(_ts_guard);
        return Rc::new(RefCell::new(Some(format!("IntList len {}", { let __slice_holder = { let __named_slice = (*v.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Registry>()).is_some() {
        let v = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<Registry>()).unwrap().clone())));
        std::mem::drop(_ts_guard);
        return Rc::new(RefCell::new(Some(format!("Registry len {}", { let __named_map_holder = v.clone(); let __named_map_guard = __named_map_holder.borrow(); let __map_holder = __named_map_guard.as_ref().map(|__v| __v.0.clone()); std::mem::drop(__named_map_guard); __map_holder.as_ref().map(|__map_holder| { let __map_guard = __map_holder.borrow(); __map_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }).unwrap_or(0) }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Rc<dyn Fn() -> i64>>>>>()).is_some() {
        let v = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Rc<dyn Fn() -> i64>>>>>()).unwrap().clone();
        std::mem::drop(_ts_guard);
        return Rc::new(RefCell::new(Some(format!("func {}", { let __f: Rc<dyn Fn() -> i64> = v.borrow().as_ref().unwrap().clone(); (__f)() }))));;
    } else if _ts_is_nil {
        let v = _ts_subject.clone();
        std::mem::drop(_ts_guard);
        return Rc::new(RefCell::new(Some("nil".to_string())));;
    } else {
        let v = _ts_subject.clone();
        std::mem::drop(_ts_guard);
        return Rc::new(RefCell::new(Some(format!("other {}", match v.borrow().as_ref() { Some(__any_value) => __go_type_name(&**__any_value), None => "<nil>" }))));;
    }
    }
    unreachable!()
}

pub fn type_name(value: Rc<RefCell<Option<Box<dyn Any>>>>) -> Rc<RefCell<Option<String>>> {
    Rc::new(RefCell::new(Some(format!("{}", match value.borrow().as_ref() { Some(__any_value) => __go_type_name(&**__any_value), None => "<nil>" }))))
}

pub fn box_slice(xs: Rc<RefCell<Option<Vec<i64>>>>) -> Rc<RefCell<Option<Box<dyn Any>>>> {
    Rc::new(RefCell::new(Some(Box::new(xs.clone()) as Box<dyn Any>)))
}

pub fn box_with_error(xs: Rc<RefCell<Option<Vec<i64>>>>) -> (Rc<RefCell<Option<Box<dyn Any>>>>, Rc<RefCell<Option<Rc<dyn StdError>>>>) {
    (Rc::new(RefCell::new(Some(Box::new(xs.clone()) as Box<dyn Any>))), Rc::new(RefCell::new(None)))
}

pub fn box_named_result(xs: Rc<RefCell<Option<Vec<i64>>>>) -> Rc<RefCell<Option<Box<dyn Any>>>> {
    let mut result: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(None));

    { let new_val = Box::new(xs.clone()) as Box<dyn Any>; *result.borrow_mut() = Some(new_val); };
    result.clone()
}

pub fn classify_all(args: Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> Rc<RefCell<Option<String>>> {
    let mut out = Rc::new(RefCell::new(Some("".to_string())));
    { let __range_holder = args.clone(); let __range_guard = __range_holder.borrow(); let __range_len = __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); std::mem::drop(__range_guard); let __range_values = 0..__range_len; for arg in __range_values.map(|__range_index| { let __range_guard = __range_holder.borrow(); go_any_clone(__range_guard.as_ref().unwrap()[__range_index].as_ref()) }) {
        {
    let _ts_ref = arg;
    let _ts_val: Option<&dyn Any> = Some(_ts_ref.as_ref() as &dyn Any);
    let _ts_is_nil = _ts_val.map_or(false, |__v| __v.is::<GoNilAny>());
    if _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Vec<i64>>>>>()).is_some() {
        { (*out.borrow_mut().as_mut().unwrap()).push_str(&"[]int ".to_string()); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>()).is_some() {
        { (*out.borrow_mut().as_mut().unwrap()).push_str(&"map ".to_string()); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Point>>>>()).is_some() {
        { (*out.borrow_mut().as_mut().unwrap()).push_str(&"*Point ".to_string()); };;
    } else {
        { (*out.borrow_mut().as_mut().unwrap()).push_str(&"other ".to_string()); };;
    }
    }
    } }
    return { let __owned = out.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
}

pub fn box_pointer(p: Rc<RefCell<Option<Point>>>) -> Rc<RefCell<Option<Box<dyn Any>>>> {
    Rc::new(RefCell::new(Some(Box::new(p.clone()) as Box<dyn Any>)))
}

pub fn sum(xs: Rc<RefCell<Option<Vec<i64>>>>) -> i64 {
    let mut total = Rc::new(RefCell::new(Some(0i64)));
    { let __range_holder = xs.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for x in __range_values.iter().copied() {
        { let __rhs = x; let mut guard = total.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }
    return (*total.borrow().as_ref().unwrap());
}

fn main() {
    go_install_panic_exit();
    let mut xs = Rc::new(RefCell::new(Some(vec![1i64, 2i64, 3i64])));
    let mut names = Rc::new(RefCell::new(Some(vec!["a".to_string(), "b".to_string()])));
    let mut counts = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i64>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(1i64))))]))));
    let mut p = Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(7))), y: Rc::new(RefCell::new(Some(8))), ..Default::default() })));
    let mut pv = Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(9))), y: Rc::new(RefCell::new(Some(10))), ..Default::default() })));
    let mut list = Rc::new(RefCell::new(Some(IntList(Rc::new(RefCell::new(Some(vec![4i64, 5i64])))))));
    let mut reg = Rc::new(RefCell::new(Some(Registry(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i64>>>>::from([("x".to_string(), Rc::new(RefCell::new(Some(1i64)))), ("y".to_string(), Rc::new(RefCell::new(Some(2i64))))]))))))));

        // Every way of putting a value into an interface.
    println!("{} {} {} {} {} {}", format!("{}", "argument:".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(xs.clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(names.clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(counts.clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    println!("{} {} {} {} {} {}", format!("{}", "argument:".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(p.clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some({ let __arg_value = Box::new({ let __arg_holder = pv.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }) as Box<dyn Any>; __arg_value })))).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(Rc::new(move || -> i64 {
        5
    }) as Rc<dyn Fn() -> i64>))).clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    println!("{} {} {} {} {} {}", format!("{}", "literal:".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(vec![4i64, 5i64]))).clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(1))), y: Rc::new(RefCell::new(Some(2))), ..Default::default() }))).clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i64>>>>::from([])))).clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    println!("{} {} {} {} {} {}", format!("{}", "defined:".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some({ let __arg_value = Box::new((*list.borrow()).clone().unwrap_or_default()) as Box<dyn Any>; __arg_value })))).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some({ let __arg_value = Box::new((*reg.borrow()).clone().unwrap_or_default()) as Box<dyn Any>; __arg_value })))).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(IntList(Rc::new(RefCell::new(Some(vec![6i64])))).clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    println!("{} {} {} {} {} {}", format!("{}", "scalar:".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(42i64) as Box<dyn Any>)))).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new("hi".to_string()) as Box<dyn Any>)))).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(3.5) as Box<dyn Any>)))).borrow().as_ref().unwrap())));

    let mut declared: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(xs.clone()) as Box<dyn Any>)));
    println!("{} {}", format!("{}", "declared:".to_string()), format!("{}", (*classify(go_any_handle_copy(&declared)).borrow().as_ref().unwrap())));
    let mut assigned: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(None));
    println!("{} {}", format!("{}", "unset:".to_string()), format!("{}", (*classify(go_any_handle_copy(&assigned)).borrow().as_ref().unwrap())));
    { let new_val = Box::new(counts.clone()) as Box<dyn Any>; *assigned.borrow_mut() = Some(new_val); };
    println!("{} {}", format!("{}", "assigned:".to_string()), format!("{}", (*classify(go_any_handle_copy(&assigned)).borrow().as_ref().unwrap())));
    { let new_val = Box::new(p.clone()) as Box<dyn Any>; *assigned.borrow_mut() = Some(new_val); };
    println!("{} {}", format!("{}", "assigned:".to_string()), format!("{}", (*classify(go_any_handle_copy(&assigned)).borrow().as_ref().unwrap())));
    { let new_val = Box::new({ let __arg_holder = pv.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }) as Box<dyn Any>; *assigned.borrow_mut() = Some(new_val); };
    println!("{} {}", format!("{}", "assigned:".to_string()), format!("{}", (*classify(go_any_handle_copy(&assigned)).borrow().as_ref().unwrap())));
    println!("{} {} {} {}", format!("{}", "returned:".to_string()), format!("{}", (*classify(box_slice(xs.clone()).clone()).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*classify(box_pointer(p.clone()).clone()).borrow().as_ref().unwrap())));

    let (mut both, _) = box_with_error(xs.clone());
    println!("{} {} {} {}", format!("{}", "returned with error:".to_string()), format!("{}", (*classify(go_any_handle_copy(&both)).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*classify(box_named_result(xs.clone()).clone()).borrow().as_ref().unwrap())));
    println!("{} {} {} {} {} {}", format!("{}", "converted:".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(xs.clone()) as Box<dyn Any>))).clone()).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(counts.clone()) as Box<dyn Any>))).clone()).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(p.clone()) as Box<dyn Any>))).clone()).borrow().as_ref().unwrap())));
    println!("{} {}", format!("{}", "variadic:".to_string()), format!("{}", (*classify_all(Rc::new(RefCell::new(Some(vec![Box::new(xs.clone()) as Box<dyn Any>, Box::new(p.clone()) as Box<dyn Any>, Box::new(counts.clone()) as Box<dyn Any>, Box::new(1i64) as Box<dyn Any>])))).borrow().as_ref().unwrap())));
    let mut pair = Rc::new(RefCell::new(Some(Pair { left: Rc::new(RefCell::new(Some(Box::new(xs.clone()) as Box<dyn Any>))), right: Rc::new(RefCell::new(Some(Box::new(p.clone()) as Box<dyn Any>))), ..Default::default() })));
    println!("{} {} {} {}", format!("{}", "positional field:".to_string()), format!("{}", (*classify(go_any_handle_copy(&(*pair.borrow().as_ref().unwrap()).left)).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*classify(go_any_handle_copy(&(*pair.borrow().as_ref().unwrap()).right)).borrow().as_ref().unwrap())));
    let mut viaMethod = Rc::new(RefCell::new(Some(Holder { v: Default::default() })));
    (*viaMethod.borrow_mut().as_mut().unwrap()).set(Rc::new(RefCell::new(Some(Box::new(counts.clone()) as Box<dyn Any>))));
    println!("{} {}", format!("{}", "method argument:".to_string()), format!("{}", (*classify(go_any_handle_copy(&(*viaMethod.borrow().as_ref().unwrap()).v)).borrow().as_ref().unwrap())));
    let xs_closure_clone = xs.clone(); let mut produce = Rc::new(RefCell::new(Some(Rc::new(move || -> Rc<RefCell<Option<Box<dyn Any>>>> {
        return Rc::new(RefCell::new(Some(Box::new(xs_closure_clone.clone()) as Box<dyn Any>)));
    }) as Rc<dyn Fn() -> Rc<RefCell<Option<Box<dyn Any>>>>>)));
    println!("{} {}", format!("{}", "closure result:".to_string()), format!("{}", (*classify({ let __f: Rc<dyn Fn() -> Rc<RefCell<Option<Box<dyn Any>>>>> = produce.borrow().as_ref().unwrap().clone(); (__f)() }.clone()).borrow().as_ref().unwrap())));

        // A nil value of a defined slice type is still a value of that type.
    let mut noList: Rc<RefCell<Option<IntList>>> = Rc::new(RefCell::new(Some(Default::default())));
    let mut shelf = Rc::new(RefCell::new(Some(Shelf { items: Rc::new(RefCell::new(Some(Default::default()))) })));
    println!("{} {} {} {}", format!("{}", "nil defined:".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some({ let __arg_value = Box::new((*noList.borrow()).clone().unwrap_or_default()) as Box<dyn Any>; __arg_value })))).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some({ let __arg_value = Box::new((*(*shelf.borrow().as_ref().unwrap()).items.borrow()).clone().unwrap_or_default()) as Box<dyn Any>; __arg_value })))).borrow().as_ref().unwrap())));
    { let new_val = IntList(Rc::new(RefCell::new(Some(vec![1i64])))); *(*shelf.borrow().as_ref().unwrap()).items.borrow_mut() = Some(new_val); };
    println!("{} {}", format!("{}", "defined field:".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some({ let __arg_value = Box::new((*(*shelf.borrow().as_ref().unwrap()).items.borrow()).clone().unwrap_or_default()) as Box<dyn Any>; __arg_value })))).borrow().as_ref().unwrap())));

    let mut holder = Rc::new(RefCell::new(Some(Holder { v: Rc::new(RefCell::new(Some(Box::new(xs.clone()) as Box<dyn Any>))), ..Default::default() })));
    println!("{} {}", format!("{}", "field:".to_string()), format!("{}", (*classify(go_any_handle_copy(&(*holder.borrow().as_ref().unwrap()).v)).borrow().as_ref().unwrap())));
    { let new_val = Box::new(p.clone()) as Box<dyn Any>; *(*holder.borrow().as_ref().unwrap()).v.borrow_mut() = Some(new_val); };
    println!("{} {}", format!("{}", "field:".to_string()), format!("{}", (*classify(go_any_handle_copy(&(*holder.borrow().as_ref().unwrap()).v)).borrow().as_ref().unwrap())));

    let mut byName = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Box<dyn Any>>>>>::from([("xs".to_string(), Rc::new(RefCell::new(Some(Box::new(xs.clone()) as Box<dyn Any>)))), ("p".to_string(), Rc::new(RefCell::new(Some(Box::new(p.clone()) as Box<dyn Any>))))]))));
    println!("{} {} {} {}", format!("{}", "map value:".to_string()), format!("{}", (*classify(go_any_handle_copy(&{ let __map_holder = byName.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"xs".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) })).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*classify(go_any_handle_copy(&{ let __map_holder = byName.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"p".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) })).borrow().as_ref().unwrap())));

    let mut items = Rc::new(RefCell::new(Some(vec![Box::new(xs.clone()) as Box<dyn Any>, Box::new(counts.clone()) as Box<dyn Any>, Box::new(p.clone()) as Box<dyn Any>, Box::new((*pv.borrow().as_ref().unwrap()).__go_value_clone()) as Box<dyn Any>, Box::new((*list.borrow()).clone().unwrap_or_default()) as Box<dyn Any>, Box::new(7i64) as Box<dyn Any>])));
    { let __range_holder = items.clone(); let __range_guard = __range_holder.borrow(); let __range_len = __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); std::mem::drop(__range_guard); let __range_values = 0..__range_len; for item in __range_values.map(|__range_index| { let __range_guard = __range_holder.borrow(); go_any_clone(__range_guard.as_ref().unwrap()[__range_index].as_ref()) }) {
        {
    let _ts_ref = item;
    let _ts_val: Option<&dyn Any> = Some(_ts_ref.as_ref() as &dyn Any);
    let _ts_is_nil = _ts_val.map_or(false, |__v| __v.is::<GoNilAny>());
    if _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Vec<i64>>>>>()).is_some() {
        let v = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Vec<i64>>>>>()).unwrap().clone();
        let mut v = { let __slice_header = GoSliceHeader::go_slice_header(&v); Rc::new(RefCell::new(__slice_header)) };
        println!("{} {}", format!("{}", "element: []int".to_string()), format!("{}", ({ let __len = (*v.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>()).is_some() {
        let v = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>()).unwrap().clone();
        println!("{} {}", format!("{}", "element: map".to_string()), format!("{}", ({ let __len = (*v.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Point>>>>()).is_some() {
        let v = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Point>>>>()).unwrap().clone();
        println!("{} {}", format!("{}", "element: *Point".to_string()), format!("{}", (*(*v.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Point>()).is_some() {
        let v = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<Point>()).unwrap().__go_value_clone())));
        println!("{} {}", format!("{}", "element: Point".to_string()), format!("{}", (*(*v.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<IntList>()).is_some() {
        let v = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<IntList>()).unwrap().clone())));
        println!("{} {}", format!("{}", "element: IntList".to_string()), format!("{}", { let __slice_holder = { let __named_slice = (*v.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }));;
    } else {
        let v = _ts_val.unwrap();
        println!("{}", format!("{}", "element: other".to_string()));;
    }
    }
    } }
    { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = items.clone(); let __append_value = Box::new(names.clone()) as Box<dyn Any>; let mut __appended: Vec<Box<dyn Any>> = (*__append_target.borrow()).clone().unwrap_or_default(); __appended.push(__append_value); Rc::new(RefCell::new(Some(__appended))) })); *items.borrow_mut() = __slice_header; };
    let mut appended = Rc::new(RefCell::new(Some(({
        let __idx = ((({ let __len = (*items.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) - (1 as i64)) as usize;
        let __seq_holder = items.clone();
        let __seq_guard = __seq_holder.borrow();
        let any_val = __seq_guard.as_ref().expect("nil []any in type assertion")[__idx].as_ref();
        { let __handle = any_val.downcast_ref::<Rc<RefCell<Option<Vec<String>>>>>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(any_val), "[]string")).clone(); let __value = (*__handle.borrow()).clone().unwrap_or_default(); __value }
    }))));
    let mut appended = { let __slice_header = GoSliceHeader::go_slice_header(&appended); Rc::new(RefCell::new(__slice_header)) };
    println!("{} {} {}", format!("{}", "appended:".to_string()), format!("{}", ({ let __len = (*appended.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", (*appended.borrow().as_ref().unwrap())[(0) as usize].clone()));

        // Assertions, in both forms.
    {
        let (mut got, mut ok) = ({
        let val = declared.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Vec<i64>>>>>() {
                (typed_val.clone(), true)
            } else {
                (Rc::new(RefCell::new(None::<Vec<i64>>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<Vec<i64>>)), false)
        }
    });
let mut got = { let __slice_header = GoSliceHeader::go_slice_header(&got); Rc::new(RefCell::new(__slice_header)) };;
        if ok {
            println!("{} {} {}", format!("{}", "comma-ok slice:".to_string()), format!("{}", ({ let __len = (*got.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", (*got.borrow().as_ref().unwrap())[(0) as usize].clone()));;
        }
    }
    {
        let (_, mut ok) = ({
        let val = declared.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Vec<String>>>>>() {
                (typed_val.clone(), true)
            } else {
                (Rc::new(RefCell::new(None::<Vec<String>>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<Vec<String>>)), false)
        }
    });;
        if !ok {
            println!("{}", format!("{}", "comma-ok slice: not []string".to_string()));;
        }
    }
    {
        let (_, mut ok) = ({
        let val = declared.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<IntList>() {
                (Rc::new(RefCell::new(Some(typed_val.clone()))), true)
            } else {
                (Rc::new(RefCell::new(Some(Default::default()))), false)
            }
        } else {
            (Rc::new(RefCell::new(Some(Default::default()))), false)
        }
    });;
        if !ok {
            println!("{}", format!("{}", "comma-ok slice: not IntList".to_string()));;
        }
    }
    let mut boxedMap: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(counts.clone()) as Box<dyn Any>)));
    {
        let (mut got, mut ok) = ({
        let val = boxedMap.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>() {
                (typed_val.clone(), true)
            } else {
                (Rc::new(RefCell::new(None::<BTreeMap<String, Rc<RefCell<Option<i64>>>>>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<BTreeMap<String, Rc<RefCell<Option<i64>>>>>)), false)
        }
    });;
        if ok {
            println!("{} {}", format!("{}", "comma-ok map:".to_string()), format!("{}", { let __map_holder = got.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));;
        }
    }
    let mut boxedPointer: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(p.clone()) as Box<dyn Any>)));
    {
        let (mut got, mut ok) = ({
        let val = boxedPointer.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Point>>>>() {
                (typed_val.clone(), true)
            } else {
                (Rc::new(RefCell::new(None::<Point>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<Point>)), false)
        }
    });;
        if ok {
            println!("{} {}", format!("{}", "comma-ok pointer:".to_string()), format!("{}", (*(*got.borrow().as_ref().unwrap()).y.borrow().as_ref().unwrap())));;
        }
    }
    {
        let (_, mut ok) = ({
        let val = boxedPointer.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Point>() {
                (Rc::new(RefCell::new(Some(typed_val.__go_value_clone()))), true)
            } else {
                (Rc::new(RefCell::new(Some(Default::default()))), false)
            }
        } else {
            (Rc::new(RefCell::new(Some(Default::default()))), false)
        }
    });;
        if !ok {
            println!("{}", format!("{}", "comma-ok pointer: not Point".to_string()));;
        }
    }
    let mut boxedList: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new((*list.borrow()).clone().unwrap_or_default()) as Box<dyn Any>)));
    {
        let (mut got, mut ok) = ({
        let val = boxedList.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<IntList>() {
                (Rc::new(RefCell::new(Some(typed_val.clone()))), true)
            } else {
                (Rc::new(RefCell::new(Some(Default::default()))), false)
            }
        } else {
            (Rc::new(RefCell::new(Some(Default::default()))), false)
        }
    });;
        if ok {
            println!("{} {}", format!("{}", "comma-ok defined:".to_string()), format!("{}", { let __slice_holder = { let __named_slice = (*got.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }));;
        }
    }
    {
        let (_, mut ok) = ({
        let val = boxedList.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Vec<i64>>>>>() {
                (typed_val.clone(), true)
            } else {
                (Rc::new(RefCell::new(None::<Vec<i64>>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<Vec<i64>>)), false)
        }
    });;
        if !ok {
            println!("{}", format!("{}", "comma-ok defined: not []int".to_string()));;
        }
    }

    let mut plain = ({ let val = declared.clone(); let guard = val.borrow(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Rc<RefCell<Option<Vec<i64>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } });
    let mut plain = { let __slice_header = GoSliceHeader::go_slice_header(&plain); Rc::new(RefCell::new(__slice_header)) };
    { let __go_print_arg_1 = Rc::new(RefCell::new(Some(({ let __len = (*plain.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64))); let __go_print_arg_2 = sum(Rc::new(RefCell::new(Some({ let __arg_value = ({
        let val = declared.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            { let __handle = any_val.downcast_ref::<Rc<RefCell<Option<Vec<i64>>>>>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "[]int")).clone(); let __value = (*__handle.borrow()).clone().unwrap_or_default(); __value }
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "[]int")
        }
    }); __arg_value })))); let __go_print_value = println!("{} {} {} {}", format!("{}", "single slice:".to_string()), format!("{}", { let __v = (*__go_print_arg_1.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", (*plain.borrow().as_ref().unwrap())[(2) as usize].clone()), format!("{}", __go_print_arg_2)); __go_print_value };
    let mut lookup = ({ let val = boxedMap.clone(); let guard = val.borrow(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } });
    println!("{} {} {}", format!("{}", "single map:".to_string()), format!("{}", ({ let __len = (*lookup.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", { let __map_holder = lookup.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));
    let mut first = Rc::new(RefCell::new(Some(({
        let __idx = (0) as usize;
        let __seq_holder = items.clone();
        let __seq_guard = __seq_holder.borrow();
        let any_val = __seq_guard.as_ref().expect("nil []any in type assertion")[__idx].as_ref();
        { let __handle = any_val.downcast_ref::<Rc<RefCell<Option<Vec<i64>>>>>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(any_val), "[]int")).clone(); let __value = (*__handle.borrow()).clone().unwrap_or_default(); __value }
    }))));
    let mut first = { let __slice_header = GoSliceHeader::go_slice_header(&first); Rc::new(RefCell::new(__slice_header)) };
    let mut second = Rc::new(RefCell::new(Some(({
        let __idx = (1) as usize;
        let __seq_holder = items.clone();
        let __seq_guard = __seq_holder.borrow();
        let any_val = __seq_guard.as_ref().expect("nil []any in type assertion")[__idx].as_ref();
        { let __handle = any_val.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(any_val), "map[string]int")).clone(); let __value = (*__handle.borrow()).clone().unwrap_or_default(); __value }
    }))));
    let mut third = ({
        let __idx = (2) as usize;
        let __seq_holder = items.clone();
        let __seq_guard = __seq_holder.borrow();
        let any_val = __seq_guard.as_ref().expect("nil []any in type assertion")[__idx].as_ref();
        any_val.downcast_ref::<Rc<RefCell<Option<Point>>>>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(any_val), "*main.Point")).clone()
    }).clone();
    println!("{} {} {} {}", format!("{}", "single element:".to_string()), format!("{}", ({ let __len = (*first.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", { let __map_holder = second.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", (*(*third.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())));
    let mut empty: Rc<RefCell<Option<Vec<i64>>>> = Rc::new(RefCell::new(None));
    let mut boxedEmpty: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(empty.clone()) as Box<dyn Any>)));
    let mut nothing = ({ let val = boxedEmpty.clone(); let guard = val.borrow(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Rc<RefCell<Option<Vec<i64>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } });
    let mut nothing = { let __slice_header = GoSliceHeader::go_slice_header(&nothing); Rc::new(RefCell::new(__slice_header)) };
    println!("{} {}", format!("{}", "single nil slice:".to_string()), format!("{}", ({ let __len = (*nothing.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })));

        // The boxed value is the same slice and the same pointee, not a copy.
    {
        let (mut shared, mut ok) = ({
        let val = declared.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Vec<i64>>>>>() {
                (typed_val.clone(), true)
            } else {
                (Rc::new(RefCell::new(None::<Vec<i64>>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<Vec<i64>>)), false)
        }
    });
let mut shared = { let __slice_header = GoSliceHeader::go_slice_header(&shared); Rc::new(RefCell::new(__slice_header)) };;
        if ok {
            { let __idx = (0) as usize; let __rhs = 99; (*shared.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };;
        }
    }
    println!("{} {}", format!("{}", "shared slice:".to_string()), format!("{}", (*xs.borrow().as_ref().unwrap())[(0) as usize].clone()));
    {
    let _ts_subject = boxedPointer.clone();
    let _ts_guard = _ts_subject.borrow();
    let _ts_is_nil = _ts_guard.as_ref().map_or(true, |__v| (**__v).is::<GoNilAny>());
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| {
        let mut __any = __v.as_ref() as &dyn Any;
        while let Some(__boxed) = __any.downcast_ref::<Box<dyn Any>>() {
            __any = __boxed.as_ref() as &dyn Any;
        }
        __any
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Point>>>>()).is_some() {
        let shared = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Point>>>>()).unwrap().clone();
        std::mem::drop(_ts_guard);
        { let new_val = 70; *(*shared.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };;
    }
    }
    println!("{} {}", format!("{}", "shared pointer:".to_string()), format!("{}", (*(*p.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())));

        // A function value is called through the interface, and keeps its state.
    let mut scale: Scale = Rc::new(RefCell::new(Some(Rc::new(move |x: Rc<RefCell<Option<i64>>>| -> i64 {
        (*x.borrow().as_ref().unwrap()) * 10
    }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64>)));
    let mut boxedScale: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new({ let __func_value = (*scale.borrow()).clone(); __func_value }))) as Box<dyn Any>)));
    println!("{} {}", format!("{}", "call defined func:".to_string()), format!("{}", ({
        let __f_holder = { let val = boxedScale.clone(); let guard = val.borrow(); match guard.as_ref() { Some(any_val) => any_val.downcast_ref::<Scale>().expect("type assertion failed").clone(), None => panic!("type assertion on nil interface") } };
        (|__f_arg0| {
            let mut __f_guard = __f_holder.borrow_mut();
            let __f: &mut Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64> = __f_guard.as_mut().expect("call of nil function");
            let __f_result = (*__f)(__f_arg0);
            __f_result
        })(Rc::new(RefCell::new(Some(4))))
    })));
    let mut calls = Rc::new(RefCell::new(Some(0i64)));
    let mut calls_closure_clone = calls.clone(); let mut counter: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(Rc::new(move || -> i64 {
        let mut calls_closure_clone = calls_closure_clone.clone();
        { let mut guard = calls_closure_clone.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return (*calls_closure_clone.borrow().as_ref().unwrap());
    }) as Rc<dyn Fn() -> i64>))).clone()) as Box<dyn Any>)));
    ({
        let __f_holder = { let val = counter.clone(); let guard = val.borrow(); match guard.as_ref() { Some(any_val) => any_val.downcast_ref::<Rc<RefCell<Option<Rc<dyn Fn() -> i64>>>>>().expect("type assertion failed").clone(), None => panic!("type assertion on nil interface") } };
        (|| {
            let mut __f_guard = __f_holder.borrow_mut();
            let __f: &mut Rc<dyn Fn() -> i64> = __f_guard.as_mut().expect("call of nil function");
            let __f_result = (*__f)();
            __f_result
        })()
    });
    println!("{} {} {}", format!("{}", "call func:".to_string()), format!("{}", ({
        let __f_holder = { let val = counter.clone(); let guard = val.borrow(); match guard.as_ref() { Some(any_val) => any_val.downcast_ref::<Rc<RefCell<Option<Rc<dyn Fn() -> i64>>>>>().expect("type assertion failed").clone(), None => panic!("type assertion on nil interface") } };
        (|| {
            let mut __f_guard = __f_holder.borrow_mut();
            let __f: &mut Rc<dyn Fn() -> i64> = __f_guard.as_mut().expect("call of nil function");
            let __f_result = (*__f)();
            __f_result
        })()
    })), format!("{}", { let __v = (*calls.borrow().as_ref().unwrap()).clone(); __v }));

        // An argument may itself call the same boxed function. Go finishes the
        // inner call before the outer one starts.
    let mut step: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(Rc::new(move |n: Rc<RefCell<Option<i64>>>| -> i64 {
        println!("{} {}", format!("{}", "step".to_string()), format!("{}", { let __v = (*n.borrow().as_ref().unwrap()).clone(); __v }));
        (*n.borrow().as_ref().unwrap()) + 1
    }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64>))).clone()) as Box<dyn Any>)));
    println!("{} {}", format!("{}", "nested call:".to_string()), format!("{}", ({
        let __f_holder = { let val = step.clone(); let guard = val.borrow(); match guard.as_ref() { Some(any_val) => any_val.downcast_ref::<Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64>>>>>().expect("type assertion failed").clone(), None => panic!("type assertion on nil interface") } };
        (|__f_arg0| {
            let mut __f_guard = __f_holder.borrow_mut();
            let __f: &mut Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64> = __f_guard.as_mut().expect("call of nil function");
            let __f_result = (*__f)(__f_arg0);
            __f_result
        })(Rc::new(RefCell::new(Some({ let __arg_value = ({
        let __f_holder = { let val = step.clone(); let guard = val.borrow(); match guard.as_ref() { Some(any_val) => any_val.downcast_ref::<Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64>>>>>().expect("type assertion failed").clone(), None => panic!("type assertion on nil interface") } };
        (|__f_arg0| {
            let mut __f_guard = __f_holder.borrow_mut();
            let __f: &mut Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64> = __f_guard.as_mut().expect("call of nil function");
            let __f_result = (*__f)(__f_arg0);
            __f_result
        })(Rc::new(RefCell::new(Some(1))))
    }); __arg_value }))))
    })));

        // An element of a slice of slices, and the loop variable over one, is
        // still a []int when it is put in an interface. So is a map's value.
    let mut matrix = Rc::new(RefCell::new(Some(vec![vec![1i64], vec![2i64, 3i64]])));
    println!("{} {}", format!("{}", "element:".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some({ let __arg_value = Box::new(Rc::new(RefCell::new(Some((*matrix.borrow().as_ref().unwrap())[(1) as usize].clone().clone())))) as Box<dyn Any>; __arg_value })))).borrow().as_ref().unwrap())));
    { let __range_holder = matrix.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for row in __range_values.iter() {
        println!("{} {}", format!("{}", "range value:".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some((*row).clone())))) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    } }
    let mut firstRow: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some((*matrix.borrow().as_ref().unwrap())[(0) as usize].clone().clone())))) as Box<dyn Any>)));
    {
        let (mut row, mut ok) = ({
        let val = firstRow.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Vec<i64>>>>>() {
                (typed_val.clone(), true)
            } else {
                (Rc::new(RefCell::new(None::<Vec<i64>>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<Vec<i64>>)), false)
        }
    });
let mut row = { let __slice_header = GoSliceHeader::go_slice_header(&row); Rc::new(RefCell::new(__slice_header)) };;
        if ok {
            println!("{} {}", format!("{}", "asserted element:".to_string()), format!("{}", ({ let __len = (*row.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })));;
        }
    }
    let mut rowsByName = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Vec<i64>>>>>::from([("k".to_string(), Rc::new(RefCell::new(Some(vec![4i64, 5i64, 6i64]))))]))));
    println!("{} {}", format!("{}", "slice in map:".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some({ let __arg_value = Box::new({ let __map_holder = rowsByName.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"k".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone()) as Box<dyn Any>; __arg_value })))).borrow().as_ref().unwrap())));
    for (_, value) in { let __range_holder = rowsByName.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        println!("{} {}", format!("{}", "slice in map, range value:".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(value.clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    }
    let mut tables = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i64>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(1i64)))), ("b".to_string(), Rc::new(RefCell::new(Some(2i64))))]))))])));
    println!("{} {}", format!("{}", "slice of maps:".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some({ let __arg_value = Box::new((*tables.borrow().as_ref().unwrap())[(0) as usize].clone().clone()) as Box<dyn Any>; __arg_value })))).borrow().as_ref().unwrap())));
    { let __range_holder = tables.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for table in __range_values.iter() {
        println!("{} {}", format!("{}", "range over maps:".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(table.clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    } }

        // %T names the dynamic type, and the static type of a typed variable.
    println!("{} {} {} {}", format!("{}", "dynamic:".to_string()), format!("{}", (*type_name(Rc::new(RefCell::new(Some(Box::new(xs.clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())), format!("{}", (*type_name(Rc::new(RefCell::new(Some(Box::new(names.clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())), format!("{}", (*type_name(Rc::new(RefCell::new(Some(Box::new(counts.clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    println!("{} {} {} {}", format!("{}", "dynamic:".to_string()), format!("{}", (*type_name(Rc::new(RefCell::new(Some(Box::new(p.clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())), format!("{}", (*type_name(Rc::new(RefCell::new(Some({ let __arg_value = Box::new({ let __arg_holder = pv.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }) as Box<dyn Any>; __arg_value })))).borrow().as_ref().unwrap())), format!("{}", (*type_name(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(vec![1.0]))).clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    println!("{} {} {} {} {}", format!("{}", "dynamic:".to_string()), format!("{}", (*type_name(Rc::new(RefCell::new(Some(Box::new(7i64) as Box<dyn Any>)))).borrow().as_ref().unwrap())), format!("{}", (*type_name(Rc::new(RefCell::new(Some(Box::new("s".to_string()) as Box<dyn Any>)))).borrow().as_ref().unwrap())), format!("{}", (*type_name(Rc::new(RefCell::new(Some(Box::new(true) as Box<dyn Any>)))).borrow().as_ref().unwrap())), format!("{}", (*type_name(Rc::new(RefCell::new(Some(Box::new(2.5) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    println!("{} {} {} {}", format!("{}", "unmatched:".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(vec![1.0]))).clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<bool>>>>::from([])))).clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    let mut none: Rc<RefCell<Option<Vec<i64>>>> = Rc::new(RefCell::new(None));
    let mut nowhere: Rc<RefCell<Option<Point>>> = Rc::new(RefCell::new(None));
    print!("static: {} {} {} {} {}\n", "[]int", "map[string]int", "*main.Point", "main.Point", "main.IntList");
    print!("static nil: {} {}\n", "[]int", "*main.Point");
}