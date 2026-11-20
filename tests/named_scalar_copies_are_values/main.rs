#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
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
        let formatted: Vec<String> = s.as_ref().iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn format_slice_values<T>(slice: &[T]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| v.to_string()).collect();
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

#[derive(Debug, Default)]
pub struct Counter(pub Rc<RefCell<Option<i64>>>);

impl Clone for Counter {
    fn clone(&self) -> Self {
        let __value = (*self.0.borrow()).clone();
        Counter(Rc::new(RefCell::new(__value)))
    }
}

impl Display for Counter {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for Counter {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i64> for Counter {
    fn eq(&self, other: &i64) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Counter {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i64> for Counter {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Counter> for i64 {
    fn eq(&self, other: &Counter) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<Counter> for i64 {
    fn partial_cmp(&self, other: &Counter) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for Counter {
    type Output = Counter;
    fn add(self, other: Self) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i64> for Counter {
    type Output = Counter;
    fn add(self, other: i64) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Counter> for i64 {
    type Output = Counter;
    fn add(self, other: Counter) -> Counter {
        Counter(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Counter {
    type Output = Counter;
    fn sub(self, other: Self) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i64> for Counter {
    type Output = Counter;
    fn sub(self, other: i64) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Counter> for i64 {
    type Output = Counter;
    fn sub(self, other: Counter) -> Counter {
        Counter(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Counter {
    type Output = Counter;
    fn mul(self, other: Self) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i64> for Counter {
    type Output = Counter;
    fn mul(self, other: i64) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Counter> for i64 {
    type Output = Counter;
    fn mul(self, other: Counter) -> Counter {
        Counter(Rc::new(RefCell::new(Some(self * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Counter {
    type Output = Counter;
    fn div(self, other: Self) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i64> for Counter {
    type Output = Counter;
    fn div(self, other: i64) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Counter> for i64 {
    type Output = Counter;
    fn div(self, other: Counter) -> Counter {
        Counter(Rc::new(RefCell::new(Some(self / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for Counter {
    type Output = Counter;
    fn neg(self) -> Counter {
        Counter(Rc::new(RefCell::new(Some(-*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for Counter {
    type Output = Counter;
    fn rem(self, other: Self) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i64> for Counter {
    type Output = Counter;
    fn rem(self, other: i64) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<Counter> for i64 {
    type Output = Counter;
    fn rem(self, other: Counter) -> Counter {
        Counter(Rc::new(RefCell::new(Some(self % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Counter {
    type Output = Counter;
    fn bitand(self, other: Self) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i64> for Counter {
    type Output = Counter;
    fn bitand(self, other: i64) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Counter> for i64 {
    type Output = Counter;
    fn bitand(self, other: Counter) -> Counter {
        Counter(Rc::new(RefCell::new(Some(self & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Counter {
    type Output = Counter;
    fn bitor(self, other: Self) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i64> for Counter {
    type Output = Counter;
    fn bitor(self, other: i64) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Counter> for i64 {
    type Output = Counter;
    fn bitor(self, other: Counter) -> Counter {
        Counter(Rc::new(RefCell::new(Some(self | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Counter {
    type Output = Counter;
    fn bitxor(self, other: Self) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i64> for Counter {
    type Output = Counter;
    fn bitxor(self, other: i64) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Counter> for i64 {
    type Output = Counter;
    fn bitxor(self, other: Counter) -> Counter {
        Counter(Rc::new(RefCell::new(Some(self ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Counter {
    type Output = Counter;
    fn not(self) -> Counter {
        Counter(Rc::new(RefCell::new(Some(!*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Counter {
    type Output = Counter;
    fn shl(self, other: Counter) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for Counter {
    type Output = Counter;
    fn shl(self, other: i32) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for Counter {
    type Output = Counter;
    fn shl(self, other: i8) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for Counter {
    type Output = Counter;
    fn shl(self, other: i16) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for Counter {
    type Output = Counter;
    fn shl(self, other: i64) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for Counter {
    type Output = Counter;
    fn shl(self, other: u32) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for Counter {
    type Output = Counter;
    fn shl(self, other: u8) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for Counter {
    type Output = Counter;
    fn shl(self, other: u16) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for Counter {
    type Output = Counter;
    fn shl(self, other: u64) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for Counter {
    type Output = Counter;
    fn shl(self, other: usize) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for Counter {
    type Output = Counter;
    fn shr(self, other: Counter) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for Counter {
    type Output = Counter;
    fn shr(self, other: i32) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for Counter {
    type Output = Counter;
    fn shr(self, other: i8) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for Counter {
    type Output = Counter;
    fn shr(self, other: i16) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for Counter {
    type Output = Counter;
    fn shr(self, other: i64) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for Counter {
    type Output = Counter;
    fn shr(self, other: u32) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for Counter {
    type Output = Counter;
    fn shr(self, other: u8) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for Counter {
    type Output = Counter;
    fn shr(self, other: u16) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for Counter {
    type Output = Counter;
    fn shr(self, other: u64) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for Counter {
    type Output = Counter;
    fn shr(self, other: usize) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl Eq for Counter {}

impl Ord for Counter {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug, Default)]
pub struct Celsius(pub Rc<RefCell<Option<f64>>>);

impl Clone for Celsius {
    fn clone(&self) -> Self {
        let __value = (*self.0.borrow()).clone();
        Celsius(Rc::new(RefCell::new(__value)))
    }
}

impl Display for Celsius {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
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


#[derive(Debug, Default)]
pub struct Name(pub Rc<RefCell<Option<String>>>);

impl Clone for Name {
    fn clone(&self) -> Self {
        let __value = (*self.0.borrow()).clone();
        Name(Rc::new(RefCell::new(__value)))
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left == __right
    }
}

impl PartialOrd for Name {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ord::cmp(self, other))
    }
}

impl Eq for Name {}

impl Ord for Name {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}

impl PartialEq<String> for Name {
    fn eq(&self, other: &String) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialEq<&str> for Name {
    fn eq(&self, other: &&str) -> bool {
        self.0.borrow().as_ref().unwrap().as_str() == *other
    }
}

impl std::ops::Add for Name {
    type Output = Name;
    fn add(self, other: Self) -> Name {
        let __joined = format!("{}{}", self.0.borrow().as_ref().unwrap(), other.0.borrow().as_ref().unwrap());
        Name(Rc::new(RefCell::new(Some(__joined))))
    }
}

impl std::ops::Add<String> for Name {
    type Output = Name;
    fn add(self, other: String) -> Name {
        let __joined = format!("{}{}", self.0.borrow().as_ref().unwrap(), other);
        Name(Rc::new(RefCell::new(Some(__joined))))
    }
}

impl std::ops::Add<&str> for Name {
    type Output = Name;
    fn add(self, other: &str) -> Name {
        let __joined = format!("{}{}", self.0.borrow().as_ref().unwrap(), other);
        Name(Rc::new(RefCell::new(Some(__joined))))
    }
}


#[derive(Debug, Default)]
pub struct Flag(pub Rc<RefCell<Option<bool>>>);

impl Clone for Flag {
    fn clone(&self) -> Self {
        let __value = (*self.0.borrow()).clone();
        Flag(Rc::new(RefCell::new(__value)))
    }
}

impl Display for Flag {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for Flag {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left == __right
    }
}


#[derive(Debug, Clone)]
pub struct Stats {
    pub hits: Rc<RefCell<Option<Counter>>>,
    pub temp: Rc<RefCell<Option<Celsius>>>,
}

impl Stats {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.hits.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.temp.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            hits: __go_clone_0_0,
            temp: __go_clone_1_0,
        }
    }
}


impl Default for Stats {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(Counter(Rc::new(RefCell::new(Some(0)))))));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(Celsius(Rc::new(RefCell::new(Some(0.0)))))));
        Self {
            hits: __go_default_0_0,
            temp: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.hits.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.temp.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl Counter {
    pub fn inc(&mut self) {
        { let mut guard = self.0.borrow_mut(); *guard = Some(guard.as_ref().unwrap().clone() + 1 as i64); }
    }
}

impl Celsius {
    pub fn warm(&mut self) {
        { let __rhs = 1.5; let mut guard = self.0.borrow_mut(); *guard = Some(guard.as_ref().unwrap().clone() + __rhs); };
    }
}

impl Name {
    pub fn shout(&mut self) {
        { (*self.0.borrow_mut().as_mut().unwrap()).push_str(&"!".to_string()); };
    }
}

impl Flag {
    pub fn toggle(&mut self) {
        { let new_val = Flag(Rc::new(RefCell::new(Some(!((*self.0.borrow().as_ref().unwrap())))))); *self = new_val; };
    }
}

pub fn bump(c: Rc<RefCell<Option<Counter>>>) -> Rc<RefCell<Option<Counter>>> {
    (*c.borrow_mut().as_mut().unwrap()).inc();
    { let __owned = c.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) }
}

fn main() {
    let mut a = Rc::new(RefCell::new(Some(Counter(Rc::new(RefCell::new(Some(1 as i64)))))));
    let mut b = { let __owned = a.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    (*b.borrow_mut().as_mut().unwrap()).inc();
    println!("{} {} {} {}", format!("{}", { let __v = (*a.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*b.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", (*bump(Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })))).borrow().as_ref().unwrap())), format!("{}", { let __v = (*a.borrow().as_ref().unwrap()).clone(); __v }));

    let mut x = Rc::new(RefCell::new(Some(Celsius(Rc::new(RefCell::new(Some(1.0 as f64)))))));
    let mut y = { let __owned = x.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    (*y.borrow_mut().as_mut().unwrap()).warm();
    println!("{} {}", format!("{}", { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*y.borrow().as_ref().unwrap()).clone(); __v }));

    let mut n = Rc::new(RefCell::new(Some(Name(Rc::new(RefCell::new(Some("go".to_string())))))));
    let mut m = { let __owned = n.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    (*m.borrow_mut().as_mut().unwrap()).shout();
    println!("{} {}", format!("{}", { let __v = (*n.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*m.borrow().as_ref().unwrap()).clone(); __v }));

    let mut f = Rc::new(RefCell::new(Some(Flag(Rc::new(RefCell::new(Some(false)))))));
    let mut g = { let __owned = f.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    (*g.borrow_mut().as_mut().unwrap()).toggle();
    println!("{} {}", format!("{}", { let __v = (*f.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*g.borrow().as_ref().unwrap()).clone(); __v }));

    let mut s = Rc::new(RefCell::new(Some(Stats { hits: Rc::new(RefCell::new(Some(Counter(Rc::new(RefCell::new(Some(5 as i64))))))), temp: Rc::new(RefCell::new(Some(Celsius(Rc::new(RefCell::new(Some(20.0))))))), ..Default::default() })));
    let mut t = { let __owned = s.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) };
    (*(*t.borrow().as_ref().unwrap()).hits.borrow_mut().as_mut().unwrap()).inc();
    (*(*t.borrow().as_ref().unwrap()).temp.borrow_mut().as_mut().unwrap()).warm();
    println!("{} {}", format!("{}", (*s.borrow().as_ref().unwrap())), format!("{}", (*t.borrow().as_ref().unwrap())));

    let mut counters = Rc::new(RefCell::new(Some(vec![Counter(Rc::new(RefCell::new(Some(1 as i64)))), Counter(Rc::new(RefCell::new(Some(2 as i64)))), Counter(Rc::new(RefCell::new(Some(3 as i64))))])));
    let mut first = Rc::new(RefCell::new(Some(Counter(Rc::new(RefCell::new(Some((*(*counters.borrow().as_ref().unwrap())[(0) as usize].clone().0.borrow().as_ref().unwrap()))))))));
    (*first.borrow_mut().as_mut().unwrap()).inc();
    { let __range_holder = counters.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for mut c in __range_values.iter().cloned() {
        c.inc();
    } }
    println!("{} {}", format!("{}", format_slice(&counters)), format!("{}", { let __v = (*first.borrow().as_ref().unwrap()).clone(); __v }));

    let mut p = a.clone();
    (*p.borrow_mut().as_mut().unwrap()).inc();
    println!("{} {}", format!("{}", { let __v = (*a.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*p.borrow().as_ref().unwrap()).clone(); __v }));
    let mut byName = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Counter>>>>::from([("k".to_string(), Rc::new(RefCell::new(Some(Counter(Rc::new(RefCell::new(Some(10 as i64))))))))]))));
    let mut v = Rc::new(RefCell::new(Some(Counter(Rc::new(RefCell::new(Some((*{ let __map_holder = byName.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"k".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| Counter(Rc::new(RefCell::new(Some(0))))) }.0.borrow().as_ref().unwrap()))))))));
    (*v.borrow_mut().as_mut().unwrap()).inc();
    println!("{} {}", format!("{}", { let __map_holder = byName.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"k".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| Counter(Rc::new(RefCell::new(Some(0))))) }), format!("{}", { let __v = (*v.borrow().as_ref().unwrap()).clone(); __v }));
}