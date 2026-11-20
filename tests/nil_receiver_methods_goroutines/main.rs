#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use std::any::Any;
use std::collections::BTreeMap;
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

fn format_slice<T, C>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[T]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn format_slice_values<T>(slice: &[T]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| v.to_string()).collect();
    format!("[{}]", formatted.join(" "))
}

fn format_slice_wrapped<T, C>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[Arc<Mutex<Option<T>>>]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| {
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
pub struct Node {
    pub value: Arc<Mutex<Option<i64>>>,
    pub next: Arc<Mutex<Option<Node>>>,
}

impl Node {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.value.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.next.clone();
        Self {
            value: __go_clone_0_0,
            next: __go_clone_1_0,
        }
    }
}


impl Default for Node {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        Self {
            value: __go_default_0_0,
            next: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.value.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { let __guard = self.next.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct Tree {
    pub left: Arc<Mutex<Option<Tree>>>,
    pub right: Arc<Mutex<Option<Tree>>>,
    pub val: Arc<Mutex<Option<i64>>>,
}

impl Tree {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.left.clone();
        let __go_clone_0_1 = self.right.clone();
        let __go_clone_1_0 = { let __guard = self.val.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            left: __go_clone_0_0,
            right: __go_clone_0_1,
            val: __go_clone_1_0,
        }
    }
}


impl Default for Tree {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_0_1 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            left: __go_default_0_0,
            right: __go_default_0_1,
            val: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.left.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", { let __guard = self.right.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_2 = format!("{}", (*self.val.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


#[derive(Debug, Clone)]
pub struct List {
    pub head: Arc<Mutex<Option<Elem>>>,
    pub size: Arc<Mutex<Option<i64>>>,
}

impl List {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.head.clone();
        let __go_clone_1_0 = { let __guard = self.size.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            head: __go_clone_0_0,
            size: __go_clone_1_0,
        }
    }
}


impl Default for List {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            head: __go_default_0_0,
            size: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.head.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", (*self.size.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct Elem {
    pub val: Arc<Mutex<Option<String>>>,
    pub next: Arc<Mutex<Option<Elem>>>,
}

impl Elem {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.val.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.next.clone();
        Self {
            val: __go_clone_0_0,
            next: __go_clone_1_0,
        }
    }
}


impl Default for Elem {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        Self {
            val: __go_default_0_0,
            next: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Elem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}
impl PartialEq for Elem {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.val.lock().unwrap().as_ref().cloned() }; let __right = { other.val.lock().unwrap().as_ref().cloned() }; __left == __right }
                && { let __left_some = self.next.lock().unwrap().is_some(); let __right_some = other.next.lock().unwrap().is_some(); (!__left_some && !__right_some) || (__left_some && __right_some && Arc::ptr_eq(&self.next, &other.next)) }
        )
    }
}


pub trait Stringer: std::fmt::Display + Any {
    fn __go_clone_box_stringer(&self) -> Box<dyn Stringer + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_stringer(&self, other: &(dyn Stringer + Send + Sync)) -> bool;
    fn string(&self) -> Arc<Mutex<Option<String>>>;
}

impl Clone for Box<dyn Stringer + Send + Sync> {
    fn clone(&self) -> Self {
        Stringer::__go_clone_box_stringer(self.as_ref())
    }
}

#[derive(Debug, Clone)]
pub struct Counter {
    pub hits: Arc<Mutex<Option<i64>>>,
}

impl Counter {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.hits.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            hits: __go_clone_0_0,
        }
    }
}


impl Default for Counter {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            hits: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.hits.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Node {
    pub fn len(&self) -> i64 {
        if false {
        return 0;
    }
        return { let __tmp_x = 1; let __tmp_y = __go_nil_recv_node_len({ let __field = self.next.clone(); __field }); __tmp_x + __tmp_y };
    }

    pub fn sum(&self) -> i64 {
        if false {
        return 0;
    }
        return {
            let __tmp_x = (*self.value.lock().unwrap().as_ref().unwrap());
            let __tmp_y = __go_nil_recv_node_sum({ let __field = self.next.clone(); __field });
            __tmp_x + __tmp_y
        };
    }
}

impl Tree {
    pub fn insert(&mut self, v: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<Tree>>> {
        if false {
        return Arc::new(Mutex::new(Some(Tree { val: Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
    }
        if { let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.val.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let new_val = __go_nil_recv_tree_insert({ let __field = self.left.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); self.left = new_val; };
    } else {
        { let new_val = __go_nil_recv_tree_insert({ let __field = self.right.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); self.right = new_val; };
    }
        Arc::new(Mutex::new(Some(self.clone())))
    }

    pub fn in_order(&self, visit: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<i64>>>) -> () + Send + Sync>>>>) {
        if false {
        return;
    }
        __go_nil_recv_tree_in_order({ let __field = self.left.clone(); __field }, visit.clone());
        {
            let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i64>>>) -> () + Send + Sync> = {
                let mut __f_guard = visit.lock().unwrap();
                __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i64>>>) -> () + Send + Sync>
            };
            let __f = unsafe { &mut *__f_ptr };
            (*__f)(
                Arc::new(Mutex::new(Some({ let __selector_holder = self.val.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))
            )
        };
        __go_nil_recv_tree_in_order({ let __field = self.right.clone(); __field }, visit.clone());
    }

    pub fn height(&self) -> i64 {
        if false {
        return 0;
    }
        let (mut l, mut r) = (__go_nil_recv_tree_height({ let __field = self.left.clone(); __field }), __go_nil_recv_tree_height({ let __field = self.right.clone(); __field }));
        if { let __tmp_x = l; let __tmp_y = r; __tmp_x > __tmp_y } {
        return { let __tmp_x = l; let __tmp_y = 1; __tmp_x + __tmp_y };
    }
        return { let __tmp_x = r; let __tmp_y = 1; __tmp_x + __tmp_y };
    }

    pub fn contains(&mut self, v: Arc<Mutex<Option<i64>>>) -> bool {
        let mut __self = Arc::new(Mutex::new(Some(self.clone())));
        while { let __self_guard = __self.lock().unwrap(); __self_guard.is_some() } {
        if { let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*(*__self.lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } {
        return true;
    }
        if { let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*(*__self.lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let new_val = (*__self.lock().unwrap().as_ref().unwrap()).left.clone(); __self = new_val; };
    } else {
        { let new_val = (*__self.lock().unwrap().as_ref().unwrap()).right.clone(); __self = new_val; };
    }
    }
        false
    }
}

impl Elem {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        if false {
        return Arc::new(Mutex::new(Some("nil".to_string())));
    }
        return Arc::new(Mutex::new(Some({
            let mut __s = String::new();
            __s.push_str(&format!("{}", (*self.val.clone().lock().unwrap().as_ref().unwrap())));
            __s.push_str(&format!("{}", "->".to_string()));
            __s.push_str(&format!("{}", (*__go_nil_recv_elem_string({ let __field = self.next.clone(); __field }).lock().unwrap().as_ref().unwrap())));
            __s
        })));
    }

    pub fn append(&mut self, v: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Elem>>> {
        if false {
        return Arc::new(Mutex::new(Some(Elem { val: Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
    }
        { let new_val = __go_nil_recv_elem_append({ let __field = self.next.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); self.next = new_val; };
        Arc::new(Mutex::new(Some(self.clone())))
    }
}

impl Stringer for Elem {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        Elem::string(self)
    }
    fn __go_clone_box_stringer(&self) -> Box<dyn Stringer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stringer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_stringer(&self, other: &(dyn Stringer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Elem>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct ElemPtr(pub Arc<Mutex<Option<Elem>>>);

impl std::fmt::Display for ElemPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Stringer for ElemPtr {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Elem::string(__recv)
    }
    fn __go_clone_box_stringer(&self) -> Box<dyn Stringer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stringer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_stringer(&self, other: &(dyn Stringer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ElemPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl List {
    pub fn push(&mut self, v: Arc<Mutex<Option<String>>>) {
        { let new_val = __go_nil_recv_elem_append({ let __field = self.head.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); self.head = new_val; };
        { let __target = self.size.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

impl Counter {
    pub fn hit(&mut self) -> i64 {
        if false {
        return -(1);
    }
        { let __target = self.hits.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return (*self.hits.lock().unwrap().as_ref().unwrap());
    }
}

pub fn describe(s: Arc<Mutex<Option<Box<dyn Stringer + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
    return Arc::new(Mutex::new(Some({
        let mut __s = String::new();
        __s.push_str(&format!("{}", "[".to_string()));
        __s.push_str(&format!("{}", (*(*s.lock().unwrap().as_ref().unwrap()).string().lock().unwrap().as_ref().unwrap())));
        __s.push_str(&format!("{}", "]".to_string()));
        __s
    })));
}

pub fn more() {
    let mut l: Arc<Mutex<Option<List>>> = Arc::new(Mutex::new(Some(Default::default())));
    { let __range_holder = Arc::new(Mutex::new(Some({ let __s = "a b c".to_string(); __s.split_whitespace().map(|__part| __part.to_string()).collect::<Vec<String>>() }))).clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for w in __range_values.iter() {
        (*l.lock().unwrap().as_mut().unwrap()).push(Arc::new(Mutex::new(Some((*w).clone()))));
    } }
    println!("{} {}", format!("{}", (*__go_nil_recv_elem_string({ let __field = (*l.lock().unwrap().as_ref().unwrap()).head.clone(); __field }).lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __field = (*l.lock().unwrap().as_ref().unwrap()).size.clone(); __field }.lock().unwrap().as_ref().unwrap())));
    let mut none: Arc<Mutex<Option<Elem>>> = Arc::new(Mutex::new(None));
    println!("{} {}", format!("{}", (*__go_nil_recv_elem_string(none.clone()).lock().unwrap().as_ref().unwrap())), format!("{}", (*describe(Arc::new(Mutex::new(Some(Box::new(ElemPtr((*l.lock().unwrap().as_ref().unwrap()).head.clone())) as Box<dyn Stringer + Send + Sync>)))).lock().unwrap().as_ref().unwrap())));
    let mut c = Arc::new(Mutex::new(Some(Counter { hits: Arc::new(Mutex::new(Some(0))) })));
    __go_nil_recv_counter_hit(c.clone());
    __go_nil_recv_counter_hit(c.clone());
    let mut missing: Arc<Mutex<Option<Counter>>> = Arc::new(Mutex::new(None));
    println!("{} {} {}", format!("{}", __go_nil_recv_counter_hit(c.clone())), format!("{}", (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).hits.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{}", __go_nil_recv_counter_hit(missing.clone())));
    let mut cs = Arc::new(Mutex::new(Some(vec![c.clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Counter { ..Default::default() })))])));
    { let __range_holder = cs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter() {
        print!("{}{}", format!("{}", __go_nil_recv_counter_hit((*x).clone())), format!("{}", " ".to_string()));
    } }
    println!();
    let mut byName = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Counter>>>>::from([("a".to_string(), c.clone())]))));
    println!("{} {}", format!("{}", __go_nil_recv_counter_hit({ let __map = { let __map_holder = byName.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) })), format!("{}", __go_nil_recv_counter_hit({ let __map = { let __map_holder = byName.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"zz".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) })));
    let mut done = GoChannel::<i64>::new();
    let c_thread = c.clone(); let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send(__go_nil_recv_counter_hit(c_thread.clone()));;;
    });
    println!("{}", format!("{}", done.recv().unwrap_or_default()));
}

fn main() {
    let mut started = GoChannel::<bool>::new();
    let started_thread = started.clone(); std::thread::spawn(move || {
        started_thread.send(true);;;
    });
    started.recv().unwrap_or_default();

    let mut list = Arc::new(Mutex::new(Some(Node {
        value: Arc::new(Mutex::new(Some(1))),
        next: Arc::new(Mutex::new(Some(Node {
            value: Arc::new(Mutex::new(Some(2))),
            next: Arc::new(Mutex::new(Some(Node { value: Arc::new(Mutex::new(Some(3))), next: Default::default(), ..Default::default() }))).clone(),
            ..Default::default()
        }))).clone(),
        ..Default::default()
    })));
    println!("{} {}", format!("{}", __go_nil_recv_node_len(list.clone())), format!("{}", __go_nil_recv_node_sum(list.clone())));
    let mut empty: Arc<Mutex<Option<Node>>> = Arc::new(Mutex::new(None));
    println!("{} {}", format!("{}", __go_nil_recv_node_len(empty.clone())), format!("{}", __go_nil_recv_node_sum(empty.clone())));

    let mut root: Arc<Mutex<Option<Tree>>> = Arc::new(Mutex::new(None));
    for v in vec![5i64, 3i64, 8i64, 1i64, 4i64, 9i64].iter().copied() {
        { let new_val = __go_nil_recv_tree_insert(root.clone(), Arc::new(Mutex::new(Some(v.clone())))).clone(); root = new_val; };
    }
    let mut out: Arc<Mutex<Option<Vec<i64>>>> = Arc::new(Mutex::new(None));
    let mut out_closure_clone = out.clone(); __go_nil_recv_tree_in_order(root.clone(), Arc::new(Mutex::new(Some(Box::new(move |v: Arc<Mutex<Option<i64>>>| {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = out_closure_clone.clone(); let __append_value = (*v.lock().unwrap().as_ref().unwrap()).clone(); let mut __appended: Vec<i64> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *out_closure_clone.lock().unwrap() = __slice_header; };
    }) as Box<dyn FnMut(Arc<Mutex<Option<i64>>>) -> () + Send + Sync>))));
    println!("{} {} {} {}", format!("{}", format_slice(&out)), format!("{}", __go_nil_recv_tree_height(root.clone())), format!("{}", __go_nil_recv_tree_contains(root.clone(), Arc::new(Mutex::new(Some(4))))), format!("{}", __go_nil_recv_tree_contains(root.clone(), Arc::new(Mutex::new(Some(7))))));
    more();
}

pub fn __go_nil_recv_counter_hit(c: Arc<Mutex<Option<Counter>>>) -> i64 {
    if { let __nil_result = (*c.lock().unwrap()).is_none(); __nil_result } {
        return -(1);
    }
    { let __target = (*c.lock().unwrap().as_ref().unwrap()).hits.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    return (*(*c.lock().unwrap().as_ref().unwrap()).hits.lock().unwrap().as_ref().unwrap());
}

pub fn __go_nil_recv_elem_append(e: Arc<Mutex<Option<Elem>>>, v: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Elem>>> {
    if { let __nil_result = (*e.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some(Elem { val: Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
    }
    { let new_val = __go_nil_recv_elem_append({ let __field = (*e.lock().unwrap().as_ref().unwrap()).next.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); (*e.lock().unwrap().as_mut().unwrap()).next = new_val; };
    e.clone()
}

pub fn __go_nil_recv_elem_string(e: Arc<Mutex<Option<Elem>>>) -> Arc<Mutex<Option<String>>> {
    if { let __nil_result = (*e.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some("nil".to_string())));
    }
    return Arc::new(Mutex::new(Some({
        let mut __s = String::new();
        __s.push_str(&format!("{}", (*{ let __field = (*e.lock().unwrap().as_ref().unwrap()).val.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()));
        __s.push_str(&format!("{}", "->".to_string()));
        __s.push_str(&format!("{}", (*__go_nil_recv_elem_string({ let __field = (*e.lock().unwrap().as_ref().unwrap()).next.clone(); __field }).lock().unwrap().as_ref().unwrap())));
        __s
    })));
}

pub fn __go_nil_recv_node_len(n: Arc<Mutex<Option<Node>>>) -> i64 {
    if { let __nil_result = (*n.lock().unwrap()).is_none(); __nil_result } {
        return 0;
    }
    return { let __tmp_x = 1; let __tmp_y = __go_nil_recv_node_len({ let __field = (*n.lock().unwrap().as_ref().unwrap()).next.clone(); __field }); __tmp_x + __tmp_y };
}

pub fn __go_nil_recv_node_sum(n: Arc<Mutex<Option<Node>>>) -> i64 {
    if { let __nil_result = (*n.lock().unwrap()).is_none(); __nil_result } {
        return 0;
    }
    return {
        let __tmp_x = (*{ let __field = (*n.lock().unwrap().as_ref().unwrap()).value.clone(); __field }.lock().unwrap().as_ref().unwrap());
        let __tmp_y = __go_nil_recv_node_sum({ let __field = (*n.lock().unwrap().as_ref().unwrap()).next.clone(); __field });
        __tmp_x + __tmp_y
    };
}

pub fn __go_nil_recv_tree_contains(mut t: Arc<Mutex<Option<Tree>>>, v: Arc<Mutex<Option<i64>>>) -> bool {
    while { let __nil_result = (*t.lock().unwrap()).is_some(); __nil_result } {
        if { let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).val.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } {
        return true;
    }
        if { let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).val.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let new_val = (*t.lock().unwrap().as_ref().unwrap()).left.clone(); t = new_val; };
    } else {
        { let new_val = (*t.lock().unwrap().as_ref().unwrap()).right.clone(); t = new_val; };
    }
    }
    false
}

pub fn __go_nil_recv_tree_height(t: Arc<Mutex<Option<Tree>>>) -> i64 {
    if { let __nil_result = (*t.lock().unwrap()).is_none(); __nil_result } {
        return 0;
    }
    let (mut l, mut r) = ({ let __tuple_value = __go_nil_recv_tree_height({ let __field = (*t.lock().unwrap().as_ref().unwrap()).left.clone(); __field }); __tuple_value }, { let __tuple_value = __go_nil_recv_tree_height({ let __field = (*t.lock().unwrap().as_ref().unwrap()).right.clone(); __field }); __tuple_value });
    if { let __tmp_x = l; let __tmp_y = r; __tmp_x > __tmp_y } {
        return { let __tmp_x = l; let __tmp_y = 1; __tmp_x + __tmp_y };
    }
    return { let __tmp_x = r; let __tmp_y = 1; __tmp_x + __tmp_y };
}

pub fn __go_nil_recv_tree_in_order(t: Arc<Mutex<Option<Tree>>>, visit: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<i64>>>) -> () + Send + Sync>>>>) {
    if { let __nil_result = (*t.lock().unwrap()).is_none(); __nil_result } {
        return;
    }
    __go_nil_recv_tree_in_order({ let __field = (*t.lock().unwrap().as_ref().unwrap()).left.clone(); __field }, visit.clone());
    {
        let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i64>>>) -> () + Send + Sync> = {
            let mut __f_guard = visit.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i64>>>) -> () + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(
            Arc::new(Mutex::new(Some({ let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).val.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))
        )
    };
    __go_nil_recv_tree_in_order({ let __field = (*t.lock().unwrap().as_ref().unwrap()).right.clone(); __field }, visit.clone());
}

pub fn __go_nil_recv_tree_insert(t: Arc<Mutex<Option<Tree>>>, v: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<Tree>>> {
    if { let __nil_result = (*t.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some(Tree { val: Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
    }
    if { let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).val.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let new_val = __go_nil_recv_tree_insert({ let __field = (*t.lock().unwrap().as_ref().unwrap()).left.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); (*t.lock().unwrap().as_mut().unwrap()).left = new_val; };
    } else {
        { let new_val = __go_nil_recv_tree_insert({ let __field = (*t.lock().unwrap().as_ref().unwrap()).right.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); (*t.lock().unwrap().as_mut().unwrap()).right = new_val; };
    }
    t.clone()
}