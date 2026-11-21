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

fn format_map<K: Display + Ord + Clone, V>(map: &Rc<RefCell<Option<BTreeMap<K, Rc<RefCell<Option<V>>>>>>>) -> String
where
    V: Display,
{
    let guard = map.borrow();
    if let Some(ref m) = *guard {
        let mut items: std::vec::Vec<_> = m.iter().collect();
        items.sort_by_key(|(k, _)| (*k).clone());

        let formatted: std::vec::Vec<String> = items
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

fn format_any(value: &dyn Any) -> String {
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
    } else {
        panic!("go2rust: cannot print this value held in an interface: its type has no formatting case yet")
    }
}

// Prints a value held inside the one being printed. Nothing this program
// puts in an interface prints differently there.
fn format_any_nested(value: &dyn Any) -> String {
    format_any(value)
}

/// Named struct with anonymous struct field
#[derive(Debug, Clone)]
pub struct Config {
    pub name: Rc<RefCell<Option<String>>>,
    pub database: Rc<RefCell<Option<AnonymousStruct1>>>,
    pub cache: Rc<RefCell<Option<AnonymousStruct3>>>,
}

impl Config {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.database.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        let __go_clone_2_0 = { let __guard = self.cache.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        Self {
            name: __go_clone_0_0,
            database: __go_clone_1_0,
            cache: __go_clone_2_0,
        }
    }
}


impl Default for Config {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(AnonymousStruct1::default())));
        let __go_default_2_0 = Rc::new(RefCell::new(Some(AnonymousStruct3::default())));
        Self {
            name: __go_default_0_0,
            database: __go_default_1_0,
            cache: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.database.borrow().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.cache.borrow().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


/// Named struct with slice of anonymous structs
#[derive(Debug, Clone)]
pub struct Dashboard {
    pub title: Rc<RefCell<Option<String>>>,
    pub widgets: Rc<RefCell<Option<Vec<AnonymousStruct4>>>>,
}

impl Dashboard {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.title.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = self.widgets.clone();
        Self {
            title: __go_clone_0_0,
            widgets: __go_clone_1_0,
        }
    }
}


impl Default for Dashboard {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(None));
        Self {
            title: __go_default_0_0,
            widgets: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Dashboard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.title.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice(&self.widgets));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct1 {
    pub host: Rc<RefCell<Option<String>>>,
    pub port: Rc<RefCell<Option<i64>>>,
    pub credentials: Rc<RefCell<Option<AnonymousStruct2>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.host.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.port.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.credentials.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        Self {
            host: __go_clone_0_0,
            port: __go_clone_1_0,
            credentials: __go_clone_2_0,
        }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_2_0 = Rc::new(RefCell::new(Some(AnonymousStruct2::default())));
        Self {
            host: __go_default_0_0,
            port: __go_default_1_0,
            credentials: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.host.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.port.borrow().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.credentials.borrow().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct2 {
    pub user: Rc<RefCell<Option<String>>>,
    pub password: Rc<RefCell<Option<String>>>,
}
impl AnonymousStruct2 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.user.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.password.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            user: __go_clone_0_0,
            password: __go_clone_1_0,
        }
    }
}


impl Default for AnonymousStruct2 {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            user: __go_default_0_0,
            password: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.user.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.password.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct3 {
    pub enabled: Rc<RefCell<Option<bool>>>,
    pub t_t_l: Rc<RefCell<Option<i64>>>,
}
impl AnonymousStruct3 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.enabled.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.t_t_l.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            enabled: __go_clone_0_0,
            t_t_l: __go_clone_1_0,
        }
    }
}


impl Default for AnonymousStruct3 {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(false)));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            enabled: __go_default_0_0,
            t_t_l: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.enabled.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.t_t_l.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct4 {
    pub i_d: Rc<RefCell<Option<i64>>>,
    pub r#type: Rc<RefCell<Option<String>>>,
    pub position: Rc<RefCell<Option<AnonymousStruct5>>>,
}
impl AnonymousStruct4 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.i_d.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.r#type.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.position.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        Self {
            i_d: __go_clone_0_0,
            r#type: __go_clone_1_0,
            position: __go_clone_2_0,
        }
    }
}


impl Default for AnonymousStruct4 {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_2_0 = Rc::new(RefCell::new(Some(AnonymousStruct5::default())));
        Self {
            i_d: __go_default_0_0,
            r#type: __go_default_1_0,
            position: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct4 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.i_d.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.r#type.borrow().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.position.borrow().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct5 {
    pub x: Rc<RefCell<Option<i64>>>,
    pub y: Rc<RefCell<Option<i64>>>,
}
impl AnonymousStruct5 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.y.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
            y: __go_clone_0_1,
        }
    }
}


impl Default for AnonymousStruct5 {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_0_1 = Rc::new(RefCell::new(Some(0)));
        Self {
            x: __go_default_0_0,
            y: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for AnonymousStruct5 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.y.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


fn main() {
        // Test named struct with anonymous struct field
    let mut config = Rc::new(RefCell::new(Some(Config { name: Rc::new(RefCell::new(Some("production".to_string()))), database: Rc::new(RefCell::new(Some(AnonymousStruct1::default()))), cache: Rc::new(RefCell::new(Some(AnonymousStruct3::default()))) })));
    { let new_val = "db.example.com".to_string(); *(*(*config.borrow().as_ref().unwrap()).database.borrow().as_ref().unwrap()).host.borrow_mut() = Some(new_val); };
    { let new_val = 5432; *(*(*config.borrow().as_ref().unwrap()).database.borrow().as_ref().unwrap()).port.borrow_mut() = Some(new_val); };
    { let new_val = "admin".to_string(); *(*(*(*config.borrow().as_ref().unwrap()).database.borrow().as_ref().unwrap()).credentials.borrow().as_ref().unwrap()).user.borrow_mut() = Some(new_val); };
    { let new_val = "secret".to_string(); *(*(*(*config.borrow().as_ref().unwrap()).database.borrow().as_ref().unwrap()).credentials.borrow().as_ref().unwrap()).password.borrow_mut() = Some(new_val); };
    { let new_val = true; *(*(*config.borrow().as_ref().unwrap()).cache.borrow().as_ref().unwrap()).enabled.borrow_mut() = Some(new_val); };
    { let new_val = 300; *(*(*config.borrow().as_ref().unwrap()).cache.borrow().as_ref().unwrap()).t_t_l.borrow_mut() = Some(new_val); };

    print!("Config: {}\n", (*(*config.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone());
    print!("Database: {}:{} (user: {})\n", (*(*(*config.borrow().as_ref().unwrap()).database.borrow().as_ref().unwrap()).host.borrow().as_ref().unwrap()).clone(), (*(*(*config.borrow().as_ref().unwrap()).database.borrow().as_ref().unwrap()).port.borrow().as_ref().unwrap()), (*(*(*(*config.borrow().as_ref().unwrap()).database.borrow().as_ref().unwrap()).credentials.borrow().as_ref().unwrap()).user.borrow().as_ref().unwrap()).clone());
    print!("Cache: enabled={}, TTL={}\n", (*(*(*config.borrow().as_ref().unwrap()).cache.borrow().as_ref().unwrap()).enabled.borrow().as_ref().unwrap()), (*(*(*config.borrow().as_ref().unwrap()).cache.borrow().as_ref().unwrap()).t_t_l.borrow().as_ref().unwrap()));

        // Test named struct with slice of anonymous structs
    let mut dashboard = Rc::new(RefCell::new(Some(Dashboard { title: Rc::new(RefCell::new(Some("Main Dashboard".to_string()))), widgets: Rc::new(RefCell::new(Some(vec![AnonymousStruct4 { i_d: Rc::new(RefCell::new(Some(1))), r#type: Rc::new(RefCell::new(Some("chart".to_string()))), position: Rc::new(RefCell::new(Some(AnonymousStruct5 { x: Rc::new(RefCell::new(Some(0))), y: Rc::new(RefCell::new(Some(0))) }))), ..Default::default() }, AnonymousStruct4 { i_d: Rc::new(RefCell::new(Some(2))), r#type: Rc::new(RefCell::new(Some("table".to_string()))), position: Rc::new(RefCell::new(Some(AnonymousStruct5 { x: Rc::new(RefCell::new(Some(100))), y: Rc::new(RefCell::new(Some(0))) }))), ..Default::default() }]))), ..Default::default() })));

    print!("\nDashboard: {}\n", (*(*dashboard.borrow().as_ref().unwrap()).title.borrow().as_ref().unwrap()).clone());
    { let __range_holder = (*dashboard.borrow().as_ref().unwrap()).widgets.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice().to_vec()).unwrap_or_default(); drop(__range_guard); for widget in __range_values.iter() {
        let widget = &widget.__go_value_clone();
        print!("Widget {} ({}) at position ({}, {})\n", (*widget.i_d.borrow().as_ref().unwrap()), (*widget.r#type.borrow().as_ref().unwrap()).clone(), (*(*widget.position.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap()), (*(*widget.position.borrow().as_ref().unwrap()).y.borrow().as_ref().unwrap()));
    } }

        // Deeply nested anonymous structs
    let mut system: Rc<RefCell<Option<AnonymousStruct6>>> = Rc::new(RefCell::new(Some(Default::default())));

    { let new_val = "1.0.0".to_string(); *(*system.borrow().as_ref().unwrap()).version.borrow_mut() = Some(new_val); };
    { let new_val = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<AnonymousStruct7>>>>::new()))); (*system.borrow_mut().as_mut().unwrap()).modules = new_val; };

        // Add a module with settings
    let mut authModule = Rc::new(RefCell::new(Some(AnonymousStruct7 { enabled: Rc::new(RefCell::new(Some(true))), settings: Rc::new(RefCell::new(Some(AnonymousStruct8::default()))) })));
    { let new_val = Rc::new(RefCell::new(Some(vec![AnonymousStruct9 { key: Rc::new(RefCell::new(Some("timeout".to_string()))), value: Rc::new(RefCell::new(Some(Box::new(3600i64) as Box<dyn Any>))), ..Default::default() }, AnonymousStruct9 { key: Rc::new(RefCell::new(Some("max_attempts".to_string()))), value: Rc::new(RefCell::new(Some(Box::new(3i64) as Box<dyn Any>))), ..Default::default() }]))); (*(*authModule.borrow().as_ref().unwrap()).settings.borrow_mut().as_mut().unwrap()).options = new_val; };
    { let __map_key = "auth".to_string(); let __map_value = { let __owned = authModule.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) }; (*(*system.borrow().as_ref().unwrap()).modules.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };

    print!("\nSystem version: {}\n", (*(*system.borrow().as_ref().unwrap()).version.borrow().as_ref().unwrap()).clone());
    for (name, module) in ((*(*system.borrow().as_ref().unwrap()).modules.borrow().as_ref().unwrap()).clone()).clone() {
        let module = Rc::new(RefCell::new(Some(module.borrow().as_ref().unwrap().__go_value_clone())));
        print!("Module {}: enabled={}\n", name, (*(*module.borrow().as_ref().unwrap()).enabled.borrow().as_ref().unwrap()));
        { let __range_holder = (*(*module.borrow().as_ref().unwrap()).settings.borrow().as_ref().unwrap()).options.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice().to_vec()).unwrap_or_default(); drop(__range_guard); for opt in __range_values.iter() {
        let opt = &opt.__go_value_clone();
        print!("  - {}: {}\n", (*opt.key.borrow().as_ref().unwrap()).clone(), format_any(opt.value.borrow().as_ref().unwrap().as_ref()));
    } }
    }
}

#[derive(Clone)]
pub struct AnonymousStruct6 {
    pub version: Rc<RefCell<Option<String>>>,
    pub modules: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<AnonymousStruct7>>>>>>>,
}
impl AnonymousStruct6 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.version.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = self.modules.clone();
        Self {
            version: __go_clone_0_0,
            modules: __go_clone_1_0,
        }
    }
}


impl Default for AnonymousStruct6 {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(None));
        Self {
            version: __go_default_0_0,
            modules: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct6 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.version.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_map(&self.modules));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct7 {
    pub enabled: Rc<RefCell<Option<bool>>>,
    pub settings: Rc<RefCell<Option<AnonymousStruct8>>>,
}
impl AnonymousStruct7 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.enabled.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.settings.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        Self {
            enabled: __go_clone_0_0,
            settings: __go_clone_1_0,
        }
    }
}


impl Default for AnonymousStruct7 {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(false)));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(AnonymousStruct8::default())));
        Self {
            enabled: __go_default_0_0,
            settings: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct7 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.enabled.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.settings.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Clone, Default)]
pub struct AnonymousStruct8 {
    pub options: Rc<RefCell<Option<Vec<AnonymousStruct9>>>>,
}
impl AnonymousStruct8 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.options.clone();
        Self {
            options: __go_clone_0_0,
        }
    }
}


impl std::fmt::Display for AnonymousStruct8 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_slice(&self.options));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct9 {
    pub key: Rc<RefCell<Option<String>>>,
    pub value: Rc<RefCell<Option<Box<dyn Any>>>>,
}
impl AnonymousStruct9 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.key.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = self.value.clone();
        Self {
            key: __go_clone_0_0,
            value: __go_clone_1_0,
        }
    }
}


impl Default for AnonymousStruct9 {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(None));
        Self {
            key: __go_default_0_0,
            value: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct9 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.key.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", match self.value.borrow().as_ref() { Some(__v) => format_any_nested(__v.as_ref()), None => "<nil>".to_string() });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}
