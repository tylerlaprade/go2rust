#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use std::cell::{RefCell};
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

#[derive(Debug, Clone)]
pub struct Point {
    pub x: Rc<RefCell<Option<i64>>>,
    pub y: Rc<RefCell<Option<i64>>>,
}

impl Point {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.y.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
            y: __go_clone_0_1,
        }
    }
}


impl Default for Point {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_0_1 = Rc::new(RefCell::new(Some(0)));
        Self {
            x: __go_default_0_0,
            y: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.y.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct Line {
    pub a: Rc<RefCell<Option<Point>>>,
    pub b: Rc<RefCell<Option<Point>>>,
}

impl Line {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.a.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        let __go_clone_0_1 = { let __guard = self.b.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        Self {
            a: __go_clone_0_0,
            b: __go_clone_0_1,
        }
    }
}


impl Default for Line {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(Point::default())));
        let __go_default_0_1 = Rc::new(RefCell::new(Some(Point::default())));
        Self {
            a: __go_default_0_0,
            b: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.a.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.b.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct Grid {
    pub cells: Rc<RefCell<Option<[Point; 2]>>>,
}

impl Grid {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.cells.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.each_ref().map(|__go_elem| __go_elem.__go_value_clone())))) };
        Self {
            cells: __go_clone_0_0,
        }
    }
}


impl Default for Grid {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(std::array::from_fn::<_, 2, _>(|_| Default::default()))));
        Self {
            cells: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_slice(&self.cells));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone, Default)]
pub struct Node {
    pub p: Rc<RefCell<Option<Point>>>,
}

impl Node {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.p.clone();
        Self {
            p: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.p.borrow(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone)]
pub struct Config {
    pub name: Rc<RefCell<Option<String>>>,
    pub port: Rc<RefCell<Option<i64>>>,
}

impl Config {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.port.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
            port: __go_clone_1_0,
        }
    }
}


impl Default for Config {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            name: __go_default_0_0,
            port: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.port.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct Location {
    pub p: Rc<RefCell<Option<Point>>>,
}

impl Location {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.p.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        Self {
            p: __go_clone_0_0,
        }
    }
}


impl Default for Location {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(Point::default())));
        Self {
            p: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.p.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone)]
pub struct User {
    pub l: Rc<RefCell<Option<Location>>>,
}

impl User {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.l.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        Self {
            l: __go_clone_0_0,
        }
    }
}


impl Default for User {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(Location::default())));
        Self {
            l: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.l.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Config {
    /// A value receiver works on a copy of the struct.
    pub fn with_name(&self, name: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<Config>>> {
        let mut __self = self.__go_value_clone();
        { let new_val = name.borrow().as_ref().unwrap().clone(); *__self.name.borrow_mut() = Some(new_val); };
        Rc::new(RefCell::new(Some(__self.clone())))
    }

    pub fn bump(&self) {
        let mut __self = self.__go_value_clone();
        { let __target = __self.port.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

impl User {
    pub fn location(&self) -> Rc<RefCell<Option<Location>>> {
        return Rc::new(RefCell::new(Some({ let __selector_holder = self.l.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = __selector_guard.as_ref().unwrap().__go_value_clone(); drop(__selector_guard); __cloned })));
    }
}

pub fn r#move(p: Rc<RefCell<Option<Point>>>) -> Rc<RefCell<Option<Point>>> {
    { let __target = (*p.borrow().as_ref().unwrap()).x.clone(); let __rhs = 10; let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    { let __owned = p.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) }
}

pub fn stretch(l: Rc<RefCell<Option<Line>>>) {
    { let new_val = 100; *(*(*l.borrow().as_ref().unwrap()).b.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
}

/// A struct is a value. Each of these makes a copy, and changing the copy
/// leaves the original alone.
fn main() {
    let mut a = Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(1))), y: Rc::new(RefCell::new(Some(2))), ..Default::default() })));
    let mut b = { let __owned = a.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) };
    { let new_val = 9; *(*b.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "assign".to_string()), format!("{}", (*a.borrow().as_ref().unwrap())), format!("{}", (*b.borrow().as_ref().unwrap())));

    let mut c: Rc<RefCell<Option<Point>>> = Rc::new(RefCell::new(Some(Default::default())));
    { let new_val = a.borrow().as_ref().unwrap().__go_value_clone(); *c.borrow_mut() = Some(new_val); };
    { let new_val = 7; *(*c.borrow().as_ref().unwrap()).y.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "assign to declared".to_string()), format!("{}", (*a.borrow().as_ref().unwrap())), format!("{}", (*c.borrow().as_ref().unwrap())));

    let mut d = r#move(Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))));
    println!("{} {} {}", format!("{}", "parameter and result".to_string()), format!("{}", (*a.borrow().as_ref().unwrap())), format!("{}", (*d.borrow().as_ref().unwrap())));

    let mut l1 = Rc::new(RefCell::new(Some(Line { a: Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(1))), y: Rc::new(RefCell::new(Some(1))), ..Default::default() }))), b: Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(2))), y: Rc::new(RefCell::new(Some(2))), ..Default::default() }))), ..Default::default() })));
    let mut l2 = { let __owned = l1.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) };
    { let new_val = 9; *(*(*l2.borrow().as_ref().unwrap()).a.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "struct in struct".to_string()), format!("{}", (*l1.borrow().as_ref().unwrap())), format!("{}", (*l2.borrow().as_ref().unwrap())));

    stretch(Rc::new(RefCell::new(Some({ let __arg_holder = l1.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))));
    println!("{} {}", format!("{}", "struct in struct, parameter".to_string()), format!("{}", (*l1.borrow().as_ref().unwrap())));

    let mut points = Rc::new(RefCell::new(Some(vec![Point { x: Rc::new(RefCell::new(Some(1))), y: Rc::new(RefCell::new(Some(1))), ..Default::default() }, Point { x: Rc::new(RefCell::new(Some(2))), y: Rc::new(RefCell::new(Some(2))), ..Default::default() }])));
    let mut e = Rc::new(RefCell::new(Some((*points.borrow().as_ref().unwrap())[(0) as usize].clone().__go_value_clone())));
    { let new_val = 50; *(*e.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "slice element".to_string()), format!("{}", format_slice(&points)), format!("{}", (*e.borrow().as_ref().unwrap())));

    { let __range_holder = points.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for p in __range_values.iter() {
        let p = &p.__go_value_clone();
        { let new_val = 60; *p.x.borrow_mut() = Some(new_val); };
    } }
    println!("{} {}", format!("{}", "range value".to_string()), format!("{}", format_slice(&points)));

    let mut arr = Rc::new(RefCell::new(Some([Point { x: Rc::new(RefCell::new(Some(1))), y: Rc::new(RefCell::new(Some(1))), ..Default::default() }, Point { x: Rc::new(RefCell::new(Some(2))), y: Rc::new(RefCell::new(Some(2))), ..Default::default() }])));
    let mut arr2 = { let __owned = arr.borrow().as_ref().unwrap().each_ref().map(|__go_elem| __go_elem.__go_value_clone()); Rc::new(RefCell::new(Some(__owned))) };
    { let new_val = 7; *(*arr2.borrow().as_ref().unwrap())[(0) as usize].clone().x.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "array".to_string()), format!("{}", format_slice(&arr)), format!("{}", format_slice(&arr2)));

    let mut g1 = Rc::new(RefCell::new(Some(Grid { cells: Rc::new(RefCell::new(Some(std::array::from_fn::<_, 2, _>(|_| Default::default())))) })));
    let mut g2 = { let __owned = g1.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) };
    { let new_val = 5; *(*(*g2.borrow().as_ref().unwrap()).cells.borrow().as_ref().unwrap())[(0) as usize].clone().x.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "array in struct".to_string()), format!("{}", (*g1.borrow().as_ref().unwrap())), format!("{}", (*g2.borrow().as_ref().unwrap())));

    let mut base = Rc::new(RefCell::new(Some(Config { name: Rc::new(RefCell::new(Some("base".to_string()))), port: Rc::new(RefCell::new(Some(80))), ..Default::default() })));
    let mut dev = (*base.borrow().as_ref().unwrap()).with_name(Rc::new(RefCell::new(Some("dev".to_string()))));
    println!("{} {} {}", format!("{}", "value receiver".to_string()), format!("{}", (*base.borrow().as_ref().unwrap())), format!("{}", (*dev.borrow().as_ref().unwrap())));
    (*base.borrow().as_ref().unwrap()).bump();
    println!("{} {}", format!("{}", "value receiver, no result".to_string()), format!("{}", (*base.borrow().as_ref().unwrap())));

    let mut u = Rc::new(RefCell::new(Some(User { l: Rc::new(RefCell::new(Some(Location { p: Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(1))), ..Default::default() }))), ..Default::default() }))), ..Default::default() })));
    let mut v = (*u.borrow().as_ref().unwrap()).location();
    { let new_val = 9; *(*(*v.borrow().as_ref().unwrap()).p.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "getter".to_string()), format!("{}", (*(*u.borrow().as_ref().unwrap()).l.borrow().as_ref().unwrap()).clone()), format!("{}", (*v.borrow().as_ref().unwrap())));

        // A pointer inside the struct is copied as a pointer: both structs
        // point at the same Point until one of them gets a new pointer.
    let mut n1 = Rc::new(RefCell::new(Some(Node { p: Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(1))), y: Rc::new(RefCell::new(Some(1))), ..Default::default() }))).clone(), ..Default::default() })));
    let mut n2 = { let __owned = n1.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) };
    { let new_val = 3; *(*(*n2.borrow().as_ref().unwrap()).p.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "pointer field shared".to_string()), format!("{}", { let __v = (*(*n1.borrow().as_ref().unwrap()).p.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*(*n2.borrow().as_ref().unwrap()).p.borrow().as_ref().unwrap()).clone(); __v }));
    { let new_val = Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(8))), y: Rc::new(RefCell::new(Some(8))), ..Default::default() }))).clone(); (*n2.borrow_mut().as_mut().unwrap()).p = new_val; };
    println!("{} {} {}", format!("{}", "pointer field replaced".to_string()), format!("{}", { let __v = (*(*n1.borrow().as_ref().unwrap()).p.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*(*n2.borrow().as_ref().unwrap()).p.borrow().as_ref().unwrap()).clone(); __v }));

        // A range over a slice reads each element when its turn comes, so it
        // sees a field that the loop itself wrote. The loop variable is still a
        // copy: writing to it does not change the slice.
    let mut seen = Rc::new(RefCell::new(Some(vec![Point { x: Rc::new(RefCell::new(Some(1))), y: Rc::new(RefCell::new(Some(1))), ..Default::default() }, Point { x: Rc::new(RefCell::new(Some(2))), y: Rc::new(RefCell::new(Some(2))), ..Default::default() }])));
    { let __range_holder = seen.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for p in __range_values.iter() {
        let p = &p.__go_value_clone();
        { let new_val = 9; *(*seen.borrow().as_ref().unwrap())[(1) as usize].clone().x.borrow_mut() = Some(new_val); };
        println!("{} {}", format!("{}", "range over a slice, field written in the loop".to_string()), format!("{}", (*p.x.borrow().as_ref().unwrap())));
        { let new_val = 99; *p.x.borrow_mut() = Some(new_val); };
    } }
    println!("{} {}", format!("{}", "range over a slice, the slice afterwards".to_string()), format!("{}", format_slice(&seen)));

        // A range over an array works on a copy of the array.
    let mut fixed = Rc::new(RefCell::new(Some([Point { x: Rc::new(RefCell::new(Some(1))), y: Rc::new(RefCell::new(Some(1))), ..Default::default() }, Point { x: Rc::new(RefCell::new(Some(2))), y: Rc::new(RefCell::new(Some(2))), ..Default::default() }])));
    { let __range_holder = fixed.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.iter().map(|__e| __e.__go_value_clone()).collect::<Vec<_>>()).unwrap_or_default(); drop(__range_guard); for p in __range_values.iter() {
        let p = &p.__go_value_clone();
        { let new_val = 9; *(*fixed.borrow().as_ref().unwrap())[(1) as usize].clone().x.borrow_mut() = Some(new_val); };
        println!("{} {}", format!("{}", "range over an array, field written in the loop".to_string()), format!("{}", (*p.x.borrow().as_ref().unwrap())));
        { let new_val = 99; *p.x.borrow_mut() = Some(new_val); };
    } }
    println!("{} {}", format!("{}", "range over an array, the array afterwards".to_string()), format!("{}", format_slice(&fixed)));
}