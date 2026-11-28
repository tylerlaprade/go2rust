#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use std::any::Any;
use std::cmp::Ord;
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

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
fn format_map<K: Display + Ord + Clone, V>(map: &Arc<Mutex<Option<BTreeMap<K, Arc<Mutex<Option<V>>>>>>>) -> String
where
    V: Display + Clone,
{
    format_map_nil(map, "<nil>")
}

// format_map_nil prints a value the map holds no value for as __nil_text:
// [] for a nil slice, as Go prints one.
fn format_map_nil<K: Display + Ord + Clone, V>(map: &Arc<Mutex<Option<BTreeMap<K, Arc<Mutex<Option<V>>>>>>>, __nil_text: &str) -> String
where
    V: Display + Clone,
{
    // The entries are copied out first, in key order, so a value's String
    // method that reads this map does not wait on the lock printing holds.
    let items: Option<Vec<(K, Arc<Mutex<Option<V>>>)>> = { let guard = map.lock().unwrap(); guard.as_ref().map(|m| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect()) };
    if let Some(items) = items {
        let formatted: Vec<String> = items
            .iter()
            .map(|(k, v)| {
                // The value is copied out too, so its String method may
                // read its own entry.
                let value = { let v_guard = v.lock().unwrap(); v_guard.clone() };
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
fn format_nested_slice<T, C, Inner>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[Inner]>,
    Inner: AsRef<[T]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        format_nested_slice_values(s.as_ref())
    } else {
        "[]".to_string()
    }
}
fn format_nested_slice_values<T, Inner>(slice: &[Inner]) -> String
where
    Inner: AsRef<[T]>,
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|inner| format_slice_values(inner.as_ref())).collect();
    format!("[{}]", formatted.join(" "))
}

fn format_any(value: &(dyn Any + Send + Sync)) -> String {
    format_any_at(value, false)
}

// Prints a value held inside the one being printed: a struct's interface
// field, an element of a []interface{}.
fn format_any_nested(value: &(dyn Any + Send + Sync)) -> String {
    format_any_at(value, true)
}

fn format_any_at(value: &(dyn Any + Send + Sync), nested: bool) -> String {
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
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<bool>>>>>>>>() {
        format_map(v)
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Square>>>>() {
        go_format_any_pointer(nested, Arc::as_ptr(v) as *const (), || v.try_lock().map(|__guard| __guard.is_none()).unwrap_or(false), || match (*v.lock().unwrap()).as_ref() { Some(__pointee) => format!("&{}", __pointee), None => "<nil>".to_string() })
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Vec<i64>>>>>() {
        format_slice(v)
    } else if let Some(v) = value.downcast_ref::<GoNilAny>() {
        "<nil>".to_string()
    } else if let Some(v) = value.downcast_ref::<Point>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<Square>() {
        format!("{}", v)
    } else {
        panic!("go2rust: cannot print this value held in an interface: its type has no formatting case yet")
    }
}

/// A nil interface value in a slot that holds a Box<dyn Any>, such as a
/// []any element: it copies, prints and compares as nil.
#[derive(Clone, Copy, Debug, PartialEq)]
struct GoNilAny;

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

/// The Go func value a func-typed expression holds, or None for a nil func.
/// A copy of a func value is a clone of the same closure.
trait GoFuncValue {
    type Func: ?Sized;
    fn go_func_value(&self) -> Option<std::sync::Arc<Self::Func>>;
}

macro_rules! impl_go_func_value {
    ($(($($arg:ident),*)),* $(,)?) => { $(
        impl<R: 'static, $($arg: 'static),*> GoFuncValue for std::sync::Arc<dyn Fn($($arg),*) -> R + Send + Sync> {
            type Func = dyn Fn($($arg),*) -> R + Send + Sync;
            fn go_func_value(&self) -> Option<std::sync::Arc<Self::Func>> { Some(self.clone()) }
        }
        impl<R: 'static, $($arg: 'static),*> GoFuncValue for Option<std::sync::Arc<dyn Fn($($arg),*) -> R + Send + Sync>> {
            type Func = dyn Fn($($arg),*) -> R + Send + Sync;
            fn go_func_value(&self) -> Option<std::sync::Arc<Self::Func>> { self.clone() }
        }
        impl<R: 'static, $($arg: 'static),*> GoFuncValue for std::sync::Arc<std::sync::Mutex<Option<std::sync::Arc<dyn Fn($($arg),*) -> R + Send + Sync>>>> {
            type Func = dyn Fn($($arg),*) -> R + Send + Sync;
            fn go_func_value(&self) -> Option<std::sync::Arc<Self::Func>> { self.lock().unwrap().clone() }
        }
    )* };
}
impl_go_func_value!((), (A0), (A0, A1), (A0, A1, A2), (A0, A1, A2, A3), (A0, A1, A2, A3, A4), (A0, A1, A2, A3, A4, A5), (A0, A1, A2, A3, A4, A5, A6), (A0, A1, A2, A3, A4, A5, A6, A7), (A0, A1, A2, A3, A4, A5, A6, A7, A8), (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9), (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10), (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11));

fn go_any_try_clone(value: &(dyn Any + Send + Sync)) -> Option<Box<dyn Any + Send + Sync>> {
    if let Some(v) = value.downcast_ref::<i32>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<i64>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<i8>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<i16>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<u32>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<u64>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<u8>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<u16>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<usize>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<isize>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<f64>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<f32>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<&'static str>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<bool>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<char>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<String>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<bool>>>>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Square>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Vec<i64>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<GoNilAny>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Point>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Square>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    None
}

fn go_any_clone(value: &(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync> {
    go_any_try_clone(value).unwrap_or_else(|| panic!("go_any_clone: unsupported dynamic type; add typed lowering instead of cloning Box<dyn Any>"))
}

#[allow(dead_code)]
fn go_any_handle_copy(handle: &Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
    let __copy = match handle.lock().unwrap().as_ref() {
        None => None,
        Some(value) => match go_any_try_clone(&**value) {
            Some(copy) => Some(copy),
            None => return handle.clone(),
        },
    };
    Arc::new(Mutex::new(__copy))
}

/// Stores Go's copy of source's value in target's cell. A source that no
/// one else holds gives up its value; a value whose type has no copy comes
/// back for the caller to share instead.
#[allow(dead_code)]
fn go_any_store(target: &Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>, source: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> Option<Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>> {
    if Arc::ptr_eq(target, &source) {
        return None;
    }
    let __value = if Arc::strong_count(&source) == 1 {
        source.lock().unwrap().take()
    } else {
        let __copy = match source.lock().unwrap().as_ref() {
            None => None,
            Some(value) => match go_any_try_clone(&**value) {
                Some(copy) => Some(copy),
                None => return Some(source.clone()),
            },
        };
        __copy
    };
    *target.lock().unwrap() = __value;
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
        if __text == GO_UNPRINTABLE_PANIC_VALUE {
            return format_any(&**__inner);
        }
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
    // Go prints a float in a panic as +1.500000e+000; a float32 is widened
    // to float64 first, as Go's print does.
    let __float = __payload
        .downcast_ref::<f64>()
        .copied()
        .or_else(|| __payload.downcast_ref::<f32>().map(|__value| *__value as f64));
    if let Some(__value) = __float {
        if __value.is_nan() {
            return String::from("NaN");
        }
        if __value.is_infinite() {
            return String::from(if __value > 0.0 { "+Inf" } else { "-Inf" });
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
    if val.is::<Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<bool>>>>>>>>() { return "map[string]bool" }
    if val.is::<Arc<Mutex<Option<Square>>>>() { return "*main.Square" }
    if val.is::<Arc<Mutex<Option<Vec<i64>>>>>() { return "[]int" }
    if val.is::<GoNilAny>() { return "<nil>" }
    if val.is::<Point>() { return "main.Point" }
    if val.is::<Square>() { return "main.Square" }
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

// Whether value has a String or an Error method. fmt prints such a value as
// the one text the method returns.
fn go_format_is_text(__value: &(dyn Any + Send + Sync)) -> bool {
    false
}

// The basic value a newtype such as "type Level int" holds, boxed.
fn go_format_named_basic_value(__value: &(dyn Any + Send + Sync)) -> Option<Box<dyn Any + Send + Sync>> {
    None
}

// value as %+v prints it, when it is a struct.
fn go_format_fields_value(__value: &(dyn Any + Send + Sync)) -> Option<String> {
    if let Some(__v) = __value.downcast_ref::<Point>() {
        return Some(format!("{:#}", __v));
    }
    if let Some(__v) = __value.downcast_ref::<Square>() {
        return Some(format!("{:#}", __v));
    }
    None
}

// The elements of value, boxed, when it is an array of basic values.
fn go_format_elements(__value: &(dyn Any + Send + Sync)) -> Option<Vec<Box<dyn Any + Send + Sync>>> {
    None
}

// A nil interface value passed to a format known only at run time. The
// arguments are boxes, which cannot be nil themselves.
#[allow(dead_code)]
struct GoFormatNil;

#[allow(unused_imports)]
mod __go_runtime_format {
    use super::{format_any, __go_type_name, go_format_is_text, go_format_named_basic_value, go_format_fields_value, go_format_elements, GoFormatNil, GoNilAny};
    use std::any::Any;
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::{Arc, Mutex};
    #[allow(unused_imports)]
    use super::{GoSlice, Vec};

// A verb with its flags, as fmt reads it from a format.
#[derive(Default, Clone, Copy)]
struct GoFormatSpec {
    plus: bool,
    minus: bool,
    sharp: bool,
    space: bool,
    zero: bool,
    width: Option<usize>,
    precision: Option<usize>,
}

// Pads text to the width of spec. A number passes the length of its sign so
// that zero padding goes between the sign and the digits.
fn go_format_pad(spec: &GoFormatSpec, text: String, sign_len: usize, zero_allowed: bool) -> String {
    let width = match spec.width {
        Some(width) => width,
        None => return text,
    };
    let len = text.chars().count();
    if len >= width {
        return text;
    }
    let fill = width - len;
    if spec.minus {
        return text + &" ".repeat(fill);
    }
    if spec.zero && zero_allowed {
        let (sign, digits) = text.split_at(sign_len);
        return format!("{}{}{}", sign, "0".repeat(fill), digits);
    }
    " ".repeat(fill) + &text
}

fn go_format_sign(spec: &GoFormatSpec, negative: bool) -> &'static str {
    if negative {
        "-"
    } else if spec.plus {
        "+"
    } else if spec.space {
        " "
    } else {
        ""
    }
}

// Whether strconv.IsPrint prints c as it is: a letter, mark, number,
// punctuation, symbol or the ASCII space. Rust's standard library has no
// table of Unicode categories, so this knows the control characters, the
// other spaces, the format characters in common use (the soft hyphen, the
// zero-width and direction marks, the byte order mark) and the private-use
// areas. It does not know which code points are unassigned.
fn go_format_is_print(c: char) -> bool {
    if c == ' ' {
        return true;
    }
    if c.is_control() || c.is_whitespace() {
        return false;
    }
    !matches!(c as u32, 0xad | 0x600..=0x605 | 0x61c | 0x6dd | 0x70f | 0x8e2 | 0x180e | 0x200b..=0x200f | 0x202a..=0x202e | 0x2060..=0x206f | 0xfeff | 0xfff9..=0xfffb | 0xe000..=0xf8ff | 0xe0001 | 0xe0020..=0xe007f | 0xf0000..=0xffffd | 0x100000..=0x10fffd)
}

// Writes text the way strconv.Quote writes the inside of a quoted string,
// with quote as the delimiter. ascii is the + flag: every character outside
// ASCII is written as an escape.
fn go_format_escape(out: &mut String, text: &str, quote: char, ascii: bool) {
    for c in text.chars() {
        match c {
            '\u{7}' => out.push_str("\\a"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{b}' => out.push_str("\\v"),
            '\\' => out.push_str("\\\\"),
            c if c == quote => {
                out.push('\\');
                out.push(c);
            }
            c if (c as u32) < 0x20 || c == '\u{7f}' => out.push_str(&format!("\\x{:02x}", c as u32)),
            c if !go_format_is_print(c) || (ascii && !c.is_ascii()) => {
                if (c as u32) < 0x10000 {
                    out.push_str(&format!("\\u{:04x}", c as u32));
                } else {
                    out.push_str(&format!("\\U{:08x}", c as u32));
                }
            }
            c => out.push(c),
        }
    }
}

fn go_format_quote(text: &str, quote: char, ascii: bool) -> String {
    let mut out = String::new();
    out.push(quote);
    go_format_escape(&mut out, text, quote, ascii);
    out.push(quote);
    out
}

fn go_format_integer(spec: &GoFormatSpec, verb: char, value: i128) -> Option<String> {
    let negative = value < 0;
    let magnitude = value.unsigned_abs();
    let mut digits = match verb {
        'd' | 'v' => magnitude.to_string(),
        'b' => format!("{:b}", magnitude),
        'o' | 'O' => format!("{:o}", magnitude),
        'x' => format!("{:x}", magnitude),
        'X' => format!("{:X}", magnitude),
        'c' => {
            let c = u32::try_from(value).ok().and_then(char::from_u32).unwrap_or('\u{fffd}');
            return Some(go_format_pad(spec, c.to_string(), 0, true));
        }
        'q' => {
            let c = u32::try_from(value).ok().and_then(char::from_u32).unwrap_or('\u{fffd}');
            return Some(go_format_pad(spec, go_format_quote(&c.to_string(), '\'', spec.plus), 0, true));
        }
        'U' => {
            // A precision above four asks for more digits.
            let digits = spec.precision.filter(|precision| *precision > 4).unwrap_or(4);
            let mut text = format!("U+{:0digits$X}", value as u64, digits = digits);
            if spec.sharp {
                if let Some(c) = u32::try_from(value).ok().and_then(char::from_u32).filter(|c| go_format_is_print(*c)) {
                    text.push_str(&format!(" '{}'", c));
                }
            }
            return Some(go_format_pad(spec, text, 0, false));
        }
        _ => return None,
    };
    let sign = go_format_sign(spec, negative);
    // A precision and zero padding both ask for a least number of digits.
    // The sign is counted in the width of zero padding; 0x, 0b and 0o are
    // not, they come on top.
    let least_digits = match spec.precision {
        // Zero with a precision of zero prints nothing but the padding.
        Some(0) if magnitude == 0 => return Some(go_format_pad(&GoFormatSpec { zero: false, ..*spec }, String::new(), 0, false)),
        Some(precision) => precision,
        None if spec.zero && !spec.minus => spec.width.unwrap_or(0).saturating_sub(sign.len()),
        None => 0,
    };
    if digits.len() < least_digits {
        digits = "0".repeat(least_digits - digits.len()) + &digits;
    }
    // The 0 that # puts before an octal number is only added to one that
    // does not start with a 0 already. %#O has it as well, after its 0o.
    let octal_zero = if spec.sharp && matches!(verb, 'o' | 'O') && !digits.starts_with('0') { "0" } else { "" };
    let prefix = match verb {
        'b' if spec.sharp => "0b",
        'O' => "0o",
        'x' if spec.sharp => "0x",
        'X' if spec.sharp => "0X",
        _ => "",
    };
    Some(go_format_pad(spec, format!("{}{}{}{}", sign, prefix, octal_zero, digits), 0, false))
}

// The digits of a float and the power of ten of the first one, from Rust's
// {:e} form, which rounds as strconv does.
fn go_format_float_digits(exponent_form: &str) -> (String, i32) {
    let (mantissa, exponent) = exponent_form.split_once('e').unwrap();
    let digits: String = mantissa.chars().filter(|c| c.is_ascii_digit()).collect();
    (digits, exponent.parse().unwrap())
}

fn go_format_float_exponent(digits: &str, exponent: i32, upper: bool) -> String {
    let mut out = String::new();
    out.push_str(&digits[..1]);
    if digits.len() > 1 {
        out.push('.');
        out.push_str(&digits[1..]);
    }
    out.push(if upper { 'E' } else { 'e' });
    out.push(if exponent < 0 { '-' } else { '+' });
    out.push_str(&format!("{:02}", exponent.abs()));
    out
}

// Formats the magnitude of a float as strconv.FormatFloat does for 'e', 'f'
// and 'g'. shortest is the float printed with as few digits as identify it.
fn go_format_float_magnitude(verb: char, precision: Option<usize>, shortest: &str, with_precision: impl Fn(usize) -> String, fixed: impl Fn(usize) -> String) -> String {
    let upper = verb.is_ascii_uppercase();
    match verb.to_ascii_lowercase() {
        'e' => {
            let (digits, exponent) = go_format_float_digits(&with_precision(precision.unwrap_or(6)));
            go_format_float_exponent(&digits, exponent, upper)
        }
        'f' => fixed(precision.unwrap_or(6)),
        _ => {
            let (digits, exponent, exponent_limit) = match precision {
                None => {
                    let (digits, exponent) = go_format_float_digits(shortest);
                    (digits, exponent, 6)
                }
                Some(precision) => {
                    let precision = precision.max(1);
                    let (digits, exponent) = go_format_float_digits(&with_precision(precision - 1));
                    let digits = digits.trim_end_matches('0');
                    let digits = if digits.is_empty() { "0".to_string() } else { digits.to_string() };
                    let mut limit = precision as i32;
                    if limit > digits.len() as i32 && digits.len() as i32 >= exponent + 1 {
                        limit = digits.len() as i32;
                    }
                    (digits, exponent, limit)
                }
            };
            if exponent < -4 || exponent >= exponent_limit {
                return go_format_float_exponent(&digits, exponent, upper);
            }
            if exponent < 0 {
                return format!("0.{}{}", "0".repeat((-exponent - 1) as usize), digits);
            }
            let whole = (exponent + 1) as usize;
            if digits.len() <= whole {
                return format!("{}{}", digits, "0".repeat(whole - digits.len()));
            }
            format!("{}.{}", &digits[..whole], &digits[whole..])
        }
    }
}

fn go_format_float(spec: &GoFormatSpec, verb: char, value: f64, single: bool) -> Option<String> {
    // fmt also writes a float in binary (%b) and in hex (%x). Neither is
    // done here.
    if matches!(verb, 'b' | 'x' | 'X') {
        panic!("go2rust: %{} of a float is not supported in a format that is only known at run time", verb);
    }
    if !matches!(verb, 'v' | 'e' | 'E' | 'f' | 'F' | 'g' | 'G') {
        return None;
    }
    if spec.sharp {
        panic!("go2rust: the # flag on %{} of a float is not supported in a format that is only known at run time", verb);
    }
    if value.is_nan() {
        let sign = if spec.plus { "+" } else if spec.space { " " } else { "" };
        return Some(go_format_pad(spec, format!("{}NaN", sign), 0, false));
    }
    let negative = value.is_sign_negative();
    if value.is_infinite() {
        // +Inf keeps its sign, unless the space flag asks for a space there.
        let sign = if negative { "-" } else if spec.space && !spec.plus { " " } else { "+" };
        return Some(go_format_pad(spec, format!("{}Inf", sign), 0, false));
    }
    let magnitude = value.abs();
    // A float32 keeps the digits of the float32, not of the float64 it was
    // widened to.
    let narrow = magnitude as f32;
    let shortest = if single { format!("{:e}", narrow) } else { format!("{:e}", magnitude) };
    let with_precision = |precision: usize| if single { format!("{:.*e}", precision, narrow) } else { format!("{:.*e}", precision, magnitude) };
    let fixed = |precision: usize| if single { format!("{:.*}", precision, narrow) } else { format!("{:.*}", precision, magnitude) };
    let verb = if verb == 'v' { 'g' } else { verb };
    let digits = go_format_float_magnitude(verb, spec.precision, &shortest, with_precision, fixed);
    let sign = go_format_sign(spec, negative);
    Some(go_format_pad(spec, format!("{}{}", sign, digits), sign.len(), true))
}

// %x and %X of a string or a []byte: two digits for each byte. A precision
// counts bytes here, not characters.
fn go_format_hex(spec: &GoFormatSpec, verb: char, bytes: &[u8]) -> String {
    let bytes = match spec.precision {
        Some(precision) if precision < bytes.len() => &bytes[..precision],
        _ => bytes,
    };
    let prefix = if !spec.sharp { "" } else if verb == 'x' { "0x" } else { "0X" };
    let digits: Vec<String> = bytes.iter().map(|b| if verb == 'x' { format!("{:02x}", b) } else { format!("{:02X}", b) }).collect();
    let text = if spec.space {
        digits.iter().map(|d| format!("{}{}", prefix, d)).collect::<Vec<_>>().join(" ")
    } else if digits.is_empty() {
        String::new()
    } else {
        format!("{}{}", prefix, digits.concat())
    };
    go_format_pad(spec, text, 0, true)
}

fn go_format_string(spec: &GoFormatSpec, verb: char, text: &str) -> Option<String> {
    if matches!(verb, 'x' | 'X') {
        return Some(go_format_hex(spec, verb, text.as_bytes()));
    }
    let truncated: String = match spec.precision {
        Some(precision) => text.chars().take(precision).collect(),
        None => text.to_string(),
    };
    let formatted = match verb {
        's' | 'v' => truncated,
        'q' => {
            // strconv.CanBackquote: no backquote, no ASCII control character
            // but tab, and no byte order mark.
            let raw = truncated.chars().all(|c| if c.is_ascii() { (c >= ' ' || c == '\t') && c != '\u{60}' && c != '\u{7f}' } else { c != '\u{feff}' });
            if spec.sharp && raw {
                format!("\u{60}{}\u{60}", truncated)
            } else {
                go_format_quote(&truncated, '"', spec.plus)
            }
        }
        _ => return None,
    };
    Some(go_format_pad(spec, formatted, 0, true))
}

// A []byte prints as text for the verbs that print a string.
fn go_format_bytes(spec: &GoFormatSpec, verb: char, bytes: &[u8]) -> Option<String> {
    if matches!(verb, 'x' | 'X') {
        return Some(go_format_hex(spec, verb, bytes));
    }
    if !matches!(verb, 's' | 'q') {
        return None;
    }
    // A precision counts characters, and a byte that is not UTF-8 counts as
    // one. It is applied first: what is cut off has no say in how the rest
    // is written.
    let mut kept = bytes.len();
    if let Some(precision) = spec.precision {
        let (mut count, mut end) = (0, 0);
        'chunks: for chunk in bytes.utf8_chunks() {
            for c in chunk.valid().chars() {
                if count == precision {
                    break 'chunks;
                }
                count += 1;
                end += c.len_utf8();
            }
            for _ in chunk.invalid() {
                if count == precision {
                    break 'chunks;
                }
                count += 1;
                end += 1;
            }
        }
        kept = end;
    }
    let bytes = &bytes[..kept];
    let spec = &GoFormatSpec { precision: None, ..*spec };
    match std::str::from_utf8(bytes) {
        Ok(text) => go_format_string(spec, verb, text),
        // %q writes a byte that is not UTF-8 as an escape.
        Err(_) if verb == 'q' => {
            let mut quoted = String::from("\"");
            for chunk in bytes.utf8_chunks() {
                go_format_escape(&mut quoted, chunk.valid(), '"', spec.plus);
                for byte in chunk.invalid() {
                    quoted.push_str(&format!("\\x{:02x}", byte));
                }
            }
            quoted.push('"');
            Some(go_format_pad(spec, quoted, 0, true))
        }
        // %s would have to write the byte as it is, and the result is a Rust
        // String, which cannot hold it.
        Err(_) => panic!("go2rust: %s of a []byte that is not UTF-8 is not supported in a format that is only known at run time"),
    }
}

// Formats one argument for one verb. None means the verb does not apply to
// the argument's type, which fmt reports in the output.
fn go_format_argument(spec: &GoFormatSpec, verb: char, value: &(dyn Any + Send + Sync)) -> Option<String> {
    if verb == 'T' {
        return Some(go_format_pad(spec, __go_type_name(value).to_string(), 0, true));
    }
    if verb == 'p' {
        panic!("go2rust: %p is not supported in a format that is only known at run time");
    }
    // %+v and %#v ask for field names and Go syntax. A number or a bool has
    // neither, so it prints as %v prints it, with two exceptions for %#v: an
    // unsigned integer is written in hex and a string in quotes.
    let (plus_v, sharp_v) = (verb == 'v' && spec.plus, verb == 'v' && spec.sharp);
    let basic = GoFormatSpec { plus: spec.plus && !plus_v, sharp: spec.sharp && !sharp_v, ..*spec };
    macro_rules! signed {
        ($($t:ty),*) => {
            $(if let Some(v) = value.downcast_ref::<$t>() {
                return go_format_integer(&basic, verb, *v as i128);
            })*
        };
    }
    signed!(i8, i16, i32, i64, isize);
    // A rune taken from a range over a string can be a Rust char; it is a
    // Go rune, an int32.
    if let Some(v) = value.downcast_ref::<char>() {
        return go_format_integer(&basic, verb, *v as i128);
    }
    macro_rules! unsigned {
        ($($t:ty),*) => {
            $(if let Some(v) = value.downcast_ref::<$t>() {
                if sharp_v {
                    return go_format_integer(&GoFormatSpec { sharp: true, ..basic }, 'x', *v as i128);
                }
                return go_format_integer(&basic, verb, *v as i128);
            })*
        };
    }
    unsigned!(u8, u16, u32, u64, usize);
    if let Some(v) = value.downcast_ref::<f64>() {
        return go_format_float(&basic, verb, *v, false);
    }
    if let Some(v) = value.downcast_ref::<f32>() {
        return go_format_float(&basic, verb, *v as f64, true);
    }
    if let Some(v) = value.downcast_ref::<bool>() {
        return if matches!(verb, 't' | 'v') { Some(go_format_pad(&basic, v.to_string(), 0, true)) } else { None };
    }
    let string_verb = if sharp_v { 'q' } else { verb };
    if let Some(v) = value.downcast_ref::<String>() {
        return go_format_string(&basic, string_verb, v);
    }
    if let Some(v) = value.downcast_ref::<&str>() {
        return go_format_string(&basic, string_verb, v);
    }
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Vec<u8>>>>>() {
        let guard = v.lock().unwrap();
        let bytes: &[u8] = guard.as_deref().unwrap_or(&[]);
        if let Some(text) = go_format_bytes(spec, verb, bytes) {
            return Some(text);
        }
        // Any other verb is applied to each byte, between brackets. A verb
        // that does not fit a number is reported for each byte. %#v writes
        // the slice as Go source, which is not done here.
        if !sharp_v {
            let items: Vec<String> = bytes.iter().map(|byte| go_format_integer(&basic, verb, *byte as i128).unwrap_or_else(|| format!("%!{}(uint8={})", verb, go_format_integer(&basic, 'd', *byte as i128).unwrap_or_default()))).collect();
            return Some(format!("[{}]", items.join(" ")));
        }
    }
    // A slice of strings, numbers or bools: the verb, with its width and
    // flags, is applied to each element, between brackets, as fmt does
    // (%q of []string is ["a" "b"]). An element the verb does not fit is
    // reported in its place. A plain %v is left to Println's form below.
    let plain_v = verb == 'v' && spec.width.is_none() && spec.precision.is_none() && !spec.plus && !spec.sharp && !spec.space;
    macro_rules! basic_slice {
        ($($t:ty),*) => {
            $(if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Vec<$t>>>>>() {
                if !plain_v && !sharp_v {
                    let guard = v.lock().unwrap();
                    let items: Vec<String> = guard.as_ref().map(|values| values.iter().map(|item| {
                        let item: &(dyn Any + Send + Sync) = item;
                        go_format_argument(spec, verb, item).unwrap_or_else(|| format!("%!{}({})", verb, go_format_typed_value(spec, item)))
                    }).collect()).unwrap_or_default();
                    return Some(format!("[{}]", items.join(" ")));
                }
            })*
        };
    }
    basic_slice!(String, i64, i32, i16, i8, u64, u32, u16, f64, f32, bool);
    // An array of them the same way. An array of bytes is the text or the
    // hex of those bytes for %s, %q, %x and %X, as a []byte is.
    if !plain_v && !sharp_v {
        if let Some(items) = go_format_elements(value) {
            let bytes: Option<Vec<u8>> = items.iter().map(|item| item.downcast_ref::<u8>().copied()).collect();
            if let Some(bytes) = bytes.filter(|bytes| !bytes.is_empty()) {
                if let Some(text) = go_format_bytes(spec, verb, &bytes) {
                    return Some(text);
                }
            }
            let items: Vec<String> = items.iter().map(|item| {
                let item: &(dyn Any + Send + Sync) = item.as_ref();
                go_format_argument(spec, verb, item).unwrap_or_else(|| format!("%!{}({})", verb, go_format_typed_value(spec, item)))
            }).collect();
            return Some(format!("[{}]", items.join(" ")));
        }
    }
    // A value with a String or an Error method is the text that method
    // returns, for the verbs that print a string.
    if go_format_is_text(value) && !sharp_v && matches!(verb, 'v' | 's' | 'q' | 'x' | 'X') {
        return go_format_string(&basic, verb, &format_any(value));
    }
    if let Some(inner) = go_format_named_basic_value(value) {
        return go_format_argument(spec, verb, inner.as_ref());
    }
    // Any other value prints as Println prints it, for a plain %v. fmt
    // applies a width, a precision, a flag or another verb to each field or
    // element, which is not done here. nil is one text.
    if plus_v && spec.width.is_none() && spec.precision.is_none() && !spec.space {
        if let Some(text) = go_format_fields_value(value) {
            return Some(text);
        }
    }
    if verb == 'v' && !plus_v && !sharp_v {
        let text = format_any(value);
        if text == "<nil>" {
            return Some(go_format_pad(spec, text, 0, true));
        }
        if spec.width.is_none() && spec.precision.is_none() && !spec.space {
            return Some(text);
        }
    }
    panic!("go2rust: %{}{}{} of a {} is not supported in a format that is only known at run time", if spec.plus { "+" } else { "" }, if spec.sharp { "#" } else { "" }, verb, __go_type_name(value));
}

// The number after % or after the dot: digits, or * for the next argument.
// Ok(None) is no number. Err is a * whose argument is not a usable integer.
fn go_format_number(chars: &mut std::iter::Peekable<std::str::Chars>, args: &[Box<dyn Any + Send + Sync>], next: &mut usize) -> Result<Option<i64>, ()> {
    if chars.peek() == Some(&'*') {
        chars.next();
        let arg: &(dyn Any + Send + Sync) = match args.get(*next) {
            Some(arg) => arg.as_ref(),
            None => return Err(()),
        };
        *next += 1;
        // fmt takes any integer type here.
        let mut value: Option<i128> = None;
        macro_rules! integer {
            ($($t:ty),*) => {
                $(if let Some(v) = arg.downcast_ref::<$t>() {
                    value = Some(*v as i128);
                })*
            };
        }
        integer!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
        return match value {
            Some(value) if value.abs() <= 1_000_000 => Ok(Some(value as i64)),
            _ => Err(()),
        };
    }
    let mut value: Option<i64> = None;
    while let Some(digit) = chars.peek().and_then(|c| c.to_digit(10)) {
        value = Some(value.unwrap_or(0).saturating_mul(10).saturating_add(digit as i64));
        chars.next();
    }
    Ok(value.filter(|value| *value <= 1_000_000))
}

// Whether value is of a type that go_format_argument formats itself.
fn go_format_is_basic(value: &(dyn Any + Send + Sync)) -> bool {
    macro_rules! is_one_of {
        ($($t:ty),*) => { false $(|| value.is::<$t>())* };
    }
    is_one_of!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, bool, String, &str, Arc<Mutex<Option<Vec<u8>>>>)
}

// A nil interface value: the nil literal, or a nil interface value boxed as
// the program boxes one.
fn go_format_is_nil(value: &(dyn Any + Send + Sync)) -> bool {
    value.is::<GoFormatNil>() || value.is::<GoNilAny>()
}

// An argument as fmt names it when it does not fit the format: type=value.
// The value is written as %v writes it, with the width, the precision and
// the flags of the verb that did not fit.
fn go_format_typed_value(spec: &GoFormatSpec, value: &(dyn Any + Send + Sync)) -> String {
    if go_format_is_nil(value) {
        return "<nil>".to_string();
    }
    let text = go_format_argument(spec, go_format_plain_verb(value), value).unwrap_or_default();
    format!("{}={}", __go_type_name(value), text)
}

// The verb that writes value as %v does and leaves the flags as they are.
// %v itself would read + and # as %+v and %#v.
fn go_format_plain_verb(value: &(dyn Any + Send + Sync)) -> char {
    macro_rules! is_one_of {
        ($($t:ty),*) => { false $(|| value.is::<$t>())* };
    }
    if is_one_of!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize) {
        'd'
    } else if is_one_of!(f32, f64) {
        'g'
    } else if value.is::<bool>() {
        't'
    } else if is_one_of!(String, &str) {
        's'
    } else {
        'v'
    }
}

// fmt.Sprintf for a format that is only known when the program runs. A
// format written as a literal is expanded when the program is translated and
// does not come here. wraps is true for fmt.Errorf, where %w prints as %v.
pub(super) fn go_sprintf(format: &str, args: &[Box<dyn Any + Send + Sync>], wraps: bool) -> String {
    let mut out = String::new();
    let mut next = 0;
    let mut chars = format.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '%' {
            out.push(c);
            continue;
        }
        let mut spec = GoFormatSpec::default();
        loop {
            match chars.peek() {
                Some('+') => spec.plus = true,
                Some('-') => spec.minus = true,
                Some('#') => spec.sharp = true,
                Some(' ') => spec.space = true,
                Some('0') => spec.zero = true,
                _ => break,
            }
            chars.next();
        }
        if chars.peek() == Some(&'[') {
            panic!("go2rust: an argument index such as %[1]d is not supported in a format that is only known at run time");
        }
        match go_format_number(&mut chars, args, &mut next) {
            // A negative width from an argument means left alignment.
            Ok(Some(width)) if width < 0 => {
                spec.minus = true;
                spec.width = Some((-width) as usize);
            }
            Ok(width) => spec.width = width.map(|width| width as usize),
            Err(()) => out.push_str("%!(BADWIDTH)"),
        }
        if chars.peek() == Some(&'.') {
            chars.next();
            match go_format_number(&mut chars, args, &mut next) {
                Ok(Some(precision)) if precision < 0 => out.push_str("%!(BADPREC)"),
                Ok(precision) => spec.precision = Some(precision.unwrap_or(0) as usize),
                Err(()) => out.push_str("%!(BADPREC)"),
            }
        }
        if chars.peek() == Some(&'[') {
            panic!("go2rust: an argument index such as %[1]d is not supported in a format that is only known at run time");
        }
        let verb = match chars.next() {
            Some(verb) => verb,
            None => {
                out.push_str("%!(NOVERB)");
                break;
            }
        };
        if verb == '%' {
            out.push('%');
            continue;
        }
        let arg = match args.get(next) {
            Some(arg) => arg,
            None => {
                out.push_str(&format!("%!{}(MISSING)", verb));
                continue;
            }
        };
        next += 1;
        // In fmt.Errorf %w prints an error as %v does. A number, a string, a
        // bool or a []byte is not an error, and fmt reports the verb: once
        // for a scalar, and for each byte of a []byte.
        // A nil interface value prints <nil> for %v and %T, and fmt reports
        // any other verb.
        if go_format_is_nil(arg.as_ref()) {
            if matches!(verb, 'v' | 'T') {
                out.push_str(&go_format_pad(&spec, "<nil>".to_string(), 0, true));
            } else {
                out.push_str(&format!("%!{}(<nil>)", verb));
            }
            continue;
        }
        let verb = if verb == 'w' && wraps && !go_format_is_basic(arg.as_ref()) { 'v' } else { verb };
        match go_format_argument(&spec, verb, arg.as_ref()) {
            Some(text) => out.push_str(&text),
            None => out.push_str(&format!("%!{}({})", verb, go_format_typed_value(&spec, arg.as_ref()))),
        }
    }
    if next < args.len() {
        let extra: Vec<String> = args[next..].iter().map(|arg| go_format_typed_value(&GoFormatSpec::default(), arg.as_ref())).collect();
        out.push_str(&format!("%!(EXTRA {})", extra.join(", ")));
    }
    out
}
}
use __go_runtime_format::go_sprintf;

#[derive(Debug)]
pub struct Level(pub Arc<Mutex<Option<i64>>>);

impl Clone for Level {
    fn clone(&self) -> Self {
        let __value = (*self.0.lock().unwrap()).clone();
        Level(Arc::new(Mutex::new(__value)))
    }
}

impl Default for Level {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(Some(Default::default()))))
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for Level {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i64> for Level {
    fn eq(&self, other: &i64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Level {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i64> for Level {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Level> for i64 {
    fn eq(&self, other: &Level) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<Level> for i64 {
    fn partial_cmp(&self, other: &Level) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for Level {
    type Output = Level;
    fn add(self, other: Self) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i64> for Level {
    type Output = Level;
    fn add(self, other: i64) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Level> for i64 {
    type Output = Level;
    fn add(self, other: Level) -> Level {
        Level(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Level {
    type Output = Level;
    fn sub(self, other: Self) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i64> for Level {
    type Output = Level;
    fn sub(self, other: i64) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Level> for i64 {
    type Output = Level;
    fn sub(self, other: Level) -> Level {
        Level(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Level {
    type Output = Level;
    fn mul(self, other: Self) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i64> for Level {
    type Output = Level;
    fn mul(self, other: i64) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Level> for i64 {
    type Output = Level;
    fn mul(self, other: Level) -> Level {
        Level(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Level {
    type Output = Level;
    fn div(self, other: Self) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i64> for Level {
    type Output = Level;
    fn div(self, other: i64) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Level> for i64 {
    type Output = Level;
    fn div(self, other: Level) -> Level {
        Level(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for Level {
    type Output = Level;
    fn neg(self) -> Level {
        Level(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for Level {
    type Output = Level;
    fn rem(self, other: Self) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i64> for Level {
    type Output = Level;
    fn rem(self, other: i64) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<Level> for i64 {
    type Output = Level;
    fn rem(self, other: Level) -> Level {
        Level(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Level {
    type Output = Level;
    fn bitand(self, other: Self) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i64> for Level {
    type Output = Level;
    fn bitand(self, other: i64) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Level> for i64 {
    type Output = Level;
    fn bitand(self, other: Level) -> Level {
        Level(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Level {
    type Output = Level;
    fn bitor(self, other: Self) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i64> for Level {
    type Output = Level;
    fn bitor(self, other: i64) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Level> for i64 {
    type Output = Level;
    fn bitor(self, other: Level) -> Level {
        Level(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Level {
    type Output = Level;
    fn bitxor(self, other: Self) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i64> for Level {
    type Output = Level;
    fn bitxor(self, other: i64) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Level> for i64 {
    type Output = Level;
    fn bitxor(self, other: Level) -> Level {
        Level(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Level {
    type Output = Level;
    fn not(self) -> Level {
        Level(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Level {
    type Output = Level;
    fn shl(self, other: Level) -> Level {
        Level(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (*other.0.lock().unwrap().as_ref().unwrap()) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i32> for Level {
    type Output = Level;
    fn shl(self, other: i32) -> Level {
        Level(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i8> for Level {
    type Output = Level;
    fn shl(self, other: i8) -> Level {
        Level(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i16> for Level {
    type Output = Level;
    fn shl(self, other: i16) -> Level {
        Level(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i64> for Level {
    type Output = Level;
    fn shl(self, other: i64) -> Level {
        Level(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u32> for Level {
    type Output = Level;
    fn shl(self, other: u32) -> Level {
        Level(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u8> for Level {
    type Output = Level;
    fn shl(self, other: u8) -> Level {
        Level(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u16> for Level {
    type Output = Level;
    fn shl(self, other: u16) -> Level {
        Level(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u64> for Level {
    type Output = Level;
    fn shl(self, other: u64) -> Level {
        Level(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<usize> for Level {
    type Output = Level;
    fn shl(self, other: usize) -> Level {
        Level(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr for Level {
    type Output = Level;
    fn shr(self, other: Level) -> Level {
        Level(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (*other.0.lock().unwrap().as_ref().unwrap()) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i32> for Level {
    type Output = Level;
    fn shr(self, other: i32) -> Level {
        Level(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i8> for Level {
    type Output = Level;
    fn shr(self, other: i8) -> Level {
        Level(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i16> for Level {
    type Output = Level;
    fn shr(self, other: i16) -> Level {
        Level(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i64> for Level {
    type Output = Level;
    fn shr(self, other: i64) -> Level {
        Level(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u32> for Level {
    type Output = Level;
    fn shr(self, other: u32) -> Level {
        Level(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u8> for Level {
    type Output = Level;
    fn shr(self, other: u8) -> Level {
        Level(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u16> for Level {
    type Output = Level;
    fn shr(self, other: u16) -> Level {
        Level(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u64> for Level {
    type Output = Level;
    fn shr(self, other: u64) -> Level {
        Level(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<usize> for Level {
    type Output = Level;
    fn shr(self, other: usize) -> Level {
        Level(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl Eq for Level {}

impl Ord for Level {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug)]
pub struct Name(pub Arc<Mutex<Option<String>>>);

impl Clone for Name {
    fn clone(&self) -> Self {
        let __value = (*self.0.lock().unwrap()).clone();
        Name(Arc::new(Mutex::new(__value)))
    }
}

impl Default for Name {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(Some(Default::default()))))
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialOrd for Name {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ord::cmp(self, other))
    }
}

impl Eq for Name {}

impl Ord for Name {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}

impl PartialEq<String> for Name {
    fn eq(&self, other: &String) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialEq<&str> for Name {
    fn eq(&self, other: &&str) -> bool {
        self.0.lock().unwrap().as_ref().unwrap().as_str() == *other
    }
}

impl std::ops::Add for Name {
    type Output = Name;
    fn add(self, other: Self) -> Name {
        let __joined = format!("{}{}", self.0.lock().unwrap().as_ref().unwrap(), other.0.lock().unwrap().as_ref().unwrap());
        Name(Arc::new(Mutex::new(Some(__joined))))
    }
}

impl std::ops::Add<String> for Name {
    type Output = Name;
    fn add(self, other: String) -> Name {
        let __joined = format!("{}{}", self.0.lock().unwrap().as_ref().unwrap(), other);
        Name(Arc::new(Mutex::new(Some(__joined))))
    }
}

impl std::ops::Add<&str> for Name {
    type Output = Name;
    fn add(self, other: &str) -> Name {
        let __joined = format!("{}{}", self.0.lock().unwrap().as_ref().unwrap(), other);
        Name(Arc::new(Mutex::new(Some(__joined))))
    }
}


#[derive(Debug)]
pub struct Celsius(pub Arc<Mutex<Option<f64>>>);

impl Clone for Celsius {
    fn clone(&self) -> Self {
        let __value = (*self.0.lock().unwrap()).clone();
        Celsius(Arc::new(Mutex::new(__value)))
    }
}

impl Default for Celsius {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(Some(Default::default()))))
    }
}

impl Display for Celsius {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", go_float_string(*self.0.lock().unwrap().as_ref().unwrap() as f64, false))
    }
}

impl PartialEq for Celsius {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<f64> for Celsius {
    fn eq(&self, other: &f64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Celsius {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<f64> for Celsius {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Celsius> for f64 {
    fn eq(&self, other: &Celsius) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<Celsius> for f64 {
    fn partial_cmp(&self, other: &Celsius) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for Celsius {
    type Output = Celsius;
    fn add(self, other: Self) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<f64> for Celsius {
    type Output = Celsius;
    fn add(self, other: f64) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Celsius> for f64 {
    type Output = Celsius;
    fn add(self, other: Celsius) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Celsius {
    type Output = Celsius;
    fn sub(self, other: Self) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<f64> for Celsius {
    type Output = Celsius;
    fn sub(self, other: f64) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Celsius> for f64 {
    type Output = Celsius;
    fn sub(self, other: Celsius) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Celsius {
    type Output = Celsius;
    fn mul(self, other: Self) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<f64> for Celsius {
    type Output = Celsius;
    fn mul(self, other: f64) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Celsius> for f64 {
    type Output = Celsius;
    fn mul(self, other: Celsius) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Celsius {
    type Output = Celsius;
    fn div(self, other: Self) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<f64> for Celsius {
    type Output = Celsius;
    fn div(self, other: f64) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Celsius> for f64 {
    type Output = Celsius;
    fn div(self, other: Celsius) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for Celsius {
    type Output = Celsius;
    fn neg(self) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}


#[derive(Debug, Clone, Default)]
pub struct IDs(pub Arc<Mutex<Option<Vec<i64>>>>);

impl Display for IDs {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice(&self.0))
    }
}


#[derive(Debug, Clone, Default)]
pub struct Tags(pub Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>);

impl Display for Tags {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_map(&self.0))
    }
}


#[derive(Debug, Clone)]
pub struct Point {
    pub x: Arc<Mutex<Option<i64>>>,
    pub y: Arc<Mutex<Option<i64>>>,
}

impl Point {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.y.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
            y: __go_clone_0_1,
        }
    }
}


impl Default for Point {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_0_1 = Arc::new(Mutex::new(Some(0)));
        Self {
            x: __go_default_0_0,
            y: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.y.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.x.lock().unwrap().as_ref().cloned() }; let __right = { other.x.lock().unwrap().as_ref().cloned() }; __left == __right }
                && { let __left = { self.y.lock().unwrap().as_ref().cloned() }; let __right = { other.y.lock().unwrap().as_ref().cloned() }; __left == __right }
        )
    }
}

impl Eq for Point {}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ord::cmp(self, other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            let __left = { self.x.lock().unwrap().as_ref().cloned() };
            let __right = { other.x.lock().unwrap().as_ref().cloned() };
            match __left.cmp(&__right) {
                std::cmp::Ordering::Equal => {}
                __ord => return __ord,
            }
        }
        {
            let __left = { self.y.lock().unwrap().as_ref().cloned() };
            let __right = { other.y.lock().unwrap().as_ref().cloned() };
            match __left.cmp(&__right) {
                std::cmp::Ordering::Equal => {}
                __ord => return __ord,
            }
        }
        std::cmp::Ordering::Equal
    }
}


pub trait Shape: std::fmt::Display + Any {
    fn __go_clone_box_shape(&self) -> Box<dyn Shape + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_print(&self, plus: bool) -> String { if plus { format!("{:#}", self) } else { format!("{}", self) } }
    fn __go_eq_shape(&self, other: &(dyn Shape + Send + Sync)) -> bool;
    fn area(&self) -> f64;
}

impl Clone for Box<dyn Shape + Send + Sync> {
    fn clone(&self) -> Self {
        Shape::__go_clone_box_shape(self.as_ref())
    }
}

#[derive(Debug, Clone)]
pub struct Square {
    pub side: Arc<Mutex<Option<f64>>>,
}

impl Square {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.side.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            side: __go_clone_0_0,
        }
    }
}


impl Default for Square {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0.0)));
        Self {
            side: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = go_float_string(((*self.side.lock().unwrap().as_ref().unwrap())) as f64, false);
        write!(f, "{{{}}}", __go_fmt_0)
    }
}
impl PartialEq for Square {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.side.lock().unwrap().as_ref().cloned() }; let __right = { other.side.lock().unwrap().as_ref().cloned() }; __left == __right }
        )
    }
}

impl Eq for Square {}

impl PartialOrd for Square {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ord::cmp(self, other))
    }
}

impl Ord for Square {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            let __left = { self.side.lock().unwrap().as_ref().cloned() };
            let __right = { other.side.lock().unwrap().as_ref().cloned() };
            match __left.partial_cmp(&__right) {
                Some(std::cmp::Ordering::Equal) | None => {}
                Some(__ord) => return __ord,
            }
        }
        std::cmp::Ordering::Equal
    }
}


#[derive(Debug, Clone)]
pub struct Circle {
    pub r: Arc<Mutex<Option<f64>>>,
}

impl Circle {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.r.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            r: __go_clone_0_0,
        }
    }
}


impl Default for Circle {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0.0)));
        Self {
            r: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = go_float_string(((*self.r.lock().unwrap().as_ref().unwrap())) as f64, false);
        write!(f, "{{{}}}", __go_fmt_0)
    }
}
impl PartialEq for Circle {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.r.lock().unwrap().as_ref().cloned() }; let __right = { other.r.lock().unwrap().as_ref().cloned() }; __left == __right }
        )
    }
}


/// Version writes itself with GoString, which %#v prefers.
#[derive(Debug, Clone)]
pub struct Version {
    pub major: Arc<Mutex<Option<i64>>>,
    pub minor: Arc<Mutex<Option<i64>>>,
}

impl Version {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.major.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.minor.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            major: __go_clone_0_0,
            minor: __go_clone_0_1,
        }
    }
}


impl Default for Version {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_0_1 = Arc::new(Mutex::new(Some(0)));
        Self {
            major: __go_default_0_0,
            minor: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.major.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.minor.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct Handle {
    pub id: Arc<Mutex<Option<i64>>>,
}

impl Handle {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.id.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            id: __go_clone_0_0,
        }
    }
}


impl Default for Handle {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            id: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Handle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.id.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone)]
pub struct Base {
    pub i_d: Arc<Mutex<Option<i64>>>,
}

impl Base {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.i_d.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            i_d: __go_clone_0_0,
        }
    }
}


impl Default for Base {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            i_d: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Base {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.i_d.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Clone)]
pub struct Record {
    pub base: Arc<Mutex<Option<Base>>>,
    pub point: Arc<Mutex<Option<Point>>>,
    pub name: Arc<Mutex<Option<String>>>,
    pub tags: Arc<Mutex<Option<Vec<String>>>>,
    pub scores: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>,
    pub flags: Arc<Mutex<Option<BTreeMap<bool, Arc<Mutex<Option<String>>>>>>>,
    pub grid: Arc<Mutex<Option<[[i64; 2]; 2]>>>,
    pub next: Arc<Mutex<Option<Record>>>,
    pub data: Arc<Mutex<Option<Vec<u8>>>>,
    pub count: Arc<Mutex<Option<u64>>>,
    pub ratio: Arc<Mutex<Option<f64>>>,
    pub any: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>,
    pub err: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>,
    pub shape: Arc<Mutex<Option<Box<dyn Shape + Send + Sync>>>>,
    pub on_done: Arc<Mutex<Option<Arc<dyn Fn() -> () + Send + Sync>>>>,
    pub events: GoChannel<i64>,
    pub hidden: Arc<Mutex<Option<i64>>>,
}

impl Record {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.base.lock().unwrap(); Arc::new(Mutex::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        let __go_clone_1_0 = self.point.clone();
        let __go_clone_2_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.tags.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = self.scores.clone();
        let __go_clone_5_0 = self.flags.clone();
        let __go_clone_6_0 = { let __guard = self.grid.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_7_0 = self.next.clone();
        let __go_clone_8_0 = { let __guard = self.data.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_9_0 = { let __guard = self.count.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_10_0 = { let __guard = self.ratio.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_11_0 = self.any.clone();
        let __go_clone_12_0 = self.err.clone();
        let __go_clone_13_0 = self.shape.clone();
        let __go_clone_14_0 = { let __guard = self.on_done.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_15_0 = self.events.clone();
        let __go_clone_16_0 = { let __guard = self.hidden.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            base: __go_clone_0_0,
            point: __go_clone_1_0,
            name: __go_clone_2_0,
            tags: __go_clone_3_0,
            scores: __go_clone_4_0,
            flags: __go_clone_5_0,
            grid: __go_clone_6_0,
            next: __go_clone_7_0,
            data: __go_clone_8_0,
            count: __go_clone_9_0,
            ratio: __go_clone_10_0,
            any: __go_clone_11_0,
            err: __go_clone_12_0,
            shape: __go_clone_13_0,
            on_done: __go_clone_14_0,
            events: __go_clone_15_0,
            hidden: __go_clone_16_0,
        }
    }
}


impl Default for Record {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Base::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_3_0 = Arc::new(Mutex::new(None));
        let __go_default_4_0 = Arc::new(Mutex::new(None));
        let __go_default_5_0 = Arc::new(Mutex::new(None));
        let __go_default_6_0 = Arc::new(Mutex::new(Some(std::array::from_fn::<_, 2, _>(|_| std::array::from_fn::<_, 2, _>(|_| 0)))));
        let __go_default_7_0 = Arc::new(Mutex::new(None));
        let __go_default_8_0 = Arc::new(Mutex::new(None));
        let __go_default_9_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_10_0 = Arc::new(Mutex::new(Some(0.0)));
        let __go_default_11_0 = Arc::new(Mutex::new(None));
        let __go_default_12_0 = Arc::new(Mutex::new(None));
        let __go_default_13_0 = Arc::new(Mutex::new(None));
        let __go_default_14_0 = Arc::new(Mutex::new(None));
        let __go_default_15_0 = Default::default();
        let __go_default_16_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            base: __go_default_0_0,
            point: __go_default_1_0,
            name: __go_default_2_0,
            tags: __go_default_3_0,
            scores: __go_default_4_0,
            flags: __go_default_5_0,
            grid: __go_default_6_0,
            next: __go_default_7_0,
            data: __go_default_8_0,
            count: __go_default_9_0,
            ratio: __go_default_10_0,
            any: __go_default_11_0,
            err: __go_default_12_0,
            shape: __go_default_13_0,
            on_done: __go_default_14_0,
            events: __go_default_15_0,
            hidden: __go_default_16_0,
        }
    }
}

impl std::fmt::Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.base.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { let __guard = self.point.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_2 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", format_slice(&self.tags));
        let __go_fmt_4 = format!("{}", format_map(&self.scores));
        let __go_fmt_5 = format!("{}", format_map(&self.flags));
        let __go_fmt_6 = format!("{}", format_nested_slice(&self.grid));
        let __go_fmt_7 = format!("{}", { let __guard = self.next.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_8 = format!("{}", format_slice(&self.data));
        let __go_fmt_9 = format!("{}", (*self.count.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_10 = go_float_string(((*self.ratio.lock().unwrap().as_ref().unwrap())) as f64, false);
        let __go_fmt_11 = format!("{}", match self.any.lock().unwrap().as_ref() { Some(__v) => format_any_nested(__v.as_ref()), None => "<nil>".to_string() });
        let __go_fmt_12 = format!("{}", match self.err.lock().unwrap().as_ref() { Some(__v) => format!("{}", __v), None => "<nil>".to_string() });
        let __go_fmt_13 = format!("{}", (*self.shape.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_14 = format!("{}", "<func>");
        let __go_fmt_15 = format!("{}", (*self.hidden.lock().unwrap().as_ref().unwrap()));
        write!(
            f,
            "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}}}",
            __go_fmt_0,
            __go_fmt_1,
            __go_fmt_2,
            __go_fmt_3,
            __go_fmt_4,
            __go_fmt_5,
            __go_fmt_6,
            __go_fmt_7,
            __go_fmt_8,
            __go_fmt_9,
            __go_fmt_10,
            __go_fmt_11,
            __go_fmt_12,
            __go_fmt_13,
            __go_fmt_14,
            __go_fmt_15
        )
    }
}


#[derive(Debug, Clone)]
pub struct Pair__string__int {
    pub key: Arc<Mutex<Option<String>>>,
    pub val: Arc<Mutex<Option<i64>>>,
}

impl Pair__string__int {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.key.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.val.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            key: __go_clone_0_0,
            val: __go_clone_1_0,
        }
    }
}


impl Default for Pair__string__int {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            key: __go_default_0_0,
            val: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Pair__string__int {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.key.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.val.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl Square {
    pub fn area(&self) -> f64 {
        return { let __tmp_x = (*self.side.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.side.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y };
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        Square::area(self)
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Shape + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_shape(&self, other: &(dyn Shape + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Square>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct SquarePtr(pub Arc<Mutex<Option<Square>>>);

impl std::fmt::Display for SquarePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Shape for SquarePtr {
    fn area(&self) -> f64 {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Square::area(__recv)
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Shape + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => if plus { format!("&{:#}", __v) } else { format!("&{}", __v) }, None => "<nil>".to_string() }
    }
    fn __go_eq_shape(&self, other: &(dyn Shape + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SquarePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Circle {
    pub fn area(&self) -> f64 {
        return {
            let __tmp_x = { let __tmp_x = 3.0; let __tmp_y = (*self.r.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y };
            let __tmp_y = (*self.r.lock().unwrap().as_ref().unwrap());
            __tmp_x * __tmp_y
        };
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        Circle::area(self)
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Shape + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_shape(&self, other: &(dyn Shape + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Circle>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct CirclePtr(pub Arc<Mutex<Option<Circle>>>);

impl std::fmt::Display for CirclePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Shape for CirclePtr {
    fn area(&self) -> f64 {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Circle::area(__recv)
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Shape + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => if plus { format!("&{:#}", __v) } else { format!("&{}", __v) }, None => "<nil>".to_string() }
    }
    fn __go_eq_shape(&self, other: &(dyn Shape + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<CirclePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Version {
    pub fn go_string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(format!("v{}.{}", format!("{}", (*self.major.lock().unwrap().as_ref().unwrap())), format!("{}", (*self.minor.lock().unwrap().as_ref().unwrap()))))))
    }
}

impl Handle {
    pub fn go_string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(format!("Handle#{}", (*self.id.lock().unwrap().as_ref().unwrap())))))
    }
}

impl Record {
}

pub fn size(t: Arc<Mutex<Option<Tags>>>) -> i64 {
    return { let __named_map_holder = t.clone(); let __named_map_guard = __named_map_holder.lock().unwrap(); let __map_holder = __named_map_guard.as_ref().map(|__v| __v.0.clone()); std::mem::drop(__named_map_guard); __map_holder.as_ref().map(|__map_holder| { let __map_guard = __map_holder.lock().unwrap(); __map_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }).unwrap_or(0) } as i64;
}

pub fn is_nil(err: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) -> bool {
    return { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result };
}

pub fn show(label: Arc<Mutex<Option<String>>>, v: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) {
    print!("{}: {}\n", format!("{}", { let __v = (*label.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", (*__go2rust_gosyntax0_1(go_any_handle_copy(&v)).lock().unwrap().as_ref().unwrap())));
}

fn main() {
    go_install_panic_exit();
    print!("{}|{}|{}|{}\n", format!("{}", (*__go2rust_gosyntax0_13(Arc::new(Mutex::new(Some(Level(Arc::new(Mutex::new(Some(3 as i64)))))))).lock().unwrap().as_ref().unwrap())), format!("{}", (*__go2rust_gosyntax0_14(Arc::new(Mutex::new(Some(Name(Arc::new(Mutex::new(Some("bo".to_string())))))))).lock().unwrap().as_ref().unwrap())), format!("{}", (*__go2rust_gosyntax0_15(Arc::new(Mutex::new(Some(Celsius(Arc::new(Mutex::new(Some(21.5 as f64)))))))).lock().unwrap().as_ref().unwrap())), format!("{}", (*__go2rust_gosyntax0_16(Arc::new(Mutex::new(Some(IDs(Arc::new(Mutex::new(Some(vec![1i64, 2i64])))))))).lock().unwrap().as_ref().unwrap())));
    print!("{}|{}|{}\n", format!("{}", (*__go2rust_gosyntax0_17(Arc::new(Mutex::new(Some(Tags(Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("b".to_string(), Arc::new(Mutex::new(Some(2i64)))), ("a".to_string(), Arc::new(Mutex::new(Some(1i64))))]))))))))).lock().unwrap().as_ref().unwrap())), format!("{}", (*__go2rust_gosyntax0_16(Arc::new(Mutex::new(Some(IDs(Arc::new(Mutex::new(None::<Vec<i64>>))))))).lock().unwrap().as_ref().unwrap())), format!("{}", (*__go2rust_gosyntax0_17(Arc::new(Mutex::new(Some(Tags(Arc::new(Mutex::new(None::<BTreeMap<String, Arc<Mutex<Option<i64>>>>>))))))).lock().unwrap().as_ref().unwrap())));
    print!("{}|{}|{}|{}\n", format!("{}", (*__go2rust_gosyntax0_3(Arc::new(Mutex::new(Some(42)))).lock().unwrap().as_ref().unwrap())), format!("{}", (*__go2rust_gosyntax0_3(Arc::new(Mutex::new(Some(-7)))).lock().unwrap().as_ref().unwrap())), format!("{}", (*__go2rust_gosyntax0_18(Arc::new(Mutex::new(Some(255 as u8)))).lock().unwrap().as_ref().unwrap())), format!("{}", (*__go2rust_gosyntax0_19(Arc::new(Mutex::new(Some(16 as usize)))).lock().unwrap().as_ref().unwrap())));
    print!("{}|{}|{}|{}|{}\n", format!("{}", (*__go2rust_gosyntax0_5(Arc::new(Mutex::new(Some(1.0)))).lock().unwrap().as_ref().unwrap())), format!("{}", (*__go2rust_gosyntax0_5(Arc::new(Mutex::new(Some(2.5)))).lock().unwrap().as_ref().unwrap())), format!("{}", (*__go2rust_gosyntax0_5(Arc::new(Mutex::new(Some(1e+21)))).lock().unwrap().as_ref().unwrap())), format!("{}", (*__go2rust_gosyntax0_20(Arc::new(Mutex::new(Some(0.1 as f32)))).lock().unwrap().as_ref().unwrap())), format!("{}", (*__go2rust_gosyntax0_8(Arc::new(Mutex::new(Some("tab\there \"q\"".to_string())))).lock().unwrap().as_ref().unwrap())));
    print!("{}|{}|{}\n", format!("{}", (*__go2rust_gosyntax0_12(Arc::new(Mutex::new(Some(true)))).lock().unwrap().as_ref().unwrap())), format!("{}", (*__go2rust_gosyntax0_21(Arc::new(Mutex::new(Some(('x' as i32) as i32)))).lock().unwrap().as_ref().unwrap())), format!("{}", (*__go2rust_gosyntax0_22(Arc::new(Mutex::new(Some(("hi".to_string()).chars().map(|c| c as i32).collect::<Vec<_>>())))).lock().unwrap().as_ref().unwrap())));

    let mut r = Arc::new(Mutex::new(Some(Record {
        base: Arc::new(Mutex::new(Some(Base { i_d: Arc::new(Mutex::new(Some(7))), ..Default::default() }))),
        name: Arc::new(Mutex::new(Some("first".to_string()))),
        tags: Arc::new(Mutex::new(Some(vec!["a".to_string(), "b".to_string()]))),
        scores: Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("z".to_string(), Arc::new(Mutex::new(Some(26i64)))), ("a".to_string(), Arc::new(Mutex::new(Some(1i64)))), ("m".to_string(), Arc::new(Mutex::new(Some(13i64))))])))),
        flags: Arc::new(Mutex::new(Some(BTreeMap::<bool, Arc<Mutex<Option<String>>>>::from([(true, Arc::new(Mutex::new(Some("yes".to_string())))), (false, Arc::new(Mutex::new(Some("no".to_string()))))])))),
        grid: Arc::new(Mutex::new(Some([[1i64, 2i64], [3i64, 4i64]]))),
        data: Arc::new(Mutex::new(Some(("hi".to_string()).as_bytes().to_go_slice()))),
        count: Arc::new(Mutex::new(Some(10 as u64))),
        ratio: Arc::new(Mutex::new(Some(0.25))),
        any: Arc::new(Mutex::new(Some(Box::new(Point { x: Arc::new(Mutex::new(Some(1))), y: Arc::new(Mutex::new(Some(2))), ..Default::default() }) as Box<dyn Any + Send + Sync>))),
        hidden: Arc::new(Mutex::new(Some(9))),
        ..Default::default()
    })));
    print!("{}\n", (*__go2rust_gosyntax0_23(Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() })))).lock().unwrap().as_ref().unwrap()));
    print!("{}\n", (*__go2rust_gosyntax0_43(Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(3))), y: Arc::new(Mutex::new(Some(4))), ..Default::default() })))).lock().unwrap().as_ref().unwrap()));
    let mut nilPoint: Arc<Mutex<Option<Point>>> = Arc::new(Mutex::new(None));
    print!("{} {}\n", format!("{}", (*__go2rust_gosyntax0_43(nilPoint.clone()).lock().unwrap().as_ref().unwrap())), format!("{}", (*__go2rust_gosyntax0_44(Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(None))])))).lock().unwrap().as_ref().unwrap())));

    print!("{}|{}|{}\n", format!("{}", (*__go2rust_gosyntax0_45(Arc::new(Mutex::new(Some(Version { major: Arc::new(Mutex::new(Some(1))), minor: Arc::new(Mutex::new(Some(2))), ..Default::default() })))).lock().unwrap().as_ref().unwrap())), format!("{}", (*__go2rust_gosyntax0_46(Arc::new(Mutex::new(Some(vec![Version { major: Arc::new(Mutex::new(Some(2))), minor: Arc::new(Mutex::new(Some(0))), ..Default::default() }])))).lock().unwrap().as_ref().unwrap())), format!("{}", (*__go2rust_gosyntax0_47(Arc::new(Mutex::new(Some(Handle { id: Arc::new(Mutex::new(Some(5))), ..Default::default() })))).lock().unwrap().as_ref().unwrap())));

    let mut shapes: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Shape + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));
    { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = shapes.clone(); let __append_values = vec![
        Arc::new(Mutex::new(Some(Box::new(Square { side: Arc::new(Mutex::new(Some(2.0))), ..Default::default() }) as Box<dyn Shape + Send + Sync>))),
        Arc::new(Mutex::new(Some(Box::new(CirclePtr(Arc::new(Mutex::new(Some(Circle { r: Arc::new(Mutex::new(Some(1.0))), ..Default::default() }))).clone())) as Box<dyn Shape + Send + Sync>))),
    ]; let mut __appended: Vec<Arc<Mutex<Option<Box<dyn Shape + Send + Sync>>>>> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.extend(__append_values); Arc::new(Mutex::new(Some(__appended))) })); *shapes.lock().unwrap() = __slice_header; };
    { let __range_holder = shapes.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for s in __range_values.iter() {
        print!("{}\n", (*__go2rust_gosyntax0_48(s.clone()).lock().unwrap().as_ref().unwrap()));
    } }
    let mut none: Arc<Mutex<Option<Box<dyn Shape + Send + Sync>>>> = Arc::new(Mutex::new(None));
    print!("{} {}\n", format!("{}", (*__go2rust_gosyntax0_48(none.clone()).lock().unwrap().as_ref().unwrap())), format!("{}", (*__go2rust_gosyntax0_48({ let __field = (*r.lock().unwrap().as_ref().unwrap()).shape.clone(); __field }).lock().unwrap().as_ref().unwrap())));

    show(Arc::new(Mutex::new(Some("int".to_string()))), Arc::new(Mutex::new(Some(Box::new(3i64) as Box<dyn Any + Send + Sync>))));
    show(Arc::new(Mutex::new(Some("string".to_string()))), Arc::new(Mutex::new(Some(Box::new("s".to_string()) as Box<dyn Any + Send + Sync>))));
    show(
        Arc::new(Mutex::new(Some("struct".to_string()))),
        Arc::new(Mutex::new(Some(Box::new(Square { side: Arc::new(Mutex::new(Some(3.0))), ..Default::default() }) as Box<dyn Any + Send + Sync>)))
    );
    show(
        Arc::new(Mutex::new(Some("pointer".to_string()))),
        Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(Square { side: Arc::new(Mutex::new(Some(4.0))), ..Default::default() }))).clone()) as Box<dyn Any + Send + Sync>)))
    );
    show(
        Arc::new(Mutex::new(Some("slice".to_string()))),
        Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(vec![1i64, 2i64]))).clone()) as Box<dyn Any + Send + Sync>)))
    );
    show(
        Arc::new(Mutex::new(Some("map".to_string()))),
        Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<bool>>>>::from([("x".to_string(), Arc::new(Mutex::new(Some(true))))])))).clone()) as Box<dyn Any + Send + Sync>)))
    );
    show(Arc::new(Mutex::new(Some("nil".to_string()))), Arc::new(Mutex::new(None::<Box<dyn Any + Send + Sync>>)));

    let mut items = Arc::new(Mutex::new(Some(vec![
        Box::new(1i64) as Box<dyn Any + Send + Sync>,
        Box::new("a".to_string()) as Box<dyn Any + Send + Sync>,
        Box::new(GoNilAny) as Box<dyn Any + Send + Sync>,
        Box::new(2.5) as Box<dyn Any + Send + Sync>,
        Box::new(Point { x: Arc::new(Mutex::new(Some(5))), y: Arc::new(Mutex::new(Some(6))), ..Default::default() }) as Box<dyn Any + Send + Sync>,
    ])));
    print!("{}\n", (*__go2rust_gosyntax0_49(items.clone()).lock().unwrap().as_ref().unwrap()));

    let mut keyed = Arc::new(Mutex::new(Some(BTreeMap::<Point, Arc<Mutex<Option<String>>>>::from([(Point { x: Arc::new(Mutex::new(Some(2))), y: Arc::new(Mutex::new(Some(1))), ..Default::default() }, Arc::new(Mutex::new(Some("b".to_string())))), (Point { x: Arc::new(Mutex::new(Some(1))), y: Arc::new(Mutex::new(Some(9))), ..Default::default() }, Arc::new(Mutex::new(Some("a".to_string())))), (Point { x: Arc::new(Mutex::new(Some(1))), y: Arc::new(Mutex::new(Some(2))), ..Default::default() }, Arc::new(Mutex::new(Some("c".to_string()))))]))));
    print!("{}\n", (*__go2rust_gosyntax0_50(keyed.clone()).lock().unwrap().as_ref().unwrap()));

    type local = AnonymousStruct1;
    let mut __go2rust_gosyntaxL0_53 = Arc::new(Mutex::new(Some(Arc::new(move |v: local| -> Arc<Mutex<Option<String>>> {
        let mut s = Arc::new(Mutex::new(Some("main.local{".to_string())));
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&format!("{}{}", "A:".to_string(), (*__go2rust_gosyntax0_3(Arc::new(Mutex::new(Some({ let __selector_holder = v.a.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap()))); };
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&format!("{}{}", ", B:".to_string(), (*__go2rust_gosyntax0_26({ let __field = v.b.clone(); __field }).lock().unwrap().as_ref().unwrap()))); };
        return Arc::new(Mutex::new(Some(format!("{}{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }, "}".to_string()))));
    }) as Arc<dyn Fn(local) -> Arc<Mutex<Option<String>>> + Send + Sync>)));
    let _ = &__go2rust_gosyntaxL0_53;
    let __go2rust_gosyntaxL0_53_closure_clone = __go2rust_gosyntaxL0_53.clone(); let mut __go2rust_gosyntaxL0_54 = Arc::new(Mutex::new(Some(Arc::new(move |v: Arc<Mutex<Option<Vec<local>>>>| -> Arc<Mutex<Option<String>>> {
        if { let __nil_result = (*v.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some("[]main.local(nil)".to_string())));
    }
        let mut s = Arc::new(Mutex::new(Some("[]main.local{".to_string())));
        let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*v.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&", ".to_string()); };
    }
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&{ let __s = {
            let __f: Arc<dyn Fn(local) -> Arc<Mutex<Option<String>>> + Send + Sync> = __go2rust_gosyntaxL0_53_closure_clone.lock().unwrap().as_ref().unwrap().clone();
            (__f)(
                { let __seq = { let __seq_holder = v.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }
            )
        }; let __value = (*__s.lock().unwrap().as_ref().unwrap()).clone(); __value }); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        return Arc::new(Mutex::new(Some(format!("{}{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }, "}".to_string()))));
    }) as Arc<dyn Fn(Arc<Mutex<Option<Vec<local>>>>) -> Arc<Mutex<Option<String>>> + Send + Sync>)));
    let _ = &__go2rust_gosyntaxL0_54;
    print!("{}|{}\n", format!("{}", (*{
    let __f: Arc<dyn Fn(local) -> Arc<Mutex<Option<String>>> + Send + Sync> = __go2rust_gosyntaxL0_53.lock().unwrap().as_ref().unwrap().clone();
    (__f)(
        local { a: Arc::new(Mutex::new(Some(1))), b: Arc::new(Mutex::new(None)), ..Default::default() }
    )
}.lock().unwrap().as_ref().unwrap())), format!("{}", (*{
    let __f: Arc<dyn Fn(Arc<Mutex<Option<Vec<local>>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> = __go2rust_gosyntaxL0_54.lock().unwrap().as_ref().unwrap().clone();
    (__f)(
        Arc::new(Mutex::new(Some(vec![local { a: Arc::new(Mutex::new(Some(2))), b: Arc::new(Mutex::new(Some(vec!["x".to_string()]))), ..Default::default() }])))
    )
}.lock().unwrap().as_ref().unwrap())));

    let mut p = Arc::new(Mutex::new(Some(Pair__string__int { key: Arc::new(Mutex::new(Some("k".to_string()))), val: Arc::new(Mutex::new(Some(1))), ..Default::default() })));
    print!("{} {}\n", format!("{}", (*__go2rust_gosyntax0_53(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() })))).lock().unwrap().as_ref().unwrap())), format!("{}", "main.Pair[string,int]"));

    print!("{}|{}|{}\n", format!("{}", (*__go2rust_gosyntax0_54(Arc::new(Mutex::new(Some(AnonymousStruct2 {  })))).lock().unwrap().as_ref().unwrap())), format!("{}", (*__go2rust_gosyntax0_55(Arc::new(Mutex::new(Some(AnonymousStruct3 { a: Arc::new(Mutex::new(Some(1))), b: Arc::new(Mutex::new(Some(2))) })))).lock().unwrap().as_ref().unwrap())), format!("{}", (*__go2rust_gosyntax0_56(Arc::new(Mutex::new(Some([])))).lock().unwrap().as_ref().unwrap())));

    let mut err = Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(format!("bad point {}", (*__go2rust_gosyntax0_2(Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(0))), y: Arc::new(Mutex::new(Some(-1))), ..Default::default() })))).lock().unwrap().as_ref().unwrap())))))));
    println!("{}", format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    print!("{} {} {}\n", format!("{}", 5), format!("{}", (*__go2rust_gosyntax0_26(Arc::new(Mutex::new(Some(Vec::<String>::new())))).lock().unwrap().as_ref().unwrap())), format!("{}", "end".to_string()));
    println!("{}", format!("{}", (*Arc::new(Mutex::new(Some(format!("{}", (*__go2rust_gosyntax0_57(Arc::new(Mutex::new(Some(BTreeMap::<i64, Arc<Mutex<Option<bool>>>>::from([(3i64, Arc::new(Mutex::new(Some(true)))), (1i64, Arc::new(Mutex::new(Some(false))))]))))).lock().unwrap().as_ref().unwrap()))))).lock().unwrap().as_ref().unwrap())));
    print!("{} {}\n", format!("{}", { let __text = match Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("boom".to_string()))))).lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", (*__go2rust_gosyntax0_58(Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>))).lock().unwrap().as_ref().unwrap())));

        // Typed nils as arguments, and a struct with a float field as a map key.
    let mut empty: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(None::<Box<dyn Any + Send + Sync>>)).clone();
    println!("{} {} {}", format!("{}", size(Arc::new(Mutex::new(Some(Tags(Arc::new(Mutex::new(None::<BTreeMap<String, Arc<Mutex<Option<i64>>>>>)))))))), format!("{}", is_nil(Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>)))), format!("{}", { let __nil_result = (*empty.lock().unwrap()).as_ref().map_or(true, |__any| (**__any).is::<GoNilAny>()); __nil_result }));
    let mut bySide = Arc::new(Mutex::new(Some(BTreeMap::<Square, Arc<Mutex<Option<i64>>>>::from([(Square { side: Arc::new(Mutex::new(Some(2.0))), ..Default::default() }, Arc::new(Mutex::new(Some(1i64)))), (Square { side: Arc::new(Mutex::new(Some(1.5))), ..Default::default() }, Arc::new(Mutex::new(Some(2i64))))]))));
    print!("{} {}\n", format!("{}", (*__go2rust_gosyntax0_59(bySide.clone()).lock().unwrap().as_ref().unwrap())), format!("{}", { let __map = { let __map_holder = bySide.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&Square { side: Arc::new(Mutex::new(Some(1.5))), ..Default::default() })).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));
}

pub fn __go2rust_gosyntax0_1(v: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
    if { let __nil_result = (*v.lock().unwrap()).as_ref().map_or(true, |__any| (**__any).is::<GoNilAny>()); __nil_result } {
        return Arc::new(Mutex::new(Some("<nil>".to_string())));
    }
    {
    let _ts_subject = v.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().map_or(true, |__v| (**__v).is::<GoNilAny>());
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| {
        let mut __any = __v.as_ref() as &dyn Any;
        while let Some(__boxed) = __any.downcast_ref::<Box<dyn Any + Send + Sync>>() {
            __any = __boxed.as_ref() as &dyn Any;
        }
        __any
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<Point>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<Point>()).unwrap().__go_value_clone())));
        std::mem::drop(_ts_guard);
        return __go2rust_gosyntax0_2(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Square>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<Square>()).unwrap().__go_value_clone())));
        std::mem::drop(_ts_guard);
        return __go2rust_gosyntax0_4(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<Circle>>>>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<Circle>>>>()).unwrap().clone();
        std::mem::drop(_ts_guard);
        return __go2rust_gosyntax0_6(x.clone());;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<i64>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<i64>()).unwrap().clone())));
        std::mem::drop(_ts_guard);
        return __go2rust_gosyntax0_3(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<String>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<String>()).unwrap().clone())));
        std::mem::drop(_ts_guard);
        return __go2rust_gosyntax0_8(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<Square>>>>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<Square>>>>()).unwrap().clone();
        std::mem::drop(_ts_guard);
        return __go2rust_gosyntax0_9(x.clone());;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<Vec<i64>>>>>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<Vec<i64>>>>>()).unwrap().clone();
        std::mem::drop(_ts_guard);
        let mut x = { let __slice_header = GoSliceHeader::go_slice_header(&x); Arc::new(Mutex::new(__slice_header)) };
        return __go2rust_gosyntax0_10(x.clone());;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<bool>>>>>>>>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<bool>>>>>>>>()).unwrap().clone();
        std::mem::drop(_ts_guard);
        return __go2rust_gosyntax0_11(x.clone());;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<f64>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<f64>()).unwrap().clone())));
        std::mem::drop(_ts_guard);
        return __go2rust_gosyntax0_5(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
    }
    }
    Arc::new(Mutex::new(Some(format!("{}", { let __text = match v.lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }))))
}

pub fn __go2rust_gosyntax0_2(v: Arc<Mutex<Option<Point>>>) -> Arc<Mutex<Option<String>>> {
    let mut s = Arc::new(Mutex::new(Some("main.Point{".to_string())));
    { (*s.lock().unwrap().as_mut().unwrap()).push_str(&format!("{}{}", "X:".to_string(), (*__go2rust_gosyntax0_3(Arc::new(Mutex::new(Some({ let __selector_holder = (*v.lock().unwrap().as_ref().unwrap()).x.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap()))); };
    { (*s.lock().unwrap().as_mut().unwrap()).push_str(&format!("{}{}", ", Y:".to_string(), (*__go2rust_gosyntax0_3(Arc::new(Mutex::new(Some({ let __selector_holder = (*v.lock().unwrap().as_ref().unwrap()).y.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap()))); };
    return Arc::new(Mutex::new(Some(format!("{}{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }, "}".to_string()))));
}

pub fn __go2rust_gosyntax0_3(v: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<String>>> {
    Arc::new(Mutex::new(Some(format!("{}", format!("{}", (*Arc::new(Mutex::new(Some(({ let __v = (*v.lock().unwrap().as_ref().unwrap()); __v }) as i64))).lock().unwrap().as_ref().unwrap()))))))
}

pub fn __go2rust_gosyntax0_4(v: Arc<Mutex<Option<Square>>>) -> Arc<Mutex<Option<String>>> {
    let mut s = Arc::new(Mutex::new(Some("main.Square{".to_string())));
    { (*s.lock().unwrap().as_mut().unwrap()).push_str(&format!("{}{}", "Side:".to_string(), (*__go2rust_gosyntax0_5(Arc::new(Mutex::new(Some({ let __selector_holder = (*v.lock().unwrap().as_ref().unwrap()).side.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap()))); };
    return Arc::new(Mutex::new(Some(format!("{}{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }, "}".to_string()))));
}

pub fn __go2rust_gosyntax0_5(v: Arc<Mutex<Option<f64>>>) -> Arc<Mutex<Option<String>>> {
    Arc::new(Mutex::new(Some(format!("{}", { let __text = go_float_string(((*Arc::new(Mutex::new(Some(({ let __v = (*v.lock().unwrap().as_ref().unwrap()); __v }) as f64))).lock().unwrap().as_ref().unwrap())) as f64, false); __text }))))
}

pub fn __go2rust_gosyntax0_6(v: Arc<Mutex<Option<Circle>>>) -> Arc<Mutex<Option<String>>> {
    if { let __nil_result = (*v.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some("(*main.Circle)(nil)".to_string())));
    }
    return Arc::new(Mutex::new(Some(format!("{}{}", "&".to_string(), (*__go2rust_gosyntax0_7(Arc::new(Mutex::new(Some({ let __v = (*v.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v })))).lock().unwrap().as_ref().unwrap())))));
}

pub fn __go2rust_gosyntax0_7(v: Arc<Mutex<Option<Circle>>>) -> Arc<Mutex<Option<String>>> {
    let mut s = Arc::new(Mutex::new(Some("main.Circle{".to_string())));
    { (*s.lock().unwrap().as_mut().unwrap()).push_str(&format!("{}{}", "R:".to_string(), (*__go2rust_gosyntax0_5(Arc::new(Mutex::new(Some({ let __selector_holder = (*v.lock().unwrap().as_ref().unwrap()).r.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap()))); };
    return Arc::new(Mutex::new(Some(format!("{}{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }, "}".to_string()))));
}

pub fn __go2rust_gosyntax0_8(v: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    Arc::new(Mutex::new(Some(format!("{}", go_sprintf(&"%q".to_string(), &[{ let __go_format_arg = Box::new({ let __v = Arc::new(Mutex::new(Some({ let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }.to_string()))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>; __go_format_arg }], false)))))
}

pub fn __go2rust_gosyntax0_9(v: Arc<Mutex<Option<Square>>>) -> Arc<Mutex<Option<String>>> {
    if { let __nil_result = (*v.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some("(*main.Square)(nil)".to_string())));
    }
    return Arc::new(Mutex::new(Some(format!("{}{}", "&".to_string(), (*__go2rust_gosyntax0_4(Arc::new(Mutex::new(Some({ let __v = (*v.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v })))).lock().unwrap().as_ref().unwrap())))));
}

pub fn __go2rust_gosyntax0_10(v: Arc<Mutex<Option<Vec<i64>>>>) -> Arc<Mutex<Option<String>>> {
    if { let __nil_result = (*v.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some("[]int(nil)".to_string())));
    }
    let mut s = Arc::new(Mutex::new(Some("[]int{".to_string())));
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*v.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&", ".to_string()); };
    }
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&{ let __s = __go2rust_gosyntax0_3(Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = v.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))); let __value = (*__s.lock().unwrap().as_ref().unwrap()).clone(); __value }); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return Arc::new(Mutex::new(Some(format!("{}{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }, "}".to_string()))));
}

pub fn __go2rust_gosyntax0_11(v: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<bool>>>>>>>) -> Arc<Mutex<Option<String>>> {
    if { let __nil_result = (*v.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some("map[string]bool(nil)".to_string())));
    }
    let mut keys = Arc::new(Mutex::new(Some({ let mut v: Vec<String> = (0..(({ let __len = (*v.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })) as usize).map(|_| "".to_string()).collect(); v.truncate((0) as usize); v })));
    for (k, _) in { let __range_holder = v.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = keys.clone(); let __append_value = k.clone(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *keys.lock().unwrap() = __slice_header; };
    }
    let mut i = Arc::new(Mutex::new(Some(1i64)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*keys.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x < __tmp_y } {
        let mut j = { let __owned = i.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    while {
        let __go_cond_0 = { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y };
        if __go_cond_0 {
            let __go_cond_1 = {
                let __tmp_x = { let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
                let __tmp_y = { let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
                __tmp_x < __tmp_y
            };
            __go_cond_1
        } else {
            false
        }
    } {
        {
            let __go_lhs_sequence_0 = keys.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = keys.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    let mut s = Arc::new(Mutex::new(Some("map[string]bool{".to_string())));
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*keys.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&", ".to_string()); };
    }
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&{
            let mut __s = String::new();
            __s.push_str(&format!("{}", (*__go2rust_gosyntax0_8(Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))).lock().unwrap().as_ref().unwrap())));
            __s.push_str(&format!("{}", ":".to_string()));
            __s.push_str(&format!("{}", (*__go2rust_gosyntax0_12(Arc::new(Mutex::new(Some({ let __arg_value = { let __map = { let __map_holder = v.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&{ let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| false) }; __arg_value })))).lock().unwrap().as_ref().unwrap())));
            __s
        }); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return Arc::new(Mutex::new(Some(format!("{}{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }, "}".to_string()))));
}

pub fn __go2rust_gosyntax0_12(v: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<String>>> {
    if { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return Arc::new(Mutex::new(Some("true".to_string())));
    }
    Arc::new(Mutex::new(Some("false".to_string())))
}

pub fn __go2rust_gosyntax0_13(v: Arc<Mutex<Option<Level>>>) -> Arc<Mutex<Option<String>>> {
    Arc::new(Mutex::new(Some(format!("{}", format!("{}", (*Arc::new(Mutex::new(Some({ let __named_value = (*{ let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()); __named_value } as i64))).lock().unwrap().as_ref().unwrap()))))))
}

pub fn __go2rust_gosyntax0_14(v: Arc<Mutex<Option<Name>>>) -> Arc<Mutex<Option<String>>> {
    Arc::new(Mutex::new(Some(format!("{}", go_sprintf(&"%q".to_string(), &[{ let __go_format_arg = Box::new({ let __v = Arc::new(Mutex::new(Some((*(*v.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()).clone()))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>; __go_format_arg }], false)))))
}

pub fn __go2rust_gosyntax0_15(v: Arc<Mutex<Option<Celsius>>>) -> Arc<Mutex<Option<String>>> {
    Arc::new(Mutex::new(Some(format!("{}", { let __text = go_float_string(((*Arc::new(Mutex::new(Some({ let __named_float = *{ let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap(); __named_float } as f64))).lock().unwrap().as_ref().unwrap())) as f64, false); __text }))))
}

pub fn __go2rust_gosyntax0_16(v: Arc<Mutex<Option<IDs>>>) -> Arc<Mutex<Option<String>>> {
    if { let __slice_holder = { let __named_slice = v.lock().unwrap().as_ref().map(|__v| __v.0.clone()).unwrap_or_else(|| Arc::new(Mutex::new(None))); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); (*__slice_guard).is_none() } {
        return Arc::new(Mutex::new(Some("main.IDs(nil)".to_string())));
    }
    let mut s = Arc::new(Mutex::new(Some("main.IDs{".to_string())));
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = ({ let __slice_holder = { let __named_slice = (*v.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64); __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&", ".to_string()); };
    }
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&{ let __s = __go2rust_gosyntax0_3(Arc::new(Mutex::new(Some({ let __seq_holder = { let __named_slice = (*v.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_index = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() })))); let __value = (*__s.lock().unwrap().as_ref().unwrap()).clone(); __value }); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return Arc::new(Mutex::new(Some(format!("{}{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }, "}".to_string()))));
}

pub fn __go2rust_gosyntax0_17(v: Arc<Mutex<Option<Tags>>>) -> Arc<Mutex<Option<String>>> {
    if { let __map_holder = { let __named_map = (*v.lock().unwrap().as_ref().unwrap()).0.clone(); __named_map }; let __map_guard = __map_holder.lock().unwrap(); (*__map_guard).is_none() } {
        return Arc::new(Mutex::new(Some("main.Tags(nil)".to_string())));
    }
    let mut keys = Arc::new(Mutex::new(Some({ let mut v: Vec<String> = (0..({ let __named_map_holder = v.clone(); let __named_map_guard = __named_map_holder.lock().unwrap(); let __map_holder = __named_map_guard.as_ref().map(|__v| __v.0.clone()); std::mem::drop(__named_map_guard); __map_holder.as_ref().map(|__map_holder| { let __map_guard = __map_holder.lock().unwrap(); __map_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }).unwrap_or(0) }) as usize).map(|_| "".to_string()).collect(); v.truncate((0) as usize); v })));
    for (k, _) in { let __range_holder = { let __named_map_holder = v.clone(); let __named_map_guard = __named_map_holder.lock().unwrap(); let __map_holder = __named_map_guard.as_ref().map(|__v| __v.0.clone()).unwrap_or_else(|| Arc::new(Mutex::new(None))); std::mem::drop(__named_map_guard); __map_holder }; let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = keys.clone(); let __append_value = k.clone(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *keys.lock().unwrap() = __slice_header; };
    }
    let mut i = Arc::new(Mutex::new(Some(1i64)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*keys.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x < __tmp_y } {
        let mut j = { let __owned = i.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    while {
        let __go_cond_0 = { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y };
        if __go_cond_0 {
            let __go_cond_1 = {
                let __tmp_x = { let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
                let __tmp_y = { let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
                __tmp_x < __tmp_y
            };
            __go_cond_1
        } else {
            false
        }
    } {
        {
            let __go_lhs_sequence_0 = keys.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = keys.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    let mut s = Arc::new(Mutex::new(Some("main.Tags{".to_string())));
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*keys.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&", ".to_string()); };
    }
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&{
            let mut __s = String::new();
            __s.push_str(&format!("{}", (*__go2rust_gosyntax0_8(Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))).lock().unwrap().as_ref().unwrap())));
            __s.push_str(&format!("{}", ":".to_string()));
            __s.push_str(&format!("{}", (*__go2rust_gosyntax0_3(Arc::new(Mutex::new(Some({ let __arg_value = { let __map = { let __map_holder = { let __named_map = (*v.lock().unwrap().as_ref().unwrap()).0.clone(); __named_map }.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&{ let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }; __arg_value })))).lock().unwrap().as_ref().unwrap())));
            __s
        }); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return Arc::new(Mutex::new(Some(format!("{}{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }, "}".to_string()))));
}

pub fn __go2rust_gosyntax0_18(v: Arc<Mutex<Option<u8>>>) -> Arc<Mutex<Option<String>>> {
    Arc::new(Mutex::new(Some(format!("0x{}", format!("{:x}", (*Arc::new(Mutex::new(Some(({ let __v = (*v.lock().unwrap().as_ref().unwrap()); __v }) as u64))).lock().unwrap().as_ref().unwrap()))))))
}

pub fn __go2rust_gosyntax0_19(v: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<String>>> {
    Arc::new(Mutex::new(Some(format!("0x{}", format!("{:x}", (*Arc::new(Mutex::new(Some(({ let __v = (*v.lock().unwrap().as_ref().unwrap()); __v }) as u64))).lock().unwrap().as_ref().unwrap()))))))
}

pub fn __go2rust_gosyntax0_20(v: Arc<Mutex<Option<f32>>>) -> Arc<Mutex<Option<String>>> {
    Arc::new(Mutex::new(Some(format!("{}", { let __text = go_float_string(((*Arc::new(Mutex::new(Some(({ let __v = (*v.lock().unwrap().as_ref().unwrap()); __v }) as f32))).lock().unwrap().as_ref().unwrap())) as f64, true); __text }))))
}

pub fn __go2rust_gosyntax0_21(v: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<String>>> {
    Arc::new(Mutex::new(Some(format!("{}", format!("{}", (*Arc::new(Mutex::new(Some(({ let __v = (*v.lock().unwrap().as_ref().unwrap()); __v }) as i64))).lock().unwrap().as_ref().unwrap()))))))
}

pub fn __go2rust_gosyntax0_22(v: Arc<Mutex<Option<Vec<i32>>>>) -> Arc<Mutex<Option<String>>> {
    if { let __nil_result = (*v.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some("[]int32(nil)".to_string())));
    }
    let mut s = Arc::new(Mutex::new(Some("[]int32{".to_string())));
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*v.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&", ".to_string()); };
    }
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&{ let __s = __go2rust_gosyntax0_21(Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = v.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))); let __value = (*__s.lock().unwrap().as_ref().unwrap()).clone(); __value }); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return Arc::new(Mutex::new(Some(format!("{}{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }, "}".to_string()))));
}

pub fn __go2rust_gosyntax0_23(v: Arc<Mutex<Option<Record>>>) -> Arc<Mutex<Option<String>>> {
    let mut s = Arc::new(Mutex::new(Some("main.Record{".to_string())));
    { (*s.lock().unwrap().as_mut().unwrap()).push_str(&format!("{}{}", "Base:".to_string(), (*__go2rust_gosyntax0_24(Arc::new(Mutex::new(Some({ let __selector_holder = (*v.lock().unwrap().as_ref().unwrap()).base.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = __selector_guard.as_ref().unwrap().__go_value_clone(); std::mem::drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap()))); };
    { (*s.lock().unwrap().as_mut().unwrap()).push_str(&format!("{}{}", ", Point:".to_string(), (*__go2rust_gosyntax0_25({ let __field = (*v.lock().unwrap().as_ref().unwrap()).point.clone(); __field }).lock().unwrap().as_ref().unwrap()))); };
    { (*s.lock().unwrap().as_mut().unwrap()).push_str(&format!("{}{}", ", Name:".to_string(), (*__go2rust_gosyntax0_8(Arc::new(Mutex::new(Some({ let __selector_holder = (*v.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap()))); };
    { (*s.lock().unwrap().as_mut().unwrap()).push_str(&format!("{}{}", ", Tags:".to_string(), (*__go2rust_gosyntax0_26({ let __field = (*v.lock().unwrap().as_ref().unwrap()).tags.clone(); __field }).lock().unwrap().as_ref().unwrap()))); };
    { (*s.lock().unwrap().as_mut().unwrap()).push_str(&format!("{}{}", ", Scores:".to_string(), (*__go2rust_gosyntax0_27({ let __field = (*v.lock().unwrap().as_ref().unwrap()).scores.clone(); __field }).lock().unwrap().as_ref().unwrap()))); };
    { (*s.lock().unwrap().as_mut().unwrap()).push_str(&format!("{}{}", ", Flags:".to_string(), (*__go2rust_gosyntax0_28({ let __field = (*v.lock().unwrap().as_ref().unwrap()).flags.clone(); __field }).lock().unwrap().as_ref().unwrap()))); };
    { (*s.lock().unwrap().as_mut().unwrap()).push_str(&format!("{}{}", ", Grid:".to_string(), (*__go2rust_gosyntax0_30(Arc::new(Mutex::new(Some({ let __selector_holder = (*v.lock().unwrap().as_ref().unwrap()).grid.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap()))); };
    { (*s.lock().unwrap().as_mut().unwrap()).push_str(&format!("{}{}", ", Next:".to_string(), (*__go2rust_gosyntax0_32({ let __field = (*v.lock().unwrap().as_ref().unwrap()).next.clone(); __field }).lock().unwrap().as_ref().unwrap()))); };
    { (*s.lock().unwrap().as_mut().unwrap()).push_str(&format!("{}{}", ", Data:".to_string(), (*__go2rust_gosyntax0_33({ let __field = (*v.lock().unwrap().as_ref().unwrap()).data.clone(); __field }).lock().unwrap().as_ref().unwrap()))); };
    { (*s.lock().unwrap().as_mut().unwrap()).push_str(&format!("{}{}", ", Count:".to_string(), (*__go2rust_gosyntax0_35(Arc::new(Mutex::new(Some({ let __selector_holder = (*v.lock().unwrap().as_ref().unwrap()).count.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap()))); };
    { (*s.lock().unwrap().as_mut().unwrap()).push_str(&format!("{}{}", ", Ratio:".to_string(), (*__go2rust_gosyntax0_5(Arc::new(Mutex::new(Some({ let __selector_holder = (*v.lock().unwrap().as_ref().unwrap()).ratio.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap()))); };
    { (*s.lock().unwrap().as_mut().unwrap()).push_str(&format!("{}{}", ", Any:".to_string(), (*__go2rust_gosyntax0_36(go_any_handle_copy(&(*v.lock().unwrap().as_ref().unwrap()).any)).lock().unwrap().as_ref().unwrap()))); };
    { (*s.lock().unwrap().as_mut().unwrap()).push_str(&format!("{}{}", ", Err:".to_string(), (*__go2rust_gosyntax0_39({ let __err_source = { let __field = (*v.lock().unwrap().as_ref().unwrap()).err.clone(); __field }; let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }).lock().unwrap().as_ref().unwrap()))); };
    { (*s.lock().unwrap().as_mut().unwrap()).push_str(&format!("{}{}", ", Shape:".to_string(), (*__go2rust_gosyntax0_40({ let __field = (*v.lock().unwrap().as_ref().unwrap()).shape.clone(); __field }).lock().unwrap().as_ref().unwrap()))); };
    { (*s.lock().unwrap().as_mut().unwrap()).push_str(&format!("{}{}", ", OnDone:".to_string(), (*__go2rust_gosyntax0_41(Arc::new(Mutex::new(GoFuncValue::go_func_value(&((*v.lock().unwrap().as_ref().unwrap()).on_done))))).lock().unwrap().as_ref().unwrap()))); };
    { (*s.lock().unwrap().as_mut().unwrap()).push_str(&format!("{}{}", ", Events:".to_string(), (*__go2rust_gosyntax0_42({ let __field = (*v.lock().unwrap().as_ref().unwrap()).events.clone(); __field }).lock().unwrap().as_ref().unwrap()))); };
    { (*s.lock().unwrap().as_mut().unwrap()).push_str(&format!("{}{}", ", hidden:".to_string(), (*__go2rust_gosyntax0_3(Arc::new(Mutex::new(Some({ let __selector_holder = (*v.lock().unwrap().as_ref().unwrap()).hidden.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap()))); };
    return Arc::new(Mutex::new(Some(format!("{}{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }, "}".to_string()))));
}

pub fn __go2rust_gosyntax0_24(v: Arc<Mutex<Option<Base>>>) -> Arc<Mutex<Option<String>>> {
    let mut s = Arc::new(Mutex::new(Some("main.Base{".to_string())));
    { (*s.lock().unwrap().as_mut().unwrap()).push_str(&format!("{}{}", "ID:".to_string(), (*__go2rust_gosyntax0_3(Arc::new(Mutex::new(Some({ let __selector_holder = (*v.lock().unwrap().as_ref().unwrap()).i_d.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap()))); };
    return Arc::new(Mutex::new(Some(format!("{}{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }, "}".to_string()))));
}

pub fn __go2rust_gosyntax0_25(v: Arc<Mutex<Option<Point>>>) -> Arc<Mutex<Option<String>>> {
    if { let __nil_result = (*v.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some("(*main.Point)(nil)".to_string())));
    }
    return Arc::new(Mutex::new(Some({
        let mut __s = String::new();
        __s.push_str(&format!("{}", "(*main.Point)(".to_string()));
        __s.push_str(&format!("{}", (*Arc::new(Mutex::new(Some(format!("{:p}", Arc::as_ptr(&v))))).lock().unwrap().as_ref().unwrap())));
        __s.push_str(&format!("{}", ")".to_string()));
        __s
    })));
}

pub fn __go2rust_gosyntax0_26(v: Arc<Mutex<Option<Vec<String>>>>) -> Arc<Mutex<Option<String>>> {
    if { let __nil_result = (*v.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some("[]string(nil)".to_string())));
    }
    let mut s = Arc::new(Mutex::new(Some("[]string{".to_string())));
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*v.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&", ".to_string()); };
    }
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&{ let __s = __go2rust_gosyntax0_8(Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = v.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))); let __value = (*__s.lock().unwrap().as_ref().unwrap()).clone(); __value }); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return Arc::new(Mutex::new(Some(format!("{}{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }, "}".to_string()))));
}

pub fn __go2rust_gosyntax0_27(v: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>) -> Arc<Mutex<Option<String>>> {
    if { let __nil_result = (*v.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some("map[string]int(nil)".to_string())));
    }
    let mut keys = Arc::new(Mutex::new(Some({ let mut v: Vec<String> = (0..(({ let __len = (*v.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })) as usize).map(|_| "".to_string()).collect(); v.truncate((0) as usize); v })));
    for (k, _) in { let __range_holder = v.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = keys.clone(); let __append_value = k.clone(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *keys.lock().unwrap() = __slice_header; };
    }
    let mut i = Arc::new(Mutex::new(Some(1i64)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*keys.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x < __tmp_y } {
        let mut j = { let __owned = i.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    while {
        let __go_cond_0 = { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y };
        if __go_cond_0 {
            let __go_cond_1 = {
                let __tmp_x = { let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
                let __tmp_y = { let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
                __tmp_x < __tmp_y
            };
            __go_cond_1
        } else {
            false
        }
    } {
        {
            let __go_lhs_sequence_0 = keys.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = keys.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    let mut s = Arc::new(Mutex::new(Some("map[string]int{".to_string())));
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*keys.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&", ".to_string()); };
    }
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&{
            let mut __s = String::new();
            __s.push_str(&format!("{}", (*__go2rust_gosyntax0_8(Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))).lock().unwrap().as_ref().unwrap())));
            __s.push_str(&format!("{}", ":".to_string()));
            __s.push_str(&format!("{}", (*__go2rust_gosyntax0_3(Arc::new(Mutex::new(Some({ let __arg_value = { let __map = { let __map_holder = v.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&{ let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }; __arg_value })))).lock().unwrap().as_ref().unwrap())));
            __s
        }); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return Arc::new(Mutex::new(Some(format!("{}{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }, "}".to_string()))));
}

pub fn __go2rust_gosyntax0_28(v: Arc<Mutex<Option<BTreeMap<bool, Arc<Mutex<Option<String>>>>>>>) -> Arc<Mutex<Option<String>>> {
    if { let __nil_result = (*v.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some("map[bool]string(nil)".to_string())));
    }
    let mut keys = Arc::new(Mutex::new(Some({ let mut v: Vec<bool> = (0..(({ let __len = (*v.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })) as usize).map(|_| false).collect(); v.truncate((0) as usize); v })));
    for (k, _) in { let __range_holder = v.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = keys.clone(); let __append_value = k; let mut __appended: Vec<bool> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *keys.lock().unwrap() = __slice_header; };
    }
    let mut i = Arc::new(Mutex::new(Some(1i64)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*keys.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x < __tmp_y } {
        let mut j = { let __owned = i.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    while {
        let __go_cond_0 = { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y };
        if __go_cond_0 {
            let __go_cond_1 = {
                let __tmp_x =
                    __go2rust_gosyntaxcmp0_29(
                        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value })))
                    );
                let __tmp_y = 0;
                __tmp_x < __tmp_y
            };
            __go_cond_1
        } else {
            false
        }
    } {
        {
            let __go_lhs_sequence_0 = keys.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = keys.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    let mut s = Arc::new(Mutex::new(Some("map[bool]string{".to_string())));
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*keys.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&", ".to_string()); };
    }
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&{
            let mut __s = String::new();
            __s.push_str(&format!("{}", (*__go2rust_gosyntax0_12(Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))).lock().unwrap().as_ref().unwrap())));
            __s.push_str(&format!("{}", ":".to_string()));
            __s.push_str(&format!("{}", (*__go2rust_gosyntax0_8(Arc::new(Mutex::new(Some({ let __arg_value = { let __map = { let __map_holder = v.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&{ let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()) }; __arg_value })))).lock().unwrap().as_ref().unwrap())));
            __s
        }); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return Arc::new(Mutex::new(Some(format!("{}{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }, "}".to_string()))));
}

pub fn __go2rust_gosyntaxcmp0_29(a: Arc<Mutex<Option<bool>>>, b: Arc<Mutex<Option<bool>>>) -> i64 {
    if { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        return 0;
    }
    if !{ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return -(1);
    }
    1
}

pub fn __go2rust_gosyntax0_30(v: Arc<Mutex<Option<[[i64; 2]; 2]>>>) -> Arc<Mutex<Option<String>>> {
    let mut s = Arc::new(Mutex::new(Some("[2][2]int{".to_string())));
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = 2; __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&", ".to_string()); };
    }
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&{ let __s = __go2rust_gosyntax0_31(Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = v.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))); let __value = (*__s.lock().unwrap().as_ref().unwrap()).clone(); __value }); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return Arc::new(Mutex::new(Some(format!("{}{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }, "}".to_string()))));
}

pub fn __go2rust_gosyntax0_31(v: Arc<Mutex<Option<[i64; 2]>>>) -> Arc<Mutex<Option<String>>> {
    let mut s = Arc::new(Mutex::new(Some("[2]int{".to_string())));
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = 2; __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&", ".to_string()); };
    }
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&{ let __s = __go2rust_gosyntax0_3(Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = v.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))); let __value = (*__s.lock().unwrap().as_ref().unwrap()).clone(); __value }); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return Arc::new(Mutex::new(Some(format!("{}{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }, "}".to_string()))));
}

pub fn __go2rust_gosyntax0_32(v: Arc<Mutex<Option<Record>>>) -> Arc<Mutex<Option<String>>> {
    if { let __nil_result = (*v.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some("(*main.Record)(nil)".to_string())));
    }
    return Arc::new(Mutex::new(Some({
        let mut __s = String::new();
        __s.push_str(&format!("{}", "(*main.Record)(".to_string()));
        __s.push_str(&format!("{}", (*Arc::new(Mutex::new(Some(format!("{:p}", Arc::as_ptr(&v))))).lock().unwrap().as_ref().unwrap())));
        __s.push_str(&format!("{}", ")".to_string()));
        __s
    })));
}

pub fn __go2rust_gosyntax0_33(v: Arc<Mutex<Option<Vec<u8>>>>) -> Arc<Mutex<Option<String>>> {
    if { let __nil_result = (*v.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some("[]uint8(nil)".to_string())));
    }
    let mut s = Arc::new(Mutex::new(Some("[]uint8{".to_string())));
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*v.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&", ".to_string()); };
    }
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&{ let __s = __go2rust_gosyntax0_34(Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = v.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))); let __value = (*__s.lock().unwrap().as_ref().unwrap()).clone(); __value }); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return Arc::new(Mutex::new(Some(format!("{}{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }, "}".to_string()))));
}

pub fn __go2rust_gosyntax0_34(v: Arc<Mutex<Option<u8>>>) -> Arc<Mutex<Option<String>>> {
    Arc::new(Mutex::new(Some(format!("0x{}", format!("{:x}", (*Arc::new(Mutex::new(Some(({ let __v = (*v.lock().unwrap().as_ref().unwrap()); __v }) as u64))).lock().unwrap().as_ref().unwrap()))))))
}

pub fn __go2rust_gosyntax0_35(v: Arc<Mutex<Option<u64>>>) -> Arc<Mutex<Option<String>>> {
    Arc::new(Mutex::new(Some(format!("0x{}", format!("{:x}", (*Arc::new(Mutex::new(Some(({ let __v = (*v.lock().unwrap().as_ref().unwrap()); __v }) as u64))).lock().unwrap().as_ref().unwrap()))))))
}

pub fn __go2rust_gosyntax0_36(v: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
    if { let __nil_result = (*v.lock().unwrap()).as_ref().map_or(true, |__any| (**__any).is::<GoNilAny>()); __nil_result } {
        return Arc::new(Mutex::new(Some("interface {}(nil)".to_string())));
    }
    {
    let _ts_subject = v.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().map_or(true, |__v| (**__v).is::<GoNilAny>());
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| {
        let mut __any = __v.as_ref() as &dyn Any;
        while let Some(__boxed) = __any.downcast_ref::<Box<dyn Any + Send + Sync>>() {
            __any = __boxed.as_ref() as &dyn Any;
        }
        __any
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<Point>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<Point>()).unwrap().__go_value_clone())));
        std::mem::drop(_ts_guard);
        return __go2rust_gosyntax0_2(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Square>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<Square>()).unwrap().__go_value_clone())));
        std::mem::drop(_ts_guard);
        return __go2rust_gosyntax0_4(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<Circle>>>>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<Circle>>>>()).unwrap().clone();
        std::mem::drop(_ts_guard);
        return __go2rust_gosyntax0_37(x.clone());;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<i64>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<i64>()).unwrap().clone())));
        std::mem::drop(_ts_guard);
        return __go2rust_gosyntax0_3(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<String>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<String>()).unwrap().clone())));
        std::mem::drop(_ts_guard);
        return __go2rust_gosyntax0_8(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<Square>>>>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<Square>>>>()).unwrap().clone();
        std::mem::drop(_ts_guard);
        return __go2rust_gosyntax0_38(x.clone());;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<Vec<i64>>>>>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<Vec<i64>>>>>()).unwrap().clone();
        std::mem::drop(_ts_guard);
        let mut x = { let __slice_header = GoSliceHeader::go_slice_header(&x); Arc::new(Mutex::new(__slice_header)) };
        return __go2rust_gosyntax0_10(x.clone());;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<bool>>>>>>>>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<bool>>>>>>>>()).unwrap().clone();
        std::mem::drop(_ts_guard);
        return __go2rust_gosyntax0_11(x.clone());;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<f64>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<f64>()).unwrap().clone())));
        std::mem::drop(_ts_guard);
        return __go2rust_gosyntax0_5(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
    }
    }
    Arc::new(Mutex::new(Some(format!("{}", { let __text = match v.lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }))))
}

pub fn __go2rust_gosyntax0_37(v: Arc<Mutex<Option<Circle>>>) -> Arc<Mutex<Option<String>>> {
    if { let __nil_result = (*v.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some("(*main.Circle)(nil)".to_string())));
    }
    return Arc::new(Mutex::new(Some({
        let mut __s = String::new();
        __s.push_str(&format!("{}", "(*main.Circle)(".to_string()));
        __s.push_str(&format!("{}", (*Arc::new(Mutex::new(Some(format!("{:p}", Arc::as_ptr(&v))))).lock().unwrap().as_ref().unwrap())));
        __s.push_str(&format!("{}", ")".to_string()));
        __s
    })));
}

pub fn __go2rust_gosyntax0_38(v: Arc<Mutex<Option<Square>>>) -> Arc<Mutex<Option<String>>> {
    if { let __nil_result = (*v.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some("(*main.Square)(nil)".to_string())));
    }
    return Arc::new(Mutex::new(Some({
        let mut __s = String::new();
        __s.push_str(&format!("{}", "(*main.Square)(".to_string()));
        __s.push_str(&format!("{}", (*Arc::new(Mutex::new(Some(format!("{:p}", Arc::as_ptr(&v))))).lock().unwrap().as_ref().unwrap())));
        __s.push_str(&format!("{}", ")".to_string()));
        __s
    })));
}

pub fn __go2rust_gosyntax0_39(v: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
    if { let __nil_result = (*v.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some("error(nil)".to_string())));
    }
    Arc::new(Mutex::new(Some(format!("{}", { let __text = match v.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }))))
}

pub fn __go2rust_gosyntax0_40(v: Arc<Mutex<Option<Box<dyn Shape + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
    if { let __nil_result = (*v.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some("main.Shape(nil)".to_string())));
    }
    {
    let _ts_subject = v.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    std::mem::drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Shape + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<Square>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<Square>()).unwrap().__go_value_clone())));
        return __go2rust_gosyntax0_4(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<CirclePtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<CirclePtr>()).unwrap().0.clone();
        return __go2rust_gosyntax0_37(x.clone());;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<SquarePtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<SquarePtr>()).unwrap().0.clone();
        return __go2rust_gosyntax0_38(x.clone());;
    }
    }
    Arc::new(Mutex::new(Some(format!("{}", match v.lock().unwrap().as_ref() { Some(__iface) => __iface.as_ref().__go_print(false), None => "<nil>".to_string() }))))
}

pub fn __go2rust_gosyntax0_41(v: Arc<Mutex<Option<Arc<dyn Fn() -> () + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
    if { let __nil_result = (*v.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some("(func())(nil)".to_string())));
    }
    Arc::new(Mutex::new(Some("(func())(0xc000010000)".to_string())))
}

pub fn __go2rust_gosyntax0_42(v: GoChannel<i64>) -> Arc<Mutex<Option<String>>> {
    if v.is_nil() {
        return Arc::new(Mutex::new(Some("(chan int)(nil)".to_string())));
    }
    Arc::new(Mutex::new(Some("(chan int)(0xc000010000)".to_string())))
}

pub fn __go2rust_gosyntax0_43(v: Arc<Mutex<Option<Point>>>) -> Arc<Mutex<Option<String>>> {
    if { let __nil_result = (*v.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some("(*main.Point)(nil)".to_string())));
    }
    return Arc::new(Mutex::new(Some(format!("{}{}", "&".to_string(), (*__go2rust_gosyntax0_2(Arc::new(Mutex::new(Some({ let __v = (*v.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v })))).lock().unwrap().as_ref().unwrap())))));
}

pub fn __go2rust_gosyntax0_44(v: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Point>>>>>>>) -> Arc<Mutex<Option<String>>> {
    if { let __nil_result = (*v.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some("[]*main.Point(nil)".to_string())));
    }
    let mut s = Arc::new(Mutex::new(Some("[]*main.Point{".to_string())));
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*v.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&", ".to_string()); };
    }
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&{ let __s = __go2rust_gosyntax0_25({ let __seq = { let __seq_holder = v.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }); let __value = (*__s.lock().unwrap().as_ref().unwrap()).clone(); __value }); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return Arc::new(Mutex::new(Some(format!("{}{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }, "}".to_string()))));
}

pub fn __go2rust_gosyntax0_45(v: Arc<Mutex<Option<Version>>>) -> Arc<Mutex<Option<String>>> {
    (*v.lock().unwrap().as_ref().unwrap()).go_string()
}

pub fn __go2rust_gosyntax0_46(v: Arc<Mutex<Option<Vec<Version>>>>) -> Arc<Mutex<Option<String>>> {
    if { let __nil_result = (*v.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some("[]main.Version(nil)".to_string())));
    }
    let mut s = Arc::new(Mutex::new(Some("[]main.Version{".to_string())));
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*v.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&", ".to_string()); };
    }
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&{ let __s = __go2rust_gosyntax0_45(Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = v.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))); let __value = (*__s.lock().unwrap().as_ref().unwrap()).clone(); __value }); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return Arc::new(Mutex::new(Some(format!("{}{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }, "}".to_string()))));
}

pub fn __go2rust_gosyntax0_47(v: Arc<Mutex<Option<Handle>>>) -> Arc<Mutex<Option<String>>> {
    { let __recv = v.clone(); let __recv_ptr: *const Handle = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Handle }; let __result = unsafe { &*__recv_ptr }.go_string(); __result }
}

pub fn __go2rust_gosyntax0_48(v: Arc<Mutex<Option<Box<dyn Shape + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
    if { let __nil_result = (*v.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some("<nil>".to_string())));
    }
    {
    let _ts_subject = v.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    std::mem::drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Shape + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<Square>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<Square>()).unwrap().__go_value_clone())));
        return __go2rust_gosyntax0_4(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<CirclePtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<CirclePtr>()).unwrap().0.clone();
        return __go2rust_gosyntax0_6(x.clone());;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<SquarePtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<SquarePtr>()).unwrap().0.clone();
        return __go2rust_gosyntax0_9(x.clone());;
    }
    }
    Arc::new(Mutex::new(Some(format!("{}", match v.lock().unwrap().as_ref() { Some(__iface) => __iface.as_ref().__go_print(false), None => "<nil>".to_string() }))))
}

pub fn __go2rust_gosyntax0_49(v: Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> Arc<Mutex<Option<String>>> {
    if { let __nil_result = (*v.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some("[]interface {}(nil)".to_string())));
    }
    let mut s = Arc::new(Mutex::new(Some("[]interface {}{".to_string())));
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*v.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&", ".to_string()); };
    }
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&{ let __s = __go2rust_gosyntax0_36(Arc::new(Mutex::new(Some({ let __idx = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_holder = v.clone(); let __seq_guard = __seq_holder.lock().unwrap(); go_any_clone(__seq_guard.as_ref().map(Vec::as_slice).unwrap_or(&[])[__idx].as_ref()) })))); let __value = (*__s.lock().unwrap().as_ref().unwrap()).clone(); __value }); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return Arc::new(Mutex::new(Some(format!("{}{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }, "}".to_string()))));
}

pub fn __go2rust_gosyntax0_50(v: Arc<Mutex<Option<BTreeMap<Point, Arc<Mutex<Option<String>>>>>>>) -> Arc<Mutex<Option<String>>> {
    if { let __nil_result = (*v.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some("map[main.Point]string(nil)".to_string())));
    }
    let mut keys: Arc<Mutex<Option<Vec<Point>>>> = Arc::new(Mutex::new(Some({ let mut v: Vec<Point> = (0..(({ let __len = (*v.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })) as usize).map(|_| Default::default()).collect(); v.truncate((0) as usize); v })));
    for (k, _) in { let __range_holder = v.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = keys.clone(); let __append_value = k.clone(); let mut __appended: Vec<Point> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.go_append_copying(|__go_slice| { __go_slice.push(__append_value); }, |__go_elem| __go_elem.__go_value_clone()); Arc::new(Mutex::new(Some(__appended))) })); *keys.lock().unwrap() = __slice_header; };
    }
    let mut i = Arc::new(Mutex::new(Some(1i64)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*keys.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x < __tmp_y } {
        let mut j = { let __owned = i.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    while {
        let __go_cond_0 = { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y };
        if __go_cond_0 {
            let __go_cond_1 = {
                let __tmp_x =
                    __go2rust_gosyntaxcmp0_51(
                        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value })))
                    );
                let __tmp_y = 0;
                __tmp_x < __tmp_y
            };
            __go_cond_1
        } else {
            false
        }
    } {
        {
            let __go_lhs_sequence_0 = keys.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = keys.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    let mut s = Arc::new(Mutex::new(Some("map[main.Point]string{".to_string())));
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*keys.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&", ".to_string()); };
    }
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&{
            let mut __s = String::new();
            __s.push_str(&format!("{}", (*__go2rust_gosyntax0_2(Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))).lock().unwrap().as_ref().unwrap())));
            __s.push_str(&format!("{}", ":".to_string()));
            __s.push_str(&format!("{}", (*__go2rust_gosyntax0_8(Arc::new(Mutex::new(Some({ let __arg_value = { let __map = { let __map_holder = v.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&{ let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()) }; __arg_value })))).lock().unwrap().as_ref().unwrap())));
            __s
        }); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return Arc::new(Mutex::new(Some(format!("{}{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }, "}".to_string()))));
}

pub fn __go2rust_gosyntaxcmp0_51(a: Arc<Mutex<Option<Point>>>, b: Arc<Mutex<Option<Point>>>) -> i64 {
    {
        let mut c = __go2rust_gosyntaxcmp0_52(Arc::new(Mutex::new(Some({ let __selector_holder = (*a.lock().unwrap().as_ref().unwrap()).x.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*b.lock().unwrap().as_ref().unwrap()).x.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }))));;
        if { let __tmp_x = c; let __tmp_y = 0; __tmp_x != __tmp_y } {
            return c;;
        }
    }
    {
        let mut c = __go2rust_gosyntaxcmp0_52(Arc::new(Mutex::new(Some({ let __selector_holder = (*a.lock().unwrap().as_ref().unwrap()).y.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*b.lock().unwrap().as_ref().unwrap()).y.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }))));;
        if { let __tmp_x = c; let __tmp_y = 0; __tmp_x != __tmp_y } {
            return c;;
        }
    }
    0
}

pub fn __go2rust_gosyntaxcmp0_52(a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>) -> i64 {
    if { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        return -(1);
    }
    if { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        return 1;
    }
    0
}

pub fn __go2rust_gosyntax0_53(v: Arc<Mutex<Option<Pair__string__int>>>) -> Arc<Mutex<Option<String>>> {
    let mut s = Arc::new(Mutex::new(Some("main.Pair[string,int]{".to_string())));
    { (*s.lock().unwrap().as_mut().unwrap()).push_str(&format!("{}{}", "Key:".to_string(), (*__go2rust_gosyntax0_8(Arc::new(Mutex::new(Some({ let __selector_holder = (*v.lock().unwrap().as_ref().unwrap()).key.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap()))); };
    { (*s.lock().unwrap().as_mut().unwrap()).push_str(&format!("{}{}", ", Val:".to_string(), (*__go2rust_gosyntax0_3(Arc::new(Mutex::new(Some({ let __selector_holder = (*v.lock().unwrap().as_ref().unwrap()).val.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap()))); };
    return Arc::new(Mutex::new(Some(format!("{}{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }, "}".to_string()))));
}

pub fn __go2rust_gosyntax0_54(v: Arc<Mutex<Option<AnonymousStruct2>>>) -> Arc<Mutex<Option<String>>> {
    let mut s = Arc::new(Mutex::new(Some("struct {}{".to_string())));
    return Arc::new(Mutex::new(Some(format!("{}{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }, "}".to_string()))));
}

pub fn __go2rust_gosyntax0_55(v: Arc<Mutex<Option<AnonymousStruct3>>>) -> Arc<Mutex<Option<String>>> {
    let mut s = Arc::new(Mutex::new(Some("struct { A int; B int }{".to_string())));
    { (*s.lock().unwrap().as_mut().unwrap()).push_str(&format!("{}{}", "A:".to_string(), (*__go2rust_gosyntax0_3(Arc::new(Mutex::new(Some({ let __selector_holder = (*v.lock().unwrap().as_ref().unwrap()).a.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap()))); };
    { (*s.lock().unwrap().as_mut().unwrap()).push_str(&format!("{}{}", ", B:".to_string(), (*__go2rust_gosyntax0_3(Arc::new(Mutex::new(Some({ let __selector_holder = (*v.lock().unwrap().as_ref().unwrap()).b.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap()))); };
    return Arc::new(Mutex::new(Some(format!("{}{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }, "}".to_string()))));
}

pub fn __go2rust_gosyntax0_56(v: Arc<Mutex<Option<[i64; 0]>>>) -> Arc<Mutex<Option<String>>> {
    let mut s = Arc::new(Mutex::new(Some("[0]int{".to_string())));
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = 0; __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&", ".to_string()); };
    }
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&{ let __s = __go2rust_gosyntax0_3(Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = v.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))); let __value = (*__s.lock().unwrap().as_ref().unwrap()).clone(); __value }); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return Arc::new(Mutex::new(Some(format!("{}{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }, "}".to_string()))));
}

pub fn __go2rust_gosyntax0_57(v: Arc<Mutex<Option<BTreeMap<i64, Arc<Mutex<Option<bool>>>>>>>) -> Arc<Mutex<Option<String>>> {
    if { let __nil_result = (*v.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some("map[int]bool(nil)".to_string())));
    }
    let mut keys = Arc::new(Mutex::new(Some({ let mut v: Vec<i64> = (0..(({ let __len = (*v.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })) as usize).map(|_| 0i64).collect(); v.truncate((0) as usize); v })));
    for (k, _) in { let __range_holder = v.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = keys.clone(); let __append_value = k; let mut __appended: Vec<i64> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *keys.lock().unwrap() = __slice_header; };
    }
    let mut i = Arc::new(Mutex::new(Some(1i64)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*keys.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x < __tmp_y } {
        let mut j = { let __owned = i.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    while {
        let __go_cond_0 = { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y };
        if __go_cond_0 {
            let __go_cond_1 = {
                let __tmp_x = { let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
                let __tmp_y = { let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
                __tmp_x < __tmp_y
            };
            __go_cond_1
        } else {
            false
        }
    } {
        {
            let __go_lhs_sequence_0 = keys.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = keys.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    let mut s = Arc::new(Mutex::new(Some("map[int]bool{".to_string())));
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*keys.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&", ".to_string()); };
    }
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&{
            let mut __s = String::new();
            __s.push_str(&format!("{}", (*__go2rust_gosyntax0_3(Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))).lock().unwrap().as_ref().unwrap())));
            __s.push_str(&format!("{}", ":".to_string()));
            __s.push_str(&format!("{}", (*__go2rust_gosyntax0_12(Arc::new(Mutex::new(Some({ let __arg_value = { let __map = { let __map_holder = v.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&{ let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| false) }; __arg_value })))).lock().unwrap().as_ref().unwrap())));
            __s
        }); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return Arc::new(Mutex::new(Some(format!("{}{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }, "}".to_string()))));
}

pub fn __go2rust_gosyntax0_58(v: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
    if { let __nil_result = (*v.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some("<nil>".to_string())));
    }
    Arc::new(Mutex::new(Some(format!("{}", { let __text = match v.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }))))
}

pub fn __go2rust_gosyntax0_59(v: Arc<Mutex<Option<BTreeMap<Square, Arc<Mutex<Option<i64>>>>>>>) -> Arc<Mutex<Option<String>>> {
    if { let __nil_result = (*v.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some("map[main.Square]int(nil)".to_string())));
    }
    let mut keys: Arc<Mutex<Option<Vec<Square>>>> = Arc::new(Mutex::new(Some({ let mut v: Vec<Square> = (0..(({ let __len = (*v.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })) as usize).map(|_| Default::default()).collect(); v.truncate((0) as usize); v })));
    for (k, _) in { let __range_holder = v.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = keys.clone(); let __append_value = k.clone(); let mut __appended: Vec<Square> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.go_append_copying(|__go_slice| { __go_slice.push(__append_value); }, |__go_elem| __go_elem.__go_value_clone()); Arc::new(Mutex::new(Some(__appended))) })); *keys.lock().unwrap() = __slice_header; };
    }
    let mut i = Arc::new(Mutex::new(Some(1i64)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*keys.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x < __tmp_y } {
        let mut j = { let __owned = i.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    while {
        let __go_cond_0 = { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y };
        if __go_cond_0 {
            let __go_cond_1 = {
                let __tmp_x =
                    __go2rust_gosyntaxcmp0_60(
                        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value })))
                    );
                let __tmp_y = 0;
                __tmp_x < __tmp_y
            };
            __go_cond_1
        } else {
            false
        }
    } {
        {
            let __go_lhs_sequence_0 = keys.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = keys.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    let mut s = Arc::new(Mutex::new(Some("map[main.Square]int{".to_string())));
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*keys.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&", ".to_string()); };
    }
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&{
            let mut __s = String::new();
            __s.push_str(&format!("{}", (*__go2rust_gosyntax0_4(Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))).lock().unwrap().as_ref().unwrap())));
            __s.push_str(&format!("{}", ":".to_string()));
            __s.push_str(&format!("{}", (*__go2rust_gosyntax0_3(Arc::new(Mutex::new(Some({ let __arg_value = { let __map = { let __map_holder = v.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&{ let __seq = { let __seq_holder = keys.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }; __arg_value })))).lock().unwrap().as_ref().unwrap())));
            __s
        }); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return Arc::new(Mutex::new(Some(format!("{}{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }, "}".to_string()))));
}

pub fn __go2rust_gosyntaxcmp0_60(a: Arc<Mutex<Option<Square>>>, b: Arc<Mutex<Option<Square>>>) -> i64 {
    {
        let mut c = __go2rust_gosyntaxcmp0_61(Arc::new(Mutex::new(Some({ let __selector_holder = (*a.lock().unwrap().as_ref().unwrap()).side.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*b.lock().unwrap().as_ref().unwrap()).side.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }))));;
        if { let __tmp_x = c; let __tmp_y = 0; __tmp_x != __tmp_y } {
            return c;;
        }
    }
    0
}

pub fn __go2rust_gosyntaxcmp0_61(a: Arc<Mutex<Option<f64>>>, b: Arc<Mutex<Option<f64>>>) -> i64 {
    if { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        return -(1);
    }
    if { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        return 1;
    }
    0
}

#[derive(Debug, Clone)]
pub struct AnonymousStruct1 {
    pub a: Arc<Mutex<Option<i64>>>,
    pub b: Arc<Mutex<Option<Vec<String>>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.a.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.b.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            a: __go_clone_0_0,
            b: __go_clone_1_0,
        }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        Self {
            a: __go_default_0_0,
            b: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.a.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice(&self.b));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone, Default)]
pub struct AnonymousStruct2 {
}
impl AnonymousStruct2 {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
    }
}


impl std::fmt::Display for AnonymousStruct2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct3 {
    pub a: Arc<Mutex<Option<i64>>>,
    pub b: Arc<Mutex<Option<i64>>>,
}
impl AnonymousStruct3 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.a.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.b.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            a: __go_clone_0_0,
            b: __go_clone_0_1,
        }
    }
}


impl Default for AnonymousStruct3 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_0_1 = Arc::new(Mutex::new(Some(0)));
        Self {
            a: __go_default_0_0,
            b: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for AnonymousStruct3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.a.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.b.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}
