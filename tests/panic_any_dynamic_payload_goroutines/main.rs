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
use std::sync::{Arc, Mutex};
use std::thread;

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


// Go expands a pointer to a struct only when the pointer is the value being
// printed. Inside a struct or a slice it prints the address. That is also
// what keeps a value that points back to itself from printing forever.
fn go_format_any_pointer(nested: bool, address: *const (), is_nil: impl FnOnce() -> bool, expand: impl FnOnce() -> String) -> String {
    if !nested {
        return expand();
    }
    if is_nil() { "<nil>".to_string() } else { format!("{:p}", address) }
}

fn format_any(value: &(dyn Any + Send + Sync)) -> String {
    format_any_at(value, false)
}

fn format_any_at(value: &(dyn Any + Send + Sync), nested: bool) -> String {
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
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>() {
        match (*v.lock().unwrap()).as_ref() { Some(__err) => format!("{}", __err), None => "<nil>".to_string() }
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<lazyPayload>>>>() {
        match (*v.lock().unwrap()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() }
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<payload>>>>() {
        go_format_any_pointer(nested, Arc::as_ptr(v) as *const (), || v.try_lock().map(|__guard| __guard.is_none()).unwrap_or(false), || match (*v.lock().unwrap()).as_ref() { Some(__pointee) => format!("&{}", __pointee), None => "<nil>".to_string() })
    } else if let Some(v) = value.downcast_ref::<label>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<payload>() {
        format!("{}", v)
    } else {
        panic!("go2rust: cannot print this value held in an interface: its type has no formatting case yet")
    }
}

fn go_any_clone(value: &(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync> {
    if let Some(v) = value.downcast_ref::<i32>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<i64>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<i8>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<i16>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<u32>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<u64>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<u8>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<u16>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<usize>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<isize>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<f64>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<f32>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<String>() { return Box::new(v.clone()) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<&'static str>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<bool>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<char>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>() { return Box::new(v.clone()) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<lazyPayload>>>>() { return Box::new(v.clone()) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<payload>>>>() { return Box::new(v.clone()) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<label>() { return Box::new(v.clone()) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<payload>() { return Box::new(v.__go_value_clone()) as Box<dyn Any + Send + Sync>; }

    panic!("go_any_clone: unsupported dynamic type; add typed lowering instead of cloning Box<dyn Any>")
}

struct GoPanicHookSuppression;

thread_local! {
    static __GO_PANIC_HOOK_SUPPRESSION_DEPTH: std::cell::Cell<usize> = std::cell::Cell::new(0);
    static __GO_FUNCTION_DEPTH: std::cell::Cell<usize> = std::cell::Cell::new(0);
}

static __GO_PANIC_HOOK_INIT: std::sync::Once = std::sync::Once::new();

struct GoFunctionDepthGuard;

fn go_enter_function() -> GoFunctionDepthGuard {
    __GO_FUNCTION_DEPTH.with(|depth| depth.set(depth.get() + 1));
    GoFunctionDepthGuard
}

fn go_current_function_depth() -> usize {
    __GO_FUNCTION_DEPTH.with(|depth| depth.get())
}

impl Drop for GoFunctionDepthGuard {
    fn drop(&mut self) {
        __GO_FUNCTION_DEPTH.with(|depth| depth.set(depth.get() - 1));
    }
}

fn go_suppress_panic_hook() -> GoPanicHookSuppression {
    __GO_PANIC_HOOK_INIT.call_once(|| {
        let previous = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            let suppressed = __GO_PANIC_HOOK_SUPPRESSION_DEPTH.with(|depth| depth.get() != 0);
            if !suppressed {
                previous(info);
            }
        }));
    });
    __GO_PANIC_HOOK_SUPPRESSION_DEPTH.with(|depth| depth.set(depth.get() + 1));
    GoPanicHookSuppression
}

impl Drop for GoPanicHookSuppression {
    fn drop(&mut self) {
        __GO_PANIC_HOOK_SUPPRESSION_DEPTH.with(|depth| depth.set(depth.get() - 1));
    }
}

struct GoPanicFrame {
    payload: Option<Box<dyn Any + Send + Sync>>,
    recover_depth: Option<usize>,
}

thread_local! {
    static __GO_RECOVER_PAYLOADS: RefCell<std::vec::Vec<GoPanicFrame>> = RefCell::new(std::vec::Vec::new());
}

fn go_recover() -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
    let payload = __GO_RECOVER_PAYLOADS.with(|stack| {
        stack.borrow_mut().last_mut().and_then(|frame| {
            if frame.recover_depth == Some(go_current_function_depth()) { frame.payload.take() } else { None }
        })
    });
    Arc::new(Mutex::new(payload))
}

fn go_push_panic_payload(payload: Box<dyn Any + Send + Sync>) {
    __GO_RECOVER_PAYLOADS.with(|stack| stack.borrow_mut().push(GoPanicFrame {
        payload: Some(payload),
        recover_depth: None,
    }));
}

fn go_call_deferred(f: Box<dyn FnOnce()>, recoverable: bool) -> Result<(), Box<dyn Any + Send>> {
    let previous = __GO_RECOVER_PAYLOADS.with(|stack| {
        stack.borrow_mut().last_mut().map(|frame| {
            let previous = frame.recover_depth;
            frame.recover_depth = if recoverable { Some(go_current_function_depth() + 1) } else { None };
            previous
        })
    });
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
    if let Some(previous) = previous {
        __GO_RECOVER_PAYLOADS.with(|stack| {
            if let Some(frame) = stack.borrow_mut().last_mut() {
                frame.recover_depth = previous;
            }
        });
    }
    result
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
                        go_push_panic_payload(payload);
                        return;
                    }
                }
            }
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<String>() {
        Ok(value) => {
            go_push_panic_payload(Box::new(go_runtime_error_text(*value)) as Box<dyn Any + Send + Sync>);
            return;
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<&'static str>() {
        Ok(value) => {
            go_push_panic_payload(Box::new(go_runtime_error_text(value.to_string())) as Box<dyn Any + Send + Sync>);
            return;
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<i32>() {
        Ok(value) => {
            go_push_panic_payload(Box::new(*value) as Box<dyn Any + Send + Sync>);
            return;
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<i64>() {
        Ok(value) => {
            go_push_panic_payload(Box::new(*value) as Box<dyn Any + Send + Sync>);
            return;
        }
        Err(payload) => payload,
    };
    let _payload = match payload.downcast::<bool>() {
        Ok(value) => {
            go_push_panic_payload(Box::new(*value) as Box<dyn Any + Send + Sync>);
            return;
        }
        Err(_payload) => _payload,
    };
    panic!("recover: unsupported Rust panic payload; emit panic_any with a Go any payload instead")
}

fn go_replace_panic_payload(payload: Box<dyn Any + Send>) {
    __GO_RECOVER_PAYLOADS.with(|stack| { stack.borrow_mut().pop(); });
    go_store_panic_payload(payload);
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
    __text
}

fn go_resume_unrecovered_panic() {
    if let Some(GoPanicFrame { payload: Some(payload), .. }) = __GO_RECOVER_PAYLOADS.with(|stack| stack.borrow_mut().pop()) {
        std::panic::panic_any(payload);
    }
}

fn go_terminate_unrecovered_goroutine_panic(
    payload: Box<dyn Any + Send>,
    formatter: fn(&(dyn Any + Send + Sync)) -> String,
) -> ! {
    let payload: Box<dyn Any + Send + Sync> = match payload.downcast::<Box<dyn Any + Send + Sync>>() {
        Ok(boxed) => *boxed,
        Err(payload) => match payload.downcast::<String>() {
            Ok(value) => Box::new(go_runtime_error_text(*value)),
            Err(payload) => match payload.downcast::<&'static str>() {
                Ok(value) => Box::new(go_runtime_error_text(value.to_string())),
                Err(payload) => match payload.downcast::<i32>() {
                    Ok(value) => Box::new(*value),
                    Err(payload) => match payload.downcast::<i64>() {
                        Ok(value) => Box::new(*value),
                        Err(payload) => match payload.downcast::<bool>() {
                            Ok(value) => Box::new(*value),
                            Err(_) => {
                                eprintln!("panic: unrecovered Go panic");
                                std::process::exit(2)
                            }
                        },
                    },
                },
            },
        },
    };
    let mut payload = payload;
    loop {
        match payload.downcast::<Box<dyn Any + Send + Sync>>() {
            Ok(boxed) => payload = *boxed,
            Err(value) => {
                eprintln!("panic: {}", formatter(value.as_ref()));
                std::process::exit(2)
            }
        }
    }
}

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

fn go_any_value_metadata_key(value: &(dyn Any + Send + Sync)) -> usize {
    value as *const (dyn Any + Send + Sync) as *const () as usize
}

fn go_register_any_type<T: Any + Send + Sync + 'static>(kind: &'static str, comparable: bool) {
    go_any_type_metadata_registry().lock().unwrap().insert(std::any::TypeId::of::<T>(), GoAnyTypeMetadata { kind, comparable, elem_kind: None, elem_comparable: false });
}

fn go_register_any_type_with_elem<T: Any + Send + Sync + 'static>(kind: &'static str, comparable: bool, elem_kind: &'static str, elem_comparable: bool) {
    go_any_type_metadata_registry().lock().unwrap().insert(std::any::TypeId::of::<T>(), GoAnyTypeMetadata { kind, comparable, elem_kind: Some(elem_kind), elem_comparable });
}

// Boxes value as itself and records what kind of Go type it is. Every reader
// of an interface value looks for the value's own type in the box, so the
// box must hold nothing else: a wrapper around the value would be a type no
// reader knows.
fn go_box_any_with_metadata<T: Any + Send + Sync + 'static>(value: T, kind: &'static str, comparable: bool) -> Box<dyn Any + Send + Sync> {
    go_register_any_type::<T>(kind, comparable);
    Box::new(value) as Box<dyn Any + Send + Sync>
}

fn go_register_any_value_metadata(value: &(dyn Any + Send + Sync), kind: &'static str, comparable: bool) {
    go_any_value_metadata_registry().lock().unwrap().insert(go_any_value_metadata_key(value), GoAnyTypeMetadata { kind, comparable, elem_kind: None, elem_comparable: false });
}

fn go_any_type_metadata(value: &(dyn Any + Send + Sync)) -> Option<GoAnyTypeMetadata> {
    go_any_value_metadata_registry().lock().unwrap().get(&go_any_value_metadata_key(value)).copied()
        .or_else(|| go_any_type_metadata_registry().lock().unwrap().get(&value.type_id()).copied())
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
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
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
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        panic!("errors_joinError.error bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone)]
pub struct payload {
    pub value: Arc<Mutex<Option<i64>>>,
}

impl payload {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.value.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            value: __go_clone_0_0,
        }
    }
}


impl Default for payload {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            value: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for payload {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.value.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone, Default)]
pub struct lazyPayload {
}

impl lazyPayload {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
    }
}

impl std::fmt::Display for lazyPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}


#[derive(Debug, Default)]
pub struct label(pub Arc<Mutex<Option<String>>>);

impl Clone for label {
    fn clone(&self) -> Self {
        let __value = (*self.0.lock().unwrap()).clone();
        label(Arc::new(Mutex::new(__value)))
    }
}

impl Display for label {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for label {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialOrd for label {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ord::cmp(self, other))
    }
}

impl Eq for label {}

impl Ord for label {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}

impl PartialEq<String> for label {
    fn eq(&self, other: &String) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialEq<&str> for label {
    fn eq(&self, other: &&str) -> bool {
        self.0.lock().unwrap().as_ref().unwrap().as_str() == *other
    }
}

impl std::ops::Add for label {
    type Output = label;
    fn add(self, other: Self) -> label {
        let __joined = format!("{}{}", self.0.lock().unwrap().as_ref().unwrap(), other.0.lock().unwrap().as_ref().unwrap());
        label(Arc::new(Mutex::new(Some(__joined))))
    }
}

impl std::ops::Add<String> for label {
    type Output = label;
    fn add(self, other: String) -> label {
        let __joined = format!("{}{}", self.0.lock().unwrap().as_ref().unwrap(), other);
        label(Arc::new(Mutex::new(Some(__joined))))
    }
}

impl std::ops::Add<&str> for label {
    type Output = label;
    fn add(self, other: &str) -> label {
        let __joined = format!("{}{}", self.0.lock().unwrap().as_ref().unwrap(), other);
        label(Arc::new(Mutex::new(Some(__joined))))
    }
}


impl lazyPayload {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        let __go_function_depth_guard = go_enter_function();
        std::panic::panic_any(Box::new("formatted while recover was still possible".to_string()) as Box<dyn Any + Send + Sync>);
    }
}

pub fn classify_recovered(result: Arc<Mutex<Option<String>>>, original: Arc<Mutex<Option<payload>>>) {
    let __go_function_depth_guard = go_enter_function();
    {
    let _ts_subject = go_recover().clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| {
        let mut __any = __v.as_ref() as &dyn Any;
        while let Some(__boxed) = __any.downcast_ref::<Box<dyn Any + Send + Sync>>() {
            __any = __boxed.as_ref() as &dyn Any;
        }
        __any
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>()).is_some() {
        let recovered = { let __err_source = _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>()).unwrap(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) };
        drop(_ts_guard);
        { let new_val = (*Arc::new(Mutex::new(Some(format!("error true {}", (*Arc::new(Mutex::new(Some(format!("{}", recovered.lock().unwrap().as_ref().unwrap())))).lock().unwrap().as_ref().unwrap()))))).lock().unwrap().as_ref().unwrap()).clone(); *result.lock().unwrap() = Some(new_val); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<payload>()).is_some() {
        let recovered = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<payload>()).unwrap().__go_value_clone())));
        drop(_ts_guard);
        { let new_val = 11; *(*recovered.lock().unwrap().as_ref().unwrap()).value.lock().unwrap() = Some(new_val); };;
        { let new_val = (*Arc::new(Mutex::new(Some(format!("struct true {} {}", format!("{}", (*{ let __field = (*recovered.lock().unwrap().as_ref().unwrap()).value.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __field = (*original.lock().unwrap().as_ref().unwrap()).value.clone(); __field }.lock().unwrap().as_ref().unwrap())))))).lock().unwrap().as_ref().unwrap()).clone(); *result.lock().unwrap() = Some(new_val); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<payload>>>>()).is_some() {
        let recovered = _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<payload>>>>()).unwrap().clone();
        drop(_ts_guard);
        { let new_val = (*Arc::new(Mutex::new(Some(format!("pointer true {} {}", format!("{}", {
    let __left = recovered.clone();
    let __right = original.clone();
    let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
    let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
    __eq
}), format!("{}", (*{ let __field = (*recovered.lock().unwrap().as_ref().unwrap()).value.clone(); __field }.lock().unwrap().as_ref().unwrap())))))).lock().unwrap().as_ref().unwrap()).clone(); *result.lock().unwrap() = Some(new_val); };;
        { let new_val = 10; *(*recovered.lock().unwrap().as_ref().unwrap()).value.lock().unwrap() = Some(new_val); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<i64>()).is_some() {
        let recovered = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<i64>()).unwrap().clone())));
        drop(_ts_guard);
        { let new_val = (*Arc::new(Mutex::new(Some(format!("int true {}", { let __v = (*recovered.lock().unwrap().as_ref().unwrap()).clone(); __v })))).lock().unwrap().as_ref().unwrap()).clone(); *result.lock().unwrap() = Some(new_val); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<String>()).is_some() {
        let recovered = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<String>()).unwrap().clone())));
        drop(_ts_guard);
        { let new_val = (*Arc::new(Mutex::new(Some(format!("string true {}", { let __v = (*recovered.lock().unwrap().as_ref().unwrap()).clone(); __v })))).lock().unwrap().as_ref().unwrap()).clone(); *result.lock().unwrap() = Some(new_val); };;
    } else {
        let recovered = _ts_subject.clone();
        drop(_ts_guard);
        { let new_val = "unknown false".to_string(); *result.lock().unwrap() = Some(new_val); };;
    }
    }
}

pub fn recover_value(value: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>, original: Arc<Mutex<Option<payload>>>) -> Arc<Mutex<Option<String>>> {
    let __go_function_depth_guard = go_enter_function();
    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    let mut result: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));

    let __go_panic_hook_guard = go_suppress_panic_hook();
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let original_defer_captured = original.clone(); let result_defer_captured = result.clone(); __defer_stack.push(Box::new(move || {
        classify_recovered(result_defer_captured.clone(), original_defer_captured.clone());
    }));
        { let __any_holder = value.clone(); let __any_payload = { let __any_guard = __any_holder.lock().unwrap(); __any_guard.as_ref().map(|__any_value| go_any_clone(__any_value.as_ref())) }; match __any_payload { Some(__any_value) => std::panic::panic_any(__any_value), None => { let __go_panic_nil = Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("panic called with nil argument".to_string()))))); std::panic::panic_any(Box::new(__go_panic_nil) as Box<dyn Any + Send + Sync>) } } };
    }));
    match __go_panic_result {
        Ok(__go_value) => {
            drop(__go_panic_hook_guard);
            __go_value
        }
        Err(__go_panic_payload) => {
            go_store_panic_payload(__go_panic_payload);
            while let Some(f) = __defer_stack.pop() {
                if let Err(__go_defer_panic) = go_call_deferred(f, true) {
                    go_replace_panic_payload(__go_defer_panic);
                }
            }
            drop(__go_panic_hook_guard);
            go_resume_unrecovered_panic();
            result.clone()
        }
    }
}

pub fn print_recovered(label_local: Arc<Mutex<Option<String>>>) {
    let __go_function_depth_guard = go_enter_function();
    let (mut value, mut ok) = ({
        let val = go_recover().clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<std::string::String>() {
                (Arc::new(Mutex::new(Some(typed_val.clone()))), true)
            } else {
                (Arc::new(Mutex::new(Some(std::string::String::new()))), false)
            }
        } else {
            (Arc::new(Mutex::new(Some(std::string::String::new()))), false)
        }
    });
    println!("{} {} {}", format!("{}", { let __v = (*label_local.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", ok), format!("{}", { let __v = (*value.lock().unwrap().as_ref().unwrap()).clone(); __v }));
}

pub fn inner_panic() {
    let __go_function_depth_guard = go_enter_function();
    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    let __go_panic_hook_guard = go_suppress_panic_hook();
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        __defer_stack.push(Box::new(move || {
        print_recovered(Arc::new(Mutex::new(Some("inner".to_string()))));
    }));
        std::panic::panic_any(Box::new("B".to_string()) as Box<dyn Any + Send + Sync>);
    }));
    match __go_panic_result {
        Ok(__go_value) => {
            drop(__go_panic_hook_guard);
            __go_value
        }
        Err(__go_panic_payload) => {
            go_store_panic_payload(__go_panic_payload);
            while let Some(f) = __defer_stack.pop() {
                if let Err(__go_defer_panic) = go_call_deferred(f, true) {
                    go_replace_panic_payload(__go_defer_panic);
                }
            }
            drop(__go_panic_hook_guard);
            go_resume_unrecovered_panic();
            ()
        }
    }
}

pub fn outer_panic() {
    let __go_function_depth_guard = go_enter_function();
    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    let __go_panic_hook_guard = go_suppress_panic_hook();
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        let __go_function_depth_guard = go_enter_function();
        inner_panic();
        println!("{}", format!("{}", "after inner".to_string()));
        ()
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
        std::panic::panic_any(Box::new("A".to_string()) as Box<dyn Any + Send + Sync>);
    }));
    match __go_panic_result {
        Ok(__go_value) => {
            drop(__go_panic_hook_guard);
            __go_value
        }
        Err(__go_panic_payload) => {
            go_store_panic_payload(__go_panic_payload);
            while let Some(f) = __defer_stack.pop() {
                if let Err(__go_defer_panic) = go_call_deferred(f, true) {
                    go_replace_panic_payload(__go_defer_panic);
                }
            }
            drop(__go_panic_hook_guard);
            go_resume_unrecovered_panic();
            ()
        }
    }
}

pub fn save_outer_panic(result: Arc<Mutex<Option<String>>>) {
    let __go_function_depth_guard = go_enter_function();
    let (mut value, mut ok) = ({
        let val = go_recover().clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<std::string::String>() {
                (Arc::new(Mutex::new(Some(typed_val.clone()))), true)
            } else {
                (Arc::new(Mutex::new(Some(std::string::String::new()))), false)
            }
        } else {
            (Arc::new(Mutex::new(Some(std::string::String::new()))), false)
        }
    });
    if ok {
        { let new_val = (*Arc::new(Mutex::new(Some(format!("{}", { let __v = (*value.lock().unwrap().as_ref().unwrap()).clone(); __v })))).lock().unwrap().as_ref().unwrap()).clone(); *result.lock().unwrap() = Some(new_val); };
    }
}

pub fn nested_panic() -> Arc<Mutex<Option<String>>> {
    let __go_function_depth_guard = go_enter_function();
    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    let mut result: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));

    let __go_panic_hook_guard = go_suppress_panic_hook();
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let result_defer_captured = result.clone(); __defer_stack.push(Box::new(move || {
        save_outer_panic(result_defer_captured.clone());
    }));
        outer_panic();
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            if let Err(__go_defer_panic) = go_call_deferred(f, false) {
                std::panic::resume_unwind(__go_defer_panic);
            }
        }
        return result.clone();
    }
    }));
    match __go_panic_result {
        Ok(__go_value) => {
            drop(__go_panic_hook_guard);
            __go_value
        }
        Err(__go_panic_payload) => {
            go_store_panic_payload(__go_panic_payload);
            while let Some(f) = __defer_stack.pop() {
                if let Err(__go_defer_panic) = go_call_deferred(f, true) {
                    go_replace_panic_payload(__go_defer_panic);
                }
            }
            drop(__go_panic_hook_guard);
            go_resume_unrecovered_panic();
            result.clone()
        }
    }
}

pub fn explicit_runtime_text() -> Arc<Mutex<Option<String>>> {
    let __go_function_depth_guard = go_enter_function();
    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    let mut result: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));

    let __go_panic_hook_guard = go_suppress_panic_hook();
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let result_defer_captured = result.clone(); __defer_stack.push(Box::new(move || {
        save_outer_panic(result_defer_captured.clone());
    }));
        std::panic::panic_any(Box::new("attempt to divide by zero".to_string()) as Box<dyn Any + Send + Sync>);
    }));
    match __go_panic_result {
        Ok(__go_value) => {
            drop(__go_panic_hook_guard);
            __go_value
        }
        Err(__go_panic_payload) => {
            go_store_panic_payload(__go_panic_payload);
            while let Some(f) = __defer_stack.pop() {
                if let Err(__go_defer_panic) = go_call_deferred(f, true) {
                    go_replace_panic_payload(__go_defer_panic);
                }
            }
            drop(__go_panic_hook_guard);
            go_resume_unrecovered_panic();
            result.clone()
        }
    }
}

pub fn panic_b() {
    let __go_function_depth_guard = go_enter_function();
    std::panic::panic_any(Box::new("B".to_string()) as Box<dyn Any + Send + Sync>);
}

pub fn replacement_panic() {
    let __go_function_depth_guard = go_enter_function();
    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    let __go_panic_hook_guard = go_suppress_panic_hook();
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        __defer_stack.push(Box::new(move || {
        print_recovered(Arc::new(Mutex::new(Some("replacement".to_string()))));
    }));
        __defer_stack.push(Box::new(move || {
        panic_b();
    }));
        std::panic::panic_any(Box::new("A".to_string()) as Box<dyn Any + Send + Sync>);
    }));
    match __go_panic_result {
        Ok(__go_value) => {
            drop(__go_panic_hook_guard);
            __go_value
        }
        Err(__go_panic_payload) => {
            go_store_panic_payload(__go_panic_payload);
            while let Some(f) = __defer_stack.pop() {
                if let Err(__go_defer_panic) = go_call_deferred(f, true) {
                    go_replace_panic_payload(__go_defer_panic);
                }
            }
            drop(__go_panic_hook_guard);
            go_resume_unrecovered_panic();
            ()
        }
    }
}

pub fn save_probe(result: Arc<Mutex<Option<String>>>) {
    let __go_function_depth_guard = go_enter_function();
    let mut value = go_recover();
    if { let __nil_result = (*value.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = "nil".to_string(); *result.lock().unwrap() = Some(new_val); };
        return;
    }
    { let new_val = (*Arc::new(Mutex::new(Some(format!("{}", { let __text = format_any(value.lock().unwrap().as_ref().unwrap().as_ref()); __text })))).lock().unwrap().as_ref().unwrap()).clone(); *result.lock().unwrap() = Some(new_val); };
}

pub fn probe() -> Arc<Mutex<Option<String>>> {
    let __go_function_depth_guard = go_enter_function();
    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    let mut result: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));

    let __go_panic_hook_guard = go_suppress_panic_hook();
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let result_defer_captured = result.clone(); __defer_stack.push(Box::new(move || {
        save_probe(result_defer_captured.clone());
    }));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            if let Err(__go_defer_panic) = go_call_deferred(f, false) {
                std::panic::resume_unwind(__go_defer_panic);
            }
        }
        return result.clone();
    }
    }));
    match __go_panic_result {
        Ok(__go_value) => {
            drop(__go_panic_hook_guard);
            __go_value
        }
        Err(__go_panic_payload) => {
            go_store_panic_payload(__go_panic_payload);
            while let Some(f) = __defer_stack.pop() {
                if let Err(__go_defer_panic) = go_call_deferred(f, true) {
                    go_replace_panic_payload(__go_defer_panic);
                }
            }
            drop(__go_panic_hook_guard);
            go_resume_unrecovered_panic();
            result.clone()
        }
    }
}

pub fn normal_probe_cannot_recover_outer() -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) {
    let __go_function_depth_guard = go_enter_function();
    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    let mut outer: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut probed: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));

    let __go_panic_hook_guard = go_suppress_panic_hook();
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let outer_defer_captured = outer.clone(); __defer_stack.push(Box::new(move || {
        save_outer_panic(outer_defer_captured.clone());
    }));
        let mut probed_defer_captured = probed.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        let __go_function_depth_guard = go_enter_function();
        { let new_val = probe(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *probed_defer_captured.lock().unwrap() = __moved_val; };
        ()
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
        std::panic::panic_any(Box::new("outer".to_string()) as Box<dyn Any + Send + Sync>);
    }));
    match __go_panic_result {
        Ok(__go_value) => {
            drop(__go_panic_hook_guard);
            __go_value
        }
        Err(__go_panic_payload) => {
            go_store_panic_payload(__go_panic_payload);
            while let Some(f) = __defer_stack.pop() {
                if let Err(__go_defer_panic) = go_call_deferred(f, true) {
                    go_replace_panic_payload(__go_defer_panic);
                }
            }
            drop(__go_panic_hook_guard);
            go_resume_unrecovered_panic();
            (outer.clone(), probed.clone())
        }
    }
}

pub fn indirect_recover() -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
    let __go_function_depth_guard = go_enter_function();
    go_recover().clone()
}

pub fn only_direct_deferred_call_can_recover() -> Arc<Mutex<Option<String>>> {
    let __go_function_depth_guard = go_enter_function();
    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    let mut result: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));

    let __go_panic_hook_guard = go_suppress_panic_hook();
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut result_defer_captured = result.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        let __go_function_depth_guard = go_enter_function();
        let mut indirect = indirect_recover();
        let mut direct = go_recover();
        { let new_val = Arc::new(Mutex::new(Some(format!("depth {} {}", format!("{}", { let __nil_result = (*indirect.lock().unwrap()).is_none(); __nil_result }), format!("{}", { let __text = format_any(direct.lock().unwrap().as_ref().unwrap().as_ref()); __text }))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *result_defer_captured.lock().unwrap() = __moved_val; };
        ()
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
        std::panic::panic_any(Box::new("direct".to_string()) as Box<dyn Any + Send + Sync>);
    }));
    match __go_panic_result {
        Ok(__go_value) => {
            drop(__go_panic_hook_guard);
            __go_value
        }
        Err(__go_panic_payload) => {
            go_store_panic_payload(__go_panic_payload);
            while let Some(f) = __defer_stack.pop() {
                if let Err(__go_defer_panic) = go_call_deferred(f, true) {
                    go_replace_panic_payload(__go_defer_panic);
                }
            }
            drop(__go_panic_hook_guard);
            go_resume_unrecovered_panic();
            result.clone()
        }
    }
}

pub fn formatter_runs_only_for_uncaught_panic() -> Arc<Mutex<Option<String>>> {
    let __go_function_depth_guard = go_enter_function();
    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    let mut result: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));

    let __go_panic_hook_guard = go_suppress_panic_hook();
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut result_defer_captured = result.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        let __go_function_depth_guard = go_enter_function();
        let (_, mut ok) = ({
        let val = go_recover().clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<lazyPayload>>>>() {
                (typed_val.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<lazyPayload>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<lazyPayload>)), false)
        }
    });
        { let new_val = Arc::new(Mutex::new(Some(format!("lazy {}", ok)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *result_defer_captured.lock().unwrap() = __moved_val; };
        ()
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
        std::panic::panic_any(Box::new(Arc::new(Mutex::new(Some(lazyPayload {  }))).clone()) as Box<dyn Any + Send + Sync>);
    }));
    match __go_panic_result {
        Ok(__go_value) => {
            drop(__go_panic_hook_guard);
            __go_value
        }
        Err(__go_panic_payload) => {
            go_store_panic_payload(__go_panic_payload);
            while let Some(f) = __defer_stack.pop() {
                if let Err(__go_defer_panic) = go_call_deferred(f, true) {
                    go_replace_panic_payload(__go_defer_panic);
                }
            }
            drop(__go_panic_hook_guard);
            go_resume_unrecovered_panic();
            result.clone()
        }
    }
}

pub fn recover_nil_value(useLiteral: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<String>>> {
    let __go_function_depth_guard = go_enter_function();
    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    let mut result: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));

    let __go_panic_hook_guard = go_suppress_panic_hook();
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut result_defer_captured = result.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        let __go_function_depth_guard = go_enter_function();
        let mut recovered = go_recover();
        let (mut err, mut ok) = ({
        let val = recovered.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>() {
                ({ let __err_source = typed_val; let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }, true)
            } else {
                (Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>)), false)
        }
    });
        if !ok {
        { let new_val = "nil false".to_string(); *result_defer_captured.lock().unwrap() = Some(new_val); };
        return;
    }
        { let new_val = Arc::new(Mutex::new(Some(format!("nil {} {}", format!("{}", ok), format!("{}", (*Arc::new(Mutex::new(Some(format!("{}", err.lock().unwrap().as_ref().unwrap())))).lock().unwrap().as_ref().unwrap())))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *result_defer_captured.lock().unwrap() = __moved_val; };
        ()
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
        if { let __v = (*useLiteral.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let __go_panic_nil = Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("panic called with nil argument".to_string()))))); std::panic::panic_any(Box::new(__go_panic_nil) as Box<dyn Any + Send + Sync>) };
    }
        let mut value: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(None));
        { let __any_holder = value.clone(); let __any_payload = { let __any_guard = __any_holder.lock().unwrap(); __any_guard.as_ref().map(|__any_value| go_any_clone(__any_value.as_ref())) }; match __any_payload { Some(__any_value) => std::panic::panic_any(__any_value), None => { let __go_panic_nil = Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("panic called with nil argument".to_string()))))); std::panic::panic_any(Box::new(__go_panic_nil) as Box<dyn Any + Send + Sync>) } } };
    }));
    match __go_panic_result {
        Ok(__go_value) => {
            drop(__go_panic_hook_guard);
            __go_value
        }
        Err(__go_panic_payload) => {
            go_store_panic_payload(__go_panic_payload);
            while let Some(f) = __defer_stack.pop() {
                if let Err(__go_defer_panic) = go_call_deferred(f, true) {
                    go_replace_panic_payload(__go_defer_panic);
                }
            }
            drop(__go_panic_hook_guard);
            go_resume_unrecovered_panic();
            result.clone()
        }
    }
}

pub fn explicit_string_is_not_error() -> Arc<Mutex<Option<String>>> {
    let __go_function_depth_guard = go_enter_function();
    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    let mut result: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));

    let __go_panic_hook_guard = go_suppress_panic_hook();
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut result_defer_captured = result.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        let __go_function_depth_guard = go_enter_function();
        let (_, mut ok) = ({
        let val = go_recover().clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>() {
                ({ let __err_source = typed_val; let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }, true)
            } else {
                (Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>)), false)
        }
    });
        { let new_val = Arc::new(Mutex::new(Some(format!("string error {}", ok)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *result_defer_captured.lock().unwrap() = __moved_val; };
        ()
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
        std::panic::panic_any(Box::new("panic called with nil argument".to_string()) as Box<dyn Any + Send + Sync>);
    }));
    match __go_panic_result {
        Ok(__go_value) => {
            drop(__go_panic_hook_guard);
            __go_value
        }
        Err(__go_panic_payload) => {
            go_store_panic_payload(__go_panic_payload);
            while let Some(f) = __defer_stack.pop() {
                if let Err(__go_defer_panic) = go_call_deferred(f, true) {
                    go_replace_panic_payload(__go_defer_panic);
                }
            }
            drop(__go_panic_hook_guard);
            go_resume_unrecovered_panic();
            result.clone()
        }
    }
}

pub fn recover_named_string() -> Arc<Mutex<Option<String>>> {
    let __go_function_depth_guard = go_enter_function();
    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    let mut result: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));

    let __go_panic_hook_guard = go_suppress_panic_hook();
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut result_defer_captured = result.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        let __go_function_depth_guard = go_enter_function();
        let mut recovered = go_recover();
        let (mut value, mut namedOK) = ({
        let val = recovered.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<label>() {
                (Arc::new(Mutex::new(Some(typed_val.clone()))), true)
            } else {
                (Arc::new(Mutex::new(Some(Default::default()))), false)
            }
        } else {
            (Arc::new(Mutex::new(Some(Default::default()))), false)
        }
    });
        let (_, mut stringOK) = ({
        let val = recovered.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<std::string::String>() {
                (Arc::new(Mutex::new(Some(typed_val.clone()))), true)
            } else {
                (Arc::new(Mutex::new(Some(std::string::String::new()))), false)
            }
        } else {
            (Arc::new(Mutex::new(Some(std::string::String::new()))), false)
        }
    });
        { let new_val = Arc::new(Mutex::new(Some(format!("named {} {} {}", format!("{}", namedOK), format!("{}", stringOK), format!("{}", { let __v = (*value.lock().unwrap().as_ref().unwrap()).clone(); __v }))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *result_defer_captured.lock().unwrap() = __moved_val; };
        ()
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
        std::panic::panic_any(Box::new(label(Arc::new(Mutex::new(Some("kept".to_string()))))) as Box<dyn Any + Send + Sync>);
    }));
    match __go_panic_result {
        Ok(__go_value) => {
            drop(__go_panic_hook_guard);
            __go_value
        }
        Err(__go_panic_payload) => {
            go_store_panic_payload(__go_panic_payload);
            while let Some(f) = __defer_stack.pop() {
                if let Err(__go_defer_panic) = go_call_deferred(f, true) {
                    go_replace_panic_payload(__go_defer_panic);
                }
            }
            drop(__go_panic_hook_guard);
            go_resume_unrecovered_panic();
            result.clone()
        }
    }
}

pub fn recover_shadowed_string(nil: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    let __go_function_depth_guard = go_enter_function();
    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    let mut result: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));

    let __go_panic_hook_guard = go_suppress_panic_hook();
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut result_defer_captured = result.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        let __go_function_depth_guard = go_enter_function();
        let (mut value, mut ok) = ({
        let val = go_recover().clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<std::string::String>() {
                (Arc::new(Mutex::new(Some(typed_val.clone()))), true)
            } else {
                (Arc::new(Mutex::new(Some(std::string::String::new()))), false)
            }
        } else {
            (Arc::new(Mutex::new(Some(std::string::String::new()))), false)
        }
    });
        { let new_val = Arc::new(Mutex::new(Some(format!("shadow string {} {}", format!("{}", ok), format!("{}", { let __v = (*value.lock().unwrap().as_ref().unwrap()).clone(); __v }))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *result_defer_captured.lock().unwrap() = __moved_val; };
        ()
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
        std::panic::panic_any(Box::new({ let __v = (*nil.lock().unwrap().as_ref().unwrap()).clone(); __v }) as Box<dyn Any + Send + Sync>);
    }));
    match __go_panic_result {
        Ok(__go_value) => {
            drop(__go_panic_hook_guard);
            __go_value
        }
        Err(__go_panic_payload) => {
            go_store_panic_payload(__go_panic_payload);
            while let Some(f) = __defer_stack.pop() {
                if let Err(__go_defer_panic) = go_call_deferred(f, true) {
                    go_replace_panic_payload(__go_defer_panic);
                }
            }
            drop(__go_panic_hook_guard);
            go_resume_unrecovered_panic();
            result.clone()
        }
    }
}

pub fn recover_shadowed_int(nil: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<String>>> {
    let __go_function_depth_guard = go_enter_function();
    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    let mut result: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));

    let __go_panic_hook_guard = go_suppress_panic_hook();
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut result_defer_captured = result.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        let __go_function_depth_guard = go_enter_function();
        let (mut value, mut ok) = ({
        let val = go_recover().clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<i64>() {
                (Arc::new(Mutex::new(Some(typed_val.clone()))), true)
            } else {
                (Arc::new(Mutex::new(Some(0))), false)
            }
        } else {
            (Arc::new(Mutex::new(Some(0))), false)
        }
    });
        { let new_val = Arc::new(Mutex::new(Some(format!("shadow int {} {}", format!("{}", ok), format!("{}", { let __v = (*value.lock().unwrap().as_ref().unwrap()).clone(); __v }))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *result_defer_captured.lock().unwrap() = __moved_val; };
        ()
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
        std::panic::panic_any(Box::new({ let __v = (*nil.lock().unwrap().as_ref().unwrap()).clone(); __v }) as Box<dyn Any + Send + Sync>);
    }));
    match __go_panic_result {
        Ok(__go_value) => {
            drop(__go_panic_hook_guard);
            __go_value
        }
        Err(__go_panic_payload) => {
            go_store_panic_payload(__go_panic_payload);
            while let Some(f) = __defer_stack.pop() {
                if let Err(__go_defer_panic) = go_call_deferred(f, true) {
                    go_replace_panic_payload(__go_defer_panic);
                }
            }
            drop(__go_panic_hook_guard);
            go_resume_unrecovered_panic();
            result.clone()
        }
    }
}

pub fn recover_slice_element() -> Arc<Mutex<Option<String>>> {
    let __go_function_depth_guard = go_enter_function();
    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    let mut result: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));

    let __go_panic_hook_guard = go_suppress_panic_hook();
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut result_defer_captured = result.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        let __go_function_depth_guard = go_enter_function();
        let (mut value, mut ok) = ({
        let val = go_recover().clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<std::string::String>() {
                (Arc::new(Mutex::new(Some(typed_val.clone()))), true)
            } else {
                (Arc::new(Mutex::new(Some(std::string::String::new()))), false)
            }
        } else {
            (Arc::new(Mutex::new(Some(std::string::String::new()))), false)
        }
    });
        { let new_val = Arc::new(Mutex::new(Some(format!("slice panic {} {}", format!("{}", ok), format!("{}", { let __v = (*value.lock().unwrap().as_ref().unwrap()).clone(); __v }))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *result_defer_captured.lock().unwrap() = __moved_val; };
        ()
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
        let mut values = Arc::new(Mutex::new(Some(vec![Box::new("element".to_string()) as Box<dyn Any + Send + Sync>])));
        std::panic::panic_any({ let __seq_holder = values.clone(); let __idx = (0) as usize; let __seq_guard = __seq_holder.lock().unwrap(); go_any_clone(__seq_guard.as_ref().expect("nil []any in interface argument")[__idx].as_ref()) });
    }));
    match __go_panic_result {
        Ok(__go_value) => {
            drop(__go_panic_hook_guard);
            __go_value
        }
        Err(__go_panic_payload) => {
            go_store_panic_payload(__go_panic_payload);
            while let Some(f) = __defer_stack.pop() {
                if let Err(__go_defer_panic) = go_call_deferred(f, true) {
                    go_replace_panic_payload(__go_defer_panic);
                }
            }
            drop(__go_panic_hook_guard);
            go_resume_unrecovered_panic();
            result.clone()
        }
    }
}

pub fn recover_in_goroutine_literals() {
    let __go_function_depth_guard = go_enter_function();
    let mut done = GoChannel::<String>::new();
    let done_thread = done.clone(); std::thread::spawn(move || {
        let __go_panic_hook_guard = go_suppress_panic_hook();
        let __go_goroutine_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();
        let __go_function_depth_guard = go_enter_function();
        let __go_panic_hook_guard = go_suppress_panic_hook();
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let done_defer_captured = done_thread.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        let __go_function_depth_guard = go_enter_function();
        let (mut value, mut ok) = ({
        let val = go_recover().clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<std::string::String>() {
                (Arc::new(Mutex::new(Some(typed_val.clone()))), true)
            } else {
                (Arc::new(Mutex::new(Some(std::string::String::new()))), false)
            }
        } else {
            (Arc::new(Mutex::new(Some(std::string::String::new()))), false)
        }
    });
        done_defer_captured.send((*Arc::new(Mutex::new(Some(format!("go literal {} {}", format!("{}", ok), format!("{}", { let __v = (*value.lock().unwrap().as_ref().unwrap()).clone(); __v }))))).lock().unwrap().as_ref().unwrap()).clone());
        ()
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));;
            std::panic::panic_any(Box::new("without args".to_string()) as Box<dyn Any + Send + Sync>);;
            while let Some(f) = __defer_stack.pop() {
                if let Err(__go_defer_panic) = go_call_deferred(f, false) {
                    std::panic::resume_unwind(__go_defer_panic);
                }
            }
        }));
        match __go_panic_result {
            Ok(__go_value) => {
                drop(__go_panic_hook_guard);
                __go_value
            }
            Err(__go_panic_payload) => {
                go_store_panic_payload(__go_panic_payload);
                while let Some(f) = __defer_stack.pop() {
                    if let Err(__go_defer_panic) = go_call_deferred(f, true) {
                        go_replace_panic_payload(__go_defer_panic);
                    }
                }
                drop(__go_panic_hook_guard);
                go_resume_unrecovered_panic();
                ()
            }
        }
;
        }));
        drop(__go_panic_hook_guard);
        if let Err(__go_panic_payload) = __go_goroutine_result {
            go_terminate_unrecovered_goroutine_panic(__go_panic_payload, format_any);
        }
    });
    println!("{}", format!("{}", done.recv().unwrap_or_default()));

    let done_thread = done.clone(); let __go_arg_0 = Arc::new(Mutex::new(Some(1))); std::thread::spawn(move || {
        let __go_panic_hook_guard = go_suppress_panic_hook();
        let __go_goroutine_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();
        let mut __closure = move |code: Arc<Mutex<Option<i64>>>| {
            let __go_function_depth_guard = go_enter_function();
            let __go_panic_hook_guard = go_suppress_panic_hook();
            let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let code_defer_captured = code.clone(); let done_defer_captured = done_thread.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        let __go_function_depth_guard = go_enter_function();
        let (mut value, mut ok) = ({
        let val = go_recover().clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<std::string::String>() {
                (Arc::new(Mutex::new(Some(typed_val.clone()))), true)
            } else {
                (Arc::new(Mutex::new(Some(std::string::String::new()))), false)
            }
        } else {
            (Arc::new(Mutex::new(Some(std::string::String::new()))), false)
        }
    });
        done_defer_captured.send((*Arc::new(Mutex::new(Some(format!("go literal arg {} {} {}", format!("{}", { let __v = (*code_defer_captured.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", ok), format!("{}", { let __v = (*value.lock().unwrap().as_ref().unwrap()).clone(); __v }))))).lock().unwrap().as_ref().unwrap()).clone());
        ()
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));;
                std::panic::panic_any(Box::new("with args".to_string()) as Box<dyn Any + Send + Sync>);;
                while let Some(f) = __defer_stack.pop() {
                    if let Err(__go_defer_panic) = go_call_deferred(f, false) {
                        std::panic::resume_unwind(__go_defer_panic);
                    }
                }
            }));
            match __go_panic_result {
                Ok(__go_value) => {
                    drop(__go_panic_hook_guard);
                    __go_value
                }
                Err(__go_panic_payload) => {
                    go_store_panic_payload(__go_panic_payload);
                    while let Some(f) = __defer_stack.pop() {
                        if let Err(__go_defer_panic) = go_call_deferred(f, true) {
                            go_replace_panic_payload(__go_defer_panic);
                        }
                    }
                    drop(__go_panic_hook_guard);
                    go_resume_unrecovered_panic();
                    ()
                }
            }

        };
        __closure(__go_arg_0);
        }));
        drop(__go_panic_hook_guard);
        if let Err(__go_panic_payload) = __go_goroutine_result {
            go_terminate_unrecovered_goroutine_panic(__go_panic_payload, format_any);
        }
    });
    println!("{}", format!("{}", done.recv().unwrap_or_default()));

    let done_thread = done.clone(); std::thread::spawn(move || {
        let __go_panic_hook_guard = go_suppress_panic_hook();
        let __go_goroutine_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();
        let __go_function_depth_guard = go_enter_function();
        let __go_panic_hook_guard = go_suppress_panic_hook();
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let done_defer_captured = done_thread.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        let __go_function_depth_guard = go_enter_function();
        go_recover();
        done_defer_captured.send("go result recovered".to_string());
        ()
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));;
            std::panic::panic_any(Box::new("result".to_string()) as Box<dyn Any + Send + Sync>);;
            while let Some(f) = __defer_stack.pop() {
                if let Err(__go_defer_panic) = go_call_deferred(f, false) {
                    std::panic::resume_unwind(__go_defer_panic);
                }
            }
        }));
        match __go_panic_result {
            Ok(__go_value) => {
                drop(__go_panic_hook_guard);
                __go_value
            }
            Err(__go_panic_payload) => {
                go_store_panic_payload(__go_panic_payload);
                while let Some(f) = __defer_stack.pop() {
                    if let Err(__go_defer_panic) = go_call_deferred(f, true) {
                        go_replace_panic_payload(__go_defer_panic);
                    }
                }
                drop(__go_panic_hook_guard);
                go_resume_unrecovered_panic();
                ()
            }
        }
;
        }));
        drop(__go_panic_hook_guard);
        if let Err(__go_panic_payload) = __go_goroutine_result {
            go_terminate_unrecovered_goroutine_panic(__go_panic_payload, format_any);
        }
    });
    println!("{}", format!("{}", done.recv().unwrap_or_default()));

    let done_thread = done.clone(); let __go_arg_0 = Arc::new(Mutex::new(Some(2))); std::thread::spawn(move || {
        let __go_panic_hook_guard = go_suppress_panic_hook();
        let __go_goroutine_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();
        let mut __closure = move |code: Arc<Mutex<Option<i64>>>| {
            let __go_function_depth_guard = go_enter_function();
            let __go_panic_hook_guard = go_suppress_panic_hook();
            let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let code_defer_captured = code.clone(); let done_defer_captured = done_thread.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        let __go_function_depth_guard = go_enter_function();
        go_recover();
        done_defer_captured.send((*Arc::new(Mutex::new(Some(format!("go result arg {} recovered", { let __v = (*code_defer_captured.lock().unwrap().as_ref().unwrap()).clone(); __v })))).lock().unwrap().as_ref().unwrap()).clone());
        ()
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));;
                std::panic::panic_any(Box::new("result arg".to_string()) as Box<dyn Any + Send + Sync>);;
                while let Some(f) = __defer_stack.pop() {
                    if let Err(__go_defer_panic) = go_call_deferred(f, false) {
                        std::panic::resume_unwind(__go_defer_panic);
                    }
                }
            }));
            match __go_panic_result {
                Ok(__go_value) => {
                    drop(__go_panic_hook_guard);
                    __go_value
                }
                Err(__go_panic_payload) => {
                    go_store_panic_payload(__go_panic_payload);
                    while let Some(f) = __defer_stack.pop() {
                        if let Err(__go_defer_panic) = go_call_deferred(f, true) {
                            go_replace_panic_payload(__go_defer_panic);
                        }
                    }
                    drop(__go_panic_hook_guard);
                    go_resume_unrecovered_panic();
                    ()
                }
            }

        };
        __closure(__go_arg_0);
        }));
        drop(__go_panic_hook_guard);
        if let Err(__go_panic_payload) = __go_goroutine_result {
            go_terminate_unrecovered_goroutine_panic(__go_panic_payload, format_any);
        }
    });
    println!("{}", format!("{}", done.recv().unwrap_or_default()));
}

pub fn recover_and_read_interface(value: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
    let __go_function_depth_guard = go_enter_function();
    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    let mut result: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));

    let __go_panic_hook_guard = go_suppress_panic_hook();
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut saved = value.clone();
        let mut result_defer_captured = result.clone(); let saved_defer_captured = saved.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        let __go_function_depth_guard = go_enter_function();
        let mut recovered = go_recover();
        { let new_val = Arc::new(Mutex::new(Some(format!("lock {} {}", format!("{}", { let __text = format_any(recovered.lock().unwrap().as_ref().unwrap().as_ref()); __text }), format!("{}", { let __text = format_any(saved.lock().unwrap().as_ref().unwrap().as_ref()); __text }))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *result_defer_captured.lock().unwrap() = __moved_val; };
        ()
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
        { let __any_holder = value.clone(); let __any_payload = { let __any_guard = __any_holder.lock().unwrap(); __any_guard.as_ref().map(|__any_value| go_any_clone(__any_value.as_ref())) }; match __any_payload { Some(__any_value) => std::panic::panic_any(__any_value), None => { let __go_panic_nil = Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("panic called with nil argument".to_string()))))); std::panic::panic_any(Box::new(__go_panic_nil) as Box<dyn Any + Send + Sync>) } } };
    }));
    match __go_panic_result {
        Ok(__go_value) => {
            drop(__go_panic_hook_guard);
            __go_value
        }
        Err(__go_panic_payload) => {
            go_store_panic_payload(__go_panic_payload);
            while let Some(f) = __defer_stack.pop() {
                if let Err(__go_defer_panic) = go_call_deferred(f, true) {
                    go_replace_panic_payload(__go_defer_panic);
                }
            }
            drop(__go_panic_hook_guard);
            go_resume_unrecovered_panic();
            result.clone()
        }
    }
}

fn main() {
    let __go_function_depth_guard = go_enter_function();
        // Force the synchronized representation; the paired fixture covers the
        // same panic payloads with local Rc/RefCell wrappers.
    std::thread::spawn(move || {
        let __go_panic_hook_guard = go_suppress_panic_hook();
        let __go_goroutine_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let __go_function_depth_guard = go_enter_function();
;
        }));
        drop(__go_panic_hook_guard);
        if let Err(__go_panic_payload) = __go_goroutine_result {
            go_terminate_unrecovered_goroutine_panic(__go_panic_payload, format_any);
        }
    });

    println!("{}", format!("{}", (*recover_value(
        Arc::new(Mutex::new(Some({ let __err_holder = Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("boom".to_string()))))).clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => Box::new(Arc::new(Mutex::new(Some(__err.clone())))) as Box<dyn Any + Send + Sync> } }))),
        Arc::new(Mutex::new(None))
    ).lock().unwrap().as_ref().unwrap())));
    let mut structValue = Arc::new(Mutex::new(Some(payload { value: Arc::new(Mutex::new(Some(7))), ..Default::default() })));
    println!("{}", format!("{}", (*recover_value(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = structValue.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }) as Box<dyn Any + Send + Sync>))), structValue.clone()).lock().unwrap().as_ref().unwrap())));

    let mut pointerValue = Arc::new(Mutex::new(Some(payload { value: Arc::new(Mutex::new(Some(8))), ..Default::default() })));
    println!("{}", format!("{}", (*recover_value(Arc::new(Mutex::new(Some(Box::new(pointerValue.clone()) as Box<dyn Any + Send + Sync>))), pointerValue.clone()).lock().unwrap().as_ref().unwrap())));
    println!("{} {}", format!("{}", "pointer write".to_string()), format!("{}", (*{ let __field = (*pointerValue.lock().unwrap().as_ref().unwrap()).value.clone(); __field }.lock().unwrap().as_ref().unwrap())));

    println!("{}", format!("{}", (*recover_value(Arc::new(Mutex::new(Some(Box::new(9i64) as Box<dyn Any + Send + Sync>))), Arc::new(Mutex::new(None))).lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", (*recover_value(Arc::new(Mutex::new(Some(Box::new("text".to_string()) as Box<dyn Any + Send + Sync>))), Arc::new(Mutex::new(None))).lock().unwrap().as_ref().unwrap())));
    println!("{} {}", format!("{}", "literal".to_string()), format!("{}", (*explicit_runtime_text().lock().unwrap().as_ref().unwrap())));
    println!("{} {}", format!("{}", "top".to_string()), format!("{}", (*nested_panic().lock().unwrap().as_ref().unwrap())));
    replacement_panic();
    println!("{} {}", format!("{}", "probe".to_string()), format!("{}", (*probe().lock().unwrap().as_ref().unwrap())));
    let (mut outer, mut probed) = normal_probe_cannot_recover_outer();
    println!("{} {} {}", format!("{}", "scoped".to_string()), format!("{}", { let __v = (*outer.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*probed.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    println!("{}", format!("{}", (*only_direct_deferred_call_can_recover().lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", (*formatter_runs_only_for_uncaught_panic().lock().unwrap().as_ref().unwrap())));
    println!("{} {}", format!("{}", "dynamic".to_string()), format!("{}", (*recover_nil_value(Arc::new(Mutex::new(Some(false)))).lock().unwrap().as_ref().unwrap())));
    println!("{} {}", format!("{}", "literal".to_string()), format!("{}", (*recover_nil_value(Arc::new(Mutex::new(Some(true)))).lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", (*explicit_string_is_not_error().lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", (*recover_named_string().lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", (*recover_shadowed_string(Arc::new(Mutex::new(Some("shadowed".to_string())))).lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", (*recover_shadowed_int(Arc::new(Mutex::new(Some(17)))).lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", (*recover_slice_element().lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", (*recover_and_read_interface(Arc::new(Mutex::new(Some(Box::new("same".to_string()) as Box<dyn Any + Send + Sync>)))).lock().unwrap().as_ref().unwrap())));
    recover_in_goroutine_literals();
}