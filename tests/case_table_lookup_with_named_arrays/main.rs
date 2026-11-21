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

#[derive(Clone)]
pub struct GoSliceElemPtr<T> {
    slice: Rc<RefCell<Option<Vec<T>>>>,
    index: usize,
}

pub struct GoSliceElemRef<T: Clone> {
    value: Option<T>,
}

pub struct GoSliceElemMutRef<T: Clone> {
    slice: Rc<RefCell<Option<Vec<T>>>>,
    index: usize,
    value: Option<T>,
}

pub trait GoArrayElemBacking<T: Clone + 'static, const N: usize> {
    fn borrow_at(&self, index: usize) -> Option<T>;
    fn assign_at(&self, index: usize, value: Option<T>);
    fn identity_at(&self, index: usize) -> (*const (), usize);
}

#[derive(Clone)]
pub struct GoDirectArrayElemBacking<T: Clone + 'static, const N: usize> {
    array: Rc<RefCell<Option<[T; N]>>>,
}

impl<T: Clone + 'static, const N: usize> GoArrayElemBacking<T, N> for GoDirectArrayElemBacking<T, N> {
    fn borrow_at(&self, index: usize) -> Option<T> {
        let guard = self.array.borrow();
        guard.as_ref().and_then(|values| values.get(index).cloned())
    }

    fn assign_at(&self, index: usize, value: Option<T>) {
        if let Some(value) = value {
            if let Some(values) = self.array.borrow_mut().as_mut() {
                values[index] = value;
            }
        }
    }

    fn identity_at(&self, index: usize) -> (*const (), usize) {
        (Rc::as_ptr(&self.array) as *const (), index)
    }
}

#[derive(Clone)]
pub struct GoNestedArrayElemBacking<T: Clone + 'static, const N: usize, const OUT: usize> {
    outer: Rc<RefCell<Option<[[T; N]; OUT]>>>,
    outer_index: usize,
}

impl<T: Clone + 'static, const N: usize, const OUT: usize> GoArrayElemBacking<T, N> for GoNestedArrayElemBacking<T, N, OUT> {
    fn borrow_at(&self, index: usize) -> Option<T> {
        let guard = self.outer.borrow();
        guard.as_ref().and_then(|values| values.get(self.outer_index)).and_then(|inner| inner.get(index)).cloned()
    }

    fn assign_at(&self, index: usize, value: Option<T>) {
        if let Some(value) = value {
            if let Some(values) = self.outer.borrow_mut().as_mut() {
                values[self.outer_index][index] = value;
            }
        }
    }

    fn identity_at(&self, index: usize) -> (*const (), usize) {
        (Rc::as_ptr(&self.outer) as *const (), self.outer_index.wrapping_mul(N).wrapping_add(index))
    }
}

#[derive(Clone)]
pub struct GoArrayElemFromElemBacking<T: Clone + 'static, const N: usize, const OUT: usize> {
    parent: GoArrayElemPtr<[T; N], OUT>,
}

impl<T: Clone + 'static, const N: usize, const OUT: usize> GoArrayElemBacking<T, N> for GoArrayElemFromElemBacking<T, N, OUT> {
    fn borrow_at(&self, index: usize) -> Option<T> {
        let inner = self.parent.borrow();
        inner.as_ref().and_then(|values| values.get(index).cloned())
    }

    fn assign_at(&self, index: usize, value: Option<T>) {
        if let Some(value) = value {
            let mut inner = self.parent.borrow_mut();
            if let Some(values) = inner.as_mut() {
                values[index] = value;
            }
        }
    }

    fn identity_at(&self, index: usize) -> (*const (), usize) {
        let (base, outer_index) = self.parent.identity();
        (base, outer_index.wrapping_mul(N).wrapping_add(index))
    }
}

#[derive(Clone)]
pub struct GoArrayElemFromGoPtrBacking<T: Clone + 'static, const N: usize> {
    parent: GoPtr<[T; N]>,
}

impl<T: Clone + 'static, const N: usize> GoArrayElemBacking<T, N> for GoArrayElemFromGoPtrBacking<T, N> {
    fn borrow_at(&self, index: usize) -> Option<T> {
        let inner = self.parent.borrow();
        inner.as_ref().and_then(|values| values.get(index).cloned())
    }

    fn assign_at(&self, index: usize, value: Option<T>) {
        if let Some(value) = value {
            self.parent.with_mut(|values| {
                values[index] = value;
            });
        }
    }

    fn identity_at(&self, index: usize) -> (*const (), usize) {
        (self.parent.addr() as *const (), index)
    }
}

#[derive(Clone)]
pub struct GoArrayElemPtr<T: Clone + 'static, const N: usize> {
    backing: Rc<dyn GoArrayElemBacking<T, N>>,
    index: usize,
}

pub struct GoArrayElemRef<T: Clone> {
    value: Option<T>,
}

pub struct GoArrayElemMutRef<T: Clone + 'static, const N: usize> {
    backing: Rc<dyn GoArrayElemBacking<T, N>>,
    index: usize,
    value: Option<T>,
}

impl<T> GoSliceElemPtr<T> {
    pub fn new(slice: Rc<RefCell<Option<Vec<T>>>>, index: usize) -> Self {
        GoSliceElemPtr { slice, index }
    }

    pub fn slice_handle(&self) -> Rc<RefCell<Option<Vec<T>>>> {
        self.slice.clone()
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

impl<T: Clone> GoSliceElemPtr<T> {
    pub fn borrow(&self) -> GoSliceElemRef<T> {
        let guard = self.slice.borrow();
        GoSliceElemRef {
            value: guard.as_ref().map(|values| values.get_cloned(self.index)),
        }
    }

    pub fn borrow_mut(&self) -> GoSliceElemMutRef<T> {
        let guard = self.slice.borrow();
        GoSliceElemMutRef {
            slice: self.slice.clone(),
            index: self.index,
            value: guard.as_ref().map(|values| values.get_cloned(self.index)),
        }
    }
}

impl<T: Clone + 'static, const N: usize> GoArrayElemPtr<T, N> {
    pub fn new(array: Rc<RefCell<Option<[T; N]>>>, index: usize) -> Self {
        GoArrayElemPtr {
            backing: Rc::new(GoDirectArrayElemBacking { array }),
            index,
        }
    }

    pub fn nested<const OUT: usize>(outer: Rc<RefCell<Option<[[T; N]; OUT]>>>, outer_index: usize, index: usize) -> Self {
        GoArrayElemPtr {
            backing: Rc::new(GoNestedArrayElemBacking { outer, outer_index }),
            index,
        }
    }

    pub fn from_array_elem<const OUT: usize>(parent: GoArrayElemPtr<[T; N], OUT>, index: usize) -> Self {
        GoArrayElemPtr {
            backing: Rc::new(GoArrayElemFromElemBacking { parent }),
            index,
        }
    }

    pub fn from_go_ptr(parent: GoPtr<[T; N]>, index: usize) -> Self {
        GoArrayElemPtr {
            backing: Rc::new(GoArrayElemFromGoPtrBacking { parent }),
            index,
        }
    }

    pub fn borrow(&self) -> GoArrayElemRef<T> {
        GoArrayElemRef {
            value: self.backing.borrow_at(self.index),
        }
    }

    pub fn borrow_mut(&self) -> GoArrayElemMutRef<T, N> {
        GoArrayElemMutRef {
            backing: self.backing.clone(),
            index: self.index,
            value: self.backing.borrow_at(self.index),
        }
    }

    pub fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut value = self.backing.borrow_at(self.index).expect("nil pointer dereference");
        let result = f(&mut value);
        self.backing.assign_at(self.index, Some(value));
        result
    }

    pub fn identity(&self) -> (*const (), usize) {
        self.backing.identity_at(self.index)
    }
}

impl<T: Clone> std::ops::Deref for GoSliceElemRef<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Clone> std::ops::Deref for GoSliceElemMutRef<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Clone> std::ops::DerefMut for GoSliceElemMutRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T: Clone> Drop for GoSliceElemMutRef<T> {
    fn drop(&mut self) {
        if let Some(value) = self.value.clone() {
            if let Some(values) = self.slice.borrow_mut().as_mut() {
                values.set(self.index, value);
            }
        }
    }
}

impl<T: Clone> std::ops::Deref for GoArrayElemRef<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Clone + 'static, const N: usize> std::ops::Deref for GoArrayElemMutRef<T, N> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Clone + 'static, const N: usize> std::ops::DerefMut for GoArrayElemMutRef<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T: Clone + 'static, const N: usize> Drop for GoArrayElemMutRef<T, N> {
    fn drop(&mut self) {
        self.backing.assign_at(self.index, self.value.clone());
    }
}

pub trait GoArrayElemPtrDyn<T: 'static> {
    fn borrow_dyn(&self) -> Option<T>;
    fn assign_dyn(&self, value: Option<T>);
    fn with_mut_dyn(&self, f: &mut dyn FnMut(&mut T));
    fn identity_dyn(&self) -> (*const (), usize);
}

pub struct GoForeignArrayElemPtrDyn<T: 'static> {
    borrow: Rc<dyn Fn() -> Option<T>>,
    assign: Rc<dyn Fn(Option<T>)>,
    with_mut: Rc<dyn Fn(&mut dyn FnMut(&mut T))>,
    identity: Rc<dyn Fn() -> (*const (), usize)>,
}

impl<T: 'static> GoArrayElemPtrDyn<T> for GoForeignArrayElemPtrDyn<T> {
    fn borrow_dyn(&self) -> Option<T> {
        (self.borrow)()
    }

    fn assign_dyn(&self, value: Option<T>) {
        (self.assign)(value)
    }

    fn with_mut_dyn(&self, f: &mut dyn FnMut(&mut T)) {
        (self.with_mut)(f)
    }

    fn identity_dyn(&self) -> (*const (), usize) {
        (self.identity)()
    }
}

impl<T: Clone + 'static, const N: usize> GoArrayElemPtrDyn<T> for GoArrayElemPtr<T, N> {
    fn borrow_dyn(&self) -> Option<T> {
        (*self.borrow()).clone()
    }

    fn assign_dyn(&self, value: Option<T>) {
        *self.borrow_mut() = value;
    }

    fn with_mut_dyn(&self, f: &mut dyn FnMut(&mut T)) {
        self.with_mut(|value| f(value));
    }

    fn identity_dyn(&self) -> (*const (), usize) {
        self.identity()
    }
}

pub enum GoPtr<T: 'static> {
    Nil,
    Raw(usize),
    Local(Rc<RefCell<Option<T>>>),
    SliceElem(GoSliceElemPtr<T>),
    ArrayElem(Rc<dyn GoArrayElemPtrDyn<T>>),
}

impl<T: 'static> Clone for GoPtr<T> {
    fn clone(&self) -> Self {
        match self {
            GoPtr::Nil => GoPtr::Nil,
            GoPtr::Raw(addr) => GoPtr::Raw(*addr),
            GoPtr::Local(value) => GoPtr::Local(value.clone()),
            GoPtr::SliceElem(value) => GoPtr::SliceElem(GoSliceElemPtr { slice: value.slice.clone(), index: value.index }),
            GoPtr::ArrayElem(value) => GoPtr::ArrayElem(value.clone()),
        }
    }
}

impl<T: 'static> GoPtr<T> {
    pub fn nil() -> Self {
        GoPtr::Nil
    }

    pub fn raw(addr: usize) -> Self {
        if addr == 0 {
            GoPtr::Nil
        } else {
            GoPtr::Raw(addr)
        }
    }

    pub fn local(value: Rc<RefCell<Option<T>>>) -> Self {
        if value.borrow().is_none() {
            GoPtr::Nil
        } else {
            GoPtr::Local(value)
        }
    }

    pub fn slice_elem(value: GoSliceElemPtr<T>) -> Self {
        GoPtr::SliceElem(value)
    }

    pub fn slice_elem_opt(value: Option<GoSliceElemPtr<T>>) -> Self {
        match value {
            Some(value) => GoPtr::SliceElem(value),
            None => GoPtr::Nil,
        }
    }

    pub fn array_elem<const N: usize>(value: GoArrayElemPtr<T, N>) -> Self
    where
        T: Clone,
    {
        GoPtr::ArrayElem(Rc::new(value))
    }

    pub fn array_elem_opt<const N: usize>(value: Option<GoArrayElemPtr<T, N>>) -> Self
    where
        T: Clone,
    {
        match value {
            Some(value) => GoPtr::ArrayElem(Rc::new(value)),
            None => GoPtr::Nil,
        }
    }

    pub fn array_elem_foreign(
        borrow: Rc<dyn Fn() -> Option<T>>,
        assign: Rc<dyn Fn(Option<T>)>,
        with_mut: Rc<dyn Fn(&mut dyn FnMut(&mut T))>,
        identity: Rc<dyn Fn() -> (*const (), usize)>,
    ) -> Self {
        GoPtr::ArrayElem(Rc::new(GoForeignArrayElemPtrDyn {
            borrow,
            assign,
            with_mut,
            identity,
        }))
    }

    pub fn is_nil(&self) -> bool {
        match self {
            GoPtr::Nil => true,
            GoPtr::Raw(addr) => *addr == 0,
            GoPtr::Local(value) => value.borrow().is_none(),
            GoPtr::SliceElem(value) => {
                let guard = value.slice.borrow();
                guard.as_ref().map(|values| value.index >= values.len()).unwrap_or(true)
            }
            GoPtr::ArrayElem(value) => value.identity_dyn().0.is_null(),
        }
    }

    pub fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        match self {
            GoPtr::Nil => panic!("nil pointer dereference"),
            GoPtr::Raw(_) => panic!("raw unsafe pointer mutable borrow requires unsafe pointee support"),
            GoPtr::Local(slot) => {
                let mut guard = slot.borrow_mut();
                f(guard.as_mut().unwrap())
            }
            GoPtr::SliceElem(slot) => {
                let mut guard = slot.slice.borrow_mut();
                let values = guard.as_mut().expect("nil pointer dereference");
                values.with_mut(slot.index, f)
            }
            GoPtr::ArrayElem(slot) => {
                let mut result = None;
                let mut callback = Some(f);
                slot.with_mut_dyn(&mut |value| {
                    let f = callback.take().expect("array element pointer mutable borrow called twice");
                    result = Some(f(value));
                });
                result.expect("nil pointer dereference")
            }
        }
    }

    pub fn ptr_eq(left: &Self, right: &Self) -> bool {
        match (left, right) {
            (GoPtr::Nil, GoPtr::Nil) => true,
            (GoPtr::Raw(_), _) | (_, GoPtr::Raw(_)) => left.addr() == right.addr(),
            (GoPtr::Local(left), GoPtr::Local(right)) => Rc::ptr_eq(left, right),
            (GoPtr::SliceElem(left), GoPtr::SliceElem(right)) => {
                Rc::ptr_eq(&left.slice_handle(), &right.slice_handle()) && left.index() == right.index()
            }
            (GoPtr::ArrayElem(left), GoPtr::ArrayElem(right)) => left.identity_dyn() == right.identity_dyn(),
            _ => false,
        }
    }

    pub fn addr(&self) -> usize {
        match self {
            GoPtr::Nil => 0,
            GoPtr::Raw(addr) => *addr,
            GoPtr::Local(value) => Rc::as_ptr(value) as usize,
            GoPtr::SliceElem(value) => (Rc::as_ptr(&value.slice_handle()) as usize).wrapping_add(value.index()),
            GoPtr::ArrayElem(value) => {
                let (base, index) = value.identity_dyn();
                (base as usize).wrapping_add(index)
            }
        }
    }
}

impl<T: Clone + 'static> GoPtr<T> {
    pub fn borrow(&self) -> Option<T> {
        match self {
            GoPtr::Nil => None,
            GoPtr::Raw(_) => panic!("raw unsafe pointer dereference requires unsafe pointee support"),
            GoPtr::Local(value) => (*value.borrow()).clone(),
            GoPtr::SliceElem(value) => (*value.borrow()).clone(),
            GoPtr::ArrayElem(value) => value.borrow_dyn(),
        }
    }

    pub fn assign(&self, value: Option<T>) {
        match self {
            GoPtr::Nil => panic!("nil pointer dereference"),
            GoPtr::Raw(_) => panic!("raw unsafe pointer assignment requires unsafe pointee support"),
            GoPtr::Local(slot) => *slot.borrow_mut() = value,
            GoPtr::SliceElem(slot) => *slot.borrow_mut() = value,
            GoPtr::ArrayElem(slot) => slot.assign_dyn(value),
        }
    }
}

impl<T: 'static> Default for GoPtr<T> {
    fn default() -> Self {
        GoPtr::Nil
    }
}

impl<T: 'static> std::fmt::Debug for GoPtr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_nil() {
            write!(f, "<nil>")
        } else {
            write!(f, "<ptr>")
        }
    }
}

pub const MAX_CASE: i64 = 3;


pub const MAX_RUNE: i32 = ('\u{10ffff}' as i32);


pub const UPPER_LOWER: i32 = MAX_RUNE + 1;


#[derive(Debug, Clone)]
pub struct d(pub Rc<RefCell<Option<[i32; 3]>>>);

impl Default for d {
    fn default() -> Self {
        d(Rc::new(RefCell::new(Some(std::array::from_fn(|_| 0)))))
    }
}

impl d {
    pub fn __go_value_clone(&self) -> Self {
        d(Rc::new(RefCell::new(self.0.borrow().as_ref().map(|__go_array| __go_array.clone()))))
    }
}

impl PartialEq for d {
    fn eq(&self, other: &Self) -> bool {
        let __left = self.0.borrow().clone();
        let __right = other.0.borrow().clone();
        __left == __right
    }
}

impl Display for d {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice(&self.0))
    }
}


#[derive(Debug, Clone)]
pub struct CaseRange {
    pub lo: Rc<RefCell<Option<u32>>>,
    pub hi: Rc<RefCell<Option<u32>>>,
    pub delta: Rc<RefCell<Option<d>>>,
}

impl CaseRange {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.lo.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.hi.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.delta.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        Self {
            lo: __go_clone_0_0,
            hi: __go_clone_1_0,
            delta: __go_clone_2_0,
        }
    }
}


impl Default for CaseRange {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_2_0 = Rc::new(RefCell::new(Some(d(Rc::new(RefCell::new(Some(std::array::from_fn(|_| 0))))))));
        Self {
            lo: __go_default_0_0,
            hi: __go_default_1_0,
            delta: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for CaseRange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.lo.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.hi.borrow().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.delta.borrow().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


pub(crate) struct GoGlobal<T> {
    value: std::cell::UnsafeCell<Option<T>>,
}
unsafe impl<T> Sync for GoGlobal<T> {}
impl<T> GoGlobal<T> {
    pub(crate) const fn new() -> Self {
        Self { value: std::cell::UnsafeCell::new(None) }
    }
    pub(crate) fn borrow(&'static self) -> &'static Option<T> {
        unsafe { &*self.value.get() }
    }
    pub(crate) fn borrow_mut(&'static self) -> &'static mut Option<T> {
        unsafe { &mut *self.value.get() }
    }
    pub(crate) fn clone(&'static self) -> std::rc::Rc<std::cell::RefCell<Option<T>>> where T: Clone {
        std::rc::Rc::new(std::cell::RefCell::new(self.borrow().clone()))
    }
}

pub(crate) static _CaseRanges: GoGlobal<Vec<CaseRange>> = GoGlobal::new();

pub static CaseRanges: GoGlobal<Vec<CaseRange>> = GoGlobal::new();


fn __go_init_globals() {
    *_CaseRanges.borrow_mut() = Some(vec![]);
    *CaseRanges.borrow_mut() = Some(vec![]);
    {
        let mut __go_slice = Vec::<CaseRange>::with_capacity(6);
        __go_slice.push(CaseRange { lo: Rc::new(RefCell::new(Some(0x0041 as u32))), hi: Rc::new(RefCell::new(Some(0x005A as u32))), delta: Rc::new(RefCell::new(Some(d(Rc::new(RefCell::new(Some([0 as i32, 32 as i32, 0 as i32]))))))), ..Default::default() });
        __go_slice.push(CaseRange { lo: Rc::new(RefCell::new(Some(0x0061 as u32))), hi: Rc::new(RefCell::new(Some(0x007A as u32))), delta: Rc::new(RefCell::new(Some(d(Rc::new(RefCell::new(Some([-32 as i32, 0 as i32, -32 as i32]))))))), ..Default::default() });
        __go_slice.push(CaseRange { lo: Rc::new(RefCell::new(Some(0x00C0 as u32))), hi: Rc::new(RefCell::new(Some(0x00D6 as u32))), delta: Rc::new(RefCell::new(Some(d(Rc::new(RefCell::new(Some([0 as i32, 32 as i32, 0 as i32]))))))), ..Default::default() });
        __go_slice.push(CaseRange { lo: Rc::new(RefCell::new(Some(0x00D8 as u32))), hi: Rc::new(RefCell::new(Some(0x00DE as u32))), delta: Rc::new(RefCell::new(Some(d(Rc::new(RefCell::new(Some([0 as i32, 32 as i32, 0 as i32]))))))), ..Default::default() });
        __go_slice.push(CaseRange { lo: Rc::new(RefCell::new(Some(0x00E0 as u32))), hi: Rc::new(RefCell::new(Some(0x00F6 as u32))), delta: Rc::new(RefCell::new(Some(d(Rc::new(RefCell::new(Some([-32 as i32, 0 as i32, -32 as i32]))))))), ..Default::default() });
        __go_slice.push(CaseRange { lo: Rc::new(RefCell::new(Some(0x0100 as u32))), hi: Rc::new(RefCell::new(Some(0x012F as u32))), delta: Rc::new(RefCell::new(Some(d(Rc::new(RefCell::new(Some([UPPER_LOWER as i32, UPPER_LOWER as i32, UPPER_LOWER as i32]))))))), ..Default::default() });
        let __go_slice = Vec::from(__go_slice.into_boxed_slice().into_vec());
        *_CaseRanges.borrow_mut() = Some(__go_slice);
    }
    *CaseRanges.borrow_mut() = Some((*_CaseRanges.borrow().as_ref().unwrap()).clone());
}


pub fn lookup_case_range(r: Rc<RefCell<Option<i32>>>, caseRange: Rc<RefCell<Option<Vec<CaseRange>>>>) -> Option<GoSliceElemPtr<CaseRange>> {
    let mut lo = Rc::new(RefCell::new(Some(0i64)));
    let mut hi = Rc::new(RefCell::new(Some((*caseRange.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i64)));
    while (*lo.borrow().as_ref().unwrap()) < (*hi.borrow().as_ref().unwrap()) {
        let mut m = Rc::new(RefCell::new(Some((((*Rc::new(RefCell::new(Some((((*lo.borrow().as_ref().unwrap()) + (*hi.borrow().as_ref().unwrap()))) as u64))).borrow().as_ref().unwrap()) >> 1)) as i64)));
        let mut cr: Option<GoSliceElemPtr<CaseRange>> = Some(GoSliceElemPtr::new(caseRange.clone(), ((*m.borrow().as_ref().unwrap())) as usize));
        if (*Rc::new(RefCell::new(Some({ let __selector_holder = (*cr.as_ref().unwrap().borrow().as_ref().unwrap()).lo.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))).borrow().as_ref().unwrap()) <= (*r.borrow().as_ref().unwrap()) && (*r.borrow().as_ref().unwrap()) <= (*Rc::new(RefCell::new(Some({ let __selector_holder = (*cr.as_ref().unwrap().borrow().as_ref().unwrap()).hi.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))).borrow().as_ref().unwrap()) {
        return cr.clone();
    }
        if (*r.borrow().as_ref().unwrap()) < (*Rc::new(RefCell::new(Some({ let __selector_holder = (*cr.as_ref().unwrap().borrow().as_ref().unwrap()).lo.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))).borrow().as_ref().unwrap()) {
        { let new_val = m.borrow().as_ref().unwrap().clone(); *hi.borrow_mut() = Some(new_val); };
    } else {
        { let new_val = (*m.borrow().as_ref().unwrap()) + 1; *lo.borrow_mut() = Some(new_val); };
    }
    }
    None
}

pub fn convert_case(_case: Rc<RefCell<Option<i64>>>, r: Rc<RefCell<Option<i32>>>, cr: GoPtr<CaseRange>) -> i32 {
    let mut delta = Rc::new(RefCell::new(Some({ let __seq_holder = { let __named_array = (*{ let __ptr_value = cr.with_mut(|__ptr_value| __ptr_value.delta.clone()); __ptr_value }.borrow().as_ref().unwrap()).0.clone(); __named_array }; let __seq_index = ((*_case.borrow().as_ref().unwrap())) as usize; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() })));
    if (*delta.borrow().as_ref().unwrap()) > MAX_RUNE as i32 {
        return (*Rc::new(RefCell::new(Some({ let __selector_holder = { let __ptr_value = cr.with_mut(|__ptr_value| __ptr_value.lo.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))).borrow().as_ref().unwrap()) + (((*r.borrow().as_ref().unwrap()) - (*Rc::new(RefCell::new(Some({ let __selector_holder = { let __ptr_value = cr.with_mut(|__ptr_value| __ptr_value.lo.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))).borrow().as_ref().unwrap())) & ! 1 as i32 | (*Rc::new(RefCell::new(Some((((*_case.borrow().as_ref().unwrap()) & 1)) as i32))).borrow().as_ref().unwrap()));
    }
    return (*r.borrow().as_ref().unwrap()) + (*delta.borrow().as_ref().unwrap());
}

pub fn to(_case: Rc<RefCell<Option<i64>>>, r: Rc<RefCell<Option<i32>>>, caseRange: Rc<RefCell<Option<Vec<CaseRange>>>>) -> (i32, bool) {
    let mut mappedRune: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(Default::default())));
    let mut foundMapping: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));

    if (*_case.borrow().as_ref().unwrap()) < 0 || 3 <= (*_case.borrow().as_ref().unwrap()) {
        return (0xFFFD, false);
    }
    {
        let mut cr: Option<GoSliceElemPtr<CaseRange>> = lookup_case_range(Rc::new(RefCell::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), caseRange.clone());;
        if cr.is_some() {
            return (convert_case(Rc::new(RefCell::new(Some({ let __arg_holder = _case.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), GoPtr::slice_elem_opt(cr.clone())), true);;
        }
    }
    ((*r.borrow().as_ref().unwrap()), false)
}

fn main() {
    __go_init_all();
    for r in vec![('a' as i32), ('Z' as i32), ('\u{f6}' as i32), ('\u{d6}' as i32), ('\u{101}' as i32), ('\u{100}' as i32), ('!' as i32)].as_slice().iter().copied() {
        let (mut u, mut ok) = to(Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(r.clone()))), CaseRanges.clone());
        let (mut l, _) = to(Rc::new(RefCell::new(Some(1))), Rc::new(RefCell::new(Some(r.clone()))), CaseRanges.clone());
        let mut cr: Option<GoSliceElemPtr<CaseRange>> = lookup_case_range(Rc::new(RefCell::new(Some(r.clone()))), CaseRanges.clone());
        println!("{} {} {} {} {}", format!("{}", (*Rc::new(RefCell::new(Some(char::from_u32((r) as u32).unwrap().to_string()))).borrow().as_ref().unwrap())), format!("{}", (*Rc::new(RefCell::new(Some(char::from_u32((u) as u32).unwrap().to_string()))).borrow().as_ref().unwrap())), format!("{}", (*Rc::new(RefCell::new(Some(char::from_u32((l) as u32).unwrap().to_string()))).borrow().as_ref().unwrap())), format!("{}", ok), format!("{}", cr.is_some()));
    }
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
