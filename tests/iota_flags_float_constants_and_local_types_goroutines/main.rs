#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use std::collections::BTreeMap;
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

fn format_slice<T, C>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[T]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| go_display_value(v)).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn format_slice_values<T>(slice: &[T]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| go_display_value(v)).collect();
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

fn go_float_string(value: f64, single: bool) -> String {
    if value.is_nan() {
        return "NaN".to_string();
    }
    if value.is_infinite() {
        return if value < 0.0 { "-Inf".to_string() } else { "+Inf".to_string() };
    }
    let sign = if value.is_sign_negative() { "-" } else { "" };
    let shortest = if single { format!("{:e}", value.abs() as f32) } else { format!("{:e}", value.abs()) };
    let (mantissa, exponent) = shortest.split_once('e').unwrap();
    let digits: String = mantissa.chars().filter(|c| c.is_ascii_digit()).collect();
    let exponent: i32 = exponent.parse().unwrap();
    if exponent < -4 || exponent >= 6 {
        let mut text = String::from(sign);
        text.push_str(&digits[..1]);
        if digits.len() > 1 {
            text.push('.');
            text.push_str(&digits[1..]);
        }
        text.push('e');
        text.push(if exponent < 0 { '-' } else { '+' });
        text.push_str(&format!("{:02}", exponent.abs()));
        return text;
    }
    if exponent < 0 {
        return format!("{}0.{}{}", sign, "0".repeat((-exponent - 1) as usize), digits);
    }
    let whole = (exponent + 1) as usize;
    if digits.len() <= whole {
        return format!("{}{}{}", sign, digits, "0".repeat(whole - digits.len()));
    }
    format!("{}{}.{}", sign, &digits[..whole], &digits[whole..])
}

// A value inside a slice or a map prints as fmt prints it: a float through
// go_float_string, anything else through its Display. The element type may
// borrow, so it is recognized by name rather than by TypeId.
fn go_display_value<T: Display>(value: &T) -> String {
    let name = std::any::type_name::<T>();
    if name == "f64" {
        // T is f64.
        return go_float_string(unsafe { *(value as *const T as *const f64) }, false);
    }
    if name == "f32" {
        // T is f32.
        return go_float_string(unsafe { *(value as *const T as *const f32) } as f64, true);
    }
    value.to_string()
}

pub const READ: u8 = 1 << 0;
pub const WRITE: u8 = 1 << 1;
pub const EXEC: u8 = 1 << 2;
pub const ALL: u8 = (READ as u8 | WRITE as u8) as u8 | EXEC as u8;


pub const K_B: f64 = 1024.0;
pub const M_B: f64 = 1048576.0;
pub const G_B: f64 = 1073741824.0;


#[derive(Debug)]
pub struct Perm(pub Arc<Mutex<Option<u8>>>);

impl Clone for Perm {
    fn clone(&self) -> Self {
        let __value = (*self.0.lock().unwrap()).clone();
        Perm(Arc::new(Mutex::new(__value)))
    }
}

impl Default for Perm {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(Some(Default::default()))))
    }
}

impl Display for Perm {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl PartialEq for Perm {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u8> for Perm {
    fn eq(&self, other: &u8) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Perm {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u8> for Perm {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Perm> for u8 {
    fn eq(&self, other: &Perm) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<Perm> for u8 {
    fn partial_cmp(&self, other: &Perm) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for Perm {
    type Output = Perm;
    fn add(self, other: Self) -> Perm {
        Perm(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u8> for Perm {
    type Output = Perm;
    fn add(self, other: u8) -> Perm {
        Perm(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Perm> for u8 {
    type Output = Perm;
    fn add(self, other: Perm) -> Perm {
        Perm(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Perm {
    type Output = Perm;
    fn sub(self, other: Self) -> Perm {
        Perm(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u8> for Perm {
    type Output = Perm;
    fn sub(self, other: u8) -> Perm {
        Perm(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Perm> for u8 {
    type Output = Perm;
    fn sub(self, other: Perm) -> Perm {
        Perm(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Perm {
    type Output = Perm;
    fn mul(self, other: Self) -> Perm {
        Perm(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u8> for Perm {
    type Output = Perm;
    fn mul(self, other: u8) -> Perm {
        Perm(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Perm> for u8 {
    type Output = Perm;
    fn mul(self, other: Perm) -> Perm {
        Perm(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Perm {
    type Output = Perm;
    fn div(self, other: Self) -> Perm {
        Perm(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u8> for Perm {
    type Output = Perm;
    fn div(self, other: u8) -> Perm {
        Perm(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Perm> for u8 {
    type Output = Perm;
    fn div(self, other: Perm) -> Perm {
        Perm(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for Perm {
    type Output = Perm;
    fn rem(self, other: Self) -> Perm {
        Perm(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u8> for Perm {
    type Output = Perm;
    fn rem(self, other: u8) -> Perm {
        Perm(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<Perm> for u8 {
    type Output = Perm;
    fn rem(self, other: Perm) -> Perm {
        Perm(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Perm {
    type Output = Perm;
    fn bitand(self, other: Self) -> Perm {
        Perm(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u8> for Perm {
    type Output = Perm;
    fn bitand(self, other: u8) -> Perm {
        Perm(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Perm> for u8 {
    type Output = Perm;
    fn bitand(self, other: Perm) -> Perm {
        Perm(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Perm {
    type Output = Perm;
    fn bitor(self, other: Self) -> Perm {
        Perm(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u8> for Perm {
    type Output = Perm;
    fn bitor(self, other: u8) -> Perm {
        Perm(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Perm> for u8 {
    type Output = Perm;
    fn bitor(self, other: Perm) -> Perm {
        Perm(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Perm {
    type Output = Perm;
    fn bitxor(self, other: Self) -> Perm {
        Perm(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u8> for Perm {
    type Output = Perm;
    fn bitxor(self, other: u8) -> Perm {
        Perm(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Perm> for u8 {
    type Output = Perm;
    fn bitxor(self, other: Perm) -> Perm {
        Perm(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Perm {
    type Output = Perm;
    fn not(self) -> Perm {
        Perm(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Perm {
    type Output = Perm;
    fn shl(self, other: Perm) -> Perm {
        Perm(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for Perm {
    type Output = Perm;
    fn shl(self, other: i32) -> Perm {
        Perm(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for Perm {
    type Output = Perm;
    fn shl(self, other: i8) -> Perm {
        Perm(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for Perm {
    type Output = Perm;
    fn shl(self, other: i16) -> Perm {
        Perm(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for Perm {
    type Output = Perm;
    fn shl(self, other: i64) -> Perm {
        Perm(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for Perm {
    type Output = Perm;
    fn shl(self, other: u32) -> Perm {
        Perm(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for Perm {
    type Output = Perm;
    fn shl(self, other: u8) -> Perm {
        Perm(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for Perm {
    type Output = Perm;
    fn shl(self, other: u16) -> Perm {
        Perm(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for Perm {
    type Output = Perm;
    fn shl(self, other: u64) -> Perm {
        Perm(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for Perm {
    type Output = Perm;
    fn shl(self, other: usize) -> Perm {
        Perm(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for Perm {
    type Output = Perm;
    fn shr(self, other: Perm) -> Perm {
        Perm(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for Perm {
    type Output = Perm;
    fn shr(self, other: i32) -> Perm {
        Perm(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for Perm {
    type Output = Perm;
    fn shr(self, other: i8) -> Perm {
        Perm(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for Perm {
    type Output = Perm;
    fn shr(self, other: i16) -> Perm {
        Perm(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for Perm {
    type Output = Perm;
    fn shr(self, other: i64) -> Perm {
        Perm(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for Perm {
    type Output = Perm;
    fn shr(self, other: u32) -> Perm {
        Perm(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for Perm {
    type Output = Perm;
    fn shr(self, other: u8) -> Perm {
        Perm(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for Perm {
    type Output = Perm;
    fn shr(self, other: u16) -> Perm {
        Perm(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for Perm {
    type Output = Perm;
    fn shr(self, other: u64) -> Perm {
        Perm(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for Perm {
    type Output = Perm;
    fn shr(self, other: usize) -> Perm {
        Perm(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for Perm {}

impl Ord for Perm {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug)]
pub struct Size(pub Arc<Mutex<Option<f64>>>);

impl Clone for Size {
    fn clone(&self) -> Self {
        let __value = (*self.0.lock().unwrap()).clone();
        Size(Arc::new(Mutex::new(__value)))
    }
}

impl Default for Size {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(Some(Default::default()))))
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", go_float_string(*self.0.lock().unwrap().as_ref().unwrap() as f64, false))
    }
}

impl PartialEq for Size {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<f64> for Size {
    fn eq(&self, other: &f64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Size {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<f64> for Size {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Size> for f64 {
    fn eq(&self, other: &Size) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<Size> for f64 {
    fn partial_cmp(&self, other: &Size) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for Size {
    type Output = Size;
    fn add(self, other: Self) -> Size {
        Size(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<f64> for Size {
    type Output = Size;
    fn add(self, other: f64) -> Size {
        Size(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Size> for f64 {
    type Output = Size;
    fn add(self, other: Size) -> Size {
        Size(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Size {
    type Output = Size;
    fn sub(self, other: Self) -> Size {
        Size(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<f64> for Size {
    type Output = Size;
    fn sub(self, other: f64) -> Size {
        Size(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Size> for f64 {
    type Output = Size;
    fn sub(self, other: Size) -> Size {
        Size(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Size {
    type Output = Size;
    fn mul(self, other: Self) -> Size {
        Size(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<f64> for Size {
    type Output = Size;
    fn mul(self, other: f64) -> Size {
        Size(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Size> for f64 {
    type Output = Size;
    fn mul(self, other: Size) -> Size {
        Size(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Size {
    type Output = Size;
    fn div(self, other: Self) -> Size {
        Size(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<f64> for Size {
    type Output = Size;
    fn div(self, other: f64) -> Size {
        Size(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Size> for f64 {
    type Output = Size;
    fn div(self, other: Size) -> Size {
        Size(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for Size {
    type Output = Size;
    fn neg(self) -> Size {
        Size(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}


pub(crate) static registry: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static order: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<String>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *registry.lock().unwrap() = Some(BTreeMap::new());
    *order.lock().unwrap() = Some(vec![]);
    *registry.lock().unwrap() = Some((*build_registry().lock().unwrap().as_ref().unwrap()).clone());
}


impl Perm {
    pub fn has(&self, q: Arc<Mutex<Option<Perm>>>) -> bool {
        return {
            let __tmp_x = Perm(Arc::new(Mutex::new(Some(((*self.0.lock().unwrap().as_ref().unwrap()) & (*{ let __v = (*q.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()))))));
            let __tmp_y = ({ let __v = (*q.lock().unwrap().as_ref().unwrap()).clone(); __v });
            __tmp_x == __tmp_y
        };
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        let mut s = Arc::new(Mutex::new(Some("".to_string())));
        for x in &vec![AnonymousStruct1 { bit: Arc::new(Mutex::new(Some(Perm(Arc::new(Mutex::new(Some(READ as u8))))))), ch: Arc::new(Mutex::new(Some(('r' as u8)))), ..Default::default() }, AnonymousStruct1 { bit: Arc::new(Mutex::new(Some(Perm(Arc::new(Mutex::new(Some(WRITE as u8))))))), ch: Arc::new(Mutex::new(Some(('w' as u8)))), ..Default::default() }, AnonymousStruct1 { bit: Arc::new(Mutex::new(Some(Perm(Arc::new(Mutex::new(Some(EXEC as u8))))))), ch: Arc::new(Mutex::new(Some(('x' as u8)))), ..Default::default() }] {
        let x = &x.__go_value_clone();
        if Perm::has(self, Arc::new(Mutex::new(Some({ let __selector_holder = x.bit.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) {
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&{ let __s = Arc::new(Mutex::new(Some(({ let __selector_holder = x.ch.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as char).to_string()))); let __value = (*__s.lock().unwrap().as_ref().unwrap()).clone(); __value }); };
    } else {
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&"-".to_string()); };
    }
    }
        return { let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }
}

pub fn build_registry() -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> {
    { let new_val = { let __collection_holder = { let __append_target = order.clone(); let __append_value = "registry".to_string(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) }.clone(); let __collection_guard = __collection_holder.lock().unwrap(); (*__collection_guard).clone() }; *order.lock().unwrap() = new_val; };
    Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some(1i64))))]))))
}

fn __go_init_0() {
    { let new_val = { let __collection_holder = { let __append_target = order.clone(); let __append_value = "init1".to_string(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) }.clone(); let __collection_guard = __collection_holder.lock().unwrap(); (*__collection_guard).clone() }; *order.lock().unwrap() = new_val; };
    { let __map_key = "b".to_string(); let __map_value = Arc::new(Mutex::new(Some(2))); (*registry.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
}

fn __go_init_1() {
    { let new_val = { let __collection_holder = { let __append_target = order.clone(); let __append_value = "init2".to_string(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) }.clone(); let __collection_guard = __collection_holder.lock().unwrap(); (*__collection_guard).clone() }; *order.lock().unwrap() = new_val; };
}

pub fn sum(nums: Arc<Mutex<Option<Vec<i64>>>>) -> i64 {
    let mut t = Arc::new(Mutex::new(Some(0i64)));
    { let __range_holder = nums.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for n in __range_values.iter().copied() {
        { let __rhs = n; let mut guard = t.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }
    return { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn apply(fs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync>>>>>>>>) -> Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync>>>> {
    let fs_closure_clone = fs.clone(); return Arc::new(Mutex::new(Some(Box::new(move |mut x: Arc<Mutex<Option<i64>>>| -> i64 {
        { let __range_holder = fs_closure_clone.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for f in __range_values.iter().cloned() {
        { let new_val = {
            let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync> = {
                let mut __f_guard = f.lock().unwrap();
                __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync>
            };
            let __f = unsafe { &mut *__f_ptr };
            (*__f)(
                x.clone()
            )
        }; *x.lock().unwrap() = Some(new_val); };
    } }
        return { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }) as Box<dyn FnMut(Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync>)));
}

fn main() {
    __go_init_all();
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
    println!("{} {} {} {} {} {} {}", format!("{}", Perm(Arc::new(Mutex::new(Some(READ as u8))))), format!("{}", Perm(Arc::new(Mutex::new(Some(WRITE as u8))))), format!("{}", Perm(Arc::new(Mutex::new(Some(EXEC as u8))))), format!("{}", Perm(Arc::new(Mutex::new(Some(ALL as u8))))), format!("{}", Perm(Arc::new(Mutex::new(Some(5 as u8))))), format!("{}", Perm(Arc::new(Mutex::new(Some(ALL as u8)))).has(Arc::new(Mutex::new(Some(Perm(Arc::new(Mutex::new(Some(WRITE as u8))))))))), format!("{}", Perm::has(&(Perm(Arc::new(Mutex::new(Some(4 as u8))))), Arc::new(Mutex::new(Some(Perm(Arc::new(Mutex::new(Some(READ as u8))))))))));
    println!("{} {} {} {}", format!("{}", Size(Arc::new(Mutex::new(Some(K_B))))), format!("{}", Size(Arc::new(Mutex::new(Some(1024.0))))), format!("{}", Size(Arc::new(Mutex::new(Some(1024.0))))), format!("{}", (*Arc::new(Mutex::new(Some(1.073741824e+09 as i64))).lock().unwrap().as_ref().unwrap())));
    println!("{} {} {}", format!("{}", format_slice(&order.clone())), format!("{}", ({ let __len = (*registry.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", { let __map = { let __map_holder = registry.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"b".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));
    let mut xs = Arc::new(Mutex::new(Some(vec![1i64, 2i64, 3i64])));
    println!("{} {} {} {}", format!("{}", sum(Arc::new(Mutex::new(Some(vec![]))))), format!("{}", sum(Arc::new(Mutex::new(Some(vec![1]))))), format!("{}", sum(xs.clone())), format!("{}", sum({ let __append_target = xs.clone(); let __append_value = 4; let mut __appended: Vec<i64> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })));
    let mut inc = Arc::new(Mutex::new(Some(Box::new(move |x: Arc<Mutex<Option<i64>>>| -> i64 {
        return { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y };
    }) as Box<dyn FnMut(Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync>)));
    let mut dbl = Arc::new(Mutex::new(Some(Box::new(move |x: Arc<Mutex<Option<i64>>>| -> i64 {
        return { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x * __tmp_y };
    }) as Box<dyn FnMut(Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync>)));
    println!("{} {}", format!("{}", { let __f_holder = apply(Arc::new(Mutex::new(Some(vec![inc.clone(), dbl.clone(), inc.clone()])))); let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(5)))) }), format!("{}", { let __f_holder = apply(Arc::new(Mutex::new(Some(vec![])))); let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(7)))) }));
    let mut fact: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync>>>> = Arc::new(Mutex::new(None));
    let fact_closure_clone = fact.clone(); { let __func_lit_target = fact_closure_clone.clone(); let new_val = Box::new(move |n: Arc<Mutex<Option<i64>>>| -> i64 {
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x <= __tmp_y } {
        return 1;
    }
        return { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = {
            let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync> = {
                let mut __f_guard = fact_closure_clone.lock().unwrap();
                __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync>
            };
            let __f = unsafe { &mut *__f_ptr };
            (*__f)(
                Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })))
            )
        }; __tmp_x * __tmp_y };
    }) as Box<dyn FnMut(Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync>; *__func_lit_target.lock().unwrap() = Some(new_val); };
    println!("{}", format!("{}", {
        let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync> = {
            let mut __f_guard = fact.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(
            Arc::new(Mutex::new(Some(10)))
        )
    }));
    let mut s = Arc::new(Mutex::new(Some("h\u{e9}llo, \u{4e16}\u{754c}".to_string())));
    let (mut bytes, mut runes) = (Arc::new(Mutex::new(Some(0i64))), Arc::new(Mutex::new(Some(0i64))));
    for _ in (*s.lock().unwrap().as_ref().unwrap()).chars() {
        { let mut guard = runes.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*s.lock().unwrap().as_ref().unwrap()).len(); __len }) as i64); __tmp_x < __tmp_y } {
        { let mut guard = bytes.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    println!("{} {} {} {} {}", format!("{}", { let __v = (*bytes.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*runes.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(1) as usize] }), format!("{}", (*Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (7) as usize; __s[__low..].to_string() }))).lock().unwrap().as_ref().unwrap()).to_string()))).lock().unwrap().as_ref().unwrap())), format!("{}", { let __seq = { let __seq_holder = Arc::new(Mutex::new(Some(({ let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }).chars().map(|c| c as i32).collect::<Vec<_>>()))).clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }));
    for (i, r) in "a\u{e9}".to_string().char_indices() {
        print!("{}{}{}{}", format!("{}", i), format!("{}", ":".to_string()), format!("{}", (*Arc::new(Mutex::new(Some(char::from_u32(((r as i32)) as u32).unwrap().to_string()))).lock().unwrap().as_ref().unwrap())), format!("{}", " ".to_string()));
    }
    println!();
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    'r#loop: loop {
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x < __tmp_y } {
        print!("{}{}{}", format!("{}", "g".to_string()), format!("{}", { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", " ".to_string()));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        continue 'r#loop;
    }
        println!();
        type local = AnonymousStruct2;
        let mut l = Arc::new(Mutex::new(Some(local { a: Arc::new(Mutex::new(Some(1))), b: Arc::new(Mutex::new(Some(2))), ..Default::default() })));
        let mut l2 = { let __owned = l.lock().unwrap().as_ref().unwrap().__go_value_clone(); Arc::new(Mutex::new(Some(__owned))) };
        { let new_val = 5; *(*l2.lock().unwrap().as_ref().unwrap()).a.lock().unwrap() = Some(new_val); };
        println!("{} {} {}", format!("{}", { let __v = (*l.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }), format!("{}", { let __v = (*l2.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }), format!("{}", { let __tmp_x = ({ let __v = (*l.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }); let __tmp_y = local { a: Arc::new(Mutex::new(Some(1))), b: Arc::new(Mutex::new(Some(2))), ..Default::default() }; __tmp_x == __tmp_y }));
        break 'r#loop;
    };
}

#[derive(Debug, Clone)]
pub struct AnonymousStruct1 {
    pub bit: Arc<Mutex<Option<Perm>>>,
    pub ch: Arc<Mutex<Option<u8>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.bit.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.ch.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            bit: __go_clone_0_0,
            ch: __go_clone_1_0,
        }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Perm(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            bit: __go_default_0_0,
            ch: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.bit.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.ch.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct2 {
    pub a: Arc<Mutex<Option<i64>>>,
    pub b: Arc<Mutex<Option<i64>>>,
}
impl PartialEq for AnonymousStruct2 {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.a.lock().unwrap().as_ref().cloned() }; let __right = { other.a.lock().unwrap().as_ref().cloned() }; __left == __right }
                && { let __left = { self.b.lock().unwrap().as_ref().cloned() }; let __right = { other.b.lock().unwrap().as_ref().cloned() }; __left == __right }
        )
    }
}
impl AnonymousStruct2 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.a.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.b.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            a: __go_clone_0_0,
            b: __go_clone_0_1,
        }
    }
}


impl Default for AnonymousStruct2 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_0_1 = Arc::new(Mutex::new(Some(0)));
        Self {
            a: __go_default_0_0,
            b: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for AnonymousStruct2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.a.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.b.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
    self::__go_init_0();
    self::__go_init_1();
}
