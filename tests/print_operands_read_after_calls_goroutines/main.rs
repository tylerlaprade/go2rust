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
use std::thread;

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

fn format_any(value: &(dyn Any + Send + Sync)) -> String {
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
    } else {
        panic!("go2rust: cannot print this value held in an interface: its type has no formatting case yet")
    }
}

fn format_any_slice_values(slice: &Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> String {
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.iter().map(|v| format_any(v.as_ref())).collect();
        formatted.join(" ")
    } else {
        String::new()
    }
}

fn format_any_slice(slice: &Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> String {
    format!("[{}]", format_any_slice_values(slice))
}

fn format_any_variadic(slice: &Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> String {
    format_any_slice_values(slice)
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

// Two channels are equal when they are the same channel, as == compares
// them in Go, or both nil.
impl<T> PartialEq for GoChannel<T> {
    fn eq(&self, other: &Self) -> bool {
        (self.is_nil() && other.is_nil()) || std::sync::Arc::ptr_eq(&self.rx, &other.rx)
    }
}

impl<T> Eq for GoChannel<T> {}

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
    use super::{format_any, __go_type_name, go_format_is_text, go_format_named_basic_value, go_format_fields_value, go_format_elements, GoFormatNil};
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
    value.is::<GoFormatNil>()
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

#[derive(Debug, Clone)]
pub struct os_File {
    pub __go_data: std::sync::Arc<std::sync::Mutex<Vec<u8>>>,
    pub __go_closed: std::sync::Arc<std::sync::atomic::AtomicBool>,
    pub __go_wait_for_close: bool,
    /// How far Read has read __go_data; shared by every copy of the File.
    pub __go_read_pos: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    /// os.Stdin: Read reads the process's standard input.
    pub __go_stdin: bool,
    /// os.Stdout (1) or os.Stderr (2): writes go to the process's standard
    /// output or error, through the same handle as print!.
    pub __go_std_stream: u8,
    /// The file os.Create opened: reads and writes go to it, and Close
    /// closes it.
    pub __go_disk: std::sync::Arc<std::sync::Mutex<Option<std::fs::File>>>,
    /// The name the File was opened with, for its errors.
    pub __go_name: String,
    /// This File was closed: shared by its copies, but not by the other
    /// end of a pipe, which __go_closed is.
    pub __go_handle_closed: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl Default for os_File {
    fn default() -> Self {
        Self {
            __go_data: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
            __go_closed: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            __go_wait_for_close: false,
            __go_read_pos: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            __go_stdin: false,
            __go_std_stream: 0,
            __go_disk: std::sync::Arc::new(std::sync::Mutex::new(None)),
            __go_name: String::new(),
            __go_handle_closed: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }
}

/// The end of a File's input until a source-translated io registers its
/// io.EOF (__go_register_io_eof), the value callers compare with.
#[derive(Debug)]
pub struct GoStubEOF;

impl std::fmt::Display for GoStubEOF {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "EOF")
    }
}

impl std::error::Error for GoStubEOF {}

impl std::fmt::Display for os_File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<os_File>")
    }
}

impl PartialEq for os_File {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__go_data, &other.__go_data)
    }
}

impl Eq for os_File {}

impl os_File {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }

    pub fn __go_write_bytes(&self, data: &[u8]) {
        let _ = self.__go_write_bytes_result(data);
    }

    /// Go's text for an I/O error: the C library's text with a lowercase
    /// first letter, without Rust's " (os error N)".
    fn __go_io_text(err: &std::io::Error) -> String {
        let mut text = err.to_string();
        if let Some(code) = err.raw_os_error() {
            let suffix = format!(" (os error {})", code);
            if text.ends_with(&suffix) {
                text.truncate(text.len() - suffix.len());
            }
            let mut chars = text.chars();
            if let Some(first) = chars.next() {
                text = first.to_lowercase().collect::<String>() + chars.as_str();
            }
        }
        text
    }

    fn __go_op_error(&self, op: &str, err: std::io::Error) -> std::io::Error {
        std::io::Error::new(err.kind(), format!("{} {}: {}", op, self.__go_name, Self::__go_io_text(&err)))
    }

    fn __go_closed_error(&self, op: &str) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::Other, format!("{} {}: file already closed", op, self.__go_name))
    }

    /// Writes data to the disk file os.Create or os.OpenFile opened, or
    /// else to the File's bytes, and reports how many bytes it wrote. A
    /// closed File takes no writes, as in Go. The disk lock is held from
    /// the closed check to the write, so Close cannot come in between.
    pub fn __go_write_counted(&self, data: &[u8]) -> (usize, Option<std::io::Error>) {
        let mut __disk = self.__go_disk.lock().unwrap();
        if self.__go_handle_closed.load(std::sync::atomic::Ordering::SeqCst) {
            return (0, Some(self.__go_closed_error("write")));
        }
        if self.__go_std_stream != 0 {
            drop(__disk);
            // os.Stdout is not buffered in Go: each write reaches it.
            let __result = if self.__go_std_stream == 1 {
                let mut __out = std::io::stdout().lock();
                std::io::Write::write_all(&mut __out, data).and_then(|_| std::io::Write::flush(&mut __out))
            } else {
                let mut __out = std::io::stderr().lock();
                std::io::Write::write_all(&mut __out, data)
            };
            return match __result {
                Ok(()) => (data.len(), None),
                Err(__err) => (0, Some(self.__go_op_error("write", __err))),
            };
        }
        if let Some(__file) = __disk.as_mut() {
            let mut __done = 0usize;
            while __done < data.len() {
                match std::io::Write::write(__file, &data[__done..]) {
                    Ok(0) => return (__done, Some(std::io::Error::new(std::io::ErrorKind::WriteZero, format!("write {}: short write", self.__go_name)))),
                    Ok(__count) => __done += __count,
                    Err(__err) if __err.kind() == std::io::ErrorKind::Interrupted => {}
                    Err(__err) => return (__done, Some(self.__go_op_error("write", __err))),
                }
            }
            return (__done, None);
        }
        drop(__disk);
        self.__go_data.lock().unwrap().extend_from_slice(data);
        (data.len(), None)
    }

    pub fn __go_write_bytes_result(&self, data: &[u8]) -> std::io::Result<()> {
        match self.__go_write_counted(data) {
            (_, None) => Ok(()),
            (_, Some(__err)) => Err(__err),
        }
    }

    pub fn __go_read_all(&self) -> Vec<u8> {
        self.__go_data.lock().unwrap().clone()
    }

    /// File.Name: the name the File was opened with (part of the
    /// os-file-create shim).
    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some::<String>(self.__go_name.clone())))
    }

    pub fn close(&self) -> Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> {
        let mut __disk = self.__go_disk.lock().unwrap();
        if self.__go_handle_closed.swap(true, std::sync::atomic::Ordering::SeqCst) {
            drop(__disk);
            let __err = self.__go_closed_error("close");
            return Arc::new(Mutex::new(Some::<Arc<dyn StdError + Send + Sync>>(std::sync::Arc::new(__err) as std::sync::Arc<dyn std::error::Error + Send + Sync>)));
        }
        self.__go_closed.store(true, std::sync::atomic::Ordering::SeqCst);
        // Dropping a File closes it and discards the result. Go's Close
        // reports a failed close(2), so on Unix close the descriptor
        // directly. The handle stays closed either way.
        if let Some(__file) = __disk.take() {
            drop(__disk);
            #[cfg(unix)]
            {
                let __fd = std::os::fd::IntoRawFd::into_raw_fd(__file);
                unsafe extern "C" {
                    fn close(fd: i32) -> i32;
                }
                if unsafe { close(__fd) } != 0 {
                    let __err = self.__go_op_error("close", std::io::Error::last_os_error());
                    return Arc::new(Mutex::new(Some::<Arc<dyn StdError + Send + Sync>>(std::sync::Arc::new(__err) as std::sync::Arc<dyn std::error::Error + Send + Sync>)));
                }
            }
            #[cfg(not(unix))]
            drop(__file);
        }
        Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>))
    }

    pub fn write<T0: 'static>(&self, arg0: T0) -> (i64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        let bytes = if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Vec<u8>>() {
            v.clone()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Arc<Mutex<Option<Vec<u8>>>>>() {
            v.lock().unwrap().as_ref().cloned().unwrap_or_default()
        } else {
            Vec::new()
        };
        let (__count, __failure) = self.__go_write_counted(&bytes);
        let n = __count as i64;
        match __failure {
            Some(__err) => (n as i64, Arc::new(Mutex::new(Some::<Arc<dyn StdError + Send + Sync>>(std::sync::Arc::new(__err) as std::sync::Arc<dyn std::error::Error + Send + Sync>)))),
            None => (n as i64, Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>))),
        }
    }

    pub fn write_string<T0: 'static>(&self, arg0: T0) -> (i64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        let value = if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<String>() {
            v.clone()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<&str>() {
            (*v).to_string()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Arc<Mutex<Option<String>>>>() {
            v.lock().unwrap().as_ref().cloned().unwrap_or_default()
        } else {
            String::new()
        };
        let (__count, __failure) = self.__go_write_counted(value.as_bytes());
        let n = __count as i64;
        match __failure {
            Some(__err) => (n as i64, Arc::new(Mutex::new(Some::<Arc<dyn StdError + Send + Sync>>(std::sync::Arc::new(__err) as std::sync::Arc<dyn std::error::Error + Send + Sync>)))),
            None => (n as i64, Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>))),
        }
    }

    pub fn read<T0: 'static>(&self, arg0: T0) -> (i64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        // Go's Read: fill as much of p as the input has, or report the end
        // of the input with no bytes read. The destination is locked before
        // the disk file, as in ReadAt.
        enum __ReadOutcome { Bytes(usize), End, Failed(std::io::Error) }
        // Close sets the closed flag while it holds the disk lock, so the
        // flag is read under that lock: a Close that lands between the check
        // and the read cannot turn a closed-file error into EOF. Go checks
        // for a closed file before the buffer length, nil buffers included.
        let __outcome = if let Some(__buf) = (&arg0 as &dyn std::any::Any).downcast_ref::<Arc<Mutex<Option<Vec<u8>>>>>() {
            let mut __guard = __buf .lock().unwrap();
            let mut __disk = self.__go_disk.lock().unwrap();
            if self.__go_handle_closed.load(std::sync::atomic::Ordering::SeqCst) {
                __ReadOutcome::Failed(self.__go_closed_error("read"))
            } else {
                match __guard.as_mut() {
                    Some(__dst) => {
                        let __dst = __dst.as_mut_slice();
                        if __dst.is_empty() {
                            __ReadOutcome::Bytes(0)
                        } else if self.__go_stdin {
                            match std::io::Read::read(&mut std::io::stdin(), __dst) {
                                Ok(0) | Err(_) => __ReadOutcome::End,
                                Ok(__count) => __ReadOutcome::Bytes(__count),
                            }
                        } else if let Some(__disk) = __disk.as_mut() {
                            match std::io::Read::read(__disk, __dst) {
                                Ok(0) => __ReadOutcome::End,
                                Ok(__count) => __ReadOutcome::Bytes(__count),
                                Err(__err) => __ReadOutcome::Failed(self.__go_op_error("read", __err)),
                            }
                        } else {
                            let __data = self.__go_data.lock().unwrap();
                            let __pos = self.__go_read_pos.load(std::sync::atomic::Ordering::SeqCst).min(__data.len());
                            let __count = (__data.len() - __pos).min(__dst.len());
                            __dst[..__count].copy_from_slice(&__data[__pos..__pos + __count]);
                            self.__go_read_pos.store(__pos + __count, std::sync::atomic::Ordering::SeqCst);
                            if __count == 0 { __ReadOutcome::End } else { __ReadOutcome::Bytes(__count) }
                        }
                    }
                    None => __ReadOutcome::Bytes(0),
                }
            }
        } else {
            __ReadOutcome::Bytes(0)
        };
        match __outcome {
            __ReadOutcome::Bytes(__n) => {
                let n = __n;
                (n as i64, Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>)))
            }
            __ReadOutcome::End => (0, __go_stub_eof()),
            __ReadOutcome::Failed(__err) => (0, Arc::new(Mutex::new(Some::<Arc<dyn StdError + Send + Sync>>(std::sync::Arc::new(__err) as std::sync::Arc<dyn std::error::Error + Send + Sync>)))),
        }
    }

    pub fn read_at<T0: 'static, T1: 'static>(&self, arg0: T0, arg1: T1) -> (i64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        let offset = if let Some(v) = (&arg1 as &dyn std::any::Any).downcast_ref::<i64>() {
            *v
        } else if let Some(v) = (&arg1 as &dyn std::any::Any).downcast_ref::<Arc<Mutex<Option<i64>>>>() {
            v .lock().unwrap().as_ref().copied().unwrap_or_default()
        } else {
            0
        };
        if offset < 0 {
            let __err = std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("readat {}: negative offset", self.__go_name));
            return (0, Arc::new(Mutex::new(Some::<Arc<dyn StdError + Send + Sync>>(std::sync::Arc::new(__err) as std::sync::Arc<dyn std::error::Error + Send + Sync>))));
        }
        let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Arc<Mutex<Option<Vec<u8>>>>>() else {
            return (0, Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>)));
        };
        // The destination first, then the disk file, as in Read.
        let mut guard = v .lock().unwrap();
        let Some(target) = guard.as_mut() else {
            return (0, Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>)));
        };
        // Go's ReadAt reads nothing into an empty p, so it succeeds even on
        // a closed file.
        if target.is_empty() {
            return (0, Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>)));
        }
        let mut __done = 0usize;
        let mut __failure: Option<std::io::Error> = None;
        // The closed flag is read under the disk lock, as in Read.
        let mut __disk = self.__go_disk.lock().unwrap();
        if self.__go_handle_closed.load(std::sync::atomic::Ordering::SeqCst) {
            drop(__disk);
            let __err = self.__go_closed_error("read");
            return (0, Arc::new(Mutex::new(Some::<Arc<dyn StdError + Send + Sync>>(std::sync::Arc::new(__err) as std::sync::Arc<dyn std::error::Error + Send + Sync>))));
        }
        if let Some(__file) = __disk.as_mut() {
            // ReadAt reads at offset without moving the File's own position.
            #[cfg(unix)]
            while __done < target.len() {
                match std::os::unix::fs::FileExt::read_at(&*__file, &mut target[__done..], offset as u64 + __done as u64) {
                    Ok(0) => break,
                    Ok(__count) => __done += __count,
                    Err(__err) if __err.kind() == std::io::ErrorKind::Interrupted => {}
                    Err(__err) => {
                        __failure = Some(self.__go_op_error("read", __err));
                        break;
                    }
                }
            }
            #[cfg(not(unix))]
            {
                // ReadAt must leave the File's position where it was: without
                // the saved position, do not seek; a failed restore is the
                // call's error.
                match std::io::Seek::stream_position(__file) {
                    Err(__err) => __failure = Some(self.__go_op_error("read", __err)),
                    Ok(__saved) => {
                        if let Err(__err) = std::io::Seek::seek(__file, std::io::SeekFrom::Start(offset as u64)) {
                            __failure = Some(self.__go_op_error("read", __err));
                        } else {
                            while __done < target.len() {
                                match std::io::Read::read(__file, &mut target[__done..]) {
                                    Ok(0) => break,
                                    Ok(__count) => __done += __count,
                                    Err(__err) if __err.kind() == std::io::ErrorKind::Interrupted => {}
                                    Err(__err) => {
                                        __failure = Some(self.__go_op_error("read", __err));
                                        break;
                                    }
                                }
                            }
                        }
                        if let Err(__err) = std::io::Seek::seek(__file, std::io::SeekFrom::Start(__saved)) {
                            __failure.get_or_insert(self.__go_op_error("read", __err));
                        }
                    }
                }
            }
        } else {
            drop(__disk);
            let data = self.__go_read_all();
            let start = offset as usize;
            if start < data.len() {
                __done = std::cmp::min(target.len(), data.len() - start);
                target[..__done].copy_from_slice(&data[start..start + __done]);
            }
        }
        let n = __done as i64;
        match __failure {
            Some(__err) => (n as i64, Arc::new(Mutex::new(Some::<Arc<dyn StdError + Send + Sync>>(std::sync::Arc::new(__err) as std::sync::Arc<dyn std::error::Error + Send + Sync>)))),
            // Go's ReadAt reports the end of the file when it cannot fill p.
            None if __done < target.len() => (n as i64, __go_stub_eof()),
            None => (n as i64, Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>))),
        }
    }
}

static __GO_STUB_IO_EOF: std::sync::OnceLock<Arc<dyn StdError + Send + Sync>> = std::sync::OnceLock::new();

/// io's init hands over io.EOF's value.
pub fn __go_register_io_eof(eof: Option<Arc<dyn StdError + Send + Sync>>) {
    if let Some(eof) = eof {
        let _ = __GO_STUB_IO_EOF.set(eof);
    }
}

fn __go_stub_eof() -> Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> {
    match __GO_STUB_IO_EOF.get() {
        Some(__eof) => Arc::new(Mutex::new(Some::<Arc<dyn StdError + Send + Sync>>(__eof.clone()))),
        None => Arc::new(Mutex::new(Some::<Arc<dyn StdError + Send + Sync>>(std::sync::Arc::new(GoStubEOF) as Arc<dyn StdError + Send + Sync>))),
    }
}


pub mod os {
    use super::*;
    pub fn Stdout() -> Arc<Mutex<Option<os_File>>> {
        static __GO_FILE: std::sync::OnceLock<Arc<Mutex<Option<os_File>>>> = std::sync::OnceLock::new();
        __GO_FILE.get_or_init(|| Arc::new(Mutex::new(Some::<os_File>(os_File { __go_std_stream: 1, __go_name: "/dev/stdout".to_string(), ..Default::default() })))).clone()
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


#[derive(Debug, Clone)]
pub struct pt {
    pub x: Arc<Mutex<Option<i64>>>,
}

impl pt {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
        }
    }
}


impl Default for pt {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            x: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for pt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


pub(crate) static level: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static order: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<String>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *level.lock().unwrap() = Some(0);
    *order.lock().unwrap() = Some(Vec::nil());
    *level.lock().unwrap() = Some(1);
}


impl r#box {
    pub fn inc(&mut self) -> i64 {
        { let __target = self.n.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return (*self.n.lock().unwrap().as_ref().unwrap());
    }
}

/// del removes s[i:j] the way slices.Delete does: it moves the tail down and
/// clears what is left at the end of the backing array.
pub fn del(s: Arc<Mutex<Option<Vec<i64>>>>, i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<Vec<i64>>>> {
    let mut n = {
        let _dst_start = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let _dst_len = (*s.lock().unwrap().as_ref().unwrap()).len() - _dst_start;
        let _src = (*Arc::new(Mutex::new(Some({
            let __seq_holder = s.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
            let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
            std::mem::drop(__seq_guard);
            let __low = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __high = __seq.len();
            let __max = __source_cap;
            __seq.reslice3_zeroed(__low, __high, __max)
        }))).lock().unwrap().as_ref().unwrap()).to_vec();
        let _n = std::cmp::min(_dst_len, _src.len());
        for _i in 0.._n {
            (*s.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
        }
        Arc::new(Mutex::new(Some(_n as i64)))
    };
    let mut k = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })));
    while { let __tmp_x = ({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x < __tmp_y } {
        { let __idx = ({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __rhs = 0; (*s.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
        { let mut guard = k.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return Arc::new(Mutex::new(Some({
        let __seq_holder = s.clone();
        let __seq_guard = __seq_holder.lock().unwrap();
        let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
        let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
        std::mem::drop(__seq_guard);
        let __low = 0;
        let __high = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
        let __max = __source_cap;
        __seq.reslice3_zeroed(__low, __high, __max)
    })));
}

fn main() {
    go_install_panic_exit();
    __go_init_all();
        // A goroutine makes every variable a locked one.
    let mut started = GoChannel::<String>::new_buffered(1 as usize);
    let started_thread = started.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        started_thread.send("started".to_string());;;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    println!("{}", format!("{}", started.recv().unwrap_or_default()));
    shapes();
    in_order();
    let mut ins = Arc::new(Mutex::new(Some(vec![1i64, 2i64, 3i64, 4i64])));
    { let __go_print_arg_1 = del(ins.clone(), Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(2))));
let mut __go_print_arg_1 = { let __slice_header = GoSliceHeader::go_slice_header(&__go_print_arg_1); Arc::new(Mutex::new(__slice_header)) }; let __go_print_value = println!("{} {}", format!("{}", format_slice(&ins)), format!("{}", format_slice(&__go_print_arg_1))); __go_print_value };
    let mut b = Arc::new(Mutex::new(Some(r#box { n: Arc::new(Mutex::new(Some(0))) })));
    { let __go_print_arg_2 = { let __recv = b.clone(); let __recv_ptr: *mut r#box = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut r#box }; let __result = unsafe { &mut *__recv_ptr }.inc(); __result }; let __go_print_arg_3 = { let __recv = b.clone(); let __recv_ptr: *mut r#box = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut r#box }; let __result = unsafe { &mut *__recv_ptr }.inc(); __result }; let __go_print_value = println!("{} {} {} {}", format!("{}", (*{ let __field = (*b.lock().unwrap().as_ref().unwrap()).n.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{}", __go_print_arg_2), format!("{}", (*{ let __field = (*b.lock().unwrap().as_ref().unwrap()).n.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{}", __go_print_arg_3)); __go_print_value };
    { let __go_print_arg_4 = { let __recv = b.clone(); let __recv_ptr: *mut r#box = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut r#box }; let __result = unsafe { &mut *__recv_ptr }.inc(); __result }; let __go_print_value = print!("{} {} {}\n", format!("{}", (*{ let __field = (*b.lock().unwrap().as_ref().unwrap()).n.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{}", __go_print_arg_4), format!("{}", { let __v = (*b.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v })); __go_print_value };
    let mut s = { let __go_print_arg_5 = { let __recv = b.clone(); let __recv_ptr: *mut r#box = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut r#box }; let __result = unsafe { &mut *__recv_ptr }.inc(); __result }; let __go_print_value = Arc::new(Mutex::new(Some(format!("{}-{}", format!("{}", (*{ let __field = (*b.lock().unwrap().as_ref().unwrap()).n.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{}", __go_print_arg_5))))); __go_print_value };
    println!("{}", format!("{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    let mut err = { let __go_print_arg_6 = { let __recv = b.clone(); let __recv_ptr: *mut r#box = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut r#box }; let __result = unsafe { &mut *__recv_ptr }.inc(); __result }; let __go_print_value = Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(format!("at {}: {}", (*{ let __field = (*b.lock().unwrap().as_ref().unwrap()).n.clone(); __field }.lock().unwrap().as_ref().unwrap()), __go_print_arg_6)))))); __go_print_value };
    println!("{}", format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    { let __go_print_arg_7 = { let __recv = b.clone(); let __recv_ptr: *mut r#box = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut r#box }; let __result = unsafe { &mut *__recv_ptr }.inc(); __result }; let __go_print_value = println!("{} {}", format!("{}", (*{ let __field = (*b.lock().unwrap().as_ref().unwrap()).n.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{}", __go_print_arg_7)); __go_print_value };
    let mut t = { let __go_print_arg_8 = { let __recv = b.clone(); let __recv_ptr: *mut r#box = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut r#box }; let __result = unsafe { &mut *__recv_ptr }.inc(); __result }; let __go_print_value = Arc::new(Mutex::new(Some(format!("{}{}{}", format!("{}", (*{ let __field = (*b.lock().unwrap().as_ref().unwrap()).n.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{}", " ".to_string()), format!("{}", __go_print_arg_8))))); __go_print_value };
    println!("{} {}", format!("{}", { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", {
        let __tmp_x = (*{ let __go_print_arg_9 = { let __recv = b.clone(); let __recv_ptr: *mut r#box = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut r#box }; let __result = unsafe { &mut *__recv_ptr }.inc(); __result }; let __go_print_value = Arc::new(Mutex::new(Some(format!("{} {}\n", format!("{}", (*{ let __field = (*b.lock().unwrap().as_ref().unwrap()).n.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{}", __go_print_arg_9))))); __go_print_value }.lock().unwrap().as_ref().unwrap()).clone();
        let __tmp_y = "9 9\n".to_string();
        __tmp_x == __tmp_y
    }));
    let mut m = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some(1i64))))]))));
    let m_closure_clone = m.clone(); let mut grow = Arc::new(Mutex::new(Some(Arc::new(move || -> i64 {
        { let __map_key = "b".to_string(); let __map_value = Arc::new(Mutex::new(Some(2))); (*m_closure_clone.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        return ({ let __len = (*m_closure_clone.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64;
    }) as Arc<dyn Fn() -> i64 + Send + Sync>)));
    { let __go_print_arg_10 = {
    let __f: Arc<dyn Fn() -> i64 + Send + Sync> = grow.lock().unwrap().as_ref().unwrap().clone();
    (__f)(

    )
}; let __go_print_value = println!("{} {}", format!("{}", format_map(&m)), format!("{}", __go_print_arg_10)); __go_print_value };
    let mut ch = GoChannel::<i64>::new_buffered(1 as usize);
    ch.send(5);
    let mut n = Arc::new(Mutex::new(Some(1i64)));
    let mut pn = n.clone();
    let pn_closure_clone = pn.clone(); { let __go_print_arg_11 = Arc::new(Mutex::new(Some(ch.recv().unwrap_or_default()))); let pn_closure_clone_closure_clone = pn_closure_clone.clone(); let __go_print_arg_12 = { let __f_holder = Arc::new(Mutex::new(Some(Arc::new(move || -> i64 {
        { let new_val = 2; *pn_closure_clone_closure_clone.lock().unwrap() = Some(new_val); };
        0
    }) as Arc<dyn Fn() -> i64 + Send + Sync>))); let __f: Arc<dyn Fn() -> i64 + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }; let __go_print_value = println!("{} {} {}", format!("{}", { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*__go_print_arg_11.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", __go_print_arg_12)); __go_print_value };
}

pub fn shapes() {
    let mut x = Arc::new(Mutex::new(Some(1i64)));
    let mut px = x.clone();
    let px_closure_clone = px.clone(); let mut f = Arc::new(Mutex::new(Some(Arc::new(move || -> i64 {
        { let new_val = 9; *px_closure_clone.lock().unwrap() = Some(new_val); };
        0
    }) as Arc<dyn Fn() -> i64 + Send + Sync>)));
    { let __go_print_arg_13 = {
    let __f: Arc<dyn Fn() -> i64 + Send + Sync> = f.lock().unwrap().as_ref().unwrap().clone();
    (__f)(

    )
}; let __go_print_value = println!("{} {}", format!("{}", { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", __go_print_arg_13)); __go_print_value };
    let mut s = Arc::new(Mutex::new(Some(vec![1i64, 2i64])));
    let s_closure_clone = s.clone(); let mut g = Arc::new(Mutex::new(Some(Arc::new(move || -> i64 {
        { let __idx = (0) as usize; let __rhs = 7; (*s_closure_clone.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
        0
    }) as Arc<dyn Fn() -> i64 + Send + Sync>)));
    { let __go_print_arg_14 = {
    let __f: Arc<dyn Fn() -> i64 + Send + Sync> = g.lock().unwrap().as_ref().unwrap().clone();
    (__f)(

    )
}; let __go_print_value = println!("{} {} {}", format!("{}", format_slice(&s)), format!("{}", { let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }), format!("{}", __go_print_arg_14)); __go_print_value };
    let mut p = Arc::new(Mutex::new(Some(pt { x: Arc::new(Mutex::new(Some(1))), ..Default::default() })));
    let mut pp = p.clone();
    let pp_closure_clone = pp.clone(); let mut h = Arc::new(Mutex::new(Some(Arc::new(move || -> i64 {
        { let new_val = 5; *(*pp_closure_clone.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
        0
    }) as Arc<dyn Fn() -> i64 + Send + Sync>)));
    { let __go_print_arg_15 = {
    let __f: Arc<dyn Fn() -> i64 + Send + Sync> = h.lock().unwrap().as_ref().unwrap().clone();
    (__f)(

    )
}; let __go_print_value = println!("{} {} {}", format!("{}", { let __v = (*p.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }), format!("{}", (*{ let __field = (*p.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{}", __go_print_arg_15)); __go_print_value };
    let mut m = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some(1i64))))]))));
    let m_closure_clone = m.clone(); let mut k = Arc::new(Mutex::new(Some(Arc::new(move || -> i64 {
        { let __map_key = "a".to_string(); let __map_value = Arc::new(Mutex::new(Some(3))); (*m_closure_clone.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        0
    }) as Arc<dyn Fn() -> i64 + Send + Sync>)));
    { let __go_print_arg_16 = {
    let __f: Arc<dyn Fn() -> i64 + Send + Sync> = k.lock().unwrap().as_ref().unwrap().clone();
    (__f)(

    )
}; let __go_print_value = println!("{} {} {}", format!("{}", format_map(&m)), format!("{}", { let __map = { let __map_holder = m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", __go_print_arg_16)); __go_print_value };
    let mut str = Arc::new(Mutex::new(Some("a".to_string())));
    let mut ps = str.clone();
    let ps_closure_clone = ps.clone(); let mut j = Arc::new(Mutex::new(Some(Arc::new(move || -> Arc<Mutex<Option<String>>> {
        { let new_val = "b".to_string(); *ps_closure_clone.lock().unwrap() = Some(new_val); };
        Arc::new(Mutex::new(Some("c".to_string())))
    }) as Arc<dyn Fn() -> Arc<Mutex<Option<String>>> + Send + Sync>)));
    { let __go_print_arg_17 = {
    let __f: Arc<dyn Fn() -> Arc<Mutex<Option<String>>> + Send + Sync> = j.lock().unwrap().as_ref().unwrap().clone();
    (__f)(

    )
}; let __go_print_value = println!("{} {} {}", format!("{}", { let __v = (*str.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*__go_print_arg_17.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", format!("{}{}", { let __v = (*str.lock().unwrap().as_ref().unwrap()).clone(); __v }, "!".to_string()))); __go_print_value };
    { let __go_print_arg_18 = {
    let __f: Arc<dyn Fn() -> i64 + Send + Sync> = f.lock().unwrap().as_ref().unwrap().clone();
    (__f)(

    )
}; let __go_print_value = print!("{} {}\n", format!("{}", { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", __go_print_arg_18)); __go_print_value };
    let mut y = { let __go_print_arg_19 = {
    let __f: Arc<dyn Fn() -> i64 + Send + Sync> = f.lock().unwrap().as_ref().unwrap().clone();
    (__f)(

    )
}; let __go_print_value = Arc::new(Mutex::new(Some(format!("{} {}", format!("{}", { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", __go_print_arg_19))))); __go_print_value };
    println!("{}", format!("{}", { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }));
}

pub fn lower() -> i64 {
    { let new_val = 0; *level.lock().unwrap() = Some(new_val); };
    7
}

pub fn change(p: Arc<Mutex<Option<r#box>>>) -> i64 {
    { let __recv = p.clone(); let __recv_ptr: *mut r#box = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut r#box }; let __result = unsafe { &mut *__recv_ptr }.inc(); __result }
}

pub fn set_first(s: Arc<Mutex<Option<Vec<i64>>>>) -> i64 {
    { let __idx = (0) as usize; let __rhs = 9; (*s.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    0
}

/// sum only reads the elements of s, so it cannot change them.
/// rangeSet writes s[0] through a range assignment.
pub fn range_set(s: Arc<Mutex<Option<Vec<i64>>>>) -> i64 {
    for __range_value in vec![9i64].iter().copied() {
        { let __idx = (0) as usize; let __rhs = __range_value; (*s.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    }
    0
}

pub fn sum(s: Arc<Mutex<Option<Vec<i64>>>>) -> i64 {
    let mut t = Arc::new(Mutex::new(Some(0i64)));
    { let __range_holder = s.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for v in __range_values.iter().copied() {
        { let __rhs = v; let mut guard = t.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }
    return { let __tmp_x = ({ let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x + __tmp_y };
}

pub fn writer() -> Arc<Mutex<Option<os_File>>> {
    { let new_val = { let __collection_holder = { let __append_target = order.clone(); let __append_value = "writer".to_string(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) }.clone(); let __collection_guard = __collection_holder.lock().unwrap(); (*__collection_guard).clone() }; *order.lock().unwrap() = new_val; };
    os::Stdout().clone()
}

pub fn format() -> Arc<Mutex<Option<String>>> {
    { let new_val = { let __collection_holder = { let __append_target = order.clone(); let __append_value = "format".to_string(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) }.clone(); let __collection_guard = __collection_holder.lock().unwrap(); (*__collection_guard).clone() }; *order.lock().unwrap() = new_val; };
    Arc::new(Mutex::new(Some("%d %d\n".to_string())))
}

pub fn step() -> i64 {
    { let new_val = { let __collection_holder = { let __append_target = order.clone(); let __append_value = "step".to_string(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) }.clone(); let __collection_guard = __collection_holder.lock().unwrap(); (*__collection_guard).clone() }; *order.lock().unwrap() = new_val; };
    ({ let __len = (*order.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64
}

/// gc evaluates a builtin call and x && y in order with the calls, and the
/// writer and the format before the operands.
pub fn in_order() {
    let mut s = Arc::new(Mutex::new(Some("a".to_string())));
    let mut s_closure_clone = s.clone(); let mut mutate = Arc::new(Mutex::new(Some(Arc::new(move || -> i64 {
        let mut s_closure_clone = s_closure_clone.clone();
        { let new_val = "long".to_string(); *s_closure_clone.lock().unwrap() = Some(new_val); };
        0
    }) as Arc<dyn Fn() -> i64 + Send + Sync>)));
    println!("{} {} {} {} {}", format!("{}", ({ let __len = (*s.lock().unwrap().as_ref().unwrap()).len(); __len })), format!("{}", {
        let __f: Arc<dyn Fn() -> i64 + Send + Sync> = mutate.lock().unwrap().as_ref().unwrap().clone();
        (__f)(

        )
    }), format!("{}", { let __tmp_x = (*level.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x > __tmp_y } && true), format!("{}", lower()), format!("{}", { let __v = (*level.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    let mut n = Arc::new(Mutex::new(Some(5i64)));
    let mut pn = n.clone();
    let pn_closure_clone = pn.clone(); { let __go_print_arg_20 = Arc::new(Mutex::new(Some(std::cmp::min(({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64), (9 as i64))))); let pn_closure_clone_closure_clone = pn_closure_clone.clone(); let __go_print_arg_21 = { let __f_holder = Arc::new(Mutex::new(Some(Arc::new(move || -> i64 {
        { let new_val = 1; *pn_closure_clone_closure_clone.lock().unwrap() = Some(new_val); };
        0
    }) as Arc<dyn Fn() -> i64 + Send + Sync>))); let __f: Arc<dyn Fn() -> i64 + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }; let __go_print_value = println!("{} {} {}", format!("{}", { let __v = (*__go_print_arg_20.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", __go_print_arg_21)); __go_print_value };
    let mut b = Arc::new(Mutex::new(Some(r#box { n: Arc::new(Mutex::new(Some(1))), ..Default::default() })));
    { let __go_print_arg_22 = change(b.clone()); let __go_print_value = println!("{} {}", format!("{}", (*{ let __field = (*b.lock().unwrap().as_ref().unwrap()).n.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{}", __go_print_arg_22)); __go_print_value };
    let mut first = Arc::new(Mutex::new(Some(vec![1i64])));
    let mut alias = Arc::new(Mutex::new(first.lock().unwrap().clone()));
    { let __go_print_arg_23 = set_first(alias.clone()); let __go_print_value = println!("{} {}", format!("{}", { let __seq = { let __seq_holder = first.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }), format!("{}", __go_print_arg_23)); __go_print_value };
    let mut t = Arc::new(Mutex::new(Some(vec![1i64, 2i64])));
    { let __go_print_arg_24 = sum(t.clone()); let __go_print_value = println!("{} {}", format!("{}", { let __seq = { let __seq_holder = t.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }), format!("{}", __go_print_arg_24)); __go_print_value };
    let mut u = Arc::new(Mutex::new(Some(vec![1i64])));
    { let __go_print_arg_25 = range_set(u.clone()); let __go_print_value = println!("{} {}", format!("{}", format_slice(&u)), format!("{}", __go_print_arg_25)); __go_print_value };
    { let __go_print_arg_26 = writer(); let __go_print_arg_27 = Arc::new(Mutex::new(Some(({ let __len = (*order.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64))); let __go_print_value = { let __s = format!("{}\n", format!("{}", { let __v = (*__go_print_arg_27.lock().unwrap().as_ref().unwrap()).clone(); __v })); (*__go_print_arg_26.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) }; __go_print_value };
    { let __go_print_arg_28 = writer(); let __go_print_arg_29 = Arc::new(Mutex::new(Some(({ let __len = (*order.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64))); let __go_print_arg_30 = step(); let __go_print_value = { let __s = format!("{} {}\n", format!("{}", { let __v = (*__go_print_arg_29.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", __go_print_arg_30)); (*__go_print_arg_28.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) }; __go_print_value };
    { let __go_print_arg_31 = format(); let __go_print_arg_32 = Arc::new(Mutex::new(Some(({ let __len = (*order.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64))); let __go_print_arg_33 = step(); let __go_print_value = print!("{}", go_sprintf(&(*__go_print_arg_31.lock().unwrap().as_ref().unwrap()).clone(), &[{ let __go_format_arg = Box::new({ let __arg_holder = __go_print_arg_32.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>; __go_format_arg }, { let __go_format_arg = Box::new(__go_print_arg_33) as Box<dyn Any + Send + Sync>; __go_format_arg }], false)); __go_print_value };
    println!("{}", format!("{}", format_slice(&order.clone())));

        // A pointer operand is staged as the handle it is.
    println!("{} {}", format!("{}", ({ let __len = (*order.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", { let __text = match Arc::new(Mutex::new(Some(r#box { n: Arc::new(Mutex::new(Some(step()))), ..Default::default() }))).lock().unwrap().as_ref() { Some(__value) => format!("&{}", __value), None => "<nil>".to_string() }; __text }));
    println!("{} {}", format!("{}", ({ let __len = (*order.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", { let __text = match new_box().lock().unwrap().as_ref() { Some(__value) => format!("&{}", __value), None => "<nil>".to_string() }; __text }));

        // An element of a [][]int is staged as the slice it is.
    let mut rows = Arc::new(Mutex::new(Some(vec![vec![1i64], vec![2i64, 3i64]])));
    println!("{} {}", format!("{}", ({ let __len = (*order.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", format_slice_values(&{ let __seq = { let __seq_holder = rows.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = step(); let __tmp_y = 2; __tmp_x % __tmp_y }) as usize].clone() })));

        // The writer runs before a spread operand's call.
    { let __go_print_arg_34 = writer(); let __go_print_arg_35 = spread();
let mut __go_print_arg_35 = { let __slice_header = GoSliceHeader::go_slice_header(&__go_print_arg_35); Arc::new(Mutex::new(__slice_header)) }; let __go_print_value = { let __s = format!("{}\n", format_any_variadic(&__go_print_arg_35)); (*__go_print_arg_34.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) }; __go_print_value };
    println!("{}", format!("{}", format_slice(&Arc::new(Mutex::new(Some({
        let __seq_holder = order.clone();
        let __seq_guard = __seq_holder.lock().unwrap();
        let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
        let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
        std::mem::drop(__seq_guard);
        let __low = ({ let __tmp_x = (({ let __len = (*order.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); let __tmp_y = 2; __tmp_x - __tmp_y }) as usize;
        let __high = __seq.len();
        let __max = __source_cap;
        __seq.reslice3_zeroed(__low, __high, __max)
    }))))));
}

pub fn spread() -> Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>> {
    { let new_val = { let __collection_holder = { let __append_target = order.clone(); let __append_value = "spread".to_string(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) }.clone(); let __collection_guard = __collection_holder.lock().unwrap(); (*__collection_guard).clone() }; *order.lock().unwrap() = new_val; };
    Arc::new(Mutex::new(Some(vec![Box::new(({ let __len = (*order.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })) as Box<dyn Any + Send + Sync>, Box::new("x".to_string()) as Box<dyn Any + Send + Sync>])))
}

pub fn new_box() -> Arc<Mutex<Option<r#box>>> {
    Arc::new(Mutex::new(Some(r#box { n: Arc::new(Mutex::new(Some(step()))), ..Default::default() })))
}

pub(crate) fn __go_init_all() {
    static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();
    __GO_INIT_ONCE.call_once(|| {
        self::__go_init_globals();
    });
}
