#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use num::Complex;
use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display};
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
/// A Go slice: an offset, a length and a capacity over a backing array that
/// every slice made from it shares. Every name below that a binding uses
/// starts with __, so no package-level variable of the program shadows it.
pub struct GoSlice<T> {
    backing: std::sync::Arc<std::sync::RwLock<std::vec::Vec<T>>>,
    off: usize,
    len: usize,
    cap: usize,
}

pub struct GoSliceReadGuard<'a, T> {
    guard: std::sync::RwLockReadGuard<'a, std::vec::Vec<T>>,
    off: usize,
    len: usize,
}

impl<T> std::ops::Deref for GoSliceReadGuard<'_, T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        &self.guard[self.off..self.off + self.len]
    }
}

impl<T> AsRef<[T]> for GoSliceReadGuard<'_, T> {
    fn as_ref(&self) -> &[T] {
        self
    }
}

pub struct GoSliceWriteGuard<'a, T> {
    guard: std::sync::RwLockWriteGuard<'a, std::vec::Vec<T>>,
    off: usize,
    len: usize,
}

impl<T> std::ops::Deref for GoSliceWriteGuard<'_, T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        &self.guard[self.off..self.off + self.len]
    }
}

impl<T> std::ops::DerefMut for GoSliceWriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut [T] {
        &mut self.guard[self.off..self.off + self.len]
    }
}

pub trait GoSliceView<T> {
    type Guard<'a>: std::ops::Deref<Target = [T]> + AsRef<[T]>
    where
        Self: 'a,
        T: 'a;
    fn go_slice_view(&self) -> Self::Guard<'_>;
}

impl<T> GoSliceView<T> for GoSlice<T> {
    type Guard<'a> = GoSliceReadGuard<'a, T> where T: 'a;
    fn go_slice_view(&self) -> Self::Guard<'_> {
        self.as_slice()
    }
}

impl<T> GoSliceView<T> for std::vec::Vec<T> {
    type Guard<'a> = &'a [T] where T: 'a;
    fn go_slice_view(&self) -> Self::Guard<'_> {
        self.as_slice()
    }
}

impl<T, const N: usize> GoSliceView<T> for [T; N] {
    type Guard<'a> = &'a [T] where T: 'a;
    fn go_slice_view(&self) -> Self::Guard<'_> {
        self.as_slice()
    }
}

impl<T> GoSliceView<T> for [T] {
    type Guard<'a> = &'a [T] where T: 'a;
    fn go_slice_view(&self) -> Self::Guard<'_> {
        self
    }
}

/// The prelude's Vec and vec! are Go slices in generated code.
pub type Vec<T> = GoSlice<T>;

impl<T> Clone for GoSlice<T> {
    /// Copies the header. The copy shares the elements, as a Go slice
    /// assignment does.
    fn clone(&self) -> Self {
        GoSlice { backing: self.backing.clone(), off: self.off, len: self.len, cap: self.cap }
    }
}

impl<T> GoSlice<T> {
    pub fn new() -> Self {
        Self::from(std::vec::Vec::new())
    }

    pub fn with_capacity(__cap: usize) -> Self {
        GoSlice { backing: std::sync::Arc::from(std::sync::RwLock::new(std::vec::Vec::with_capacity(__cap))), off: 0, len: 0, cap: __cap }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get_cloned(&self, __index: usize) -> T
    where
        T: Clone,
    {
        self.as_slice()[__index].clone()
    }

    pub fn set(&self, __index: usize, __value: T) {
        let mut __guard = self.backing.write().unwrap();
        __guard[self.off + __index] = __value;
    }

    pub fn with_mut<R>(&self, __index: usize, __f: impl FnOnce(&mut T) -> R) -> R {
        let mut __guard = self.backing.write().unwrap();
        __f(&mut __guard[self.off + __index])
    }

    pub fn swap(&self, __left: usize, __right: usize) {
        let mut __guard = self.backing.write().unwrap();
        __guard.swap(self.off + __left, self.off + __right);
    }

    pub fn reverse(&mut self) {
        self.as_mut_slice().reverse();
    }

    pub fn sort(&mut self)
    where
        T: Ord,
    {
        self.as_mut_slice().sort();
    }

    pub fn sort_by<F>(&mut self, __compare: F)
    where
        F: FnMut(&T, &T) -> std::cmp::Ordering,
    {
        self.as_mut_slice().sort_by(__compare);
    }

    pub fn as_slice(&self) -> GoSliceReadGuard<'_, T> {
        GoSliceReadGuard { guard: self.backing.read().unwrap(), off: self.off, len: self.len }
    }

    pub fn as_mut_slice(&mut self) -> GoSliceWriteGuard<'_, T> {
        GoSliceWriteGuard { guard: self.backing.write().unwrap(), off: self.off, len: self.len }
    }

    /// Go's cap(s).
    pub fn capacity(&self) -> usize {
        self.cap
    }

    /// Whether two slices share their backing array.
    pub fn shares_backing_with(&self, __other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.backing, &__other.backing)
    }

    fn with_pair<R>(&self, __other: &Self, __f: impl FnOnce(&[T], &[T]) -> R) -> R {
        if self.shares_backing_with(__other) {
            let __guard = self.backing.read().unwrap();
            let __left = &__guard[self.off..self.off + self.len];
            let __right = &__guard[__other.off..__other.off + __other.len];
            return __f(__left, __right);
        }
        let __self_first = std::sync::Arc::as_ptr(&self.backing) < std::sync::Arc::as_ptr(&__other.backing);
        if __self_first {
            let __left_guard = self.backing.read().unwrap();
            let __right_guard = __other.backing.read().unwrap();
            __f(
                &__left_guard[self.off..self.off + self.len],
                &__right_guard[__other.off..__other.off + __other.len],
            )
        } else {
            let __right_guard = __other.backing.read().unwrap();
            let __left_guard = self.backing.read().unwrap();
            __f(
                &__left_guard[self.off..self.off + self.len],
                &__right_guard[__other.off..__other.off + __other.len],
            )
        }
    }

    /// Go's s[lo:hi].
    pub fn reslice(&self, __lo: usize, __hi: usize) -> Self {
        if __lo > __hi || __hi > self.cap {
            panic!("runtime error: slice bounds out of range [{}:{}] with capacity {}", __lo, __hi, self.cap);
        }
        if self.off + __hi > self.backing.read().unwrap().len() {
            panic!("slice [{}:{}] reaches past the elements GoSlice has stored", __lo, __hi);
        }
        GoSlice { backing: self.backing.clone(), off: self.off + __lo, len: __hi - __lo, cap: self.cap - __lo }
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
            let mut __window = self.as_mut_slice();
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
        let mut __elems = std::mem::take(__backing.get_mut().unwrap());
        __elems.truncate(__off + __len);
        __elems.drain(..__off);
        Some(__elems)
    }

    fn fresh(__elems: std::vec::Vec<T>, __cap: usize) -> Self {
        let __len = __elems.len();
        GoSlice { backing: std::sync::Arc::from(std::sync::RwLock::new(__elems)), off: 0, len: __len, cap: __cap.max(__len) }
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
        let mut __backing = self.backing.write().unwrap();
        if __at < __backing.len() {
            __backing[__at] = __value;
        } else {
            __backing.push(__value);
        }
        self.len += 1;
    }

}

impl<T: Clone> GoSlice<T> {
    pub fn to_vec(&self) -> std::vec::Vec<T> {
        self.as_slice().to_vec()
    }

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

/// Go's append for interface values, which have no Clone. When the slice
/// is full and shares its backing array, its existing values are copied into
/// a fresh backing array before the new values are written.
pub trait GoSliceAppendBoxed<T> {
    fn push(&mut self, __value: T);
    fn extend<I: GoSliceBoxedValues<T>>(&mut self, __values: I);
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
    fn default() -> Self {
        Self::new()
    }
}

impl<T> From<std::vec::Vec<T>> for GoSlice<T> {
    fn from(__elems: std::vec::Vec<T>) -> Self {
        let __len = __elems.len();
        GoSlice { backing: std::sync::Arc::from(std::sync::RwLock::new(__elems)), off: 0, len: __len, cap: __len }
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

impl<T: std::fmt::Debug> std::fmt::Debug for GoSlice<T> {
    fn fmt(&self, __f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let __guard = self.as_slice();
        __guard.fmt(__f)
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

impl GoSlice<String> {
    pub fn join(&self, __separator: &str) -> String {
        self.as_slice().join(__separator)
    }
}

impl<T: PartialEq> PartialEq for GoSlice<T> {
    fn eq(&self, __other: &Self) -> bool {
        self.with_pair(__other, |__left, __right| __left == __right)
    }
}

impl<T: PartialEq> PartialEq<[T]> for GoSlice<T> {
    fn eq(&self, __other: &[T]) -> bool {
        let __guard = self.as_slice();
        &*__guard == __other
    }
}

impl<T: PartialEq> PartialEq<std::vec::Vec<T>> for GoSlice<T> {
    fn eq(&self, __other: &std::vec::Vec<T>) -> bool {
        let __guard = self.as_slice();
        &*__guard == __other.as_slice()
    }
}

impl<T: Eq> Eq for GoSlice<T> {}

impl<T: PartialOrd> PartialOrd for GoSlice<T> {
    fn partial_cmp(&self, __other: &Self) -> Option<std::cmp::Ordering> {
        self.with_pair(__other, |__left, __right| __left.partial_cmp(__right))
    }
}

impl<T: Ord> Ord for GoSlice<T> {
    fn cmp(&self, __other: &Self) -> std::cmp::Ordering {
        self.with_pair(__other, |__left, __right| __left.cmp(__right))
    }
}

impl<T: std::hash::Hash> std::hash::Hash for GoSlice<T> {
    fn hash<H: std::hash::Hasher>(&self, __state: &mut H) {
        let __guard = self.as_slice();
        __guard.hash(__state)
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
    C: GoSliceView<T>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: std::vec::Vec<String> = s.go_slice_view().iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn format_slice_values<T, C>(slice: &C) -> String
where
    C: GoSliceView<T> + ?Sized,
    T: Display,
{
    let __view = slice.go_slice_view();
    let formatted: std::vec::Vec<String> = __view.iter().map(|v| v.to_string()).collect();
    format!("[{}]", formatted.join(" "))
}

fn format_slice_wrapped<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: GoSliceView<Rc<RefCell<Option<T>>>>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: std::vec::Vec<String> = s.go_slice_view().iter().map(|v| {
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
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f32>() {
        v.to_string()
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

fn __go_type_name(val: &dyn Any) -> &'static str {
    if val.is::<i64>() { return "int" }
    if val.is::<i32>() { return "int32" }
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

fn main() {
        // Basic numeric type conversions
    println!("{}", format!("{}", "=== Basic numeric conversions ===".to_string()));

    let mut i: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(42)));
    let mut f: Rc<RefCell<Option<f64>>> = Rc::new(RefCell::new(Some((*i.borrow().as_ref().unwrap()) as f64)));
    let mut i2: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some((*f.borrow().as_ref().unwrap()) as i64)));

    print!("int: {}\n", { let __v = (*i.borrow().as_ref().unwrap()).clone(); __v });
    print!("float64: {:.2}\n", { let __v = (*f.borrow().as_ref().unwrap()).clone(); __v });
    print!("back to int: {}\n", { let __v = (*i2.borrow().as_ref().unwrap()).clone(); __v });

        // Different integer sizes
    println!("{}", format!("{}", "\n=== Integer size conversions ===".to_string()));

    let mut i8: Rc<RefCell<Option<i8>>> = Rc::new(RefCell::new(Some(127)));
    let mut i16: Rc<RefCell<Option<i16>>> = Rc::new(RefCell::new(Some((*i8.borrow().as_ref().unwrap()) as i16)));
    let mut i32: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some((*i16.borrow().as_ref().unwrap()) as i32)));
    let mut i64: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some((*i32.borrow().as_ref().unwrap()) as i64)));

    print!("int8: {}\n", { let __v = (*i8.borrow().as_ref().unwrap()).clone(); __v });
    print!("int16: {}\n", { let __v = (*i16.borrow().as_ref().unwrap()).clone(); __v });
    print!("int32: {}\n", { let __v = (*i32.borrow().as_ref().unwrap()).clone(); __v });
    print!("int64: {}\n", { let __v = (*i64.borrow().as_ref().unwrap()).clone(); __v });

        // Unsigned integers
    println!("{}", format!("{}", "\n=== Unsigned integer conversions ===".to_string()));

    let mut ui: Rc<RefCell<Option<u64>>> = Rc::new(RefCell::new(Some(42)));
    let mut ui8: Rc<RefCell<Option<u8>>> = Rc::new(RefCell::new(Some((*ui.borrow().as_ref().unwrap()) as u8)));
    let mut ui16: Rc<RefCell<Option<u16>>> = Rc::new(RefCell::new(Some((*ui8.borrow().as_ref().unwrap()) as u16)));
    let mut ui32: Rc<RefCell<Option<u32>>> = Rc::new(RefCell::new(Some((*ui16.borrow().as_ref().unwrap()) as u32)));
    let mut ui64: Rc<RefCell<Option<u64>>> = Rc::new(RefCell::new(Some((*ui32.borrow().as_ref().unwrap()) as u64)));

    print!("uint: {}\n", { let __v = (*ui.borrow().as_ref().unwrap()).clone(); __v });
    print!("uint8: {}\n", { let __v = (*ui8.borrow().as_ref().unwrap()).clone(); __v });
    print!("uint16: {}\n", { let __v = (*ui16.borrow().as_ref().unwrap()).clone(); __v });
    print!("uint32: {}\n", { let __v = (*ui32.borrow().as_ref().unwrap()).clone(); __v });
    print!("uint64: {}\n", { let __v = (*ui64.borrow().as_ref().unwrap()).clone(); __v });

        // Float conversions
    println!("{}", format!("{}", "\n=== Float conversions ===".to_string()));

    let mut f64: Rc<RefCell<Option<f64>>> = Rc::new(RefCell::new(Some(3.14159265359)));
    let mut f32: Rc<RefCell<Option<f32>>> = Rc::new(RefCell::new(Some((*f64.borrow().as_ref().unwrap()) as f32)));
    let mut backToF64: Rc<RefCell<Option<f64>>> = Rc::new(RefCell::new(Some((*f32.borrow().as_ref().unwrap()) as f64)));

    print!("float64: {:.10}\n", { let __v = (*f64.borrow().as_ref().unwrap()).clone(); __v });
    print!("float32: {:.10}\n", { let __v = (*f32.borrow().as_ref().unwrap()).clone(); __v });
    print!("back to float64: {:.10}\n", { let __v = (*backToF64.borrow().as_ref().unwrap()).clone(); __v });

        // String conversions
    println!("{}", format!("{}", "\n=== String conversions ===".to_string()));

    let mut r: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(('A' as i32))));
    let mut b: Rc<RefCell<Option<u8>>> = Rc::new(RefCell::new(Some(65)));

    print!("rune 'A': {} ({})\n", ({ let __v = (*r.borrow().as_ref().unwrap()).clone(); __v }) as u8 as char, { let __v = (*r.borrow().as_ref().unwrap()).clone(); __v });
    print!("byte 65: {} ({})\n", ({ let __v = (*b.borrow().as_ref().unwrap()).clone(); __v }) as u8 as char, { let __v = (*b.borrow().as_ref().unwrap()).clone(); __v });

        // Rune to string
    let mut str = Rc::new(RefCell::new(Some(char::from_u32(((*r.borrow().as_ref().unwrap())) as u32).unwrap().to_string())));
    print!("rune to string: {}\n", { let __v = (*str.borrow().as_ref().unwrap()).clone(); __v });

        // Byte slice to string
    let mut bytes = Rc::new(RefCell::new(Some(vec![72 as u8, 101 as u8, 108 as u8, 108 as u8, 111 as u8])));
    let mut strFromBytes = Rc::new(RefCell::new(Some(String::from_utf8(std::vec::Vec::from((*bytes.borrow().as_ref().unwrap()).clone())).unwrap())));
    print!("bytes to string: {}\n", { let __v = (*strFromBytes.borrow().as_ref().unwrap()).clone(); __v });

        // String to byte slice
    let mut backToBytes = Rc::new(RefCell::new(Some(((*strFromBytes.borrow().as_ref().unwrap())).as_bytes().to_go_slice())));
    print!("string to bytes: {}\n", format_slice(&backToBytes));

        // String to rune slice
    let mut runes = Rc::new(RefCell::new(Some(("Hello, \u{4e16}\u{754c}".to_string()).chars().map(|c| c as i32).collect::<Vec<_>>())));
    print!("string to runes: {}\n", format_slice(&runes));
    print!("rune count: {}\n", (*runes.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0));

        // Rune slice back to string
    let mut backToString = Rc::new(RefCell::new(Some((*runes.borrow().as_ref().unwrap()).as_slice().iter().map(|&c| char::from_u32(c as u32).unwrap()).collect::<String>())));
    print!("runes to string: {}\n", { let __v = (*backToString.borrow().as_ref().unwrap()).clone(); __v });

        // Boolean conversions (not direct, but showing concept)
    println!("{}", format!("{}", "\n=== Boolean-like conversions ===".to_string()));

    let mut zero: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));
    let mut nonZero: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(42)));

        // Go doesn't have direct bool conversion, but we can demonstrate the concept
    print!("zero == 0: {}\n", (*zero.borrow().as_ref().unwrap()) == 0);
    print!("nonZero != 0: {}\n", (*nonZero.borrow().as_ref().unwrap()) != 0);

        // Pointer conversions
    println!("{}", format!("{}", "\n=== Pointer conversions ===".to_string()));

    let mut num: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(100)));
    let mut ptr: Rc<RefCell<Option<i64>>> = num.clone();

    print!("value: {}\n", { let __v = (*num.borrow().as_ref().unwrap()).clone(); __v });
    print!("pointer: {}\n", "0xDEADBEEF".to_string());
    print!("dereferenced: {}\n", { let __v = (*ptr.borrow().as_ref().unwrap()).clone(); __v });

        // Interface conversions (basic)
    println!("{}", format!("{}", "\n=== Interface conversions ===".to_string()));

    let mut any: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(42i64) as Box<dyn Any>)));
    print!("interface{{}} value: {}\n", format_any(any.borrow().as_ref().unwrap().as_ref()));
    print!("interface{{}} type: {}\n", __go_type_name(&**any.borrow().as_ref().unwrap()));

        // Type assertion
    {
        let (mut intVal, mut ok) = ({
        let val = any.clone();
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
            print!("asserted as int: {}\n", { let __v = (*intVal.borrow().as_ref().unwrap()).clone(); __v });;
        }
    }

        // Change interface value
    { let new_val = Box::new("hello".to_string()) as Box<dyn Any>; *any.borrow_mut() = Some(new_val); };
    print!("new interface{{}} value: {}\n", format_any(any.borrow().as_ref().unwrap().as_ref()));
    print!("new interface{{}} type: {}\n", __go_type_name(&**any.borrow().as_ref().unwrap()));

    {
        let (mut strVal, mut ok) = ({
        let val = any.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<std::string::String>() {
                (Rc::new(RefCell::new(Some(typed_val.clone()))), true)
            } else {
                (Rc::new(RefCell::new(Some(std::string::String::new()))), false)
            }
        } else {
            (Rc::new(RefCell::new(Some(std::string::String::new()))), false)
        }
    });;
        if ok {
            print!("asserted as string: {}\n", { let __v = (*strVal.borrow().as_ref().unwrap()).clone(); __v });;
        }
    }

        // Complex number conversions
    println!("{}", format!("{}", "\n=== Complex number conversions ===".to_string()));

    let mut c64: Rc<RefCell<Option<num::Complex<f32>>>> = Rc::new(RefCell::new(Some(num::Complex::<f32>::new(3.0, 4.0))));
    let mut c128: Rc<RefCell<Option<num::Complex<f64>>>> = Rc::new(RefCell::new(Some({ let __z = (*c64.borrow().as_ref().unwrap()); num::Complex::<f64>::new(__z.re as f64, __z.im as f64) })));

    print!("complex64: {}\n", format!("({})", { let __v = (*c64.borrow().as_ref().unwrap()).clone(); __v }));
    print!("complex128: {}\n", format!("({})", { let __v = (*c128.borrow().as_ref().unwrap()).clone(); __v }));

        // Extract real and imaginary parts
    let mut real = Rc::new(RefCell::new(Some((*c128.borrow().as_ref().unwrap()).re)));
    let mut imag = Rc::new(RefCell::new(Some((*c128.borrow().as_ref().unwrap()).im)));
    print!("real part: {:.2}\n", { let __v = (*real.borrow().as_ref().unwrap()).clone(); __v });
    print!("imaginary part: {:.2}\n", { let __v = (*imag.borrow().as_ref().unwrap()).clone(); __v });

        // Create complex from parts
    let mut newComplex = Rc::new(RefCell::new(Some(num::Complex::new((*real.borrow().as_ref().unwrap()) as f64, (*imag.borrow().as_ref().unwrap()) as f64))));
    print!("reconstructed: {}\n", format!("({})", { let __v = (*newComplex.borrow().as_ref().unwrap()).clone(); __v }));

        // Overflow demonstration (be careful!)
    println!("{}", format!("{}", "\n=== Overflow examples ===".to_string()));

    let mut bigInt: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(1000000)));
    let mut smallInt: Rc<RefCell<Option<i8>>> = Rc::new(RefCell::new(Some((*bigInt.borrow().as_ref().unwrap()) as i8)));
    print!("int64: {}\n", { let __v = (*bigInt.borrow().as_ref().unwrap()).clone(); __v });
    print!("int8 (overflow): {}\n", { let __v = (*smallInt.borrow().as_ref().unwrap()).clone(); __v });

        // Precision loss in float conversion
    let mut preciseFloat: Rc<RefCell<Option<f64>>> = Rc::new(RefCell::new(Some(1.23456789012345)));
    let mut lessPrec: Rc<RefCell<Option<f32>>> = Rc::new(RefCell::new(Some((*preciseFloat.borrow().as_ref().unwrap()) as f32)));
    print!("float64: {:.15}\n", { let __v = (*preciseFloat.borrow().as_ref().unwrap()).clone(); __v });
    print!("float32: {:.15}\n", { let __v = (*lessPrec.borrow().as_ref().unwrap()).clone(); __v });

        // Custom type conversions
    println!("{}", format!("{}", "\n=== Custom type conversions ===".to_string()));

    type MyInt = Rc<RefCell<Option<i64>>>;
    type MyString = Rc<RefCell<Option<String>>>;

    let mut mi: MyInt = Rc::new(RefCell::new(Some(42)));
    let mut regularInt: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some((*mi.borrow().as_ref().unwrap()) as i64)));
    let mut backToMyInt: MyInt = Rc::new(RefCell::new(Some((*regularInt.borrow().as_ref().unwrap()) as i64)));

    print!("MyInt: {}\n", { let __v = (*mi.borrow().as_ref().unwrap()).clone(); __v });
    print!("regular int: {}\n", { let __v = (*regularInt.borrow().as_ref().unwrap()).clone(); __v });
    print!("back to MyInt: {}\n", { let __v = (*backToMyInt.borrow().as_ref().unwrap()).clone(); __v });

    let mut ms: MyString = Rc::new(RefCell::new(Some("hello".to_string())));
    let mut regularString: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(Some((*ms.borrow().as_ref().unwrap()).clone())));
    let mut backToMyString: MyString = Rc::new(RefCell::new(Some((*regularString.borrow().as_ref().unwrap()).clone())));

    print!("MyString: {}\n", { let __v = (*ms.borrow().as_ref().unwrap()).clone(); __v });
    print!("regular string: {}\n", { let __v = (*regularString.borrow().as_ref().unwrap()).clone(); __v });
    print!("back to MyString: {}\n", { let __v = (*backToMyString.borrow().as_ref().unwrap()).clone(); __v });
}