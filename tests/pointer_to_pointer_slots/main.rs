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

fn format_map<K: Display + Ord + Clone, V>(map: &Rc<RefCell<Option<BTreeMap<K, Rc<RefCell<Option<V>>>>>>>) -> String
where
    V: Display,
{
    format_map_nil(map, "<nil>")
}

// format_map_nil prints a value the map holds no value for as __nil_text:
// [] for a nil slice, as Go prints one.
fn format_map_nil<K: Display + Ord + Clone, V>(map: &Rc<RefCell<Option<BTreeMap<K, Rc<RefCell<Option<V>>>>>>>, __nil_text: &str) -> String
where
    V: Display,
{
    let guard = map.borrow();
    if let Some(ref m) = *guard {
        let mut items: Vec<_> = m.iter().collect();
        items.sort_by_key(|(k, _)| (*k).clone());

        let formatted: Vec<String> = items
            .into_iter()
            .map(|(k, v)| {
                let v_guard = v.borrow();
                if let Some(ref val) = *v_guard {
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
pub struct Node {
    pub val: Rc<RefCell<Option<i64>>>,
    pub left: Rc<RefCell<Option<Node>>>,
    pub right: Rc<RefCell<Option<Node>>>,
}

impl Node {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.val.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = self.left.clone();
        let __go_clone_1_1 = self.right.clone();
        Self {
            val: __go_clone_0_0,
            left: __go_clone_1_0,
            right: __go_clone_1_1,
        }
    }
}


impl Default for Node {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_1_0 = Rc::new(RefCell::new(None));
        let __go_default_1_1 = Rc::new(RefCell::new(None));
        Self {
            val: __go_default_0_0,
            left: __go_default_1_0,
            right: __go_default_1_1,
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.val.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { let __guard = self.left.borrow(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_2 = format!("{}", { let __guard = self.right.borrow(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


#[derive(Debug, Clone, Default)]
pub struct Tree {
    pub root: Rc<RefCell<Option<Node>>>,
}

impl Tree {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.root.clone();
        Self {
            root: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.root.borrow(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone)]
pub struct LNode {
    pub v: Rc<RefCell<Option<i64>>>,
    pub next: Rc<RefCell<Option<LNode>>>,
}

impl LNode {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.v.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = self.next.clone();
        Self {
            v: __go_clone_0_0,
            next: __go_clone_1_0,
        }
    }
}


impl Default for LNode {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_1_0 = Rc::new(RefCell::new(None));
        Self {
            v: __go_default_0_0,
            next: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for LNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.v.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { let __guard = self.next.borrow(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone, Default)]
pub struct List {
    pub head: Rc<RefCell<Option<LNode>>>,
}

impl List {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.head.clone();
        Self {
            head: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().borrow().as_ref().unwrap()))
    }
}


#[derive(Clone)]
pub struct Holder {
    pub slot: Rc<RefCell<Option<__ptrSlot_LNode>>>,
}

impl Holder {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.slot.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        Self {
            slot: __go_clone_0_0,
        }
    }
}


impl Default for Holder {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(__ptrSlot_LNode::default())));
        Self {
            slot: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.slot.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Clone, Default)]
pub struct __ptrSlot_LNode {
    pub get: Rc<RefCell<Option<Rc<dyn Fn() -> Rc<RefCell<Option<LNode>>>>>>>,
    pub set: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<LNode>>>) -> ()>>>>,
}

impl __ptrSlot_LNode {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.get.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.set.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            get: __go_clone_0_0,
            set: __go_clone_1_0,
        }
    }
}

impl std::fmt::Display for __ptrSlot_LNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", "<func>");
        let __go_fmt_1 = format!("{}", "<func>");
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Clone, Default)]
pub struct __ptrSlot_Node {
    pub get: Rc<RefCell<Option<Rc<dyn Fn() -> Rc<RefCell<Option<Node>>>>>>>,
    pub set: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Node>>>) -> ()>>>>,
}

impl __ptrSlot_Node {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.get.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.set.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            get: __go_clone_0_0,
            set: __go_clone_1_0,
        }
    }
}

impl std::fmt::Display for __ptrSlot_Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", "<func>");
        let __go_fmt_1 = format!("{}", "<func>");
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Clone, Default)]
pub struct __ptrSlot_map_string_int {
    pub get: Rc<RefCell<Option<Rc<dyn Fn() -> Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>>>>,
    pub set: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>) -> ()>>>>,
}

impl __ptrSlot_map_string_int {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.get.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.set.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            get: __go_clone_0_0,
            set: __go_clone_1_0,
        }
    }
}

impl std::fmt::Display for __ptrSlot_map_string_int {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", "<func>");
        let __go_fmt_1 = format!("{}", "<func>");
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl List {
    pub fn string(&self) -> Rc<RefCell<Option<String>>> {
        let mut s = Rc::new(RefCell::new(Some("[".to_string())));
        let mut n = self.head.clone();
    while { let __nil_result = (*n.borrow()).is_some(); __nil_result } {
        if { let __left = n.clone(); let __right = self.head.clone(); let __both_nil = (*__left.borrow()).is_none() && (*__right.borrow()).is_none(); let __eq = __both_nil || Rc::ptr_eq(&__left, &__right) || { let __left_id = __left.borrow().as_ref().map(|__v| Rc::as_ptr(&__v.v) as *const () as usize); let __right_id = __right.borrow().as_ref().map(|__v| Rc::as_ptr(&__v.v) as *const () as usize); __left_id.is_some() && __left_id == __right_id }; !__eq } {
        { (*s.borrow_mut().as_mut().unwrap()).push_str(&" ".to_string()); };
    }
        { (*s.borrow_mut().as_mut().unwrap()).push_str(&{ let __s = Rc::new(RefCell::new(Some(format!("{}", format!("{}", (*(*n.borrow().as_ref().unwrap()).v.borrow().as_ref().unwrap())))))); let __value = (*__s.borrow().as_ref().unwrap()).clone(); __value }); };
        { let new_val = (*n.borrow().as_ref().unwrap()).next.clone(); n = new_val; };
    }
        return Rc::new(RefCell::new(Some(format!("{}{}", (*s.borrow().as_ref().unwrap()), "]".to_string()))));
    }
}

pub fn insert(mut pp: Rc<RefCell<Option<__ptrSlot_Node>>>, v: Rc<RefCell<Option<i64>>>) {
    while { let __nil_result = (*{ let __f_holder = (*pp.borrow().as_ref().unwrap()).get.clone(); let __f: Rc<dyn Fn() -> Rc<RefCell<Option<Node>>>> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)() }.borrow()).is_some(); __nil_result } {
        if (*v.borrow().as_ref().unwrap()) < (*(*{ let __f_holder = (*pp.borrow().as_ref().unwrap()).get.clone(); let __f: Rc<dyn Fn() -> Rc<RefCell<Option<Node>>>> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)() }.borrow().as_ref().unwrap()).val.borrow().as_ref().unwrap()) {
        { let new_val = { let __f_holder = Rc::new(RefCell::new(Some(Rc::new(move |__slot_o1: Rc<RefCell<Option<Node>>>| -> Rc<RefCell<Option<__ptrSlot_Node>>> {
        let __slot_o1_closure_clone = __slot_o1.clone(); return Rc::new(RefCell::new(Some(__ptrSlot_Node { get: Rc::new(RefCell::new(Some({ let __slot_o1_closure_clone_closure_clone = __slot_o1_closure_clone.clone(); Rc::new(move || -> Rc<RefCell<Option<Node>>> {
        (*__slot_o1_closure_clone_closure_clone.borrow().as_ref().unwrap()).left.clone()
    }) as Rc<dyn Fn() -> Rc<RefCell<Option<Node>>>> }))), set: Rc::new(RefCell::new(Some({ let __slot_o1_closure_clone_closure_clone = __slot_o1_closure_clone.clone(); Rc::new(move |__slot_v2: Rc<RefCell<Option<Node>>>| {
        { let new_val = __slot_v2.clone(); (*__slot_o1_closure_clone_closure_clone.borrow_mut().as_mut().unwrap()).left = new_val; };
    }) as Rc<dyn Fn(Rc<RefCell<Option<Node>>>) -> ()> }))), ..Default::default() })));
    }) as Rc<dyn Fn(Rc<RefCell<Option<Node>>>) -> Rc<RefCell<Option<__ptrSlot_Node>>>>))); let __f: Rc<dyn Fn(Rc<RefCell<Option<Node>>>) -> Rc<RefCell<Option<__ptrSlot_Node>>>> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)({ let __f_holder = (*pp.borrow().as_ref().unwrap()).get.clone(); let __f: Rc<dyn Fn() -> Rc<RefCell<Option<Node>>>> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)() }) }; let __moved_val = { let mut __guard = new_val.borrow_mut(); __guard.take() }; *pp.borrow_mut() = __moved_val; };
    } else {
        { let new_val = { let __f_holder = Rc::new(RefCell::new(Some(Rc::new(move |__slot_o3: Rc<RefCell<Option<Node>>>| -> Rc<RefCell<Option<__ptrSlot_Node>>> {
        let __slot_o3_closure_clone = __slot_o3.clone(); return Rc::new(RefCell::new(Some(__ptrSlot_Node { get: Rc::new(RefCell::new(Some({ let __slot_o3_closure_clone_closure_clone = __slot_o3_closure_clone.clone(); Rc::new(move || -> Rc<RefCell<Option<Node>>> {
        (*__slot_o3_closure_clone_closure_clone.borrow().as_ref().unwrap()).right.clone()
    }) as Rc<dyn Fn() -> Rc<RefCell<Option<Node>>>> }))), set: Rc::new(RefCell::new(Some({ let __slot_o3_closure_clone_closure_clone = __slot_o3_closure_clone.clone(); Rc::new(move |__slot_v4: Rc<RefCell<Option<Node>>>| {
        { let new_val = __slot_v4.clone(); (*__slot_o3_closure_clone_closure_clone.borrow_mut().as_mut().unwrap()).right = new_val; };
    }) as Rc<dyn Fn(Rc<RefCell<Option<Node>>>) -> ()> }))), ..Default::default() })));
    }) as Rc<dyn Fn(Rc<RefCell<Option<Node>>>) -> Rc<RefCell<Option<__ptrSlot_Node>>>>))); let __f: Rc<dyn Fn(Rc<RefCell<Option<Node>>>) -> Rc<RefCell<Option<__ptrSlot_Node>>>> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)({ let __f_holder = (*pp.borrow().as_ref().unwrap()).get.clone(); let __f: Rc<dyn Fn() -> Rc<RefCell<Option<Node>>>> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)() }) }; let __moved_val = { let mut __guard = new_val.borrow_mut(); __guard.take() }; *pp.borrow_mut() = __moved_val; };
    }
    }
    { let __f_holder = (*pp.borrow().as_ref().unwrap()).set.clone(); let __f: Rc<dyn Fn(Rc<RefCell<Option<Node>>>) -> ()> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(Node { val: Rc::new(RefCell::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })))) };
}

pub fn inorder(n: Rc<RefCell<Option<Node>>>, out: Rc<RefCell<Option<Vec<i64>>>>) {
    if { let __nil_result = (*n.borrow()).is_none(); __nil_result } {
        return;
    }
    inorder({ let __field = (*n.borrow().as_ref().unwrap()).left.clone(); __field }, out.clone());
    { let new_val = { let __append_target = out.clone(); let __append_value = { let __selector_holder = (*n.borrow().as_ref().unwrap()).val.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }; let mut __appended: Vec<i64> = (*__append_target.borrow()).clone().unwrap_or_default(); __appended.push(__append_value); Rc::new(RefCell::new(Some(__appended))) }; let __cloned_val = { let __guard = new_val.borrow(); (*__guard).clone() }; *out.borrow_mut() = __cloned_val; };
    inorder({ let __field = (*n.borrow().as_ref().unwrap()).right.clone(); __field }, out.clone());
}

pub fn min_link(mut pp: Rc<RefCell<Option<__ptrSlot_Node>>>) -> Rc<RefCell<Option<__ptrSlot_Node>>> {
    while { let __nil_target = (*{ let __f_holder = (*pp.borrow().as_ref().unwrap()).get.clone(); let __f: Rc<dyn Fn() -> Rc<RefCell<Option<Node>>>> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)() }.borrow().as_ref().unwrap()).left.clone(); let __nil_result = (*__nil_target.borrow()).is_some(); __nil_result } {
        { let new_val = { let __f_holder = Rc::new(RefCell::new(Some(Rc::new(move |__slot_o5: Rc<RefCell<Option<Node>>>| -> Rc<RefCell<Option<__ptrSlot_Node>>> {
        let __slot_o5_closure_clone = __slot_o5.clone(); return Rc::new(RefCell::new(Some(__ptrSlot_Node { get: Rc::new(RefCell::new(Some({ let __slot_o5_closure_clone_closure_clone = __slot_o5_closure_clone.clone(); Rc::new(move || -> Rc<RefCell<Option<Node>>> {
        (*__slot_o5_closure_clone_closure_clone.borrow().as_ref().unwrap()).left.clone()
    }) as Rc<dyn Fn() -> Rc<RefCell<Option<Node>>>> }))), set: Rc::new(RefCell::new(Some({ let __slot_o5_closure_clone_closure_clone = __slot_o5_closure_clone.clone(); Rc::new(move |__slot_v6: Rc<RefCell<Option<Node>>>| {
        { let new_val = __slot_v6.clone(); (*__slot_o5_closure_clone_closure_clone.borrow_mut().as_mut().unwrap()).left = new_val; };
    }) as Rc<dyn Fn(Rc<RefCell<Option<Node>>>) -> ()> }))), ..Default::default() })));
    }) as Rc<dyn Fn(Rc<RefCell<Option<Node>>>) -> Rc<RefCell<Option<__ptrSlot_Node>>>>))); let __f: Rc<dyn Fn(Rc<RefCell<Option<Node>>>) -> Rc<RefCell<Option<__ptrSlot_Node>>>> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)({ let __f_holder = (*pp.borrow().as_ref().unwrap()).get.clone(); let __f: Rc<dyn Fn() -> Rc<RefCell<Option<Node>>>> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)() }) }; let __moved_val = { let mut __guard = new_val.borrow_mut(); __guard.take() }; *pp.borrow_mut() = __moved_val; };
    }
    return { let __owned = pp.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) };
}

pub fn remove(l: Rc<RefCell<Option<List>>>, v: Rc<RefCell<Option<i64>>>) -> bool {
    let mut pp = { let __f_holder = Rc::new(RefCell::new(Some(Rc::new(move |__slot_o7: Rc<RefCell<Option<List>>>| -> Rc<RefCell<Option<__ptrSlot_LNode>>> {
        let __slot_o7_closure_clone = __slot_o7.clone(); return Rc::new(RefCell::new(Some(__ptrSlot_LNode { get: Rc::new(RefCell::new(Some({ let __slot_o7_closure_clone_closure_clone = __slot_o7_closure_clone.clone(); Rc::new(move || -> Rc<RefCell<Option<LNode>>> {
        (*__slot_o7_closure_clone_closure_clone.borrow().as_ref().unwrap()).head.clone()
    }) as Rc<dyn Fn() -> Rc<RefCell<Option<LNode>>>> }))), set: Rc::new(RefCell::new(Some({ let __slot_o7_closure_clone_closure_clone = __slot_o7_closure_clone.clone(); Rc::new(move |__slot_v8: Rc<RefCell<Option<LNode>>>| {
        { let new_val = __slot_v8.clone(); (*__slot_o7_closure_clone_closure_clone.borrow_mut().as_mut().unwrap()).head = new_val; };
    }) as Rc<dyn Fn(Rc<RefCell<Option<LNode>>>) -> ()> }))), ..Default::default() })));
    }) as Rc<dyn Fn(Rc<RefCell<Option<List>>>) -> Rc<RefCell<Option<__ptrSlot_LNode>>>>))); let __f: Rc<dyn Fn(Rc<RefCell<Option<List>>>) -> Rc<RefCell<Option<__ptrSlot_LNode>>>> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)(l.clone()) };
    while { let __nil_result = (*{ let __f_holder = (*pp.borrow().as_ref().unwrap()).get.clone(); let __f: Rc<dyn Fn() -> Rc<RefCell<Option<LNode>>>> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)() }.borrow()).is_some(); __nil_result } {
        if (*(*{ let __f_holder = (*pp.borrow().as_ref().unwrap()).get.clone(); let __f: Rc<dyn Fn() -> Rc<RefCell<Option<LNode>>>> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)() }.borrow().as_ref().unwrap()).v.borrow().as_ref().unwrap()) == (*v.borrow().as_ref().unwrap()) {
        { let __f_holder = (*pp.borrow().as_ref().unwrap()).set.clone(); let __f: Rc<dyn Fn(Rc<RefCell<Option<LNode>>>) -> ()> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)({ let __field = (*{ let __f_holder = (*pp.borrow().as_ref().unwrap()).get.clone(); let __f: Rc<dyn Fn() -> Rc<RefCell<Option<LNode>>>> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)() }.borrow().as_ref().unwrap()).next.clone(); __field }) };
        return true;
    }
        { let new_val = { let __f_holder = Rc::new(RefCell::new(Some(Rc::new(move |__slot_o9: Rc<RefCell<Option<LNode>>>| -> Rc<RefCell<Option<__ptrSlot_LNode>>> {
        let __slot_o9_closure_clone = __slot_o9.clone(); return Rc::new(RefCell::new(Some(__ptrSlot_LNode { get: Rc::new(RefCell::new(Some({ let __slot_o9_closure_clone_closure_clone = __slot_o9_closure_clone.clone(); Rc::new(move || -> Rc<RefCell<Option<LNode>>> {
        (*__slot_o9_closure_clone_closure_clone.borrow().as_ref().unwrap()).next.clone()
    }) as Rc<dyn Fn() -> Rc<RefCell<Option<LNode>>>> }))), set: Rc::new(RefCell::new(Some({ let __slot_o9_closure_clone_closure_clone = __slot_o9_closure_clone.clone(); Rc::new(move |__slot_v10: Rc<RefCell<Option<LNode>>>| {
        { let new_val = __slot_v10.clone(); (*__slot_o9_closure_clone_closure_clone.borrow_mut().as_mut().unwrap()).next = new_val; };
    }) as Rc<dyn Fn(Rc<RefCell<Option<LNode>>>) -> ()> }))), ..Default::default() })));
    }) as Rc<dyn Fn(Rc<RefCell<Option<LNode>>>) -> Rc<RefCell<Option<__ptrSlot_LNode>>>>))); let __f: Rc<dyn Fn(Rc<RefCell<Option<LNode>>>) -> Rc<RefCell<Option<__ptrSlot_LNode>>>> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)({ let __f_holder = (*pp.borrow().as_ref().unwrap()).get.clone(); let __f: Rc<dyn Fn() -> Rc<RefCell<Option<LNode>>>> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)() }) }; let __moved_val = { let mut __guard = new_val.borrow_mut(); __guard.take() }; *pp.borrow_mut() = __moved_val; };
    }
    false
}

pub fn push(pp: Rc<RefCell<Option<__ptrSlot_LNode>>>, v: Rc<RefCell<Option<i64>>>) {
    { let __f_holder = (*pp.borrow().as_ref().unwrap()).set.clone(); let __f: Rc<dyn Fn(Rc<RefCell<Option<LNode>>>) -> ()> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(LNode { v: Rc::new(RefCell::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), next: { let __f_holder = (*pp.borrow().as_ref().unwrap()).get.clone(); let __f: Rc<dyn Fn() -> Rc<RefCell<Option<LNode>>>> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)() }.clone(), ..Default::default() })))) };
}

pub fn add_count(m: Rc<RefCell<Option<__ptrSlot_map_string_int>>>, k: Rc<RefCell<Option<String>>>) {
    if { let __nil_result = (*{ let __f_holder = (*m.borrow().as_ref().unwrap()).get.clone(); let __f: Rc<dyn Fn() -> Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)() }.borrow()).is_none(); __nil_result } {
        { let __f_holder = (*m.borrow().as_ref().unwrap()).set.clone(); let __f: Rc<dyn Fn(Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>) -> ()> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i64>>>>::from([]))))) };
    }
    { let __map_holder = ({ let __f_holder = (*m.borrow().as_ref().unwrap()).get.clone(); let __f: Rc<dyn Fn() -> Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)() }).clone(); let __key = (*k.borrow().as_ref().unwrap()).clone(); let mut __map_guard = __map_holder.borrow_mut(); let __map = __map_guard.as_mut().unwrap(); let __entry = __map.entry(__key).or_insert_with(|| Rc::new(RefCell::new(Some(0)))); let mut __value = __entry.borrow_mut(); * __value = Some((*__value.as_ref().unwrap()).clone() + 1); }
}

fn main() {
    go_install_panic_exit();
    let mut root: Rc<RefCell<Option<Node>>> = Rc::new(RefCell::new(None));
    let mut __cell_root_1 = Rc::new(RefCell::new(Some(AnonymousStruct1 { v: root.clone() })));
    for v in vec![5i64, 3i64, 8i64, 1i64, 4i64].iter().copied() {
        let __cell_root_1_closure_clone = __cell_root_1.clone(); insert(Rc::new(RefCell::new(Some({ let __arg_value = __ptrSlot_Node { get: Rc::new(RefCell::new(Some({ let __cell_root_1_closure_clone_closure_clone = __cell_root_1_closure_clone.clone(); Rc::new(move || -> Rc<RefCell<Option<Node>>> {
        return (*__cell_root_1_closure_clone_closure_clone.borrow().as_ref().unwrap()).v.clone();
    }) as Rc<dyn Fn() -> Rc<RefCell<Option<Node>>>> }))), set: Rc::new(RefCell::new(Some({ let __cell_root_1_closure_clone_closure_clone = __cell_root_1_closure_clone.clone(); Rc::new(move |__slot_v11: Rc<RefCell<Option<Node>>>| {
        { let new_val = __slot_v11.clone(); (*__cell_root_1_closure_clone_closure_clone.borrow_mut().as_mut().unwrap()).v = new_val; };
    }) as Rc<dyn Fn(Rc<RefCell<Option<Node>>>) -> ()> }))), ..Default::default() }; __arg_value }))), Rc::new(RefCell::new(Some(v.clone()))));
    }
    let mut vals: Rc<RefCell<Option<Vec<i64>>>> = Rc::new(RefCell::new(None));
    inorder({ let __field = (*__cell_root_1.borrow().as_ref().unwrap()).v.clone(); __field }, vals.clone());
    println!("{}", format!("{}", format_slice(&vals)));
    let __cell_root_1_closure_clone = __cell_root_1.clone(); let mut low = min_link(Rc::new(RefCell::new(Some({ let __arg_value = __ptrSlot_Node { get: Rc::new(RefCell::new(Some({ let __cell_root_1_closure_clone_closure_clone = __cell_root_1_closure_clone.clone(); Rc::new(move || -> Rc<RefCell<Option<Node>>> {
        return (*__cell_root_1_closure_clone_closure_clone.borrow().as_ref().unwrap()).v.clone();
    }) as Rc<dyn Fn() -> Rc<RefCell<Option<Node>>>> }))), set: Rc::new(RefCell::new(Some({ let __cell_root_1_closure_clone_closure_clone = __cell_root_1_closure_clone.clone(); Rc::new(move |__slot_v12: Rc<RefCell<Option<Node>>>| {
        { let new_val = __slot_v12.clone(); (*__cell_root_1_closure_clone_closure_clone.borrow_mut().as_mut().unwrap()).v = new_val; };
    }) as Rc<dyn Fn(Rc<RefCell<Option<Node>>>) -> ()> }))), ..Default::default() }; __arg_value }))));
    println!("{} {}", format!("{}", "min".to_string()), format!("{}", (*(*{ let __f_holder = (*low.borrow().as_ref().unwrap()).get.clone(); let __f: Rc<dyn Fn() -> Rc<RefCell<Option<Node>>>> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)() }.borrow().as_ref().unwrap()).val.borrow().as_ref().unwrap())));
    { let __f_holder = (*low.borrow().as_ref().unwrap()).set.clone(); let __f: Rc<dyn Fn(Rc<RefCell<Option<Node>>>) -> ()> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)({ let __field = (*{ let __f_holder = (*low.borrow().as_ref().unwrap()).get.clone(); let __f: Rc<dyn Fn() -> Rc<RefCell<Option<Node>>>> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)() }.borrow().as_ref().unwrap()).right.clone(); __field }) };
    { let __slice_header = None; *vals.borrow_mut() = __slice_header; };
    inorder({ let __field = (*__cell_root_1.borrow().as_ref().unwrap()).v.clone(); __field }, vals.clone());
    println!("{} {}", format!("{}", "without min".to_string()), format!("{}", format_slice(&vals)));

    let mut t = Rc::new(RefCell::new(Some(Tree { root: Default::default() })));
    for v in vec![2i64, 1i64, 3i64].iter().copied() {
        insert({ let __f_holder = Rc::new(RefCell::new(Some(Rc::new(move |__slot_o13: Rc<RefCell<Option<Tree>>>| -> Rc<RefCell<Option<__ptrSlot_Node>>> {
        let __slot_o13_closure_clone = __slot_o13.clone(); return Rc::new(RefCell::new(Some(__ptrSlot_Node { get: Rc::new(RefCell::new(Some({ let __slot_o13_closure_clone_closure_clone = __slot_o13_closure_clone.clone(); Rc::new(move || -> Rc<RefCell<Option<Node>>> {
        return (*__slot_o13_closure_clone_closure_clone.borrow().as_ref().unwrap()).root.clone();
    }) as Rc<dyn Fn() -> Rc<RefCell<Option<Node>>>> }))), set: Rc::new(RefCell::new(Some({ let __slot_o13_closure_clone_closure_clone = __slot_o13_closure_clone.clone(); Rc::new(move |__slot_v14: Rc<RefCell<Option<Node>>>| {
        { let new_val = __slot_v14.clone(); (*__slot_o13_closure_clone_closure_clone.borrow_mut().as_mut().unwrap()).root = new_val; };
    }) as Rc<dyn Fn(Rc<RefCell<Option<Node>>>) -> ()> }))), ..Default::default() })));
    }) as Rc<dyn Fn(Rc<RefCell<Option<Tree>>>) -> Rc<RefCell<Option<__ptrSlot_Node>>>>))); let __f: Rc<dyn Fn(Rc<RefCell<Option<Tree>>>) -> Rc<RefCell<Option<__ptrSlot_Node>>>> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)(t.clone()) }, Rc::new(RefCell::new(Some(v.clone()))));
    }
    { let __slice_header = None; *vals.borrow_mut() = __slice_header; };
    inorder({ let __field = (*t.borrow().as_ref().unwrap()).root.clone(); __field }, vals.clone());
    println!("{} {}", format!("{}", format_slice(&vals)), format!("{}", (*(*(*t.borrow().as_ref().unwrap()).root.borrow().as_ref().unwrap()).val.borrow().as_ref().unwrap())));

    let mut l = Rc::new(RefCell::new(Some(List { head: Default::default() })));
    let mut i = Rc::new(RefCell::new(Some(1i64)));
    while (*i.borrow().as_ref().unwrap()) <= 4 {
        push({ let __f_holder = Rc::new(RefCell::new(Some(Rc::new(move |__slot_o15: Rc<RefCell<Option<List>>>| -> Rc<RefCell<Option<__ptrSlot_LNode>>> {
        let __slot_o15_closure_clone = __slot_o15.clone(); return Rc::new(RefCell::new(Some(__ptrSlot_LNode { get: Rc::new(RefCell::new(Some({ let __slot_o15_closure_clone_closure_clone = __slot_o15_closure_clone.clone(); Rc::new(move || -> Rc<RefCell<Option<LNode>>> {
        (*__slot_o15_closure_clone_closure_clone.borrow().as_ref().unwrap()).head.clone()
    }) as Rc<dyn Fn() -> Rc<RefCell<Option<LNode>>>> }))), set: Rc::new(RefCell::new(Some({ let __slot_o15_closure_clone_closure_clone = __slot_o15_closure_clone.clone(); Rc::new(move |__slot_v16: Rc<RefCell<Option<LNode>>>| {
        { let new_val = __slot_v16.clone(); (*__slot_o15_closure_clone_closure_clone.borrow_mut().as_mut().unwrap()).head = new_val; };
    }) as Rc<dyn Fn(Rc<RefCell<Option<LNode>>>) -> ()> }))), ..Default::default() })));
    }) as Rc<dyn Fn(Rc<RefCell<Option<List>>>) -> Rc<RefCell<Option<__ptrSlot_LNode>>>>))); let __f: Rc<dyn Fn(Rc<RefCell<Option<List>>>) -> Rc<RefCell<Option<__ptrSlot_LNode>>>> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)(l.clone()) }, Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    println!("{}", format!("{}", { let __text = match l.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    println!("{} {} {} {}", format!("{}", remove(l.clone(), Rc::new(RefCell::new(Some(3))))), format!("{}", remove(l.clone(), Rc::new(RefCell::new(Some(4))))), format!("{}", remove(l.clone(), Rc::new(RefCell::new(Some(9))))), format!("{}", { let __text = match l.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

    let mut head: Rc<RefCell<Option<LNode>>> = Rc::new(RefCell::new(None));
    let mut __cell_head_2 = Rc::new(RefCell::new(Some(AnonymousStruct2 { v: head.clone() })));
    let __cell_head_2_closure_clone = __cell_head_2.clone(); push(Rc::new(RefCell::new(Some({ let __arg_value = __ptrSlot_LNode { get: Rc::new(RefCell::new(Some({ let __cell_head_2_closure_clone_closure_clone = __cell_head_2_closure_clone.clone(); Rc::new(move || -> Rc<RefCell<Option<LNode>>> {
        return (*__cell_head_2_closure_clone_closure_clone.borrow().as_ref().unwrap()).v.clone();
    }) as Rc<dyn Fn() -> Rc<RefCell<Option<LNode>>>> }))), set: Rc::new(RefCell::new(Some({ let __cell_head_2_closure_clone_closure_clone = __cell_head_2_closure_clone.clone(); Rc::new(move |__slot_v17: Rc<RefCell<Option<LNode>>>| {
        { let new_val = __slot_v17.clone(); (*__cell_head_2_closure_clone_closure_clone.borrow_mut().as_mut().unwrap()).v = new_val; };
    }) as Rc<dyn Fn(Rc<RefCell<Option<LNode>>>) -> ()> }))), ..Default::default() }; __arg_value }))), Rc::new(RefCell::new(Some(1))));
    let __cell_head_2_closure_clone = __cell_head_2.clone(); push(Rc::new(RefCell::new(Some({ let __arg_value = __ptrSlot_LNode { get: Rc::new(RefCell::new(Some({ let __cell_head_2_closure_clone_closure_clone = __cell_head_2_closure_clone.clone(); Rc::new(move || -> Rc<RefCell<Option<LNode>>> {
        return (*__cell_head_2_closure_clone_closure_clone.borrow().as_ref().unwrap()).v.clone();
    }) as Rc<dyn Fn() -> Rc<RefCell<Option<LNode>>>> }))), set: Rc::new(RefCell::new(Some({ let __cell_head_2_closure_clone_closure_clone = __cell_head_2_closure_clone.clone(); Rc::new(move |__slot_v18: Rc<RefCell<Option<LNode>>>| {
        { let new_val = __slot_v18.clone(); (*__cell_head_2_closure_clone_closure_clone.borrow_mut().as_mut().unwrap()).v = new_val; };
    }) as Rc<dyn Fn(Rc<RefCell<Option<LNode>>>) -> ()> }))), ..Default::default() }; __arg_value }))), Rc::new(RefCell::new(Some(2))));
    println!("{}", format!("{}", { let __text = match Rc::new(RefCell::new(Some(List { head: { let __field = (*__cell_head_2.borrow().as_ref().unwrap()).v.clone(); __field }, ..Default::default() }))).borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

    let mut arr = Rc::new(RefCell::new(Some([Rc::new(RefCell::new(None)), Rc::new(RefCell::new(None))])));
    push({ let __f_holder = Rc::new(RefCell::new(Some(Rc::new(move |__slot_s19: Rc<RefCell<Option<[Rc<RefCell<Option<LNode>>>; 2]>>>, __slot_i20: Rc<RefCell<Option<i64>>>| -> Rc<RefCell<Option<__ptrSlot_LNode>>> {
        let __slot_i20_closure_clone = __slot_i20.clone(); let __slot_s19_closure_clone = __slot_s19.clone(); return Rc::new(RefCell::new(Some(__ptrSlot_LNode { get: Rc::new(RefCell::new(Some({ let __slot_i20_closure_clone_closure_clone = __slot_i20_closure_clone.clone(); let __slot_s19_closure_clone_closure_clone = __slot_s19_closure_clone.clone(); Rc::new(move || -> Rc<RefCell<Option<LNode>>> {
        (*__slot_s19_closure_clone_closure_clone.borrow().as_ref().unwrap())[((*__slot_i20_closure_clone_closure_clone.borrow().as_ref().unwrap())) as usize].clone()
    }) as Rc<dyn Fn() -> Rc<RefCell<Option<LNode>>>> }))), set: Rc::new(RefCell::new(Some({ let __slot_i20_closure_clone_closure_clone = __slot_i20_closure_clone.clone(); let __slot_s19_closure_clone_closure_clone = __slot_s19_closure_clone.clone(); Rc::new(move |__slot_v21: Rc<RefCell<Option<LNode>>>| {
        { let __idx = ((*__slot_i20_closure_clone_closure_clone.borrow().as_ref().unwrap())) as usize; let __rhs = __slot_v21.clone(); (*__slot_s19_closure_clone_closure_clone.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    }) as Rc<dyn Fn(Rc<RefCell<Option<LNode>>>) -> ()> }))), ..Default::default() })));
    }) as Rc<dyn Fn(Rc<RefCell<Option<[Rc<RefCell<Option<LNode>>>; 2]>>>, Rc<RefCell<Option<i64>>>) -> Rc<RefCell<Option<__ptrSlot_LNode>>>>))); let __f: Rc<dyn Fn(Rc<RefCell<Option<[Rc<RefCell<Option<LNode>>>; 2]>>>, Rc<RefCell<Option<i64>>>) -> Rc<RefCell<Option<__ptrSlot_LNode>>>> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)(arr.clone(), Rc::new(RefCell::new(Some(0)))) }, Rc::new(RefCell::new(Some(7))));
    push({ let __f_holder = Rc::new(RefCell::new(Some(Rc::new(move |__slot_s22: Rc<RefCell<Option<[Rc<RefCell<Option<LNode>>>; 2]>>>, __slot_i23: Rc<RefCell<Option<i64>>>| -> Rc<RefCell<Option<__ptrSlot_LNode>>> {
        let __slot_i23_closure_clone = __slot_i23.clone(); let __slot_s22_closure_clone = __slot_s22.clone(); return Rc::new(RefCell::new(Some(__ptrSlot_LNode { get: Rc::new(RefCell::new(Some({ let __slot_i23_closure_clone_closure_clone = __slot_i23_closure_clone.clone(); let __slot_s22_closure_clone_closure_clone = __slot_s22_closure_clone.clone(); Rc::new(move || -> Rc<RefCell<Option<LNode>>> {
        (*__slot_s22_closure_clone_closure_clone.borrow().as_ref().unwrap())[((*__slot_i23_closure_clone_closure_clone.borrow().as_ref().unwrap())) as usize].clone()
    }) as Rc<dyn Fn() -> Rc<RefCell<Option<LNode>>>> }))), set: Rc::new(RefCell::new(Some({ let __slot_i23_closure_clone_closure_clone = __slot_i23_closure_clone.clone(); let __slot_s22_closure_clone_closure_clone = __slot_s22_closure_clone.clone(); Rc::new(move |__slot_v24: Rc<RefCell<Option<LNode>>>| {
        { let __idx = ((*__slot_i23_closure_clone_closure_clone.borrow().as_ref().unwrap())) as usize; let __rhs = __slot_v24.clone(); (*__slot_s22_closure_clone_closure_clone.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    }) as Rc<dyn Fn(Rc<RefCell<Option<LNode>>>) -> ()> }))), ..Default::default() })));
    }) as Rc<dyn Fn(Rc<RefCell<Option<[Rc<RefCell<Option<LNode>>>; 2]>>>, Rc<RefCell<Option<i64>>>) -> Rc<RefCell<Option<__ptrSlot_LNode>>>>))); let __f: Rc<dyn Fn(Rc<RefCell<Option<[Rc<RefCell<Option<LNode>>>; 2]>>>, Rc<RefCell<Option<i64>>>) -> Rc<RefCell<Option<__ptrSlot_LNode>>>> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)(arr.clone(), Rc::new(RefCell::new(Some(0)))) }, Rc::new(RefCell::new(Some(8))));
    println!("{} {}", format!("{}", { let __text = match Rc::new(RefCell::new(Some(List { head: (*arr.borrow().as_ref().unwrap())[(0) as usize].clone().clone(), ..Default::default() }))).borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __nil_result = (*(*arr.borrow().as_ref().unwrap())[(1) as usize].clone().borrow()).is_none(); __nil_result }));

    let mut links = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(None)), Rc::new(RefCell::new(None))])));
    push({ let __f_holder = Rc::new(RefCell::new(Some(Rc::new(move |__slot_s25: Rc<RefCell<Option<Vec<Rc<RefCell<Option<LNode>>>>>>>, __slot_i26: Rc<RefCell<Option<i64>>>| -> Rc<RefCell<Option<__ptrSlot_LNode>>> {
        let mut __slot_s25 = Rc::new(RefCell::new((*__slot_s25.borrow()).clone()));
        let __slot_i26_closure_clone = __slot_i26.clone(); let __slot_s25_closure_clone = __slot_s25.clone(); return Rc::new(RefCell::new(Some(__ptrSlot_LNode { get: Rc::new(RefCell::new(Some({ let __slot_i26_closure_clone_closure_clone = __slot_i26_closure_clone.clone(); let __slot_s25_closure_clone_closure_clone = __slot_s25_closure_clone.clone(); Rc::new(move || -> Rc<RefCell<Option<LNode>>> {
        (*__slot_s25_closure_clone_closure_clone.borrow().as_ref().unwrap())[((*__slot_i26_closure_clone_closure_clone.borrow().as_ref().unwrap())) as usize].clone()
    }) as Rc<dyn Fn() -> Rc<RefCell<Option<LNode>>>> }))), set: Rc::new(RefCell::new(Some({ let __slot_i26_closure_clone_closure_clone = __slot_i26_closure_clone.clone(); let __slot_s25_closure_clone_closure_clone = __slot_s25_closure_clone.clone(); Rc::new(move |__slot_v27: Rc<RefCell<Option<LNode>>>| {
        { let __idx = ((*__slot_i26_closure_clone_closure_clone.borrow().as_ref().unwrap())) as usize; let __rhs = __slot_v27.clone(); (*__slot_s25_closure_clone_closure_clone.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    }) as Rc<dyn Fn(Rc<RefCell<Option<LNode>>>) -> ()> }))), ..Default::default() })));
    }) as Rc<dyn Fn(Rc<RefCell<Option<Vec<Rc<RefCell<Option<LNode>>>>>>>, Rc<RefCell<Option<i64>>>) -> Rc<RefCell<Option<__ptrSlot_LNode>>>>))); let __f: Rc<dyn Fn(Rc<RefCell<Option<Vec<Rc<RefCell<Option<LNode>>>>>>>, Rc<RefCell<Option<i64>>>) -> Rc<RefCell<Option<__ptrSlot_LNode>>>> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)(links.clone(), Rc::new(RefCell::new(Some(1)))) }, Rc::new(RefCell::new(Some(6))));
    println!("{} {}", format!("{}", { let __nil_result = (*(*links.borrow().as_ref().unwrap())[(0) as usize].clone().borrow()).is_none(); __nil_result }), format!("{}", (*(*(*links.borrow().as_ref().unwrap())[(1) as usize].clone().borrow().as_ref().unwrap()).v.borrow().as_ref().unwrap())));

    let mut first = { let __f_holder = Rc::new(RefCell::new(Some(Rc::new(move |__slot_s28: Rc<RefCell<Option<Vec<Rc<RefCell<Option<LNode>>>>>>>, __slot_i29: Rc<RefCell<Option<i64>>>| -> Rc<RefCell<Option<__ptrSlot_LNode>>> {
        let mut __slot_s28 = Rc::new(RefCell::new((*__slot_s28.borrow()).clone()));
        let __slot_i29_closure_clone = __slot_i29.clone(); let __slot_s28_closure_clone = __slot_s28.clone(); return Rc::new(RefCell::new(Some(__ptrSlot_LNode { get: Rc::new(RefCell::new(Some({ let __slot_i29_closure_clone_closure_clone = __slot_i29_closure_clone.clone(); let __slot_s28_closure_clone_closure_clone = __slot_s28_closure_clone.clone(); Rc::new(move || -> Rc<RefCell<Option<LNode>>> {
        (*__slot_s28_closure_clone_closure_clone.borrow().as_ref().unwrap())[((*__slot_i29_closure_clone_closure_clone.borrow().as_ref().unwrap())) as usize].clone()
    }) as Rc<dyn Fn() -> Rc<RefCell<Option<LNode>>>> }))), set: Rc::new(RefCell::new(Some({ let __slot_i29_closure_clone_closure_clone = __slot_i29_closure_clone.clone(); let __slot_s28_closure_clone_closure_clone = __slot_s28_closure_clone.clone(); Rc::new(move |__slot_v30: Rc<RefCell<Option<LNode>>>| {
        { let __idx = ((*__slot_i29_closure_clone_closure_clone.borrow().as_ref().unwrap())) as usize; let __rhs = __slot_v30.clone(); (*__slot_s28_closure_clone_closure_clone.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    }) as Rc<dyn Fn(Rc<RefCell<Option<LNode>>>) -> ()> }))), ..Default::default() })));
    }) as Rc<dyn Fn(Rc<RefCell<Option<Vec<Rc<RefCell<Option<LNode>>>>>>>, Rc<RefCell<Option<i64>>>) -> Rc<RefCell<Option<__ptrSlot_LNode>>>>))); let __f: Rc<dyn Fn(Rc<RefCell<Option<Vec<Rc<RefCell<Option<LNode>>>>>>>, Rc<RefCell<Option<i64>>>) -> Rc<RefCell<Option<__ptrSlot_LNode>>>> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)(links.clone(), Rc::new(RefCell::new(Some(0)))) };
    let mut old = Rc::new(RefCell::new(links.borrow().clone()));
    { let __slice_header = GoSliceHeader::go_slice_header(&(Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(LNode { v: Rc::new(RefCell::new(Some(2))), ..Default::default() }))), Rc::new(RefCell::new(None))]))))); *links.borrow_mut() = __slice_header; };
    push(Rc::new(RefCell::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))), Rc::new(RefCell::new(Some(5))));
    println!("{} {}", format!("{}", (*(*(*old.borrow().as_ref().unwrap())[(0) as usize].clone().borrow().as_ref().unwrap()).v.borrow().as_ref().unwrap())), format!("{}", (*(*(*links.borrow().as_ref().unwrap())[(0) as usize].clone().borrow().as_ref().unwrap()).v.borrow().as_ref().unwrap())));

    let __cell_head_2_closure_clone = __cell_head_2.clone(); let mut h = Rc::new(RefCell::new(Some(Holder { slot: Rc::new(RefCell::new(Some(__ptrSlot_LNode { get: Rc::new(RefCell::new(Some({ let __cell_head_2_closure_clone_closure_clone = __cell_head_2_closure_clone.clone(); Rc::new(move || -> Rc<RefCell<Option<LNode>>> {
        return (*__cell_head_2_closure_clone_closure_clone.borrow().as_ref().unwrap()).v.clone();
    }) as Rc<dyn Fn() -> Rc<RefCell<Option<LNode>>>> }))), set: Rc::new(RefCell::new(Some({ let __cell_head_2_closure_clone_closure_clone = __cell_head_2_closure_clone.clone(); Rc::new(move |__slot_v31: Rc<RefCell<Option<LNode>>>| {
        { let new_val = __slot_v31.clone(); (*__cell_head_2_closure_clone_closure_clone.borrow_mut().as_mut().unwrap()).v = new_val; };
    }) as Rc<dyn Fn(Rc<RefCell<Option<LNode>>>) -> ()> }))), ..Default::default() }))), ..Default::default() })));
    push(Rc::new(RefCell::new(Some({ let __selector_holder = (*h.borrow().as_ref().unwrap()).slot.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = __selector_guard.as_ref().unwrap().__go_value_clone(); std::mem::drop(__selector_guard); __cloned }))), Rc::new(RefCell::new(Some(3))));
    println!("{} {}", format!("{}", { let __text = match Rc::new(RefCell::new(Some(List { head: { let __field = (*__cell_head_2.borrow().as_ref().unwrap()).v.clone(); __field }, ..Default::default() }))).borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __nil_target = (*(*h.borrow().as_ref().unwrap()).slot.borrow().as_ref().unwrap()).get.clone(); let __nil_result = (*__nil_target.borrow()).is_some(); __nil_result }));

    let mut empty: Rc<RefCell<Option<Holder>>> = Rc::new(RefCell::new(Some(Default::default())));
    println!("{}", format!("{}", { let __nil_target = (*(*empty.borrow().as_ref().unwrap()).slot.borrow().as_ref().unwrap()).get.clone(); let __nil_result = (*__nil_target.borrow()).is_none(); __nil_result }));

    let mut pp = { let __f_holder = Rc::new(RefCell::new(Some(Rc::new(move || -> Rc<RefCell<Option<__ptrSlot_LNode>>> {
        let mut __slot_p32: Rc<RefCell<Option<LNode>>> = Rc::new(RefCell::new(None));
        let mut __cell___slot_p32_3 = Rc::new(RefCell::new(Some(AnonymousStruct2 { v: __slot_p32.clone() })));
        let __cell___slot_p32_3_closure_clone = __cell___slot_p32_3.clone(); return Rc::new(RefCell::new(Some(__ptrSlot_LNode { get: Rc::new(RefCell::new(Some({ let __cell___slot_p32_3_closure_clone_closure_clone = __cell___slot_p32_3_closure_clone.clone(); Rc::new(move || -> Rc<RefCell<Option<LNode>>> {
        return (*__cell___slot_p32_3_closure_clone_closure_clone.borrow().as_ref().unwrap()).v.clone();
    }) as Rc<dyn Fn() -> Rc<RefCell<Option<LNode>>>> }))), set: Rc::new(RefCell::new(Some({ let __cell___slot_p32_3_closure_clone_closure_clone = __cell___slot_p32_3_closure_clone.clone(); Rc::new(move |__slot_v33: Rc<RefCell<Option<LNode>>>| {
        { let new_val = __slot_v33.clone(); (*__cell___slot_p32_3_closure_clone_closure_clone.borrow_mut().as_mut().unwrap()).v = new_val; };
    }) as Rc<dyn Fn(Rc<RefCell<Option<LNode>>>) -> ()> }))), ..Default::default() })));
    }) as Rc<dyn Fn() -> Rc<RefCell<Option<__ptrSlot_LNode>>>>))); let __f: Rc<dyn Fn() -> Rc<RefCell<Option<__ptrSlot_LNode>>>> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)() };
    push(Rc::new(RefCell::new(Some({ let __arg_holder = pp.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))), Rc::new(RefCell::new(Some(9))));
    println!("{} {}", format!("{}", (*(*{ let __f_holder = (*pp.borrow().as_ref().unwrap()).get.clone(); let __f: Rc<dyn Fn() -> Rc<RefCell<Option<LNode>>>> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)() }.borrow().as_ref().unwrap()).v.borrow().as_ref().unwrap())), format!("{}", { let __nil_target = (*{ let __f_holder = (*pp.borrow().as_ref().unwrap()).get.clone(); let __f: Rc<dyn Fn() -> Rc<RefCell<Option<LNode>>>> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)() }.borrow().as_ref().unwrap()).next.clone(); let __nil_result = (*__nil_target.borrow()).is_none(); __nil_result }));

    let mut counts: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>> = Rc::new(RefCell::new(None));
    let mut __cell_counts_4 = Rc::new(RefCell::new(Some(AnonymousStruct3 { v: counts.clone() })));
    let __cell_counts_4_closure_clone = __cell_counts_4.clone(); add_count(Rc::new(RefCell::new(Some({ let __arg_value = __ptrSlot_map_string_int { get: Rc::new(RefCell::new(Some({ let __cell_counts_4_closure_clone_closure_clone = __cell_counts_4_closure_clone.clone(); Rc::new(move || -> Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>> {
        return (*__cell_counts_4_closure_clone_closure_clone.borrow().as_ref().unwrap()).v.clone();
    }) as Rc<dyn Fn() -> Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>> }))), set: Rc::new(RefCell::new(Some({ let __cell_counts_4_closure_clone_closure_clone = __cell_counts_4_closure_clone.clone(); Rc::new(move |__slot_v34: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>| {
        { let new_val = __slot_v34.clone(); (*__cell_counts_4_closure_clone_closure_clone.borrow_mut().as_mut().unwrap()).v = new_val; };
    }) as Rc<dyn Fn(Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>) -> ()> }))), ..Default::default() }; __arg_value }))), Rc::new(RefCell::new(Some("a".to_string()))));
    let __cell_counts_4_closure_clone = __cell_counts_4.clone(); add_count(Rc::new(RefCell::new(Some({ let __arg_value = __ptrSlot_map_string_int { get: Rc::new(RefCell::new(Some({ let __cell_counts_4_closure_clone_closure_clone = __cell_counts_4_closure_clone.clone(); Rc::new(move || -> Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>> {
        return (*__cell_counts_4_closure_clone_closure_clone.borrow().as_ref().unwrap()).v.clone();
    }) as Rc<dyn Fn() -> Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>> }))), set: Rc::new(RefCell::new(Some({ let __cell_counts_4_closure_clone_closure_clone = __cell_counts_4_closure_clone.clone(); Rc::new(move |__slot_v35: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>| {
        { let new_val = __slot_v35.clone(); (*__cell_counts_4_closure_clone_closure_clone.borrow_mut().as_mut().unwrap()).v = new_val; };
    }) as Rc<dyn Fn(Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>) -> ()> }))), ..Default::default() }; __arg_value }))), Rc::new(RefCell::new(Some("a".to_string()))));
    let __cell_counts_4_closure_clone = __cell_counts_4.clone(); add_count(Rc::new(RefCell::new(Some({ let __arg_value = __ptrSlot_map_string_int { get: Rc::new(RefCell::new(Some({ let __cell_counts_4_closure_clone_closure_clone = __cell_counts_4_closure_clone.clone(); Rc::new(move || -> Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>> {
        return (*__cell_counts_4_closure_clone_closure_clone.borrow().as_ref().unwrap()).v.clone();
    }) as Rc<dyn Fn() -> Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>> }))), set: Rc::new(RefCell::new(Some({ let __cell_counts_4_closure_clone_closure_clone = __cell_counts_4_closure_clone.clone(); Rc::new(move |__slot_v36: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>| {
        { let new_val = __slot_v36.clone(); (*__cell_counts_4_closure_clone_closure_clone.borrow_mut().as_mut().unwrap()).v = new_val; };
    }) as Rc<dyn Fn(Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>) -> ()> }))), ..Default::default() }; __arg_value }))), Rc::new(RefCell::new(Some("b".to_string()))));
    println!("{} {} {}", format!("{}", { let __map_holder = (*__cell_counts_4.borrow().as_ref().unwrap()).v.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", { let __map_holder = (*__cell_counts_4.borrow().as_ref().unwrap()).v.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"b".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", ({ let __len_target = { let __field = (*__cell_counts_4.borrow().as_ref().unwrap()).v.clone(); __field }; let __len_guard = __len_target.borrow(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })));
}

#[derive(Debug, Clone, Default)]
pub struct AnonymousStruct1 {
    pub v: Rc<RefCell<Option<Node>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.v.clone();
        Self {
            v: __go_clone_0_0,
        }
    }
}


impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.v.borrow(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone, Default)]
pub struct AnonymousStruct2 {
    pub v: Rc<RefCell<Option<LNode>>>,
}
impl AnonymousStruct2 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.v.clone();
        Self {
            v: __go_clone_0_0,
        }
    }
}


impl std::fmt::Display for AnonymousStruct2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.v.borrow(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone, Default)]
pub struct AnonymousStruct3 {
    pub v: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>,
}
impl AnonymousStruct3 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.v.clone();
        Self {
            v: __go_clone_0_0,
        }
    }
}


impl std::fmt::Display for AnonymousStruct3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_map(&self.v));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}
