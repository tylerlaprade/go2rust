#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use std::cmp::Ord;
use std::collections::BTreeMap;
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

fn format_map<K: Display + Ord + Clone, V>(map: &Arc<Mutex<Option<BTreeMap<K, Arc<Mutex<Option<V>>>>>>>) -> String
where
    V: Display,
{
    let guard = map.lock().unwrap();
    if let Some(ref m) = *guard {
        let mut items: Vec<_> = m.iter().collect();
        items.sort_by_key(|(k, _)| (*k).clone());

        let formatted: Vec<String> = items
            .into_iter()
            .map(|(k, v)| {
                let v_guard = v.lock().unwrap();
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
    pub val: Arc<Mutex<Option<i64>>>,
    pub left: Arc<Mutex<Option<Node>>>,
    pub right: Arc<Mutex<Option<Node>>>,
}

impl Node {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.val.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
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
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        let __go_default_1_1 = Arc::new(Mutex::new(None));
        Self {
            val: __go_default_0_0,
            left: __go_default_1_0,
            right: __go_default_1_1,
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.val.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { let __guard = self.left.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_2 = format!("{}", { let __guard = self.right.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


#[derive(Debug, Clone, Default)]
pub struct Tree {
    pub root: Arc<Mutex<Option<Node>>>,
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
        let __go_fmt_0 = format!("{}", { let __guard = self.root.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone)]
pub struct LNode {
    pub v: Arc<Mutex<Option<i64>>>,
    pub next: Arc<Mutex<Option<LNode>>>,
}

impl LNode {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.v.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.next.clone();
        Self {
            v: __go_clone_0_0,
            next: __go_clone_1_0,
        }
    }
}


impl Default for LNode {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        Self {
            v: __go_default_0_0,
            next: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for LNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.v.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { let __guard = self.next.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone, Default)]
pub struct List {
    pub head: Arc<Mutex<Option<LNode>>>,
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
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}


#[derive(Clone)]
pub struct Holder {
    pub slot: Arc<Mutex<Option<__ptrSlot_LNode>>>,
}

impl Holder {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.slot.lock().unwrap(); Arc::new(Mutex::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        Self {
            slot: __go_clone_0_0,
        }
    }
}


impl Default for Holder {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(__ptrSlot_LNode::default())));
        Self {
            slot: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.slot.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Clone, Default)]
pub struct __ptrSlot_LNode {
    pub get: Arc<Mutex<Option<Arc<dyn Fn() -> Arc<Mutex<Option<LNode>>> + Send + Sync>>>>,
    pub set: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<LNode>>>) -> () + Send + Sync>>>>,
}

impl __ptrSlot_LNode {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.get.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.set.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
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
    pub get: Arc<Mutex<Option<Arc<dyn Fn() -> Arc<Mutex<Option<Node>>> + Send + Sync>>>>,
    pub set: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Node>>>) -> () + Send + Sync>>>>,
}

impl __ptrSlot_Node {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.get.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.set.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
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
    pub get: Arc<Mutex<Option<Arc<dyn Fn() -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> + Send + Sync>>>>,
    pub set: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>) -> () + Send + Sync>>>>,
}

impl __ptrSlot_map_string_int {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.get.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.set.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
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
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        let mut s = Arc::new(Mutex::new(Some("[".to_string())));
        let mut n = self.head.clone();
    while { let __nil_result = (*n.lock().unwrap()).is_some(); __nil_result } {
        if { let __left = n.clone(); let __right = self.head.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right) || { let __left_id = __left.lock().unwrap().as_ref().map(|__v| Arc::as_ptr(&__v.v) as *const () as usize); let __right_id = __right.lock().unwrap().as_ref().map(|__v| Arc::as_ptr(&__v.v) as *const () as usize); __left_id.is_some() && __left_id == __right_id }; !__eq } {
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&" ".to_string()); };
    }
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&{ let __s = Arc::new(Mutex::new(Some(format!("{}", format!("{}", (*{ let __field = (*n.lock().unwrap().as_ref().unwrap()).v.clone(); __field }.lock().unwrap().as_ref().unwrap())))))); let __value = (*__s.lock().unwrap().as_ref().unwrap()).clone(); __value }); };
        { let new_val = (*n.lock().unwrap().as_ref().unwrap()).next.clone(); n = new_val; };
    }
        return Arc::new(Mutex::new(Some(format!("{}{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }, "]".to_string()))));
    }
}

pub fn insert(mut pp: Arc<Mutex<Option<__ptrSlot_Node>>>, v: Arc<Mutex<Option<i64>>>) {
    while { let __nil_result = (*{ let __f_holder = (*pp.lock().unwrap().as_ref().unwrap()).get.clone(); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<Node>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }.lock().unwrap()).is_some(); __nil_result } {
        if {
            let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v };
            let __tmp_y = (*(*({ let __f_holder = (*pp.lock().unwrap().as_ref().unwrap()).get.clone(); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<Node>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }).lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_ref().unwrap());
            __tmp_x < __tmp_y
        } {
        { let new_val = { let __f_holder = Arc::new(Mutex::new(Some(Arc::new(move |__slot_o1: Arc<Mutex<Option<Node>>>| -> Arc<Mutex<Option<__ptrSlot_Node>>> {
        let __slot_o1_closure_clone = __slot_o1.clone(); return Arc::new(Mutex::new(Some(__ptrSlot_Node {
            get: Arc::new(Mutex::new(Some({ let __slot_o1_closure_clone_closure_clone = __slot_o1_closure_clone.clone(); Arc::new(move || -> Arc<Mutex<Option<Node>>> {
        (*__slot_o1_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).left.clone()
    }) as Arc<dyn Fn() -> Arc<Mutex<Option<Node>>> + Send + Sync> }))),
            set: Arc::new(Mutex::new(Some({ let __slot_o1_closure_clone_closure_clone = __slot_o1_closure_clone.clone(); Arc::new(move |__slot_v2: Arc<Mutex<Option<Node>>>| {
        { let new_val = __slot_v2.clone(); (*__slot_o1_closure_clone_closure_clone.lock().unwrap().as_mut().unwrap()).left = new_val; };
    }) as Arc<dyn Fn(Arc<Mutex<Option<Node>>>) -> () + Send + Sync> }))),
            ..Default::default()
        })));
    }) as Arc<dyn Fn(Arc<Mutex<Option<Node>>>) -> Arc<Mutex<Option<__ptrSlot_Node>>> + Send + Sync>))); let __f: Arc<dyn Fn(Arc<Mutex<Option<Node>>>) -> Arc<Mutex<Option<__ptrSlot_Node>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)({ let __f_holder = (*pp.lock().unwrap().as_ref().unwrap()).get.clone(); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<Node>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }) }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *pp.lock().unwrap() = __moved_val; };
    } else {
        { let new_val = { let __f_holder = Arc::new(Mutex::new(Some(Arc::new(move |__slot_o3: Arc<Mutex<Option<Node>>>| -> Arc<Mutex<Option<__ptrSlot_Node>>> {
        let __slot_o3_closure_clone = __slot_o3.clone(); return Arc::new(Mutex::new(Some(__ptrSlot_Node {
            get: Arc::new(Mutex::new(Some({ let __slot_o3_closure_clone_closure_clone = __slot_o3_closure_clone.clone(); Arc::new(move || -> Arc<Mutex<Option<Node>>> {
        (*__slot_o3_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).right.clone()
    }) as Arc<dyn Fn() -> Arc<Mutex<Option<Node>>> + Send + Sync> }))),
            set: Arc::new(Mutex::new(Some({ let __slot_o3_closure_clone_closure_clone = __slot_o3_closure_clone.clone(); Arc::new(move |__slot_v4: Arc<Mutex<Option<Node>>>| {
        { let new_val = __slot_v4.clone(); (*__slot_o3_closure_clone_closure_clone.lock().unwrap().as_mut().unwrap()).right = new_val; };
    }) as Arc<dyn Fn(Arc<Mutex<Option<Node>>>) -> () + Send + Sync> }))),
            ..Default::default()
        })));
    }) as Arc<dyn Fn(Arc<Mutex<Option<Node>>>) -> Arc<Mutex<Option<__ptrSlot_Node>>> + Send + Sync>))); let __f: Arc<dyn Fn(Arc<Mutex<Option<Node>>>) -> Arc<Mutex<Option<__ptrSlot_Node>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)({ let __f_holder = (*pp.lock().unwrap().as_ref().unwrap()).get.clone(); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<Node>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }) }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *pp.lock().unwrap() = __moved_val; };
    }
    }
    { let __f_holder = (*pp.lock().unwrap().as_ref().unwrap()).set.clone(); let __f: Arc<dyn Fn(Arc<Mutex<Option<Node>>>) -> () + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)(Arc::new(Mutex::new(Some(Node { val: Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })))) };
}

pub fn inorder(n: Arc<Mutex<Option<Node>>>, out: Arc<Mutex<Option<Vec<i64>>>>) {
    if { let __nil_result = (*n.lock().unwrap()).is_none(); __nil_result } {
        return;
    }
    inorder({ let __field = (*n.lock().unwrap().as_ref().unwrap()).left.clone(); __field }, out.clone());
    { let new_val = { let __append_target = out.clone(); let __append_value = { let __selector_holder = (*n.lock().unwrap().as_ref().unwrap()).val.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }; let mut __appended: Vec<i64> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) }; let __cloned_val = { let __guard = new_val.lock().unwrap(); (*__guard).clone() }; *out.lock().unwrap() = __cloned_val; };
    inorder({ let __field = (*n.lock().unwrap().as_ref().unwrap()).right.clone(); __field }, out.clone());
}

pub fn min_link(mut pp: Arc<Mutex<Option<__ptrSlot_Node>>>) -> Arc<Mutex<Option<__ptrSlot_Node>>> {
    while { let __nil_target = (*({ let __f_holder = (*pp.lock().unwrap().as_ref().unwrap()).get.clone(); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<Node>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }).lock().unwrap().as_ref().unwrap()).left.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = { let __f_holder = Arc::new(Mutex::new(Some(Arc::new(move |__slot_o5: Arc<Mutex<Option<Node>>>| -> Arc<Mutex<Option<__ptrSlot_Node>>> {
        let __slot_o5_closure_clone = __slot_o5.clone(); return Arc::new(Mutex::new(Some(__ptrSlot_Node {
            get: Arc::new(Mutex::new(Some({ let __slot_o5_closure_clone_closure_clone = __slot_o5_closure_clone.clone(); Arc::new(move || -> Arc<Mutex<Option<Node>>> {
        (*__slot_o5_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).left.clone()
    }) as Arc<dyn Fn() -> Arc<Mutex<Option<Node>>> + Send + Sync> }))),
            set: Arc::new(Mutex::new(Some({ let __slot_o5_closure_clone_closure_clone = __slot_o5_closure_clone.clone(); Arc::new(move |__slot_v6: Arc<Mutex<Option<Node>>>| {
        { let new_val = __slot_v6.clone(); (*__slot_o5_closure_clone_closure_clone.lock().unwrap().as_mut().unwrap()).left = new_val; };
    }) as Arc<dyn Fn(Arc<Mutex<Option<Node>>>) -> () + Send + Sync> }))),
            ..Default::default()
        })));
    }) as Arc<dyn Fn(Arc<Mutex<Option<Node>>>) -> Arc<Mutex<Option<__ptrSlot_Node>>> + Send + Sync>))); let __f: Arc<dyn Fn(Arc<Mutex<Option<Node>>>) -> Arc<Mutex<Option<__ptrSlot_Node>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)({ let __f_holder = (*pp.lock().unwrap().as_ref().unwrap()).get.clone(); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<Node>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }) }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *pp.lock().unwrap() = __moved_val; };
    }
    return { let __owned = pp.lock().unwrap().as_ref().unwrap().__go_value_clone(); Arc::new(Mutex::new(Some(__owned))) };
}

pub fn remove(l: Arc<Mutex<Option<List>>>, v: Arc<Mutex<Option<i64>>>) -> bool {
    let mut pp = { let __f_holder = Arc::new(Mutex::new(Some(Arc::new(move |__slot_o7: Arc<Mutex<Option<List>>>| -> Arc<Mutex<Option<__ptrSlot_LNode>>> {
        let __slot_o7_closure_clone = __slot_o7.clone(); return Arc::new(Mutex::new(Some(__ptrSlot_LNode {
            get: Arc::new(Mutex::new(Some({ let __slot_o7_closure_clone_closure_clone = __slot_o7_closure_clone.clone(); Arc::new(move || -> Arc<Mutex<Option<LNode>>> {
        (*__slot_o7_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).head.clone()
    }) as Arc<dyn Fn() -> Arc<Mutex<Option<LNode>>> + Send + Sync> }))),
            set: Arc::new(Mutex::new(Some({ let __slot_o7_closure_clone_closure_clone = __slot_o7_closure_clone.clone(); Arc::new(move |__slot_v8: Arc<Mutex<Option<LNode>>>| {
        { let new_val = __slot_v8.clone(); (*__slot_o7_closure_clone_closure_clone.lock().unwrap().as_mut().unwrap()).head = new_val; };
    }) as Arc<dyn Fn(Arc<Mutex<Option<LNode>>>) -> () + Send + Sync> }))),
            ..Default::default()
        })));
    }) as Arc<dyn Fn(Arc<Mutex<Option<List>>>) -> Arc<Mutex<Option<__ptrSlot_LNode>>> + Send + Sync>))); let __f: Arc<dyn Fn(Arc<Mutex<Option<List>>>) -> Arc<Mutex<Option<__ptrSlot_LNode>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)(l.clone()) };
    while { let __nil_result = (*{ let __f_holder = (*pp.lock().unwrap().as_ref().unwrap()).get.clone(); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<LNode>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }.lock().unwrap()).is_some(); __nil_result } {
        if {
            let __tmp_x = (*(*({ let __f_holder = (*pp.lock().unwrap().as_ref().unwrap()).get.clone(); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<LNode>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }).lock().unwrap().as_ref().unwrap()).v.lock().unwrap().as_ref().unwrap());
            let __tmp_y = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v };
            __tmp_x == __tmp_y
        } {
        { let __f_holder = (*pp.lock().unwrap().as_ref().unwrap()).set.clone(); let __f: Arc<dyn Fn(Arc<Mutex<Option<LNode>>>) -> () + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)({ let __field = (*({ let __f_holder = (*pp.lock().unwrap().as_ref().unwrap()).get.clone(); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<LNode>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }).lock().unwrap().as_ref().unwrap()).next.clone(); __field }) };
        return true;
    }
        { let new_val = { let __f_holder = Arc::new(Mutex::new(Some(Arc::new(move |__slot_o9: Arc<Mutex<Option<LNode>>>| -> Arc<Mutex<Option<__ptrSlot_LNode>>> {
        let __slot_o9_closure_clone = __slot_o9.clone(); return Arc::new(Mutex::new(Some(__ptrSlot_LNode {
            get: Arc::new(Mutex::new(Some({ let __slot_o9_closure_clone_closure_clone = __slot_o9_closure_clone.clone(); Arc::new(move || -> Arc<Mutex<Option<LNode>>> {
        (*__slot_o9_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).next.clone()
    }) as Arc<dyn Fn() -> Arc<Mutex<Option<LNode>>> + Send + Sync> }))),
            set: Arc::new(Mutex::new(Some({ let __slot_o9_closure_clone_closure_clone = __slot_o9_closure_clone.clone(); Arc::new(move |__slot_v10: Arc<Mutex<Option<LNode>>>| {
        { let new_val = __slot_v10.clone(); (*__slot_o9_closure_clone_closure_clone.lock().unwrap().as_mut().unwrap()).next = new_val; };
    }) as Arc<dyn Fn(Arc<Mutex<Option<LNode>>>) -> () + Send + Sync> }))),
            ..Default::default()
        })));
    }) as Arc<dyn Fn(Arc<Mutex<Option<LNode>>>) -> Arc<Mutex<Option<__ptrSlot_LNode>>> + Send + Sync>))); let __f: Arc<dyn Fn(Arc<Mutex<Option<LNode>>>) -> Arc<Mutex<Option<__ptrSlot_LNode>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)({ let __f_holder = (*pp.lock().unwrap().as_ref().unwrap()).get.clone(); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<LNode>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }) }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *pp.lock().unwrap() = __moved_val; };
    }
    false
}

pub fn push(pp: Arc<Mutex<Option<__ptrSlot_LNode>>>, v: Arc<Mutex<Option<i64>>>) {
    { let __f_holder = (*pp.lock().unwrap().as_ref().unwrap()).set.clone(); let __f: Arc<dyn Fn(Arc<Mutex<Option<LNode>>>) -> () + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)(Arc::new(Mutex::new(Some(LNode {
        v: Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        next: { let __f_holder = (*pp.lock().unwrap().as_ref().unwrap()).get.clone(); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<LNode>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }.clone(),
        ..Default::default()
    })))) };
}

pub fn add_count(m: Arc<Mutex<Option<__ptrSlot_map_string_int>>>, k: Arc<Mutex<Option<String>>>) {
    if { let __nil_result = (*{ let __f_holder = (*m.lock().unwrap().as_ref().unwrap()).get.clone(); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }.lock().unwrap()).is_none(); __nil_result } {
        { let __f_holder = (*m.lock().unwrap().as_ref().unwrap()).set.clone(); let __f: Arc<dyn Fn(Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>) -> () + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)(Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([]))))) };
    }
    { let __map_holder = ({ let __f_holder = (*m.lock().unwrap().as_ref().unwrap()).get.clone(); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }).clone(); let __key = ({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }); let mut __map_guard = __map_holder.lock().unwrap(); let __map = __map_guard.as_mut().unwrap(); let __entry = __map.entry(__key).or_insert_with(|| Arc::new(Mutex::new(Some(0)))); let mut __value = __entry.lock().unwrap(); * __value = Some((*__value.as_ref().unwrap()).clone() + 1); }
}

fn main() {
    let mut done = GoChannel::<bool>::new();
    let done_thread = done.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        done_thread.send(true);;;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    done.recv().unwrap_or_default();

    let mut root: Arc<Mutex<Option<Node>>> = Arc::new(Mutex::new(None));
    let mut __cell_root_1 = Arc::new(Mutex::new(Some(AnonymousStruct1 { v: root.clone() })));
    for v in vec![5i64, 3i64, 8i64, 1i64, 4i64].iter().copied() {
        let __cell_root_1_closure_clone = __cell_root_1.clone(); insert(
            Arc::new(Mutex::new(Some({ let __arg_value = __ptrSlot_Node {
                get: Arc::new(Mutex::new(Some({ let __cell_root_1_closure_clone_closure_clone = __cell_root_1_closure_clone.clone(); Arc::new(move || -> Arc<Mutex<Option<Node>>> {
        return (*__cell_root_1_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).v.clone();
    }) as Arc<dyn Fn() -> Arc<Mutex<Option<Node>>> + Send + Sync> }))),
                set: Arc::new(Mutex::new(Some({ let __cell_root_1_closure_clone_closure_clone = __cell_root_1_closure_clone.clone(); Arc::new(move |__slot_v11: Arc<Mutex<Option<Node>>>| {
        { let new_val = __slot_v11.clone(); (*__cell_root_1_closure_clone_closure_clone.lock().unwrap().as_mut().unwrap()).v = new_val; };
    }) as Arc<dyn Fn(Arc<Mutex<Option<Node>>>) -> () + Send + Sync> }))),
                ..Default::default()
            }; __arg_value }))),
            Arc::new(Mutex::new(Some(v.clone())))
        );
    }
    let mut vals: Arc<Mutex<Option<Vec<i64>>>> = Arc::new(Mutex::new(None));
    inorder({ let __field = (*__cell_root_1.lock().unwrap().as_ref().unwrap()).v.clone(); __field }, vals.clone());
    println!("{}", format!("{}", format_slice(&vals)));
    let __cell_root_1_closure_clone = __cell_root_1.clone(); let mut low = min_link(Arc::new(Mutex::new(Some({ let __arg_value = __ptrSlot_Node {
        get: Arc::new(Mutex::new(Some({ let __cell_root_1_closure_clone_closure_clone = __cell_root_1_closure_clone.clone(); Arc::new(move || -> Arc<Mutex<Option<Node>>> {
        return (*__cell_root_1_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).v.clone();
    }) as Arc<dyn Fn() -> Arc<Mutex<Option<Node>>> + Send + Sync> }))),
        set: Arc::new(Mutex::new(Some({ let __cell_root_1_closure_clone_closure_clone = __cell_root_1_closure_clone.clone(); Arc::new(move |__slot_v12: Arc<Mutex<Option<Node>>>| {
        { let new_val = __slot_v12.clone(); (*__cell_root_1_closure_clone_closure_clone.lock().unwrap().as_mut().unwrap()).v = new_val; };
    }) as Arc<dyn Fn(Arc<Mutex<Option<Node>>>) -> () + Send + Sync> }))),
        ..Default::default()
    }; __arg_value }))));
    println!("{} {}", format!("{}", "min".to_string()), format!("{}", (*(*({ let __f_holder = (*low.lock().unwrap().as_ref().unwrap()).get.clone(); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<Node>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }).lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_ref().unwrap())));
    { let __f_holder = (*low.lock().unwrap().as_ref().unwrap()).set.clone(); let __f: Arc<dyn Fn(Arc<Mutex<Option<Node>>>) -> () + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)({ let __field = (*({ let __f_holder = (*low.lock().unwrap().as_ref().unwrap()).get.clone(); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<Node>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }).lock().unwrap().as_ref().unwrap()).right.clone(); __field }) };
    { let __slice_header = None; *vals.lock().unwrap() = __slice_header; };
    inorder({ let __field = (*__cell_root_1.lock().unwrap().as_ref().unwrap()).v.clone(); __field }, vals.clone());
    println!("{} {}", format!("{}", "without min".to_string()), format!("{}", format_slice(&vals)));

    let mut t = Arc::new(Mutex::new(Some(Tree { root: Default::default() })));
    for v in vec![2i64, 1i64, 3i64].iter().copied() {
        insert(
            { let __f_holder = Arc::new(Mutex::new(Some(Arc::new(move |__slot_o13: Arc<Mutex<Option<Tree>>>| -> Arc<Mutex<Option<__ptrSlot_Node>>> {
        let __slot_o13_closure_clone = __slot_o13.clone(); return Arc::new(Mutex::new(Some(__ptrSlot_Node {
            get: Arc::new(Mutex::new(Some({ let __slot_o13_closure_clone_closure_clone = __slot_o13_closure_clone.clone(); Arc::new(move || -> Arc<Mutex<Option<Node>>> {
        return (*__slot_o13_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).root.clone();
    }) as Arc<dyn Fn() -> Arc<Mutex<Option<Node>>> + Send + Sync> }))),
            set: Arc::new(Mutex::new(Some({ let __slot_o13_closure_clone_closure_clone = __slot_o13_closure_clone.clone(); Arc::new(move |__slot_v14: Arc<Mutex<Option<Node>>>| {
        { let new_val = __slot_v14.clone(); (*__slot_o13_closure_clone_closure_clone.lock().unwrap().as_mut().unwrap()).root = new_val; };
    }) as Arc<dyn Fn(Arc<Mutex<Option<Node>>>) -> () + Send + Sync> }))),
            ..Default::default()
        })));
    }) as Arc<dyn Fn(Arc<Mutex<Option<Tree>>>) -> Arc<Mutex<Option<__ptrSlot_Node>>> + Send + Sync>))); let __f: Arc<dyn Fn(Arc<Mutex<Option<Tree>>>) -> Arc<Mutex<Option<__ptrSlot_Node>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)(t.clone()) },
            Arc::new(Mutex::new(Some(v.clone())))
        );
    }
    { let __slice_header = None; *vals.lock().unwrap() = __slice_header; };
    inorder({ let __field = (*t.lock().unwrap().as_ref().unwrap()).root.clone(); __field }, vals.clone());
    println!("{} {}", format!("{}", format_slice(&vals)), format!("{}", (*(*(*t.lock().unwrap().as_ref().unwrap()).root.lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_ref().unwrap())));

    let mut l = Arc::new(Mutex::new(Some(List { head: Default::default() })));
    let mut i = Arc::new(Mutex::new(Some(1i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x <= __tmp_y } {
        push(
            { let __f_holder = Arc::new(Mutex::new(Some(Arc::new(move |__slot_o15: Arc<Mutex<Option<List>>>| -> Arc<Mutex<Option<__ptrSlot_LNode>>> {
        let __slot_o15_closure_clone = __slot_o15.clone(); return Arc::new(Mutex::new(Some(__ptrSlot_LNode {
            get: Arc::new(Mutex::new(Some({ let __slot_o15_closure_clone_closure_clone = __slot_o15_closure_clone.clone(); Arc::new(move || -> Arc<Mutex<Option<LNode>>> {
        (*__slot_o15_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).head.clone()
    }) as Arc<dyn Fn() -> Arc<Mutex<Option<LNode>>> + Send + Sync> }))),
            set: Arc::new(Mutex::new(Some({ let __slot_o15_closure_clone_closure_clone = __slot_o15_closure_clone.clone(); Arc::new(move |__slot_v16: Arc<Mutex<Option<LNode>>>| {
        { let new_val = __slot_v16.clone(); (*__slot_o15_closure_clone_closure_clone.lock().unwrap().as_mut().unwrap()).head = new_val; };
    }) as Arc<dyn Fn(Arc<Mutex<Option<LNode>>>) -> () + Send + Sync> }))),
            ..Default::default()
        })));
    }) as Arc<dyn Fn(Arc<Mutex<Option<List>>>) -> Arc<Mutex<Option<__ptrSlot_LNode>>> + Send + Sync>))); let __f: Arc<dyn Fn(Arc<Mutex<Option<List>>>) -> Arc<Mutex<Option<__ptrSlot_LNode>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)(l.clone()) },
            Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
        );
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    println!("{}", format!("{}", { let __text = match l.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    println!("{} {} {} {}", format!("{}", remove(l.clone(), Arc::new(Mutex::new(Some(3))))), format!("{}", remove(l.clone(), Arc::new(Mutex::new(Some(4))))), format!("{}", remove(l.clone(), Arc::new(Mutex::new(Some(9))))), format!("{}", { let __text = match l.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

    let mut head: Arc<Mutex<Option<LNode>>> = Arc::new(Mutex::new(None));
    let mut __cell_head_2 = Arc::new(Mutex::new(Some(AnonymousStruct2 { v: head.clone() })));
    let __cell_head_2_closure_clone = __cell_head_2.clone(); push(
        Arc::new(Mutex::new(Some({ let __arg_value = __ptrSlot_LNode {
            get: Arc::new(Mutex::new(Some({ let __cell_head_2_closure_clone_closure_clone = __cell_head_2_closure_clone.clone(); Arc::new(move || -> Arc<Mutex<Option<LNode>>> {
        return (*__cell_head_2_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).v.clone();
    }) as Arc<dyn Fn() -> Arc<Mutex<Option<LNode>>> + Send + Sync> }))),
            set: Arc::new(Mutex::new(Some({ let __cell_head_2_closure_clone_closure_clone = __cell_head_2_closure_clone.clone(); Arc::new(move |__slot_v17: Arc<Mutex<Option<LNode>>>| {
        { let new_val = __slot_v17.clone(); (*__cell_head_2_closure_clone_closure_clone.lock().unwrap().as_mut().unwrap()).v = new_val; };
    }) as Arc<dyn Fn(Arc<Mutex<Option<LNode>>>) -> () + Send + Sync> }))),
            ..Default::default()
        }; __arg_value }))),
        Arc::new(Mutex::new(Some(1)))
    );
    let __cell_head_2_closure_clone = __cell_head_2.clone(); push(
        Arc::new(Mutex::new(Some({ let __arg_value = __ptrSlot_LNode {
            get: Arc::new(Mutex::new(Some({ let __cell_head_2_closure_clone_closure_clone = __cell_head_2_closure_clone.clone(); Arc::new(move || -> Arc<Mutex<Option<LNode>>> {
        return (*__cell_head_2_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).v.clone();
    }) as Arc<dyn Fn() -> Arc<Mutex<Option<LNode>>> + Send + Sync> }))),
            set: Arc::new(Mutex::new(Some({ let __cell_head_2_closure_clone_closure_clone = __cell_head_2_closure_clone.clone(); Arc::new(move |__slot_v18: Arc<Mutex<Option<LNode>>>| {
        { let new_val = __slot_v18.clone(); (*__cell_head_2_closure_clone_closure_clone.lock().unwrap().as_mut().unwrap()).v = new_val; };
    }) as Arc<dyn Fn(Arc<Mutex<Option<LNode>>>) -> () + Send + Sync> }))),
            ..Default::default()
        }; __arg_value }))),
        Arc::new(Mutex::new(Some(2)))
    );
    println!("{}", format!("{}", { let __text = match Arc::new(Mutex::new(Some(List { head: { let __field = (*__cell_head_2.lock().unwrap().as_ref().unwrap()).v.clone(); __field }, ..Default::default() }))).lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

    let mut arr = Arc::new(Mutex::new(Some([Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None))])));
    push(
        { let __f_holder = Arc::new(Mutex::new(Some(Arc::new(move |__slot_s19: Arc<Mutex<Option<[Arc<Mutex<Option<LNode>>>; 2]>>>, __slot_i20: Arc<Mutex<Option<i64>>>| -> Arc<Mutex<Option<__ptrSlot_LNode>>> {
        let __slot_i20_closure_clone = __slot_i20.clone(); let __slot_s19_closure_clone = __slot_s19.clone(); return Arc::new(Mutex::new(Some(__ptrSlot_LNode {
            get: Arc::new(Mutex::new(Some({ let __slot_i20_closure_clone_closure_clone = __slot_i20_closure_clone.clone(); let __slot_s19_closure_clone_closure_clone = __slot_s19_closure_clone.clone(); Arc::new(move || -> Arc<Mutex<Option<LNode>>> {
        {
            let __seq = { let __seq_holder = __slot_s19_closure_clone_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); std::mem::drop(__seq_guard); __cloned };
            __seq[({ let __v = (*__slot_i20_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone()
        }
    }) as Arc<dyn Fn() -> Arc<Mutex<Option<LNode>>> + Send + Sync> }))),
            set: Arc::new(Mutex::new(Some({ let __slot_i20_closure_clone_closure_clone = __slot_i20_closure_clone.clone(); let __slot_s19_closure_clone_closure_clone = __slot_s19_closure_clone.clone(); Arc::new(move |__slot_v21: Arc<Mutex<Option<LNode>>>| {
        { let __idx = ({ let __v = (*__slot_i20_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __rhs = __slot_v21.clone(); (*__slot_s19_closure_clone_closure_clone.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    }) as Arc<dyn Fn(Arc<Mutex<Option<LNode>>>) -> () + Send + Sync> }))),
            ..Default::default()
        })));
    }) as Arc<dyn Fn(Arc<Mutex<Option<[Arc<Mutex<Option<LNode>>>; 2]>>>, Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<__ptrSlot_LNode>>> + Send + Sync>))); let __f: Arc<dyn Fn(Arc<Mutex<Option<[Arc<Mutex<Option<LNode>>>; 2]>>>, Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<__ptrSlot_LNode>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)(arr.clone(), Arc::new(Mutex::new(Some(0)))) },
        Arc::new(Mutex::new(Some(7)))
    );
    push(
        { let __f_holder = Arc::new(Mutex::new(Some(Arc::new(move |__slot_s22: Arc<Mutex<Option<[Arc<Mutex<Option<LNode>>>; 2]>>>, __slot_i23: Arc<Mutex<Option<i64>>>| -> Arc<Mutex<Option<__ptrSlot_LNode>>> {
        let __slot_i23_closure_clone = __slot_i23.clone(); let __slot_s22_closure_clone = __slot_s22.clone(); return Arc::new(Mutex::new(Some(__ptrSlot_LNode {
            get: Arc::new(Mutex::new(Some({ let __slot_i23_closure_clone_closure_clone = __slot_i23_closure_clone.clone(); let __slot_s22_closure_clone_closure_clone = __slot_s22_closure_clone.clone(); Arc::new(move || -> Arc<Mutex<Option<LNode>>> {
        {
            let __seq = { let __seq_holder = __slot_s22_closure_clone_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); std::mem::drop(__seq_guard); __cloned };
            __seq[({ let __v = (*__slot_i23_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone()
        }
    }) as Arc<dyn Fn() -> Arc<Mutex<Option<LNode>>> + Send + Sync> }))),
            set: Arc::new(Mutex::new(Some({ let __slot_i23_closure_clone_closure_clone = __slot_i23_closure_clone.clone(); let __slot_s22_closure_clone_closure_clone = __slot_s22_closure_clone.clone(); Arc::new(move |__slot_v24: Arc<Mutex<Option<LNode>>>| {
        { let __idx = ({ let __v = (*__slot_i23_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __rhs = __slot_v24.clone(); (*__slot_s22_closure_clone_closure_clone.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    }) as Arc<dyn Fn(Arc<Mutex<Option<LNode>>>) -> () + Send + Sync> }))),
            ..Default::default()
        })));
    }) as Arc<dyn Fn(Arc<Mutex<Option<[Arc<Mutex<Option<LNode>>>; 2]>>>, Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<__ptrSlot_LNode>>> + Send + Sync>))); let __f: Arc<dyn Fn(Arc<Mutex<Option<[Arc<Mutex<Option<LNode>>>; 2]>>>, Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<__ptrSlot_LNode>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)(arr.clone(), Arc::new(Mutex::new(Some(0)))) },
        Arc::new(Mutex::new(Some(8)))
    );
    println!("{} {}", format!("{}", { let __text = match Arc::new(Mutex::new(Some(List {
        head: { let __seq = { let __seq_holder = arr.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone(),
        ..Default::default()
    }))).lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __nil_result = (*{ let __seq = { let __seq_holder = arr.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.lock().unwrap()).is_none(); __nil_result }));

    let mut links = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None))])));
    push(
        { let __f_holder = Arc::new(Mutex::new(Some(Arc::new(move |__slot_s25: Arc<Mutex<Option<Vec<Arc<Mutex<Option<LNode>>>>>>>, __slot_i26: Arc<Mutex<Option<i64>>>| -> Arc<Mutex<Option<__ptrSlot_LNode>>> {
        let mut __slot_s25 = Arc::new(Mutex::new((*__slot_s25.lock().unwrap()).clone()));
        let __slot_i26_closure_clone = __slot_i26.clone(); let __slot_s25_closure_clone = __slot_s25.clone(); return Arc::new(Mutex::new(Some(__ptrSlot_LNode {
            get: Arc::new(Mutex::new(Some({ let __slot_i26_closure_clone_closure_clone = __slot_i26_closure_clone.clone(); let __slot_s25_closure_clone_closure_clone = __slot_s25_closure_clone.clone(); Arc::new(move || -> Arc<Mutex<Option<LNode>>> {
        {
            let __seq = { let __seq_holder = __slot_s25_closure_clone_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned };
            __seq[({ let __v = (*__slot_i26_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone()
        }
    }) as Arc<dyn Fn() -> Arc<Mutex<Option<LNode>>> + Send + Sync> }))),
            set: Arc::new(Mutex::new(Some({ let __slot_i26_closure_clone_closure_clone = __slot_i26_closure_clone.clone(); let __slot_s25_closure_clone_closure_clone = __slot_s25_closure_clone.clone(); Arc::new(move |__slot_v27: Arc<Mutex<Option<LNode>>>| {
        { let __idx = ({ let __v = (*__slot_i26_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __rhs = __slot_v27.clone(); (*__slot_s25_closure_clone_closure_clone.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    }) as Arc<dyn Fn(Arc<Mutex<Option<LNode>>>) -> () + Send + Sync> }))),
            ..Default::default()
        })));
    }) as Arc<dyn Fn(Arc<Mutex<Option<Vec<Arc<Mutex<Option<LNode>>>>>>>, Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<__ptrSlot_LNode>>> + Send + Sync>))); let __f: Arc<dyn Fn(Arc<Mutex<Option<Vec<Arc<Mutex<Option<LNode>>>>>>>, Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<__ptrSlot_LNode>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)(links.clone(), Arc::new(Mutex::new(Some(1)))) },
        Arc::new(Mutex::new(Some(6)))
    );
    println!("{} {}", format!("{}", { let __nil_result = (*{ let __seq = { let __seq_holder = links.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap()).is_none(); __nil_result }), format!("{}", (*(*{ let __seq = { let __seq_holder = links.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.lock().unwrap().as_ref().unwrap()).v.lock().unwrap().as_ref().unwrap())));

    let mut first = { let __f_holder = Arc::new(Mutex::new(Some(Arc::new(move |__slot_s28: Arc<Mutex<Option<Vec<Arc<Mutex<Option<LNode>>>>>>>, __slot_i29: Arc<Mutex<Option<i64>>>| -> Arc<Mutex<Option<__ptrSlot_LNode>>> {
        let mut __slot_s28 = Arc::new(Mutex::new((*__slot_s28.lock().unwrap()).clone()));
        let __slot_i29_closure_clone = __slot_i29.clone(); let __slot_s28_closure_clone = __slot_s28.clone(); return Arc::new(Mutex::new(Some(__ptrSlot_LNode {
            get: Arc::new(Mutex::new(Some({ let __slot_i29_closure_clone_closure_clone = __slot_i29_closure_clone.clone(); let __slot_s28_closure_clone_closure_clone = __slot_s28_closure_clone.clone(); Arc::new(move || -> Arc<Mutex<Option<LNode>>> {
        {
            let __seq = { let __seq_holder = __slot_s28_closure_clone_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned };
            __seq[({ let __v = (*__slot_i29_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone()
        }
    }) as Arc<dyn Fn() -> Arc<Mutex<Option<LNode>>> + Send + Sync> }))),
            set: Arc::new(Mutex::new(Some({ let __slot_i29_closure_clone_closure_clone = __slot_i29_closure_clone.clone(); let __slot_s28_closure_clone_closure_clone = __slot_s28_closure_clone.clone(); Arc::new(move |__slot_v30: Arc<Mutex<Option<LNode>>>| {
        { let __idx = ({ let __v = (*__slot_i29_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __rhs = __slot_v30.clone(); (*__slot_s28_closure_clone_closure_clone.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    }) as Arc<dyn Fn(Arc<Mutex<Option<LNode>>>) -> () + Send + Sync> }))),
            ..Default::default()
        })));
    }) as Arc<dyn Fn(Arc<Mutex<Option<Vec<Arc<Mutex<Option<LNode>>>>>>>, Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<__ptrSlot_LNode>>> + Send + Sync>))); let __f: Arc<dyn Fn(Arc<Mutex<Option<Vec<Arc<Mutex<Option<LNode>>>>>>>, Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<__ptrSlot_LNode>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)(links.clone(), Arc::new(Mutex::new(Some(0)))) };
    let mut old = Arc::new(Mutex::new(links.lock().unwrap().clone()));
    { let __slice_header = GoSliceHeader::go_slice_header(&(Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(LNode { v: Arc::new(Mutex::new(Some(2))), ..Default::default() }))), Arc::new(Mutex::new(None))]))))); *links.lock().unwrap() = __slice_header; };
    push(Arc::new(Mutex::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))), Arc::new(Mutex::new(Some(5))));
    println!("{} {}", format!("{}", (*(*{ let __seq = { let __seq_holder = old.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap().as_ref().unwrap()).v.lock().unwrap().as_ref().unwrap())), format!("{}", (*(*{ let __seq = { let __seq_holder = links.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap().as_ref().unwrap()).v.lock().unwrap().as_ref().unwrap())));

    let __cell_head_2_closure_clone = __cell_head_2.clone(); let mut h = Arc::new(Mutex::new(Some(Holder { slot: Arc::new(Mutex::new(Some(__ptrSlot_LNode {
        get: Arc::new(Mutex::new(Some({ let __cell_head_2_closure_clone_closure_clone = __cell_head_2_closure_clone.clone(); Arc::new(move || -> Arc<Mutex<Option<LNode>>> {
        return (*__cell_head_2_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).v.clone();
    }) as Arc<dyn Fn() -> Arc<Mutex<Option<LNode>>> + Send + Sync> }))),
        set: Arc::new(Mutex::new(Some({ let __cell_head_2_closure_clone_closure_clone = __cell_head_2_closure_clone.clone(); Arc::new(move |__slot_v31: Arc<Mutex<Option<LNode>>>| {
        { let new_val = __slot_v31.clone(); (*__cell_head_2_closure_clone_closure_clone.lock().unwrap().as_mut().unwrap()).v = new_val; };
    }) as Arc<dyn Fn(Arc<Mutex<Option<LNode>>>) -> () + Send + Sync> }))),
        ..Default::default()
    }))), ..Default::default() })));
    push(Arc::new(Mutex::new(Some({ let __selector_holder = (*h.lock().unwrap().as_ref().unwrap()).slot.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = __selector_guard.as_ref().unwrap().__go_value_clone(); std::mem::drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(3))));
    println!("{} {}", format!("{}", { let __text = match Arc::new(Mutex::new(Some(List { head: { let __field = (*__cell_head_2.lock().unwrap().as_ref().unwrap()).v.clone(); __field }, ..Default::default() }))).lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __nil_target = (*(*h.lock().unwrap().as_ref().unwrap()).slot.lock().unwrap().as_ref().unwrap()).get.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result }));

    let mut empty: Arc<Mutex<Option<Holder>>> = Arc::new(Mutex::new(Some(Default::default())));
    println!("{}", format!("{}", { let __nil_target = (*(*empty.lock().unwrap().as_ref().unwrap()).slot.lock().unwrap().as_ref().unwrap()).get.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result }));

    let mut pp = { let __f_holder = Arc::new(Mutex::new(Some(Arc::new(move || -> Arc<Mutex<Option<__ptrSlot_LNode>>> {
        let mut __slot_p32: Arc<Mutex<Option<LNode>>> = Arc::new(Mutex::new(None));
        let mut __cell___slot_p32_3 = Arc::new(Mutex::new(Some(AnonymousStruct2 { v: __slot_p32.clone() })));
        let __cell___slot_p32_3_closure_clone = __cell___slot_p32_3.clone(); return Arc::new(Mutex::new(Some(__ptrSlot_LNode {
            get: Arc::new(Mutex::new(Some({ let __cell___slot_p32_3_closure_clone_closure_clone = __cell___slot_p32_3_closure_clone.clone(); Arc::new(move || -> Arc<Mutex<Option<LNode>>> {
        return (*__cell___slot_p32_3_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).v.clone();
    }) as Arc<dyn Fn() -> Arc<Mutex<Option<LNode>>> + Send + Sync> }))),
            set: Arc::new(Mutex::new(Some({ let __cell___slot_p32_3_closure_clone_closure_clone = __cell___slot_p32_3_closure_clone.clone(); Arc::new(move |__slot_v33: Arc<Mutex<Option<LNode>>>| {
        { let new_val = __slot_v33.clone(); (*__cell___slot_p32_3_closure_clone_closure_clone.lock().unwrap().as_mut().unwrap()).v = new_val; };
    }) as Arc<dyn Fn(Arc<Mutex<Option<LNode>>>) -> () + Send + Sync> }))),
            ..Default::default()
        })));
    }) as Arc<dyn Fn() -> Arc<Mutex<Option<__ptrSlot_LNode>>> + Send + Sync>))); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<__ptrSlot_LNode>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() };
    push(Arc::new(Mutex::new(Some({ let __arg_holder = pp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))), Arc::new(Mutex::new(Some(9))));
    println!("{} {}", format!("{}", (*(*({ let __f_holder = (*pp.lock().unwrap().as_ref().unwrap()).get.clone(); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<LNode>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }).lock().unwrap().as_ref().unwrap()).v.lock().unwrap().as_ref().unwrap())), format!("{}", { let __nil_target = (*({ let __f_holder = (*pp.lock().unwrap().as_ref().unwrap()).get.clone(); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<LNode>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }).lock().unwrap().as_ref().unwrap()).next.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result }));

    let mut counts: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> = Arc::new(Mutex::new(None));
    let mut __cell_counts_4 = Arc::new(Mutex::new(Some(AnonymousStruct3 { v: counts.clone() })));
    let __cell_counts_4_closure_clone = __cell_counts_4.clone(); add_count(
        Arc::new(Mutex::new(Some({ let __arg_value = __ptrSlot_map_string_int {
            get: Arc::new(Mutex::new(Some({ let __cell_counts_4_closure_clone_closure_clone = __cell_counts_4_closure_clone.clone(); Arc::new(move || -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> {
        return (*__cell_counts_4_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).v.clone();
    }) as Arc<dyn Fn() -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> + Send + Sync> }))),
            set: Arc::new(Mutex::new(Some({ let __cell_counts_4_closure_clone_closure_clone = __cell_counts_4_closure_clone.clone(); Arc::new(move |__slot_v34: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>| {
        { let new_val = __slot_v34.clone(); (*__cell_counts_4_closure_clone_closure_clone.lock().unwrap().as_mut().unwrap()).v = new_val; };
    }) as Arc<dyn Fn(Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>) -> () + Send + Sync> }))),
            ..Default::default()
        }; __arg_value }))),
        Arc::new(Mutex::new(Some("a".to_string())))
    );
    let __cell_counts_4_closure_clone = __cell_counts_4.clone(); add_count(
        Arc::new(Mutex::new(Some({ let __arg_value = __ptrSlot_map_string_int {
            get: Arc::new(Mutex::new(Some({ let __cell_counts_4_closure_clone_closure_clone = __cell_counts_4_closure_clone.clone(); Arc::new(move || -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> {
        return (*__cell_counts_4_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).v.clone();
    }) as Arc<dyn Fn() -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> + Send + Sync> }))),
            set: Arc::new(Mutex::new(Some({ let __cell_counts_4_closure_clone_closure_clone = __cell_counts_4_closure_clone.clone(); Arc::new(move |__slot_v35: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>| {
        { let new_val = __slot_v35.clone(); (*__cell_counts_4_closure_clone_closure_clone.lock().unwrap().as_mut().unwrap()).v = new_val; };
    }) as Arc<dyn Fn(Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>) -> () + Send + Sync> }))),
            ..Default::default()
        }; __arg_value }))),
        Arc::new(Mutex::new(Some("a".to_string())))
    );
    let __cell_counts_4_closure_clone = __cell_counts_4.clone(); add_count(
        Arc::new(Mutex::new(Some({ let __arg_value = __ptrSlot_map_string_int {
            get: Arc::new(Mutex::new(Some({ let __cell_counts_4_closure_clone_closure_clone = __cell_counts_4_closure_clone.clone(); Arc::new(move || -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> {
        return (*__cell_counts_4_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).v.clone();
    }) as Arc<dyn Fn() -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> + Send + Sync> }))),
            set: Arc::new(Mutex::new(Some({ let __cell_counts_4_closure_clone_closure_clone = __cell_counts_4_closure_clone.clone(); Arc::new(move |__slot_v36: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>| {
        { let new_val = __slot_v36.clone(); (*__cell_counts_4_closure_clone_closure_clone.lock().unwrap().as_mut().unwrap()).v = new_val; };
    }) as Arc<dyn Fn(Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>) -> () + Send + Sync> }))),
            ..Default::default()
        }; __arg_value }))),
        Arc::new(Mutex::new(Some("b".to_string())))
    );
    println!("{} {} {}", format!("{}", { let __map = { let __map_holder = (*__cell_counts_4.lock().unwrap().as_ref().unwrap()).v.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", { let __map = { let __map_holder = (*__cell_counts_4.lock().unwrap().as_ref().unwrap()).v.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"b".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", ({ let __len = ({ let __len_target = { let __field = (*__cell_counts_4.lock().unwrap().as_ref().unwrap()).v.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }); __len })));
}

#[derive(Debug, Clone, Default)]
pub struct AnonymousStruct1 {
    pub v: Arc<Mutex<Option<Node>>>,
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
        let __go_fmt_0 = format!("{}", { let __guard = self.v.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone, Default)]
pub struct AnonymousStruct2 {
    pub v: Arc<Mutex<Option<LNode>>>,
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
        let __go_fmt_0 = format!("{}", { let __guard = self.v.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone, Default)]
pub struct AnonymousStruct3 {
    pub v: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>,
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
