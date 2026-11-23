#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
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

pub mod os {
    use super::*;
    type GoError = Arc<Mutex<Option<std::sync::Arc<dyn std::error::Error + Send + Sync>>>>;

    fn no_error() -> GoError {
        Arc::new(Mutex::new(None))
    }

    fn io_error(err: std::io::Error) -> GoError {
        Arc::new(Mutex::new(Some(std::sync::Arc::new(err) as std::sync::Arc<dyn std::error::Error + Send + Sync>)))
    }

    fn go_path(path: String) -> std::path::PathBuf {
        #[cfg(unix)]
        {
            use std::os::unix::ffi::OsStringExt;
            std::path::PathBuf::from(std::ffi::OsString::from_vec(__go_string_bytes(&path)))
        }
        #[cfg(not(unix))]
        {
            std::path::PathBuf::from(path)
        }
    }

    pub fn setenv<T0: 'static, T1: 'static>(_arg0: T0, _arg1: T1) -> Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> {
        let key = if let Some(v) = (&_arg0 as &dyn Any).downcast_ref::<String>() {
            v.clone()
        } else if let Some(v) = (&_arg0 as &dyn Any).downcast_ref::<Arc<Mutex<Option<String>>>>() {
            v.lock().unwrap().as_ref().cloned().unwrap_or_default()
        } else {
            panic!("os environment bridge: expected string argument")
        };
        let value = if let Some(v) = (&_arg1 as &dyn Any).downcast_ref::<String>() {
            v.clone()
        } else if let Some(v) = (&_arg1 as &dyn Any).downcast_ref::<Arc<Mutex<Option<String>>>>() {
            v.lock().unwrap().as_ref().cloned().unwrap_or_default()
        } else {
            panic!("os environment bridge: expected string argument")
        };
        let key_bytes = __go_string_bytes(&key);
        let value_bytes = __go_string_bytes(&value);
        if key_bytes.is_empty() || key_bytes.contains(&b'=') || key_bytes.contains(&0) || value_bytes.contains(&0) {
            return io_error(std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid environment variable"));
        }
        #[cfg(unix)]
        {
            use std::os::unix::ffi::OsStringExt;
            std::env::set_var(std::ffi::OsString::from_vec(key_bytes), std::ffi::OsString::from_vec(value_bytes));
        }
        #[cfg(not(unix))]
        std::env::set_var(key, value);
        no_error()
    }

    pub fn unsetenv<T0: 'static>(_arg0: T0) -> Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> {
        let key = if let Some(v) = (&_arg0 as &dyn Any).downcast_ref::<String>() {
            v.clone()
        } else if let Some(v) = (&_arg0 as &dyn Any).downcast_ref::<Arc<Mutex<Option<String>>>>() {
            v.lock().unwrap().as_ref().cloned().unwrap_or_default()
        } else {
            panic!("os environment bridge: expected string argument")
        };
        let key_bytes = __go_string_bytes(&key);
        if key_bytes.is_empty() || key_bytes.contains(&b'=') || key_bytes.contains(&0) {
            return no_error();
        }
        #[cfg(unix)]
        {
            use std::os::unix::ffi::OsStringExt;
            std::env::remove_var(std::ffi::OsString::from_vec(key_bytes));
        }
        #[cfg(not(unix))]
        std::env::remove_var(key);
        no_error()
    }

    pub fn user_cache_dir() -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        #[cfg(unix)]
        let read_env = |key: &str| {
            use std::os::unix::ffi::OsStringExt;
            std::env::var_os(key).map(|value| __go_string_from_bytes(value.into_vec()))
        };
        #[cfg(not(unix))]
        let read_env = |key: &str| std::env::var(key).ok();
        let nonempty = |value: Option<String>| value.filter(|value| !value.is_empty());

        #[cfg(windows)]
        let result = nonempty(read_env("LocalAppData"))
            .ok_or("%LocalAppData% is not defined");

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        let result = nonempty(read_env("HOME"))
            .map(|home| __go_string_concat(&home, "/Library/Caches"))
            .ok_or("$HOME is not defined");

        #[cfg(all(unix, not(any(target_os = "macos", target_os = "ios"))))]
        let result = match nonempty(read_env("XDG_CACHE_HOME")) {
            Some(dir) => {
                use std::os::unix::ffi::OsStringExt;
                let path = std::path::PathBuf::from(std::ffi::OsString::from_vec(__go_string_bytes(&dir)));
                if path.is_absolute() {
                    Ok(dir)
                } else {
                    Err("path in $XDG_CACHE_HOME is relative")
                }
            }
            None => nonempty(read_env("HOME"))
                .map(|home| __go_string_concat(&home, "/.cache"))
                .ok_or("neither $XDG_CACHE_HOME nor $HOME are defined"),
        };

        #[cfg(not(any(unix, windows)))]
        let result = nonempty(read_env("HOME"))
            .map(|home| __go_string_concat(&home, "/.cache"))
            .ok_or("$HOME is not defined");

        match result {
            Ok(value) => (Arc::new(Mutex::new(Some::<String>(value))), no_error()),
            Err(message) => (Arc::new(Mutex::new(Some::<String>(String::new()))), io_error(std::io::Error::new(std::io::ErrorKind::NotFound, message))),
        }
    }
}


fn main() {
    let mut ready = GoChannel::<AnonymousStruct1>::new();
    let ready_thread = ready.clone(); std::thread::spawn(move || {
        ready_thread.close();;;
    });
    ready.recv().unwrap_or_default();

    let mut home = Arc::new(Mutex::new(Some("/tmp/go2rust-cache-home".to_string())));
    let mut unixCache = Arc::new(Mutex::new(Some(__go_string_concat(&{ let __v = (*home.lock().unwrap().as_ref().unwrap()).clone(); __v }, &"/.cache".to_string()))));
    let mut darwinCache = Arc::new(Mutex::new(Some(__go_string_concat(&{ let __v = (*home.lock().unwrap().as_ref().unwrap()).clone(); __v }, &"/Library/Caches".to_string()))));
    let mut setupOK = Arc::new(Mutex::new(Some({ let __nil_result = (*os::setenv("HOME".to_string(), { let __arg_holder = home.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }).lock().unwrap()).is_none(); __nil_result } && { let __nil_result = (*os::setenv("LocalAppData".to_string(), { let __arg_holder = unixCache.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }).lock().unwrap()).is_none(); __nil_result } && { let __nil_result = (*os::unsetenv("XDG_CACHE_HOME".to_string()).lock().unwrap()).is_none(); __nil_result })));
    let (mut ordinary, mut ordinaryErr) = os::user_cache_dir();
    let mut ordinaryOK = Arc::new(Mutex::new(Some({ let __nil_result = (*ordinaryErr.lock().unwrap()).is_none(); __nil_result } && ({ let __left = __go_string_from_bytes(__go_string_bytes(&{ let __v = (*ordinary.lock().unwrap().as_ref().unwrap()).clone(); __v })); let __right = __go_string_from_bytes(__go_string_bytes(&{ let __v = (*unixCache.lock().unwrap().as_ref().unwrap()).clone(); __v })); __go_string_cmp(&__left, &__right) == std::cmp::Ordering::Equal } || { let __left = __go_string_from_bytes(__go_string_bytes(&{ let __v = (*ordinary.lock().unwrap().as_ref().unwrap()).clone(); __v })); let __right = __go_string_from_bytes(__go_string_bytes(&{ let __v = (*darwinCache.lock().unwrap().as_ref().unwrap()).clone(); __v })); __go_string_cmp(&__left, &__right) == std::cmp::Ordering::Equal }))));

    { let new_val = { let __v = (*setupOK.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __nil_result = (*os::unsetenv("HOME".to_string()).lock().unwrap()).is_none(); __nil_result } && { let __nil_result = (*os::unsetenv("LocalAppData".to_string()).lock().unwrap()).is_none(); __nil_result }; *setupOK.lock().unwrap() = Some(new_val); };
    let (mut missing, mut missingErr) = os::user_cache_dir();
    let mut missingOK = Arc::new(Mutex::new(Some({ let __left = __go_string_from_bytes(__go_string_bytes(&{ let __v = (*missing.lock().unwrap().as_ref().unwrap()).clone(); __v })); let __right = __go_string_from_bytes(__go_string_bytes(&"".to_string())); __go_string_cmp(&__left, &__right) == std::cmp::Ordering::Equal } && { let __nil_result = (*missingErr.lock().unwrap()).is_some(); __nil_result })));

    let mut raw = Arc::new(Mutex::new(Some(__go_string_from_bytes(std::vec::Vec::from((*Arc::new(Mutex::new(Some(vec![('/' as u8), ('t' as u8), ('m' as u8), ('p' as u8), ('/' as u8), 0xfa as u8, ('C' as u8)]))).lock().unwrap().as_ref().unwrap()).clone())))));
    { let new_val = {
        let __go_cond_0 = {
            let __go_cond_1 = {
                let __go_cond_2 = { let __v = (*setupOK.lock().unwrap().as_ref().unwrap()).clone(); __v };
                if __go_cond_2 {
                    let __go_cond_3 = { let __nil_result = (*os::setenv("HOME".to_string(), { let __arg_holder = home.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }).lock().unwrap()).is_none(); __nil_result };
                    __go_cond_3
                } else {
                    false
                }
            };
            if __go_cond_1 {
                let __go_cond_4 = { let __nil_result = (*os::setenv("LocalAppData".to_string(), { let __arg_holder = unixCache.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }).lock().unwrap()).is_none(); __nil_result };
                __go_cond_4
            } else {
                false
            }
        };
        if __go_cond_0 {
            let __go_cond_5 = { let __nil_result = (*os::setenv("XDG_CACHE_HOME".to_string(), { let __arg_holder = raw.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }).lock().unwrap()).is_none(); __nil_result };
            __go_cond_5
        } else {
            false
        }
    }; *setupOK.lock().unwrap() = Some(new_val); };
    let (mut rawCache, mut rawErr) = os::user_cache_dir();
    let mut rawOK = Arc::new(Mutex::new(Some({
        let __go_cond_0 = { let __nil_result = (*rawErr.lock().unwrap()).is_none(); __nil_result };
        if __go_cond_0 {
            let __go_cond_1 = {
                let __go_cond_2 = {
                    let __go_cond_3 = { let __left = __go_string_from_bytes(__go_string_bytes(&{ let __v = (*rawCache.lock().unwrap().as_ref().unwrap()).clone(); __v })); let __right = __go_string_from_bytes(__go_string_bytes(&{ let __v = (*raw.lock().unwrap().as_ref().unwrap()).clone(); __v })); __go_string_cmp(&__left, &__right) == std::cmp::Ordering::Equal };
                    if __go_cond_3 {
                        true
                    } else {
                        let __go_cond_4 = { let __left = __go_string_from_bytes(__go_string_bytes(&{ let __v = (*rawCache.lock().unwrap().as_ref().unwrap()).clone(); __v })); let __right = __go_string_from_bytes(__go_string_bytes(&{ let __v = (*unixCache.lock().unwrap().as_ref().unwrap()).clone(); __v })); __go_string_cmp(&__left, &__right) == std::cmp::Ordering::Equal };
                        __go_cond_4
                    }
                };
                if __go_cond_2 {
                    true
                } else {
                    let __go_cond_5 = { let __left = __go_string_from_bytes(__go_string_bytes(&{ let __v = (*rawCache.lock().unwrap().as_ref().unwrap()).clone(); __v })); let __right = __go_string_from_bytes(__go_string_bytes(&{ let __v = (*darwinCache.lock().unwrap().as_ref().unwrap()).clone(); __v })); __go_string_cmp(&__left, &__right) == std::cmp::Ordering::Equal };
                    __go_cond_5
                }
            };
            __go_cond_1
        } else {
            false
        }
    })));

    { let new_val = { let __v = (*setupOK.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __nil_result = (*os::setenv("XDG_CACHE_HOME".to_string(), "relative/cache".to_string()).lock().unwrap()).is_none(); __nil_result }; *setupOK.lock().unwrap() = Some(new_val); };
    let (mut relative, mut relativeErr) = os::user_cache_dir();
    let mut relativeOK = Arc::new(Mutex::new(Some({
        let __go_cond_0 = {
            let __go_cond_1 = { let __left = __go_string_from_bytes(__go_string_bytes(&{ let __v = (*relative.lock().unwrap().as_ref().unwrap()).clone(); __v })); let __right = __go_string_from_bytes(__go_string_bytes(&"".to_string())); __go_string_cmp(&__left, &__right) == std::cmp::Ordering::Equal };
            if __go_cond_1 {
                let __go_cond_2 = { let __nil_result = (*relativeErr.lock().unwrap()).is_some(); __nil_result };
                __go_cond_2
            } else {
                false
            }
        };
        if __go_cond_0 {
            true
        } else {
            let __go_cond_3 = {
                let __go_cond_4 = { let __nil_result = (*relativeErr.lock().unwrap()).is_none(); __nil_result };
                if __go_cond_4 {
                    let __go_cond_5 = {
                        let __go_cond_6 = { let __left = __go_string_from_bytes(__go_string_bytes(&{ let __v = (*relative.lock().unwrap().as_ref().unwrap()).clone(); __v })); let __right = __go_string_from_bytes(__go_string_bytes(&{ let __v = (*unixCache.lock().unwrap().as_ref().unwrap()).clone(); __v })); __go_string_cmp(&__left, &__right) == std::cmp::Ordering::Equal };
                        if __go_cond_6 {
                            true
                        } else {
                            let __go_cond_7 = { let __left = __go_string_from_bytes(__go_string_bytes(&{ let __v = (*relative.lock().unwrap().as_ref().unwrap()).clone(); __v })); let __right = __go_string_from_bytes(__go_string_bytes(&{ let __v = (*darwinCache.lock().unwrap().as_ref().unwrap()).clone(); __v })); __go_string_cmp(&__left, &__right) == std::cmp::Ordering::Equal };
                            __go_cond_7
                        }
                    };
                    __go_cond_5
                } else {
                    false
                }
            };
            __go_cond_3
        }
    })));

    println!("{} {} {} {} {}", format!("{}", { let __v = (*setupOK.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*ordinaryOK.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*missingOK.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*rawOK.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*relativeOK.lock().unwrap().as_ref().unwrap()).clone(); __v }));
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
