#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use std::any::Any;
use std::cell::{RefCell};
use std::error::Error as StdError;
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
    } else if let Some(v) = value.downcast_ref::<Code>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<NotFound>>>>() {
        match (*v.borrow()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() }
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<QueryError>>>>() {
        match (*v.borrow()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() }
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() {
        match (*v.borrow()).as_ref() { Some(__err) => format!("{}", __err), None => "<nil>".to_string() }
    } else {
        panic!("go2rust: cannot print this value held in an interface: its type has no formatting case yet")
    }
}

std::thread_local! {
    static __GO_SILENT_PANIC_DEPTH: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
}

static __GO_SILENT_PANIC_HOOK_INIT: std::sync::Once = std::sync::Once::new();

fn go_silent_catch_unwind<F, R>(f: F) -> Result<R, Box<dyn std::any::Any + Send>>
where
    F: FnOnce() -> R,
{
    __GO_SILENT_PANIC_HOOK_INIT.call_once(|| {
        let previous = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            let suppressed = __GO_SILENT_PANIC_DEPTH.with(|depth| depth.get() > 0);
            if !suppressed {
                previous(info);
            }
        }));
    });

    struct SilentPanicGuard;
    impl Drop for SilentPanicGuard {
        fn drop(&mut self) {
            __GO_SILENT_PANIC_DEPTH.with(|depth| depth.set(depth.get().saturating_sub(1)));
        }
    }

    __GO_SILENT_PANIC_DEPTH.with(|depth| depth.set(depth.get().saturating_add(1)));
    let _guard = SilentPanicGuard;
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(f))
}

#[derive(Clone, Copy)]
struct GoAnyTypeMetadata {
    pub kind: &'static str,
    pub comparable: bool,
    pub elem_kind: Option<&'static str>,
    pub elem_comparable: bool,
}

fn go_any_type_metadata_registry() -> &'static std::sync::Mutex<std::collections::HashMap<std::any::TypeId, GoAnyTypeMetadata>> {
    static REGISTRY: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<std::any::TypeId, GoAnyTypeMetadata>>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

fn go_any_value_metadata_registry() -> &'static std::sync::Mutex<std::collections::HashMap<usize, GoAnyTypeMetadata>> {
    static REGISTRY: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<usize, GoAnyTypeMetadata>>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

fn go_any_value_metadata_key(value: &dyn Any) -> usize {
    value as *const dyn Any as *const () as usize
}

fn go_register_any_type<T: Any + 'static>(kind: &'static str, comparable: bool) {
    go_any_type_metadata_registry().lock().unwrap().insert(std::any::TypeId::of::<T>(), GoAnyTypeMetadata { kind, comparable, elem_kind: None, elem_comparable: false });
}

fn go_register_any_type_with_elem<T: Any + 'static>(kind: &'static str, comparable: bool, elem_kind: &'static str, elem_comparable: bool) {
    go_any_type_metadata_registry().lock().unwrap().insert(std::any::TypeId::of::<T>(), GoAnyTypeMetadata { kind, comparable, elem_kind: Some(elem_kind), elem_comparable });
}

// Boxes value as itself and records what kind of Go type it is. Every reader
// of an interface value looks for the value's own type in the box, so the
// box must hold nothing else: a wrapper around the value would be a type no
// reader knows.
fn go_box_any_with_metadata<T: Any + 'static>(value: T, kind: &'static str, comparable: bool) -> Box<dyn Any> {
    go_register_any_type::<T>(kind, comparable);
    Box::new(value) as Box<dyn Any>
}

fn go_register_any_value_metadata(value: &dyn Any, kind: &'static str, comparable: bool) {
    go_any_value_metadata_registry().lock().unwrap().insert(go_any_value_metadata_key(value), GoAnyTypeMetadata { kind, comparable, elem_kind: None, elem_comparable: false });
}

fn go_any_type_metadata(value: &dyn Any) -> Option<GoAnyTypeMetadata> {
    go_any_value_metadata_registry().lock().unwrap().get(&go_any_value_metadata_key(value)).copied()
        .or_else(|| go_any_type_metadata_registry().lock().unwrap().get(&value.type_id()).copied())
}

fn __go_type_name(val: &dyn Any) -> &'static str {
    if val.is::<i64>() { return "int" }
    if val.is::<i32>() { return "int32" }
    if val.is::<i8>() { return "int8" }
    if val.is::<i16>() { return "int16" }
    if val.is::<u64>() { return "uint" }
    if val.is::<u32>() { return "uint32" }
    if val.is::<u8>() { return "uint8" }
    if val.is::<u16>() { return "uint16" }
    if val.is::<f64>() { return "float64" }
    if val.is::<f32>() { return "float32" }
    if val.is::<bool>() { return "bool" }
    if val.is::<String>() { return "string" }
    if val.is::<Vec<i64>>() { return "[]int" }
    if val.is::<Vec<i32>>() { return "[]int32" }
    if val.is::<Vec<f64>>() { return "[]float64" }
    if val.is::<Vec<String>>() { return "[]string" }
    if val.is::<Vec<bool>>() { return "[]bool" }
    if val.is::<Code>() { return "main.Code" }
    if val.is::<Rc<RefCell<Option<NotFound>>>>() { return "*main.NotFound" }
    if val.is::<Rc<RefCell<Option<QueryError>>>>() { return "*main.QueryError" }
    std::any::type_name_of_val(val)
}

// An error that wraps others: fmt.Errorf with %w, or errors.Join. many is
// true when it wraps more than one, and then errors.Unwrap does not see it.
#[derive(Debug)]
struct GoWrapError {
    msg: String,
    wrapped: Vec<Rc<dyn StdError>>,
    many: bool,
}

impl std::fmt::Display for GoWrapError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl StdError for GoWrapError {}

fn go_error_wrap(msg: String, wrapped: Vec<Option<Rc<dyn StdError>>>) -> Rc<dyn StdError> {
    let many = wrapped.len() > 1;
    Rc::new(GoWrapError { msg, wrapped: wrapped.into_iter().flatten().collect(), many }) as Rc<dyn StdError>
}

fn go_errors_join(errs: Vec<Option<Rc<dyn StdError>>>) -> Option<Rc<dyn StdError>> {
    let errs: Vec<Rc<dyn StdError>> = errs.into_iter().flatten().collect();
    if errs.is_empty() {
        return None;
    }
    let msg = errs.as_slice().iter().map(|e| e.to_string()).collect::<Vec<_>>().join("\n");
    Some(Rc::new(GoWrapError { msg, wrapped: errs, many: true }) as Rc<dyn StdError>)
}

// Visit err and every error it wraps, depth first, in the order errors.Is and
// errors.As use. Stop before unwrapping the first value accepted by visit.
fn go_error_chain_visit<F>(err: Option<Rc<dyn StdError>>, mut visit: F) -> bool
where
    F: FnMut(&Rc<dyn StdError>) -> bool,
{
    let mut stack: Vec<Rc<dyn StdError>> = err.into_iter().collect();
    while let Some(e) = stack.pop() {
        if visit(&e) {
            return true;
        }
        if let Some(w) = e.downcast_ref::<GoWrapError>() {
            stack.extend(w.wrapped.as_slice().iter().rev().cloned());
        }
        if let Some(__v) = e.downcast_ref::<QueryError>() { if let Some(__c) = (*__v.unwrap().borrow()).clone() { stack.push(__c); } }
        if let Some(__v) = e.downcast_ref::<__go_error_ptr_QueryError>() { if let Some(__c) = (*__go_nil_recv_query_error_unwrap(__v.0.clone()).borrow()).clone() { stack.push(__c); } }

    }
    false
}

fn go_errors_unwrap(err: Option<Rc<dyn StdError>>) -> Option<Rc<dyn StdError>> {
    let err = err?;
    if let Some(w) = err.downcast_ref::<GoWrapError>() {
        return if w.many { None } else { w.wrapped.as_slice().first().cloned() };
    }
    let e = err;
    let mut stack: Vec<Rc<dyn StdError>> = Vec::new();
        if let Some(__v) = e.downcast_ref::<QueryError>() { if let Some(__c) = (*__v.unwrap().borrow()).clone() { stack.push(__c); } }
        if let Some(__v) = e.downcast_ref::<__go_error_ptr_QueryError>() { if let Some(__c) = (*__go_nil_recv_query_error_unwrap(__v.0.clone()).borrow()).clone() { stack.push(__c); } }
    stack.pop()
}

// Two errors are the same error when they point to the same value, as ==
// on two errors is written.
fn go_errors_is(err: Option<Rc<dyn StdError>>, target: Option<Rc<dyn StdError>>) -> bool {
    match (err, target) {
        (None, None) => true,
        (Some(err), Some(target)) => go_error_chain_visit(Some(err), |e| std::ptr::addr_eq(&**e, &*target)),
        _ => false,
    }
}

// Whether value has a String or an Error method. fmt prints such a value as
// the one text the method returns.
fn go_format_is_text(__value: &dyn Any) -> bool {
    false
        || __value.is::<Code>()
        || __value.is::<Rc<RefCell<Option<NotFound>>>>()
        || __value.is::<Rc<RefCell<Option<QueryError>>>>()
        || __value.is::<Rc<RefCell<Option<Rc<dyn StdError>>>>>()
}

// The basic value a newtype such as "type Level int" holds, boxed.
fn go_format_named_basic_value(__value: &dyn Any) -> Option<Box<dyn Any>> {
    if let Some(__v) = __value.downcast_ref::<Code>() {
        return Some(Box::new((*__v.0.borrow().as_ref().unwrap()).clone()));
    }
    None
}

#[allow(unused_imports)]
mod __go_runtime_format {
    use super::{format_any, __go_string_bytes, __go_type_name, go_format_is_text, go_format_named_basic_value};
    use std::any::Any;
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::{Arc, Mutex};
    #[allow(unused_imports)]
    use super::{GoSlice, Vec};

// A verb with its flags, as fmt reads it from a format.
#[derive(Default, Clone, Copy)]
struct GoFormatSpec {
    plus: bool,
    minus: bool,
    sharp: bool,
    space: bool,
    zero: bool,
    width: Option<usize>,
    precision: Option<usize>,
}

// Pads text to the width of spec. A number passes the length of its sign so
// that zero padding goes between the sign and the digits.
fn go_format_pad(spec: &GoFormatSpec, text: String, sign_len: usize, zero_allowed: bool) -> String {
    let width = match spec.width {
        Some(width) => width,
        None => return text,
    };
    let len = text.chars().count();
    if len >= width {
        return text;
    }
    let fill = width - len;
    if spec.minus {
        return text + &" ".repeat(fill);
    }
    if spec.zero && zero_allowed {
        let (sign, digits) = text.split_at(sign_len);
        return format!("{}{}{}", sign, "0".repeat(fill), digits);
    }
    " ".repeat(fill) + &text
}

fn go_format_sign(spec: &GoFormatSpec, negative: bool) -> &'static str {
    if negative {
        "-"
    } else if spec.plus {
        "+"
    } else if spec.space {
        " "
    } else {
        ""
    }
}

// Whether strconv.IsPrint prints c as it is: a letter, mark, number,
// punctuation, symbol or the ASCII space. Rust's standard library has no
// table of Unicode categories, so this knows the control characters, the
// other spaces, the format characters in common use (the soft hyphen, the
// zero-width and direction marks, the byte order mark) and the private-use
// areas. It does not know which code points are unassigned.
fn go_format_is_print(c: char) -> bool {
    if c == ' ' {
        return true;
    }
    if c.is_control() || c.is_whitespace() {
        return false;
    }
    !matches!(c as u32, 0xad | 0x600..=0x605 | 0x61c | 0x6dd | 0x70f | 0x8e2 | 0x180e | 0x200b..=0x200f | 0x202a..=0x202e | 0x2060..=0x206f | 0xfeff | 0xfff9..=0xfffb | 0xe000..=0xf8ff | 0xe0001 | 0xe0020..=0xe007f | 0xf0000..=0xffffd | 0x100000..=0x10fffd)
}

// Writes text the way strconv.Quote writes the inside of a quoted string,
// with quote as the delimiter. ascii is the + flag: every character outside
// ASCII is written as an escape.
fn go_format_escape(out: &mut String, text: &str, quote: char, ascii: bool) {
    for c in text.chars() {
        match c {
            '\u{7}' => out.push_str("\\a"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{b}' => out.push_str("\\v"),
            '\\' => out.push_str("\\\\"),
            c if c == quote => {
                out.push('\\');
                out.push(c);
            }
            c if (c as u32) < 0x20 || c == '\u{7f}' => out.push_str(&format!("\\x{:02x}", c as u32)),
            c if !go_format_is_print(c) || (ascii && !c.is_ascii()) => {
                if (c as u32) < 0x10000 {
                    out.push_str(&format!("\\u{:04x}", c as u32));
                } else {
                    out.push_str(&format!("\\U{:08x}", c as u32));
                }
            }
            c => out.push(c),
        }
    }
}

fn go_format_quote(text: &str, quote: char, ascii: bool) -> String {
    let mut out = String::new();
    out.push(quote);
    go_format_escape(&mut out, text, quote, ascii);
    out.push(quote);
    out
}

fn go_format_integer(spec: &GoFormatSpec, verb: char, value: i128) -> Option<String> {
    let negative = value < 0;
    let magnitude = value.unsigned_abs();
    let mut digits = match verb {
        'd' | 'v' => magnitude.to_string(),
        'b' => format!("{:b}", magnitude),
        'o' | 'O' => format!("{:o}", magnitude),
        'x' => format!("{:x}", magnitude),
        'X' => format!("{:X}", magnitude),
        'c' => {
            let c = u32::try_from(value).ok().and_then(char::from_u32).unwrap_or('\u{fffd}');
            return Some(go_format_pad(spec, c.to_string(), 0, true));
        }
        'q' => {
            let c = u32::try_from(value).ok().and_then(char::from_u32).unwrap_or('\u{fffd}');
            return Some(go_format_pad(spec, go_format_quote(&c.to_string(), '\'', spec.plus), 0, true));
        }
        'U' => {
            // A precision above four asks for more digits.
            let digits = spec.precision.filter(|precision| *precision > 4).unwrap_or(4);
            let mut text = format!("U+{:0digits$X}", value as u64, digits = digits);
            if spec.sharp {
                if let Some(c) = u32::try_from(value).ok().and_then(char::from_u32).filter(|c| go_format_is_print(*c)) {
                    text.push_str(&format!(" '{}'", c));
                }
            }
            return Some(go_format_pad(spec, text, 0, false));
        }
        _ => return None,
    };
    let sign = go_format_sign(spec, negative);
    // A precision and zero padding both ask for a least number of digits.
    // The sign is counted in the width of zero padding; 0x, 0b and 0o are
    // not, they come on top.
    let least_digits = match spec.precision {
        // Zero with a precision of zero prints nothing but the padding.
        Some(0) if magnitude == 0 => return Some(go_format_pad(&GoFormatSpec { zero: false, ..*spec }, String::new(), 0, false)),
        Some(precision) => precision,
        None if spec.zero && !spec.minus => spec.width.unwrap_or(0).saturating_sub(sign.len()),
        None => 0,
    };
    if digits.len() < least_digits {
        digits = "0".repeat(least_digits - digits.len()) + &digits;
    }
    // The 0 that # puts before an octal number is only added to one that
    // does not start with a 0 already. %#O has it as well, after its 0o.
    let octal_zero = if spec.sharp && matches!(verb, 'o' | 'O') && !digits.starts_with('0') { "0" } else { "" };
    let prefix = match verb {
        'b' if spec.sharp => "0b",
        'O' => "0o",
        'x' if spec.sharp => "0x",
        'X' if spec.sharp => "0X",
        _ => "",
    };
    Some(go_format_pad(spec, format!("{}{}{}{}", sign, prefix, octal_zero, digits), 0, false))
}

// The digits of a float and the power of ten of the first one, from Rust's
// {:e} form, which rounds as strconv does.
fn go_format_float_digits(exponent_form: &str) -> (String, i32) {
    let (mantissa, exponent) = exponent_form.split_once('e').unwrap();
    let digits: String = mantissa.chars().filter(|c| c.is_ascii_digit()).collect();
    (digits, exponent.parse().unwrap())
}

fn go_format_float_exponent(digits: &str, exponent: i32, upper: bool) -> String {
    let mut out = String::new();
    out.push_str(&digits[..1]);
    if digits.len() > 1 {
        out.push('.');
        out.push_str(&digits[1..]);
    }
    out.push(if upper { 'E' } else { 'e' });
    out.push(if exponent < 0 { '-' } else { '+' });
    out.push_str(&format!("{:02}", exponent.abs()));
    out
}

// Formats the magnitude of a float as strconv.FormatFloat does for 'e', 'f'
// and 'g'. shortest is the float printed with as few digits as identify it.
fn go_format_float_magnitude(verb: char, precision: Option<usize>, shortest: &str, with_precision: impl Fn(usize) -> String, fixed: impl Fn(usize) -> String) -> String {
    let upper = verb.is_ascii_uppercase();
    match verb.to_ascii_lowercase() {
        'e' => {
            let (digits, exponent) = go_format_float_digits(&with_precision(precision.unwrap_or(6)));
            go_format_float_exponent(&digits, exponent, upper)
        }
        'f' => fixed(precision.unwrap_or(6)),
        _ => {
            let (digits, exponent, exponent_limit) = match precision {
                None => {
                    let (digits, exponent) = go_format_float_digits(shortest);
                    (digits, exponent, 6)
                }
                Some(precision) => {
                    let precision = precision.max(1);
                    let (digits, exponent) = go_format_float_digits(&with_precision(precision - 1));
                    let digits = digits.trim_end_matches('0');
                    let digits = if digits.is_empty() { "0".to_string() } else { digits.to_string() };
                    let mut limit = precision as i32;
                    if limit > digits.len() as i32 && digits.len() as i32 >= exponent + 1 {
                        limit = digits.len() as i32;
                    }
                    (digits, exponent, limit)
                }
            };
            if exponent < -4 || exponent >= exponent_limit {
                return go_format_float_exponent(&digits, exponent, upper);
            }
            if exponent < 0 {
                return format!("0.{}{}", "0".repeat((-exponent - 1) as usize), digits);
            }
            let whole = (exponent + 1) as usize;
            if digits.len() <= whole {
                return format!("{}{}", digits, "0".repeat(whole - digits.len()));
            }
            format!("{}.{}", &digits[..whole], &digits[whole..])
        }
    }
}

fn go_format_float(spec: &GoFormatSpec, verb: char, value: f64, single: bool) -> Option<String> {
    // fmt also writes a float in binary (%b) and in hex (%x). Neither is
    // done here.
    if matches!(verb, 'b' | 'x' | 'X') {
        panic!("go2rust: %{} of a float is not supported in a format that is only known at run time", verb);
    }
    if !matches!(verb, 'v' | 'e' | 'E' | 'f' | 'F' | 'g' | 'G') {
        return None;
    }
    if spec.sharp {
        panic!("go2rust: the # flag on %{} of a float is not supported in a format that is only known at run time", verb);
    }
    if value.is_nan() {
        let sign = if spec.plus { "+" } else if spec.space { " " } else { "" };
        return Some(go_format_pad(spec, format!("{}NaN", sign), 0, false));
    }
    let negative = value.is_sign_negative();
    if value.is_infinite() {
        // +Inf keeps its sign, unless the space flag asks for a space there.
        let sign = if negative { "-" } else if spec.space && !spec.plus { " " } else { "+" };
        return Some(go_format_pad(spec, format!("{}Inf", sign), 0, false));
    }
    let magnitude = value.abs();
    // A float32 keeps the digits of the float32, not of the float64 it was
    // widened to.
    let narrow = magnitude as f32;
    let shortest = if single { format!("{:e}", narrow) } else { format!("{:e}", magnitude) };
    let with_precision = |precision: usize| if single { format!("{:.*e}", precision, narrow) } else { format!("{:.*e}", precision, magnitude) };
    let fixed = |precision: usize| if single { format!("{:.*}", precision, narrow) } else { format!("{:.*}", precision, magnitude) };
    let verb = if verb == 'v' { 'g' } else { verb };
    let digits = go_format_float_magnitude(verb, spec.precision, &shortest, with_precision, fixed);
    let sign = go_format_sign(spec, negative);
    Some(go_format_pad(spec, format!("{}{}", sign, digits), sign.len(), true))
}

// %x and %X of a string or a []byte: two digits for each byte. A precision
// counts bytes here, not characters.
fn go_format_hex(spec: &GoFormatSpec, verb: char, bytes: &[u8]) -> String {
    let bytes = match spec.precision {
        Some(precision) if precision < bytes.len() => &bytes[..precision],
        _ => bytes,
    };
    let prefix = if !spec.sharp { "" } else if verb == 'x' { "0x" } else { "0X" };
    let digits: Vec<String> = bytes.iter().map(|b| if verb == 'x' { format!("{:02x}", b) } else { format!("{:02X}", b) }).collect();
    let text = if spec.space {
        digits.as_slice().iter().map(|d| format!("{}{}", prefix, d)).collect::<Vec<_>>().join(" ")
    } else if digits.is_empty() {
        String::new()
    } else {
        format!("{}{}", prefix, digits.as_slice().concat())
    };
    go_format_pad(spec, text, 0, true)
}

fn go_format_string(spec: &GoFormatSpec, verb: char, text: &str) -> Option<String> {
    if matches!(verb, 'x' | 'X') {
        return Some(go_format_hex(spec, verb, text.as_bytes()));
    }
    let truncated: String = match spec.precision {
        Some(precision) => text.chars().take(precision).collect(),
        None => text.to_string(),
    };
    let formatted = match verb {
        's' | 'v' => truncated,
        'q' => {
            // strconv.CanBackquote: no backquote, no ASCII control character
            // but tab, and no byte order mark.
            let raw = truncated.chars().all(|c| if c.is_ascii() { (c >= ' ' || c == '\t') && c != '\u{60}' && c != '\u{7f}' } else { c != '\u{feff}' });
            if spec.sharp && raw {
                format!("\u{60}{}\u{60}", truncated)
            } else {
                go_format_quote(&truncated, '"', spec.plus)
            }
        }
        _ => return None,
    };
    Some(go_format_pad(spec, formatted, 0, true))
}

// A []byte prints as text for the verbs that print a string.
fn go_format_bytes(spec: &GoFormatSpec, verb: char, bytes: &[u8]) -> Option<String> {
    if matches!(verb, 'x' | 'X') {
        return Some(go_format_hex(spec, verb, bytes));
    }
    if !matches!(verb, 's' | 'q') {
        return None;
    }
    // A precision counts characters, and a byte that is not UTF-8 counts as
    // one. It is applied first: what is cut off has no say in how the rest
    // is written.
    let mut kept = bytes.len();
    if let Some(precision) = spec.precision {
        let (mut count, mut end) = (0, 0);
        'chunks: for chunk in bytes.utf8_chunks() {
            for c in chunk.valid().chars() {
                if count == precision {
                    break 'chunks;
                }
                count += 1;
                end += c.len_utf8();
            }
            for _ in chunk.invalid() {
                if count == precision {
                    break 'chunks;
                }
                count += 1;
                end += 1;
            }
        }
        kept = end;
    }
    let bytes = &bytes[..kept];
    let spec = &GoFormatSpec { precision: None, ..*spec };
    match std::str::from_utf8(bytes) {
        Ok(text) => go_format_string(spec, verb, text),
        // %q writes a byte that is not UTF-8 as an escape.
        Err(_) if verb == 'q' => {
            let mut quoted = String::from("\"");
            for chunk in bytes.utf8_chunks() {
                go_format_escape(&mut quoted, chunk.valid(), '"', spec.plus);
                for byte in chunk.invalid() {
                    quoted.push_str(&format!("\\x{:02x}", byte));
                }
            }
            quoted.push('"');
            Some(go_format_pad(spec, quoted, 0, true))
        }
        // %s would have to write the byte as it is, and the result is a Rust
        // String, which cannot hold it.
        Err(_) => panic!("go2rust: %s of a []byte that is not UTF-8 is not supported in a format that is only known at run time"),
    }
}

// Formats one argument for one verb. None means the verb does not apply to
// the argument's type, which fmt reports in the output.
fn go_format_argument(spec: &GoFormatSpec, verb: char, value: &dyn Any) -> Option<String> {
    if verb == 'T' {
        return Some(go_format_pad(spec, __go_type_name(value).to_string(), 0, true));
    }
    if verb == 'p' {
        panic!("go2rust: %p is not supported in a format that is only known at run time");
    }
    // %+v and %#v ask for field names and Go syntax. A number or a bool has
    // neither, so it prints as %v prints it, with two exceptions for %#v: an
    // unsigned integer is written in hex and a string in quotes.
    let (plus_v, sharp_v) = (verb == 'v' && spec.plus, verb == 'v' && spec.sharp);
    let basic = GoFormatSpec { plus: spec.plus && !plus_v, sharp: spec.sharp && !sharp_v, ..*spec };
    macro_rules! signed {
        ($($t:ty),*) => {
            $(if let Some(v) = value.downcast_ref::<$t>() {
                return go_format_integer(&basic, verb, *v as i128);
            })*
        };
    }
    signed!(i8, i16, i32, i64, isize);
    macro_rules! unsigned {
        ($($t:ty),*) => {
            $(if let Some(v) = value.downcast_ref::<$t>() {
                if sharp_v {
                    return go_format_integer(&GoFormatSpec { sharp: true, ..basic }, 'x', *v as i128);
                }
                return go_format_integer(&basic, verb, *v as i128);
            })*
        };
    }
    unsigned!(u8, u16, u32, u64, usize);
    if let Some(v) = value.downcast_ref::<f64>() {
        return go_format_float(&basic, verb, *v, false);
    }
    if let Some(v) = value.downcast_ref::<f32>() {
        return go_format_float(&basic, verb, *v as f64, true);
    }
    if let Some(v) = value.downcast_ref::<bool>() {
        return if matches!(verb, 't' | 'v') { Some(go_format_pad(&basic, v.to_string(), 0, true)) } else { None };
    }
    let string_verb = if sharp_v { 'q' } else { verb };
    if let Some(v) = value.downcast_ref::<String>() {
		if matches!(string_verb, 'q' | 'x' | 'X') {
			return go_format_bytes(&basic, string_verb, &__go_string_bytes(v));
		}
        return go_format_string(&basic, string_verb, v);
    }
    if let Some(v) = value.downcast_ref::<&str>() {
		if matches!(string_verb, 'q' | 'x' | 'X') {
			return go_format_bytes(&basic, string_verb, &__go_string_bytes(v));
		}
        return go_format_string(&basic, string_verb, v);
    }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<u8>>>>>() {
        let guard = v.borrow();
        let bytes_guard = guard.as_ref().map(|bytes| bytes.as_slice());
        let bytes: &[u8] = bytes_guard.as_deref().unwrap_or(&[]);
        if let Some(text) = go_format_bytes(spec, verb, bytes) {
            return Some(text);
        }
        // Any other verb is applied to each byte, between brackets. A verb
        // that does not fit a number is reported for each byte. %#v writes
        // the slice as Go source, which is not done here.
        if !sharp_v {
            let items: Vec<String> = bytes.iter().map(|byte| go_format_integer(&basic, verb, *byte as i128).unwrap_or_else(|| format!("%!{}(uint8={})", verb, go_format_integer(&basic, 'd', *byte as i128).unwrap_or_default()))).collect();
            return Some(format!("[{}]", items.join(" ")));
        }
    }
    // A value with a String or an Error method is the text that method
    // returns, for the verbs that print a string.
    if go_format_is_text(value) && !sharp_v && matches!(verb, 'v' | 's' | 'q' | 'x' | 'X') {
		let text = format_any(value);
		if matches!(verb, 'q' | 'x' | 'X') {
			return go_format_bytes(&basic, verb, &__go_string_bytes(&text));
		}
        return go_format_string(&basic, verb, &text);
    }
    if let Some(inner) = go_format_named_basic_value(value) {
        return go_format_argument(spec, verb, inner.as_ref());
    }
    // Any other value prints as Println prints it, for a plain %v. fmt
    // applies a width, a precision, a flag or another verb to each field or
    // element, which is not done here. nil is one text.
    if verb == 'v' && !plus_v && !sharp_v {
        let text = format_any(value);
        if text == "<nil>" {
            return Some(go_format_pad(spec, text, 0, true));
        }
        if spec.width.is_none() && spec.precision.is_none() && !spec.space {
            return Some(text);
        }
    }
    panic!("go2rust: %{}{}{} of a {} is not supported in a format that is only known at run time", if spec.plus { "+" } else { "" }, if spec.sharp { "#" } else { "" }, verb, __go_type_name(value));
}

// The number after % or after the dot: digits, or * for the next argument.
// Ok(None) is no number. Err is a * whose argument is not a usable integer.
fn go_format_number(chars: &mut std::iter::Peekable<std::str::Chars>, args: &[Box<dyn Any>], next: &mut usize) -> Result<Option<i64>, ()> {
    if chars.peek() == Some(&'*') {
        chars.next();
        let arg: &dyn Any = match args.get(*next) {
            Some(arg) => arg.as_ref(),
            None => return Err(()),
        };
        *next += 1;
        // fmt takes any integer type here.
        let mut value: Option<i128> = None;
        macro_rules! integer {
            ($($t:ty),*) => {
                $(if let Some(v) = arg.downcast_ref::<$t>() {
                    value = Some(*v as i128);
                })*
            };
        }
        integer!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
        return match value {
            Some(value) if value.abs() <= 1_000_000 => Ok(Some(value as i64)),
            _ => Err(()),
        };
    }
    let mut value: Option<i64> = None;
    while let Some(digit) = chars.peek().and_then(|c| c.to_digit(10)) {
        value = Some(value.unwrap_or(0).saturating_mul(10).saturating_add(digit as i64));
        chars.next();
    }
    Ok(value.filter(|value| *value <= 1_000_000))
}

// Whether value is of a type that go_format_argument formats itself.
fn go_format_is_basic(value: &dyn Any) -> bool {
    macro_rules! is_one_of {
        ($($t:ty),*) => { false $(|| value.is::<$t>())* };
    }
    is_one_of!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, bool, String, &str, Rc<RefCell<Option<Vec<u8>>>>)
}

// An argument as fmt names it when it does not fit the format: type=value.
// The value is written as %v writes it, with the width, the precision and
// the flags of the verb that did not fit.
fn go_format_typed_value(spec: &GoFormatSpec, value: &dyn Any) -> String {
    let text = go_format_argument(spec, go_format_plain_verb(value), value).unwrap_or_default();
    format!("{}={}", __go_type_name(value), text)
}

// The verb that writes value as %v does and leaves the flags as they are.
// %v itself would read + and # as %+v and %#v.
fn go_format_plain_verb(value: &dyn Any) -> char {
    macro_rules! is_one_of {
        ($($t:ty),*) => { false $(|| value.is::<$t>())* };
    }
    if is_one_of!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize) {
        'd'
    } else if is_one_of!(f32, f64) {
        'g'
    } else if value.is::<bool>() {
        't'
    } else if is_one_of!(String, &str) {
        's'
    } else {
        'v'
    }
}

// fmt.Sprintf for a format that is only known when the program runs. A
// format written as a literal is expanded when the program is translated and
// does not come here. wraps is true for fmt.Errorf, where %w prints as %v.
pub(super) fn go_sprintf(format: &str, args: &[Box<dyn Any>], wraps: bool) -> String {
    let mut out = String::new();
    let mut next = 0;
    let mut chars = format.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '%' {
            out.push(c);
            continue;
        }
        let mut spec = GoFormatSpec::default();
        loop {
            match chars.peek() {
                Some('+') => spec.plus = true,
                Some('-') => spec.minus = true,
                Some('#') => spec.sharp = true,
                Some(' ') => spec.space = true,
                Some('0') => spec.zero = true,
                _ => break,
            }
            chars.next();
        }
        if chars.peek() == Some(&'[') {
            panic!("go2rust: an argument index such as %[1]d is not supported in a format that is only known at run time");
        }
        match go_format_number(&mut chars, args, &mut next) {
            // A negative width from an argument means left alignment.
            Ok(Some(width)) if width < 0 => {
                spec.minus = true;
                spec.width = Some((-width) as usize);
            }
            Ok(width) => spec.width = width.map(|width| width as usize),
            Err(()) => out.push_str("%!(BADWIDTH)"),
        }
        if chars.peek() == Some(&'.') {
            chars.next();
            match go_format_number(&mut chars, args, &mut next) {
                Ok(Some(precision)) if precision < 0 => out.push_str("%!(BADPREC)"),
                Ok(precision) => spec.precision = Some(precision.unwrap_or(0) as usize),
                Err(()) => out.push_str("%!(BADPREC)"),
            }
        }
        if chars.peek() == Some(&'[') {
            panic!("go2rust: an argument index such as %[1]d is not supported in a format that is only known at run time");
        }
        let verb = match chars.next() {
            Some(verb) => verb,
            None => {
                out.push_str("%!(NOVERB)");
                break;
            }
        };
        if verb == '%' {
            out.push('%');
            continue;
        }
        let arg = match args.get(next) {
            Some(arg) => arg,
            None => {
                out.push_str(&format!("%!{}(MISSING)", verb));
                continue;
            }
        };
        next += 1;
        // In fmt.Errorf %w prints an error as %v does. A number, a string, a
        // bool or a []byte is not an error, and fmt reports the verb: once
        // for a scalar, and for each byte of a []byte.
        let verb = if verb == 'w' && wraps && !go_format_is_basic(arg.as_ref()) { 'v' } else { verb };
        match go_format_argument(&spec, verb, arg.as_ref()) {
            Some(text) => out.push_str(&text),
            None => out.push_str(&format!("%!{}({})", verb, go_format_typed_value(&spec, arg.as_ref()))),
        }
    }
    if next < args.len() {
        let extra: Vec<String> = args[next..].iter().map(|arg| go_format_typed_value(&GoFormatSpec::default(), arg.as_ref())).collect();
        out.push_str(&format!("%!(EXTRA {})", extra.join(", ")));
    }
    out
}
}
use __go_runtime_format::go_sprintf;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct errors_errorString;

impl std::fmt::Display for errors_errorString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<errors_errorString>")
    }
}

impl std::error::Error for errors_errorString {}


impl errors_errorString {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        panic!("errors_errorString.error bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct errors_joinError;

impl std::fmt::Display for errors_joinError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<errors_joinError>")
    }
}

impl std::error::Error for errors_joinError {}


impl errors_joinError {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        panic!("errors_joinError.error bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone)]
pub struct NotFound {
    pub key: Rc<RefCell<Option<String>>>,
}

impl NotFound {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.key.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            key: __go_clone_0_0,
        }
    }
}


impl Default for NotFound {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            key: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for NotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}


#[derive(Debug)]
pub struct Code(pub Rc<RefCell<Option<i64>>>);

impl Clone for Code {
    fn clone(&self) -> Self {
        let __value = (*self.0.borrow()).clone();
        Code(Rc::new(RefCell::new(__value)))
    }
}

impl Default for Code {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(Some(Default::default()))))
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}

impl PartialEq for Code {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i64> for Code {
    fn eq(&self, other: &i64) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Code {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i64> for Code {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Code> for i64 {
    fn eq(&self, other: &Code) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<Code> for i64 {
    fn partial_cmp(&self, other: &Code) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for Code {
    type Output = Code;
    fn add(self, other: Self) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i64> for Code {
    type Output = Code;
    fn add(self, other: i64) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Code> for i64 {
    type Output = Code;
    fn add(self, other: Code) -> Code {
        Code(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Code {
    type Output = Code;
    fn sub(self, other: Self) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i64> for Code {
    type Output = Code;
    fn sub(self, other: i64) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Code> for i64 {
    type Output = Code;
    fn sub(self, other: Code) -> Code {
        Code(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Code {
    type Output = Code;
    fn mul(self, other: Self) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i64> for Code {
    type Output = Code;
    fn mul(self, other: i64) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Code> for i64 {
    type Output = Code;
    fn mul(self, other: Code) -> Code {
        Code(Rc::new(RefCell::new(Some(self * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Code {
    type Output = Code;
    fn div(self, other: Self) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i64> for Code {
    type Output = Code;
    fn div(self, other: i64) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Code> for i64 {
    type Output = Code;
    fn div(self, other: Code) -> Code {
        Code(Rc::new(RefCell::new(Some(self / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for Code {
    type Output = Code;
    fn neg(self) -> Code {
        Code(Rc::new(RefCell::new(Some(-*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for Code {
    type Output = Code;
    fn rem(self, other: Self) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i64> for Code {
    type Output = Code;
    fn rem(self, other: i64) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<Code> for i64 {
    type Output = Code;
    fn rem(self, other: Code) -> Code {
        Code(Rc::new(RefCell::new(Some(self % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Code {
    type Output = Code;
    fn bitand(self, other: Self) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i64> for Code {
    type Output = Code;
    fn bitand(self, other: i64) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Code> for i64 {
    type Output = Code;
    fn bitand(self, other: Code) -> Code {
        Code(Rc::new(RefCell::new(Some(self & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Code {
    type Output = Code;
    fn bitor(self, other: Self) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i64> for Code {
    type Output = Code;
    fn bitor(self, other: i64) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Code> for i64 {
    type Output = Code;
    fn bitor(self, other: Code) -> Code {
        Code(Rc::new(RefCell::new(Some(self | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Code {
    type Output = Code;
    fn bitxor(self, other: Self) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i64> for Code {
    type Output = Code;
    fn bitxor(self, other: i64) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Code> for i64 {
    type Output = Code;
    fn bitxor(self, other: Code) -> Code {
        Code(Rc::new(RefCell::new(Some(self ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Code {
    type Output = Code;
    fn not(self) -> Code {
        Code(Rc::new(RefCell::new(Some(!*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Code {
    type Output = Code;
    fn shl(self, other: Code) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for Code {
    type Output = Code;
    fn shl(self, other: i32) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for Code {
    type Output = Code;
    fn shl(self, other: i8) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for Code {
    type Output = Code;
    fn shl(self, other: i16) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for Code {
    type Output = Code;
    fn shl(self, other: i64) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for Code {
    type Output = Code;
    fn shl(self, other: u32) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for Code {
    type Output = Code;
    fn shl(self, other: u8) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for Code {
    type Output = Code;
    fn shl(self, other: u16) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for Code {
    type Output = Code;
    fn shl(self, other: u64) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for Code {
    type Output = Code;
    fn shl(self, other: usize) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for Code {
    type Output = Code;
    fn shr(self, other: Code) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for Code {
    type Output = Code;
    fn shr(self, other: i32) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for Code {
    type Output = Code;
    fn shr(self, other: i8) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for Code {
    type Output = Code;
    fn shr(self, other: i16) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for Code {
    type Output = Code;
    fn shr(self, other: i64) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for Code {
    type Output = Code;
    fn shr(self, other: u32) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for Code {
    type Output = Code;
    fn shr(self, other: u8) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for Code {
    type Output = Code;
    fn shr(self, other: u16) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for Code {
    type Output = Code;
    fn shr(self, other: u64) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for Code {
    type Output = Code;
    fn shr(self, other: usize) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl Eq for Code {}

impl Ord for Code {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Clone)]
pub struct QueryError {
    pub query: Rc<RefCell<Option<String>>>,
    pub err: Rc<RefCell<Option<Rc<dyn StdError>>>>,
}

impl QueryError {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.query.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = self.err.clone();
        Self {
            query: __go_clone_0_0,
            err: __go_clone_1_0,
        }
    }
}


impl Default for QueryError {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(None));
        Self {
            query: __go_default_0_0,
            err: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}
impl std::fmt::Debug for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
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

pub static ErrNotFound: GoGlobal<Rc<dyn StdError>> = GoGlobal::new();

pub static ErrDenied: GoGlobal<Rc<dyn StdError>> = GoGlobal::new();

pub static ErrTimeout: GoGlobal<Rc<dyn StdError>> = GoGlobal::new();


fn __go_init_globals() {
    *ErrNotFound.borrow_mut() = None;
    *ErrDenied.borrow_mut() = None;
    *ErrTimeout.borrow_mut() = None;
    { let __rhs_holder = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("not found".to_string()))))).clone(); let new_val = (*__rhs_holder.borrow()).clone(); *ErrNotFound.borrow_mut() = new_val; }
    { let __rhs_holder = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("denied".to_string()))))).clone(); let new_val = (*__rhs_holder.borrow()).clone(); *ErrDenied.borrow_mut() = new_val; }
    { let __rhs_holder = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("timeout".to_string()))))).clone(); let new_val = (*__rhs_holder.borrow()).clone(); *ErrTimeout.borrow_mut() = new_val; }
}


impl NotFound {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(__go_string_concat(&"missing ".to_string(), &self.key.clone()))))
    }
}

impl StdError for NotFound {}

#[derive(Debug, Clone)]
pub struct __go_error_ptr_NotFound(pub Rc<RefCell<Option<NotFound>>>);

impl std::fmt::Display for __go_error_ptr_NotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_is_nil = self.0.borrow().is_none();
        if !__go_is_nil {
            let __guard = self.0.borrow();
            std::fmt::Display::fmt(__guard.as_ref().unwrap(), f)
        } else {
            let __go_error_result = go_silent_catch_unwind(|| __go_nil_recv_not_found_error(Rc::new(RefCell::new(None))));
            match __go_error_result { Ok(__text) => { let __guard = __text.borrow(); match __guard.as_ref() { Some(__v) => std::fmt::Display::fmt(__v, f), None => write!(f, "<nil>") } }, Err(_) => write!(f, "<nil>") }
        }
    }
}

impl StdError for __go_error_ptr_NotFound {}


impl Code {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("code {}", (*Rc::new(RefCell::new(Some((*self.0.borrow().as_ref().unwrap()) as i64))).borrow().as_ref().unwrap())))))
    }
}

impl StdError for Code {}

#[derive(Debug, Clone)]
pub struct __go_error_ptr_Code(pub Rc<RefCell<Option<Code>>>);

impl std::fmt::Display for __go_error_ptr_Code {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => std::fmt::Display::fmt(__v, f), None => write!(f, "<nil>") }
    }
}

impl StdError for __go_error_ptr_Code {}


impl QueryError {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some({
            let mut __s = String::new();
            __s = __go_string_concat(&__s, &self.query.clone());
            __s = __go_string_concat(&__s, &": ".to_string());
            __s = __go_string_concat(&__s, &{ let __s = Rc::new(RefCell::new(Some(format!("{}", self.err.borrow().as_ref().unwrap())))); let __value = (*__s.borrow().as_ref().unwrap()).clone(); __value });
            __s
        })));
    }

    pub fn unwrap(&self) -> Rc<RefCell<Option<Rc<dyn StdError>>>> {
        { let __err_source = { let __field = self.err.clone(); __field }; let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }
    }
}

impl StdError for QueryError {}

#[derive(Debug, Clone)]
pub struct __go_error_ptr_QueryError(pub Rc<RefCell<Option<QueryError>>>);

impl std::fmt::Display for __go_error_ptr_QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_is_nil = self.0.borrow().is_none();
        if !__go_is_nil {
            let __guard = self.0.borrow();
            std::fmt::Display::fmt(__guard.as_ref().unwrap(), f)
        } else {
            let __go_error_result = go_silent_catch_unwind(|| __go_nil_recv_query_error_error(Rc::new(RefCell::new(None))));
            match __go_error_result { Ok(__text) => { let __guard = __text.borrow(); match __guard.as_ref() { Some(__v) => std::fmt::Display::fmt(__v, f), None => write!(f, "<nil>") } }, Err(_) => write!(f, "<nil>") }
        }
    }
}

impl StdError for __go_error_ptr_QueryError {}


pub fn find(key: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<Rc<dyn StdError>>>> {
    if { let __left = __go_string_from_bytes(__go_string_bytes(&(*key.borrow().as_ref().unwrap()))); let __right = __go_string_from_bytes(__go_string_bytes(&"".to_string())); __go_string_cmp(&__left, &__right) == std::cmp::Ordering::Equal } {
        return Rc::new(RefCell::new(Some(Rc::new(__go_error_ptr_NotFound(Rc::new(RefCell::new(Some(NotFound { key: Rc::new(RefCell::new(Some("empty".to_string()))), ..Default::default() }))).clone())) as Rc<dyn StdError>)));
    }
    Rc::new(RefCell::new(Some(go_error_wrap(format!("{}", go_sprintf(&"find %q: %w".to_string(), &[Box::new({ let __arg_holder = key.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any>, {
        let __err_holder = ErrNotFound.clone();
        let __err_guard = __err_holder.borrow();
        match __err_guard.as_ref() {
            None => panic!("nil error-to-any lowering requires nil interface representation"),
            Some(__err) => {
                if let Some(typed_val) = __err.downcast_ref::<Code>() {
                    go_box_any_with_metadata(typed_val.clone(), "basic", true)
                } else if let Some(typed_val) = __err.downcast_ref::<__go_error_ptr_NotFound>() {
                    go_box_any_with_metadata(typed_val.0.clone(), "pointer", true)
                } else if let Some(typed_val) = __err.downcast_ref::<__go_error_ptr_QueryError>() {
                    go_box_any_with_metadata(typed_val.0.clone(), "pointer", true)
                } else {
                    Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any>
                }
            }
        }
    }], true)), vec![{ let __err_handle = { let __err_source = ErrNotFound.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value }]))))
}

pub fn load(key: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<Rc<dyn StdError>>>> {
    {
        let mut err = find(Rc::new(RefCell::new(Some({ let __arg_holder = key.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if { let __nil_result = (*err.borrow()).is_some(); __nil_result } {
            return Rc::new(RefCell::new(Some(go_error_wrap(format!("load: {}", match { let __err_handle = { let __err_source = err.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value } { Some(__wrapped) => format!("{}", __wrapped), None => "%!w(<nil>)".to_string() }), vec![{ let __err_handle = { let __err_source = err.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value }]))));;
        }
    }
    Rc::new(RefCell::new(None))
}

pub fn run() -> Rc<RefCell<Option<Rc<dyn StdError>>>> {
    Rc::new(RefCell::new(Some(Rc::new(__go_error_ptr_QueryError(Rc::new(RefCell::new(Some(QueryError { query: Rc::new(RefCell::new(Some("select".to_string()))), err: { let __err_source = ErrTimeout.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }, ..Default::default() }))).clone())) as Rc<dyn StdError>)))
}

pub fn user_unwrap() {
    let mut err = run();
    println!("{} {} {}", format!("{}", { let __go_print_handle = err.clone(); let __go_print_value = { let __go_guard = __go_print_handle.borrow(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", (*Rc::new(RefCell::new(Some(go_errors_is({ let __err_handle = { let __err_source = err.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value }, { let __err_handle = { let __err_source = ErrTimeout.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value })))).borrow().as_ref().unwrap())), format!("{}", { let __left = Rc::new(RefCell::new(go_errors_unwrap({ let __err_handle = { let __err_source = err.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value }))).clone(); let __right = ErrTimeout.clone(); let __left_value = { (*__left.borrow()).clone() }; let __right_value = { (*__right.borrow()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<Code>(), __r.downcast_ref::<Code>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_Code>(), __r.downcast_ref::<__go_error_ptr_Code>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_NotFound>(), __r.downcast_ref::<__go_error_ptr_NotFound>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_QueryError>(), __r.downcast_ref::<__go_error_ptr_QueryError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq }));
    let mut wrapped = Rc::new(RefCell::new(Some(go_error_wrap(format!("db: {}", match { let __err_handle = { let __err_source = err.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value } { Some(__wrapped) => format!("{}", __wrapped), None => "%!w(<nil>)".to_string() }), vec![{ let __err_handle = { let __err_source = err.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value }]))));
    let mut qe: Rc<RefCell<Option<QueryError>>> = Rc::new(RefCell::new(None));
    { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = format!("{}", (*Rc::new(RefCell::new(Some(go_errors_is({ let __err_handle = { let __err_source = wrapped.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value }, { let __err_handle = { let __err_source = ErrTimeout.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value })))).borrow().as_ref().unwrap())); __go_sprint_bytes.extend(__go_sprint_arg_0.into_bytes()); __go_sprint_bytes.push(b' '); let __go_sprint_arg_1 = format!("{}", (*Rc::new(RefCell::new(Some({ let mut __as_found = None; go_error_chain_visit({ let __err_handle = { let __err_source = wrapped.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value }, |__as_err| { if let Some(__as_value) = __as_err.downcast_ref::<__go_error_ptr_QueryError>() { __as_found = Some(__as_value.0.clone()); true } else { false } }); match __as_found { Some(__as_value) => { let new_val = __as_value; qe = new_val; true } None => false } }))).borrow().as_ref().unwrap())); __go_sprint_bytes.extend(__go_sprint_arg_1.into_bytes()); __go_sprint_bytes.push(b' '); let __go_sprint_arg_2 = (*(*qe.borrow().as_ref().unwrap()).query.borrow().as_ref().unwrap()).clone(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_2)); __go_sprint_bytes.push(b'\n'); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Rc::new(RefCell::new(None::<Rc<dyn StdError>>))), Err(e) => (0, Rc::new(RefCell::new(Some::<Rc<dyn StdError>>(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(e)))))) } };
}

fn main() {
    __go_init_all();
    let mut base = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("base".to_string())))));
    let mut wrapped = Rc::new(RefCell::new(Some(go_error_wrap(format!("ctx: {}", match { let __err_handle = { let __err_source = base.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value } { Some(__wrapped) => format!("{}", __wrapped), None => "%!w(<nil>)".to_string() }), vec![{ let __err_handle = { let __err_source = base.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value }]))));
    let mut other = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("base".to_string())))));
    println!("{} {}", format!("{}", (*Rc::new(RefCell::new(Some(go_errors_is({ let __err_handle = { let __err_source = wrapped.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value }, { let __err_handle = { let __err_source = base.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value })))).borrow().as_ref().unwrap())), format!("{}", (*Rc::new(RefCell::new(Some(go_errors_is({ let __err_handle = { let __err_source = wrapped.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value }, { let __err_handle = { let __err_source = other.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value })))).borrow().as_ref().unwrap())));
    println!("{} {}", format!("{}", { let __left = Rc::new(RefCell::new(go_errors_unwrap({ let __err_handle = { let __err_source = wrapped.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value }))).clone(); let __right = base.clone(); let __left_value = { (*__left.borrow()).clone() }; let __right_value = { (*__right.borrow()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<Code>(), __r.downcast_ref::<Code>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_Code>(), __r.downcast_ref::<__go_error_ptr_Code>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_NotFound>(), __r.downcast_ref::<__go_error_ptr_NotFound>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_QueryError>(), __r.downcast_ref::<__go_error_ptr_QueryError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq }), format!("{}", { let __nil_result = (*Rc::new(RefCell::new(go_errors_unwrap({ let __err_handle = { let __err_source = base.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value }))).borrow()).is_none(); __nil_result }));
    println!("{}", format!("{}", { let __go_print_handle = wrapped.clone(); let __go_print_value = { let __go_guard = __go_print_handle.borrow(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

    let mut err = load(Rc::new(RefCell::new(Some("k".to_string()))));
    println!("{} {} {}", format!("{}", { let __go_print_handle = err.clone(); let __go_print_value = { let __go_guard = __go_print_handle.borrow(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", (*Rc::new(RefCell::new(Some(go_errors_is({ let __err_handle = { let __err_source = err.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value }, { let __err_handle = { let __err_source = ErrNotFound.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value })))).borrow().as_ref().unwrap())), format!("{}", (*Rc::new(RefCell::new(Some(go_errors_is({ let __err_handle = { let __err_source = err.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value }, { let __err_handle = { let __err_source = ErrDenied.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value })))).borrow().as_ref().unwrap())));
    let mut inner = Rc::new(RefCell::new(go_errors_unwrap({ let __err_handle = Rc::new(RefCell::new(go_errors_unwrap({ let __err_handle = { let __err_source = err.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value }))); let __err_value = (*__err_handle.borrow()).clone(); __err_value })));
    println!("{} {}", format!("{}", { let __left = inner.clone(); let __right = ErrNotFound.clone(); let __left_value = { (*__left.borrow()).clone() }; let __right_value = { (*__right.borrow()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<Code>(), __r.downcast_ref::<Code>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_Code>(), __r.downcast_ref::<__go_error_ptr_Code>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_NotFound>(), __r.downcast_ref::<__go_error_ptr_NotFound>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_QueryError>(), __r.downcast_ref::<__go_error_ptr_QueryError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq }), format!("{}", { let __go_print_handle = inner.clone(); let __go_print_value = { let __go_guard = __go_print_handle.borrow(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

    { let __rhs_holder = load(Rc::new(RefCell::new(Some("".to_string())))).clone(); let new_val = (*__rhs_holder.borrow()).clone(); *err.borrow_mut() = new_val; };
    let mut nf: Rc<RefCell<Option<NotFound>>> = Rc::new(RefCell::new(None));
    if (*Rc::new(RefCell::new(Some({ let mut __as_found = None; go_error_chain_visit({ let __err_handle = { let __err_source = err.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value }, |__as_err| { if let Some(__as_value) = __as_err.downcast_ref::<__go_error_ptr_NotFound>() { __as_found = Some(__as_value.0.clone()); true } else { false } }); match __as_found { Some(__as_value) => { let new_val = __as_value; nf = new_val; true } None => false } }))).borrow().as_ref().unwrap()) {
        { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = "as:".to_string(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_sprint_bytes.push(b' '); let __go_sprint_arg_1 = (*(*nf.borrow().as_ref().unwrap()).key.borrow().as_ref().unwrap()).clone(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_1)); __go_sprint_bytes.push(b' '); let __go_sprint_arg_2 = { let __go_string_method_receiver = { let __text = match nf.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }; let __go_string_method_result = __go_string_method_receiver.error(); let __go_string_method_text = { let __go_string_method_guard = __go_string_method_result.borrow(); (*__go_string_method_guard.as_ref().unwrap()).clone() }; __go_string_method_text }; __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_2)); __go_sprint_bytes.push(b'\n'); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Rc::new(RefCell::new(None::<Rc<dyn StdError>>))), Err(e) => (0, Rc::new(RefCell::new(Some::<Rc<dyn StdError>>(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(e)))))) } };
    }
    let mut code: Rc<RefCell<Option<Code>>> = Rc::new(RefCell::new(Some(Code(Rc::new(RefCell::new(Some(0)))))));
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some({ let mut __as_found = None; go_error_chain_visit({ let __err_handle = { let __err_source = err.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value }, |__as_err| { if let Some(__as_value) = __as_err.downcast_ref::<Code>() { __as_found = Some(__as_value.clone()); true } else { false } }); match __as_found { Some(__as_value) => { *code.borrow_mut() = Some(__as_value); true } None => false } }))).borrow().as_ref().unwrap())));
    { let __rhs_holder = Rc::new(RefCell::new(Some(go_error_wrap(format!("outer: {}", match { let __err_handle = Rc::new(RefCell::new(Some(Rc::new(Code(Rc::new(RefCell::new(Some(7 as i64 as i64))))) as Rc<dyn StdError>))); let __err_value = (*__err_handle.borrow()).clone(); __err_value } { Some(__wrapped) => format!("{}", __wrapped), None => "%!w(<nil>)".to_string() }), vec![{ let __err_handle = Rc::new(RefCell::new(Some(Rc::new(Code(Rc::new(RefCell::new(Some(7 as i64 as i64))))) as Rc<dyn StdError>))); let __err_value = (*__err_handle.borrow()).clone(); __err_value }])))).clone(); let new_val = (*__rhs_holder.borrow()).clone(); *err.borrow_mut() = new_val; };
    if (*Rc::new(RefCell::new(Some({ let mut __as_found = None; go_error_chain_visit({ let __err_handle = { let __err_source = err.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value }, |__as_err| { if let Some(__as_value) = __as_err.downcast_ref::<Code>() { __as_found = Some(__as_value.clone()); true } else { false } }); match __as_found { Some(__as_value) => { *code.borrow_mut() = Some(__as_value); true } None => false } }))).borrow().as_ref().unwrap()) {
        { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = "code:".to_string(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_sprint_bytes.push(b' '); let __go_sprint_arg_1 = { let __go_string_method_receiver = { let __v = (*code.borrow().as_ref().unwrap()).clone(); __v }; let __go_string_method_result = __go_string_method_receiver.error(); let __go_string_method_text = { let __go_string_method_guard = __go_string_method_result.borrow(); (*__go_string_method_guard.as_ref().unwrap()).clone() }; __go_string_method_text }; __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_1)); __go_sprint_bytes.push(b' '); let __go_sprint_arg_2 = format!("{}", (*Rc::new(RefCell::new(Some((*(*code.borrow().as_ref().unwrap()).0.borrow().as_ref().unwrap()) as i64))).borrow().as_ref().unwrap())); __go_sprint_bytes.extend(__go_sprint_arg_2.into_bytes()); __go_sprint_bytes.push(b'\n'); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Rc::new(RefCell::new(None::<Rc<dyn StdError>>))), Err(e) => (0, Rc::new(RefCell::new(Some::<Rc<dyn StdError>>(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(e)))))) } };
    }

    let mut both = Rc::new(RefCell::new(Some(go_error_wrap(format!("{} and {}", match { let __err_handle = { let __err_source = ErrNotFound.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value } { Some(__wrapped) => format!("{}", __wrapped), None => "%!w(<nil>)".to_string() }, match { let __err_handle = { let __err_source = ErrDenied.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value } { Some(__wrapped) => format!("{}", __wrapped), None => "%!w(<nil>)".to_string() }), vec![{ let __err_handle = { let __err_source = ErrNotFound.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value }, { let __err_handle = { let __err_source = ErrDenied.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value }]))));
    println!("{} {} {} {}", format!("{}", { let __go_print_handle = both.clone(); let __go_print_value = { let __go_guard = __go_print_handle.borrow(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", (*Rc::new(RefCell::new(Some(go_errors_is({ let __err_handle = { let __err_source = both.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value }, { let __err_handle = { let __err_source = ErrNotFound.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value })))).borrow().as_ref().unwrap())), format!("{}", (*Rc::new(RefCell::new(Some(go_errors_is({ let __err_handle = { let __err_source = both.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value }, { let __err_handle = { let __err_source = ErrDenied.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value })))).borrow().as_ref().unwrap())), format!("{}", { let __nil_result = (*Rc::new(RefCell::new(go_errors_unwrap({ let __err_handle = { let __err_source = both.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value }))).borrow()).is_none(); __nil_result }));
    let mut joined = Rc::new(RefCell::new(go_errors_join(vec![{ let __err_handle = { let __err_source = ErrDenied.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value }, { let __err_handle = Rc::new(RefCell::new(None::<Rc<dyn StdError>>)); let __err_value = (*__err_handle.borrow()).clone(); __err_value }, { let __err_handle = { let __err_source = base.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value }])));
    println!("{} {}", format!("{}", (*Rc::new(RefCell::new(Some(go_errors_is({ let __err_handle = { let __err_source = joined.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value }, { let __err_handle = { let __err_source = base.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value })))).borrow().as_ref().unwrap())), format!("{}", (*Rc::new(RefCell::new(Some(go_errors_is({ let __err_handle = { let __err_source = joined.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value }, { let __err_handle = { let __err_source = ErrNotFound.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value })))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", { let __go_print_handle = joined.clone(); let __go_print_value = { let __go_guard = __go_print_handle.borrow(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    println!("{} {} {}", format!("{}", { let __nil_result = (*Rc::new(RefCell::new(go_errors_join(vec![{ let __err_handle = Rc::new(RefCell::new(None::<Rc<dyn StdError>>)); let __err_value = (*__err_handle.borrow()).clone(); __err_value }, { let __err_handle = Rc::new(RefCell::new(None::<Rc<dyn StdError>>)); let __err_value = (*__err_handle.borrow()).clone(); __err_value }]))).borrow()).is_none(); __nil_result }), format!("{}", (*Rc::new(RefCell::new(Some(go_errors_is({ let __err_handle = Rc::new(RefCell::new(None::<Rc<dyn StdError>>)); let __err_value = (*__err_handle.borrow()).clone(); __err_value }, { let __err_handle = Rc::new(RefCell::new(None::<Rc<dyn StdError>>)); let __err_value = (*__err_handle.borrow()).clone(); __err_value })))).borrow().as_ref().unwrap())), format!("{}", (*Rc::new(RefCell::new(Some(go_errors_is({ let __err_handle = Rc::new(RefCell::new(None::<Rc<dyn StdError>>)); let __err_value = (*__err_handle.borrow()).clone(); __err_value }, { let __err_handle = { let __err_source = base.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value })))).borrow().as_ref().unwrap())));
    let mut nilErr: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(None));
    println!("{} {}", format!("{}", { let __go_print_handle = Rc::new(RefCell::new(Some(go_error_wrap(format!("wrap nil: {}", match { let __err_handle = { let __err_source = nilErr.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value } { Some(__wrapped) => format!("{}", __wrapped), None => "%!w(<nil>)".to_string() }), vec![{ let __err_handle = { let __err_source = nilErr.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value }])))); let __go_print_value = { let __go_guard = __go_print_handle.borrow(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __nil_result = (*Rc::new(RefCell::new(go_errors_unwrap({ let __err_handle = Rc::new(RefCell::new(Some(go_error_wrap(format!("x: {}", match { let __err_handle = { let __err_source = nilErr.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value } { Some(__wrapped) => format!("{}", __wrapped), None => "%!w(<nil>)".to_string() }), vec![{ let __err_handle = { let __err_source = nilErr.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value }])))); let __err_value = (*__err_handle.borrow()).clone(); __err_value }))).borrow()).is_none(); __nil_result }));
    user_unwrap();
}

pub fn __go_nil_recv_not_found_error(e: Rc<RefCell<Option<NotFound>>>) -> Rc<RefCell<Option<String>>> {
    Rc::new(RefCell::new(Some(__go_string_concat(&"missing ".to_string(), &(*(*e.borrow().as_ref().unwrap()).key.borrow().as_ref().unwrap()).clone()))))
}

pub fn __go_nil_recv_query_error_error(e: Rc<RefCell<Option<QueryError>>>) -> Rc<RefCell<Option<String>>> {
    return Rc::new(RefCell::new(Some({
        let mut __s = String::new();
        __s = __go_string_concat(&__s, &(*(*e.borrow().as_ref().unwrap()).query.borrow().as_ref().unwrap()).clone());
        __s = __go_string_concat(&__s, &": ".to_string());
        __s = __go_string_concat(&__s, &{ let __s = Rc::new(RefCell::new(Some(format!("{}", (*e.borrow().as_ref().unwrap()).err.borrow().as_ref().unwrap())))); let __value = (*__s.borrow().as_ref().unwrap()).clone(); __value });
        __s
    })));
}

pub fn __go_nil_recv_query_error_unwrap(e: Rc<RefCell<Option<QueryError>>>) -> Rc<RefCell<Option<Rc<dyn StdError>>>> {
    { let __err_source = { let __field = (*e.borrow().as_ref().unwrap()).err.clone(); __field }; let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
