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
pub struct Pair__string__int {
    pub key: Rc<RefCell<Option<String>>>,
    pub val: Rc<RefCell<Option<i64>>>,
}

impl Pair__string__int {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.key.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.val.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            key: __go_clone_0_0,
            val: __go_clone_1_0,
        }
    }
}


impl Default for Pair__string__int {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            key: __go_default_0_0,
            val: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Pair__string__int {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct Set__string(pub Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<AnonymousStruct1>>>>>>>);


#[derive(Debug, Clone, Default)]
pub struct AnonymousStruct1 {
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
    }
}


impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


impl Pair__string__int {
    pub fn string(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("{}={}", (*self.key.borrow().as_ref().unwrap()), (*self.val.borrow().as_ref().unwrap())))))
    }
}

impl Set__string {
    pub fn add(&self, v: Rc<RefCell<Option<String>>>) {
        let mut __self = self.clone();
        { let __map_key = (*v.borrow().as_ref().unwrap()).clone(); let __map_value = Rc::new(RefCell::new(Some(AnonymousStruct1 {  }))); (*__self.0.clone().borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    }

    pub fn has(&self, v: Rc<RefCell<Option<String>>>) -> bool {
        let (_, mut ok) = { let __map_holder = self.0.clone().clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&(*v.borrow().as_ref().unwrap()).clone())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Rc::new(RefCell::new(Some(Default::default()))), false) } };
        ok
    }

    pub fn len(&self) -> i64 {
        return { let __map_holder = self.0.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64;
    }
}

pub fn keys__string__int(m: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>) -> Rc<RefCell<Option<Vec<String>>>> {
    let mut keys = Rc::new(RefCell::new(Some({ let mut v: Vec<String> = (0..((*m.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize).map(|_| "".to_string()).collect(); v.truncate((0) as usize); v })));
    for (k, _) in { let __range_holder = m.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = keys.clone(); let __append_value = k.clone(); let mut __appended: Vec<String> = (*__append_target.borrow()).clone().unwrap_or_default(); __appended.push(__append_value); Rc::new(RefCell::new(Some(__appended))) })); *keys.borrow_mut() = __slice_header; };
    }
    let keys_closure_clone = keys.clone(); { let __sort_target = keys_closure_clone.clone(); let __sort_less = Rc::new(RefCell::new(Some(Box::new(move |i: Rc<RefCell<Option<i64>>>, j: Rc<RefCell<Option<i64>>>| -> bool {
        return (*keys_closure_clone.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone() < (*keys_closure_clone.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
    }) as Box<dyn FnMut(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> bool>))); let __sort_len = { let __sort_guard = __sort_target.borrow(); __sort_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; for __sort_i in 1..__sort_len { let mut __sort_j = __sort_i; while __sort_j > 0 { let __should_swap = { let mut __less_guard = __sort_less.borrow_mut(); let __less = __less_guard.as_mut().expect("sort.Slice less function is nil"); __less(Rc::new(RefCell::new(Some(__sort_j as i64))), Rc::new(RefCell::new(Some((__sort_j - 1) as i64)))) }; if !__should_swap { break; } { let mut __sort_guard = __sort_target.borrow_mut(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.swap(__sort_j, __sort_j - 1); } } __sort_j -= 1; } } };
    return keys.clone();
}

pub fn keys__int__bool(m: Rc<RefCell<Option<BTreeMap<i64, Rc<RefCell<Option<bool>>>>>>>) -> Rc<RefCell<Option<Vec<i64>>>> {
    let mut keys = Rc::new(RefCell::new(Some({ let mut v: Vec<i64> = (0..((*m.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize).map(|_| 0).collect(); v.truncate((0) as usize); v })));
    for (k, _) in { let __range_holder = m.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = keys.clone(); let __append_value = k; let mut __appended: Vec<i64> = (*__append_target.borrow()).clone().unwrap_or_default(); __appended.push(__append_value); Rc::new(RefCell::new(Some(__appended))) })); *keys.borrow_mut() = __slice_header; };
    }
    let keys_closure_clone = keys.clone(); { let __sort_target = keys_closure_clone.clone(); let __sort_less = Rc::new(RefCell::new(Some(Box::new(move |i: Rc<RefCell<Option<i64>>>, j: Rc<RefCell<Option<i64>>>| -> bool {
        return (*keys_closure_clone.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone() < (*keys_closure_clone.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
    }) as Box<dyn FnMut(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> bool>))); let __sort_len = { let __sort_guard = __sort_target.borrow(); __sort_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; for __sort_i in 1..__sort_len { let mut __sort_j = __sort_i; while __sort_j > 0 { let __should_swap = { let mut __less_guard = __sort_less.borrow_mut(); let __less = __less_guard.as_mut().expect("sort.Slice less function is nil"); __less(Rc::new(RefCell::new(Some(__sort_j as i64))), Rc::new(RefCell::new(Some((__sort_j - 1) as i64)))) }; if !__should_swap { break; } { let mut __sort_guard = __sort_target.borrow_mut(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.swap(__sort_j, __sort_j - 1); } } __sort_j -= 1; } } };
    return keys.clone();
}

pub fn max__int(a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) -> i64 {
    if (*a.borrow().as_ref().unwrap()) > (*b.borrow().as_ref().unwrap()) {
        return (*a.borrow().as_ref().unwrap());
    }
    (*b.borrow().as_ref().unwrap())
}

pub fn max__float64(a: Rc<RefCell<Option<f64>>>, b: Rc<RefCell<Option<f64>>>) -> f64 {
    if (*a.borrow().as_ref().unwrap()) > (*b.borrow().as_ref().unwrap()) {
        return (*a.borrow().as_ref().unwrap());
    }
    (*b.borrow().as_ref().unwrap())
}

pub fn max__string(a: Rc<RefCell<Option<String>>>, b: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {
    if (*a.borrow().as_ref().unwrap()) > (*b.borrow().as_ref().unwrap()) {
        return { let __owned = a.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    }
    { let __owned = b.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) }
}

pub fn filter__int(xs: Rc<RefCell<Option<Vec<i64>>>>, keep: Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> bool>>>>) -> Rc<RefCell<Option<Vec<i64>>>> {
    let mut out: Rc<RefCell<Option<Vec<i64>>>> = Rc::new(RefCell::new(None));

    { let __range_holder = xs.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter().copied() {
        if { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> bool> = { let mut __f_guard = keep.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> bool> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(x)))) } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = out.clone(); let __append_value = x; let mut __appended: Vec<i64> = (*__append_target.borrow()).clone().unwrap_or_default(); __appended.push(__append_value); Rc::new(RefCell::new(Some(__appended))) })); *out.borrow_mut() = __slice_header; };
    }
    } }
    return out.clone();
}

pub fn reduce__int__int(xs: Rc<RefCell<Option<Vec<i64>>>>, init: Rc<RefCell<Option<i64>>>, f: Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>>>>) -> i64 {
    let mut acc = { let __owned = init.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    { let __range_holder = xs.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter().copied() {
        { let new_val = { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> = { let mut __f_guard = f.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(acc.clone(), Rc::new(RefCell::new(Some(x)))) }; *acc.borrow_mut() = Some(new_val); };
    } }
    return (*acc.borrow().as_ref().unwrap());
}

pub fn reduce__string__string(xs: Rc<RefCell<Option<Vec<String>>>>, init: Rc<RefCell<Option<String>>>, f: Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>>>>>) -> Rc<RefCell<Option<String>>> {
    let mut acc = { let __owned = init.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    { let __range_holder = xs.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter() {
        { let new_val = { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>> = { let mut __f_guard = f.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some({ let __arg_holder = acc.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some((*x).clone())))) }; let __moved_val = { let mut __guard = new_val.borrow_mut(); __guard.take() }; *acc.borrow_mut() = __moved_val; };
    } }
    return { let __owned = acc.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
}

pub fn map__int__string(xs: Rc<RefCell<Option<Vec<i64>>>>, f: Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> Rc<RefCell<Option<String>>>>>>>) -> Rc<RefCell<Option<Vec<String>>>> {
    let mut out = Rc::new(RefCell::new(Some(vec!["".to_string(); ((*xs.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize])));
    { let __range_holder = xs.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, x) in __range_values.iter().copied().enumerate() {
        { let __idx = (i) as usize; let __rhs = (*{ let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> Rc<RefCell<Option<String>>>> = { let mut __f_guard = f.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> Rc<RefCell<Option<String>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(x)))) }.borrow().as_ref().unwrap()).clone(); (*out.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    } }
    return out.clone();
}

fn main() {
    let mut ages = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i64>>>>::from([("bob".to_string(), Rc::new(RefCell::new(Some(30i64)))), ("alice".to_string(), Rc::new(RefCell::new(Some(25i64)))), ("carol".to_string(), Rc::new(RefCell::new(Some(35i64))))]))));
    let mut pairs: Rc<RefCell<Option<Vec<Pair__string__int>>>> = Rc::new(RefCell::new(None));
    { let __range_holder = keys__string__int(ages.clone()).clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for k in __range_values.iter() {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = pairs.clone(); let __append_value = Pair__string__int { key: Rc::new(RefCell::new(Some((*k).clone()))), val: Rc::new(RefCell::new(Some({ let __map_holder = ages.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(k)).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }))), ..Default::default() }; let mut __appended: Vec<Pair__string__int> = (*__append_target.borrow()).clone().unwrap_or_default(); __appended.push(__append_value); Rc::new(RefCell::new(Some(__appended))) })); *pairs.borrow_mut() = __slice_header; };
    } }
    println!("{}", format!("{}", format_slice(&pairs)));
    println!("{} {}", format!("{}", (*pairs.borrow().as_ref().unwrap())[(0) as usize].clone()), format!("{}", (*pairs.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)));
    let mut s = Rc::new(RefCell::new(Some(Set__string(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<AnonymousStruct1>>>>::from([]))))))));
    { let __range_holder = Rc::new(RefCell::new(Some({ let __s = "a b a c b".to_string(); __s.split_whitespace().map(|__part| __part.to_string()).collect::<Vec<String>>() }))).clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for w in __range_values.iter() {
        (*s.borrow().as_ref().unwrap()).add(Rc::new(RefCell::new(Some((*w).clone()))));
    } }
    println!("{} {} {}", format!("{}", (*s.borrow().as_ref().unwrap()).len()), format!("{}", (*s.borrow().as_ref().unwrap()).has(Rc::new(RefCell::new(Some("a".to_string()))))), format!("{}", (*s.borrow().as_ref().unwrap()).has(Rc::new(RefCell::new(Some("z".to_string()))))));
    println!("{} {} {}", format!("{}", max__int(Rc::new(RefCell::new(Some(3))), Rc::new(RefCell::new(Some(7))))), { let __text = go_float_string((max__float64(Rc::new(RefCell::new(Some(2.5))), Rc::new(RefCell::new(Some(-1.0))))) as f64, false); __text }, format!("{}", (*max__string(Rc::new(RefCell::new(Some("go".to_string()))), Rc::new(RefCell::new(Some("rust".to_string())))).borrow().as_ref().unwrap())));
    let mut evens = filter__int(Rc::new(RefCell::new(Some(vec![1i64, 2i64, 3i64, 4i64, 5i64, 6i64]))), Rc::new(RefCell::new(Some(Box::new(move |n: Rc<RefCell<Option<i64>>>| -> bool {
        (*n.borrow().as_ref().unwrap()) % 2 == 0
    }) as Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> bool>))));
    let mut evens = { let __slice_header = GoSliceHeader::go_slice_header(&evens); Rc::new(RefCell::new(__slice_header)) };
    println!("{} {}", format!("{}", format_slice(&evens)), format!("{}", reduce__int__int(evens.clone(), Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(Box::new(move |a: Rc<RefCell<Option<i64>>>, n: Rc<RefCell<Option<i64>>>| -> i64 {
        (*a.borrow().as_ref().unwrap()) + (*n.borrow().as_ref().unwrap())
    }) as Box<dyn FnMut(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>))))));
    let mut words = map__int__string(Rc::new(RefCell::new(Some(vec![1i64, 22i64, 333i64]))), Rc::new(RefCell::new(Some(Box::new(move |n: Rc<RefCell<Option<i64>>>| -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some({ let __s = "x".to_string(); let __count = (*Rc::new(RefCell::new(Some(format!("{}", format!("{}", { let __v = (*n.borrow().as_ref().unwrap()).clone(); __v }))))).borrow().as_ref().unwrap()).len(); __s.repeat(__count as usize) })))
    }) as Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> Rc<RefCell<Option<String>>>>))));
    let mut words = { let __slice_header = GoSliceHeader::go_slice_header(&words); Rc::new(RefCell::new(__slice_header)) };
    println!("{} {}", format!("{}", format_slice(&words)), format!("{}", (*reduce__string__string(words.clone(), Rc::new(RefCell::new(Some("".to_string()))), Rc::new(RefCell::new(Some(Box::new(move |a: Rc<RefCell<Option<String>>>, w: Rc<RefCell<Option<String>>>| -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some({
            let mut __s = String::new();
            __s.push_str(&format!("{}", (*a.borrow().as_ref().unwrap())));
            __s.push_str(&format!("{}", (*w.borrow().as_ref().unwrap())));
            __s.push_str(&format!("{}", "|".to_string()));
            __s
        })))
    }) as Box<dyn FnMut(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>>)))).borrow().as_ref().unwrap())));
    let mut ids = keys__int__bool(Rc::new(RefCell::new(Some(BTreeMap::<i64, Rc<RefCell<Option<bool>>>>::from([(3i64, Rc::new(RefCell::new(Some(true)))), (1i64, Rc::new(RefCell::new(Some(false)))), (2i64, Rc::new(RefCell::new(Some(true))))])))));
    let mut ids = { let __slice_header = GoSliceHeader::go_slice_header(&ids); Rc::new(RefCell::new(__slice_header)) };
    println!("{}", format!("{}", format_slice(&ids)));
}