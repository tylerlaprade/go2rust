#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use std::any::Any;
use std::cell::{RefCell};
use std::error::Error as StdError;
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
    } else if let Some(v) = value.downcast_ref::<Celsius>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<GoNilAny>() {
        "<nil>".to_string()
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() {
        match (*v.borrow()).as_ref() { Some(__err) => format!("{}", __err), None => "<nil>".to_string() }
    } else {
        panic!("go2rust: cannot print this value held in an interface: its type has no formatting case yet")
    }
}

/// A nil interface value in a slot that holds a Box<dyn Any>, such as a
/// []any element: it copies, prints and compares as nil.
#[derive(Clone, Copy, Debug, PartialEq)]
struct GoNilAny;

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
    if let Some(v) = value.downcast_ref::<Celsius>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<GoNilAny>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
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

struct GoPointerError<T> {
    handle: Rc<RefCell<Option<T>>>,
}

impl<T> GoPointerError<T> {
    fn new(handle: Rc<RefCell<Option<T>>>) -> Self { Self { handle } }
}

impl<T: std::fmt::Display> std::fmt::Display for GoPointerError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let guard = self.handle.borrow();
        match guard.as_ref() {
            Some(value) => write!(f, "{}", value),
            None => Ok(()),
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Debug for GoPointerError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl<T: std::fmt::Display + 'static> std::error::Error for GoPointerError<T> {}

#[derive(Clone, Copy)]
struct GoAnyTypeMetadata {
    pub kind: &'static str,
    pub comparable: bool,
    pub elem_kind: Option<&'static str>,
    pub elem_comparable: bool,
}

fn go_any_type_metadata_registry() -> &'static std::sync::Mutex<std::collections::HashMap<std::any::TypeId, GoAnyTypeMetadata>> {
    static REGISTRY: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<std::any::TypeId, GoAnyTypeMetadata>>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

fn go_any_value_metadata_registry() -> &'static std::sync::Mutex<std::collections::HashMap<usize, GoAnyTypeMetadata>> {
    static REGISTRY: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<usize, GoAnyTypeMetadata>>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

fn go_any_value_metadata_key(value: &dyn Any) -> usize {
    value as *const dyn Any as *const () as usize
}

fn go_register_any_type<T: Any + 'static>(kind: &'static str, comparable: bool) {
    go_any_type_metadata_registry().lock().unwrap().insert(std::any::TypeId::of::<T>(), GoAnyTypeMetadata { kind, comparable, elem_kind: None, elem_comparable: false });
}

fn go_register_any_type_with_elem<T: Any + 'static>(kind: &'static str, comparable: bool, elem_kind: &'static str, elem_comparable: bool) {
    go_any_type_metadata_registry().lock().unwrap().insert(std::any::TypeId::of::<T>(), GoAnyTypeMetadata { kind, comparable, elem_kind: Some(elem_kind), elem_comparable });
}

// Boxes value as itself and records what kind of Go type it is. Every reader
// of an interface value looks for the value's own type in the box, so the
// box must hold nothing else: a wrapper around the value would be a type no
// reader knows.
fn go_box_any_with_metadata<T: Any + 'static>(value: T, kind: &'static str, comparable: bool) -> Box<dyn Any> {
    go_register_any_type::<T>(kind, comparable);
    Box::new(value) as Box<dyn Any>
}

fn go_register_any_value_metadata(value: &dyn Any, kind: &'static str, comparable: bool) {
    go_any_value_metadata_registry().lock().unwrap().insert(go_any_value_metadata_key(value), GoAnyTypeMetadata { kind, comparable, elem_kind: None, elem_comparable: false });
}

fn go_any_type_metadata(value: &dyn Any) -> Option<GoAnyTypeMetadata> {
    go_any_value_metadata_registry().lock().unwrap().get(&go_any_value_metadata_key(value)).copied()
        .or_else(|| go_any_type_metadata_registry().lock().unwrap().get(&value.type_id()).copied())
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
    if val.is::<Celsius>() { return "main.Celsius" }
    if val.is::<GoNilAny>() { return "<nil>" }
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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct errors_errorString;

impl std::fmt::Display for errors_errorString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<errors_errorString>")
    }
}

impl std::error::Error for errors_errorString {}


impl errors_errorString {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        panic!("errors_errorString.error bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct errors_joinError;

impl std::fmt::Display for errors_joinError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<errors_joinError>")
    }
}

impl std::error::Error for errors_joinError {}


impl errors_joinError {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        panic!("errors_joinError.error bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug)]
pub struct Celsius(pub Rc<RefCell<Option<f64>>>);

impl Clone for Celsius {
    fn clone(&self) -> Self {
        let __value = (*self.0.borrow()).clone();
        Celsius(Rc::new(RefCell::new(__value)))
    }
}

impl Default for Celsius {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(Some(Default::default()))))
    }
}

impl Display for Celsius {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", go_float_string(*self.0.borrow().as_ref().unwrap() as f64, false))
    }
}

impl PartialEq for Celsius {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<f64> for Celsius {
    fn eq(&self, other: &f64) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Celsius {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<f64> for Celsius {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Celsius> for f64 {
    fn eq(&self, other: &Celsius) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<Celsius> for f64 {
    fn partial_cmp(&self, other: &Celsius) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for Celsius {
    type Output = Celsius;
    fn add(self, other: Self) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<f64> for Celsius {
    type Output = Celsius;
    fn add(self, other: f64) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Celsius> for f64 {
    type Output = Celsius;
    fn add(self, other: Celsius) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Celsius {
    type Output = Celsius;
    fn sub(self, other: Self) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<f64> for Celsius {
    type Output = Celsius;
    fn sub(self, other: f64) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Celsius> for f64 {
    type Output = Celsius;
    fn sub(self, other: Celsius) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Celsius {
    type Output = Celsius;
    fn mul(self, other: Self) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<f64> for Celsius {
    type Output = Celsius;
    fn mul(self, other: f64) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Celsius> for f64 {
    type Output = Celsius;
    fn mul(self, other: Celsius) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(self * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Celsius {
    type Output = Celsius;
    fn div(self, other: Self) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div<f64> for Celsius {
    type Output = Celsius;
    fn div(self, other: f64) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Celsius> for f64 {
    type Output = Celsius;
    fn div(self, other: Celsius) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(self / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for Celsius {
    type Output = Celsius;
    fn neg(self) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(-*self.0.borrow().as_ref().unwrap()))))
    }
}


fn main() {
    go_install_panic_exit();
    let mut v: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(9i64) as Box<dyn Any>)));
    let mut f: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(2.5) as Box<dyn Any>)));
    let mut s: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new("go".to_string()) as Box<dyn Any>)));
    let mut c: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(Celsius(Rc::new(RefCell::new(Some(20.0 as f64))))) as Box<dyn Any>)));
    println!("{} {} {} {}", format!("{}", ({
        let val = v.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<i64>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "int")).clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "int")
        }
    }) + 1), format!("{}", ({
        let val = v.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<i64>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "int")).clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "int")
        }
    }) * 2), format!("{}", 10 - ({
        let val = v.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<i64>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "int")).clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "int")
        }
    })), format!("{}", ({
        let val = v.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<i64>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "int")).clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "int")
        }
    }) % 4));
    println!("{} {} {}", { let __text = go_float_string((({
        let val = f.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<f64>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "float64")).clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "float64")
        }
    }) * 2.0) as f64, false); __text }, format!("{}", format!("{}{}", ({
        let val = s.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<String>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "string")).clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "string")
        }
    }), "pher".to_string())), format!("{}", Celsius(Rc::new(RefCell::new(Some(((*({
        let val = c.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<Celsius>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "main.Celsius")).clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "main.Celsius")
        }
    }).0.borrow().as_ref().unwrap()) * 2.0)))))));
    println!("{} {} {}", format!("{}", ({
        let val = v.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<i64>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "int")).clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "int")
        }
    }) > 5), format!("{}", ({
        let val = s.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<String>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "string")).clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "string")
        }
    }) == "go".to_string()), format!("{}", ({
        let val = f.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<f64>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "float64")).clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "float64")
        }
    }) < 1.0));
    let mut n = Rc::new(RefCell::new(Some(({
        let val = v.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<i64>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "int")).clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "int")
        }
    }) << 2)));
    println!("{}", format!("{}", { let __v = (*n.borrow().as_ref().unwrap()).clone(); __v }));
    let mut vals = Rc::new(RefCell::new(Some(vec![Box::new(1i64) as Box<dyn Any>, Box::new(2i64) as Box<dyn Any>, Box::new(3i64) as Box<dyn Any>])));
    let mut total = Rc::new(RefCell::new(Some(0i64)));
    { let __range_holder = vals.clone(); let __range_guard = __range_holder.borrow(); let __range_len = __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); std::mem::drop(__range_guard); let __range_values = 0..__range_len; for x in __range_values.map(|__range_index| { let __range_guard = __range_holder.borrow(); go_any_clone(__range_guard.as_ref().unwrap()[__range_index].as_ref()) }) {
        { let __rhs = ({
        let any_val: &dyn Any = x.as_ref();
        any_val.downcast_ref::<i64>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(any_val), "int")).clone()
    }) * 10; let mut guard = total.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }
    println!("{}", format!("{}", { let __v = (*total.borrow().as_ref().unwrap()).clone(); __v }));
    for x in &vec![{ let __err_holder = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("boom".to_string()))))).clone(); let __err_guard = __err_holder.borrow(); match __err_guard.as_ref() { None => Box::new(GoNilAny) as Box<dyn Any>, Some(__err) => Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any> } }, Box::new(7i64) as Box<dyn Any>] {
        {
        let (mut err, mut ok) = ({
        let val = Rc::new(RefCell::new(Some(go_any_clone(x.as_ref()))));
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() {
                ({ let __err_source = typed_val; let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }, true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<errors_errorString>>>>() {
                (Rc::new(RefCell::new(Some(Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<errors_joinError>>>>() {
                (Rc::new(RefCell::new(Some(Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError>))), true)
            } else {
                (Rc::new(RefCell::new(None::<Rc<dyn StdError>>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<Rc<dyn StdError>>)), false)
        }
    });;
        if ok {
            println!("{} {}", format!("{}", "err".to_string()), format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", err.borrow().as_ref().unwrap())))).borrow().as_ref().unwrap())));;
        }
    }
        {
    let _ts_ref = x;
    let _ts_val: Option<&dyn Any> = Some(_ts_ref.as_ref() as &dyn Any);
    let _ts_is_nil = _ts_val.map_or(false, |__v| __v.is::<GoNilAny>());
    if _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<errors_errorString>>>>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<errors_joinError>>>>()).is_some() {
        let w = if let Some(__err_handle) = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>()) { { let __err_source = __err_handle; let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) } } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<errors_errorString>>>>()) { Rc::new(RefCell::new(Some(Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError>))) } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<errors_joinError>>>>()) { Rc::new(RefCell::new(Some(Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError>))) } else { unreachable!("type switch error case matched no error payload") };
        println!("{} {}", format!("{}", "error".to_string()), format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", ({
        let any_val: &dyn Any = x.as_ref();
        { if let Some(__err_handle) = any_val.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() { (*__err_handle.borrow()).clone().expect("type assertion failed") } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<errors_errorString>>>>() { Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError> } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<errors_joinError>>>>() { Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError> } else { panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(any_val), "error") } }
    }))))).borrow().as_ref().unwrap())));;
    } else {
        let w = _ts_val.unwrap();
        {
        let (mut n, mut ok) = ({
        let val = Rc::new(RefCell::new(Some(go_any_clone(w))));
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<i64>() {
                (Rc::new(RefCell::new(Some(typed_val.clone()))), true)
            } else {
                (Rc::new(RefCell::new(Some(0))), false)
            }
        } else {
            (Rc::new(RefCell::new(Some(0))), false)
        }
    });;
        if ok {
            println!("{} {}", format!("{}", "int".to_string()), format!("{}", (*n.borrow().as_ref().unwrap()) + 1));;
        }
    };
    }
    }
    }
    let (mut a, mut b) = (go_any_handle_copy(&v), go_any_handle_copy(&s));
    println!("{} {}", format!("{}", match a.borrow().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }), format!("{}", match b.borrow().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }));
}