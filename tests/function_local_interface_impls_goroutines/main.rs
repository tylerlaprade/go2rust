#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

/// A Go slice: an offset, a length and a capacity over a backing array that
/// every slice made from it shares. Every name below that a binding uses
/// starts with __, so no package-level variable of the program shadows it.
pub struct GoSlice<T> {
    backing: std::sync::Arc<std::cell::UnsafeCell<std::vec::Vec<T>>>,
    off: usize,
    len: usize,
    cap: usize,
}

unsafe impl<T: Send> Send for GoSlice<T> {}
unsafe impl<T: Send + Sync> Sync for GoSlice<T> {}

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
        GoSlice { backing: std::sync::Arc::from(std::cell::UnsafeCell::new(std::vec::Vec::with_capacity(__cap))), off: 0, len: 0, cap: __cap }
    }

    fn backing_ref(&self) -> &std::vec::Vec<T> {
        unsafe { &*self.backing.get() }
    }

    #[allow(clippy::mut_from_ref)]
    fn backing_mut(&self) -> &mut std::vec::Vec<T> {
        unsafe { &mut *self.backing.get() }
    }

    pub fn as_slice(&self) -> &[T] {
        &self.backing_ref()[self.off..self.off + self.len]
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        let (__off, __len) = (self.off, self.len);
        &mut self.backing_mut()[__off..__off + __len]
    }

    /// Go's cap(s).
    pub fn capacity(&self) -> usize {
        self.cap
    }

    /// Whether two slices share their backing array.
    pub fn shares_backing_with(&self, __other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.backing, &__other.backing)
    }

    /// Go's s[lo:hi].
    pub fn reslice(&self, __lo: usize, __hi: usize) -> Self {
        if __lo > __hi || __hi > self.cap {
            panic!("runtime error: slice bounds out of range [{}:{}] with capacity {}", __lo, __hi, self.cap);
        }
        if self.off + __hi > self.backing_ref().len() {
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
            let __window = self.as_mut_slice();
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
        let mut __elems = std::mem::take(__backing.get_mut());
        __elems.truncate(__off + __len);
        __elems.drain(..__off);
        Some(__elems)
    }

    fn fresh(__elems: std::vec::Vec<T>, __cap: usize) -> Self {
        let __len = __elems.len();
        GoSlice { backing: std::sync::Arc::from(std::cell::UnsafeCell::new(__elems)), off: 0, len: __len, cap: __cap.max(__len) }
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
        let __backing = self.backing_mut();
        if __at < __backing.len() {
            __backing[__at] = __value;
        } else {
            __backing.push(__value);
        }
        self.len += 1;
    }

    /// append for elements that cannot be copied: when the slice is full and
    /// shares its backing array, the value goes at the end of that array.
    fn push_without_copy(&mut self, __value: T) {
        if self.len < self.cap {
            self.store_at_end(__value);
            return;
        }
        let __cap = self.grown_cap(self.len + 1);
        if let Some(mut __elems) = self.take_unique() {
            __elems.reserve(__cap.saturating_sub(__elems.len()));
            __elems.push(__value);
            *self = Self::fresh(__elems, __cap);
            return;
        }
        self.store_at_end(__value);
        self.cap = self.len;
    }
}

impl<T: Clone> GoSlice<T> {
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

impl<T> GoSliceBoxedValues<T> for GoSlice<T> {
    fn into_boxed_values(mut self) -> std::vec::Vec<T> {
        match self.take_unique() {
            Some(__elems) => __elems,
            None => panic!("append of interface values held by another slice"),
        }
    }
}

/// Go's append for interface values, which have no Clone. When the slice
/// is full and shares its backing array, the new element goes at the end of
/// that array instead of into a copy.
pub trait GoSliceAppendBoxed<T> {
    fn push(&mut self, __value: T);
    fn extend<I: GoSliceBoxedValues<T>>(&mut self, __values: I);
}

impl GoSliceAppendBoxed<Box<dyn std::any::Any>> for GoSlice<Box<dyn std::any::Any>> {
    fn push(&mut self, __value: Box<dyn std::any::Any>) {
        self.push_without_copy(__value);
    }
    fn extend<I: GoSliceBoxedValues<Box<dyn std::any::Any>>>(&mut self, __values: I) {
        for __value in __values.into_boxed_values() {
            self.push_without_copy(__value);
        }
    }
}

impl GoSliceAppendBoxed<Box<dyn std::any::Any + Send + Sync>> for GoSlice<Box<dyn std::any::Any + Send + Sync>> {
    fn push(&mut self, __value: Box<dyn std::any::Any + Send + Sync>) {
        self.push_without_copy(__value);
    }
    fn extend<I: GoSliceBoxedValues<Box<dyn std::any::Any + Send + Sync>>>(&mut self, __values: I) {
        for __value in __values.into_boxed_values() {
            self.push_without_copy(__value);
        }
    }
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
        GoSlice { backing: std::sync::Arc::from(std::cell::UnsafeCell::new(__elems)), off: 0, len: __len, cap: __len }
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

impl<T> std::ops::Deref for GoSlice<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> std::ops::DerefMut for GoSlice<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T> AsRef<[T]> for GoSlice<T> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> AsMut<[T]> for GoSlice<T> {
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T> std::borrow::Borrow<[T]> for GoSlice<T> {
    fn borrow(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> std::borrow::BorrowMut<[T]> for GoSlice<T> {
    fn borrow_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
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

impl<'a, T> IntoIterator for &'a GoSlice<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}

impl<'a, T> IntoIterator for &'a mut GoSlice<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.as_mut_slice().iter_mut()
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for GoSlice<T> {
    fn fmt(&self, __f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_slice().fmt(__f)
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

impl<T: PartialEq> PartialEq for GoSlice<T> {
    fn eq(&self, __other: &Self) -> bool {
        self.as_slice() == __other.as_slice()
    }
}

impl<T: PartialEq> PartialEq<[T]> for GoSlice<T> {
    fn eq(&self, __other: &[T]) -> bool {
        self.as_slice() == __other
    }
}

impl<T: PartialEq> PartialEq<std::vec::Vec<T>> for GoSlice<T> {
    fn eq(&self, __other: &std::vec::Vec<T>) -> bool {
        self.as_slice() == __other.as_slice()
    }
}

impl<T: Eq> Eq for GoSlice<T> {}

impl<T: PartialOrd> PartialOrd for GoSlice<T> {
    fn partial_cmp(&self, __other: &Self) -> Option<std::cmp::Ordering> {
        self.as_slice().partial_cmp(__other.as_slice())
    }
}

impl<T: Ord> Ord for GoSlice<T> {
    fn cmp(&self, __other: &Self) -> std::cmp::Ordering {
        self.as_slice().cmp(__other.as_slice())
    }
}

impl<T: std::hash::Hash> std::hash::Hash for GoSlice<T> {
    fn hash<H: std::hash::Hasher>(&self, __state: &mut H) {
        self.as_slice().hash(__state)
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


fn go_embedded_owner_registry() -> &'static std::sync::Mutex<std::collections::HashMap<usize, Box<dyn Any + Send + Sync>>> {
    static REGISTRY: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<usize, Box<dyn Any + Send + Sync>>>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

fn go_register_embedded_owner<T: Send + Sync + 'static>(embedded_key: usize, owner: Arc<Mutex<Option<T>>>) {
    go_embedded_owner_registry().lock().unwrap().insert(embedded_key, Box::new(owner));
}

fn go_lookup_embedded_owner<T: Send + Sync + 'static>(embedded_key: usize, target: &str) -> Arc<Mutex<Option<T>>> {
    let registry = go_embedded_owner_registry().lock().unwrap();
    let owner = registry.get(&embedded_key).unwrap_or_else(|| panic!("embedded owner registry missing {}", target));
    owner
        .downcast_ref::<Arc<Mutex<Option<T>>>>()
        .unwrap_or_else(|| panic!("embedded owner registry type mismatch for {}", target))
        .clone()
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

pub trait incer: std::fmt::Display + Any {
    fn __go_clone_box_incer(&self) -> Box<dyn incer + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_incer(&self, other: &(dyn incer + Send + Sync)) -> bool;
    fn inc(&mut self);
}

impl Clone for Box<dyn incer + Send + Sync> {
    fn clone(&self) -> Self {
        incer::__go_clone_box_incer(self.as_ref())
    }
}

pub trait namer: std::fmt::Display + Any {
    fn __go_clone_box_namer(&self) -> Box<dyn namer + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_namer(&self, other: &(dyn namer + Send + Sync)) -> bool;
    fn name(&self) -> Arc<Mutex<Option<String>>>;
}

impl Clone for Box<dyn namer + Send + Sync> {
    fn clone(&self) -> Self {
        namer::__go_clone_box_namer(self.as_ref())
    }
}

pub trait speaker: std::fmt::Display + Any {
    fn __go_clone_box_speaker(&self) -> Box<dyn speaker + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_speaker(&self, other: &(dyn speaker + Send + Sync)) -> bool;
    fn name(&self) -> Arc<Mutex<Option<String>>>;
    fn sound(&self) -> Arc<Mutex<Option<String>>>;
}

impl Clone for Box<dyn speaker + Send + Sync> {
    fn clone(&self) -> Self {
        speaker::__go_clone_box_speaker(self.as_ref())
    }
}

#[derive(Debug, Clone)]
pub struct animal {
    pub name: Arc<Mutex<Option<String>>>,
}

impl animal {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for animal {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for animal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}
impl PartialEq for animal {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.name.lock().unwrap().as_ref().cloned() }; let __right = { other.name.lock().unwrap().as_ref().cloned() }; __left == __right }
        )
    }
}


#[derive(Debug, Clone)]
pub struct dog {
    pub animal: Arc<Mutex<Option<animal>>>,
    pub breed: Arc<Mutex<Option<String>>>,
}

impl dog {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.animal.lock().unwrap(); Arc::new(Mutex::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        let __go_clone_1_0 = { let __guard = self.breed.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            animal: __go_clone_0_0,
            breed: __go_clone_1_0,
        }
    }
}


impl Default for dog {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(animal::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            animal: __go_default_0_0,
            breed: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for dog {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.animal.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.breed.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct cat {
    pub animal: Arc<Mutex<Option<animal>>>,
    pub lives: Arc<Mutex<Option<i64>>>,
}

impl cat {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.animal.clone();
        let __go_clone_1_0 = { let __guard = self.lives.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            animal: __go_clone_0_0,
            lives: __go_clone_1_0,
        }
    }
}


impl Default for cat {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            animal: __go_default_0_0,
            lives: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for cat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.animal.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", (*self.lives.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct counter {
    pub n: Arc<Mutex<Option<i64>>>,
}

impl counter {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            n: __go_clone_0_0,
        }
    }
}


impl Default for counter {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            n: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for counter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.n.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}
impl PartialEq for counter {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.n.lock().unwrap().as_ref().cloned() }; let __right = { other.n.lock().unwrap().as_ref().cloned() }; __left == __right }
        )
    }
}


impl animal {
    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some({ let __selector_holder = self.name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    }
}

impl namer for animal {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        animal::name(self)
    }
    fn __go_clone_box_namer(&self) -> Box<dyn namer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn namer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_namer(&self, other: &(dyn namer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<animal>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct animalPtr(pub Arc<Mutex<Option<animal>>>);

impl std::fmt::Display for animalPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl namer for animalPtr {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        animal::name(__recv)
    }
    fn __go_clone_box_namer(&self) -> Box<dyn namer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn namer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_namer(&self, other: &(dyn namer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<animalPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl dog {
    pub fn sound(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some("woof".to_string())))
    }

    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.animal.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.name()
    }
}

impl namer for dog {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        dog::name(self)
    }
    fn __go_clone_box_namer(&self) -> Box<dyn namer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn namer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_namer(&self, other: &(dyn namer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<dog>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct dogPtr(pub Arc<Mutex<Option<dog>>>);

impl std::fmt::Display for dogPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl namer for dogPtr {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        dog::name(__recv)
    }
    fn __go_clone_box_namer(&self) -> Box<dyn namer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn namer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_namer(&self, other: &(dyn namer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<dogPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl speaker for dog {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        dog::name(self)
    }
    fn sound(&self) -> Arc<Mutex<Option<String>>> {
        dog::sound(self)
    }
    fn __go_clone_box_speaker(&self) -> Box<dyn speaker + Send + Sync> {
        Box::new(self.clone()) as Box<dyn speaker + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_speaker(&self, other: &(dyn speaker + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<dog>() {
            false
        } else {
            false
        }
    }
}

impl speaker for dogPtr {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        dog::name(__recv)
    }
    fn sound(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        dog::sound(__recv)
    }
    fn __go_clone_box_speaker(&self) -> Box<dyn speaker + Send + Sync> {
        Box::new(self.clone()) as Box<dyn speaker + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_speaker(&self, other: &(dyn speaker + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<dogPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl cat {
    pub fn sound(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some("meow".to_string())))
    }

    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.animal.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.name()
    }
}

impl namer for cat {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        cat::name(self)
    }
    fn __go_clone_box_namer(&self) -> Box<dyn namer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn namer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_namer(&self, other: &(dyn namer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<cat>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct catPtr(pub Arc<Mutex<Option<cat>>>);

impl std::fmt::Display for catPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl namer for catPtr {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        cat::name(__recv)
    }
    fn __go_clone_box_namer(&self) -> Box<dyn namer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn namer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_namer(&self, other: &(dyn namer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<catPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl speaker for cat {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        cat::name(self)
    }
    fn sound(&self) -> Arc<Mutex<Option<String>>> {
        cat::sound(self)
    }
    fn __go_clone_box_speaker(&self) -> Box<dyn speaker + Send + Sync> {
        Box::new(self.clone()) as Box<dyn speaker + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_speaker(&self, other: &(dyn speaker + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<cat>() {
            false
        } else {
            false
        }
    }
}

impl speaker for catPtr {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        cat::name(__recv)
    }
    fn sound(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        cat::sound(__recv)
    }
    fn __go_clone_box_speaker(&self) -> Box<dyn speaker + Send + Sync> {
        Box::new(self.clone()) as Box<dyn speaker + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_speaker(&self, other: &(dyn speaker + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<catPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl counter {
    pub fn inc(&mut self) {
        { let __target = self.n.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

impl incer for counter {
    fn inc(&mut self) {
        counter::inc(self)
    }
    fn __go_clone_box_incer(&self) -> Box<dyn incer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn incer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_incer(&self, other: &(dyn incer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<counter>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct counterPtr(pub Arc<Mutex<Option<counter>>>);

impl std::fmt::Display for counterPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl incer for counterPtr {
    fn inc(&mut self) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        counter::inc(__recv)
    }
    fn __go_clone_box_incer(&self) -> Box<dyn incer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn incer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_incer(&self, other: &(dyn incer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<counterPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

fn main() {
    let mut started = GoChannel::<bool>::new();
    let started_thread = started.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        started_thread.send(true);;;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    started.recv().unwrap_or_default();

    
    let mut all: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn speaker + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));
    { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = all.clone(); let __append_value = Arc::new(Mutex::new(Some(Box::new(dog { animal: Arc::new(Mutex::new(Some(animal { name: Arc::new(Mutex::new(Some("rex".to_string()))), ..Default::default() }))), breed: Arc::new(Mutex::new(Some("lab".to_string()))), ..Default::default() }) as Box<dyn speaker + Send + Sync>))); let mut __appended: Vec<Arc<Mutex<Option<Box<dyn speaker + Send + Sync>>>>> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *all.lock().unwrap() = __slice_header; };
    { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = all.clone(); let __append_value = Arc::new(Mutex::new(Some(Box::new(catPtr({
        let __owner = Arc::new(Mutex::new(Some(cat {
            animal: Arc::new(Mutex::new(Some(animal { name: Arc::new(Mutex::new(Some("tom".to_string()))), ..Default::default() }))).clone(),
            lives: Arc::new(Mutex::new(Some(9))),
            ..Default::default()
        })));
        let __embedded = { let __owner_guard = __owner.lock().unwrap(); __owner_guard.as_ref().unwrap().animal.clone() };
        let __embedded_key = { let __embedded_guard = __embedded.lock().unwrap(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) };
        go_register_embedded_owner(__embedded_key, __owner.clone());
        __owner
    }.clone())) as Box<dyn speaker + Send + Sync>))); let mut __appended: Vec<Arc<Mutex<Option<Box<dyn speaker + Send + Sync>>>>> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *all.lock().unwrap() = __slice_header; };
    { let __range_holder = all.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for s in __range_values.iter() {
        println!("{} {} {}", format!("{}", (*(*s.lock().unwrap().as_ref().unwrap()).name().lock().unwrap().as_ref().unwrap())), format!("{}", "says".to_string()), format!("{}", (*(*s.lock().unwrap().as_ref().unwrap()).sound().lock().unwrap().as_ref().unwrap())));
    } }

    let mut names = Arc::new(Mutex::new(Some(Vec::<String>::new())));
    { let __range_holder = all.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for s in __range_values.iter() {
        {
    let _ts_subject = s.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn speaker + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<dog>()).is_some() {
        let v = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<dog>()).unwrap().clone())));
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = names.clone(); let __append_value = format!("{}{}", "dog:".to_string(), (*{ let __field = (*v.lock().unwrap().as_ref().unwrap()).breed.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *names.lock().unwrap() = __slice_header; };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<catPtr>()).is_some() {
        let v = _ts_val.and_then(|__v| __v.downcast_ref::<catPtr>()).unwrap().0.clone();
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = names.clone(); let __append_value = (*Arc::new(Mutex::new(Some(format!("{}{}", format!("{}", "cat:".to_string()), format!("{}", (*{ let __field = (*v.lock().unwrap().as_ref().unwrap()).lives.clone(); __field }.lock().unwrap().as_ref().unwrap())))))).lock().unwrap().as_ref().unwrap()).clone(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *names.lock().unwrap() = __slice_header; };;
    }
    }
    } }
    { let __sort_target = names.clone(); let mut __sort_guard = __sort_target.lock().unwrap(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort(); } };
    println!("{}", format!("{}", (*Arc::new(Mutex::new(Some({ let __parts = (*names.lock().unwrap()).as_ref().cloned().unwrap_or_default(); let __sep = ",".to_string(); __parts.join(&__sep) }))).lock().unwrap().as_ref().unwrap())));

    
    let mut c = Arc::new(Mutex::new(Some(counter { n: Arc::new(Mutex::new(Some(0))) })));
    let mut i = Arc::new(Mutex::new(Some(Box::new(counterPtr(c.clone())) as Box<dyn incer + Send + Sync>)));
    (*i.lock().unwrap().as_mut().unwrap()).inc();
    (*i.lock().unwrap().as_mut().unwrap()).inc();
    println!("{} {}", format!("{}", "count".to_string()), format!("{}", (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).n.clone(); __field }.lock().unwrap().as_ref().unwrap())));

    
    let mut n = Arc::new(Mutex::new(Some(Box::new(dog { animal: Arc::new(Mutex::new(Some(animal { name: Arc::new(Mutex::new(Some("fido".to_string()))), ..Default::default() }))), breed: Arc::new(Mutex::new(Some("pug".to_string()))), ..Default::default() }) as Box<dyn namer + Send + Sync>)));
    {
        let (mut sp, mut ok) = ({
        let val = n.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn namer + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<catPtr>() {
                (Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn speaker + Send + Sync>))), true)
            } else if let Some(typed_val) = <dyn namer + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<dog>() {
                (Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn speaker + Send + Sync>))), true)
            } else {
                (Arc::new(Mutex::new(None::<Box<dyn speaker + Send + Sync>>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Box<dyn speaker + Send + Sync>>)), false)
        }
    });;
        if ok {
            println!("{} {}", format!("{}", "also a speaker:".to_string()), format!("{}", (*(*sp.lock().unwrap().as_ref().unwrap()).sound().lock().unwrap().as_ref().unwrap())));;
        }
    }
    print!("{}\n", (*(*n.lock().unwrap().as_ref().unwrap()).name().lock().unwrap().as_ref().unwrap()));
}