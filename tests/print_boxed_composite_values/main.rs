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


// Go expands a pointer to a struct only when the pointer is the value being
// printed. Inside a struct or a slice it prints the address. That is also
// what keeps a value that points back to itself from printing forever.
fn go_format_any_pointer(nested: bool, address: *const (), is_nil: impl FnOnce() -> bool, expand: impl FnOnce() -> String) -> String {
    if !nested {
        return expand();
    }
    if is_nil() { "<nil>".to_string() } else { format!("{:p}", address) }
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

fn format_any(value: &dyn Any) -> String {
    format_any_at(value, false)
}

// Prints a value held inside the one being printed: a struct's interface
// field, an element of a []interface{}.
fn format_any_nested(value: &dyn Any) -> String {
    format_any_at(value, true)
}

fn format_any_at(value: &dyn Any, nested: bool) -> String {
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
    } else if let Some(v) = value.downcast_ref::<Label>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<Node>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<NotFound>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<Outcome>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<Point>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<Point>>>>>>>>() {
        format_map(v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>() {
        format_map(v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<BTreeMap<i64, Rc<RefCell<Option<String>>>>>>>>() {
        format_map(v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Node>>>>() {
        go_format_any_pointer(nested, Rc::as_ptr(v) as *const (), || v.try_borrow().map(|__guard| __guard.is_none()).unwrap_or(false), || match (*v.borrow()).as_ref() { Some(__pointee) => format!("&{}", __pointee), None => "<nil>".to_string() })
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<NotFound>>>>() {
        match (*v.borrow()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() }
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Point>>>>() {
        go_format_any_pointer(nested, Rc::as_ptr(v) as *const (), || v.try_borrow().map(|__guard| __guard.is_none()).unwrap_or(false), || match (*v.borrow()).as_ref() { Some(__pointee) => format!("&{}", __pointee), None => "<nil>".to_string() })
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Status>>>>() {
        match (*v.borrow()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() }
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<Point>>>>>() {
        format_slice(v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<String>>>>>() {
        format_slice(v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<bool>>>>>() {
        format_slice(v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<f64>>>>>() {
        format_slice(v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<i64>>>>>() {
        format_slice(v)
    } else if let Some(v) = value.downcast_ref::<Report>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<Status>() {
        format!("{}", v)
    } else {
        panic!("go2rust: cannot print this value held in an interface: its type has no formatting case yet")
    }
}

fn format_any_slice_values(slice: &Rc<RefCell<Option<Vec<Box<dyn Any>>>>>, format_value: fn(&dyn Any) -> String) -> String {
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.iter().map(|v| format_value(v.as_ref())).collect();
        formatted.join(" ")
    } else {
        String::new()
    }
}

fn format_any_slice(slice: &Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> String {
    format!("[{}]", format_any_slice_values(slice, format_any_nested))
}

fn format_any_variadic(slice: &Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> String {
    format_any_slice_values(slice, format_any)
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
    if let Some(v) = value.downcast_ref::<Label>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Node>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<NotFound>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Outcome>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Point>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<Point>>>>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<BTreeMap<i64, Rc<RefCell<Option<String>>>>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Node>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<NotFound>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Point>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Status>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<Point>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<String>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<bool>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<f64>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<i64>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Report>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Status>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    None
}

fn go_any_clone(value: &dyn Any) -> Box<dyn Any> {
    go_any_try_clone(value).unwrap_or_else(|| panic!("go_any_clone: unsupported dynamic type; add typed lowering instead of cloning Box<dyn Any>"))
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
pub struct Status {
    pub code: Rc<RefCell<Option<i64>>>,
}

impl Status {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.code.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            code: __go_clone_0_0,
        }
    }
}


impl Default for Status {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            code: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct NotFound {
    pub name: Rc<RefCell<Option<String>>>,
}

impl NotFound {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for NotFound {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for NotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}


/// Node can point at itself through its interface field.
#[derive(Clone)]
pub struct Node {
    pub name: Rc<RefCell<Option<String>>>,
    pub next: Rc<RefCell<Option<Box<dyn Any>>>>,
}

impl Node {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = self.next.clone();
        Self {
            name: __go_clone_0_0,
            next: __go_clone_1_0,
        }
    }
}


impl Default for Node {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(None));
        Self {
            name: __go_default_0_0,
            next: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", match self.next.borrow().as_ref() { Some(__v) => format_any_nested(__v.as_ref()), None => "<nil>".to_string() });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


/// Label prints itself by printing another value. That is a print call of its
/// own: the pointer it prints is at the top level of that call, wherever the
/// Label sits in the value being printed.
#[derive(Debug, Clone)]
pub struct Label {
    pub x: Rc<RefCell<Option<i64>>>,
}

impl Label {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
        }
    }
}


impl Default for Label {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            x: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().borrow().as_ref().unwrap()))
    }
}


/// Missing does the same from its Error method.
#[derive(Debug, Clone)]
pub struct Missing {
    pub line: Rc<RefCell<Option<i64>>>,
}

impl Missing {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.line.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            line: __go_clone_0_0,
        }
    }
}


impl Default for Missing {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            line: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Missing {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}


/// Outcome is the usual result struct: its error is nil unless something
/// failed.
#[derive(Clone)]
pub struct Outcome {
    pub value: Rc<RefCell<Option<i64>>>,
    pub err: Rc<RefCell<Option<Rc<dyn StdError>>>>,
}

impl Outcome {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.value.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = self.err.clone();
        Self {
            value: __go_clone_0_0,
            err: __go_clone_1_0,
        }
    }
}


impl Default for Outcome {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_1_0 = Rc::new(RefCell::new(None));
        Self {
            value: __go_default_0_0,
            err: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Outcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.value.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", match self.err.borrow().as_ref() { Some(__v) => format!("{}", __v), None => "<nil>".to_string() });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


/// Failure is another name for error. A field of this type is an error field.
pub type Failure = Rc<RefCell<Option<Rc<dyn StdError>>>>;


#[derive(Clone, Default)]
pub struct Report {
    pub err: Failure,
}

impl Report {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.err.clone();
        Self {
            err: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", match self.err.borrow().as_ref() { Some(__v) => format!("{}", __v), None => "<nil>".to_string() });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Status {
    pub fn string(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("status {}", (*self.code.borrow().as_ref().unwrap())))))
    }
}

impl NotFound {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some(format!("{}{}", (*self.name.clone().borrow().as_ref().unwrap()), " not found".to_string()))));
    }
}

impl StdError for NotFound {}


impl Label {
    pub fn string(&self) -> Rc<RefCell<Option<String>>> {
        render(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some({ let __selector_holder = self.x.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }))), ..Default::default() }))).clone()) as Box<dyn Any>))))
    }
}

impl Missing {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("{}{}", "missing at ".to_string(), (*render(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some({ let __selector_holder = self.line.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }))), ..Default::default() }))).clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())))))
    }
}

impl StdError for Missing {}


pub fn render(v: Rc<RefCell<Option<Box<dyn Any>>>>) -> Rc<RefCell<Option<String>>> {
    Rc::new(RefCell::new(Some(format!("{}", match v.borrow().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }))))
}

/// shape describes text around the addresses in it, which change from run to
/// run: whether it starts with prefix, and how many addresses it holds.
pub fn shape(text: Rc<RefCell<Option<String>>>, prefix: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {
    Rc::new(RefCell::new(Some(format!("{} {}", (*Rc::new(RefCell::new(Some({ let __s = (*text.borrow().as_ref().unwrap()).clone(); let __arg = (*prefix.borrow().as_ref().unwrap()).clone(); __s.starts_with(&__arg) }))).borrow().as_ref().unwrap()), ((*Rc::new(RefCell::new(Some({ let __s = (*text.borrow().as_ref().unwrap()).clone(); let __sep = "0x".to_string(); __s.split(&__sep).map(|__part| __part.to_string()).collect::<Vec<String>>() }))).borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i64) - (1 as i64)))))
}

/// show prints whatever it is given. The value arrives as an interface, so
/// the printer has to find out what it holds.
pub fn show(v: Rc<RefCell<Option<Box<dyn Any>>>>) {
    println!("{}", format!("{}", match v.borrow().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }));
}

fn main() {
    show(Rc::new(RefCell::new(Some(Box::new(42i64) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new("hi".to_string()) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(2.5) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(true) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(vec![1i64, 2i64, 3i64]))).clone()) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(vec!["a".to_string(), "b".to_string()]))).clone()) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(vec![true, false]))).clone()) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(vec![1.5, 2.0]))).clone()) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(vec![Point { x: Rc::new(RefCell::new(Some(1))), y: Rc::new(RefCell::new(Some(2))), ..Default::default() }, Point { x: Rc::new(RefCell::new(Some(3))), y: Rc::new(RefCell::new(Some(4))), ..Default::default() }]))).clone()) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i64>>>>::from([("b".to_string(), Rc::new(RefCell::new(Some(2i64)))), ("a".to_string(), Rc::new(RefCell::new(Some(1i64))))])))).clone()) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(BTreeMap::<i64, Rc<RefCell<Option<String>>>>::from([(2i64, Rc::new(RefCell::new(Some("two".to_string())))), (1i64, Rc::new(RefCell::new(Some("one".to_string()))))])))).clone()) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Point>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(1))), y: Rc::new(RefCell::new(Some(2))), ..Default::default() }))))])))).clone()) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(Point { x: Rc::new(RefCell::new(Some(1))), y: Rc::new(RefCell::new(Some(2))), ..Default::default() }) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(3))), y: Rc::new(RefCell::new(Some(4))), ..Default::default() }))).clone()) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(Status { code: Rc::new(RefCell::new(Some(404))), ..Default::default() }) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(NotFound { name: Rc::new(RefCell::new(Some("x".to_string()))), ..Default::default() }) as Box<dyn Any>))));

    let mut none: Rc<RefCell<Option<Vec<i64>>>> = Rc::new(RefCell::new(None));
    show(Rc::new(RefCell::new(Some(Box::new(none.clone()) as Box<dyn Any>))));
    let mut nowhere: Rc<RefCell<Option<Point>>> = Rc::new(RefCell::new(None));
    show(Rc::new(RefCell::new(Some(Box::new(nowhere.clone()) as Box<dyn Any>))));

    let mut xs = Rc::new(RefCell::new(Some(vec![4i64, 5i64])));
    let mut p = Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(6))), y: Rc::new(RefCell::new(Some(7))), ..Default::default() })));
    let mut boxed: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(xs.clone()) as Box<dyn Any>)));
    println!("{} {} {}", format!("{}", "println:".to_string()), format!("{}", match boxed.borrow().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }), format!("{}", Point { x: Rc::new(RefCell::new(Some(5))), y: Rc::new(RefCell::new(Some(6))), ..Default::default() }));
    print!("printf: {} {}\n", match boxed.borrow().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }, 7);
    { let new_val = Box::new(p.clone()) as Box<dyn Any>; *boxed.borrow_mut() = Some(new_val); };
    print!("printf: {}\n", match boxed.borrow().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() });

    let mut items = Rc::new(RefCell::new(Some(vec![Box::new(Rc::new(RefCell::new(Some(vec![1i64, 2i64]))).clone()) as Box<dyn Any>, Box::new(Point { x: Rc::new(RefCell::new(Some(7))), y: Rc::new(RefCell::new(Some(8))), ..Default::default() }) as Box<dyn Any>, Box::new(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i64>>>>::from([("k".to_string(), Rc::new(RefCell::new(Some(1i64))))])))).clone()) as Box<dyn Any>, Box::new("s".to_string()) as Box<dyn Any>, Box::new(3i64) as Box<dyn Any>])));
    { let __range_holder = items.clone(); let __range_guard = __range_holder.borrow(); let __range_len = __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); std::mem::drop(__range_guard); let __range_values = 0..__range_len; for item in __range_values.map(|__range_index| { let __range_guard = __range_holder.borrow(); go_any_clone(__range_guard.as_ref().unwrap()[__range_index].as_ref()) }) {
        println!("{} {}", format!("{}", "item:".to_string()), format!("{}", format_any(item.as_ref())));
    } }

    let mut names = Rc::new(RefCell::new(Some(vec!["x".to_string()])));
    let mut byName = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Box<dyn Any>>>>>::from([("point".to_string(), Rc::new(RefCell::new(Some(Box::new(Point { x: Rc::new(RefCell::new(Some(9))), y: Rc::new(RefCell::new(Some(9))), ..Default::default() }) as Box<dyn Any>)))), ("list".to_string(), Rc::new(RefCell::new(Some(Box::new(names.clone()) as Box<dyn Any>))))]))));
    println!("{} {}", format!("{}", match ({ let __map_holder = byName.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"point".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }).borrow().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }), format!("{}", match ({ let __map_holder = byName.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"list".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }).borrow().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }));

        // A pointer below the top level prints as its address, so a value that
        // points back to itself prints once and stops. The address changes from
        // run to run; check the shape around it.
    let mut r#loop = Rc::new(RefCell::new(Some(Node { name: Rc::new(RefCell::new(Some("loop".to_string()))), ..Default::default() })));
    { let new_val = Box::new(r#loop.clone()) as Box<dyn Any>; *(*r#loop.borrow().as_ref().unwrap()).next.borrow_mut() = Some(new_val); };
    let mut text = render(Rc::new(RefCell::new(Some(Box::new(r#loop.clone()) as Box<dyn Any>))));
    println!("{} {} {} {}", format!("{}", "cycle:".to_string()), format!("{}", (*Rc::new(RefCell::new(Some({ let __s = (*text.borrow().as_ref().unwrap()).clone(); let __arg = "&{loop 0x".to_string(); __s.starts_with(&__arg) }))).borrow().as_ref().unwrap())), format!("{}", (*Rc::new(RefCell::new(Some({ let __s = (*text.borrow().as_ref().unwrap()).clone(); let __arg = "}".to_string(); __s.ends_with(&__arg) }))).borrow().as_ref().unwrap())), format!("{}", ((*Rc::new(RefCell::new(Some({ let __s = (*text.borrow().as_ref().unwrap()).clone(); let __sep = "loop".to_string(); __s.split(&__sep).map(|__part| __part.to_string()).collect::<Vec<String>>() }))).borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i64) - (1 as i64)));
    let mut tail = Rc::new(RefCell::new(Some(Node { name: Rc::new(RefCell::new(Some("tail".to_string()))), ..Default::default() })));
    { let new_val = render(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(Node { name: Rc::new(RefCell::new(Some("head".to_string()))), next: Rc::new(RefCell::new(Some(Box::new(tail.clone()) as Box<dyn Any>))), ..Default::default() }))).clone()) as Box<dyn Any>)))); let __moved_val = { let mut __guard = new_val.borrow_mut(); __guard.take() }; *text.borrow_mut() = __moved_val; };
    println!("{} {} {}", format!("{}", "chain:".to_string()), format!("{}", (*Rc::new(RefCell::new(Some({ let __s = (*text.borrow().as_ref().unwrap()).clone(); let __arg = "&{head 0x".to_string(); __s.starts_with(&__arg) }))).borrow().as_ref().unwrap())), format!("{}", (*Rc::new(RefCell::new(Some({ let __s = (*text.borrow().as_ref().unwrap()).clone(); let __arg = "tail".to_string(); __s.contains(&__arg) }))).borrow().as_ref().unwrap())));

        // The same holds when the value being printed is a struct and the
        // pointer is in one of its fields, however the struct reaches the
        // printer, and when the pointer is an element of a slice of interfaces.
    println!("{} {}", format!("{}", "struct value, boxed:".to_string()), format!("{}", (*shape(render(Rc::new(RefCell::new(Some(Box::new({ let __v = (*r#loop.borrow().as_ref().unwrap()).clone(); __v }) as Box<dyn Any>)))), Rc::new(RefCell::new(Some("{loop 0x".to_string())))).borrow().as_ref().unwrap())));
    println!("{} {}", format!("{}", "struct value, typed:".to_string()), format!("{}", (*shape(Rc::new(RefCell::new(Some(format!("{}", { let __v = (*r#loop.borrow().as_ref().unwrap()).clone(); __v })))), Rc::new(RefCell::new(Some("{loop 0x".to_string())))).borrow().as_ref().unwrap())));
    println!("{} {}", format!("{}", "pointer, typed:".to_string()), format!("{}", (*shape(Rc::new(RefCell::new(Some(format!("{}", { let __text = match r#loop.borrow().as_ref() { Some(__value) => format!("&{}", __value), None => "<nil>".to_string() }; __text })))), Rc::new(RefCell::new(Some("&{loop 0x".to_string())))).borrow().as_ref().unwrap())));
    { let new_val = render(Rc::new(RefCell::new(Some(Box::new(Node { name: Rc::new(RefCell::new(Some("holder".to_string()))), next: Rc::new(RefCell::new(Some(Box::new(tail.clone()) as Box<dyn Any>))), ..Default::default() }) as Box<dyn Any>)))); let __moved_val = { let mut __guard = new_val.borrow_mut(); __guard.take() }; *text.borrow_mut() = __moved_val; };
    println!("{} {} {}", format!("{}", "struct holding a pointer:".to_string()), format!("{}", (*shape(Rc::new(RefCell::new(Some({ let __arg_holder = text.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some("{holder 0x".to_string())))).borrow().as_ref().unwrap())), format!("{}", (*Rc::new(RefCell::new(Some({ let __s = (*text.borrow().as_ref().unwrap()).clone(); let __arg = "tail".to_string(); __s.contains(&__arg) }))).borrow().as_ref().unwrap())));
    let mut mixed = Rc::new(RefCell::new(Some(vec![Box::new(tail.clone()) as Box<dyn Any>, Box::new(1i64) as Box<dyn Any>])));
    { let new_val = Rc::new(RefCell::new(Some(format!("{}", format_any_slice(&mixed))))); let __moved_val = { let mut __guard = new_val.borrow_mut(); __guard.take() }; *text.borrow_mut() = __moved_val; };
    println!("{} {} {}", format!("{}", "slice holding a pointer:".to_string()), format!("{}", (*shape(Rc::new(RefCell::new(Some({ let __arg_holder = text.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some("[0x".to_string())))).borrow().as_ref().unwrap())), format!("{}", (*Rc::new(RefCell::new(Some({ let __s = (*text.borrow().as_ref().unwrap()).clone(); let __arg = " 1]".to_string(); __s.ends_with(&__arg) }))).borrow().as_ref().unwrap())));
    let mut noNode: Rc<RefCell<Option<Node>>> = Rc::new(RefCell::new(None));
    show(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(Node { name: Rc::new(RefCell::new(Some("end".to_string()))), next: Rc::new(RefCell::new(Some(Box::new(noNode.clone()) as Box<dyn Any>))), ..Default::default() }))).clone()) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(p.clone()) as Box<dyn Any>))));

        // The zero value of a struct has nil interface and error fields. They
        // print as <nil>, whether the struct is printed directly or through an
        // interface.
    println!("{}", format!("{}", Node { name: Rc::new(RefCell::new(Some("typed".to_string()))), ..Default::default() }));
    show(Rc::new(RefCell::new(Some(Box::new(Node { name: Rc::new(RefCell::new(Some("boxed".to_string()))), ..Default::default() }) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(Node { name: Rc::new(RefCell::new(Some("pointer".to_string()))), ..Default::default() }))).clone()) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(Node { name: Rc::new(RefCell::new(Some("full".to_string()))), next: Rc::new(RefCell::new(Some(Box::new(3i64) as Box<dyn Any>))), ..Default::default() }) as Box<dyn Any>))));
    println!("{}", format!("{}", Outcome { value: Rc::new(RefCell::new(Some(1))), ..Default::default() }));
    show(Rc::new(RefCell::new(Some(Box::new(Outcome { value: Rc::new(RefCell::new(Some(2))), ..Default::default() }) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(Outcome { value: Rc::new(RefCell::new(Some(3))), err: Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(format!("failed")))))), ..Default::default() }) as Box<dyn Any>))));
    println!("{}", format!("{}", Report { err: Default::default() }));
    show(Rc::new(RefCell::new(Some(Box::new(Report { err: Default::default() }) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(Report { err: Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(format!("reported")))))), ..Default::default() }) as Box<dyn Any>))));

        // A String or Error method that prints starts a print call of its own.
        // The pointer it prints is expanded, however deep the value with the
        // method sits: in a field, behind a pointer, in a slice, in an error field.
    println!("{}", format!("{}", Label { x: Rc::new(RefCell::new(Some(7))), ..Default::default() }));
    show(Rc::new(RefCell::new(Some(Box::new(Label { x: Rc::new(RefCell::new(Some(7))), ..Default::default() }) as Box<dyn Any>))));
    println!("{}", format!("{}", Node { name: Rc::new(RefCell::new(Some("row".to_string()))), next: Rc::new(RefCell::new(Some(Box::new(Label { x: Rc::new(RefCell::new(Some(7))), ..Default::default() }) as Box<dyn Any>))), ..Default::default() }));
    show(Rc::new(RefCell::new(Some(Box::new(Node { name: Rc::new(RefCell::new(Some("row".to_string()))), next: Rc::new(RefCell::new(Some(Box::new(Label { x: Rc::new(RefCell::new(Some(7))), ..Default::default() }) as Box<dyn Any>))), ..Default::default() }) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(Node { name: Rc::new(RefCell::new(Some("row".to_string()))), next: Rc::new(RefCell::new(Some(Box::new(Label { x: Rc::new(RefCell::new(Some(7))), ..Default::default() }) as Box<dyn Any>))), ..Default::default() }))).clone()) as Box<dyn Any>))));
    let mut labels = Rc::new(RefCell::new(Some(vec![Box::new(Label { x: Rc::new(RefCell::new(Some(8))), ..Default::default() }) as Box<dyn Any>, Box::new(Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(2))), y: Rc::new(RefCell::new(Some(3))), ..Default::default() }))).clone()) as Box<dyn Any>, Box::new(1i64) as Box<dyn Any>])));
    println!("{} {}", format!("{}", "slice:".to_string()), format!("{}", (*shape(Rc::new(RefCell::new(Some(format!("{}", format_any_slice(&labels))))), Rc::new(RefCell::new(Some("[&{8 0} 0x".to_string())))).borrow().as_ref().unwrap())));
    println!("{}", format_any_variadic(&labels));
    println!("{}", format!("{}", Outcome { value: Rc::new(RefCell::new(Some(4))), err: Rc::new(RefCell::new(Some(Rc::new(Missing { line: Rc::new(RefCell::new(Some(9))), ..Default::default() }) as Rc<dyn StdError>))), ..Default::default() }));
    show(Rc::new(RefCell::new(Some(Box::new(Node { name: Rc::new(RefCell::new(Some("outer".to_string()))), next: Rc::new(RefCell::new(Some(Box::new(Outcome { value: Rc::new(RefCell::new(Some(5))), err: Rc::new(RefCell::new(Some(Rc::new(Missing { line: Rc::new(RefCell::new(Some(9))), ..Default::default() }) as Rc<dyn StdError>))), ..Default::default() }) as Box<dyn Any>))), ..Default::default() }) as Box<dyn Any>))));

        // A pointer to a type with a String or Error method prints through the
        // method, at the top level and inside another value. A nil one is <nil>.
    show(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(Status { code: Rc::new(RefCell::new(Some(404))), ..Default::default() }))).clone()) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(Node { name: Rc::new(RefCell::new(Some("n".to_string()))), next: Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(Status { code: Rc::new(RefCell::new(Some(5))), ..Default::default() }))).clone()) as Box<dyn Any>))), ..Default::default() }) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(NotFound { name: Rc::new(RefCell::new(Some("y".to_string()))), ..Default::default() }))).clone()) as Box<dyn Any>))));
    let mut noStatus: Rc<RefCell<Option<Status>>> = Rc::new(RefCell::new(None));
    show(Rc::new(RefCell::new(Some(Box::new(noStatus.clone()) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(Node { name: Rc::new(RefCell::new(Some("n".to_string()))), next: Rc::new(RefCell::new(Some(Box::new(noStatus.clone()) as Box<dyn Any>))), ..Default::default() }) as Box<dyn Any>))));
}