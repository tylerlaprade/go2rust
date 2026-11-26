#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use std::any::Any;
use std::cell::{RefCell};
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

fn go_install_panic_exit() {
    std::panic::set_hook(Box::new(|__info| {
        let __text = go_panic_payload_text(__info.payload());
        {
            use std::io::Write;
            std::io::stdout().flush().ok();
        }
        eprintln!("panic: {}", __text);
        std::process::exit(2);
    }));
}

fn go_panic_payload_text(__payload: &dyn Any) -> String {
    if let Some(__text) = __payload.downcast_ref::<String>() {
        return go_runtime_error_text(__text.clone());
    }
    if let Some(__text) = __payload.downcast_ref::<&'static str>() {
        return go_runtime_error_text(__text.to_string());
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
pub struct Tree {
    pub left: Rc<RefCell<Option<Tree>>>,
    pub right: Rc<RefCell<Option<Tree>>>,
    pub val: Rc<RefCell<Option<i64>>>,
}

impl Tree {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.left.clone();
        let __go_clone_0_1 = self.right.clone();
        let __go_clone_1_0 = { let __guard = self.val.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            left: __go_clone_0_0,
            right: __go_clone_0_1,
            val: __go_clone_1_0,
        }
    }
}


impl Default for Tree {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(None));
        let __go_default_0_1 = Rc::new(RefCell::new(None));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            left: __go_default_0_0,
            right: __go_default_0_1,
            val: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.left.borrow(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", { let __guard = self.right.borrow(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_2 = format!("{}", (*self.val.borrow().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


impl Tree {
    pub fn all(&self, r#yield: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> bool>>>>) -> bool {
        if false {
        return true;
    }
        __go_nil_recv_tree_all({ let __field = self.left.clone(); __field }, Rc::new(RefCell::new(GoFuncValue::go_func_value(&(r#yield))))) && { let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> bool> = r#yield.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __selector_holder = self.val.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned })))) } && __go_nil_recv_tree_all({ let __field = self.right.clone(); __field }, Rc::new(RefCell::new(GoFuncValue::go_func_value(&(r#yield)))))
    }

    pub fn walk(&self) -> Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> bool>>>>) -> ()>>>> {
        let mut t_closure_clone = (*self).clone(); return Rc::new(RefCell::new(Some(Rc::new(move |r#yield: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> bool>>>>| {
        let mut t_closure_clone = t_closure_clone.clone();
        t_closure_clone.all(Rc::new(RefCell::new(GoFuncValue::go_func_value(&(r#yield)))));
    }) as Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> bool>>>>) -> ()>)));
    }
}

pub fn count(n: Rc<RefCell<Option<i64>>>) -> Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> bool>>>>) -> ()>>>> {
    let n_closure_clone = n.clone(); return Rc::new(RefCell::new(Some(Rc::new(move |r#yield: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> bool>>>>| {
        let mut i = Rc::new(RefCell::new(Some(0i64)));
    while (*i.borrow().as_ref().unwrap()) < (*n_closure_clone.borrow().as_ref().unwrap()) {
        if !{ let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> bool> = r#yield.borrow().as_ref().unwrap().clone(); (__f)(i.clone()) } {
        println!("{} {}", format!("{}", "stopped at".to_string()), format!("{}", { let __v = (*i.borrow().as_ref().unwrap()).clone(); __v }));
        return;
    }
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }) as Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> bool>>>>) -> ()>)));
}

pub fn find(target: Rc<RefCell<Option<i64>>>) -> (i64, bool) {
    let mut __rf_done_1: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));
    let mut __rf_result_1_0: Rc<RefCell<Option<[i64; 1]>>> = Rc::new(RefCell::new(Some(std::array::from_fn::<_, 1, _>(|_| 0))));
    let mut __rf_result_1_1: Rc<RefCell<Option<[bool; 1]>>> = Rc::new(RefCell::new(Some(std::array::from_fn::<_, 1, _>(|_| false))));
    let mut __rf_done_1_closure_clone = __rf_done_1.clone(); let __rf_result_1_0_closure_clone = __rf_result_1_0.clone(); let __rf_result_1_1_closure_clone = __rf_result_1_1.clone(); let target_closure_clone = target.clone(); { let __f_holder = count(Rc::new(RefCell::new(Some(10)))); let __f: Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> bool>>>>) -> ()> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(Rc::new(move |__rf_param_1_0: Rc<RefCell<Option<i64>>>| -> bool {
        let mut __rf_done_1_closure_clone = __rf_done_1_closure_clone.clone();
        let mut v = { let __owned = __rf_param_1_0.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
        if (*v.borrow().as_ref().unwrap()) == (*target_closure_clone.borrow().as_ref().unwrap()) {
        {
            { let __idx = (0) as usize; let __rhs = (*v.borrow().as_ref().unwrap()) * 100; (*__rf_result_1_0_closure_clone.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
            { let __idx = (0) as usize; let __rhs = true; (*__rf_result_1_1_closure_clone.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
            { let new_val = true; *__rf_done_1_closure_clone.borrow_mut() = Some(new_val); };
            return false;
        }
    }
        true
    }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> bool>)))) };
    if (*__rf_done_1.borrow().as_ref().unwrap()) {
        return ((*__rf_result_1_0.borrow().as_ref().unwrap())[(0) as usize].clone(), (*__rf_result_1_1.borrow().as_ref().unwrap())[(0) as usize].clone());
    }
    (-(1), false)
}

pub fn named() -> Rc<RefCell<Option<String>>> {
    let mut out: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(Some(String::new())));

    let mut __rf_done_2: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));
    let mut __rf_result_2_0: Rc<RefCell<Option<[String; 1]>>> = Rc::new(RefCell::new(Some(std::array::from_fn::<_, 1, _>(|_| String::new()))));
    let mut __rf_done_2_closure_clone = __rf_done_2.clone(); let __rf_result_2_0_closure_clone = __rf_result_2_0.clone(); let mut out_closure_clone = out.clone(); { let __f_holder = count(Rc::new(RefCell::new(Some(5)))); let __f: Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> bool>>>>) -> ()> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(Rc::new(move |__rf_param_2_0: Rc<RefCell<Option<i64>>>| -> bool {
        let mut __rf_done_2_closure_clone = __rf_done_2_closure_clone.clone();
        let mut out_closure_clone = out_closure_clone.clone();
        let mut v = { let __owned = __rf_param_2_0.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
        { (*out_closure_clone.borrow_mut().as_mut().unwrap()).push_str(&{ let __s = Rc::new(RefCell::new(Some(format!("{}", format!("{}", { let __v = (*v.borrow().as_ref().unwrap()).clone(); __v }))))); let __value = (*__s.borrow().as_ref().unwrap()).clone(); __value }); };
        if (*v.borrow().as_ref().unwrap()) == 2 {
        {
            { let __idx = (0) as usize; let __rhs = (*out_closure_clone.borrow().as_ref().unwrap()).clone(); (*__rf_result_2_0_closure_clone.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
            { let new_val = true; *__rf_done_2_closure_clone.borrow_mut() = Some(new_val); };
            return false;
        }
    }
        true
    }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> bool>)))) };
    if (*__rf_done_2.borrow().as_ref().unwrap()) {
        return Rc::new(RefCell::new(Some((*__rf_result_2_0.borrow().as_ref().unwrap())[(0) as usize].clone())));
    }
    Rc::new(RefCell::new(Some("never".to_string())))
}

fn main() {
    go_install_panic_exit();
    let (mut __go_mv_1_0, mut __go_mv_1_1) = find(Rc::new(RefCell::new(Some(3))));
    println!("{} {}", format!("{}", __go_mv_1_0), format!("{}", __go_mv_1_1));
    let (mut __go_mv_2_0, mut __go_mv_2_1) = find(Rc::new(RefCell::new(Some(42))));
    println!("{} {}", format!("{}", __go_mv_2_0), format!("{}", __go_mv_2_1));
    println!("{}", format!("{}", (*named().borrow().as_ref().unwrap())));
    let mut total = Rc::new(RefCell::new(Some(0i64)));
    let mut __rf_done_3: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));
    let mut total_closure_clone = total.clone(); { let __f_holder = count(Rc::new(RefCell::new(Some(6)))); let __f: Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> bool>>>>) -> ()> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(Rc::new(move |__rf_param_3_0: Rc<RefCell<Option<i64>>>| -> bool {
        let mut total_closure_clone = total_closure_clone.clone();
        let mut v = { let __owned = __rf_param_3_0.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
        if (*v.borrow().as_ref().unwrap()) % 2 == 1 {
        return true;
    }
        { let _switch_val = (*v.borrow().as_ref().unwrap());
    if _switch_val == (4) {
        } else {
            { let __rhs = (*v.borrow().as_ref().unwrap()); let mut guard = total_closure_clone.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        }
    }
        let mut j = Rc::new(RefCell::new(Some(0i64)));
    while (*j.borrow().as_ref().unwrap()) < 3 {
        if (*j.borrow().as_ref().unwrap()) == 1 {
        break
    }
        { let __rhs = 10; let mut guard = total_closure_clone.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        true
    }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> bool>)))) };
    if (*__rf_done_3.borrow().as_ref().unwrap()) {
        return;
    }
    println!("{}", format!("{}", { let __v = (*total.borrow().as_ref().unwrap()).clone(); __v }));
    let mut t = Rc::new(RefCell::new(Some(Tree { val: Rc::new(RefCell::new(Some(2))), left: Rc::new(RefCell::new(Some(Tree { val: Rc::new(RefCell::new(Some(1))), ..Default::default() }))).clone(), right: Rc::new(RefCell::new(Some(Tree { val: Rc::new(RefCell::new(Some(3))), ..Default::default() }))).clone(), ..Default::default() })));
    let mut vals: Rc<RefCell<Option<Vec<i64>>>> = Rc::new(RefCell::new(None));
    let mut __rf_done_4: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));
    let mut vals_closure_clone = vals.clone(); { let __f_holder = (*t.borrow().as_ref().unwrap()).walk(); let __f: Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> bool>>>>) -> ()> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(Rc::new(move |__rf_param_4_0: Rc<RefCell<Option<i64>>>| -> bool {
        let mut vals_closure_clone = vals_closure_clone.clone();
        let mut v = { let __owned = __rf_param_4_0.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = vals_closure_clone.clone(); let __append_value = (*v.borrow().as_ref().unwrap()).clone(); let mut __appended: Vec<i64> = (*__append_target.borrow()).clone().unwrap_or_default(); __appended.push(__append_value); Rc::new(RefCell::new(Some(__appended))) })); *vals_closure_clone.borrow_mut() = __slice_header; };
        true
    }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> bool>)))) };
    if (*__rf_done_4.borrow().as_ref().unwrap()) {
        return;
    }
    println!("{}", format!("{}", format_slice(&vals)));
    let mut __rf_done_5: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));
    { let __f_holder = count(Rc::new(RefCell::new(Some(2)))); let __f: Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> bool>>>>) -> ()> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(Rc::new(move |_: Rc<RefCell<Option<i64>>>| -> bool {
        println!("{}", format!("{}", "tick".to_string()));
        true
    }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> bool>)))) };
    if (*__rf_done_5.borrow().as_ref().unwrap()) {
        return;
    }
}

pub fn __go_nil_recv_tree_all(t: Rc<RefCell<Option<Tree>>>, r#yield: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> bool>>>>) -> bool {
    if { let __nil_result = (*t.borrow()).is_none(); __nil_result } {
        return true;
    }
    __go_nil_recv_tree_all({ let __field = (*t.borrow().as_ref().unwrap()).left.clone(); __field }, Rc::new(RefCell::new(GoFuncValue::go_func_value(&(r#yield))))) && { let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> bool> = r#yield.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __selector_holder = (*t.borrow().as_ref().unwrap()).val.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned })))) } && __go_nil_recv_tree_all({ let __field = (*t.borrow().as_ref().unwrap()).right.clone(); __field }, Rc::new(RefCell::new(GoFuncValue::go_func_value(&(r#yield)))))
}