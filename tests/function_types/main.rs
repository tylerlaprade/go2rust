#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use std::cell::{RefCell};
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

/// Function type definitions
pub type BinaryOp = Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>>>>;


pub type UnaryOp = Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> i64>>>>;


pub type Predicate = Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> bool>>>>;


pub type StringProcessor = Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>>>>>;


/// Struct with function fields
#[derive(Clone, Default)]
pub struct Calculator {
    pub add: BinaryOp,
    pub subtract: BinaryOp,
    pub multiply: BinaryOp,
}

impl Calculator {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.add.clone();
        let __go_clone_1_0 = self.subtract.clone();
        let __go_clone_2_0 = self.multiply.clone();
        Self {
            add: __go_clone_0_0,
            subtract: __go_clone_1_0,
            multiply: __go_clone_2_0,
        }
    }
}

impl std::fmt::Display for Calculator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", "<func>");
        let __go_fmt_1 = format!("{}", "<func>");
        let __go_fmt_2 = format!("{}", "<func>");
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


/// Functions that match the types
pub fn add(a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) -> i64 {
    (*a.borrow().as_ref().unwrap()) + (*b.borrow().as_ref().unwrap())
}

pub fn multiply(a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) -> i64 {
    (*a.borrow().as_ref().unwrap()) * (*b.borrow().as_ref().unwrap())
}

pub fn square(x: Rc<RefCell<Option<i64>>>) -> i64 {
    { let __bin_x = (*x.borrow().as_ref().unwrap()).clone(); __bin_x * __bin_x }
}

pub fn is_even(x: Rc<RefCell<Option<i64>>>) -> bool {
    (*x.borrow().as_ref().unwrap()) % 2 == 0
}

pub fn to_upper(s: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {
    let mut result = Rc::new(RefCell::new(Some("".to_string())));
    for (_, char) in (*s.borrow().as_ref().unwrap()).char_indices() {
        if char >= 'a' && char <= 'z' {
        { (*result.borrow_mut().as_mut().unwrap()).push_str(&{ let __s = Rc::new(RefCell::new(Some(char::from_u32(((((char as i32) - 32))) as u32).unwrap().to_string()))); let __value = (*__s.borrow().as_ref().unwrap()).clone(); __value }); };
    } else {
        { (*result.borrow_mut().as_mut().unwrap()).push_str(&{ let __s = Rc::new(RefCell::new(Some(char::from_u32(((char as i32)) as u32).unwrap().to_string()))); let __value = (*__s.borrow().as_ref().unwrap()).clone(); __value }); };
    }
    }
    return { let __owned = result.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
}

/// Higher-order functions
pub fn apply_binary(op: BinaryOp, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) -> i64 {
    { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> = { let mut __f_guard = op.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(a.clone(), b.clone()) }
}

pub fn apply_unary(op: UnaryOp, x: Rc<RefCell<Option<i64>>>) -> i64 {
    { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> i64> = { let mut __f_guard = op.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> i64> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(x.clone()) }
}

pub fn filter(numbers: Rc<RefCell<Option<Vec<i64>>>>, pred: Predicate) -> Rc<RefCell<Option<Vec<i64>>>> {
    let mut result: Rc<RefCell<Option<Vec<i64>>>> = Rc::new(RefCell::new(None));
    { let __range_holder = numbers.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice().to_vec()).unwrap_or_default(); drop(__range_guard); for num in __range_values.iter().copied() {
        if { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> bool> = { let mut __f_guard = pred.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> bool> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(num)))) } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = result.clone(); let __append_value = num; let mut __appended: Vec<i64> = (*__append_target.borrow()).clone().unwrap_or_default(); __appended.push(__append_value); Rc::new(RefCell::new(Some(__appended))) })); *result.borrow_mut() = __slice_header; };
    }
    } }
    return result.clone();
}

pub fn transform(numbers: Rc<RefCell<Option<Vec<i64>>>>, op: UnaryOp) -> Rc<RefCell<Option<Vec<i64>>>> {
    let mut result = Rc::new(RefCell::new(Some(vec![0; ((*numbers.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize])));
    { let __range_holder = numbers.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice().to_vec()).unwrap_or_default(); drop(__range_guard); for (i, num) in __range_values.iter().copied().enumerate() {
        { let __idx = (i) as usize; let __rhs = { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> i64> = { let mut __f_guard = op.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> i64> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(num)))) }; result.borrow_mut().as_mut().unwrap().set(__idx, __rhs); };
    } }
    return result.clone();
}

pub fn process_string(s: Rc<RefCell<Option<String>>>, processor: StringProcessor) -> Rc<RefCell<Option<String>>> {
    { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>> = { let mut __f_guard = processor.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })))) }
}

/// Function that returns a function
pub fn make_multiplier(factor: Rc<RefCell<Option<i64>>>) -> UnaryOp {
    let factor_closure_clone = factor.clone(); return Rc::new(RefCell::new(Some(Box::new(move |x: Rc<RefCell<Option<i64>>>| -> i64 {
        (*x.borrow().as_ref().unwrap()) * (*factor_closure_clone.borrow().as_ref().unwrap())
    }) as Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> i64>)));
}

pub fn make_adder(addend: Rc<RefCell<Option<i64>>>) -> BinaryOp {
    let addend_closure_clone = addend.clone(); return Rc::new(RefCell::new(Some(Box::new(move |a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>| -> i64 {
        (*a.borrow().as_ref().unwrap()) + (*b.borrow().as_ref().unwrap()) + (*addend_closure_clone.borrow().as_ref().unwrap())
    }) as Box<dyn FnMut(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>)));
}

fn main() {
        // Basic function type usage
    println!("{}", format!("{}", "=== Basic function types ===".to_string()));

    let mut op: BinaryOp = Rc::new(RefCell::new(None));
    { let new_val = Box::new(move |__arg0: Rc<RefCell<Option<i64>>>, __arg1: Rc<RefCell<Option<i64>>>| -> i64 { add(__arg0, __arg1) }) as Box<dyn FnMut(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>; *op.borrow_mut() = Some(new_val); };
    print!("5 + 3 = {}\n", { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> = { let mut __f_guard = op.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(5))), Rc::new(RefCell::new(Some(3)))) });

    { let new_val = Box::new(move |__arg0: Rc<RefCell<Option<i64>>>, __arg1: Rc<RefCell<Option<i64>>>| -> i64 { multiply(__arg0, __arg1) }) as Box<dyn FnMut(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>; *op.borrow_mut() = Some(new_val); };
    print!("5 * 3 = {}\n", { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> = { let mut __f_guard = op.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(5))), Rc::new(RefCell::new(Some(3)))) });

        // Higher-order functions
    println!("{}", format!("{}", "\n=== Higher-order functions ===".to_string()));
    let mut result = apply_binary(Rc::new(RefCell::new(Some(Box::new(move |__arg0: Rc<RefCell<Option<i64>>>, __arg1: Rc<RefCell<Option<i64>>>| -> i64 { add(__arg0, __arg1) }) as Box<dyn FnMut(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>))), Rc::new(RefCell::new(Some(10))), Rc::new(RefCell::new(Some(20))));
    print!("applyBinary(add, 10, 20) = {}\n", result);

    { let new_val = apply_binary(Rc::new(RefCell::new(Some(Box::new(move |__arg0: Rc<RefCell<Option<i64>>>, __arg1: Rc<RefCell<Option<i64>>>| -> i64 { multiply(__arg0, __arg1) }) as Box<dyn FnMut(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>))), Rc::new(RefCell::new(Some(4))), Rc::new(RefCell::new(Some(7)))); result = new_val; };
    print!("applyBinary(multiply, 4, 7) = {}\n", result);

    let mut unaryResult = apply_unary(Rc::new(RefCell::new(Some(Box::new(move |__arg0: Rc<RefCell<Option<i64>>>| -> i64 { square(__arg0) }) as Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> i64>))), Rc::new(RefCell::new(Some(6))));
    print!("applyUnary(square, 6) = {}\n", unaryResult);

        // Function slices and filtering
    println!("{}", format!("{}", "\n=== Function slices and filtering ===".to_string()));
    let mut numbers = Rc::new(RefCell::new(Some(vec![1i64, 2i64, 3i64, 4i64, 5i64, 6i64, 7i64, 8i64, 9i64, 10i64])));

    let mut evens = filter(numbers.clone(), Rc::new(RefCell::new(Some(Box::new(move |__arg0: Rc<RefCell<Option<i64>>>| -> bool { is_even(__arg0) }) as Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> bool>))));
    let mut evens = { let __slice_header = GoSliceHeader::go_slice_header(&evens); Rc::new(RefCell::new(__slice_header)) };
    print!("Even numbers: {}\n", format_slice(&evens));

    let mut odds = filter(numbers.clone(), Rc::new(RefCell::new(Some(Box::new(move |x: Rc<RefCell<Option<i64>>>| -> bool {
        (*x.borrow().as_ref().unwrap()) % 2 != 0
    }) as Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> bool>))));
    let mut odds = { let __slice_header = GoSliceHeader::go_slice_header(&odds); Rc::new(RefCell::new(__slice_header)) };
    print!("Odd numbers: {}\n", format_slice(&odds));

        // Transform with function types
    println!("{}", format!("{}", "\n=== Transform operations ===".to_string()));
    let mut squared = transform(Rc::new(RefCell::new(Some(vec![1i64, 2i64, 3i64, 4i64, 5i64]))), Rc::new(RefCell::new(Some(Box::new(move |__arg0: Rc<RefCell<Option<i64>>>| -> i64 { square(__arg0) }) as Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> i64>))));
    let mut squared = { let __slice_header = GoSliceHeader::go_slice_header(&squared); Rc::new(RefCell::new(__slice_header)) };
    print!("Squared: {}\n", format_slice(&squared));

    let mut doubled = transform(Rc::new(RefCell::new(Some(vec![1i64, 2i64, 3i64, 4i64, 5i64]))), Rc::new(RefCell::new(Some(Box::new(move |x: Rc<RefCell<Option<i64>>>| -> i64 {
        (*x.borrow().as_ref().unwrap()) * 2
    }) as Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> i64>))));
    let mut doubled = { let __slice_header = GoSliceHeader::go_slice_header(&doubled); Rc::new(RefCell::new(__slice_header)) };
    print!("Doubled: {}\n", format_slice(&doubled));

        // String processing
    println!("{}", format!("{}", "\n=== String processing ===".to_string()));
    let mut text = Rc::new(RefCell::new(Some("hello world".to_string())));
    let mut upper = process_string(Rc::new(RefCell::new(Some({ let __arg_holder = text.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some(Box::new(move |__arg0: Rc<RefCell<Option<String>>>| -> Rc<RefCell<Option<String>>> { to_upper(__arg0) }) as Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>>))));
    print!("'{}' -> '{}'\n", { let __v = (*text.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*upper.borrow().as_ref().unwrap()).clone(); __v });

    let mut reversed = process_string(Rc::new(RefCell::new(Some("hello".to_string()))), Rc::new(RefCell::new(Some(Box::new(move |s: Rc<RefCell<Option<String>>>| -> Rc<RefCell<Option<String>>> {
        let mut runes = Rc::new(RefCell::new(Some(((*s.borrow().as_ref().unwrap())).chars().map(|c| c as i32).collect::<Vec<_>>())));
        let (mut i, mut j) = (Rc::new(RefCell::new(Some(0i64))), Rc::new(RefCell::new(Some(((*runes.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i64) - (1 as i64)))));
    while (*i.borrow().as_ref().unwrap()) < (*j.borrow().as_ref().unwrap()) {
        {
            let __go_lhs_sequence_0 = runes.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = runes.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = runes.borrow().as_ref().unwrap().get_cloned(((*j.borrow().as_ref().unwrap())) as usize);
            let __tmp_1 = runes.borrow().as_ref().unwrap().get_cloned(((*i.borrow().as_ref().unwrap())) as usize);
            __go_lhs_sequence_0.borrow_mut().as_mut().unwrap().set(__go_lhs_index_0, __tmp_0);
            __go_lhs_sequence_1.borrow_mut().as_mut().unwrap().set(__go_lhs_index_1, __tmp_1);
        };
        {
            let __tmp_0 = (*i.borrow().as_ref().unwrap()) + 1;
            let __tmp_1 = (*j.borrow().as_ref().unwrap()) - 1;
            *i.borrow_mut() = Some(__tmp_0);
            *j.borrow_mut() = Some(__tmp_1);
        };
    }
        return Rc::new(RefCell::new(Some((*runes.borrow().as_ref().unwrap()).as_slice().iter().map(|&c| char::from_u32(c as u32).unwrap()).collect::<String>())));
    }) as Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>>))));
    print!("Reversed: {}\n", { let __v = (*reversed.borrow().as_ref().unwrap()).clone(); __v });

        // Functions that return functions
    println!("{}", format!("{}", "\n=== Functions returning functions ===".to_string()));
    let mut triple = make_multiplier(Rc::new(RefCell::new(Some(3))));
    print!("triple(4) = {}\n", { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> i64> = { let mut __f_guard = triple.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> i64> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(4)))) });

    let mut addTen = make_adder(Rc::new(RefCell::new(Some(10))));
    print!("addTen(5, 3) = {}\n", { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> = { let mut __f_guard = addTen.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(5))), Rc::new(RefCell::new(Some(3)))) });

        // Struct with function fields
    println!("{}", format!("{}", "\n=== Struct with function fields ===".to_string()));
    let mut calc = Rc::new(RefCell::new(Some(Calculator { add: Rc::new(RefCell::new(Some(Box::new(move |a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>| -> i64 {
        (*a.borrow().as_ref().unwrap()) + (*b.borrow().as_ref().unwrap())
    }) as Box<dyn FnMut(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>))), subtract: Rc::new(RefCell::new(Some(Box::new(move |a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>| -> i64 {
        (*a.borrow().as_ref().unwrap()) - (*b.borrow().as_ref().unwrap())
    }) as Box<dyn FnMut(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>))), multiply: Rc::new(RefCell::new(Some(Box::new(move |__arg0: Rc<RefCell<Option<i64>>>, __arg1: Rc<RefCell<Option<i64>>>| -> i64 { multiply(__arg0, __arg1) }) as Box<dyn FnMut(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>))), ..Default::default() })));

        // Reuse existing function
    print!("calc.Add(10, 5) = {}\n", { let __f_holder = (*calc.borrow().as_ref().unwrap()).add.clone(); let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(10))), Rc::new(RefCell::new(Some(5)))) });
    print!("calc.Subtract(10, 5) = {}\n", { let __f_holder = (*calc.borrow().as_ref().unwrap()).subtract.clone(); let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(10))), Rc::new(RefCell::new(Some(5)))) });
    print!("calc.Multiply(10, 5) = {}\n", { let __f_holder = (*calc.borrow().as_ref().unwrap()).multiply.clone(); let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(10))), Rc::new(RefCell::new(Some(5)))) });

        // Function variables
    println!("{}", format!("{}", "\n=== Function variables ===".to_string()));
    let mut processor: StringProcessor = Rc::new(RefCell::new(None));
    { let new_val = Box::new(move |__arg0: Rc<RefCell<Option<String>>>| -> Rc<RefCell<Option<String>>> { to_upper(__arg0) }) as Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>>; *processor.borrow_mut() = Some(new_val); };
    print!("Using toUpper: {}\n", (*{ let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>> = { let mut __f_guard = processor.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some("test".to_string())))) }.borrow().as_ref().unwrap()));

    { let new_val = Box::new(move |s: Rc<RefCell<Option<String>>>| -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("{}{}", "processed: ".to_string(), (*s.borrow().as_ref().unwrap())))))
    }) as Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>>; *processor.borrow_mut() = Some(new_val); };
    print!("Using anonymous: {}\n", (*{ let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>> = { let mut __f_guard = processor.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some("test".to_string())))) }.borrow().as_ref().unwrap()));
}