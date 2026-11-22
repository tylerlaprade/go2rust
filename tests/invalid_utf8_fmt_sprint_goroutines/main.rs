#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use std::any::Any;
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

fn format_any(value: &(dyn Any + Send + Sync)) -> String {
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
    } else if let Some(v) = value.downcast_ref::<boxedError>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<boxedStringer>() {
        format!("{}", v)
    } else {
        panic!("go2rust: cannot print this value held in an interface: its type has no formatting case yet")
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

#[derive(Debug)]
pub struct stringerText(pub Arc<Mutex<Option<String>>>);

impl Clone for stringerText {
    fn clone(&self) -> Self {
        let __value = (*self.0.lock().unwrap()).clone();
        stringerText(Arc::new(Mutex::new(__value)))
    }
}

impl Default for stringerText {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(Some(Default::default()))))
    }
}

impl Display for stringerText {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl PartialEq for stringerText {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialOrd for stringerText {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ord::cmp(self, other))
    }
}

impl Eq for stringerText {}

impl Ord for stringerText {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}

impl PartialEq<String> for stringerText {
    fn eq(&self, other: &String) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialEq<&str> for stringerText {
    fn eq(&self, other: &&str) -> bool {
        self.0.lock().unwrap().as_ref().unwrap().as_str() == *other
    }
}

impl std::ops::Add for stringerText {
    type Output = stringerText;
    fn add(self, other: Self) -> stringerText {
        let __joined = format!("{}{}", self.0.lock().unwrap().as_ref().unwrap(), other.0.lock().unwrap().as_ref().unwrap());
        stringerText(Arc::new(Mutex::new(Some(__joined))))
    }
}

impl std::ops::Add<String> for stringerText {
    type Output = stringerText;
    fn add(self, other: String) -> stringerText {
        let __joined = format!("{}{}", self.0.lock().unwrap().as_ref().unwrap(), other);
        stringerText(Arc::new(Mutex::new(Some(__joined))))
    }
}

impl std::ops::Add<&str> for stringerText {
    type Output = stringerText;
    fn add(self, other: &str) -> stringerText {
        let __joined = format!("{}{}", self.0.lock().unwrap().as_ref().unwrap(), other);
        stringerText(Arc::new(Mutex::new(Some(__joined))))
    }
}


#[derive(Debug)]
pub struct errorText(pub Arc<Mutex<Option<String>>>);

impl Clone for errorText {
    fn clone(&self) -> Self {
        let __value = (*self.0.lock().unwrap()).clone();
        errorText(Arc::new(Mutex::new(__value)))
    }
}

impl Default for errorText {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(Some(Default::default()))))
    }
}

impl Display for errorText {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}

impl PartialEq for errorText {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialOrd for errorText {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ord::cmp(self, other))
    }
}

impl Eq for errorText {}

impl Ord for errorText {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}

impl PartialEq<String> for errorText {
    fn eq(&self, other: &String) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialEq<&str> for errorText {
    fn eq(&self, other: &&str) -> bool {
        self.0.lock().unwrap().as_ref().unwrap().as_str() == *other
    }
}

impl std::ops::Add for errorText {
    type Output = errorText;
    fn add(self, other: Self) -> errorText {
        let __joined = format!("{}{}", self.0.lock().unwrap().as_ref().unwrap(), other.0.lock().unwrap().as_ref().unwrap());
        errorText(Arc::new(Mutex::new(Some(__joined))))
    }
}

impl std::ops::Add<String> for errorText {
    type Output = errorText;
    fn add(self, other: String) -> errorText {
        let __joined = format!("{}{}", self.0.lock().unwrap().as_ref().unwrap(), other);
        errorText(Arc::new(Mutex::new(Some(__joined))))
    }
}

impl std::ops::Add<&str> for errorText {
    type Output = errorText;
    fn add(self, other: &str) -> errorText {
        let __joined = format!("{}{}", self.0.lock().unwrap().as_ref().unwrap(), other);
        errorText(Arc::new(Mutex::new(Some(__joined))))
    }
}


#[derive(Debug, Clone, Default)]
pub struct valueStringer {
}

impl valueStringer {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
    }
}

impl std::fmt::Display for valueStringer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct valueError {
}

impl valueError {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
    }
}

impl std::fmt::Display for valueError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}
impl PartialEq for valueError {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}


#[derive(Debug, Clone, Default)]
pub struct boxedStringer {
}

impl boxedStringer {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
    }
}

impl std::fmt::Display for boxedStringer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct boxedError {
}

impl boxedError {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
    }
}

impl std::fmt::Display for boxedError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}
impl PartialEq for boxedError {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}


impl stringerText {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(__go_string_concat(&"string-method".to_string(), &{ let __s = Arc::new(Mutex::new(Some(__go_string_from_bytes(std::vec::Vec::from((*Arc::new(Mutex::new(Some(vec![0xfb as u8]))).lock().unwrap().as_ref().unwrap()).clone()))))); let __value = (*__s.lock().unwrap().as_ref().unwrap()).clone(); __value }))));
    }
}

impl errorText {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(__go_string_concat(&"error-method".to_string(), &{ let __s = Arc::new(Mutex::new(Some(__go_string_from_bytes(std::vec::Vec::from((*Arc::new(Mutex::new(Some(vec![0xfc as u8]))).lock().unwrap().as_ref().unwrap()).clone()))))); let __value = (*__s.lock().unwrap().as_ref().unwrap()).clone(); __value }))));
    }
}

impl StdError for errorText {}

#[derive(Debug, Clone)]
pub struct __go_error_ptr_errorText(pub Arc<Mutex<Option<errorText>>>);

impl std::fmt::Display for __go_error_ptr_errorText {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => std::fmt::Display::fmt(__v, f), None => write!(f, "<nil>") }
    }
}

impl StdError for __go_error_ptr_errorText {}


impl valueStringer {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(__go_string_concat(&"value-stringer".to_string(), &{ let __s = Arc::new(Mutex::new(Some(__go_string_from_bytes(std::vec::Vec::from((*Arc::new(Mutex::new(Some(vec![0xfd as u8]))).lock().unwrap().as_ref().unwrap()).clone()))))); let __value = (*__s.lock().unwrap().as_ref().unwrap()).clone(); __value }))));
    }
}

impl valueError {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(__go_string_concat(&"value-error".to_string(), &{ let __s = Arc::new(Mutex::new(Some(__go_string_from_bytes(std::vec::Vec::from((*Arc::new(Mutex::new(Some(vec![0xfe as u8]))).lock().unwrap().as_ref().unwrap()).clone()))))); let __value = (*__s.lock().unwrap().as_ref().unwrap()).clone(); __value }))));
    }
}

impl StdError for valueError {}

#[derive(Debug, Clone)]
pub struct __go_error_ptr_valueError(pub Arc<Mutex<Option<valueError>>>);

impl std::fmt::Display for __go_error_ptr_valueError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => std::fmt::Display::fmt(__v, f), None => write!(f, "<nil>") }
    }
}

impl StdError for __go_error_ptr_valueError {}


impl boxedStringer {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(__go_string_concat(&"boxed-stringer".to_string(), &{ let __s = Arc::new(Mutex::new(Some(__go_string_from_bytes(std::vec::Vec::from((*Arc::new(Mutex::new(Some(vec![0xf8 as u8]))).lock().unwrap().as_ref().unwrap()).clone()))))); let __value = (*__s.lock().unwrap().as_ref().unwrap()).clone(); __value }))));
    }
}

impl boxedError {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(__go_string_concat(&"boxed-error".to_string(), &{ let __s = Arc::new(Mutex::new(Some(__go_string_from_bytes(std::vec::Vec::from((*Arc::new(Mutex::new(Some(vec![0xf9 as u8]))).lock().unwrap().as_ref().unwrap()).clone()))))); let __value = (*__s.lock().unwrap().as_ref().unwrap()).clone(); __value }))));
    }
}

impl StdError for boxedError {}

#[derive(Debug, Clone)]
pub struct __go_error_ptr_boxedError(pub Arc<Mutex<Option<boxedError>>>);

impl std::fmt::Display for __go_error_ptr_boxedError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => std::fmt::Display::fmt(__v, f), None => write!(f, "<nil>") }
    }
}

impl StdError for __go_error_ptr_boxedError {}


fn main() {
    let mut done = GoChannel::<AnonymousStruct1>::new();
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.close();;;
    });
    done.recv().unwrap_or_default();

    let mut invalid = Arc::new(Mutex::new(Some(__go_string_from_bytes(std::vec::Vec::from((*Arc::new(Mutex::new(Some(vec![0xfa as u8]))).lock().unwrap().as_ref().unwrap()).clone())))));
    let (mut n, mut err) = { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = "print<".to_string(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); let __go_sprint_arg_1 = (*invalid.lock().unwrap().as_ref().unwrap()).clone(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_1)); let __go_sprint_arg_2 = ">\n".to_string(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_2)); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>))), Err(e) => (0, Arc::new(Mutex::new(Some::<Arc<dyn StdError + Send + Sync>>(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(e)))))) } };
    println!("{} {}", format!("{}", n), format!("{}", { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result }));
    { let (__tmp_0, __tmp_1) = { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = "println<".to_string(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_sprint_bytes.push(b' '); let __go_sprint_arg_1 = (*invalid.lock().unwrap().as_ref().unwrap()).clone(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_1)); __go_sprint_bytes.push(b' '); let __go_sprint_arg_2 = ">".to_string(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_2)); __go_sprint_bytes.push(b'\n'); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>))), Err(e) => (0, Arc::new(Mutex::new(Some::<Arc<dyn StdError + Send + Sync>>(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(e)))))) } }; n = __tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    println!("{} {}", format!("{}", n), format!("{}", { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result }));
    { let (__tmp_0, __tmp_1) = { let __s = { let mut __go_sprintf_bytes = std::vec::Vec::<u8>::new(); __go_sprintf_bytes.extend_from_slice(&[112u8, 114u8, 105u8, 110u8, 116u8, 102u8, 60u8]); let __go_sprintf_arg_0 = (*invalid.lock().unwrap().as_ref().unwrap()).clone(); __go_sprintf_bytes.extend(__go_string_bytes(&__go_sprintf_arg_0)); __go_sprintf_bytes.extend_from_slice(&[62u8, 10u8]); __go_string_from_bytes(__go_sprintf_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>))), Err(e) => (0, Arc::new(Mutex::new(Some::<Arc<dyn StdError + Send + Sync>>(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(e)))))) } }; n = __tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    println!("{} {}", format!("{}", n), format!("{}", { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result }));
    { let (__tmp_0, __tmp_1) = { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = "fprint<".to_string(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); let __go_sprint_arg_1 = (*invalid.lock().unwrap().as_ref().unwrap()).clone(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_1)); let __go_sprint_arg_2 = ">\n".to_string(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_2)); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>))), Err(e) => (0, Arc::new(Mutex::new(Some::<Arc<dyn StdError + Send + Sync>>(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(e)))))) } }; n = __tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    println!("{} {}", format!("{}", n), format!("{}", { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result }));
    { let (__tmp_0, __tmp_1) = { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = "fprintln<".to_string(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_sprint_bytes.push(b' '); let __go_sprint_arg_1 = (*invalid.lock().unwrap().as_ref().unwrap()).clone(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_1)); __go_sprint_bytes.push(b' '); let __go_sprint_arg_2 = ">".to_string(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_2)); __go_sprint_bytes.push(b'\n'); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>))), Err(e) => (0, Arc::new(Mutex::new(Some::<Arc<dyn StdError + Send + Sync>>(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(e)))))) } }; n = __tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    println!("{} {}", format!("{}", n), format!("{}", { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result }));
    { let (__tmp_0, __tmp_1) = { let __s = { let mut __go_sprintf_bytes = std::vec::Vec::<u8>::new(); __go_sprintf_bytes.extend_from_slice(&[102u8, 112u8, 114u8, 105u8, 110u8, 116u8, 102u8, 60u8]); let __go_sprintf_arg_0 = (*invalid.lock().unwrap().as_ref().unwrap()).clone(); __go_sprintf_bytes.extend(__go_string_bytes(&__go_sprintf_arg_0)); __go_sprintf_bytes.extend_from_slice(&[62u8, 10u8]); __go_string_from_bytes(__go_sprintf_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stderr(), &__bytes) { Ok(()) => (__n, Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>))), Err(e) => (0, Arc::new(Mutex::new(Some::<Arc<dyn StdError + Send + Sync>>(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(e)))))) } }; n = __tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    println!("{} {}", format!("{}", n), format!("{}", { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result }));
    { let (__tmp_0, __tmp_1) = { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = { let __go_string_method_receiver = valueStringer {  }; let __go_string_method_result = __go_string_method_receiver.string(); let __go_string_method_text = { let __go_string_method_guard = __go_string_method_result.lock().unwrap(); (*__go_string_method_guard.as_ref().unwrap()).clone() }; __go_string_method_text }; __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>))), Err(e) => (0, Arc::new(Mutex::new(Some::<Arc<dyn StdError + Send + Sync>>(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(e)))))) } }; n = __tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = "\n".to_string(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>))), Err(e) => (0, Arc::new(Mutex::new(Some::<Arc<dyn StdError + Send + Sync>>(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(e)))))) } };
    println!("{} {}", format!("{}", n), format!("{}", { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result }));
    { let (__tmp_0, __tmp_1) = { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = { let __go_string_method_receiver = valueError {  }; let __go_string_method_result = __go_string_method_receiver.error(); let __go_string_method_text = { let __go_string_method_guard = __go_string_method_result.lock().unwrap(); (*__go_string_method_guard.as_ref().unwrap()).clone() }; __go_string_method_text }; __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_sprint_bytes.push(b'\n'); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>))), Err(e) => (0, Arc::new(Mutex::new(Some::<Arc<dyn StdError + Send + Sync>>(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(e)))))) } }; n = __tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    println!("{} {}", format!("{}", n), format!("{}", { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result }));
    { let (__tmp_0, __tmp_1) = { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = { let __go_string_method_receiver = valueStringer {  }; let __go_string_method_result = __go_string_method_receiver.string(); let __go_string_method_text = { let __go_string_method_guard = __go_string_method_result.lock().unwrap(); (*__go_string_method_guard.as_ref().unwrap()).clone() }; __go_string_method_text }; __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>))), Err(e) => (0, Arc::new(Mutex::new(Some::<Arc<dyn StdError + Send + Sync>>(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(e)))))) } }; n = __tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = "\n".to_string(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>))), Err(e) => (0, Arc::new(Mutex::new(Some::<Arc<dyn StdError + Send + Sync>>(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(e)))))) } };
    println!("{} {}", format!("{}", n), format!("{}", { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result }));
    { let (__tmp_0, __tmp_1) = { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = { let __go_string_method_receiver = valueError {  }; let __go_string_method_result = __go_string_method_receiver.error(); let __go_string_method_text = { let __go_string_method_guard = __go_string_method_result.lock().unwrap(); (*__go_string_method_guard.as_ref().unwrap()).clone() }; __go_string_method_text }; __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_sprint_bytes.push(b'\n'); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stderr(), &__bytes) { Ok(()) => (__n, Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>))), Err(e) => (0, Arc::new(Mutex::new(Some::<Arc<dyn StdError + Send + Sync>>(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(e)))))) } }; n = __tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    println!("{} {}", format!("{}", n), format!("{}", { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result }));
    { let (__tmp_0, __tmp_1) = { let __s = { let mut __go_sprintf_bytes = std::vec::Vec::<u8>::new(); __go_sprintf_bytes.extend_from_slice(&[112u8, 114u8, 105u8, 110u8, 116u8, 102u8, 45u8, 109u8, 101u8, 116u8, 104u8, 111u8, 100u8, 60u8]); let __go_sprintf_arg_0 = { let __go_string_method_receiver = valueStringer {  }; let __go_string_method_result = __go_string_method_receiver.string(); let __go_string_method_text = { let __go_string_method_guard = __go_string_method_result.lock().unwrap(); (*__go_string_method_guard.as_ref().unwrap()).clone() }; __go_string_method_text }; __go_sprintf_bytes.extend(__go_string_bytes(&__go_sprintf_arg_0)); __go_sprintf_bytes.extend_from_slice(&[62u8, 10u8]); __go_string_from_bytes(__go_sprintf_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>))), Err(e) => (0, Arc::new(Mutex::new(Some::<Arc<dyn StdError + Send + Sync>>(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(e)))))) } }; n = __tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    println!("{} {}", format!("{}", n), format!("{}", { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result }));
    { let (__tmp_0, __tmp_1) = { let __s = { let mut __go_sprintf_bytes = std::vec::Vec::<u8>::new(); __go_sprintf_bytes.extend_from_slice(&[102u8, 112u8, 114u8, 105u8, 110u8, 116u8, 102u8, 45u8, 109u8, 101u8, 116u8, 104u8, 111u8, 100u8, 60u8]); let __go_sprintf_arg_0 = { let __go_string_method_receiver = valueError {  }; let __go_string_method_result = __go_string_method_receiver.error(); let __go_string_method_text = { let __go_string_method_guard = __go_string_method_result.lock().unwrap(); (*__go_string_method_guard.as_ref().unwrap()).clone() }; __go_string_method_text }; __go_sprintf_bytes.extend(__go_string_bytes(&__go_sprintf_arg_0)); __go_sprintf_bytes.extend_from_slice(&[62u8, 10u8]); __go_string_from_bytes(__go_sprintf_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stderr(), &__bytes) { Ok(()) => (__n, Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>))), Err(e) => (0, Arc::new(Mutex::new(Some::<Arc<dyn StdError + Send + Sync>>(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(e)))))) } }; n = __tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    println!("{} {}", format!("{}", n), format!("{}", { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result }));
    println!("{}", format!("{}", format_slice(&Arc::new(Mutex::new(Some(Vec::from(__go_string_bytes(&(*Arc::new(Mutex::new(Some({ let mut __go_sprintf_bytes = std::vec::Vec::<u8>::new(); __go_sprintf_bytes.extend_from_slice(&[60u8]); let __go_sprintf_arg_0 = { let __go_string_method_receiver = valueStringer {  }; let __go_string_method_result = __go_string_method_receiver.string(); let __go_string_method_text = { let __go_string_method_guard = __go_string_method_result.lock().unwrap(); (*__go_string_method_guard.as_ref().unwrap()).clone() }; __go_string_method_text }; __go_sprintf_bytes.extend(__go_string_bytes(&__go_sprintf_arg_0)); __go_sprintf_bytes.extend_from_slice(&[62u8]); __go_string_from_bytes(__go_sprintf_bytes) }))).lock().unwrap().as_ref().unwrap())))))))));
    println!("{}", format!("{}", format_slice(&Arc::new(Mutex::new(Some(Vec::from(__go_string_bytes(&(*Arc::new(Mutex::new(Some({ let mut __go_sprintf_bytes = std::vec::Vec::<u8>::new(); __go_sprintf_bytes.extend_from_slice(&[60u8]); let __go_sprintf_arg_0 = { let __go_string_method_receiver = valueError {  }; let __go_string_method_result = __go_string_method_receiver.error(); let __go_string_method_text = { let __go_string_method_guard = __go_string_method_result.lock().unwrap(); (*__go_string_method_guard.as_ref().unwrap()).clone() }; __go_string_method_text }; __go_sprintf_bytes.extend(__go_string_bytes(&__go_sprintf_arg_0)); __go_sprintf_bytes.extend_from_slice(&[62u8]); __go_string_from_bytes(__go_sprintf_bytes) }))).lock().unwrap().as_ref().unwrap())))))))));
    let mut boxedText: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(boxedStringer {  }) as Box<dyn Any + Send + Sync>)));
    let mut boxedProblem: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(boxedError {  }) as Box<dyn Any + Send + Sync>)));
    let mut boxedValues = Arc::new(Mutex::new(Some(vec![Box::new(boxedStringer {  }) as Box<dyn Any + Send + Sync>, Box::new(boxedError {  }) as Box<dyn Any + Send + Sync>])));
    { let (__tmp_0, __tmp_1) = { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = { let __text = format_any(boxedText.lock().unwrap().as_ref().unwrap().as_ref()); __text }; __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>))), Err(e) => (0, Arc::new(Mutex::new(Some::<Arc<dyn StdError + Send + Sync>>(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(e)))))) } }; n = __tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = "\n".to_string(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>))), Err(e) => (0, Arc::new(Mutex::new(Some::<Arc<dyn StdError + Send + Sync>>(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(e)))))) } };
    println!("{} {}", format!("{}", n), format!("{}", { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result }));
    { let (__tmp_0, __tmp_1) = { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = { let __text = format_any(boxedProblem.lock().unwrap().as_ref().unwrap().as_ref()); __text }; __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_sprint_bytes.push(b'\n'); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>))), Err(e) => (0, Arc::new(Mutex::new(Some::<Arc<dyn StdError + Send + Sync>>(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(e)))))) } }; n = __tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    println!("{} {}", format!("{}", n), format!("{}", { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result }));
    { let (__tmp_0, __tmp_1) = { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = { let __text = format_any(boxedText.lock().unwrap().as_ref().unwrap().as_ref()); __text }; __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>))), Err(e) => (0, Arc::new(Mutex::new(Some::<Arc<dyn StdError + Send + Sync>>(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(e)))))) } }; n = __tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = "\n".to_string(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>))), Err(e) => (0, Arc::new(Mutex::new(Some::<Arc<dyn StdError + Send + Sync>>(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(e)))))) } };
    println!("{} {}", format!("{}", n), format!("{}", { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result }));
    { let (__tmp_0, __tmp_1) = { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = { let __text = format_any(boxedProblem.lock().unwrap().as_ref().unwrap().as_ref()); __text }; __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_sprint_bytes.push(b'\n'); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stderr(), &__bytes) { Ok(()) => (__n, Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>))), Err(e) => (0, Arc::new(Mutex::new(Some::<Arc<dyn StdError + Send + Sync>>(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(e)))))) } }; n = __tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    println!("{} {}", format!("{}", n), format!("{}", { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result }));
    println!("{}", format!("{}", format_slice(&Arc::new(Mutex::new(Some(Vec::from(__go_string_bytes(&(*Arc::new(Mutex::new(Some({ let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = { let __text = format_any(boxedText.lock().unwrap().as_ref().unwrap().as_ref()); __text }; __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_string_from_bytes(__go_sprint_bytes) }))).lock().unwrap().as_ref().unwrap())))))))));
    println!("{}", format!("{}", format_slice(&Arc::new(Mutex::new(Some(Vec::from(__go_string_bytes(&(*Arc::new(Mutex::new(Some({ let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = { let __text = format_any(boxedProblem.lock().unwrap().as_ref().unwrap().as_ref()); __text }; __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_sprint_bytes.push(b'\n'); __go_string_from_bytes(__go_sprint_bytes) }))).lock().unwrap().as_ref().unwrap())))))))));
    println!("{}", format!("{}", format_slice(&Arc::new(Mutex::new(Some(Vec::from(__go_string_bytes(&(*Arc::new(Mutex::new(Some({ let __sprint_values = boxedValues.clone(); let __sprint_guard = __sprint_values.lock().unwrap(); let mut __sprint_bytes = std::vec::Vec::<u8>::new(); let mut __sprint_after_string = true; if let Some(__sprint_values) = __sprint_guard.as_ref() { let __sprint_slice = __sprint_values.as_slice(); for (__sprint_index, __sprint_value) in __sprint_slice.iter().enumerate() { let __sprint_is_string = __sprint_value.is::<String>(); if __sprint_index > 0 && !__sprint_is_string && !__sprint_after_string { __sprint_bytes.push(b' '); } let __sprint_text = format_any(__sprint_value.as_ref()); __sprint_bytes.extend(__go_string_bytes(&__sprint_text)); __sprint_after_string = __sprint_is_string; } } __go_string_from_bytes(__sprint_bytes) }))).lock().unwrap().as_ref().unwrap())))))))));
    println!("{}", format!("{}", format_slice(&Arc::new(Mutex::new(Some(Vec::from(__go_string_bytes(&(*Arc::new(Mutex::new(Some({ let __sprint_values = boxedValues.clone(); let __sprint_guard = __sprint_values.lock().unwrap(); let mut __sprint_bytes = std::vec::Vec::<u8>::new(); let mut __sprint_after_string = true; if let Some(__sprint_values) = __sprint_guard.as_ref() { let __sprint_slice = __sprint_values.as_slice(); for (__sprint_index, __sprint_value) in __sprint_slice.iter().enumerate() { let __sprint_is_string = __sprint_value.is::<String>(); if __sprint_index > 0 && !__sprint_is_string && !__sprint_after_string { __sprint_bytes.push(b' '); } let __sprint_text = format_any(__sprint_value.as_ref()); __sprint_bytes.extend(__go_string_bytes(&__sprint_text)); __sprint_after_string = __sprint_is_string; } } __sprint_bytes.push(b'\n'); __go_string_from_bytes(__sprint_bytes) }))).lock().unwrap().as_ref().unwrap())))))))));

    println!("{}", format!("{}", format_slice(&Arc::new(Mutex::new(Some(Vec::from(__go_string_bytes(&(*Arc::new(Mutex::new(Some({ let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = "x".to_string(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); let __go_sprint_arg_1 = (*invalid.lock().unwrap().as_ref().unwrap()).clone(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_1)); __go_string_from_bytes(__go_sprint_bytes) }))).lock().unwrap().as_ref().unwrap())))))))));
    println!("{}", format!("{}", format_slice(&Arc::new(Mutex::new(Some(Vec::from(__go_string_bytes(&(*Arc::new(Mutex::new(Some({ let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = (*invalid.lock().unwrap().as_ref().unwrap()).clone(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_sprint_bytes.push(b'\n'); __go_string_from_bytes(__go_sprint_bytes) }))).lock().unwrap().as_ref().unwrap())))))))));
    println!("{}", format!("{}", format_slice(&Arc::new(Mutex::new(Some(Vec::from(__go_string_bytes(&(*Arc::new(Mutex::new(Some({ let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = format!("{}", 1); __go_sprint_bytes.extend(__go_sprint_arg_0.into_bytes()); __go_sprint_bytes.push(b' '); let __go_sprint_arg_1 = format!("{}", 2); __go_sprint_bytes.extend(__go_sprint_arg_1.into_bytes()); let __go_sprint_arg_2 = (*invalid.lock().unwrap().as_ref().unwrap()).clone(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_2)); __go_string_from_bytes(__go_sprint_bytes) }))).lock().unwrap().as_ref().unwrap())))))))));
    println!("{}", format!("{}", format_slice(&Arc::new(Mutex::new(Some(Vec::from(__go_string_bytes(&(*Arc::new(Mutex::new(Some({ let mut __go_sprintf_bytes = std::vec::Vec::<u8>::new(); __go_sprintf_bytes.extend_from_slice(&[60u8]); let __go_sprintf_arg_0 = (*invalid.lock().unwrap().as_ref().unwrap()).clone(); __go_sprintf_bytes.extend(__go_string_bytes(&__go_sprintf_arg_0)); __go_sprintf_bytes.extend_from_slice(&[62u8]); __go_string_from_bytes(__go_sprintf_bytes) }))).lock().unwrap().as_ref().unwrap())))))))));
    println!("{}", format!("{}", format_slice(&Arc::new(Mutex::new(Some(Vec::from(__go_string_bytes(&(*Arc::new(Mutex::new(Some({ let mut __go_sprintf_bytes = std::vec::Vec::<u8>::new(); let __go_sprintf_arg_0 = (*invalid.lock().unwrap().as_ref().unwrap()).clone(); __go_sprintf_bytes.extend(__go_string_bytes(&__go_sprintf_arg_0)); __go_sprintf_bytes.extend_from_slice(&[37u8, 58u8]); let __go_sprintf_arg_1 = "z".to_string(); __go_sprintf_bytes.extend(__go_string_bytes(&__go_sprintf_arg_1)); __go_string_from_bytes(__go_sprintf_bytes) }))).lock().unwrap().as_ref().unwrap())))))))));
    println!("{}", format!("{}", format_slice(&Arc::new(Mutex::new(Some(Vec::from(__go_string_bytes(&(*Arc::new(Mutex::new(Some({ let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = { let __go_string_method_receiver = stringerText(Arc::new(Mutex::new(Some((*invalid.lock().unwrap().as_ref().unwrap()).clone())))); let __go_string_method_result = __go_string_method_receiver.string(); let __go_string_method_text = { let __go_string_method_guard = __go_string_method_result.lock().unwrap(); (*__go_string_method_guard.as_ref().unwrap()).clone() }; __go_string_method_text }; __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_string_from_bytes(__go_sprint_bytes) }))).lock().unwrap().as_ref().unwrap())))))))));
    println!("{}", format!("{}", format_slice(&Arc::new(Mutex::new(Some(Vec::from(__go_string_bytes(&(*Arc::new(Mutex::new(Some({ let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = { let __go_string_method_receiver = errorText(Arc::new(Mutex::new(Some((*invalid.lock().unwrap().as_ref().unwrap()).clone())))); let __go_string_method_result = __go_string_method_receiver.error(); let __go_string_method_text = { let __go_string_method_guard = __go_string_method_result.lock().unwrap(); (*__go_string_method_guard.as_ref().unwrap()).clone() }; __go_string_method_text }; __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_sprint_bytes.push(b'\n'); __go_string_from_bytes(__go_sprint_bytes) }))).lock().unwrap().as_ref().unwrap())))))))));
    println!("{}", format!("{}", format_slice(&Arc::new(Mutex::new(Some(Vec::from(__go_string_bytes(&(*Arc::new(Mutex::new(Some({ let mut __go_sprintf_bytes = std::vec::Vec::<u8>::new(); __go_sprintf_bytes.extend_from_slice(&[60u8]); let __go_sprintf_arg_0 = { let __go_string_method_receiver = stringerText(Arc::new(Mutex::new(Some((*invalid.lock().unwrap().as_ref().unwrap()).clone())))); let __go_string_method_result = __go_string_method_receiver.string(); let __go_string_method_text = { let __go_string_method_guard = __go_string_method_result.lock().unwrap(); (*__go_string_method_guard.as_ref().unwrap()).clone() }; __go_string_method_text }; __go_sprintf_bytes.extend(__go_string_bytes(&__go_sprintf_arg_0)); __go_sprintf_bytes.extend_from_slice(&[62u8]); __go_string_from_bytes(__go_sprintf_bytes) }))).lock().unwrap().as_ref().unwrap())))))))));
    println!("{}", format!("{}", format_slice(&Arc::new(Mutex::new(Some(Vec::from(__go_string_bytes(&(*Arc::new(Mutex::new(Some({ let mut __go_sprintf_bytes = std::vec::Vec::<u8>::new(); __go_sprintf_bytes.extend_from_slice(&[60u8]); let __go_sprintf_arg_0 = { let __go_string_method_receiver = errorText(Arc::new(Mutex::new(Some((*invalid.lock().unwrap().as_ref().unwrap()).clone())))); let __go_string_method_result = __go_string_method_receiver.error(); let __go_string_method_text = { let __go_string_method_guard = __go_string_method_result.lock().unwrap(); (*__go_string_method_guard.as_ref().unwrap()).clone() }; __go_string_method_text }; __go_sprintf_bytes.extend(__go_string_bytes(&__go_sprintf_arg_0)); __go_sprintf_bytes.extend_from_slice(&[62u8]); __go_string_from_bytes(__go_sprintf_bytes) }))).lock().unwrap().as_ref().unwrap())))))))));
}

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
