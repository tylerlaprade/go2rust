#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use std::any::Any;
use std::cmp::Ord;
use std::collections::BTreeMap;
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
const __GO_STRING_BYTES_PREFIX: &str = "\u{fdd0}\u{fdd1}";

/// Stores arbitrary Go string bytes in a Rust String without violating
/// Rust's UTF-8 invariant. Valid UTF-8 keeps its ordinary representation;
/// other byte strings use a reserved noncharacter prefix plus hex bytes.
pub fn __go_string_from_bytes(__bytes: std::vec::Vec<u8>) -> String {
    if let Ok(__text) = std::str::from_utf8(&__bytes) {
        if !__text.starts_with(__GO_STRING_BYTES_PREFIX) {
            return __text.to_string();
        }
    }
    let mut __encoded = String::from(__GO_STRING_BYTES_PREFIX);
    for __byte in __bytes {
        use std::fmt::Write as _;
        write!(&mut __encoded, "{:02x}", __byte).unwrap();
    }
    __encoded
}

pub fn __go_string_bytes(__text: &str) -> std::vec::Vec<u8> {
    let Some(__hex) = __text.strip_prefix(__GO_STRING_BYTES_PREFIX) else {
        return __text.as_bytes().to_vec();
    };
    if __hex.len() % 2 != 0 || !__hex.as_bytes().iter().all(|__b| __b.is_ascii_hexdigit()) {
        return __text.as_bytes().to_vec();
    }
    (0..__hex.len())
        .step_by(2)
        .map(|__i| u8::from_str_radix(&__hex[__i..__i + 2], 16).unwrap())
        .collect()
}

pub fn __go_string_concat(__left: &str, __right: &str) -> String {
    let mut __bytes = __go_string_bytes(__left);
    __bytes.extend(__go_string_bytes(__right));
    __go_string_from_bytes(__bytes)
}

pub fn __go_string_cmp(__left: &str, __right: &str) -> std::cmp::Ordering {
    __go_string_bytes(__left).cmp(&__go_string_bytes(__right))
}

pub fn __go_string_len(__text: &str) -> usize {
    __go_string_bytes(__text).len()
}

pub fn __go_string_range(__text: &str) -> std::vec::Vec<(usize, char)> {
    let __bytes = __go_string_bytes(__text);
    let mut __items = std::vec::Vec::new();
    let mut __index = 0usize;
    while __index < __bytes.len() {
        let __first = __bytes[__index];
        let __width = match __first {
            0x00..=0x7f => 1,
            0xc2..=0xdf => 2,
            0xe0..=0xef => 3,
            0xf0..=0xf4 => 4,
            _ => 1,
        };
        let mut __rune = '\u{fffd}';
        let mut __used = 1usize;
        if __first <= 0x7f {
            __rune = __first as char;
        } else if __width > 1 && __index + __width <= __bytes.len() {
            if let Ok(__valid) = std::str::from_utf8(&__bytes[__index..__index + __width]) {
                if let Some(__decoded) = __valid.chars().next() {
                    __rune = __decoded;
                    __used = __width;
                }
            }
        }
        __items.push((__index, __rune));
        __index += __used;
    }
    __items
}

pub fn __go_string_slice(__text: &str, __low: usize, __high: usize) -> String {
    let __bytes = __go_string_bytes(__text);
    __go_string_from_bytes(__bytes[__low..__high].to_vec())
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

fn format_map<K: Display + Ord + Clone, V>(map: &Arc<Mutex<Option<BTreeMap<K, Arc<Mutex<Option<V>>>>>>>) -> String
where
    V: Display,
{
    let guard = map.lock().unwrap();
    if let Some(ref m) = *guard {
        let mut items: std::vec::Vec<_> = m.iter().collect();
        items.sort_by_key(|(k, _)| (*k).clone());

        let formatted: std::vec::Vec<String> = items
            .into_iter()
            .map(|(k, v)| {
                let v_guard = v.lock().unwrap();
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
fn format_slice<T, C>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: GoSliceView<T>,
    T: Display,
{
    let guard = slice.lock().unwrap();
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

fn format_slice_wrapped<T, C>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: GoSliceView<Arc<Mutex<Option<T>>>>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: std::vec::Vec<String> = s.go_slice_view().iter().map(|v| {
            let inner = v.lock().unwrap();
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

#[derive(Debug, Clone)]
pub struct Cache__string__User {
    pub data: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<User>>>>>>>,
    pub order: Arc<Mutex<Option<Vec<String>>>>,
    pub limit: Arc<Mutex<Option<i64>>>,
}

impl Cache__string__User {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.data.clone();
        let __go_clone_1_0 = self.order.clone();
        let __go_clone_2_0 = { let __guard = self.limit.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            data: __go_clone_0_0,
            order: __go_clone_1_0,
            limit: __go_clone_2_0,
        }
    }
}


impl Default for Cache__string__User {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
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
        let __go_fmt_2 = format!("{}", (*self.limit.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


#[derive(Debug, Clone)]
pub struct Cache__int__slice_string {
    pub data: Arc<Mutex<Option<BTreeMap<i64, Arc<Mutex<Option<Vec<String>>>>>>>>,
    pub order: Arc<Mutex<Option<Vec<i64>>>>,
    pub limit: Arc<Mutex<Option<i64>>>,
}

impl Cache__int__slice_string {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.data.clone();
        let __go_clone_1_0 = self.order.clone();
        let __go_clone_2_0 = { let __guard = self.limit.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            data: __go_clone_0_0,
            order: __go_clone_1_0,
            limit: __go_clone_2_0,
        }
    }
}


impl Default for Cache__int__slice_string {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
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
        let __go_fmt_2 = format!("{}", (*self.limit.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


#[derive(Debug, Clone)]
pub struct User {
    pub name: Arc<Mutex<Option<String>>>,
    pub age: Arc<Mutex<Option<i64>>>,
}

impl User {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.age.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
            age: __go_clone_1_0,
        }
    }
}


impl Default for User {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            name: __go_default_0_0,
            age: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.age.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}
impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.name.lock().unwrap().as_ref().cloned() }; let __right = { other.name.lock().unwrap().as_ref().cloned() }; __left == __right }
                && { let __left = { self.age.lock().unwrap().as_ref().cloned() }; let __right = { other.age.lock().unwrap().as_ref().cloned() }; __left == __right }
        )
    }
}


pub trait Number: std::fmt::Display + Any {
    fn __go_clone_box_number(&self) -> Box<dyn Number + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_number(&self, other: &(dyn Number + Send + Sync)) -> bool;
}

impl Clone for Box<dyn Number + Send + Sync> {
    fn clone(&self) -> Self {
        Number::__go_clone_box_number(self.as_ref())
    }
}

impl Number for i8 {
    fn __go_clone_box_number(&self) -> Box<dyn Number + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Number + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &(dyn Number + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<i8>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for i16 {
    fn __go_clone_box_number(&self) -> Box<dyn Number + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Number + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &(dyn Number + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<i16>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for i32 {
    fn __go_clone_box_number(&self) -> Box<dyn Number + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Number + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &(dyn Number + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<i32>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for i64 {
    fn __go_clone_box_number(&self) -> Box<dyn Number + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Number + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &(dyn Number + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<i64>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for isize {
    fn __go_clone_box_number(&self) -> Box<dyn Number + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Number + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &(dyn Number + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<isize>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for u8 {
    fn __go_clone_box_number(&self) -> Box<dyn Number + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Number + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &(dyn Number + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<u8>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for u16 {
    fn __go_clone_box_number(&self) -> Box<dyn Number + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Number + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &(dyn Number + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<u16>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for u32 {
    fn __go_clone_box_number(&self) -> Box<dyn Number + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Number + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &(dyn Number + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<u32>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for u64 {
    fn __go_clone_box_number(&self) -> Box<dyn Number + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Number + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &(dyn Number + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<u64>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for usize {
    fn __go_clone_box_number(&self) -> Box<dyn Number + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Number + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &(dyn Number + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<usize>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for f32 {
    fn __go_clone_box_number(&self) -> Box<dyn Number + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Number + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &(dyn Number + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<f32>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for f64 {
    fn __go_clone_box_number(&self) -> Box<dyn Number + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Number + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &(dyn Number + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<f64>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for String {
    fn __go_clone_box_number(&self) -> Box<dyn Number + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Number + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &(dyn Number + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<String>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct Celsius(pub Arc<Mutex<Option<f64>>>);

impl Clone for Celsius {
    fn clone(&self) -> Self {
        let __value = (*self.0.lock().unwrap()).clone();
        Celsius(Arc::new(Mutex::new(__value)))
    }
}

impl Default for Celsius {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(Some(Default::default()))))
    }
}

impl Display for Celsius {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for Celsius {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<f64> for Celsius {
    fn eq(&self, other: &f64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Celsius {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<f64> for Celsius {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Celsius> for f64 {
    fn eq(&self, other: &Celsius) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<Celsius> for f64 {
    fn partial_cmp(&self, other: &Celsius) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for Celsius {
    type Output = Celsius;
    fn add(self, other: Self) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<f64> for Celsius {
    type Output = Celsius;
    fn add(self, other: f64) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Celsius> for f64 {
    type Output = Celsius;
    fn add(self, other: Celsius) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Celsius {
    type Output = Celsius;
    fn sub(self, other: Self) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<f64> for Celsius {
    type Output = Celsius;
    fn sub(self, other: f64) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Celsius> for f64 {
    type Output = Celsius;
    fn sub(self, other: Celsius) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Celsius {
    type Output = Celsius;
    fn mul(self, other: Self) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<f64> for Celsius {
    type Output = Celsius;
    fn mul(self, other: f64) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Celsius> for f64 {
    type Output = Celsius;
    fn mul(self, other: Celsius) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Celsius {
    type Output = Celsius;
    fn div(self, other: Self) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<f64> for Celsius {
    type Output = Celsius;
    fn div(self, other: f64) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Celsius> for f64 {
    type Output = Celsius;
    fn div(self, other: Celsius) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for Celsius {
    type Output = Celsius;
    fn neg(self) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}


impl Cache__int__slice_string {
    pub fn put(&mut self, k: Arc<Mutex<Option<i64>>>, v: Arc<Mutex<Option<Vec<String>>>>) {
        {
        let (_, mut ok) = { let __map = { let __map_holder = self.data.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&{ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v })) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } };;
        if !ok {
            { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = self.order.clone(); let __append_value = ({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }); let mut __appended: Vec<i64> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *self.order.lock().unwrap() = __slice_header; };;
        }
    }
        { let __map_key = { let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __map_value = { let __slice_header = GoSliceHeader::go_slice_header(&v); Arc::new(Mutex::new(__slice_header)) }; (*self.data.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        if {
            let __tmp_x = (({ let __len = ({ let __len_target = { let __field = self.order.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }); __len }) as i64);
            let __tmp_y = ((*self.limit.clone().lock().unwrap().as_ref().unwrap()) as i64);
            __tmp_x > __tmp_y
        } {
        let mut oldest = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.order.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq.get_cloned((0) as usize) })));
        { let new_val = Arc::new(Mutex::new(Some({
            let __seq_holder = self.order.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
            let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
            drop(__seq_guard);
            let __low = (1) as usize;
            let __high = __seq.len();
            let __max = __source_cap;
            __seq.reslice3(__low, __high, __max)
        }))); self.order = new_val; };
        { let __map_handle = self.data.clone(); let mut __map_guard = __map_handle.lock().unwrap(); __map_guard.as_mut().unwrap().remove(&{ let __v = (*oldest.lock().unwrap().as_ref().unwrap()).clone(); __v }); };
    }
    }

    pub fn get(&self, k: Arc<Mutex<Option<i64>>>) -> (Arc<Mutex<Option<Vec<String>>>>, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        let (mut v, mut ok) = { let __map = { let __map_holder = self.data.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&{ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v })) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } };
        let mut v = { let __slice_header = GoSliceHeader::go_slice_header(&v); Arc::new(Mutex::new(__slice_header)) };
        if !ok {
        let mut zero: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
        return (
            zero.clone(),
            Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("miss".to_string())))))
        );
    }
        return (v.clone(), Arc::new(Mutex::new(None)));
    }
}

impl Cache__string__User {
    pub fn put(&mut self, k: Arc<Mutex<Option<String>>>, v: Arc<Mutex<Option<User>>>) {
        {
        let (_, mut ok) = { let __map = { let __map_holder = self.data.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }))) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Arc::new(Mutex::new(Some(Default::default()))), false) } };;
        if !ok {
            { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = self.order.clone(); let __append_value = ({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *self.order.lock().unwrap() = __slice_header; };;
        }
    }
        { let __map_key = ({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __map_value = { let __owned = v.lock().unwrap().as_ref().unwrap().__go_value_clone(); Arc::new(Mutex::new(Some(__owned))) }; (*self.data.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        if {
            let __tmp_x = (({ let __len = ({ let __len_target = { let __field = self.order.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }); __len }) as i64);
            let __tmp_y = ((*self.limit.clone().lock().unwrap().as_ref().unwrap()) as i64);
            __tmp_x > __tmp_y
        } {
        let mut oldest = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.order.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq.get_cloned((0) as usize) })));
        { let new_val = Arc::new(Mutex::new(Some({
            let __seq_holder = self.order.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
            let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
            drop(__seq_guard);
            let __low = (1) as usize;
            let __high = __seq.len();
            let __max = __source_cap;
            __seq.reslice3(__low, __high, __max)
        }))); self.order = new_val; };
        { let __map_handle = self.data.clone(); let mut __map_guard = __map_handle.lock().unwrap(); __map_guard.as_mut().unwrap().remove(&({ let __v = (*oldest.lock().unwrap().as_ref().unwrap()).clone(); __v })); };
    }
    }

    pub fn get(&self, k: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<User>>>, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        let (mut v, mut ok) = { let __map = { let __map_holder = self.data.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }))) { /* MAP_COMMA_OK */ Some(v) => (Arc::new(Mutex::new(Some(v.lock().unwrap().as_ref().unwrap().__go_value_clone()))), true), None => (Arc::new(Mutex::new(Some(Default::default()))), false) } };
        if !ok {
        let mut zero: Arc<Mutex<Option<User>>> = Arc::new(Mutex::new(Some(Default::default())));
        return (
            { let __owned = zero.lock().unwrap().as_ref().unwrap().__go_value_clone(); Arc::new(Mutex::new(Some(__owned))) },
            Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("miss".to_string())))))
        );
    }
        return ({ let __owned = v.lock().unwrap().as_ref().unwrap().__go_value_clone(); Arc::new(Mutex::new(Some(__owned))) }, Arc::new(Mutex::new(None)));
    }
}

impl Celsius {
}

impl Number for Celsius {
    fn __go_clone_box_number(&self) -> Box<dyn Number + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Number + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &(dyn Number + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Celsius>() {
            self == __other
        } else {
            false
        }
    }
}

pub fn new_cache__string___user(limit: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<Cache__string__User>>> {
    Arc::new(Mutex::new(Some(Cache__string__User { data: Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<User>>>>::new()))), limit: Arc::new(Mutex::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })))
}

pub fn new_cache__int__slice_string(limit: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<Cache__int__slice_string>>> {
    Arc::new(Mutex::new(Some(Cache__int__slice_string { data: Arc::new(Mutex::new(Some(BTreeMap::<i64, Arc<Mutex<Option<Vec<String>>>>>::new()))), limit: Arc::new(Mutex::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })))
}

pub fn sum_by___user__int(xs: Arc<Mutex<Option<Vec<User>>>>, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<User>>>) -> i64 + Send + Sync>>>>) -> i64 {
    let mut total: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));

    { let __range_holder = xs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice().to_vec()).unwrap_or_default(); drop(__range_guard); for x in __range_values.iter() {
        let x = &x.__go_value_clone();
        { let __rhs = {
            let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<User>>>) -> i64 + Send + Sync> = {
                let mut __f_guard = f.lock().unwrap();
                __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<User>>>) -> i64 + Send + Sync>
            };
            let __f = unsafe { &mut *__f_ptr };
            (*__f)(
                Arc::new(Mutex::new(Some((*x).__go_value_clone())))
            )
        }; let mut guard = total.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }
    return { let __v = (*total.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn sum_by___user__float64(xs: Arc<Mutex<Option<Vec<User>>>>, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<User>>>) -> f64 + Send + Sync>>>>) -> f64 {
    let mut total: Arc<Mutex<Option<f64>>> = Arc::new(Mutex::new(Some(0.0)));

    { let __range_holder = xs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice().to_vec()).unwrap_or_default(); drop(__range_guard); for x in __range_values.iter() {
        let x = &x.__go_value_clone();
        { let __rhs = {
            let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<User>>>) -> f64 + Send + Sync> = {
                let mut __f_guard = f.lock().unwrap();
                __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<User>>>) -> f64 + Send + Sync>
            };
            let __f = unsafe { &mut *__f_ptr };
            (*__f)(
                Arc::new(Mutex::new(Some((*x).__go_value_clone())))
            )
        }; let mut guard = total.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }
    return { let __v = (*total.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn sum_by___celsius___celsius(xs: Arc<Mutex<Option<Vec<Celsius>>>>, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Celsius>>>) -> Arc<Mutex<Option<Celsius>>> + Send + Sync>>>>) -> Arc<Mutex<Option<Celsius>>> {
    let mut total: Arc<Mutex<Option<Celsius>>> = Arc::new(Mutex::new(Some(Celsius(Arc::new(Mutex::new(Some(0.0)))))));

    { let __range_holder = xs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice().to_vec()).unwrap_or_default(); drop(__range_guard); for x in __range_values.iter().cloned() {
        { let __rhs = (*{
            let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Celsius>>>) -> Arc<Mutex<Option<Celsius>>> + Send + Sync> = {
                let mut __f_guard = f.lock().unwrap();
                __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Celsius>>>) -> Arc<Mutex<Option<Celsius>>> + Send + Sync>
            };
            let __f = unsafe { &mut *__f_ptr };
            (*__f)(
                Arc::new(Mutex::new(Some(x.clone())))
            )
        }.lock().unwrap().as_ref().unwrap()).clone(); let mut guard = total.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() + __rhs); };
    } }
    return { let __owned = total.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}

pub fn group_by___user__int(xs: Arc<Mutex<Option<Vec<User>>>>, key: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<User>>>) -> i64 + Send + Sync>>>>) -> Arc<Mutex<Option<BTreeMap<i64, Arc<Mutex<Option<Vec<User>>>>>>>> {
    let mut out = Arc::new(Mutex::new(Some(BTreeMap::<i64, Arc<Mutex<Option<Vec<User>>>>>::new())));
    { let __range_holder = xs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice().to_vec()).unwrap_or_default(); drop(__range_guard); for x in __range_values.iter() {
        let x = &x.__go_value_clone();
        let mut k = {
            let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<User>>>) -> i64 + Send + Sync> = {
                let mut __f_guard = key.lock().unwrap();
                __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<User>>>) -> i64 + Send + Sync>
            };
            let __f = unsafe { &mut *__f_ptr };
            (*__f)(
                Arc::new(Mutex::new(Some((*x).__go_value_clone())))
            )
        };
        { let __map_key = k; let __map_value = { let __slice = { let __map_holder = out.clone(); let __map_guard = __map_holder.lock().unwrap(); __map_guard.as_ref().unwrap().get(&k).cloned().unwrap_or_else(|| Arc::new(Mutex::new(None))) }; let __append_value = (*x).__go_value_clone(); let mut __appended = (*__slice.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) }; (*out.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    } }
    return out.clone();
}

fn main() {
    let mut started = GoChannel::<bool>::new();
    let started_thread = started.clone(); std::thread::spawn(move || {
        started_thread.send(true);;;
    });
    started.recv().unwrap_or_default();
    let mut c = new_cache__string___user(Arc::new(Mutex::new(Some(2))));
    { let __recv = c.clone(); let __recv_ptr: *mut Cache__string__User = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Cache__string__User }; let __result = unsafe { &mut *__recv_ptr }.put(
        Arc::new(Mutex::new(Some("a".to_string()))),
        Arc::new(Mutex::new(Some(User { name: Arc::new(Mutex::new(Some("Ann".to_string()))), age: Arc::new(Mutex::new(Some(30))), ..Default::default() }))),
    ); __result };
    { let __recv = c.clone(); let __recv_ptr: *mut Cache__string__User = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Cache__string__User }; let __result = unsafe { &mut *__recv_ptr }.put(
        Arc::new(Mutex::new(Some("b".to_string()))),
        Arc::new(Mutex::new(Some(User { name: Arc::new(Mutex::new(Some("Bob".to_string()))), age: Arc::new(Mutex::new(Some(25))), ..Default::default() }))),
    ); __result };
    { let __recv = c.clone(); let __recv_ptr: *mut Cache__string__User = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Cache__string__User }; let __result = unsafe { &mut *__recv_ptr }.put(
        Arc::new(Mutex::new(Some("c".to_string()))),
        Arc::new(Mutex::new(Some(User { name: Arc::new(Mutex::new(Some("Cy".to_string()))), age: Arc::new(Mutex::new(Some(40))), ..Default::default() }))),
    ); __result };
    let (_, mut err) = { let __recv = c.clone(); let __recv_ptr: *const Cache__string__User = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Cache__string__User }; let __result = unsafe { &*__recv_ptr }.get(Arc::new(Mutex::new(Some("a".to_string())))); __result };
    let (mut u, mut err2) = { let __recv = c.clone(); let __recv_ptr: *const Cache__string__User = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Cache__string__User }; let __result = unsafe { &*__recv_ptr }.get(Arc::new(Mutex::new(Some("c".to_string())))); __result };
    println!("{} {} {} {} {}", format!("{}", { let __go_print_handle = err.clone(); let __go_print_value = { let __go_guard = __go_print_handle.lock().unwrap(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __v = (*u.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }), format!("{}", { let __go_print_handle = err2.clone(); let __go_print_value = { let __go_guard = __go_print_handle.lock().unwrap(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", ({ let __len = ({ let __len_target = { let __field = (*c.lock().unwrap().as_ref().unwrap()).data.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }); __len })), format!("{}", format_slice_values(&(*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).order.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone())));

    let mut users = Arc::new(Mutex::new(Some(vec![
        User { name: Arc::new(Mutex::new(Some("Ann".to_string()))), age: Arc::new(Mutex::new(Some(30))), ..Default::default() },
        User { name: Arc::new(Mutex::new(Some("Bob".to_string()))), age: Arc::new(Mutex::new(Some(25))), ..Default::default() },
        User { name: Arc::new(Mutex::new(Some("Cy".to_string()))), age: Arc::new(Mutex::new(Some(40))), ..Default::default() },
        User { name: Arc::new(Mutex::new(Some("Di".to_string()))), age: Arc::new(Mutex::new(Some(25))), ..Default::default() },
    ])));
    println!("{}", format!("{}", sum_by___user__int(users.clone(), Arc::new(Mutex::new(Some(Box::new(move |u: Arc<Mutex<Option<User>>>| -> i64 {
        return (*(*u.lock().unwrap().as_ref().unwrap()).age.lock().unwrap().as_ref().unwrap());
    }) as Box<dyn FnMut(Arc<Mutex<Option<User>>>) -> i64 + Send + Sync>))))));
    println!("{}", format!("{}", sum_by___user__float64(
        users.clone(),
        Arc::new(Mutex::new(Some(Box::new(move |u: Arc<Mutex<Option<User>>>| -> f64 {
        return {
            let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*u.lock().unwrap().as_ref().unwrap()).age.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as f64))).lock().unwrap().as_ref().unwrap());
            let __tmp_y = 2.0;
            __tmp_x / __tmp_y
        };
    }) as Box<dyn FnMut(Arc<Mutex<Option<User>>>) -> f64 + Send + Sync>)))
    )));
    let mut temps = Arc::new(Mutex::new(Some(vec![Celsius(Arc::new(Mutex::new(Some(20.5)))), Celsius(Arc::new(Mutex::new(Some(22.0)))), Celsius(Arc::new(Mutex::new(Some(19.5))))])));
    println!("{}", format!("{}", (*sum_by___celsius___celsius(temps.clone(), Arc::new(Mutex::new(Some(Box::new(move |c: Arc<Mutex<Option<Celsius>>>| -> Arc<Mutex<Option<Celsius>>> {
        return { let __owned = c.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }) as Box<dyn FnMut(Arc<Mutex<Option<Celsius>>>) -> Arc<Mutex<Option<Celsius>>> + Send + Sync>)))).lock().unwrap().as_ref().unwrap())));
    let mut groups = group_by___user__int(users.clone(), Arc::new(Mutex::new(Some(Box::new(move |u: Arc<Mutex<Option<User>>>| -> i64 {
        return (*(*u.lock().unwrap().as_ref().unwrap()).age.lock().unwrap().as_ref().unwrap());
    }) as Box<dyn FnMut(Arc<Mutex<Option<User>>>) -> i64 + Send + Sync>))));
    let mut ages = Arc::new(Mutex::new(Some({ let mut v: Vec<i64> = (0..(({ let __len = (*groups.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })) as usize).map(|_| 0).collect(); v.truncate((0) as usize); v })));
    for (age, _) in { let __range_holder = groups.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = ages.clone(); let __append_value = age; let mut __appended: Vec<i64> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *ages.lock().unwrap() = __slice_header; };
    }
    { let __sort_target = ages.clone(); let mut __sort_guard = __sort_target.lock().unwrap(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort(); } };
    { let __range_holder = ages.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice().to_vec()).unwrap_or_default(); drop(__range_guard); for age in __range_values.iter().copied() {
        let mut names: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
        { let __range_holder = { let __map = { let __map_holder = groups.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&age)).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice().to_vec()).unwrap_or_default(); drop(__range_guard); for u in __range_values.iter() {
        let u = &u.__go_value_clone();
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = names.clone(); let __append_value = { let __selector_holder = u.name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *names.lock().unwrap() = __slice_header; };
    } }
        { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = format!("{}", age); __go_sprint_bytes.extend(__go_sprint_arg_0.into_bytes()); __go_sprint_bytes.push(b' '); let __go_sprint_arg_1 = (*Arc::new(Mutex::new(Some({ let __parts = (*names.lock().unwrap()).as_ref().cloned().unwrap_or_default(); let __sep = ",".to_string(); let __sep_bytes = __go_string_bytes(&__sep); let mut __out = Vec::new(); for (__i, __part) in __parts.iter().enumerate() { if __i != 0 { __out.extend_from_slice(&__sep_bytes); } __out.extend(__go_string_bytes(__part)); } __go_string_from_bytes(__out) }))).lock().unwrap().as_ref().unwrap()).clone(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_1)); __go_sprint_bytes.push(b'\n'); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>))), Err(e) => (0, Arc::new(Mutex::new(Some::<Arc<dyn StdError + Send + Sync>>(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(e)))))) } };
    } }
    let mut ic = new_cache__int__slice_string(Arc::new(Mutex::new(Some(3))));
    { let __recv = ic.clone(); let __recv_ptr: *mut Cache__int__slice_string = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Cache__int__slice_string }; let __result = unsafe { &mut *__recv_ptr }.put(
        Arc::new(Mutex::new(Some(1))),
        Arc::new(Mutex::new(Some(vec!["x".to_string()]))),
    ); __result };
    let (mut got, _) = { let __recv = ic.clone(); let __recv_ptr: *const Cache__int__slice_string = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Cache__int__slice_string }; let __result = unsafe { &*__recv_ptr }.get(Arc::new(Mutex::new(Some(1)))); __result };
    let mut got = { let __slice_header = GoSliceHeader::go_slice_header(&got); Arc::new(Mutex::new(__slice_header)) };
    println!("{} {}", format!("{}", format_slice(&got)), format!("{}", ({ let __len = (*got.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })));
}