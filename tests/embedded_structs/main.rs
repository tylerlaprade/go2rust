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

#[derive(Debug, Clone)]
pub struct Person {
    pub name: Rc<RefCell<Option<String>>>,
    pub age: Rc<RefCell<Option<i64>>>,
}

impl Person {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.age.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
            age: __go_clone_1_0,
        }
    }
}


impl Default for Person {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            name: __go_default_0_0,
            age: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.age.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct Address {
    pub street: Rc<RefCell<Option<String>>>,
    pub city: Rc<RefCell<Option<String>>>,
    pub state: Rc<RefCell<Option<String>>>,
}

impl Address {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.street.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.city.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.state.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            street: __go_clone_0_0,
            city: __go_clone_1_0,
            state: __go_clone_2_0,
        }
    }
}


impl Default for Address {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_2_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            street: __go_default_0_0,
            city: __go_default_1_0,
            state: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.street.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.city.borrow().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.state.borrow().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


#[derive(Debug, Clone)]
pub struct Employee {
    pub person: Rc<RefCell<Option<Person>>>,
    pub address: Rc<RefCell<Option<Address>>>,
    pub i_d: Rc<RefCell<Option<i64>>>,
    pub salary: Rc<RefCell<Option<f64>>>,
}

impl Employee {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.person.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        let __go_clone_1_0 = { let __guard = self.address.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        let __go_clone_2_0 = { let __guard = self.i_d.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.salary.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            person: __go_clone_0_0,
            address: __go_clone_1_0,
            i_d: __go_clone_2_0,
            salary: __go_clone_3_0,
        }
    }
}


impl Default for Employee {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(Person::default())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(Address::default())));
        let __go_default_2_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_3_0 = Rc::new(RefCell::new(Some(0.0)));
        Self {
            person: __go_default_0_0,
            address: __go_default_1_0,
            i_d: __go_default_2_0,
            salary: __go_default_3_0,
        }
    }
}

impl std::fmt::Display for Employee {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.person.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.address.borrow().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.i_d.borrow().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.salary.borrow().as_ref().unwrap()));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}


#[derive(Debug, Clone)]
pub struct Manager {
    pub employee: Rc<RefCell<Option<Employee>>>,
    pub team: Rc<RefCell<Option<Vec<String>>>>,
}

impl Manager {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.employee.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        let __go_clone_1_0 = self.team.clone();
        Self {
            employee: __go_clone_0_0,
            team: __go_clone_1_0,
        }
    }
}


impl Default for Manager {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(Employee::default())));
        let __go_default_1_0 = Rc::new(RefCell::new(None));
        Self {
            employee: __go_default_0_0,
            team: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Manager {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.employee.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice(&self.team));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


/// Anonymous struct embedding
#[derive(Debug, Clone)]
pub struct CompanyInfo {
    pub founded: Rc<RefCell<Option<i64>>>,
    pub c_e_o: Rc<RefCell<Option<String>>>,
}

impl CompanyInfo {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.founded.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.c_e_o.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            founded: __go_clone_0_0,
            c_e_o: __go_clone_1_0,
        }
    }
}


impl Default for CompanyInfo {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            founded: __go_default_0_0,
            c_e_o: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for CompanyInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.founded.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.c_e_o.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct Company {
    pub name: Rc<RefCell<Option<String>>>,
    pub company_info: Rc<RefCell<Option<CompanyInfo>>>,
}

impl Company {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.company_info.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        Self {
            name: __go_clone_0_0,
            company_info: __go_clone_1_0,
        }
    }
}


impl Default for Company {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(CompanyInfo::default())));
        Self {
            name: __go_default_0_0,
            company_info: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Company {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.company_info.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl Person {
    pub fn greet(&self) {
        print!("Hello, I'm {}\n", (*self.name.borrow().as_ref().unwrap()));
    }

    pub fn get_info(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("{} ({} years old)", (*self.name.borrow().as_ref().unwrap()), (*self.age.borrow().as_ref().unwrap())))))
    }
}

impl Address {
    pub fn full_address(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some({ let mut __go_sprintf_bytes = std::vec::Vec::<u8>::new(); let __go_sprintf_arg_0 = (*self.street.borrow().as_ref().unwrap()).clone(); __go_sprintf_bytes.extend(__go_string_bytes(&__go_sprintf_arg_0)); __go_sprintf_bytes.extend_from_slice(&[44u8, 32u8]); let __go_sprintf_arg_1 = (*self.city.borrow().as_ref().unwrap()).clone(); __go_sprintf_bytes.extend(__go_string_bytes(&__go_sprintf_arg_1)); __go_sprintf_bytes.extend_from_slice(&[44u8, 32u8]); let __go_sprintf_arg_2 = (*self.state.borrow().as_ref().unwrap()).clone(); __go_sprintf_bytes.extend(__go_string_bytes(&__go_sprintf_arg_2)); __go_string_from_bytes(__go_sprintf_bytes) })))
    }
}

impl Employee {
    pub fn work(&self) {
        print!("{} is working (ID: {})\n", (*self.person.borrow().as_ref().unwrap().name.borrow().as_ref().unwrap()), (*self.i_d.borrow().as_ref().unwrap()));
    }

    pub fn full_address(&self) -> Rc<RefCell<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.address.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.full_address()
    }

    pub fn get_info(&self) -> Rc<RefCell<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.person.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.get_info()
    }

    pub fn greet(&self) {
        // Forward to embedded type's method
        let embedded = self.person.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.greet()
    }
}

impl Manager {
    pub fn manage(&self) {
        print!("Manager {} is managing team: {}\n", (*self.employee.borrow().as_ref().unwrap().person.borrow().as_ref().unwrap().name.borrow().as_ref().unwrap()), format_slice(&self.team.clone()));
    }

    pub fn full_address(&self) -> Rc<RefCell<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.employee.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.full_address()
    }

    pub fn get_info(&self) -> Rc<RefCell<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.employee.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.get_info()
    }

    pub fn greet(&self) {
        // Forward to embedded type's method
        let embedded = self.employee.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.greet()
    }

    pub fn work(&self) {
        // Forward to embedded type's method
        let embedded = self.employee.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.work()
    }
}

impl Company {
}

fn main() {
        // Basic embedded struct
    println!("{}", format!("{}", "=== Basic embedded struct ===".to_string()));
    let mut emp = Rc::new(RefCell::new(Some(Employee { person: Rc::new(RefCell::new(Some(Person { name: Rc::new(RefCell::new(Some("Alice".to_string()))), age: Rc::new(RefCell::new(Some(30))), ..Default::default() }))), address: Rc::new(RefCell::new(Some(Address { street: Rc::new(RefCell::new(Some("123 Main St".to_string()))), city: Rc::new(RefCell::new(Some("Anytown".to_string()))), state: Rc::new(RefCell::new(Some("CA".to_string()))), ..Default::default() }))), i_d: Rc::new(RefCell::new(Some(1001))), salary: Rc::new(RefCell::new(Some(75000.0))), ..Default::default() })));

        // Access embedded fields directly
    print!("Name: {}\n", (*(*(*emp.borrow().as_ref().unwrap()).person.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone());
    print!("Age: {}\n", (*(*(*emp.borrow().as_ref().unwrap()).person.borrow().as_ref().unwrap()).age.borrow().as_ref().unwrap()));
    print!("Street: {}\n", (*(*(*emp.borrow().as_ref().unwrap()).address.borrow().as_ref().unwrap()).street.borrow().as_ref().unwrap()).clone());
    print!("ID: {}\n", (*(*emp.borrow().as_ref().unwrap()).i_d.borrow().as_ref().unwrap()));

        // Call embedded methods
    (*emp.borrow().as_ref().unwrap()).greet();
    println!("{} {}", format!("{}", "Info:".to_string()), format!("{}", (*(*emp.borrow().as_ref().unwrap()).get_info().borrow().as_ref().unwrap())));
    println!("{} {}", format!("{}", "Address:".to_string()), format!("{}", (*(*emp.borrow().as_ref().unwrap()).full_address().borrow().as_ref().unwrap())));
    (*emp.borrow().as_ref().unwrap()).work();

        // Nested embedding
    println!("{}", format!("{}", "\n=== Nested embedding ===".to_string()));
    let mut mgr = Rc::new(RefCell::new(Some(Manager { employee: Rc::new(RefCell::new(Some(Employee { person: Rc::new(RefCell::new(Some(Person { name: Rc::new(RefCell::new(Some("Bob".to_string()))), age: Rc::new(RefCell::new(Some(35))), ..Default::default() }))), address: Rc::new(RefCell::new(Some(Address { street: Rc::new(RefCell::new(Some("456 Oak Ave".to_string()))), city: Rc::new(RefCell::new(Some("Somewhere".to_string()))), state: Rc::new(RefCell::new(Some("NY".to_string()))), ..Default::default() }))), i_d: Rc::new(RefCell::new(Some(2001))), salary: Rc::new(RefCell::new(Some(95000.0))), ..Default::default() }))), team: Rc::new(RefCell::new(Some(vec!["Alice".to_string(), "Charlie".to_string(), "Diana".to_string()]))), ..Default::default() })));

        // Access deeply nested fields
    print!("Manager: {}\n", (*(*(*mgr.borrow().as_ref().unwrap()).employee.borrow().as_ref().unwrap().person.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone());
    print!("Manager ID: {}\n", (*(*(*mgr.borrow().as_ref().unwrap()).employee.borrow().as_ref().unwrap()).i_d.borrow().as_ref().unwrap()));
    print!("Manager City: {}\n", (*(*(*mgr.borrow().as_ref().unwrap()).employee.borrow().as_ref().unwrap().address.borrow().as_ref().unwrap()).city.borrow().as_ref().unwrap()).clone());

        // Call methods from all levels
    (*mgr.borrow().as_ref().unwrap()).greet();
    (*mgr.borrow().as_ref().unwrap()).work();
    (*mgr.borrow().as_ref().unwrap()).manage();

        // Anonymous struct embedding
    println!("{}", format!("{}", "\n=== Anonymous struct embedding ===".to_string()));
    let mut company = Rc::new(RefCell::new(Some(Company { name: Rc::new(RefCell::new(Some("TechCorp".to_string()))), company_info: Rc::new(RefCell::new(Some(CompanyInfo::default()))) })));
    { let new_val = 2010; *(*(*company.borrow_mut().as_mut().unwrap()).company_info.borrow_mut().as_mut().unwrap()).founded.borrow_mut() = Some(new_val); };
    { let new_val = "John Doe".to_string(); *(*(*company.borrow_mut().as_mut().unwrap()).company_info.borrow_mut().as_mut().unwrap()).c_e_o.borrow_mut() = Some(new_val); };

    print!("Company: {}\n", (*(*company.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone());
    print!("Founded: {}\n", (*(*(*company.borrow().as_ref().unwrap()).company_info.borrow().as_ref().unwrap()).founded.borrow().as_ref().unwrap()));
    print!("CEO: {}\n", (*(*(*company.borrow().as_ref().unwrap()).company_info.borrow().as_ref().unwrap()).c_e_o.borrow().as_ref().unwrap()).clone());

        // Method promotion
    println!("{}", format!("{}", "\n=== Method promotion ===".to_string()));
    println!("{}", format!("{}", "Employee methods are promoted from Person and Address".to_string()));
    print!("Employee can call: {}\n", (*(*emp.borrow().as_ref().unwrap()).get_info().borrow().as_ref().unwrap()));
    print!("Employee address: {}\n", (*(*emp.borrow().as_ref().unwrap()).full_address().borrow().as_ref().unwrap()));
}