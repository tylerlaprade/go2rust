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
                    format!("{}:{}", k, val)
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

#[derive(Debug, Clone)]
pub struct Cache__string__User {
    pub data: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<User>>>>>>>,
    pub order: Rc<RefCell<Option<Vec<String>>>>,
    pub limit: Rc<RefCell<Option<i64>>>,
}

impl Cache__string__User {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.data.clone();
        let __go_clone_1_0 = self.order.clone();
        let __go_clone_2_0 = { let __guard = self.limit.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            data: __go_clone_0_0,
            order: __go_clone_1_0,
            limit: __go_clone_2_0,
        }
    }
}


impl Default for Cache__string__User {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(None));
        let __go_default_1_0 = Rc::new(RefCell::new(None));
        let __go_default_2_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            data: __go_default_0_0,
            order: __go_default_1_0,
            limit: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for Cache__string__User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_map(&self.data));
        let __go_fmt_1 = format!("{}", format_slice(&self.order));
        let __go_fmt_2 = format!("{}", (*self.limit.borrow().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


#[derive(Debug, Clone)]
pub struct Cache__int__slice_string {
    pub data: Rc<RefCell<Option<BTreeMap<i64, Rc<RefCell<Option<Vec<String>>>>>>>>,
    pub order: Rc<RefCell<Option<Vec<i64>>>>,
    pub limit: Rc<RefCell<Option<i64>>>,
}

impl Cache__int__slice_string {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.data.clone();
        let __go_clone_1_0 = self.order.clone();
        let __go_clone_2_0 = { let __guard = self.limit.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            data: __go_clone_0_0,
            order: __go_clone_1_0,
            limit: __go_clone_2_0,
        }
    }
}


impl Default for Cache__int__slice_string {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(None));
        let __go_default_1_0 = Rc::new(RefCell::new(None));
        let __go_default_2_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            data: __go_default_0_0,
            order: __go_default_1_0,
            limit: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for Cache__int__slice_string {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", "<map>");
        let __go_fmt_1 = format!("{}", format_slice(&self.order));
        let __go_fmt_2 = format!("{}", (*self.limit.borrow().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub name: Rc<RefCell<Option<String>>>,
    pub age: Rc<RefCell<Option<i64>>>,
}

impl User {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.age.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
            age: __go_clone_1_0,
        }
    }
}


impl Default for User {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            name: __go_default_0_0,
            age: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.age.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub trait Number: std::fmt::Display + Any {
    fn __go_clone_box_number(&self) -> Box<dyn Number>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_number(&self, other: &dyn Number) -> bool;
}

impl Clone for Box<dyn Number> {
    fn clone(&self) -> Self {
        Number::__go_clone_box_number(self.as_ref())
    }
}

impl Number for i8 {
    fn __go_clone_box_number(&self) -> Box<dyn Number> {
        Box::new(self.clone()) as Box<dyn Number>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &dyn Number) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<i8>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for i16 {
    fn __go_clone_box_number(&self) -> Box<dyn Number> {
        Box::new(self.clone()) as Box<dyn Number>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &dyn Number) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<i16>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for i32 {
    fn __go_clone_box_number(&self) -> Box<dyn Number> {
        Box::new(self.clone()) as Box<dyn Number>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &dyn Number) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<i32>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for i64 {
    fn __go_clone_box_number(&self) -> Box<dyn Number> {
        Box::new(self.clone()) as Box<dyn Number>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &dyn Number) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<i64>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for isize {
    fn __go_clone_box_number(&self) -> Box<dyn Number> {
        Box::new(self.clone()) as Box<dyn Number>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &dyn Number) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<isize>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for u8 {
    fn __go_clone_box_number(&self) -> Box<dyn Number> {
        Box::new(self.clone()) as Box<dyn Number>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &dyn Number) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<u8>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for u16 {
    fn __go_clone_box_number(&self) -> Box<dyn Number> {
        Box::new(self.clone()) as Box<dyn Number>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &dyn Number) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<u16>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for u32 {
    fn __go_clone_box_number(&self) -> Box<dyn Number> {
        Box::new(self.clone()) as Box<dyn Number>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &dyn Number) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<u32>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for u64 {
    fn __go_clone_box_number(&self) -> Box<dyn Number> {
        Box::new(self.clone()) as Box<dyn Number>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &dyn Number) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<u64>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for usize {
    fn __go_clone_box_number(&self) -> Box<dyn Number> {
        Box::new(self.clone()) as Box<dyn Number>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &dyn Number) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<usize>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for f32 {
    fn __go_clone_box_number(&self) -> Box<dyn Number> {
        Box::new(self.clone()) as Box<dyn Number>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &dyn Number) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<f32>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for f64 {
    fn __go_clone_box_number(&self) -> Box<dyn Number> {
        Box::new(self.clone()) as Box<dyn Number>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &dyn Number) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<f64>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for String {
    fn __go_clone_box_number(&self) -> Box<dyn Number> {
        Box::new(self.clone()) as Box<dyn Number>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &dyn Number) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<String>() {
            self == __other
        } else {
            false
        }
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


impl Cache__int__slice_string {
    pub fn put(&mut self, k: Rc<RefCell<Option<i64>>>, v: Rc<RefCell<Option<Vec<String>>>>) {
        {
        let (_, mut ok) = { let __map_holder = self.data.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&(*k.borrow().as_ref().unwrap()))) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } };;
        if !ok {
            { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = self.order.clone(); let __append_value = (*k.borrow().as_ref().unwrap()).clone(); let mut __appended: Vec<i64> = (*__append_target.borrow()).clone().unwrap_or_default(); __appended.push(__append_value); Rc::new(RefCell::new(Some(__appended))) })); *self.order.borrow_mut() = __slice_header; };;
        }
    }
        { let __map_key = (*k.borrow().as_ref().unwrap()); let __map_value = { let __slice_header = GoSliceHeader::go_slice_header(&v); Rc::new(RefCell::new(__slice_header)) }; (*self.data.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
        if (({ let __len_target = { let __field = self.order.clone(); __field }; let __len_guard = __len_target.borrow(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i64) > ((*self.limit.clone().borrow().as_ref().unwrap()) as i64) {
        let mut oldest = Rc::new(RefCell::new(Some((*self.order.borrow().as_ref().unwrap())[(0) as usize].clone())));
        { let new_val = Rc::new(RefCell::new(Some({ let __seq_holder = self.order.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = (1) as usize; let __high = __seq.len(); let __max = __source_cap; __seq.reslice3(__low, __high, __max) }))); self.order = new_val; };
        { let __map_handle = self.data.clone(); let mut __map_guard = __map_handle.borrow_mut(); __map_guard.as_mut().unwrap().remove(&(*oldest.borrow().as_ref().unwrap())); };
    }
    }

    pub fn get(&self, k: Rc<RefCell<Option<i64>>>) -> (Rc<RefCell<Option<Vec<String>>>>, Rc<RefCell<Option<Rc<dyn StdError>>>>) {
        let (mut v, mut ok) = { let __map_holder = self.data.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&(*k.borrow().as_ref().unwrap()))) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } };
        let mut v = { let __slice_header = GoSliceHeader::go_slice_header(&v); Rc::new(RefCell::new(__slice_header)) };
        if !ok {
        let mut zero: Rc<RefCell<Option<Vec<String>>>> = Rc::new(RefCell::new(None));
        return (zero.clone(), Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("miss".to_string()))))));
    }
        return (v.clone(), Rc::new(RefCell::new(None)));
    }
}

impl Cache__string__User {
    pub fn put(&mut self, k: Rc<RefCell<Option<String>>>, v: Rc<RefCell<Option<User>>>) {
        {
        let (_, mut ok) = { let __map_holder = self.data.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&(*k.borrow().as_ref().unwrap()).clone())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Rc::new(RefCell::new(Some(Default::default()))), false) } };;
        if !ok {
            { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = self.order.clone(); let __append_value = (*k.borrow().as_ref().unwrap()).clone(); let mut __appended: Vec<String> = (*__append_target.borrow()).clone().unwrap_or_default(); __appended.push(__append_value); Rc::new(RefCell::new(Some(__appended))) })); *self.order.borrow_mut() = __slice_header; };;
        }
    }
        { let __map_key = (*k.borrow().as_ref().unwrap()).clone(); let __map_value = { let __owned = v.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) }; (*self.data.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
        if (({ let __len_target = { let __field = self.order.clone(); __field }; let __len_guard = __len_target.borrow(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i64) > ((*self.limit.clone().borrow().as_ref().unwrap()) as i64) {
        let mut oldest = Rc::new(RefCell::new(Some((*self.order.borrow().as_ref().unwrap())[(0) as usize].clone())));
        { let new_val = Rc::new(RefCell::new(Some({ let __seq_holder = self.order.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = (1) as usize; let __high = __seq.len(); let __max = __source_cap; __seq.reslice3(__low, __high, __max) }))); self.order = new_val; };
        { let __map_handle = self.data.clone(); let mut __map_guard = __map_handle.borrow_mut(); __map_guard.as_mut().unwrap().remove(&(*oldest.borrow().as_ref().unwrap()).clone()); };
    }
    }

    pub fn get(&self, k: Rc<RefCell<Option<String>>>) -> (Rc<RefCell<Option<User>>>, Rc<RefCell<Option<Rc<dyn StdError>>>>) {
        let (mut v, mut ok) = { let __map_holder = self.data.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&(*k.borrow().as_ref().unwrap()).clone())) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().__go_value_clone()))), true), None => (Rc::new(RefCell::new(Some(Default::default()))), false) } };
        if !ok {
        let mut zero: Rc<RefCell<Option<User>>> = Rc::new(RefCell::new(Some(Default::default())));
        return ({ let __owned = zero.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) }, Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("miss".to_string()))))));
    }
        return ({ let __owned = v.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) }, Rc::new(RefCell::new(None)));
    }
}

impl Celsius {
}

impl Number for Celsius {
    fn __go_clone_box_number(&self) -> Box<dyn Number> {
        Box::new(self.clone()) as Box<dyn Number>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &dyn Number) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Celsius>() {
            self == __other
        } else {
            false
        }
    }
}

pub fn new_cache__string___user(limit: Rc<RefCell<Option<i64>>>) -> Rc<RefCell<Option<Cache__string__User>>> {
    Rc::new(RefCell::new(Some(Cache__string__User { data: Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<User>>>>::new()))), limit: Rc::new(RefCell::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })))
}

pub fn new_cache__int__slice_string(limit: Rc<RefCell<Option<i64>>>) -> Rc<RefCell<Option<Cache__int__slice_string>>> {
    Rc::new(RefCell::new(Some(Cache__int__slice_string { data: Rc::new(RefCell::new(Some(BTreeMap::<i64, Rc<RefCell<Option<Vec<String>>>>>::new()))), limit: Rc::new(RefCell::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })))
}

pub fn sum_by___user__int(xs: Rc<RefCell<Option<Vec<User>>>>, f: Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<User>>>) -> i64>>>>) -> i64 {
    let mut total: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));

    { let __range_holder = xs.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for x in __range_values.iter() {
        let x = &x.__go_value_clone();
        { let __rhs = { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<User>>>) -> i64> = { let mut __f_guard = f.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<User>>>) -> i64> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some((*x).__go_value_clone())))) }; let mut guard = total.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }
    return (*total.borrow().as_ref().unwrap());
}

pub fn sum_by___user__float64(xs: Rc<RefCell<Option<Vec<User>>>>, f: Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<User>>>) -> f64>>>>) -> f64 {
    let mut total: Rc<RefCell<Option<f64>>> = Rc::new(RefCell::new(Some(0.0)));

    { let __range_holder = xs.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for x in __range_values.iter() {
        let x = &x.__go_value_clone();
        { let __rhs = { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<User>>>) -> f64> = { let mut __f_guard = f.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<User>>>) -> f64> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some((*x).__go_value_clone())))) }; let mut guard = total.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }
    return (*total.borrow().as_ref().unwrap());
}

pub fn sum_by___celsius___celsius(xs: Rc<RefCell<Option<Vec<Celsius>>>>, f: Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<Celsius>>>) -> Rc<RefCell<Option<Celsius>>>>>>>) -> Rc<RefCell<Option<Celsius>>> {
    let mut total: Rc<RefCell<Option<Celsius>>> = Rc::new(RefCell::new(Some(Celsius(Rc::new(RefCell::new(Some(0.0)))))));

    { let __range_holder = xs.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for x in __range_values.iter().cloned() {
        { let __rhs = (*{ let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<Celsius>>>) -> Rc<RefCell<Option<Celsius>>>> = { let mut __f_guard = f.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<Celsius>>>) -> Rc<RefCell<Option<Celsius>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(x.clone())))) }.borrow().as_ref().unwrap()).clone(); let mut guard = total.borrow_mut(); *guard = Some(guard.as_ref().unwrap().clone() + __rhs); };
    } }
    return { let __owned = total.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
}

pub fn group_by___user__int(xs: Rc<RefCell<Option<Vec<User>>>>, key: Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<User>>>) -> i64>>>>) -> Rc<RefCell<Option<BTreeMap<i64, Rc<RefCell<Option<Vec<User>>>>>>>> {
    let mut out = Rc::new(RefCell::new(Some(BTreeMap::<i64, Rc<RefCell<Option<Vec<User>>>>>::new())));
    { let __range_holder = xs.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for x in __range_values.iter() {
        let x = &x.__go_value_clone();
        let mut k = { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<User>>>) -> i64> = { let mut __f_guard = key.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<User>>>) -> i64> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some((*x).__go_value_clone())))) };
        { let __map_key = k; let __map_value = { let __slice = { let __map_holder = out.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().unwrap().get(&k).cloned().unwrap_or_else(|| Rc::new(RefCell::new(None))) }; let __append_value = (*x).__go_value_clone(); let mut __appended = (*__slice.borrow()).clone().unwrap_or_default(); __appended.push(__append_value); Rc::new(RefCell::new(Some(__appended))) }; (*out.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    } }
    return out.clone();
}

fn main() {
    let mut c = new_cache__string___user(Rc::new(RefCell::new(Some(2))));
    (*c.borrow_mut().as_mut().unwrap()).put(Rc::new(RefCell::new(Some("a".to_string()))), Rc::new(RefCell::new(Some(User { name: Rc::new(RefCell::new(Some("Ann".to_string()))), age: Rc::new(RefCell::new(Some(30))), ..Default::default() }))));
    (*c.borrow_mut().as_mut().unwrap()).put(Rc::new(RefCell::new(Some("b".to_string()))), Rc::new(RefCell::new(Some(User { name: Rc::new(RefCell::new(Some("Bob".to_string()))), age: Rc::new(RefCell::new(Some(25))), ..Default::default() }))));
    (*c.borrow_mut().as_mut().unwrap()).put(Rc::new(RefCell::new(Some("c".to_string()))), Rc::new(RefCell::new(Some(User { name: Rc::new(RefCell::new(Some("Cy".to_string()))), age: Rc::new(RefCell::new(Some(40))), ..Default::default() }))));
    let (_, mut err) = (*c.borrow().as_ref().unwrap()).get(Rc::new(RefCell::new(Some("a".to_string()))));
    let (mut u, mut err2) = (*c.borrow().as_ref().unwrap()).get(Rc::new(RefCell::new(Some("c".to_string()))));
    println!("{} {} {} {} {}", format!("{}", { let __text = match err.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", (*u.borrow().as_ref().unwrap())), format!("{}", { let __text = match err2.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", ({ let __len_target = { let __field = (*c.borrow().as_ref().unwrap()).data.clone(); __field }; let __len_guard = __len_target.borrow(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })), format!("{}", format_slice_values(&(*(*c.borrow().as_ref().unwrap()).order.borrow().as_ref().unwrap()).clone())));

    let mut users = Rc::new(RefCell::new(Some(vec![User { name: Rc::new(RefCell::new(Some("Ann".to_string()))), age: Rc::new(RefCell::new(Some(30))), ..Default::default() }, User { name: Rc::new(RefCell::new(Some("Bob".to_string()))), age: Rc::new(RefCell::new(Some(25))), ..Default::default() }, User { name: Rc::new(RefCell::new(Some("Cy".to_string()))), age: Rc::new(RefCell::new(Some(40))), ..Default::default() }, User { name: Rc::new(RefCell::new(Some("Di".to_string()))), age: Rc::new(RefCell::new(Some(25))), ..Default::default() }])));
    println!("{}", format!("{}", sum_by___user__int(users.clone(), Rc::new(RefCell::new(Some(Box::new(move |u: Rc<RefCell<Option<User>>>| -> i64 {
        return (*(*u.borrow().as_ref().unwrap()).age.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut(Rc<RefCell<Option<User>>>) -> i64>))))));
    println!("{}", format!("{}", sum_by___user__float64(users.clone(), Rc::new(RefCell::new(Some(Box::new(move |u: Rc<RefCell<Option<User>>>| -> f64 {
        (*Rc::new(RefCell::new(Some({ let __selector_holder = (*u.borrow().as_ref().unwrap()).age.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned } as f64))).borrow().as_ref().unwrap()) / 2.0
    }) as Box<dyn FnMut(Rc<RefCell<Option<User>>>) -> f64>))))));
    let mut temps = Rc::new(RefCell::new(Some(vec![Celsius(Rc::new(RefCell::new(Some(20.5)))), Celsius(Rc::new(RefCell::new(Some(22.0)))), Celsius(Rc::new(RefCell::new(Some(19.5))))])));
    println!("{}", format!("{}", (*sum_by___celsius___celsius(temps.clone(), Rc::new(RefCell::new(Some(Box::new(move |c: Rc<RefCell<Option<Celsius>>>| -> Rc<RefCell<Option<Celsius>>> {
        { let __owned = c.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) }
    }) as Box<dyn FnMut(Rc<RefCell<Option<Celsius>>>) -> Rc<RefCell<Option<Celsius>>>>)))).borrow().as_ref().unwrap())));
    let mut groups = group_by___user__int(users.clone(), Rc::new(RefCell::new(Some(Box::new(move |u: Rc<RefCell<Option<User>>>| -> i64 {
        return (*(*u.borrow().as_ref().unwrap()).age.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut(Rc<RefCell<Option<User>>>) -> i64>))));
    let mut ages = Rc::new(RefCell::new(Some({ let mut v: Vec<i64> = (0..((*groups.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize).map(|_| 0).collect(); v.truncate((0) as usize); v })));
    for (age, _) in { let __range_holder = groups.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = ages.clone(); let __append_value = age; let mut __appended: Vec<i64> = (*__append_target.borrow()).clone().unwrap_or_default(); __appended.push(__append_value); Rc::new(RefCell::new(Some(__appended))) })); *ages.borrow_mut() = __slice_header; };
    }
    { let __sort_target = ages.clone(); let mut __sort_guard = __sort_target.borrow_mut(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort(); } };
    { let __range_holder = ages.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for age in __range_values.iter().copied() {
        let mut names: Rc<RefCell<Option<Vec<String>>>> = Rc::new(RefCell::new(None));
        { let __range_holder = { let __map_holder = groups.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&age)).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for u in __range_values.iter() {
        let u = &u.__go_value_clone();
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = names.clone(); let __append_value = { let __selector_holder = u.name.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }; let mut __appended: Vec<String> = (*__append_target.borrow()).clone().unwrap_or_default(); __appended.push(__append_value); Rc::new(RefCell::new(Some(__appended))) })); *names.borrow_mut() = __slice_header; };
    } }
        println!("{} {}", format!("{}", age), format!("{}", (*Rc::new(RefCell::new(Some({ let __parts = (*names.borrow()).as_ref().cloned().unwrap_or_default(); let __sep = ",".to_string(); __parts.join(&__sep) }))).borrow().as_ref().unwrap())));
    } }
    let mut ic = new_cache__int__slice_string(Rc::new(RefCell::new(Some(3))));
    (*ic.borrow_mut().as_mut().unwrap()).put(Rc::new(RefCell::new(Some(1))), Rc::new(RefCell::new(Some(vec!["x".to_string()]))));
    let (mut got, _) = (*ic.borrow().as_ref().unwrap()).get(Rc::new(RefCell::new(Some(1))));
    let mut got = { let __slice_header = GoSliceHeader::go_slice_header(&got); Rc::new(RefCell::new(__slice_header)) };
    println!("{} {}", format!("{}", format_slice(&got)), format!("{}", (*got.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)));
}