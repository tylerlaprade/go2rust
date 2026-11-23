#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use std::any::Any;
use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

/// A Go slice: an offset, a length and a capacity over a backing array that
/// every slice made from it shares. Every name below that a binding uses
/// starts with __, so no package-level variable of the program shadows it.
pub struct GoSlice<T> {
    backing: std::sync::Arc<std::cell::UnsafeCell<std::vec::Vec<T>>>,
    off: usize,
    len: usize,
    cap: usize,
}

unsafe impl<T: Send> Send for GoSlice<T> {}
unsafe impl<T: Send + Sync> Sync for GoSlice<T> {}

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
        GoSlice { backing: std::sync::Arc::from(std::cell::UnsafeCell::new(std::vec::Vec::with_capacity(__cap))), off: 0, len: 0, cap: __cap }
    }

    fn backing_ref(&self) -> &std::vec::Vec<T> {
        unsafe { &*self.backing.get() }
    }

    #[allow(clippy::mut_from_ref)]
    fn backing_mut(&self) -> &mut std::vec::Vec<T> {
        unsafe { &mut *self.backing.get() }
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
        let mut __elems = std::mem::take(__backing.get_mut());
        __elems.truncate(__off + __len);
        __elems.drain(..__off);
        Some(__elems)
    }

    fn fresh(__elems: std::vec::Vec<T>, __cap: usize) -> Self {
        let __len = __elems.len();
        GoSlice { backing: std::sync::Arc::from(std::cell::UnsafeCell::new(__elems)), off: 0, len: __len, cap: __cap.max(__len) }
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
            __backing.push(__value);
        }
        self.len += 1;
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
    fn default() -> Self {
        Self::new()
    }
}

impl<T> From<std::vec::Vec<T>> for GoSlice<T> {
    fn from(__elems: std::vec::Vec<T>) -> Self {
        let __len = __elems.len();
        GoSlice { backing: std::sync::Arc::from(std::cell::UnsafeCell::new(__elems)), off: 0, len: __len, cap: __len }
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
    None
}

fn go_any_clone(value: &dyn Any) -> Box<dyn Any> {
    go_any_try_clone(value).unwrap_or_else(|| panic!("go_any_clone: unsupported dynamic type; add typed lowering instead of cloning Box<dyn Any>"))
}

fn go_float_string(value: f64, single: bool) -> String {
    if value.is_nan() {
        return "NaN".to_string();
    }
    if value.is_infinite() {
        return if value < 0.0 { "-Inf".to_string() } else { "+Inf".to_string() };
    }
    let sign = if value.is_sign_negative() { "-" } else { "" };
    let shortest = if single { format!("{:e}", value.abs() as f32) } else { format!("{:e}", value.abs()) };
    let (mantissa, exponent) = shortest.split_once('e').unwrap();
    let digits: String = mantissa.chars().filter(|c| c.is_ascii_digit()).collect();
    let exponent: i32 = exponent.parse().unwrap();
    if exponent < -4 || exponent >= 6 {
        let mut text = String::from(sign);
        text.push_str(&digits[..1]);
        if digits.len() > 1 {
            text.push('.');
            text.push_str(&digits[1..]);
        }
        text.push('e');
        text.push(if exponent < 0 { '-' } else { '+' });
        text.push_str(&format!("{:02}", exponent.abs()));
        return text;
    }
    if exponent < 0 {
        return format!("{}0.{}{}", sign, "0".repeat((-exponent - 1) as usize), digits);
    }
    let whole = (exponent + 1) as usize;
    if digits.len() <= whole {
        return format!("{}{}{}", sign, digits, "0".repeat(whole - digits.len()));
    }
    format!("{}{}.{}", sign, &digits[..whole], &digits[whole..])
}

// A value inside a slice or a map prints as fmt prints it: a float through
// go_float_string, anything else through its Display. The element type may
// borrow, so it is recognized by name rather than by TypeId.
fn go_display_value<T: Display>(value: &T) -> String {
    let name = std::any::type_name::<T>();
    if name == "f64" {
        // T is f64.
        return go_float_string(unsafe { *(value as *const T as *const f64) }, false);
    }
    if name == "f32" {
        // T is f32.
        return go_float_string(unsafe { *(value as *const T as *const f32) } as f64, true);
    }
    value.to_string()
}

#[derive(Clone)]
pub struct Query {
    pub table: Rc<RefCell<Option<String>>>,
    pub wheres: Rc<RefCell<Option<Vec<String>>>>,
    pub order: Rc<RefCell<Option<String>>>,
    pub limit: Rc<RefCell<Option<i64>>>,
    pub params: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<Box<dyn Any>>>>>>>>,
}

impl Query {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.table.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = self.wheres.clone();
        let __go_clone_2_0 = { let __guard = self.order.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.limit.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_4_0 = self.params.clone();
        Self {
            table: __go_clone_0_0,
            wheres: __go_clone_1_0,
            order: __go_clone_2_0,
            limit: __go_clone_3_0,
            params: __go_clone_4_0,
        }
    }
}


impl Default for Query {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(None));
        let __go_default_2_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_3_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_4_0 = Rc::new(RefCell::new(None));
        Self {
            table: __go_default_0_0,
            wheres: __go_default_1_0,
            order: __go_default_2_0,
            limit: __go_default_3_0,
            params: __go_default_4_0,
        }
    }
}

impl std::fmt::Display for Query {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct User {
    pub name: Rc<RefCell<Option<String>>>,
    pub age: Rc<RefCell<Option<i64>>>,
    pub roles: Rc<RefCell<Option<Vec<String>>>>,
}

impl User {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.age.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_2_0 = self.roles.clone();
        Self {
            name: __go_clone_0_0,
            age: __go_clone_1_0,
            roles: __go_clone_2_0,
        }
    }
}


impl Default for User {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_2_0 = Rc::new(RefCell::new(None));
        Self {
            name: __go_default_0_0,
            age: __go_default_1_0,
            roles: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().borrow().as_ref().unwrap()))
    }
}


pub trait Writer: std::fmt::Display + Any {
    fn __go_clone_box_writer(&self) -> Box<dyn Writer>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_writer(&self, other: &dyn Writer) -> bool;
    fn write(&mut self, p: Rc<RefCell<Option<Vec<u8>>>>) -> (i64, Rc<RefCell<Option<Rc<dyn StdError>>>>);
}

impl Clone for Box<dyn Writer> {
    fn clone(&self) -> Self {
        Writer::__go_clone_box_writer(self.as_ref())
    }
}

#[derive(Debug, Clone, Default)]
pub struct upperWriter {
    pub buf: Rc<RefCell<Option<Vec<u8>>>>,
}

impl upperWriter {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.buf.clone();
        Self {
            buf: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for upperWriter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_slice(&self.buf));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Query {
    pub fn r#where(&mut self, cond: Rc<RefCell<Option<String>>>, args: Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> Rc<RefCell<Option<Query>>> {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = self.wheres.clone(); let __append_value = (*cond.borrow().as_ref().unwrap()).clone(); let mut __appended: Vec<String> = (*__append_target.borrow()).clone().unwrap_or_default(); __appended.push(__append_value); Rc::new(RefCell::new(Some(__appended))) })); *self.wheres.borrow_mut() = __slice_header; };
        if { let __nil_target = self.params.clone(); let __nil_result = (*__nil_target.borrow()).is_none(); __nil_result } {
        { let new_val = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Box<dyn Any>>>>>::from([])))); self.params = new_val; };
    }
        { let __range_holder = args.clone(); let __range_guard = __range_holder.borrow(); let __range_len = __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); drop(__range_guard); let __range_values = 0..__range_len; for (i, a) in __range_values.map(|__range_index| { let __range_guard = __range_holder.borrow(); go_any_clone(__range_guard.as_ref().unwrap()[__range_index].as_ref()) }).enumerate() {
        { let __map_key = { let __map_key_holder = Rc::new(RefCell::new(Some(format!("p{}", (({ let __len_target = { let __field = self.params.clone(); __field }; let __len_guard = __len_target.borrow(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i64) + (i as i64))))).clone(); let __map_key_guard = __map_key_holder.borrow(); let __cloned = (*__map_key_guard.as_ref().unwrap()).clone(); drop(__map_key_guard); __cloned }; let __map_value = Rc::new(RefCell::new(Some(go_any_clone(a.as_ref())))); (*self.params.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    } }
        Rc::new(RefCell::new(Some(self.clone())))
    }

    pub fn order_by(&mut self, col: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<Query>>> {
        { let new_val = col.borrow().as_ref().unwrap().clone(); *self.order.borrow_mut() = Some(new_val); };
        Rc::new(RefCell::new(Some(self.clone())))
    }

    pub fn limit(&mut self, n: Rc<RefCell<Option<i64>>>) -> Rc<RefCell<Option<Query>>> {
        { let new_val = n.borrow().as_ref().unwrap().clone(); *self.limit.borrow_mut() = Some(new_val); };
        Rc::new(RefCell::new(Some(self.clone())))
    }

    pub fn string(&self) -> Rc<RefCell<Option<String>>> {
        let mut b: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(Some(Default::default())));
        (*b.clone().borrow_mut().as_mut().unwrap()).push_str(&format!("SELECT * FROM {}", (*self.table.borrow().as_ref().unwrap())));
        if (({ let __len_target = { let __field = self.wheres.clone(); __field }; let __len_guard = __len_target.borrow(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i64) > (0 as i64) {
        (*b.borrow_mut().as_mut().unwrap()).push_str(&format!("{}{}", " WHERE ".to_string(), (*Rc::new(RefCell::new(Some({ let __parts = (*self.wheres.borrow()).as_ref().cloned().unwrap_or_default(); let __sep = " AND ".to_string(); __parts.join(&__sep) }))).borrow().as_ref().unwrap())));
    }
        if (*self.order.borrow().as_ref().unwrap()).clone() != "".to_string() {
        (*b.borrow_mut().as_mut().unwrap()).push_str(&format!("{}{}", " ORDER BY ".to_string(), (*self.order.clone().borrow().as_ref().unwrap())));
    }
        if (*self.limit.borrow().as_ref().unwrap()) > 0 {
        (*b.clone().borrow_mut().as_mut().unwrap()).push_str(&format!(" LIMIT {}", (*self.limit.borrow().as_ref().unwrap())));
    }
        return Rc::new(RefCell::new(Some({ let __builder = b.clone(); let __guard = __builder.borrow(); let __value = (*__guard.as_ref().unwrap()).clone(); drop(__guard); __value })));
    }
}

impl User {
    pub fn string(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("{}({})", (*self.name.borrow().as_ref().unwrap()), (*self.age.borrow().as_ref().unwrap())))))
    }
}

impl upperWriter {
    pub fn write(&mut self, p: Rc<RefCell<Option<Vec<u8>>>>) -> (i64, Rc<RefCell<Option<Rc<dyn StdError>>>>) {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = self.buf.clone(); let __append_values: Vec<_> = { let __slice_holder = Rc::new(RefCell::new(Some((*Rc::new(RefCell::new(Some({ let __s = (*Rc::new(RefCell::new(Some(String::from_utf8(std::vec::Vec::from((*p.borrow().as_ref().unwrap()).clone())).unwrap()))).borrow().as_ref().unwrap()).clone(); __s.to_uppercase() }))).borrow().as_ref().unwrap()).as_bytes().to_go_slice()))).clone(); let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned().collect(); let mut __appended: Vec<u8> = (*__append_target.borrow()).clone().unwrap_or_default(); __appended.extend(__append_values); Rc::new(RefCell::new(Some(__appended))) })); *self.buf.borrow_mut() = __slice_header; };
        ((*p.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i64, Rc::new(RefCell::new(None)))
    }
}

impl Writer for upperWriter {
    fn write(&mut self, p: Rc<RefCell<Option<Vec<u8>>>>) -> (i64, Rc<RefCell<Option<Rc<dyn StdError>>>>) {
        upperWriter::write(self, p)
    }
    fn __go_clone_box_writer(&self) -> Box<dyn Writer> {
        Box::new(self.clone()) as Box<dyn Writer>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_writer(&self, other: &dyn Writer) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<upperWriter>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct upperWriterPtr(pub Rc<RefCell<Option<upperWriter>>>);

impl std::fmt::Display for upperWriterPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Writer for upperWriterPtr {
    fn write(&mut self, p: Rc<RefCell<Option<Vec<u8>>>>) -> (i64, Rc<RefCell<Option<Rc<dyn StdError>>>>) {
        let mut __recv_guard = self.0.borrow_mut();
        let __recv = __recv_guard.as_mut().unwrap();
        upperWriter::write(__recv, p)
    }
    fn __go_clone_box_writer(&self) -> Box<dyn Writer> {
        Box::new(self.clone()) as Box<dyn Writer>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_writer(&self, other: &dyn Writer) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<upperWriterPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn from(t: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<Query>>> {
    Rc::new(RefCell::new(Some(Query { table: Rc::new(RefCell::new(Some({ let __arg_holder = t.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })))
}

pub fn emit(w: Rc<RefCell<Option<Box<dyn Writer>>>>, parts: Rc<RefCell<Option<Vec<String>>>>) -> i64 {
    let mut total = Rc::new(RefCell::new(Some(0i64)));
    { let __range_holder = parts.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for p in __range_values.iter() {
        let (mut n, _) = (*w.borrow_mut().as_mut().unwrap()).write(Rc::new(RefCell::new(Some((p).as_bytes().to_go_slice()))));
        { let __rhs = n; let mut guard = total.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }
    return (*total.borrow().as_ref().unwrap());
}

fn main() {
    let mut q = { let __recv = { let __recv = { let __recv = { let __recv = from(Rc::new(RefCell::new(Some("users".to_string())))); let __result = (*__recv.borrow_mut().as_mut().unwrap()).r#where(Rc::new(RefCell::new(Some("age > ?".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(18i64) as Box<dyn Any>])))); __result }; let __result = (*__recv.borrow_mut().as_mut().unwrap()).r#where(Rc::new(RefCell::new(Some("name LIKE ?".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new("a%".to_string()) as Box<dyn Any>])))); __result }; let __result = (*__recv.borrow_mut().as_mut().unwrap()).order_by(Rc::new(RefCell::new(Some("name".to_string())))); __result }; let __result = (*__recv.borrow_mut().as_mut().unwrap()).limit(Rc::new(RefCell::new(Some(10)))); __result };
    println!("{}", format!("{}", { let __text = match q.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    let mut keys = Rc::new(RefCell::new(Some(Vec::<String>::with_capacity(0))));
    for (k, _) in ((*(*q.borrow().as_ref().unwrap()).params.borrow().as_ref().unwrap()).clone()).clone() {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = keys.clone(); let __append_value = k.clone(); let mut __appended: Vec<String> = (*__append_target.borrow()).clone().unwrap_or_default(); __appended.push(__append_value); Rc::new(RefCell::new(Some(__appended))) })); *keys.borrow_mut() = __slice_header; };
    }
    { let __sort_target = keys.clone(); let mut __sort_guard = __sort_target.borrow_mut(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort(); } };
    { let __range_holder = keys.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for k in __range_values.iter() {
        print!("{}{}{}{}", format!("{}", k), format!("{}", "=".to_string()), format!("{}", match ((*(*q.borrow().as_ref().unwrap()).params.borrow().as_ref().unwrap()).clone().get(k).map(|__v| __v.clone()).unwrap_or_else(|| Default::default())).borrow().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }), format!("{}", " ".to_string()));
    } }
    println!();

    let mut users = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(User { name: Rc::new(RefCell::new(Some("bob".to_string()))), age: Rc::new(RefCell::new(Some(30))), roles: Rc::new(RefCell::new(None)), ..Default::default() }))), Rc::new(RefCell::new(Some(User { name: Rc::new(RefCell::new(Some("al".to_string()))), age: Rc::new(RefCell::new(Some(25))), roles: Rc::new(RefCell::new(Some(vec!["admin".to_string()]))), ..Default::default() }))), Rc::new(RefCell::new(Some(User { name: Rc::new(RefCell::new(Some("cy".to_string()))), age: Rc::new(RefCell::new(Some(35))), roles: Rc::new(RefCell::new(Some(vec!["dev".to_string(), "ops".to_string()]))), ..Default::default() })))])));
    let users_closure_clone = users.clone(); { let __sort_target = users_closure_clone.clone(); let __sort_less = Rc::new(RefCell::new(Some(Box::new(move |i: Rc<RefCell<Option<i64>>>, j: Rc<RefCell<Option<i64>>>| -> bool {
        return (*(*(*users_closure_clone.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone().borrow().as_ref().unwrap()).age.borrow().as_ref().unwrap()) < (*(*(*users_closure_clone.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone().borrow().as_ref().unwrap()).age.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> bool>))); let __sort_len = { let __sort_guard = __sort_target.borrow(); __sort_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; for __sort_i in 1..__sort_len { let mut __sort_j = __sort_i; while __sort_j > 0 { let __should_swap = { let mut __less_guard = __sort_less.borrow_mut(); let __less = __less_guard.as_mut().expect("sort.Slice less function is nil"); __less(Rc::new(RefCell::new(Some(__sort_j as i64))), Rc::new(RefCell::new(Some((__sort_j - 1) as i64)))) }; if !__should_swap { break; } { let mut __sort_guard = __sort_target.borrow_mut(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.swap(__sort_j, __sort_j - 1); } } __sort_j -= 1; } } };
    println!("{} {} {}", format!("{}", { let __text = match (*users.borrow().as_ref().unwrap())[(0) as usize].clone().borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", (*{ let __recv = (*users.borrow().as_ref().unwrap())[(1) as usize].clone(); let __result = (*__recv.borrow().as_ref().unwrap()).string(); __result }.borrow().as_ref().unwrap())), format!("{}", (*users.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)));
    let mut byRole = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Vec<Rc<RefCell<Option<User>>>>>>>>::from([]))));
    { let __range_holder = users.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for u in __range_values.iter() {
        { let __range_holder = (*u.borrow().as_ref().unwrap()).roles.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for r in __range_values.iter() {
        { let __map_key = r.clone(); let __map_value = { let __slice = { let __map_holder = byRole.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().unwrap().get(r).cloned().unwrap_or_else(|| Rc::new(RefCell::new(None))) }; let __append_value = (*u).clone(); let mut __appended = (*__slice.borrow()).clone().unwrap_or_default(); __appended.push(__append_value); Rc::new(RefCell::new(Some(__appended))) }; (*byRole.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    } }
    } }
    { let __target = (*(*{ let __map_holder = byRole.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"admin".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.borrow().as_ref().unwrap())[(0) as usize].clone().borrow().as_ref().unwrap()).age.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    println!("{} {} {}", format!("{}", (*(*(*users.borrow().as_ref().unwrap())[(0) as usize].clone().borrow().as_ref().unwrap()).age.borrow().as_ref().unwrap())), format!("{}", { let __text = match (*{ let __map_holder = byRole.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"admin".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.borrow().as_ref().unwrap())[(0) as usize].clone().borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", (*{ let __map_holder = byRole.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"dev".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)));
    let mut idx = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<User>>>>::from([]))));
    { let __range_holder = users.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for u in __range_values.iter() {
        { let __map_key = { let __selector_holder = (*u.borrow().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __map_value = u.clone(); (*idx.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    } }
    { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = (*{ let __map_holder = idx.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"cy".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.borrow().as_ref().unwrap()).roles.clone(); let __append_value = "lead".to_string(); let mut __appended: Vec<String> = (*__append_target.borrow()).clone().unwrap_or_default(); __appended.push(__append_value); Rc::new(RefCell::new(Some(__appended))) })); *(*{ let __map_holder = idx.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"cy".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.borrow().as_ref().unwrap()).roles.borrow_mut() = __slice_header; };
    println!("{} {}", format!("{}", format_slice_values(&(*(*(*users.borrow().as_ref().unwrap())[(2) as usize].clone().borrow().as_ref().unwrap()).roles.borrow().as_ref().unwrap()).clone())), format!("{}", { let __text = match { let __map_holder = idx.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"bob".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

    let mut w = Rc::new(RefCell::new(Some(upperWriter { buf: Default::default() })));
    let mut n = emit(Rc::new(RefCell::new(Some(Box::new(upperWriterPtr(w.clone())) as Box<dyn Writer>))), Rc::new(RefCell::new(Some(vec!["hello ".to_string(), "world".to_string()]))));
    println!("{} {}", format!("{}", n), format!("{}", (*Rc::new(RefCell::new(Some(String::from_utf8(std::vec::Vec::from({ let __slice_holder = (*w.borrow().as_ref().unwrap()).buf.clone(); let __slice_guard = __slice_holder.borrow(); (*__slice_guard.as_ref().unwrap()).clone() })).unwrap()))).borrow().as_ref().unwrap())));
    { let __s = format!(" {}!", 42); (*w.borrow_mut().as_mut().unwrap()).write(Rc::new(RefCell::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some(String::from_utf8(std::vec::Vec::from({ let __slice_holder = (*w.borrow().as_ref().unwrap()).buf.clone(); let __slice_guard = __slice_holder.borrow(); (*__slice_guard.as_ref().unwrap()).clone() })).unwrap()))).borrow().as_ref().unwrap())));
}