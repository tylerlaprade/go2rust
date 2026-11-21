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
pub struct Point {
    pub x: Arc<Mutex<Option<i64>>>,
    pub y: Arc<Mutex<Option<i64>>>,
}

impl Point {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.y.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
            y: __go_clone_0_1,
        }
    }
}


impl Default for Point {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_0_1 = Arc::new(Mutex::new(Some(0)));
        Self {
            x: __go_default_0_0,
            y: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.y.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


/// Status has a String method on the value, so *Status has it too.
#[derive(Debug, Clone)]
pub struct Status {
    pub code: Arc<Mutex<Option<i64>>>,
}

impl Status {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.code.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            code: __go_clone_0_0,
        }
    }
}


impl Default for Status {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            code: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}


/// Gauge has a String method on the pointer only.
#[derive(Debug, Clone)]
pub struct Gauge {
    pub level: Arc<Mutex<Option<i64>>>,
}

impl Gauge {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.level.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            level: __go_clone_0_0,
        }
    }
}


impl Default for Gauge {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            level: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Gauge {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}


/// Broken is an error through its pointer.
#[derive(Debug, Clone)]
pub struct Broken {
    pub why: Arc<Mutex<Option<String>>>,
}

impl Broken {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.why.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            why: __go_clone_0_0,
        }
    }
}


impl Default for Broken {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            why: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Broken {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}


/// Timeout is an error through its value.
#[derive(Debug, Clone)]
pub struct Timeout {
    pub seconds: Arc<Mutex<Option<i64>>>,
}

impl Timeout {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.seconds.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            seconds: __go_clone_0_0,
        }
    }
}


impl Default for Timeout {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            seconds: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Timeout {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}
impl PartialEq for Timeout {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.seconds.lock().unwrap().as_ref().cloned() }; let __right = { other.seconds.lock().unwrap().as_ref().cloned() }; __left == __right }
        )
    }
}


impl Status {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(format!("status {}", (*self.code.lock().unwrap().as_ref().unwrap())))))
    }
}

impl Gauge {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(format!("gauge {}", (*self.level.lock().unwrap().as_ref().unwrap())))))
    }
}

impl Broken {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("{}{}", "broken: ".to_string(), (*self.why.clone().lock().unwrap().as_ref().unwrap())))));
    }
}

impl StdError for Broken {}

#[derive(Debug, Clone)]
pub struct __go_error_ptr_Broken(pub Arc<Mutex<Option<Broken>>>);

impl std::fmt::Display for __go_error_ptr_Broken {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_is_nil = self.0.lock().unwrap().is_none();
        if !__go_is_nil {
            let __guard = self.0.lock().unwrap();
            std::fmt::Display::fmt(__guard.as_ref().unwrap(), f)
        } else {
            let __go_error_result = go_silent_catch_unwind(|| __go_nil_recv_broken_error(Arc::new(Mutex::new(None))));
            match __go_error_result { Ok(__text) => { let __guard = __text.lock().unwrap(); match __guard.as_ref() { Some(__v) => std::fmt::Display::fmt(__v, f), None => write!(f, "<nil>") } }, Err(_) => write!(f, "<nil>") }
        }
    }
}

impl StdError for __go_error_ptr_Broken {}


impl Timeout {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(format!("timeout after {}s", (*self.seconds.lock().unwrap().as_ref().unwrap())))))
    }
}

impl StdError for Timeout {}

#[derive(Debug, Clone)]
pub struct __go_error_ptr_Timeout(pub Arc<Mutex<Option<Timeout>>>);

impl std::fmt::Display for __go_error_ptr_Timeout {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => std::fmt::Display::fmt(__v, f), None => write!(f, "<nil>") }
    }
}

impl StdError for __go_error_ptr_Timeout {}


pub fn find(ok: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> {
    if { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return Arc::new(Mutex::new(None));
    }
    Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("missing".to_string())))))
}

pub fn lookup(ok: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<Point>>> {
    if { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(1))), y: Arc::new(Mutex::new(Some(2))), ..Default::default() })));
    }
    return Arc::new(Mutex::new(None));
}

fn main() {
        // A goroutine makes every shared value a locked one.
    let mut done = GoChannel::<String>::new_buffered(1 as usize);
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send("started".to_string());;;
    });
    println!("{}", format!("{}", done.recv().unwrap_or_default()));

        // A nil error prints as <nil>, whichever print call it reaches and
        // whether it comes from a variable or straight from a call.
    let mut err = find(Arc::new(Mutex::new(Some(true))));
    println!("{}", format!("{}", { let __go_print_handle = err.clone(); let __go_print_value = { let __go_guard = __go_print_handle.lock().unwrap(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    println!("{} {}", format!("{}", "result:".to_string()), format!("{}", { let __go_print_handle = err.clone(); let __go_print_value = { let __go_guard = __go_print_handle.lock().unwrap(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    print!("{}\n", { let __go_print_handle = err.clone(); let __go_print_value = { let __go_guard = __go_print_handle.lock().unwrap(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
    print!("{} and {}\n", format!("{}", { let __go_print_handle = err.clone(); let __go_print_value = { let __go_guard = __go_print_handle.lock().unwrap(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", 3));
    println!("{}", format!("{}", { let __go_print_handle = find(Arc::new(Mutex::new(Some(true)))); let __go_print_value = { let __go_guard = __go_print_handle.lock().unwrap(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    println!("{}", format!("{}", (*Arc::new(Mutex::new(Some(format!("{}", { let __go_print_handle = err.clone(); let __go_print_value = { let __go_guard = __go_print_handle.lock().unwrap(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text })))).lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", (*Arc::new(Mutex::new(Some(format!("got {}", { let __go_print_handle = find(Arc::new(Mutex::new(Some(true)))); let __go_print_value = { let __go_guard = __go_print_handle.lock().unwrap(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text })))).lock().unwrap().as_ref().unwrap())));

        // An error that is not nil prints its message, as before.
    { let __rhs_holder = find(Arc::new(Mutex::new(Some(false)))).clone(); let new_val = (*__rhs_holder.lock().unwrap()).clone(); *err.lock().unwrap() = new_val; };
    println!("{}", format!("{}", { let __go_print_handle = err.clone(); let __go_print_value = { let __go_guard = __go_print_handle.lock().unwrap(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    println!("{} {}", format!("{}", "result:".to_string()), format!("{}", { let __go_print_handle = err.clone(); let __go_print_value = { let __go_guard = __go_print_handle.lock().unwrap(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    print!("{} {}\n", format!("{}", { let __go_print_handle = err.clone(); let __go_print_value = { let __go_guard = __go_print_handle.lock().unwrap(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __go_print_handle = err.clone(); let __go_print_value = { let __go_guard = __go_print_handle.lock().unwrap(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    println!("{}", format!("{}", { let __go_print_handle = find(Arc::new(Mutex::new(Some(false)))); let __go_print_value = { let __go_guard = __go_print_handle.lock().unwrap(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

        // A nil pointer prints as <nil>.
    let mut p: Arc<Mutex<Option<Point>>> = Arc::new(Mutex::new(None));
    println!("{}", format!("{}", { let __text = match p.lock().unwrap().as_ref() { Some(__value) => format!("&{}", __value), None => "<nil>".to_string() }; __text }));
    println!("{} {}", format!("{}", "point:".to_string()), format!("{}", { let __text = match p.lock().unwrap().as_ref() { Some(__value) => format!("&{}", __value), None => "<nil>".to_string() }; __text }));
    print!("{}\n", { let __text = match p.lock().unwrap().as_ref() { Some(__value) => format!("&{}", __value), None => "<nil>".to_string() }; __text });
    println!("{}", format!("{}", { let __text = match lookup(Arc::new(Mutex::new(Some(false)))).lock().unwrap().as_ref() { Some(__value) => format!("&{}", __value), None => "<nil>".to_string() }; __text }));
    { let new_val = lookup(Arc::new(Mutex::new(Some(true)))).clone(); p = new_val; };
    println!("{}", format!("{}", { let __text = match p.lock().unwrap().as_ref() { Some(__value) => format!("&{}", __value), None => "<nil>".to_string() }; __text }));
    println!("{}", format!("{}", { let __text = match lookup(Arc::new(Mutex::new(Some(true)))).lock().unwrap().as_ref() { Some(__value) => format!("&{}", __value), None => "<nil>".to_string() }; __text }));

        // A pointer whose type has a String or Error method prints through the
        // method, with no & in front. A nil one prints as <nil>.
    let mut s = Arc::new(Mutex::new(Some(Status { code: Arc::new(Mutex::new(Some(7))), ..Default::default() })));
    println!("{}", format!("{}", { let __text = match s.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    print!("{} {}\n", format!("{}", { let __text = match s.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __text = match s.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    let mut g = Arc::new(Mutex::new(Some(Gauge { level: Arc::new(Mutex::new(Some(3))), ..Default::default() })));
    println!("{}", format!("{}", { let __text = match g.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    println!("{}", format!("{}", (*Arc::new(Mutex::new(Some(format!("{}", { let __text = match g.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text })))).lock().unwrap().as_ref().unwrap())));
    let mut b = Arc::new(Mutex::new(Some(Broken { why: Arc::new(Mutex::new(Some("z".to_string()))), ..Default::default() })));
    println!("{}", format!("{}", { let __text = match b.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    println!("{} {} {} {}", format!("{}", "gauge:".to_string()), format!("{}", { let __text = match g.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", "broken:".to_string()), format!("{}", { let __text = match b.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    let mut none: Arc<Mutex<Option<Status>>> = Arc::new(Mutex::new(None));
    println!("{}", format!("{}", { let __text = match none.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    let mut empty: Arc<Mutex<Option<Gauge>>> = Arc::new(Mutex::new(None));
    println!("{}", format!("{}", { let __text = match empty.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

        // A struct or a pointer that implements error can start an error
        // variable, and prints as the error it is.
    let mut declared: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(Some(Arc::new(__go_error_ptr_Broken(Arc::new(Mutex::new(Some(Broken { why: Arc::new(Mutex::new(Some("declared".to_string()))), ..Default::default() }))).clone())) as Arc<dyn StdError + Send + Sync>)));
    println!("{}", format!("{}", { let __go_print_handle = declared.clone(); let __go_print_value = { let __go_guard = __go_print_handle.lock().unwrap(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    let mut plain: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(Some(Arc::new(Timeout { seconds: Arc::new(Mutex::new(Some(3))), ..Default::default() }) as Arc<dyn StdError + Send + Sync>)));
    println!("{}", format!("{}", { let __go_print_handle = plain.clone(); let __go_print_value = { let __go_guard = __go_print_handle.lock().unwrap(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    println!("{} {}", format!("{}", { let __nil_result = (*declared.lock().unwrap()).is_some(); __nil_result }), format!("{}", { let __nil_result = (*plain.lock().unwrap()).is_some(); __nil_result }));
    let mut unset: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));
    println!("{} {}", format!("{}", { let __go_print_handle = unset.clone(); let __go_print_value = { let __go_guard = __go_print_handle.lock().unwrap(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __nil_result = (*unset.lock().unwrap()).is_none(); __nil_result }));
}

pub fn __go_nil_recv_broken_error(b: Arc<Mutex<Option<Broken>>>) -> Arc<Mutex<Option<String>>> {
    return Arc::new(Mutex::new(Some(format!("{}{}", "broken: ".to_string(), (*{ let __field = (*b.lock().unwrap().as_ref().unwrap()).why.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()))));
}