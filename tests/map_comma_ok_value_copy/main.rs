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
                    format!("{}:<nil>", k)
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
pub struct Point {
    pub x: Rc<RefCell<Option<i64>>>,
}

impl Point {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
        }
    }
}


impl Default for Point {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            x: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone, Default)]
pub struct Registry {
    pub by_name: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<Point>>>>>>>,
    pub counts: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>,
}

impl Registry {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.by_name.clone();
        let __go_clone_1_0 = self.counts.clone();
        Self {
            by_name: __go_clone_0_0,
            counts: __go_clone_1_0,
        }
    }
}

impl std::fmt::Display for Registry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_map(&self.by_name));
        let __go_fmt_1 = format!("{}", format_map(&self.counts));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone, Default)]
pub struct Scores(pub Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>);

impl Display for Scores {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_map(&self.0))
    }
}


/// "v, ok := m[k]" gives v its own copy of the value. Writing to v afterwards
/// must not change the map, and reading into existing variables must not take
/// the value out of the map.
fn main() {
    go_install_panic_exit();
    let mut counts = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i64>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(1i64))))]))));
    let (mut n, mut ok) = { let __map_holder = counts.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().clone()))), true), None => (Rc::new(RefCell::new(Some(0))), false) } };
    { let new_val = 5; *n.borrow_mut() = Some(new_val); };
    println!("{} {} {} {}", format!("{}", "int".to_string()), format!("{}", { let __v = (*n.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", ok), format!("{}", { let __map_holder = counts.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));

    let mut names = Rc::new(RefCell::new(Some(BTreeMap::<i64, Rc<RefCell<Option<String>>>>::from([(1i64, Rc::new(RefCell::new(Some("one".to_string()))))]))));
    let (mut name, mut ok) = { let __map_holder = names.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&1)) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().clone()))), true), None => (Rc::new(RefCell::new(Some(String::new()))), false) } };
    { (*name.borrow_mut().as_mut().unwrap()).push_str(&"!".to_string()); };
    println!("{} {} {} {}", format!("{}", "string".to_string()), format!("{}", { let __v = (*name.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", ok), format!("{}", { let __map_holder = names.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&1)).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()) }));

    {
        let (mut v, mut ok) = { let __map_holder = counts.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().clone()))), true), None => (Rc::new(RefCell::new(Some(0))), false) } };;
        if ok {
            { let mut guard = v.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); };
            println!("{} {} {}", format!("{}", "in an if".to_string()), format!("{}", { let __v = (*v.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __map_holder = counts.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));;
        }
    }

    let mut m: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));
    let mut found: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));
    { let (__tmp_0, __tmp_1) = { let __map_holder = counts.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().clone()))), true), None => (Rc::new(RefCell::new(Some(0))), false) } }; let __moved_tmp_0 = { let mut __guard = __tmp_0.borrow_mut(); __guard.take() }; *m.borrow_mut() = __moved_tmp_0; *found.borrow_mut() = Some(__tmp_1); };
    { let __rhs = 10; let mut guard = m.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    println!("{} {} {} {} {}", format!("{}", "int into variables".to_string()), format!("{}", { let __v = (*m.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*found.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __map_holder = counts.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", ({ let __len = (*counts.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })));

    { let (__tmp_0, __tmp_1) = { let __map_holder = names.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&1)) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().clone()))), true), None => (Rc::new(RefCell::new(Some(String::new()))), false) } }; let __moved_tmp_0 = { let mut __guard = __tmp_0.borrow_mut(); __guard.take() }; *name.borrow_mut() = __moved_tmp_0; ok = __tmp_1; };
    println!("{} {} {} {}", format!("{}", "string into variables".to_string()), format!("{}", { let __v = (*name.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", ok), format!("{}", { let __map_holder = names.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&1)).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()) }));
    { let (__tmp_0, __tmp_1) = { let __map_holder = names.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&2)) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().clone()))), true), None => (Rc::new(RefCell::new(Some(String::new()))), false) } }; let __moved_tmp_0 = { let mut __guard = __tmp_0.borrow_mut(); __guard.take() }; *name.borrow_mut() = __moved_tmp_0; ok = __tmp_1; };
    println!("{} {} {} {}", format!("{}", "missing key into variables".to_string()), format!("{}", (*name.borrow().as_ref().unwrap()).clone() == "".to_string()), format!("{}", ok), format!("{}", { let __map_holder = names.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&1)).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()) }));

    let mut points = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Point>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(1))), ..Default::default() }))))]))));
    let (mut p, mut ok) = { let __map_holder = points.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().__go_value_clone()))), true), None => (Rc::new(RefCell::new(Some(Default::default()))), false) } };
    { let new_val = 9; *(*p.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    println!("{} {} {} {}", format!("{}", "struct".to_string()), format!("{}", ok), format!("{}", (*{ let __map_holder = points.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.borrow().as_ref().unwrap().__go_value_clone()).unwrap_or_else(|| Default::default()) }.x.borrow().as_ref().unwrap())), format!("{}", (*(*p.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())));

    let mut q: Rc<RefCell<Option<Point>>> = Rc::new(RefCell::new(Some(Default::default())));
    { let (__tmp_0, __tmp_1) = { let __map_holder = points.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().__go_value_clone()))), true), None => (Rc::new(RefCell::new(Some(Default::default()))), false) } }; let __moved_tmp_0 = { let mut __guard = __tmp_0.borrow_mut(); __guard.take() }; *q.borrow_mut() = __moved_tmp_0; *found.borrow_mut() = Some(__tmp_1); };
    { let new_val = 8; *(*q.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    println!("{} {} {} {}", format!("{}", "struct into variables".to_string()), format!("{}", { let __v = (*found.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", (*{ let __map_holder = points.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.borrow().as_ref().unwrap().__go_value_clone()).unwrap_or_else(|| Default::default()) }.x.borrow().as_ref().unwrap())), format!("{}", (*(*q.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())));

    {
        let (mut r, mut ok) = { let __map_holder = points.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().__go_value_clone()))), true), None => (Rc::new(RefCell::new(Some(Default::default()))), false) } };;
        if ok {
            { let new_val = 7; *(*r.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };;
            println!("{} {} {}", format!("{}", "struct in an if".to_string()), format!("{}", (*{ let __map_holder = points.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.borrow().as_ref().unwrap().__go_value_clone()).unwrap_or_else(|| Default::default()) }.x.borrow().as_ref().unwrap())), format!("{}", (*(*r.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())));;
        }
    }

    let (mut missing, mut ok) = { let __map_holder = points.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"zzz".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().__go_value_clone()))), true), None => (Rc::new(RefCell::new(Some(Default::default()))), false) } };
    { let new_val = 5; *(*missing.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    println!("{} {} {} {}", format!("{}", "struct, missing key".to_string()), format!("{}", ok), format!("{}", (*(*missing.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())), format!("{}", ({ let __len = (*points.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })));

    let mut reg = Rc::new(RefCell::new(Some(Registry { by_name: Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Point>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(1))), ..Default::default() }))))])))), counts: Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i64>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(1i64))))])))), ..Default::default() })));
    let (mut rp, mut ok) = { let __map_holder = (*reg.borrow().as_ref().unwrap()).by_name.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().__go_value_clone()))), true), None => (Rc::new(RefCell::new(Some(Default::default()))), false) } };
    { let new_val = 7; *(*rp.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    let (mut c, mut ok2) = { let __map_holder = (*reg.borrow().as_ref().unwrap()).counts.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().clone()))), true), None => (Rc::new(RefCell::new(Some(0))), false) } };
    { let mut guard = c.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    println!("{} {} {} {} {} {} {}", format!("{}", "map in a field".to_string()), format!("{}", ok), format!("{}", ok2), format!("{}", (*{ let __map_holder = (*reg.borrow().as_ref().unwrap()).by_name.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.borrow().as_ref().unwrap().__go_value_clone()).unwrap_or_else(|| Default::default()) }.x.borrow().as_ref().unwrap())), format!("{}", (*(*rp.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())), format!("{}", { let __map_holder = (*reg.borrow().as_ref().unwrap()).counts.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", { let __v = (*c.borrow().as_ref().unwrap()).clone(); __v }));

    let mut c2: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));
    { let (__tmp_0, __tmp_1) = { let __map_holder = (*reg.borrow().as_ref().unwrap()).counts.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().clone()))), true), None => (Rc::new(RefCell::new(Some(0))), false) } }; let __moved_tmp_0 = { let mut __guard = __tmp_0.borrow_mut(); __guard.take() }; *c2.borrow_mut() = __moved_tmp_0; ok = __tmp_1; };
    { let __rhs = 5; let mut guard = c2.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    println!("{} {} {} {}", format!("{}", "map in a field into variables".to_string()), format!("{}", ok), format!("{}", { let __map_holder = (*reg.borrow().as_ref().unwrap()).counts.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", { let __v = (*c2.borrow().as_ref().unwrap()).clone(); __v }));

    let mut sc = Rc::new(RefCell::new(Some(Scores(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i64>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(1i64))))]))))))));
    let (mut v, mut ok) = { let __map_holder = { let __named_map = (*sc.borrow().as_ref().unwrap()).0.clone(); __named_map }.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().clone()))), true), None => (Rc::new(RefCell::new(Some(0))), false) } };
    { let new_val = 3; *v.borrow_mut() = Some(new_val); };
    println!("{} {} {} {}", format!("{}", "named map type".to_string()), format!("{}", ok), format!("{}", { let __map_holder = { let __named_map = (*sc.borrow().as_ref().unwrap()).0.clone(); __named_map }.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", { let __v = (*v.borrow().as_ref().unwrap()).clone(); __v }));

    let (_, mut has) = { let __map_holder = { let __named_map = (*sc.borrow().as_ref().unwrap()).0.clone(); __named_map }.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"zzz".to_string())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Rc::new(RefCell::new(Some(0))), false) } };
    let (mut z, _) = { let __map_holder = { let __named_map = (*sc.borrow().as_ref().unwrap()).0.clone(); __named_map }.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().clone()))), true), None => (Rc::new(RefCell::new(Some(0))), false) } };
    { let new_val = 4; *z.borrow_mut() = Some(new_val); };
    println!("{} {} {} {}", format!("{}", "blank results".to_string()), format!("{}", has), format!("{}", { let __map_holder = { let __named_map = (*sc.borrow().as_ref().unwrap()).0.clone(); __named_map }.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", { let __v = (*z.borrow().as_ref().unwrap()).clone(); __v }));

    let mut nested = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i64>>>>::from([("x".to_string(), Rc::new(RefCell::new(Some(1i64))))])))))]))));
    let (mut x, mut ok) = { let __map_holder = { let __map_holder = nested.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"x".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().clone()))), true), None => (Rc::new(RefCell::new(Some(0))), false) } };
    { let new_val = 99; *x.borrow_mut() = Some(new_val); };
    println!("{} {} {} {}", format!("{}", "nested lookup".to_string()), format!("{}", ok), format!("{}", { let __map_holder = { let __map_holder = nested.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"x".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v }));

        // A pointer, a slice and a map are shared with the map entry, as in Go.
    let mut ptrs = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Point>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(1))), ..Default::default() }))).clone())]))));
    let (mut pp, mut ok) = { let __map_holder = ptrs.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } };
    { let new_val = 9; *(*pp.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    println!("{} {} {} {}", format!("{}", "pointer value".to_string()), format!("{}", ok), format!("{}", (*(*{ let __map_holder = ptrs.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())), format!("{}", (*(*pp.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())));

    let mut pq: Rc<RefCell<Option<Point>>> = Rc::new(RefCell::new(None));
    { let (__tmp_0, __tmp_1) = { let __map_holder = ptrs.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } }; pq = __tmp_0.clone(); ok = __tmp_1; };
    { let new_val = 10; *(*pq.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    println!("{} {} {} {}", format!("{}", "pointer value into variables".to_string()), format!("{}", ok), format!("{}", (*(*{ let __map_holder = ptrs.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())), format!("{}", (*(*pq.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())));

    let mut lists = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Vec<i64>>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(vec![1i64, 2i64]))))]))));
    let (mut l, mut ok) = { let __map_holder = lists.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } };
    let mut l = { let __slice_header = GoSliceHeader::go_slice_header(&l); Rc::new(RefCell::new(__slice_header)) };
    { let __idx = (0) as usize; let __rhs = 50; (*l.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {} {} {}", format!("{}", "slice value".to_string()), format!("{}", ok), format!("{}", format_slice(&{ let __map_holder = lists.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) })), format!("{}", format_slice(&l)));

    let mut l2: Rc<RefCell<Option<Vec<i64>>>> = Rc::new(RefCell::new(None));
    { let (__tmp_0, __tmp_1) = { let __map_holder = lists.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } }; l2 = __tmp_0; ok = __tmp_1; };
    { let __idx = (1) as usize; let __rhs = 60; (*l2.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {} {} {}", format!("{}", "slice value into variables".to_string()), format!("{}", ok), format!("{}", format_slice(&{ let __map_holder = lists.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) })), format!("{}", format_slice(&l2)));

    let (mut inner, mut ok) = { let __map_holder = nested.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } };
    { let __map_key = "x".to_string(); let __map_value = Rc::new(RefCell::new(Some(2))); (*inner.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{} {} {} {}", format!("{}", "map value".to_string()), format!("{}", ok), format!("{}", { let __map_holder = { let __map_holder = nested.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"x".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", { let __map_holder = inner.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"x".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));

    let mut inner2: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>> = Rc::new(RefCell::new(None));
    { let (__tmp_0, __tmp_1) = { let __map_holder = nested.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } }; inner2 = __tmp_0; ok = __tmp_1; };
    { let __map_key = "x".to_string(); let __map_value = Rc::new(RefCell::new(Some(3))); (*inner2.borrow_mut().as_mut().unwrap_or_else(|| panic!("assignment to entry in nil map"))).insert(__map_key, __map_value); };
    println!("{} {} {} {}", format!("{}", "map value into variables".to_string()), format!("{}", ok), format!("{}", { let __map_holder = { let __map_holder = nested.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"x".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", { let __map_holder = inner2.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"x".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));
}