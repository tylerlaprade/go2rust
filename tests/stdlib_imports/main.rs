#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use std::cell::{RefCell};
use std::error::Error as StdError;
use std::fmt::{Display};
use std::rc::{Rc};

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

fn go_strconv_format_int(value: i64, base: i32) -> String {
    if base == 10 {
        return value.to_string();
    }
    if !(2..=36).contains(&base) {
        return value.to_string();
    }

    let negative = value < 0;
    let mut n = if negative {
        value.wrapping_neg() as u64
    } else {
        value as u64
    };
    let base = base as u64;
    let digits = b"0123456789abcdefghijklmnopqrstuvwxyz";
    let mut out = Vec::new();
    if n == 0 {
        out.push(b'0');
    }
    while n > 0 {
        out.push(digits[(n % base) as usize]);
        n /= base;
    }
    if negative {
        out.push(b'-');
    }
    out.reverse();
    __go_string_from_bytes(out.to_vec())
}

fn go_strconv_format_float(value: f64, fmt: char, precision: i32) -> String {
    let precision = if precision < 0 { 6 } else { precision as usize };
    match fmt {
        'e' => format!("{:.*e}", precision, value),
        'E' => format!("{:.*E}", precision, value),
        'f' => format!("{:.*}", precision, value),
        'g' | 'G' => {
            if precision == 0 {
                format!("{:.0}", value)
            } else {
                format!("{:.*}", precision, value)
            }
        }
        _ => value.to_string(),
    }
}

fn __go_collect_os_args() -> Vec<String> {
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStringExt;
        std::env::args_os()
            .map(|arg| __go_string_from_bytes(arg.into_vec()))
            .collect::<Vec<String>>()
    }
    #[cfg(not(unix))]
    {
        std::env::args().collect::<Vec<String>>()
    }
}

thread_local! {
    static __GO_OS_ARGS: std::rc::Rc<std::cell::RefCell<Option<Vec<String>>>> =
        std::rc::Rc::new(std::cell::RefCell::new(Some(__go_collect_os_args())));
}

fn go_os_args() -> std::rc::Rc<std::cell::RefCell<Option<Vec<String>>>> {
    __GO_OS_ARGS.with(|args| args.clone())
}

#[derive(Clone, Debug, Default)]
struct GoTime {
    seconds: i64,
    nanos: i32,
}

fn go_time_civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = mp + if mp < 10 { 3 } else { -9 };
    let year = y + if m <= 2 { 1 } else { 0 };
    (year, m as u32, d as u32)
}

impl GoTime {
    fn now() -> Self {
        let duration = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();
        GoTime {
            seconds: duration.as_secs() as i64,
            nanos: duration.subsec_nanos() as i32,
        }
    }

    fn from_unix(seconds: i64, nanos: i64) -> Self {
        let seconds = seconds + nanos.div_euclid(1_000_000_000);
        let nanos = nanos.rem_euclid(1_000_000_000);
        GoTime {
            seconds,
            nanos: nanos as i32,
        }
    }

    fn add(&self, duration: Rc<RefCell<Option<std::time::Duration>>>) -> Rc<RefCell<Option<GoTime>>> {
        let duration = *duration.borrow().as_ref().unwrap();
        Rc::new(RefCell::new(Some(GoTime::from_unix(
            self.seconds + duration.as_secs() as i64,
            self.nanos as i64 + duration.subsec_nanos() as i64,
        ))))
    }

    fn u_t_c(&self) -> Rc<RefCell<Option<GoTime>>> {
        Rc::new(RefCell::new(Some(self.clone())))
    }

    fn unix(&self) -> i64 {
        self.seconds
    }

    fn unix_nano(&self) -> i64 {
        self.seconds * 1_000_000_000 + self.nanos as i64
    }

    fn is_zero(&self) -> bool {
        self.seconds == 0 && self.nanos == 0
    }

    fn format(&self, _layout: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(self.to_string())))
    }
}

impl std::fmt::Display for GoTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let days = self.seconds.div_euclid(86_400);
        let secs_of_day = self.seconds.rem_euclid(86_400);
        let (year, month, day) = go_time_civil_from_days(days);
        let hour = secs_of_day / 3_600;
        let minute = (secs_of_day % 3_600) / 60;
        let second = secs_of_day % 60;
        if self.nanos == 0 {
            write!(
                f,
                "{:04}-{:02}-{:02} {:02}:{:02}:{:02} +0000 UTC",
                year, month, day, hour, minute, second
            )
        } else {
            let mut fraction = format!("{:09}", self.nanos);
            while fraction.ends_with('0') {
                fraction.pop();
            }
            write!(
                f,
                "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{} +0000 UTC",
                year, month, day, hour, minute, second, fraction
            )
        }
    }
}

fn main() {
        // Testing multiple standard library imports
    { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = "=== Testing multiple stdlib imports ===".to_string(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_sprint_bytes.push(b'\n'); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Rc::new(RefCell::new(None::<Rc<dyn StdError>>))), Err(e) => (0, Rc::new(RefCell::new(Some::<Rc<dyn StdError>>(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(e)))))) } };

        // strings package
    { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = "\n--- strings package ---".to_string(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_sprint_bytes.push(b'\n'); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Rc::new(RefCell::new(None::<Rc<dyn StdError>>))), Err(e) => (0, Rc::new(RefCell::new(Some::<Rc<dyn StdError>>(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(e)))))) } };
    let mut upper = Rc::new(RefCell::new(Some({ let __s = "hello world".to_string(); let __s_bytes = __go_string_bytes(&__s); let __runes = __go_string_range(&__s); let mut __out = Vec::new(); for __i in 0..__runes.len() { let __start = __runes[__i].0; let __end = if __i + 1 < __runes.len() { __runes[__i + 1].0 } else { __s_bytes.len() }; if let Ok(__valid) = std::str::from_utf8(&__s_bytes[__start..__end]) { let __mapped = __valid.to_uppercase(); __out.extend_from_slice(__mapped.as_bytes()); } else { __out.extend_from_slice("\u{fffd}".as_bytes()); } } __go_string_from_bytes(__out) })));
    { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = "Upper:".to_string(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_sprint_bytes.push(b' '); let __go_sprint_arg_1 = (*upper.borrow().as_ref().unwrap()).clone(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_1)); __go_sprint_bytes.push(b'\n'); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Rc::new(RefCell::new(None::<Rc<dyn StdError>>))), Err(e) => (0, Rc::new(RefCell::new(Some::<Rc<dyn StdError>>(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(e)))))) } };

    let mut lower = Rc::new(RefCell::new(Some({ let __s = "HELLO WORLD".to_string(); let __s_bytes = __go_string_bytes(&__s); let __runes = __go_string_range(&__s); let mut __out = Vec::new(); for __i in 0..__runes.len() { let __start = __runes[__i].0; let __end = if __i + 1 < __runes.len() { __runes[__i + 1].0 } else { __s_bytes.len() }; if let Ok(__valid) = std::str::from_utf8(&__s_bytes[__start..__end]) { let __mapped = __valid.to_lowercase(); __out.extend_from_slice(__mapped.as_bytes()); } else { __out.extend_from_slice("\u{fffd}".as_bytes()); } } __go_string_from_bytes(__out) })));
    { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = "Lower:".to_string(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_sprint_bytes.push(b' '); let __go_sprint_arg_1 = (*lower.borrow().as_ref().unwrap()).clone(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_1)); __go_sprint_bytes.push(b'\n'); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Rc::new(RefCell::new(None::<Rc<dyn StdError>>))), Err(e) => (0, Rc::new(RefCell::new(Some::<Rc<dyn StdError>>(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(e)))))) } };

    let mut trimmed = Rc::new(RefCell::new(Some({ let __s = "  hello  ".to_string(); let __s_bytes = __go_string_bytes(&__s); let __runes = __go_string_range(&__s); let mut __first = 0usize; while __first < __runes.len() && __runes[__first].1.is_whitespace() { __first += 1; } let __start = if __first < __runes.len() { __runes[__first].0 } else { __s_bytes.len() }; let mut __last = __runes.len(); while __last > __first && __runes[__last - 1].1.is_whitespace() { __last -= 1; } let __end = if __last < __runes.len() { __runes[__last].0 } else { __s_bytes.len() }; __go_string_from_bytes(__s_bytes[__start..__end].to_vec()) })));
    { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = "Trimmed:".to_string(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_sprint_bytes.push(b' '); let __go_sprint_arg_1 = (*trimmed.borrow().as_ref().unwrap()).clone(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_1)); __go_sprint_bytes.push(b'\n'); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Rc::new(RefCell::new(None::<Rc<dyn StdError>>))), Err(e) => (0, Rc::new(RefCell::new(Some::<Rc<dyn StdError>>(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(e)))))) } };

    let mut split = Rc::new(RefCell::new(Some({ let __s = "a,b,c".to_string(); let __sep = ",".to_string(); let __s_bytes = __go_string_bytes(&__s); let __sep_bytes = __go_string_bytes(&__sep); let mut __parts = Vec::new(); if __sep_bytes.is_empty() { let __runes = __go_string_range(&__s); for __i in 0..__runes.len() { let __start = __runes[__i].0; let __end = if __i + 1 < __runes.len() { __runes[__i + 1].0 } else { __s_bytes.len() }; __parts.push(__go_string_from_bytes(__s_bytes[__start..__end].to_vec())); } } else { let mut __start = 0usize; loop { let __found = __s_bytes[__start..].windows(__sep_bytes.len()).position(|__window| __window == __sep_bytes.as_slice()); if let Some(__relative) = __found { let __idx = __start + __relative; __parts.push(__go_string_from_bytes(__s_bytes[__start..__idx].to_vec())); __start = __idx + __sep_bytes.len(); } else { __parts.push(__go_string_from_bytes(__s_bytes[__start..].to_vec())); break; } } } __parts })));
    let mut split = { let __slice_header = GoSliceHeader::go_slice_header(&split); Rc::new(RefCell::new(__slice_header)) };
    { let __s = format!("Split: {}\n", format_slice(&split)); let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Rc::new(RefCell::new(None::<Rc<dyn StdError>>))), Err(e) => (0, Rc::new(RefCell::new(Some::<Rc<dyn StdError>>(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(e)))))) } };

        // strconv package
    { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = "\n--- strconv package ---".to_string(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_sprint_bytes.push(b'\n'); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Rc::new(RefCell::new(None::<Rc<dyn StdError>>))), Err(e) => (0, Rc::new(RefCell::new(Some::<Rc<dyn StdError>>(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(e)))))) } };
    let mut num = Rc::new(RefCell::new(Some(42i64)));
    let mut str = Rc::new(RefCell::new(Some(((*num.borrow().as_ref().unwrap())).to_string())));
    { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = "Number as string:".to_string(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_sprint_bytes.push(b' '); let __go_sprint_arg_1 = (*str.borrow().as_ref().unwrap()).clone(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_1)); __go_sprint_bytes.push(b'\n'); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Rc::new(RefCell::new(None::<Rc<dyn StdError>>))), Err(e) => (0, Rc::new(RefCell::new(Some::<Rc<dyn StdError>>(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(e)))))) } };

    let (mut parsed, mut err) = { let __atoi_input = "123".to_string(); match __atoi_input.parse::<i64>() { Ok(n) => (n, Rc::new(RefCell::new(None))), Err(_) => (0 as i64, Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(format!("strconv.Atoi: parsing \"{}\": invalid syntax", __atoi_input))))))) } };
    if { let __nil_result = (*err.borrow()).is_some(); __nil_result } {
        { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = "Parse error:".to_string(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_sprint_bytes.push(b' '); let __go_sprint_arg_1 = format!("{}", { let __go_print_handle = err.clone(); let __go_print_value = { let __go_guard = __go_print_handle.borrow(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }); __go_sprint_bytes.extend(__go_sprint_arg_1.into_bytes()); __go_sprint_bytes.push(b'\n'); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Rc::new(RefCell::new(None::<Rc<dyn StdError>>))), Err(e) => (0, Rc::new(RefCell::new(Some::<Rc<dyn StdError>>(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(e)))))) } };
    } else {
        { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = "Parsed number:".to_string(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_sprint_bytes.push(b' '); let __go_sprint_arg_1 = format!("{}", parsed); __go_sprint_bytes.extend(__go_sprint_arg_1.into_bytes()); __go_sprint_bytes.push(b'\n'); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Rc::new(RefCell::new(None::<Rc<dyn StdError>>))), Err(e) => (0, Rc::new(RefCell::new(Some::<Rc<dyn StdError>>(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(e)))))) } };
    }

    let mut floatStr = Rc::new(RefCell::new(Some(go_strconv_format_float(3.14159 as f64, char::from_u32((('f' as u8)) as u32).unwrap_or('f'), 2 as i32))));
    { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = "Float as string:".to_string(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_sprint_bytes.push(b' '); let __go_sprint_arg_1 = (*floatStr.borrow().as_ref().unwrap()).clone(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_1)); __go_sprint_bytes.push(b'\n'); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Rc::new(RefCell::new(None::<Rc<dyn StdError>>))), Err(e) => (0, Rc::new(RefCell::new(Some::<Rc<dyn StdError>>(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(e)))))) } };

        // math package
    { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = "\n--- math package ---".to_string(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_sprint_bytes.push(b'\n'); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Rc::new(RefCell::new(None::<Rc<dyn StdError>>))), Err(e) => (0, Rc::new(RefCell::new(Some::<Rc<dyn StdError>>(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(e)))))) } };
    { let __s = format!("Pi: {:.6}\n", std::f64::consts::PI); let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Rc::new(RefCell::new(None::<Rc<dyn StdError>>))), Err(e) => (0, Rc::new(RefCell::new(Some::<Rc<dyn StdError>>(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(e)))))) } };
    { let __s = format!("E: {:.6}\n", std::f64::consts::E); let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Rc::new(RefCell::new(None::<Rc<dyn StdError>>))), Err(e) => (0, Rc::new(RefCell::new(Some::<Rc<dyn StdError>>(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(e)))))) } };
    { let __s = format!("Sqrt(16): {:.2}\n", (*Rc::new(RefCell::new(Some((16.0 as f64).sqrt()))).borrow().as_ref().unwrap())); let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Rc::new(RefCell::new(None::<Rc<dyn StdError>>))), Err(e) => (0, Rc::new(RefCell::new(Some::<Rc<dyn StdError>>(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(e)))))) } };
    { let __s = format!("Pow(2, 8): {:.0}\n", (*Rc::new(RefCell::new(Some((2.0 as f64).powf(8.0 as f64)))).borrow().as_ref().unwrap())); let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Rc::new(RefCell::new(None::<Rc<dyn StdError>>))), Err(e) => (0, Rc::new(RefCell::new(Some::<Rc<dyn StdError>>(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(e)))))) } };
    { let __s = format!("Max(10, 20): {:.0}\n", (*Rc::new(RefCell::new(Some((10.0 as f64).max(20.0 as f64)))).borrow().as_ref().unwrap())); let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Rc::new(RefCell::new(None::<Rc<dyn StdError>>))), Err(e) => (0, Rc::new(RefCell::new(Some::<Rc<dyn StdError>>(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(e)))))) } };
    { let __s = format!("Min(10, 20): {:.0}\n", (*Rc::new(RefCell::new(Some((10.0 as f64).min(20.0 as f64)))).borrow().as_ref().unwrap())); let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Rc::new(RefCell::new(None::<Rc<dyn StdError>>))), Err(e) => (0, Rc::new(RefCell::new(Some::<Rc<dyn StdError>>(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(e)))))) } };

        // time package
    { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = "\n--- time package ---".to_string(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_sprint_bytes.push(b'\n'); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Rc::new(RefCell::new(None::<Rc<dyn StdError>>))), Err(e) => (0, Rc::new(RefCell::new(Some::<Rc<dyn StdError>>(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(e)))))) } };
    let mut localFixed = Rc::new(RefCell::new(Some(GoTime::from_unix(1700000000 as i64, 0 as i64))));
    let mut fixed = (*localFixed.borrow().as_ref().unwrap()).u_t_c();
    { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = "Fixed timestamp:".to_string(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_sprint_bytes.push(b' '); let __go_sprint_arg_1 = format!("{}", (*fixed.borrow().as_ref().unwrap()).unix()); __go_sprint_bytes.extend(__go_sprint_arg_1.into_bytes()); __go_sprint_bytes.push(b'\n'); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Rc::new(RefCell::new(None::<Rc<dyn StdError>>))), Err(e) => (0, Rc::new(RefCell::new(Some::<Rc<dyn StdError>>(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(e)))))) } };
    let mut later = (*fixed.borrow().as_ref().unwrap()).add(Rc::new(RefCell::new(Some(std::time::Duration::from_secs(3600)))));
    { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = "One hour later:".to_string(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_sprint_bytes.push(b' '); let __go_sprint_arg_1 = format!("{}", (*later.borrow().as_ref().unwrap()).unix()); __go_sprint_bytes.extend(__go_sprint_arg_1.into_bytes()); __go_sprint_bytes.push(b'\n'); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Rc::new(RefCell::new(None::<Rc<dyn StdError>>))), Err(e) => (0, Rc::new(RefCell::new(Some::<Rc<dyn StdError>>(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(e)))))) } };

        // os package
    { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = "\n--- os package ---".to_string(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_sprint_bytes.push(b'\n'); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Rc::new(RefCell::new(None::<Rc<dyn StdError>>))), Err(e) => (0, Rc::new(RefCell::new(Some::<Rc<dyn StdError>>(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(e)))))) } };
    { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = "Argument count:".to_string(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_sprint_bytes.push(b' '); let __go_sprint_arg_1 = format!("{}", ({ let __len_target = { let __field = go_os_args().clone(); __field }; let __len_guard = __len_target.borrow(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })); __go_sprint_bytes.extend(__go_sprint_arg_1.into_bytes()); __go_sprint_bytes.push(b'\n'); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Rc::new(RefCell::new(None::<Rc<dyn StdError>>))), Err(e) => (0, Rc::new(RefCell::new(Some::<Rc<dyn StdError>>(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(e)))))) } };

        // Combined usage
    { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = "\n--- Combined usage ---".to_string(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_sprint_bytes.push(b'\n'); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Rc::new(RefCell::new(None::<Rc<dyn StdError>>))), Err(e) => (0, Rc::new(RefCell::new(Some::<Rc<dyn StdError>>(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(e)))))) } };
    let mut timestamp = (*fixed.borrow().as_ref().unwrap()).unix();
    let mut timestampStr = Rc::new(RefCell::new(Some(go_strconv_format_int(timestamp as i64, 10 as i32))));
    let mut message = Rc::new(RefCell::new(Some(__go_string_concat(&"Timestamp: ".to_string(), &(*timestampStr.borrow().as_ref().unwrap())))));
    { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = (*message.borrow().as_ref().unwrap()).clone(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_sprint_bytes.push(b'\n'); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Rc::new(RefCell::new(None::<Rc<dyn StdError>>))), Err(e) => (0, Rc::new(RefCell::new(Some::<Rc<dyn StdError>>(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(e)))))) } };

        // Mathematical calculation with string formatting
    let mut result = Rc::new(RefCell::new(Some(((*Rc::new(RefCell::new(Some((3.0 as f64).powf(2.0 as f64)))).borrow().as_ref().unwrap()) + (*Rc::new(RefCell::new(Some((4.0 as f64).powf(2.0 as f64)))).borrow().as_ref().unwrap()) as f64).sqrt())));
    let mut resultStr = Rc::new(RefCell::new(Some(go_strconv_format_float((*result.borrow().as_ref().unwrap()) as f64, char::from_u32((('f' as u8)) as u32).unwrap_or('f'), 2 as i32))));
    { let __s = { let mut __go_sprintf_bytes = std::vec::Vec::<u8>::new(); __go_sprintf_bytes.extend_from_slice(&[72u8, 121u8, 112u8, 111u8, 116u8, 101u8, 110u8, 117u8, 115u8, 101u8, 32u8, 111u8, 102u8, 32u8, 51u8, 44u8, 52u8, 32u8, 116u8, 114u8, 105u8, 97u8, 110u8, 103u8, 108u8, 101u8, 58u8, 32u8]); let __go_sprintf_arg_0 = (*resultStr.borrow().as_ref().unwrap()).clone(); __go_sprintf_bytes.extend(__go_string_bytes(&__go_sprintf_arg_0)); __go_sprintf_bytes.extend_from_slice(&[10u8]); __go_string_from_bytes(__go_sprintf_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Rc::new(RefCell::new(None::<Rc<dyn StdError>>))), Err(e) => (0, Rc::new(RefCell::new(Some::<Rc<dyn StdError>>(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(e)))))) } };
}