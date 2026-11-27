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

#[derive(Clone, Default)]
pub struct Box_ {
    pub r#fn: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64>>>>,
}

impl Box_ {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.r#fn.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            r#fn: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for Box_ {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", "<func>");
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


pub type Op = Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64>>>>;


pub(crate) struct GoGlobal<T> {
    value: std::cell::UnsafeCell<Option<T>>,
}
unsafe impl<T> Sync for GoGlobal<T> {}
impl<T> GoGlobal<T> {
    pub(crate) const fn new() -> Self {
        Self { value: std::cell::UnsafeCell::new(None) }
    }
    pub(crate) fn borrow(&'static self) -> &'static Option<T> {
        unsafe { &*self.value.get() }
    }
    pub(crate) fn borrow_mut(&'static self) -> &'static mut Option<T> {
        unsafe { &mut *self.value.get() }
    }
    pub(crate) fn clone(&'static self) -> std::rc::Rc<std::cell::RefCell<Option<T>>> where T: Clone {
        std::rc::Rc::new(std::cell::RefCell::new(self.borrow().clone()))
    }
}

pub(crate) static now_1: GoGlobal<Rc<dyn Fn() -> i64>> = GoGlobal::new();


fn __go_init_globals() {
    *now_1.borrow_mut() = Some(Rc::new(move || -> i64 {
        1
    }) as Rc<dyn Fn() -> i64>);
}


pub trait OpMethods {
    fn twice(&self, x: Rc<RefCell<Option<i64>>>) -> i64;
}

impl OpMethods for Op {
    fn twice(&self, x: Rc<RefCell<Option<i64>>>) -> i64 {
        { let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64> = self.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64> = self.borrow().as_ref().unwrap().clone(); (__f)(x.clone()) })))) }
    }
}

pub fn keep(g: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64>>>>) -> Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64>>>> {
    Rc::new(RefCell::new((*g.borrow()).clone()))
}

pub fn swap_in(mut f: Rc<RefCell<Option<Rc<dyn Fn() -> Rc<RefCell<Option<String>>>>>>>) -> Rc<RefCell<Option<Rc<dyn Fn() -> Rc<RefCell<Option<String>>>>>>> {
    let mut old = Rc::new(RefCell::new((*f.borrow()).clone()));
    { let new_val = Rc::new(move || -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some("new".to_string())))
    }) as Rc<dyn Fn() -> Rc<RefCell<Option<String>>>>; *f.borrow_mut() = Some(new_val); };
    let _ = &f;
    return Rc::new(RefCell::new((*old.borrow()).clone()));
}

pub fn make_adder(n: Rc<RefCell<Option<i64>>>) -> Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64>>>> {
    let n_closure_clone = n.clone(); return Rc::new(RefCell::new(Some(Rc::new(move |x: Rc<RefCell<Option<i64>>>| -> i64 {
        (*x.borrow().as_ref().unwrap()) + (*n_closure_clone.borrow().as_ref().unwrap())
    }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64>)));
}

pub fn deferred() {
    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    let __go_previous_panic_hook = go_silence_panic_hook();
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut act = Rc::new(RefCell::new(Some(Rc::new(move |i: Rc<RefCell<Option<i64>>>| {
        println!("{} {}", format!("{}", "act".to_string()), format!("{}", { let __v = (*i.borrow().as_ref().unwrap()).clone(); __v }));
    }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> ()>)));
        let act_defer_captured = Rc::new(RefCell::new((*act.borrow()).clone())); __defer_stack.push(Box::new(move || {
        { let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> ()> = act_defer_captured.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(1)))) };
    }));
        let mut saved = Rc::new(RefCell::new((*act.borrow()).clone()));
        { let new_val = Rc::new(move |i: Rc<RefCell<Option<i64>>>| {
        println!("{} {}", format!("{}", "replaced".to_string()), format!("{}", { let __v = (*i.borrow().as_ref().unwrap()).clone(); __v }));
    }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> ()>; *act.borrow_mut() = Some(new_val); };
        { let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> ()> = saved.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(2)))) };
        { let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> ()> = act.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(3)))) };

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
    __go_init_all();
    deferred();

    let mut inc = Rc::new(RefCell::new(Some(Rc::new(move |x: Rc<RefCell<Option<i64>>>| -> i64 {
        (*x.borrow().as_ref().unwrap()) + 1
    }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64>)));
    let mut other: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64>>>> = Rc::new(RefCell::new(None));
    { let __func_value = (*inc.borrow()).clone(); *other.borrow_mut() = __func_value; };
    { let new_val = Rc::new(move |x: Rc<RefCell<Option<i64>>>| -> i64 {
        (*x.borrow().as_ref().unwrap()) * 10
    }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64>; *inc.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "assigned:".to_string()), format!("{}", { let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64> = other.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(1)))) }), format!("{}", { let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64> = inc.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(1)))) }));

    let mut b = Rc::new(RefCell::new(Some(Box_ { r#fn: Rc::new(RefCell::new((*inc.borrow()).clone())), ..Default::default() })));
    { let new_val = Rc::new(move |x: Rc<RefCell<Option<i64>>>| -> i64 {
        -((*x.borrow().as_ref().unwrap()))
    }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64>; *inc.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "field:".to_string()), format!("{}", { let __f_holder = (*b.borrow().as_ref().unwrap()).r#fn.clone(); let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(2)))) }), format!("{}", { let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64> = inc.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(2)))) }));

    let mut kept = keep(Rc::new(RefCell::new((*inc.borrow()).clone())));
    let mut kept = Rc::new(RefCell::new(GoFuncValue::go_func_value(&kept)));
    { let new_val = Rc::new(move |x: Rc<RefCell<Option<i64>>>| -> i64 {
        { let __bin_x = (*x.borrow().as_ref().unwrap()).clone(); __bin_x * __bin_x }
    }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64>; *inc.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "argument:".to_string()), format!("{}", { let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64> = kept.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(3)))) }), format!("{}", { let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64> = inc.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(3)))) }));

    let mut b2 = { let __owned = b.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) };
    { let new_val = Rc::new(RefCell::new((*inc.borrow()).clone())); (*b.borrow_mut().as_mut().unwrap()).r#fn = new_val; };
    println!("{} {} {}", format!("{}", "struct copy:".to_string()), format!("{}", { let __f_holder = (*b2.borrow().as_ref().unwrap()).r#fn.clone(); let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(4)))) }), format!("{}", { let __f_holder = (*b.borrow().as_ref().unwrap()).r#fn.clone(); let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(4)))) }));
    let mut b3 = { let __owned = b.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) };
    { let new_val = Rc::new(move |x: Rc<RefCell<Option<i64>>>| -> i64 {
        1000 + (*x.borrow().as_ref().unwrap())
    }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64>; *(*b.borrow().as_ref().unwrap()).r#fn.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "struct copy, literal:".to_string()), format!("{}", { let __f_holder = (*b3.borrow().as_ref().unwrap()).r#fn.clone(); let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(4)))) }), format!("{}", { let __f_holder = (*b.borrow().as_ref().unwrap()).r#fn.clone(); let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(4)))) }));

    let mut fns = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new((*inc.borrow()).clone()))])));
    { let new_val = Rc::new(move |x: Rc<RefCell<Option<i64>>>| -> i64 {
        100 + (*x.borrow().as_ref().unwrap())
    }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64>; *inc.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "slice:".to_string()), format!("{}", { let __f_holder = (*fns.borrow().as_ref().unwrap())[(0) as usize].clone(); let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(5)))) }), format!("{}", { let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64> = inc.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(5)))) }));

    let mut m = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64>>>>>::from([("a".to_string(), Rc::new(RefCell::new((*inc.borrow()).clone())))]))));
    { let new_val = Rc::new(move |x: Rc<RefCell<Option<i64>>>| -> i64 {
        7
    }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64>; *inc.borrow_mut() = Some(new_val); };
    { let __map_key = "b".to_string(); let __map_value = Rc::new(RefCell::new((*inc.borrow()).clone())); (*m.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{} {} {}", format!("{}", "map:".to_string()), format!("{}", { let __f_holder = { let __map_holder = m.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }; let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(1)))) }), format!("{}", { let __f_holder = { let __map_holder = m.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"b".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }; let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(1)))) }));

        // A closure that calls a variable sees the variable's later value.
    let mut cb: Rc<RefCell<Option<Rc<dyn Fn() -> Rc<RefCell<Option<String>>>>>>> = Rc::new(RefCell::new(None));
    let cb_closure_clone = cb.clone(); let mut run = Rc::new(RefCell::new(Some(Rc::new(move || -> Rc<RefCell<Option<String>>> {
        return { let __f: Rc<dyn Fn() -> Rc<RefCell<Option<String>>>> = cb_closure_clone.borrow().as_ref().unwrap().clone(); (__f)() };
    }) as Rc<dyn Fn() -> Rc<RefCell<Option<String>>>>)));
    let mut h = Rc::new(RefCell::new(Some(Rc::new(move || -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some("h".to_string())))
    }) as Rc<dyn Fn() -> Rc<RefCell<Option<String>>>>)));
    { let __func_value = (*h.borrow()).clone(); *cb.borrow_mut() = __func_value; };
    println!("{} {}", format!("{}", "captured:".to_string()), format!("{}", (*{ let __f: Rc<dyn Fn() -> Rc<RefCell<Option<String>>>> = run.borrow().as_ref().unwrap().clone(); (__f)() }.borrow().as_ref().unwrap())));
    { let new_val = Rc::new(move || -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some("changed h".to_string())))
    }) as Rc<dyn Fn() -> Rc<RefCell<Option<String>>>>; *h.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "captured after:".to_string()), format!("{}", (*{ let __f: Rc<dyn Fn() -> Rc<RefCell<Option<String>>>> = run.borrow().as_ref().unwrap().clone(); (__f)() }.borrow().as_ref().unwrap())), format!("{}", (*{ let __f: Rc<dyn Fn() -> Rc<RefCell<Option<String>>>> = h.borrow().as_ref().unwrap().clone(); (__f)() }.borrow().as_ref().unwrap())));

        // Wrapping a function in a new one that calls the old one.
    let mut log = Rc::new(RefCell::new(Some(Rc::new(move |s: Rc<RefCell<Option<String>>>| -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("{}{}", "log:".to_string(), (*s.borrow().as_ref().unwrap())))))
    }) as Rc<dyn Fn(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>>)));
    let mut next = Rc::new(RefCell::new((*log.borrow()).clone()));
    let next_closure_clone = next.clone(); { let new_val = Rc::new(move |s: Rc<RefCell<Option<String>>>| -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some(format!("{}{}", "before ".to_string(), (*{ let __f: Rc<dyn Fn(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>> = next_closure_clone.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })))) }.borrow().as_ref().unwrap())))));
    }) as Rc<dyn Fn(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>>; *log.borrow_mut() = Some(new_val); };
    println!("{}", format!("{}", (*{ let __f: Rc<dyn Fn(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>> = log.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some("x".to_string())))) }.borrow().as_ref().unwrap())));

        // Saving and restoring a package-level func variable.
    let mut orig = Rc::new(RefCell::new(Some({ let __global_func = GoFuncValue::go_func_value(&(*now_1.borrow())); Rc::new(move || -> i64 { (__global_func.as_ref().expect("runtime error: invalid memory address or nil pointer dereference"))() }) as Rc<dyn Fn() -> i64> })));
    let mut orig = Rc::new(RefCell::new(GoFuncValue::go_func_value(&(*now_1.borrow()))));
    { let new_val = Rc::new(move || -> i64 {
        42
    }) as Rc<dyn Fn() -> i64>; *now_1.borrow_mut() = Some(new_val); };
    println!("{} {}", format!("{}", "global:".to_string()), format!("{}", { let __f: Rc<dyn Fn() -> i64> = now_1.borrow().as_ref().unwrap().clone(); (__f)() }));
    { let __func_value = (*orig.borrow()).clone(); *now_1.borrow_mut() = __func_value; };
    println!("{} {}", format!("{}", "restored:".to_string()), format!("{}", { let __f: Rc<dyn Fn() -> i64> = now_1.borrow().as_ref().unwrap().clone(); (__f)() }));

        // Recursion through a variable, and a copy of it.
    let mut fib: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64>>>> = Rc::new(RefCell::new(None));
    let fib_closure_clone = fib.clone(); { let __func_lit_target = fib_closure_clone.clone(); let new_val = { let fib_closure_clone_closure_clone = fib_closure_clone.clone(); Rc::new(move |n: Rc<RefCell<Option<i64>>>| -> i64 {
        if (*n.borrow().as_ref().unwrap()) < 2 {
        return (*n.borrow().as_ref().unwrap());
    }
        return { let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64> = fib_closure_clone_closure_clone.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*n.borrow().as_ref().unwrap()) - 1; __arg_value })))) } + { let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64> = fib_closure_clone_closure_clone.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*n.borrow().as_ref().unwrap()) - 2; __arg_value })))) };
    }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64> }; *__func_lit_target.borrow_mut() = Some(new_val); };
    let mut f2 = Rc::new(RefCell::new((*fib.borrow()).clone()));
    println!("{} {} {}", format!("{}", "fib:".to_string()), format!("{}", { let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64> = fib.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(10)))) }), format!("{}", { let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64> = f2.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(11)))) }));

        // nil copies stay nil.
    let mut z: Rc<RefCell<Option<Rc<dyn Fn() -> Rc<RefCell<Option<String>>>>>>> = Rc::new(RefCell::new(None));
    println!("{} {}", format!("{}", "nil:".to_string()), format!("{}", { let __nil_result = (*z.borrow()).is_none(); __nil_result }));
    { let __func_value = (*cb.borrow()).clone(); *z.borrow_mut() = __func_value; };
    *cb.borrow_mut() = None;
    { let __go_print_arg_1 = { let __f: Rc<dyn Fn() -> Rc<RefCell<Option<String>>>> = z.borrow().as_ref().unwrap().clone(); (__f)() }; let __go_print_value = println!("{} {} {} {}", format!("{}", "nil after:".to_string()), format!("{}", { let __nil_result = (*z.borrow()).is_none(); __nil_result }), format!("{}", { let __nil_result = (*cb.borrow()).is_none(); __nil_result }), format!("{}", { let __v = (*__go_print_arg_1.borrow().as_ref().unwrap()).clone(); __v })); __go_print_value };

        // A named func type with a method.
    let mut o: Op = Rc::new(RefCell::new(Some(Rc::new(move |x: Rc<RefCell<Option<i64>>>| -> i64 {
        (*x.borrow().as_ref().unwrap()) + 1
    }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64>)));
    let mut o2 = Rc::new(RefCell::new((*o.borrow()).clone()));
    { let new_val = Rc::new(move |x: Rc<RefCell<Option<i64>>>| -> i64 {
        (*x.borrow().as_ref().unwrap()) * 3
    }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64>; *o.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "named:".to_string()), format!("{}", o2.twice(Rc::new(RefCell::new(Some(1))))), format!("{}", o.twice(Rc::new(RefCell::new(Some(1))))));

        // A callee that assigns its parameter does not change the caller's.
    let mut g = Rc::new(RefCell::new(Some(Rc::new(move || -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some("orig".to_string())))
    }) as Rc<dyn Fn() -> Rc<RefCell<Option<String>>>>)));
    let mut old = swap_in(Rc::new(RefCell::new((*g.borrow()).clone())));
    let mut old = Rc::new(RefCell::new(GoFuncValue::go_func_value(&old)));
    println!("{} {} {}", format!("{}", "param:".to_string()), format!("{}", (*{ let __f: Rc<dyn Fn() -> Rc<RefCell<Option<String>>>> = old.borrow().as_ref().unwrap().clone(); (__f)() }.borrow().as_ref().unwrap())), format!("{}", (*{ let __f: Rc<dyn Fn() -> Rc<RefCell<Option<String>>>> = g.borrow().as_ref().unwrap().clone(); (__f)() }.borrow().as_ref().unwrap())));

        // A function's result.
    let mut adder = make_adder(Rc::new(RefCell::new(Some(5))));
    let mut adder = Rc::new(RefCell::new(GoFuncValue::go_func_value(&adder)));
    let mut savedAdder = Rc::new(RefCell::new((*adder.borrow()).clone()));
    { let __func_value = GoFuncValue::go_func_value(&(make_adder(Rc::new(RefCell::new(Some(100)))))); *adder.borrow_mut() = __func_value; };
    println!("{} {} {}", format!("{}", "result:".to_string()), format!("{}", { let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64> = savedAdder.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(1)))) }), format!("{}", { let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64> = adder.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(1)))) }));

        // A closure that assigns a captured func variable.
    let mut step = Rc::new(RefCell::new(Some(Rc::new(move || -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some("a".to_string())))
    }) as Rc<dyn Fn() -> Rc<RefCell<Option<String>>>>)));
    let mut step_closure_clone = step.clone(); let mut advance = Rc::new(RefCell::new(Some(Rc::new(move || {
        let mut step_closure_clone = step_closure_clone.clone();
        { let new_val = Rc::new(move || -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some("b".to_string())))
    }) as Rc<dyn Fn() -> Rc<RefCell<Option<String>>>>; *step_closure_clone.borrow_mut() = Some(new_val); };
    }) as Rc<dyn Fn() -> ()>)));
    let mut before = Rc::new(RefCell::new((*step.borrow()).clone()));
    { let __f: Rc<dyn Fn() -> ()> = advance.borrow().as_ref().unwrap().clone(); (__f)() };
    println!("{} {} {}", format!("{}", "closure assigns:".to_string()), format!("{}", (*{ let __f: Rc<dyn Fn() -> Rc<RefCell<Option<String>>>> = before.borrow().as_ref().unwrap().clone(); (__f)() }.borrow().as_ref().unwrap())), format!("{}", (*{ let __f: Rc<dyn Fn() -> Rc<RefCell<Option<String>>>> = step.borrow().as_ref().unwrap().clone(); (__f)() }.borrow().as_ref().unwrap())));

        // A function that assigns the variable it was called through.
    let mut self_: Rc<RefCell<Option<Rc<dyn Fn() -> Rc<RefCell<Option<String>>>>>>> = Rc::new(RefCell::new(None));
    let mut self_closure_clone = self_.clone(); { let __func_lit_target = self_closure_clone.clone(); let new_val = { let mut self_closure_clone_closure_clone = self_closure_clone.clone(); Rc::new(move || -> Rc<RefCell<Option<String>>> {
        let mut self_closure_clone_closure_clone = self_closure_clone_closure_clone.clone();
        { let new_val = Rc::new(move || -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some("second".to_string())))
    }) as Rc<dyn Fn() -> Rc<RefCell<Option<String>>>>; *self_closure_clone_closure_clone.borrow_mut() = Some(new_val); };
        Rc::new(RefCell::new(Some("first".to_string())))
    }) as Rc<dyn Fn() -> Rc<RefCell<Option<String>>>> }; *__func_lit_target.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "self:".to_string()), format!("{}", (*{ let __f: Rc<dyn Fn() -> Rc<RefCell<Option<String>>>> = self_.borrow().as_ref().unwrap().clone(); (__f)() }.borrow().as_ref().unwrap())), format!("{}", (*{ let __f: Rc<dyn Fn() -> Rc<RefCell<Option<String>>>> = self_.borrow().as_ref().unwrap().clone(); (__f)() }.borrow().as_ref().unwrap())));
}

pub(crate) fn __go_init_all() {
    static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();
    __GO_INIT_ONCE.call_once(|| {
        self::__go_init_globals();
    });
}
