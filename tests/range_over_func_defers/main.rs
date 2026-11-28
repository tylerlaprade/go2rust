#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display};
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

fn format_any(value: &dyn Any) -> String {
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

thread_local! {
    static __GO_RECOVER_PAYLOAD: RefCell<Option<Box<dyn Any + Send>>> = RefCell::new(None);
}

fn go_recover() -> Rc<RefCell<Option<Box<dyn Any>>>> {
    __GO_RECOVER_PAYLOAD.with(|slot| Rc::new(RefCell::new(slot.borrow_mut().take().map(|__payload| __payload as Box<dyn Any>))))
}

fn go_store_panic_payload(mut __payload: Box<dyn Any + Send>) {
    // A payload raised again by go_resume_unrecovered_panic arrives boxed
    // once more.
    loop {
        match __payload.downcast::<Box<dyn Any + Send>>() {
            Ok(__inner) => __payload = *__inner,
            Err(__outer) => {
                __payload = __outer;
                break;
            }
        }
    }
    let __payload: Box<dyn Any + Send> = match __payload.downcast::<&'static str>() {
        Ok(__text) => Box::new(go_runtime_error_text(__text.to_string())),
        Err(__payload) => match __payload.downcast::<String>() {
            Ok(__text) => Box::new(go_runtime_error_text_of_string(*__text)),
            Err(__payload) => __payload,
        },
    };
    __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(__payload));
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
    if let Some(__payload) = __GO_RECOVER_PAYLOAD.with(|slot| slot.borrow_mut().take()) {
        match __payload.downcast::<String>() {
            Ok(__text) => std::panic::panic_any(*__text),
            // panic_any, not resume_unwind: the panic hook must see a panic
            // that nothing recovers, to end the program as Go does.
            Err(__payload) => std::panic::panic_any(__payload),
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

pub fn seq3(r#yield: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> bool>>>>) {
    let mut i = Rc::new(RefCell::new(Some(0i64)));
    while (*i.borrow().as_ref().unwrap()) < 3 {
        if !{ let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> bool> = r#yield.borrow().as_ref().unwrap().clone(); (__f)(i.clone()) } {
        return;
    }
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

pub fn pairs(r#yield: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<i64>>>) -> bool>>>>) {
    let _ = { let __f: Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<i64>>>) -> bool> = r#yield.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some("a".to_string()))), Rc::new(RefCell::new(Some(1)))) } && { let __f: Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<i64>>>) -> bool> = r#yield.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some("b".to_string()))), Rc::new(RefCell::new(Some(2)))) };
}

pub fn f() {
    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    let __go_previous_panic_hook = go_silence_panic_hook();
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        __defer_stack.push(Box::new(move || {
        println!("{}", format!("{}", "first defer".to_string()));
    }));
        let mut __rf_done_1: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));
        let mut __rf_defers_1: Rc<RefCell<Option<[Vec<Rc<RefCell<Option<Rc<dyn Fn() -> ()>>>>>; 1]>>> = Rc::new(RefCell::new(Some(std::array::from_fn::<_, 1, _>(|_| Vec::nil()))));
        let __rf_defers_1_defer_captured = __rf_defers_1.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Rc::new(move || {
        let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();
        let __go_previous_panic_hook = go_silence_panic_hook();
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        for __rf_f_1 in (*__rf_defers_1_defer_captured.borrow().as_ref().unwrap())[(0) as usize].clone().iter().cloned() {
        let __rf_f_1_defer_captured = __rf_f_1.clone(); __defer_stack.push(Box::new(move || {
        { let __f: Rc<dyn Fn() -> ()> = __rf_f_1_defer_captured.borrow().as_ref().unwrap().clone(); (__f)() };
    }));
    }
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
    }) as Rc<dyn Fn() -> ()>))); let __f: Rc<dyn Fn() -> ()> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)() };
    }));
        let __rf_defers_1_closure_clone = __rf_defers_1.clone(); seq3(Rc::new(RefCell::new(Some(Rc::new(move |__rf_param_1_0: Rc<RefCell<Option<i64>>>| -> bool {
        let mut i = { let __owned = __rf_param_1_0.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
        let mut __rf_arg_1_1_1 = { let __owned = i.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
        let __rf_arg_1_1_1_closure_clone = __rf_arg_1_1_1.clone(); let mut __rf_call_1_1 = Rc::new(RefCell::new(Some(Rc::new(move || {
        println!("{} {}", format!("{}", "deferred".to_string()), format!("{}", { let __v = (*__rf_arg_1_1_1_closure_clone.borrow().as_ref().unwrap()).clone(); __v }));
    }) as Rc<dyn Fn() -> ()>)));
        { let __idx = (0) as usize; let __rhs = { let __append_idx = (0) as usize; let __append_value = Rc::new(RefCell::new({ let __func_value = (*__rf_call_1_1.borrow()).clone(); __func_value })); let mut __appended = (*__rf_defers_1_closure_clone.borrow().as_ref().unwrap())[__append_idx].clone(); __appended.push(__append_value); __appended }; (*__rf_defers_1_closure_clone.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
        if (*i.borrow().as_ref().unwrap()) == 1 {
        let i_closure_clone = i.clone(); let mut __rf_call_1_2 = Rc::new(RefCell::new(Some(Rc::new(move || {
        let i_closure_clone_closure_clone = i_closure_clone.clone(); { let __f_holder = Rc::new(RefCell::new(Some(Rc::new(move || {
        println!("{} {}", format!("{}", "closure at".to_string()), format!("{}", { let __v = (*i_closure_clone_closure_clone.borrow().as_ref().unwrap()).clone(); __v }));
    }) as Rc<dyn Fn() -> ()>))); let __f: Rc<dyn Fn() -> ()> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)() };
    }) as Rc<dyn Fn() -> ()>)));
        { let __idx = (0) as usize; let __rhs = { let __append_idx = (0) as usize; let __append_value = Rc::new(RefCell::new({ let __func_value = (*__rf_call_1_2.borrow()).clone(); __func_value })); let mut __appended = (*__rf_defers_1_closure_clone.borrow().as_ref().unwrap())[__append_idx].clone(); __appended.push(__append_value); __appended }; (*__rf_defers_1_closure_clone.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    }
        true
    }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> bool>))));
        if (*__rf_done_1.borrow().as_ref().unwrap()) {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
        __defer_stack.push(Box::new(move || {
        println!("{}", format!("{}", "last defer".to_string()));
    }));
        println!("{}", format!("{}", "after loop".to_string()));

        // Execute deferred functions
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
}

pub fn g() -> i64 {
    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    let mut n: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));

    let __go_previous_panic_hook = go_silence_panic_hook();
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut __rf_done_2: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));
        let mut __rf_result_2_0: Rc<RefCell<Option<[i64; 1]>>> = Rc::new(RefCell::new(Some(std::array::from_fn::<_, 1, _>(|_| 0))));
        let mut __rf_defers_2: Rc<RefCell<Option<[Vec<Rc<RefCell<Option<Rc<dyn Fn() -> ()>>>>>; 1]>>> = Rc::new(RefCell::new(Some(std::array::from_fn::<_, 1, _>(|_| Vec::nil()))));
        let __rf_defers_2_defer_captured = __rf_defers_2.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Rc::new(move || {
        let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();
        let __go_previous_panic_hook = go_silence_panic_hook();
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        for __rf_f_2 in (*__rf_defers_2_defer_captured.borrow().as_ref().unwrap())[(0) as usize].clone().iter().cloned() {
        let __rf_f_2_defer_captured = __rf_f_2.clone(); __defer_stack.push(Box::new(move || {
        { let __f: Rc<dyn Fn() -> ()> = __rf_f_2_defer_captured.borrow().as_ref().unwrap().clone(); (__f)() };
    }));
    }
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
    }) as Rc<dyn Fn() -> ()>))); let __f: Rc<dyn Fn() -> ()> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)() };
    }));
        let __rf_defers_2_closure_clone = __rf_defers_2.clone(); let mut __rf_done_2_closure_clone = __rf_done_2.clone(); let __rf_result_2_0_closure_clone = __rf_result_2_0.clone(); let n_closure_clone = n.clone(); pairs(Rc::new(RefCell::new(Some(Rc::new(move |__rf_param_2_0: Rc<RefCell<Option<String>>>, __rf_param_2_1: Rc<RefCell<Option<i64>>>| -> bool {
        let mut __rf_done_2_closure_clone = __rf_done_2_closure_clone.clone();
        let (mut k, mut v) = ({ let __owned = __rf_param_2_0.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) }, { let __owned = __rf_param_2_1.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) });
        let mut __rf_arg_2_1_0 = { let __owned = k.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
        let __rf_arg_2_1_0_closure_clone = __rf_arg_2_1_0.clone(); let n_closure_clone_closure_clone = n_closure_clone.clone(); let mut __rf_call_2_1 = Rc::new(RefCell::new(Some(Rc::new(move || {
        let n_closure_clone_closure_clone_closure_clone = n_closure_clone_closure_clone.clone(); { let __f_holder = Rc::new(RefCell::new(Some(Rc::new(move |s: Rc<RefCell<Option<String>>>| {
        println!("{} {} {}", format!("{}", "pair".to_string()), format!("{}", { let __v = (*s.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*n_closure_clone_closure_clone_closure_clone.borrow().as_ref().unwrap()).clone(); __v }));
    }) as Rc<dyn Fn(Rc<RefCell<Option<String>>>) -> ()>))); let __f: Rc<dyn Fn(Rc<RefCell<Option<String>>>) -> ()> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_holder = __rf_arg_2_1_0_closure_clone.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })))) };
    }) as Rc<dyn Fn() -> ()>)));
        { let __idx = (0) as usize; let __rhs = { let __append_idx = (0) as usize; let __append_value = Rc::new(RefCell::new({ let __func_value = (*__rf_call_2_1.borrow()).clone(); __func_value })); let mut __appended = (*__rf_defers_2_closure_clone.borrow().as_ref().unwrap())[__append_idx].clone(); __appended.push(__append_value); __appended }; (*__rf_defers_2_closure_clone.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
        let mut __rf_done_3: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));
        let mut __rf_result_3_0: Rc<RefCell<Option<[i64; 1]>>> = Rc::new(RefCell::new(Some(std::array::from_fn::<_, 1, _>(|_| 0))));
        let __rf_defers_2_closure_clone_closure_clone = __rf_defers_2_closure_clone.clone(); let k_closure_clone = k.clone(); seq3(Rc::new(RefCell::new(Some(Rc::new(move |__rf_param_3_0: Rc<RefCell<Option<i64>>>| -> bool {
        let mut i = { let __owned = __rf_param_3_0.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
        if (*i.borrow().as_ref().unwrap()) == 2 {
        let mut __rf_arg_2_2_1 = { let __owned = k_closure_clone.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
        let mut __rf_arg_2_2_2 = { let __owned = i.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
        let __rf_arg_2_2_1_closure_clone = __rf_arg_2_2_1.clone(); let __rf_arg_2_2_2_closure_clone = __rf_arg_2_2_2.clone(); let mut __rf_call_2_2 = Rc::new(RefCell::new(Some(Rc::new(move || {
        println!("{} {} {}", format!("{}", "inner".to_string()), format!("{}", { let __v = (*__rf_arg_2_2_1_closure_clone.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*__rf_arg_2_2_2_closure_clone.borrow().as_ref().unwrap()).clone(); __v }));
    }) as Rc<dyn Fn() -> ()>)));
        { let __idx = (0) as usize; let __rhs = { let __append_idx = (0) as usize; let __append_value = Rc::new(RefCell::new({ let __func_value = (*__rf_call_2_2.borrow()).clone(); __func_value })); let mut __appended = (*__rf_defers_2_closure_clone_closure_clone.borrow().as_ref().unwrap())[__append_idx].clone(); __appended.push(__append_value); __appended }; (*__rf_defers_2_closure_clone_closure_clone.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    }
        true
    }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> bool>))));
        if (*__rf_done_3.borrow().as_ref().unwrap()) {
        {
            { let __idx = (0) as usize; let __rhs = (*__rf_result_3_0.borrow().as_ref().unwrap())[(0) as usize].clone(); (*__rf_result_2_0_closure_clone.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
            { let new_val = true; *__rf_done_2_closure_clone.borrow_mut() = Some(new_val); };
            return false;
        }
    }
        if (*v.borrow().as_ref().unwrap()) == 2 {
        {
            { let __idx = (0) as usize; let __rhs = (*v.borrow().as_ref().unwrap()) * 10; (*__rf_result_2_0_closure_clone.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
            { let new_val = true; *__rf_done_2_closure_clone.borrow_mut() = Some(new_val); };
            return false;
        }
    }
        true
    }) as Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<i64>>>) -> bool>))));
        if (*__rf_done_2.borrow().as_ref().unwrap()) {
        {
        { let new_val = (*__rf_result_2_0.borrow().as_ref().unwrap())[(0) as usize].clone(); *n.borrow_mut() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*n.borrow().as_ref().unwrap());
    }
    }
        {
        { let new_val = -1; *n.borrow_mut() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*n.borrow().as_ref().unwrap());
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
            (*n.borrow().as_ref().unwrap())
        }
    }
}

pub fn pair() -> (Rc<RefCell<Option<String>>>, i64) {
    (Rc::new(RefCell::new(Some("p".to_string()))), 7)
}

pub fn record(s: Rc<RefCell<Option<String>>>, n: Rc<RefCell<Option<i64>>>) {
    println!("{} {} {}", format!("{}", "record".to_string()), format!("{}", { let __v = (*s.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*n.borrow().as_ref().unwrap()).clone(); __v }));
}

pub fn show(prefix: Rc<RefCell<Option<String>>>, xs: Rc<RefCell<Option<Vec<i64>>>>) {
    println!("{} {}", format!("{}", { let __v = (*prefix.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", format_slice(&xs)));
}

pub fn picker(name: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> ()>>>> {
    println!("{} {}", format!("{}", "pick".to_string()), format!("{}", { let __v = (*name.borrow().as_ref().unwrap()).clone(); __v }));
    let name_closure_clone = name.clone(); return Rc::new(RefCell::new(Some(Rc::new(move |i: Rc<RefCell<Option<i64>>>| {
        println!("{} {} {}", format!("{}", "picked".to_string()), format!("{}", { let __v = (*name_closure_clone.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*i.borrow().as_ref().unwrap()).clone(); __v }));
    }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> ()>)));
}

/// h defers calls whose function and arguments are evaluated at the defer:
/// a call with two results as the arguments, a slice passed on with ...,
/// and a function that a call returns. A deferred panic does not keep the
/// calls deferred before it from running.
pub fn h() {
    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    let __go_previous_panic_hook = go_silence_panic_hook();
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Rc::new(move || {
        println!("{} {}", format!("{}", "recovered".to_string()), format!("{}", match (go_recover()).borrow().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }));
    }) as Rc<dyn Fn() -> ()>))); let __f: Rc<dyn Fn() -> ()> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)() };
    }));
        let mut act = Rc::new(RefCell::new(Some(Rc::new(move |i: Rc<RefCell<Option<i64>>>| {
        println!("{} {}", format!("{}", "act".to_string()), format!("{}", { let __v = (*i.borrow().as_ref().unwrap()).clone(); __v }));
    }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> ()>)));
        let mut xs = Rc::new(RefCell::new(Some(vec![1i64, 2i64])));
        let mut __rf_done_4: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));
        let mut __rf_defers_4: Rc<RefCell<Option<[Vec<Rc<RefCell<Option<Rc<dyn Fn() -> ()>>>>>; 1]>>> = Rc::new(RefCell::new(Some(std::array::from_fn::<_, 1, _>(|_| Vec::nil()))));
        let __rf_defers_4_defer_captured = __rf_defers_4.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Rc::new(move || {
        let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();
        let __go_previous_panic_hook = go_silence_panic_hook();
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        for __rf_f_4 in (*__rf_defers_4_defer_captured.borrow().as_ref().unwrap())[(0) as usize].clone().iter().cloned() {
        let __rf_f_4_defer_captured = __rf_f_4.clone(); __defer_stack.push(Box::new(move || {
        { let __f: Rc<dyn Fn() -> ()> = __rf_f_4_defer_captured.borrow().as_ref().unwrap().clone(); (__f)() };
    }));
    }
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
    }) as Rc<dyn Fn() -> ()>))); let __f: Rc<dyn Fn() -> ()> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)() };
    }));
        let __rf_defers_4_closure_clone = __rf_defers_4.clone(); let act_closure_clone = act.clone(); let xs_closure_clone = xs.clone(); seq3(Rc::new(RefCell::new(Some(Rc::new(move |__rf_param_4_0: Rc<RefCell<Option<i64>>>| -> bool {
        let mut i = { let __owned = __rf_param_4_0.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
        let (mut __rf_arg_4_1_0_0, mut __rf_arg_4_1_0_1) = pair();
        let __rf_arg_4_1_0_0_closure_clone = __rf_arg_4_1_0_0.clone(); let __rf_arg_4_1_0_1_closure_clone = __rf_arg_4_1_0_1.clone(); let mut __rf_call_4_1 = Rc::new(RefCell::new(Some(Rc::new(move || {
        record(Rc::new(RefCell::new(Some({ let __arg_holder = __rf_arg_4_1_0_0_closure_clone.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some(__rf_arg_4_1_0_1_closure_clone))));
    }) as Rc<dyn Fn() -> ()>)));
        { let __idx = (0) as usize; let __rhs = { let __append_idx = (0) as usize; let __append_value = Rc::new(RefCell::new({ let __func_value = (*__rf_call_4_1.borrow()).clone(); __func_value })); let mut __appended = (*__rf_defers_4_closure_clone.borrow().as_ref().unwrap())[__append_idx].clone(); __appended.push(__append_value); __appended }; (*__rf_defers_4_closure_clone.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
        let mut __rf_arg_4_2_1 = Rc::new(RefCell::new(xs_closure_clone.borrow().clone()));
        let __rf_arg_4_2_1_closure_clone = __rf_arg_4_2_1.clone(); let mut __rf_call_4_2 = Rc::new(RefCell::new(Some(Rc::new(move || {
        show(Rc::new(RefCell::new(Some("xs".to_string()))), __rf_arg_4_2_1.clone());
    }) as Rc<dyn Fn() -> ()>)));
        { let __idx = (0) as usize; let __rhs = { let __append_idx = (0) as usize; let __append_value = Rc::new(RefCell::new({ let __func_value = (*__rf_call_4_2.borrow()).clone(); __func_value })); let mut __appended = (*__rf_defers_4_closure_clone.borrow().as_ref().unwrap())[__append_idx].clone(); __appended.push(__append_value); __appended }; (*__rf_defers_4_closure_clone.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
        let mut __rf_fn_4_3 = Rc::new(RefCell::new({ let __func_value = (*act_closure_clone.borrow()).clone(); __func_value }));
        let mut __rf_arg_4_3_0 = { let __owned = i.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
        let __rf_arg_4_3_0_closure_clone = __rf_arg_4_3_0.clone(); let __rf_fn_4_3_closure_clone = __rf_fn_4_3.clone(); let mut __rf_call_4_3 = Rc::new(RefCell::new(Some(Rc::new(move || {
        { let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> ()> = __rf_fn_4_3_closure_clone.borrow().as_ref().unwrap().clone(); (__f)(__rf_arg_4_3_0_closure_clone.clone()) };
    }) as Rc<dyn Fn() -> ()>)));
        { let __idx = (0) as usize; let __rhs = { let __append_idx = (0) as usize; let __append_value = Rc::new(RefCell::new({ let __func_value = (*__rf_call_4_3.borrow()).clone(); __func_value })); let mut __appended = (*__rf_defers_4_closure_clone.borrow().as_ref().unwrap())[__append_idx].clone(); __appended.push(__append_value); __appended }; (*__rf_defers_4_closure_clone.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
        let mut __rf_fn_4_4 = picker(Rc::new(RefCell::new(Some("at".to_string()))));
let mut __rf_fn_4_4 = Rc::new(RefCell::new(GoFuncValue::go_func_value(&__rf_fn_4_4)));
        let mut __rf_arg_4_4_0 = { let __owned = i.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
        let __rf_arg_4_4_0_closure_clone = __rf_arg_4_4_0.clone(); let __rf_fn_4_4_closure_clone = __rf_fn_4_4.clone(); let mut __rf_call_4_4 = Rc::new(RefCell::new(Some(Rc::new(move || {
        { let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> ()> = __rf_fn_4_4_closure_clone.borrow().as_ref().unwrap().clone(); (__f)(__rf_arg_4_4_0_closure_clone.clone()) };
    }) as Rc<dyn Fn() -> ()>)));
        { let __idx = (0) as usize; let __rhs = { let __append_idx = (0) as usize; let __append_value = Rc::new(RefCell::new({ let __func_value = (*__rf_call_4_4.borrow()).clone(); __func_value })); let mut __appended = (*__rf_defers_4_closure_clone.borrow().as_ref().unwrap())[__append_idx].clone(); __appended.push(__append_value); __appended }; (*__rf_defers_4_closure_clone.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
        if (*i.borrow().as_ref().unwrap()) == 0 {
        let mut __rf_call_4_5 = Rc::new(RefCell::new(Some(Rc::new(move || {
        println!("{}", format!("{}", "cleanup".to_string()));
    }) as Rc<dyn Fn() -> ()>)));
        { let __idx = (0) as usize; let __rhs = { let __append_idx = (0) as usize; let __append_value = Rc::new(RefCell::new({ let __func_value = (*__rf_call_4_5.borrow()).clone(); __func_value })); let mut __appended = (*__rf_defers_4_closure_clone.borrow().as_ref().unwrap())[__append_idx].clone(); __appended.push(__append_value); __appended }; (*__rf_defers_4_closure_clone.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    }
        if (*i.borrow().as_ref().unwrap()) == 2 {
        let mut __rf_call_4_6 = Rc::new(RefCell::new(Some(Rc::new(move || -> () {
        std::panic::panic_any(format!("boom"));
    }) as Rc<dyn Fn() -> ()>)));
        { let __idx = (0) as usize; let __rhs = { let __append_idx = (0) as usize; let __append_value = Rc::new(RefCell::new({ let __func_value = (*__rf_call_4_6.borrow()).clone(); __func_value })); let mut __appended = (*__rf_defers_4_closure_clone.borrow().as_ref().unwrap())[__append_idx].clone(); __appended.push(__append_value); __appended }; (*__rf_defers_4_closure_clone.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    }
        true
    }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> bool>))));
        if (*__rf_done_4.borrow().as_ref().unwrap()) {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
        { let __idx = (0) as usize; let __rhs = 9; (*xs.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
        println!("{}", format!("{}", "h after loop".to_string()));

        // Execute deferred functions
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
}

fn main() {
    go_install_panic_exit();
    h();
    f();
    println!("{} {}", format!("{}", "g:".to_string()), format!("{}", g()));
}