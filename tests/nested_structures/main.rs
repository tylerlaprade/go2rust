#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use std::any::Any;
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


fn format_slice_wrapped_stringer<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: GoSliceView<Rc<RefCell<Option<T>>>>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        { let __view = s.go_slice_view(); format_slice_wrapped_stringer_values(__view.as_ref()) }
    } else {
        "[]".to_string()
    }
}

fn format_slice_wrapped_stringer_values<T>(slice: &[Rc<RefCell<Option<T>>>]) -> String
where
    T: Display,
{
    let formatted: std::vec::Vec<String> = slice.iter().map(|v| {
        let inner = v.borrow();
        match inner.as_ref() {
            Some(value) => value.to_string(),
            None => "<nil>".to_string(),
        }
    }).collect();
    format!("[{}]", formatted.join(" "))
}

/// Interface for drawing
pub trait Drawable: std::fmt::Display + Any {
    fn __go_clone_box_drawable(&self) -> Box<dyn Drawable>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_drawable(&self, other: &dyn Drawable) -> bool;
    fn draw(&self) -> Rc<RefCell<Option<String>>>;
}

impl Clone for Box<dyn Drawable> {
    fn clone(&self) -> Self {
        Drawable::__go_clone_box_drawable(self.as_ref())
    }
}

/// Shape types
#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    pub radius: Rc<RefCell<Option<f64>>>,
}

impl Circle {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.radius.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            radius: __go_clone_0_0,
        }
    }
}


impl Default for Circle {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0.0)));
        Self {
            radius: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.radius.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    pub width: Rc<RefCell<Option<f64>>>,
    pub height: Rc<RefCell<Option<f64>>>,
}

impl Rectangle {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.width.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.height.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            width: __go_clone_0_0,
            height: __go_clone_0_1,
        }
    }
}


impl Default for Rectangle {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0.0)));
        let __go_default_0_1 = Rc::new(RefCell::new(Some(0.0)));
        Self {
            width: __go_default_0_0,
            height: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.width.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.height.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Clone)]
pub struct Canvas {
    pub name: Rc<RefCell<Option<String>>>,
    pub shapes: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Box<dyn Drawable>>>>>>>>,
}

impl Canvas {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = self.shapes.clone();
        Self {
            name: __go_clone_0_0,
            shapes: __go_clone_1_0,
        }
    }
}


impl Default for Canvas {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(None));
        Self {
            name: __go_default_0_0,
            shapes: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Canvas {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice_wrapped_stringer(&self.shapes));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


/// Nested struct definitions
#[derive(Debug, Clone)]
pub struct Address {
    pub street: Rc<RefCell<Option<String>>>,
    pub city: Rc<RefCell<Option<String>>>,
    pub state: Rc<RefCell<Option<String>>>,
    pub zip_code: Rc<RefCell<Option<String>>>,
    pub country: Rc<RefCell<Option<String>>>,
}

impl Address {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.street.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.city.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.state.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.zip_code.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.country.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            street: __go_clone_0_0,
            city: __go_clone_1_0,
            state: __go_clone_2_0,
            zip_code: __go_clone_3_0,
            country: __go_clone_4_0,
        }
    }
}


impl Default for Address {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_2_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_3_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_4_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            street: __go_default_0_0,
            city: __go_default_1_0,
            state: __go_default_2_0,
            zip_code: __go_default_3_0,
            country: __go_default_4_0,
        }
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.street.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.city.borrow().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.state.borrow().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.zip_code.borrow().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.country.borrow().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4)
    }
}


#[derive(Debug, Clone)]
pub struct Contact {
    pub email: Rc<RefCell<Option<String>>>,
    pub phone: Rc<RefCell<Option<String>>>,
}

impl Contact {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.email.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.phone.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            email: __go_clone_0_0,
            phone: __go_clone_1_0,
        }
    }
}


impl Default for Contact {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            email: __go_default_0_0,
            phone: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Contact {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.email.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.phone.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct Person {
    pub name: Rc<RefCell<Option<String>>>,
    pub age: Rc<RefCell<Option<i64>>>,
    pub address: Rc<RefCell<Option<Address>>>,
    pub contact: Rc<RefCell<Option<Contact>>>,
}

impl Person {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.age.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.address.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        let __go_clone_3_0 = { let __guard = self.contact.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        Self {
            name: __go_clone_0_0,
            age: __go_clone_1_0,
            address: __go_clone_2_0,
            contact: __go_clone_3_0,
        }
    }
}


impl Default for Person {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_2_0 = Rc::new(RefCell::new(Some(Address::default())));
        let __go_default_3_0 = Rc::new(RefCell::new(Some(Contact::default())));
        Self {
            name: __go_default_0_0,
            age: __go_default_1_0,
            address: __go_default_2_0,
            contact: __go_default_3_0,
        }
    }
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.age.borrow().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.address.borrow().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.contact.borrow().as_ref().unwrap()));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}


#[derive(Debug, Clone)]
pub struct Department {
    pub name: Rc<RefCell<Option<String>>>,
    pub manager: Rc<RefCell<Option<Person>>>,
    pub employees: Rc<RefCell<Option<Vec<Person>>>>,
    pub budget: Rc<RefCell<Option<f64>>>,
}

impl Department {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.manager.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        let __go_clone_2_0 = self.employees.clone();
        let __go_clone_3_0 = { let __guard = self.budget.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
            manager: __go_clone_1_0,
            employees: __go_clone_2_0,
            budget: __go_clone_3_0,
        }
    }
}


impl Default for Department {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(Person::default())));
        let __go_default_2_0 = Rc::new(RefCell::new(None));
        let __go_default_3_0 = Rc::new(RefCell::new(Some(0.0)));
        Self {
            name: __go_default_0_0,
            manager: __go_default_1_0,
            employees: __go_default_2_0,
            budget: __go_default_3_0,
        }
    }
}

impl std::fmt::Display for Department {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.manager.borrow().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", format_slice(&self.employees));
        let __go_fmt_3 = format!("{}", (*self.budget.borrow().as_ref().unwrap()));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}


#[derive(Debug, Clone)]
pub struct Company {
    pub name: Rc<RefCell<Option<String>>>,
    pub departments: Rc<RefCell<Option<Vec<Department>>>>,
    pub headquarters: Rc<RefCell<Option<Address>>>,
}

impl Company {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = self.departments.clone();
        let __go_clone_2_0 = { let __guard = self.headquarters.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        Self {
            name: __go_clone_0_0,
            departments: __go_clone_1_0,
            headquarters: __go_clone_2_0,
        }
    }
}


impl Default for Company {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(None));
        let __go_default_2_0 = Rc::new(RefCell::new(Some(Address::default())));
        Self {
            name: __go_default_0_0,
            departments: __go_default_1_0,
            headquarters: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for Company {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice(&self.departments));
        let __go_fmt_2 = format!("{}", (*self.headquarters.borrow().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


impl Circle {
    pub fn draw(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("Circle(r={:.1})", (*self.radius.borrow().as_ref().unwrap())))))
    }
}

impl Drawable for Circle {
    fn draw(&self) -> Rc<RefCell<Option<String>>> {
        Circle::draw(self)
    }
    fn __go_clone_box_drawable(&self) -> Box<dyn Drawable> {
        Box::new(self.clone()) as Box<dyn Drawable>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_drawable(&self, other: &dyn Drawable) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Circle>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct CirclePtr(pub Rc<RefCell<Option<Circle>>>);

impl std::fmt::Display for CirclePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Drawable for CirclePtr {
    fn draw(&self) -> Rc<RefCell<Option<String>>> {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        Circle::draw(__recv)
    }
    fn __go_clone_box_drawable(&self) -> Box<dyn Drawable> {
        Box::new(self.clone()) as Box<dyn Drawable>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_drawable(&self, other: &dyn Drawable) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<CirclePtr>() {
            let __go_left_nil = self.0.borrow().is_none();
            let __go_right_nil = __other.0.borrow().is_none();
            if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&self.0, &__other.0) }
        } else {
            false
        }
    }
}

impl Rectangle {
    pub fn draw(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("Rectangle({:.1}x{:.1})", (*self.width.borrow().as_ref().unwrap()), (*self.height.borrow().as_ref().unwrap())))))
    }
}

impl Drawable for Rectangle {
    fn draw(&self) -> Rc<RefCell<Option<String>>> {
        Rectangle::draw(self)
    }
    fn __go_clone_box_drawable(&self) -> Box<dyn Drawable> {
        Box::new(self.clone()) as Box<dyn Drawable>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_drawable(&self, other: &dyn Drawable) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Rectangle>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct RectanglePtr(pub Rc<RefCell<Option<Rectangle>>>);

impl std::fmt::Display for RectanglePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Drawable for RectanglePtr {
    fn draw(&self) -> Rc<RefCell<Option<String>>> {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        Rectangle::draw(__recv)
    }
    fn __go_clone_box_drawable(&self) -> Box<dyn Drawable> {
        Box::new(self.clone()) as Box<dyn Drawable>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_drawable(&self, other: &dyn Drawable) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<RectanglePtr>() {
            let __go_left_nil = self.0.borrow().is_none();
            let __go_right_nil = __other.0.borrow().is_none();
            if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&self.0, &__other.0) }
        } else {
            false
        }
    }
}

fn main() {
        // Create nested structures
    println!("{}", format!("{}", "=== Creating nested structures ===".to_string()));

        // Create addresses
    let mut hq = Rc::new(RefCell::new(Some(Address { street: Rc::new(RefCell::new(Some("123 Corporate Blvd".to_string()))), city: Rc::new(RefCell::new(Some("Tech City".to_string()))), state: Rc::new(RefCell::new(Some("CA".to_string()))), zip_code: Rc::new(RefCell::new(Some("90210".to_string()))), country: Rc::new(RefCell::new(Some("USA".to_string()))), ..Default::default() })));

    let mut managerAddr = Rc::new(RefCell::new(Some(Address { street: Rc::new(RefCell::new(Some("456 Manager St".to_string()))), city: Rc::new(RefCell::new(Some("Suburb".to_string()))), state: Rc::new(RefCell::new(Some("CA".to_string()))), zip_code: Rc::new(RefCell::new(Some("90211".to_string()))), country: Rc::new(RefCell::new(Some("USA".to_string()))), ..Default::default() })));

    let mut emp1Addr = Rc::new(RefCell::new(Some(Address { street: Rc::new(RefCell::new(Some("789 Employee Ave".to_string()))), city: Rc::new(RefCell::new(Some("Hometown".to_string()))), state: Rc::new(RefCell::new(Some("CA".to_string()))), zip_code: Rc::new(RefCell::new(Some("90212".to_string()))), country: Rc::new(RefCell::new(Some("USA".to_string()))), ..Default::default() })));

    let mut emp2Addr = Rc::new(RefCell::new(Some(Address { street: Rc::new(RefCell::new(Some("321 Worker Way".to_string()))), city: Rc::new(RefCell::new(Some("Village".to_string()))), state: Rc::new(RefCell::new(Some("CA".to_string()))), zip_code: Rc::new(RefCell::new(Some("90213".to_string()))), country: Rc::new(RefCell::new(Some("USA".to_string()))), ..Default::default() })));

        // Create contacts
    let mut managerContact = Rc::new(RefCell::new(Some(Contact { email: Rc::new(RefCell::new(Some("manager@company.com".to_string()))), phone: Rc::new(RefCell::new(Some("555-0001".to_string()))), ..Default::default() })));

    let mut emp1Contact = Rc::new(RefCell::new(Some(Contact { email: Rc::new(RefCell::new(Some("emp1@company.com".to_string()))), phone: Rc::new(RefCell::new(Some("555-0002".to_string()))), ..Default::default() })));

    let mut emp2Contact = Rc::new(RefCell::new(Some(Contact { email: Rc::new(RefCell::new(Some("emp2@company.com".to_string()))), phone: Rc::new(RefCell::new(Some("555-0003".to_string()))), ..Default::default() })));

        // Create people
    let mut manager = Rc::new(RefCell::new(Some(Person { name: Rc::new(RefCell::new(Some("Alice Manager".to_string()))), age: Rc::new(RefCell::new(Some(45))), address: Rc::new(RefCell::new(Some({ let __arg_holder = managerAddr.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))), contact: Rc::new(RefCell::new(Some({ let __arg_holder = managerContact.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))), ..Default::default() })));

    let mut employee1 = Rc::new(RefCell::new(Some(Person { name: Rc::new(RefCell::new(Some("Bob Employee".to_string()))), age: Rc::new(RefCell::new(Some(30))), address: Rc::new(RefCell::new(Some({ let __arg_holder = emp1Addr.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))), contact: Rc::new(RefCell::new(Some({ let __arg_holder = emp1Contact.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))), ..Default::default() })));

    let mut employee2 = Rc::new(RefCell::new(Some(Person { name: Rc::new(RefCell::new(Some("Carol Worker".to_string()))), age: Rc::new(RefCell::new(Some(28))), address: Rc::new(RefCell::new(Some({ let __arg_holder = emp2Addr.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))), contact: Rc::new(RefCell::new(Some({ let __arg_holder = emp2Contact.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))), ..Default::default() })));

        // Create department
    let mut engineering = Rc::new(RefCell::new(Some(Department { name: Rc::new(RefCell::new(Some("Engineering".to_string()))), manager: Rc::new(RefCell::new(Some({ let __arg_holder = manager.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))), employees: Rc::new(RefCell::new(Some(vec![(*employee1.borrow().as_ref().unwrap()).__go_value_clone(), (*employee2.borrow().as_ref().unwrap()).__go_value_clone()]))), budget: Rc::new(RefCell::new(Some(1e+06))), ..Default::default() })));

        // Create company
    let mut company = Rc::new(RefCell::new(Some(Company { name: Rc::new(RefCell::new(Some("TechCorp Inc".to_string()))), departments: Rc::new(RefCell::new(Some(vec![(*engineering.borrow().as_ref().unwrap()).__go_value_clone()]))), headquarters: Rc::new(RefCell::new(Some({ let __arg_holder = hq.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))), ..Default::default() })));

        // Access nested data
    println!("{}", format!("{}", "\n=== Accessing nested data ===".to_string()));

    print!("Company: {}\n", (*(*company.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone());
    print!("HQ Address: {}, {}, {} {}\n", (*(*(*company.borrow().as_ref().unwrap()).headquarters.borrow().as_ref().unwrap()).street.borrow().as_ref().unwrap()).clone(), (*(*(*company.borrow().as_ref().unwrap()).headquarters.borrow().as_ref().unwrap()).city.borrow().as_ref().unwrap()).clone(), (*(*(*company.borrow().as_ref().unwrap()).headquarters.borrow().as_ref().unwrap()).state.borrow().as_ref().unwrap()).clone(), (*(*(*company.borrow().as_ref().unwrap()).headquarters.borrow().as_ref().unwrap()).zip_code.borrow().as_ref().unwrap()).clone());

    print!("Department: {}\n", (*(*company.borrow().as_ref().unwrap()).departments.borrow().as_ref().unwrap().get_cloned((0) as usize).name.borrow().as_ref().unwrap()).clone());
    print!("Department Budget: ${:.2}\n", (*(*company.borrow().as_ref().unwrap()).departments.borrow().as_ref().unwrap().get_cloned((0) as usize).budget.borrow().as_ref().unwrap()));

    print!("Manager: {} (Age: {})\n", (*(*(*company.borrow().as_ref().unwrap()).departments.borrow().as_ref().unwrap().get_cloned((0) as usize).manager.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone(), (*(*(*company.borrow().as_ref().unwrap()).departments.borrow().as_ref().unwrap().get_cloned((0) as usize).manager.borrow().as_ref().unwrap()).age.borrow().as_ref().unwrap()));

    print!("Manager Email: {}\n", (*(*(*(*company.borrow().as_ref().unwrap()).departments.borrow().as_ref().unwrap().get_cloned((0) as usize).manager.borrow().as_ref().unwrap()).contact.borrow().as_ref().unwrap()).email.borrow().as_ref().unwrap()).clone());

    print!("Manager Address: {}, {}\n", (*(*(*(*company.borrow().as_ref().unwrap()).departments.borrow().as_ref().unwrap().get_cloned((0) as usize).manager.borrow().as_ref().unwrap()).address.borrow().as_ref().unwrap()).city.borrow().as_ref().unwrap()).clone(), (*(*(*(*company.borrow().as_ref().unwrap()).departments.borrow().as_ref().unwrap().get_cloned((0) as usize).manager.borrow().as_ref().unwrap()).address.borrow().as_ref().unwrap()).state.borrow().as_ref().unwrap()).clone());

        // Iterate through employees
    println!("{}", format!("{}", "\n=== Department employees ===".to_string()));

    { let __range_holder = (*company.borrow().as_ref().unwrap()).departments.borrow().as_ref().unwrap().get_cloned((0) as usize).employees.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice().to_vec()).unwrap_or_default(); drop(__range_guard); for (i, emp) in __range_values.iter().enumerate() {
        let emp = &emp.__go_value_clone();
        print!("Employee {}: {}\n", i as i64 + 1, (*emp.name.borrow().as_ref().unwrap()).clone());
        print!("  Age: {}\n", (*emp.age.borrow().as_ref().unwrap()));
        print!("  Email: {}\n", (*(*emp.contact.borrow().as_ref().unwrap()).email.borrow().as_ref().unwrap()).clone());
        print!("  Phone: {}\n", (*(*emp.contact.borrow().as_ref().unwrap()).phone.borrow().as_ref().unwrap()).clone());
        print!("  Address: {}, {}, {}\n", (*(*emp.address.borrow().as_ref().unwrap()).street.borrow().as_ref().unwrap()).clone(), (*(*emp.address.borrow().as_ref().unwrap()).city.borrow().as_ref().unwrap()).clone(), (*(*emp.address.borrow().as_ref().unwrap()).state.borrow().as_ref().unwrap()).clone());
        println!();
    } }

        // Nested maps
    println!("{}", format!("{}", "=== Nested maps ===".to_string()));

        // Map of maps
    let mut inventory = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>::from([("electronics".to_string(), Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i64>>>>::from([("laptops".to_string(), Rc::new(RefCell::new(Some(50i64)))), ("phones".to_string(), Rc::new(RefCell::new(Some(100i64)))), ("tablets".to_string(), Rc::new(RefCell::new(Some(25i64))))]))))), ("furniture".to_string(), Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i64>>>>::from([("chairs".to_string(), Rc::new(RefCell::new(Some(200i64)))), ("desks".to_string(), Rc::new(RefCell::new(Some(75i64)))), ("lamps".to_string(), Rc::new(RefCell::new(Some(150i64))))]))))), ("supplies".to_string(), Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i64>>>>::from([("pens".to_string(), Rc::new(RefCell::new(Some(1000i64)))), ("paper".to_string(), Rc::new(RefCell::new(Some(500i64)))), ("folders".to_string(), Rc::new(RefCell::new(Some(300i64))))])))))]))));

    println!("{}", format!("{}", "Inventory:".to_string()));
    let mut categories: Rc<RefCell<Option<Vec<String>>>> = Rc::new(RefCell::new(None));
    for (category, _) in { let __range_holder = inventory.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = categories.clone(); let __append_value = category.clone(); let mut __appended: Vec<String> = (*__append_target.borrow()).clone().unwrap_or_default(); __appended.push(__append_value); Rc::new(RefCell::new(Some(__appended))) })); *categories.borrow_mut() = __slice_header; };
    }
    { let __sort_target = categories.clone(); let mut __sort_guard = __sort_target.borrow_mut(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort(); } };
    { let __range_holder = categories.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice().to_vec()).unwrap_or_default(); drop(__range_guard); for category in __range_values.iter() {
        let mut items = { let __map_holder = inventory.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(category)).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) };
        print!("  {}:\n", category);
        let mut itemNames: Rc<RefCell<Option<Vec<String>>>> = Rc::new(RefCell::new(None));
        for (item, _) in { let __range_holder = items.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = itemNames.clone(); let __append_value = item.clone(); let mut __appended: Vec<String> = (*__append_target.borrow()).clone().unwrap_or_default(); __appended.push(__append_value); Rc::new(RefCell::new(Some(__appended))) })); *itemNames.borrow_mut() = __slice_header; };
    }
        { let __sort_target = itemNames.clone(); let mut __sort_guard = __sort_target.borrow_mut(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort(); } };
        { let __range_holder = itemNames.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice().to_vec()).unwrap_or_default(); drop(__range_guard); for item in __range_values.iter() {
        let mut count = Rc::new(RefCell::new(Some({ let __map_holder = items.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(item)).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) })));
        print!("    {}: {}\n", item, { let __v = (*count.borrow().as_ref().unwrap()).clone(); __v });
    } }
    } }

        // Access nested map values
    let mut laptopCount = Rc::new(RefCell::new(Some({ let __map_holder = { let __map_holder = inventory.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"electronics".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"laptops".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) })));
    print!("Laptop count: {}\n", { let __v = (*laptopCount.borrow().as_ref().unwrap()).clone(); __v });

        // Nested slices
    println!("{}", format!("{}", "\n=== Nested slices ===".to_string()));

        // Matrix (slice of slices)
    let mut matrix = Rc::new(RefCell::new(Some(vec![vec![1i64, 2i64, 3i64], vec![4i64, 5i64, 6i64], vec![7i64, 8i64, 9i64]])));

    println!("{}", format!("{}", "Matrix:".to_string()));
    { let __range_holder = matrix.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice().to_vec()).unwrap_or_default(); drop(__range_guard); for (i, row) in __range_values.iter().enumerate() {
        print!("Row {}: ", i);
        for (j, val) in row.as_slice().iter().copied().enumerate() {
        print!("{} ", val);
        if (j as i64) < ((row.len() as i64) - (1 as i64) as i64) {
        print!("{}", format!("{}", " ".to_string()));
    }
    }
        println!();
    } }

        // Access nested slice elements
    let mut centerElement = Rc::new(RefCell::new(Some(matrix.borrow().as_ref().unwrap().get_cloned((1) as usize).get_cloned((1) as usize))));
    print!("Center element: {}\n", { let __v = (*centerElement.borrow().as_ref().unwrap()).clone(); __v });

        // 3D slice
    let mut cube = Rc::new(RefCell::new(Some(vec![vec![vec![1i64, 2i64], vec![3i64, 4i64]], vec![vec![5i64, 6i64], vec![7i64, 8i64]]])));

    println!("{}", format!("{}", "\n3D Cube:".to_string()));
    { let __range_holder = cube.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice().to_vec()).unwrap_or_default(); drop(__range_guard); for (i, layer) in __range_values.iter().enumerate() {
        print!("Layer {}:\n", i);
        for (j, row) in layer.as_slice().iter().enumerate() {
        print!("  Row {}: ", j);
        for val in row.as_slice().iter().copied() {
        print!("{} ", val);
    }
        println!();
    }
    } }

        // Complex nested structure with interfaces
    println!("{}", format!("{}", "\n=== Complex nested with interfaces ===".to_string()));

    let mut canvas = Rc::new(RefCell::new(Some(Canvas { name: Rc::new(RefCell::new(Some("My Drawing".to_string()))), shapes: Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(Box::new(Circle { radius: Rc::new(RefCell::new(Some(5.0))), ..Default::default() }) as Box<dyn Drawable>))), Rc::new(RefCell::new(Some(Box::new(Rectangle { width: Rc::new(RefCell::new(Some(10.0))), height: Rc::new(RefCell::new(Some(8.0))), ..Default::default() }) as Box<dyn Drawable>))), Rc::new(RefCell::new(Some(Box::new(Circle { radius: Rc::new(RefCell::new(Some(3.0))), ..Default::default() }) as Box<dyn Drawable>)))]))), ..Default::default() })));

    print!("Canvas: {}\n", (*(*canvas.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone());
    { let __range_holder = (*canvas.borrow().as_ref().unwrap()).shapes.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice().to_vec()).unwrap_or_default(); drop(__range_guard); for (i, shape) in __range_values.iter().enumerate() {
        print!("Shape {}: {}\n", i as i64 + 1, (*(*shape.borrow().as_ref().unwrap()).draw().borrow().as_ref().unwrap()));
    } }

        // Modify nested structures
    println!("{}", format!("{}", "\n=== Modifying nested structures ===".to_string()));

        // Update employee contact
    { let new_val = "bob.new@company.com".to_string(); *(*(*company.borrow().as_ref().unwrap()).departments.borrow().as_ref().unwrap().get_cloned((0) as usize).employees.borrow().as_ref().unwrap().get_cloned((0) as usize).contact.borrow().as_ref().unwrap()).email.borrow_mut() = Some(new_val); };
    print!("Updated employee email: {}\n", (*(*(*company.borrow().as_ref().unwrap()).departments.borrow().as_ref().unwrap().get_cloned((0) as usize).employees.borrow().as_ref().unwrap().get_cloned((0) as usize).contact.borrow().as_ref().unwrap()).email.borrow().as_ref().unwrap()).clone());

        // Add new employee
    let mut newEmployee = Rc::new(RefCell::new(Some(Person { name: Rc::new(RefCell::new(Some("Dave Newbie".to_string()))), age: Rc::new(RefCell::new(Some(25))), address: Rc::new(RefCell::new(Some(Address { street: Rc::new(RefCell::new(Some("999 New St".to_string()))), city: Rc::new(RefCell::new(Some("Newtown".to_string()))), state: Rc::new(RefCell::new(Some("CA".to_string()))), zip_code: Rc::new(RefCell::new(Some("90214".to_string()))), country: Rc::new(RefCell::new(Some("USA".to_string()))), ..Default::default() }))), contact: Rc::new(RefCell::new(Some(Contact { email: Rc::new(RefCell::new(Some("dave@company.com".to_string()))), phone: Rc::new(RefCell::new(Some("555-0004".to_string()))), ..Default::default() }))), ..Default::default() })));

    { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = (*company.borrow().as_ref().unwrap()).departments.borrow().as_ref().unwrap().get_cloned((0) as usize).employees.clone(); let __append_value = (*newEmployee.borrow().as_ref().unwrap()).__go_value_clone(); let mut __appended: Vec<Person> = (*__append_target.borrow()).clone().unwrap_or_default(); __appended.push(__append_value); Rc::new(RefCell::new(Some(__appended))) })); *(*company.borrow().as_ref().unwrap()).departments.borrow().as_ref().unwrap().get_cloned((0) as usize).employees.borrow_mut() = __slice_header; };
    print!("Added new employee: {}\n", (*(*newEmployee.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone());
    print!("Total employees now: {}\n", ({ let __len_target = { let __field = (*company.borrow().as_ref().unwrap()).departments.borrow().as_ref().unwrap().get_cloned((0) as usize).employees.clone(); __field }; let __len_guard = __len_target.borrow(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }));
}