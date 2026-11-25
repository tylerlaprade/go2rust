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
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Celsius>>>>() {
        match (*v.lock().unwrap()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() }
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Counter>>>>() {
        match (*v.lock().unwrap()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() }
    } else if let Some(v) = value.downcast_ref::<Celsius>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<Counter>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<GoNilAny>() {
        "<nil>".to_string()
    } else if let Some(v) = value.downcast_ref::<Plain>() {
        format!("{}", v)
    } else {
        panic!("go2rust: cannot print this value held in an interface: its type has no formatting case yet")
    }
}

/// A nil interface value in a slot that holds a Box<dyn Any>, such as a
/// []any element: it copies, prints and compares as nil.
#[derive(Clone, Copy, Debug, PartialEq)]
struct GoNilAny;

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
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Celsius>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Counter>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Celsius>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Counter>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<GoNilAny>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Plain>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
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
    if val.is::<Arc<Mutex<Option<Celsius>>>>() { return "*main.Celsius" }
    if val.is::<Arc<Mutex<Option<Counter>>>>() { return "*main.Counter" }
    if val.is::<Celsius>() { return "main.Celsius" }
    if val.is::<Counter>() { return "main.Counter" }
    if val.is::<GoNilAny>() { return "<nil>" }
    if val.is::<Plain>() { return "main.Plain" }
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

fn __go_next_external_interface_id() -> usize {
    static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}



#[derive(Clone)]
pub struct fmt_Stringer {
    pub __go_id: usize,
    pub __go_value: Arc<dyn std::any::Any + Send + Sync>,
    /// The text of the value's String method.
    pub __go_text: Option<Arc<dyn Fn() -> String + Send + Sync>>,
}

impl fmt_Stringer {
    pub fn __go_from<T: 'static + Send + Sync>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_value: Arc::new(value), __go_text: None }
    }
    /// A value whose type has a String method, which its Display impl calls.
    pub fn __go_from_display<T: std::fmt::Display + Send + Sync + 'static>(value: T) -> Self {
        let shared = Arc::new(value);
        let text = shared.clone();
        Self { __go_id: __go_next_external_interface_id(), __go_value: shared, __go_text: Some(Arc::new(move || text.to_string())) }
    }
    /// A pointer whose element type has a String method.
    pub fn __go_from_display_cell<T: std::fmt::Display + Send + Sync + 'static>(cell: Arc<Mutex<Option<T>>>) -> Self {
        let text = cell.clone();
        Self { __go_id: __go_next_external_interface_id(), __go_value: Arc::new(cell), __go_text: Some(Arc::new(move || text.lock().unwrap().as_ref().map(|__v| __v.to_string()).unwrap_or_default())) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
    fn __go_string(&self) -> String {
        match &self.__go_text {
            Some(text) => text(),
            None => panic!("fmt.Stringer value of a type with no String method"),
        }
    }
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some::<String>(self.__go_string())))
    }
}

impl Default for fmt_Stringer {
    fn default() -> Self {
        Self { __go_id: 0, __go_value: Arc::new(()), __go_text: None }
    }
}

impl std::fmt::Debug for fmt_Stringer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<fmt_Stringer>")
    }
}

impl std::fmt::Display for fmt_Stringer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.__go_text {
            Some(text) => write!(f, "{}", text()),
            None => write!(f, "<fmt_Stringer>"),
        }
    }
}

impl PartialEq for fmt_Stringer {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for fmt_Stringer {}

impl PartialOrd for fmt_Stringer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ord::cmp(self, other))
    }
}

impl Ord for fmt_Stringer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
    }
}


#[derive(Debug, Clone)]
pub struct Celsius {
    pub deg: Arc<Mutex<Option<i64>>>,
}

impl Celsius {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.deg.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            deg: __go_clone_0_0,
        }
    }
}


impl Default for Celsius {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            deg: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Celsius {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct Counter {
    pub n: Arc<Mutex<Option<i64>>>,
}

impl Counter {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            n: __go_clone_0_0,
        }
    }
}


impl Default for Counter {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            n: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct Plain {
    pub x: Arc<Mutex<Option<i64>>>,
}

impl Plain {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
        }
    }
}


impl Default for Plain {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            x: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Plain {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Celsius {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(format!("{}°C", (*self.deg.lock().unwrap().as_ref().unwrap())))))
    }
}

impl Counter {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(format!("Counter<{}>", (*self.n.lock().unwrap().as_ref().unwrap())))))
    }
}

pub fn make_celsius(d: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<Celsius>>> {
    Arc::new(Mutex::new(Some(Celsius { deg: Arc::new(Mutex::new(Some({ let __arg_holder = d.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })))
}

/// Stringer returns its argument as a fmt.Stringer.
pub fn stringer(v: Arc<Mutex<Option<fmt_Stringer>>>) -> Arc<Mutex<Option<fmt_Stringer>>> {
    return v.clone();
}

pub fn join(items: Arc<Mutex<Option<Vec<fmt_Stringer>>>>) -> Arc<Mutex<Option<String>>> {
    let mut out = Arc::new(Mutex::new(Some("".to_string())));
    { let __range_holder = items.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for (i, it) in __range_values.iter().enumerate() {
        if { let __tmp_x = i as i64; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { (*out.lock().unwrap().as_mut().unwrap()).push_str(&", ".to_string()); };
    }
        { (*out.lock().unwrap().as_mut().unwrap()).push_str(&{ let __s = it.string(); let __value = (*__s.lock().unwrap().as_ref().unwrap()).clone(); __value }); };
    } }
    return { let __owned = out.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}

pub fn show(s: Arc<Mutex<Option<fmt_Stringer>>>) {
    println!("{} {} {}", format!("{}", "show:".to_string()), format!("{}", (*(*s.lock().unwrap().as_ref().unwrap()).string().lock().unwrap().as_ref().unwrap())), format!("{}", match s.lock().unwrap().as_ref() { Some(__iface) => format!("{}", __iface), None => "<nil>".to_string() }));
}

pub fn describe(v: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
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
    if _ts_val.and_then(|__v| __v.downcast_ref::<fmt_Stringer>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<Celsius>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<Celsius>>>>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<Counter>>>>()).is_some() {
        let x: Arc<Mutex<Option<fmt_Stringer>>> = if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<fmt_Stringer>()) { Arc::new(Mutex::new(Some(typed_val.clone()))) } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<Celsius>()) { Arc::new(Mutex::new(Some(fmt_Stringer::__go_from_display(typed_val.clone())))) } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<Celsius>>>>()) { Arc::new(Mutex::new(Some(fmt_Stringer::__go_from_display_cell(typed_val.clone())))) } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<Counter>>>>()) { Arc::new(Mutex::new(Some(fmt_Stringer::__go_from_display_cell(typed_val.clone())))) } else { Arc::new(Mutex::new(None)) };
        std::mem::drop(_ts_guard);
        return Arc::new(Mutex::new(Some(format!("{}{}", "stringer ".to_string(), (*(*x.lock().unwrap().as_ref().unwrap()).string().lock().unwrap().as_ref().unwrap())))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<i64>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<i64>()).unwrap().clone())));
        std::mem::drop(_ts_guard);
        return Arc::new(Mutex::new(Some(format!("int {}", { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }))));;
    } else {
        let x = _ts_subject.clone();
        std::mem::drop(_ts_guard);
        return Arc::new(Mutex::new(Some(format!("other {}", { let __type_name = match x.lock().unwrap().as_ref() { Some(__any_value) => __go_type_name(&**__any_value), None => "<nil>" }; __type_name }))));;
    }
    }
    unreachable!()
}

pub fn label(v: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
    {
        let (mut s, mut ok) = ({
        let val = v.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<fmt_Stringer>() { (Arc::new(Mutex::new(Some(typed_val.clone()))), true) } else if let Some(typed_val) = any_val.downcast_ref::<Celsius>() { (Arc::new(Mutex::new(Some(fmt_Stringer::__go_from_display(typed_val.clone())))), true) } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<Celsius>>>>() { (Arc::new(Mutex::new(Some(fmt_Stringer::__go_from_display_cell(typed_val.clone())))), true) } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<Counter>>>>() { (Arc::new(Mutex::new(Some(fmt_Stringer::__go_from_display_cell(typed_val.clone())))), true) } else { (Arc::new(Mutex::new(None::<fmt_Stringer>)), false) }
        } else {
            (Arc::new(Mutex::new(None::<fmt_Stringer>)), false)
        }
    });;
        if ok {
            return Arc::new(Mutex::new(Some(format!("{}{}", "labelled ".to_string(), (*(*s.lock().unwrap().as_ref().unwrap()).string().lock().unwrap().as_ref().unwrap())))));;
        }
    }
    Arc::new(Mutex::new(Some("unlabelled".to_string())))
}

pub fn must_string(v: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    let mut out: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));

    let __go_previous_panic_hook = go_silence_panic_hook();
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut out_defer_captured = out.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        {
        let mut r = go_recover();;
        if !{ let __nil_result = (*r.lock().unwrap()).as_ref().map_or(true, |__any| (**__any).is::<GoNilAny>()); __nil_result } {
            { let new_val = Arc::new(Mutex::new(Some(format!("{}{}", format!("{}", "panic: ".to_string()), format!("{}", { let __text = match r.lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *out_defer_captured.lock().unwrap() = __moved_val; };;
        }
    }
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
        {
        { let new_val = ({
        let val = v.clone();
        let __stringer: Result<fmt_Stringer, String> = {
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<fmt_Stringer>() { Ok(typed_val.clone()) } else if let Some(typed_val) = any_val.downcast_ref::<Celsius>() { Ok(fmt_Stringer::__go_from_display(typed_val.clone())) } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<Celsius>>>>() { Ok(fmt_Stringer::__go_from_display_cell(typed_val.clone())) } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<Counter>>>>() { Ok(fmt_Stringer::__go_from_display_cell(typed_val.clone())) } else { Err(format!("interface conversion: {} is not fmt.Stringer: missing method String", __go_type_name(&**any_val))) }
        } else {
            Err("interface conversion: interface is nil, not fmt.Stringer".to_string())
        }
        };
        match __stringer { Ok(__s) => __s, Err(__message) => panic!("{}", __message) }
    }).string(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *out.lock().unwrap() = __moved_val; };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return out.clone();
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
            out.clone()
        }
    }
}

fn main() {
    let mut s: Arc<Mutex<Option<fmt_Stringer>>> = Arc::new(Mutex::new(Some(fmt_Stringer::__go_from_display(Celsius { deg: Arc::new(Mutex::new(Some(21))), ..Default::default() }))));
    println!("{}", format!("{}", (*(*s.lock().unwrap().as_ref().unwrap()).string().lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", match s.lock().unwrap().as_ref() { Some(__iface) => format!("{}", __iface), None => "<nil>".to_string() }));

    let mut c = Arc::new(Mutex::new(Some(Counter { n: Arc::new(Mutex::new(Some(1))), ..Default::default() })));
    show(Arc::new(Mutex::new(Some(fmt_Stringer::__go_from_display_cell(c.clone())))));
    { let new_val = 2; *(*c.lock().unwrap().as_ref().unwrap()).n.lock().unwrap() = Some(new_val); };
    show(Arc::new(Mutex::new(Some(fmt_Stringer::__go_from_display_cell(c.clone())))));

    println!("{}", format!("{}", (*join(Arc::new(Mutex::new(Some(Vec::<fmt_Stringer>::from([fmt_Stringer::__go_from_display(Celsius { deg: Arc::new(Mutex::new(Some(-3))), ..Default::default() }), fmt_Stringer::__go_from_display_cell(Arc::new(Mutex::new(Some(Counter { n: Arc::new(Mutex::new(Some(9))), ..Default::default() })))), ({ let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v })]))))).lock().unwrap().as_ref().unwrap())));

    println!("{}", format!("{}", (*describe(Arc::new(Mutex::new(Some(Box::new(Celsius { deg: Arc::new(Mutex::new(Some(5))), ..Default::default() }) as Box<dyn Any + Send + Sync>)))).lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", (*describe(Arc::new(Mutex::new(Some(Box::new(c.clone()) as Box<dyn Any + Send + Sync>)))).lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", (*describe(Arc::new(Mutex::new(Some(Box::new(Counter { n: Arc::new(Mutex::new(Some(0))) }) as Box<dyn Any + Send + Sync>)))).lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", (*describe(Arc::new(Mutex::new(Some(Box::new(7i64) as Box<dyn Any + Send + Sync>)))).lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", (*describe(Arc::new(Mutex::new(Some(Box::new(Plain { x: Arc::new(Mutex::new(Some(0))) }) as Box<dyn Any + Send + Sync>)))).lock().unwrap().as_ref().unwrap())));

    println!("{}", format!("{}", (*label(Arc::new(Mutex::new(Some(Box::new(Celsius { deg: Arc::new(Mutex::new(Some(0))), ..Default::default() }) as Box<dyn Any + Send + Sync>)))).lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", (*label(Arc::new(Mutex::new(Some(Box::new(c.clone()) as Box<dyn Any + Send + Sync>)))).lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", (*label(Arc::new(Mutex::new(Some(Box::new("text".to_string()) as Box<dyn Any + Send + Sync>)))).lock().unwrap().as_ref().unwrap())));
    let mut nothing: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(None));
    println!("{}", format!("{}", (*label(go_any_handle_copy(&nothing)).lock().unwrap().as_ref().unwrap())));

    println!("{}", format!("{}", (*must_string(Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(Counter { n: Arc::new(Mutex::new(Some(4))), ..Default::default() }))).clone()) as Box<dyn Any + Send + Sync>)))).lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", (*must_string(Arc::new(Mutex::new(Some(Box::new(5i64) as Box<dyn Any + Send + Sync>)))).lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", (*must_string(go_any_handle_copy(&nothing)).lock().unwrap().as_ref().unwrap())));

        // A value receiver's String makes *Celsius a Stringer too.
    println!("{} {}", format!("{}", (*label(Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(Celsius { deg: Arc::new(Mutex::new(Some(8))), ..Default::default() }))).clone()) as Box<dyn Any + Send + Sync>)))).lock().unwrap().as_ref().unwrap())), format!("{}", (*describe(Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(Celsius { deg: Arc::new(Mutex::new(Some(9))), ..Default::default() }))).clone()) as Box<dyn Any + Send + Sync>)))).lock().unwrap().as_ref().unwrap())));

        // An any conversion asserted in place.
    {
        let (mut st, mut ok) = ({
        let val = Arc::new(Mutex::new(Some(Box::new(Celsius { deg: Arc::new(Mutex::new(Some(10))), ..Default::default() }) as Box<dyn Any + Send + Sync>))).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<fmt_Stringer>() { (Arc::new(Mutex::new(Some(typed_val.clone()))), true) } else if let Some(typed_val) = any_val.downcast_ref::<Celsius>() { (Arc::new(Mutex::new(Some(fmt_Stringer::__go_from_display(typed_val.clone())))), true) } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<Celsius>>>>() { (Arc::new(Mutex::new(Some(fmt_Stringer::__go_from_display_cell(typed_val.clone())))), true) } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<Counter>>>>() { (Arc::new(Mutex::new(Some(fmt_Stringer::__go_from_display_cell(typed_val.clone())))), true) } else { (Arc::new(Mutex::new(None::<fmt_Stringer>)), false) }
        } else {
            (Arc::new(Mutex::new(None::<fmt_Stringer>)), false)
        }
    });;
        if ok {
            println!("{} {}", format!("{}", "in place".to_string()), format!("{}", (*(*st.lock().unwrap().as_ref().unwrap()).string().lock().unwrap().as_ref().unwrap())));;
        }
    }
    print!("{} {} {}\n", format!("{}", { let __type_name = match (s).lock().unwrap().as_ref() { Some(__stringer) => __go_type_name(&*__stringer.__go_value), None => "<nil>" }; __type_name }), format!("{}", { let __type_name = match (stringer(Arc::new(Mutex::new(Some(fmt_Stringer::__go_from_display_cell(c.clone())))))).lock().unwrap().as_ref() { Some(__stringer) => __go_type_name(&*__stringer.__go_value), None => "<nil>" }; __type_name }), format!("{}", match s.lock().unwrap().as_ref() { Some(__iface) => format!("{}", __iface), None => "<nil>".to_string() }));

        // A call's result, an assertion printed directly, and %T of one.
    let mut made: Arc<Mutex<Option<fmt_Stringer>>> = Arc::new(Mutex::new(Some(fmt_Stringer::__go_from_display((*make_celsius(Arc::new(Mutex::new(Some(11)))).lock().unwrap().as_ref().unwrap()).clone()))));
    let mut held: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(Celsius { deg: Arc::new(Mutex::new(Some(12))), ..Default::default() }) as Box<dyn Any + Send + Sync>)));
    println!("{} {}", format!("{}", (*(*made.lock().unwrap().as_ref().unwrap()).string().lock().unwrap().as_ref().unwrap())), format!("{}", format!("{}", ({
        let val = held.clone();
        let __stringer: Result<fmt_Stringer, String> = {
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<fmt_Stringer>() { Ok(typed_val.clone()) } else if let Some(typed_val) = any_val.downcast_ref::<Celsius>() { Ok(fmt_Stringer::__go_from_display(typed_val.clone())) } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<Celsius>>>>() { Ok(fmt_Stringer::__go_from_display_cell(typed_val.clone())) } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<Counter>>>>() { Ok(fmt_Stringer::__go_from_display_cell(typed_val.clone())) } else { Err(format!("interface conversion: {} is not fmt.Stringer: missing method String", __go_type_name(&**any_val))) }
        } else {
            Err("interface conversion: interface is nil, not fmt.Stringer".to_string())
        }
        };
        match __stringer { Ok(__s) => __s, Err(__message) => panic!("{}", __message) }
    }))));
    print!("{}\n", { let __type_name = __go_type_name(&*(({
        let val = held.clone();
        let __stringer: Result<fmt_Stringer, String> = {
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<fmt_Stringer>() { Ok(typed_val.clone()) } else if let Some(typed_val) = any_val.downcast_ref::<Celsius>() { Ok(fmt_Stringer::__go_from_display(typed_val.clone())) } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<Celsius>>>>() { Ok(fmt_Stringer::__go_from_display_cell(typed_val.clone())) } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<Counter>>>>() { Ok(fmt_Stringer::__go_from_display_cell(typed_val.clone())) } else { Err(format!("interface conversion: {} is not fmt.Stringer: missing method String", __go_type_name(&**any_val))) }
        } else {
            Err("interface conversion: interface is nil, not fmt.Stringer".to_string())
        }
        };
        match __stringer { Ok(__s) => __s, Err(__message) => panic!("{}", __message) }
    })).__go_value); __type_name });

        // A failed assertion that is recovered leaves the value usable.
    let mut five: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(5i64) as Box<dyn Any + Send + Sync>)));
    println!("{} {}", format!("{}", (*must_string(go_any_handle_copy(&five)).lock().unwrap().as_ref().unwrap())), format!("{}", (*label(go_any_handle_copy(&five)).lock().unwrap().as_ref().unwrap())));

    let mut wg = WaitGroup::new();
    let mut results = Arc::new(Mutex::new(Some(vec!["".to_string(); (2) as usize])));
    for (i, v) in vec![Box::new(Celsius { deg: Arc::new(Mutex::new(Some(30))), ..Default::default() }) as Box<dyn Any + Send + Sync>, Box::new(Arc::new(Mutex::new(Some(Counter { n: Arc::new(Mutex::new(Some(5))), ..Default::default() }))).clone()) as Box<dyn Any + Send + Sync>].iter().enumerate() {
        wg.add(1);
        let results_thread = results.clone(); let wg_thread = wg.clone(); let __go_arg_0 = Arc::new(Mutex::new(Some(i as i64))); let __go_arg_1 = Arc::new(Mutex::new(Some(go_any_clone(v.as_ref())))); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();
        let mut __closure = move |i: Arc<Mutex<Option<i64>>>, v: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>| {
            let __go_previous_panic_hook = go_silence_panic_hook();
let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
let wg_defer_captured = wg_thread.clone(); __defer_stack.push(Box::new(move || {
        wg_defer_captured.done();
    }));;
            { let __idx = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __rhs = (*label(go_any_handle_copy(&v)).lock().unwrap().as_ref().unwrap()).clone(); (*results_thread.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };;
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
        __closure(__go_arg_0, __go_arg_1);
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    }
    wg.wait();
    println!("{} {} {}", format!("{}", { let __seq = { let __seq_holder = results.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }), format!("{}", "|".to_string()), format!("{}", { let __seq = { let __seq_holder = results.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }));
}