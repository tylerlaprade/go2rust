#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

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

pub trait Shape: std::fmt::Display + Any {
    fn __go_clone_box_shape(&self) -> Box<dyn Shape + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_shape(&self, other: &(dyn Shape + Send + Sync)) -> bool;
    fn area(&self) -> i64;
    fn name(&self) -> Arc<Mutex<Option<String>>>;
}

impl Clone for Box<dyn Shape + Send + Sync> {
    fn clone(&self) -> Self {
        Shape::__go_clone_box_shape(self.as_ref())
    }
}

#[derive(Debug, Clone)]
pub struct Rect {
    pub w: Arc<Mutex<Option<i64>>>,
    pub h: Arc<Mutex<Option<i64>>>,
}

impl Rect {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.w.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.h.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            w: __go_clone_0_0,
            h: __go_clone_0_1,
        }
    }
}


impl Default for Rect {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_0_1 = Arc::new(Mutex::new(Some(0)));
        Self {
            w: __go_default_0_0,
            h: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.w.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.h.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}
impl PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.w.lock().unwrap().as_ref().cloned() }; let __right = { other.w.lock().unwrap().as_ref().cloned() }; __left == __right }
                && { let __left = { self.h.lock().unwrap().as_ref().cloned() }; let __right = { other.h.lock().unwrap().as_ref().cloned() }; __left == __right }
        )
    }
}


#[derive(Debug, Clone)]
pub struct Square {
    pub s: Arc<Mutex<Option<i64>>>,
}

impl Square {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.s.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            s: __go_clone_0_0,
        }
    }
}


impl Default for Square {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            s: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.s.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}
impl PartialEq for Square {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.s.lock().unwrap().as_ref().cloned() }; let __right = { other.s.lock().unwrap().as_ref().cloned() }; __left == __right }
        )
    }
}


#[derive(Clone, Default)]
pub struct Pipeline {
    pub r#in: GoChannel<Option<Box<dyn Shape + Send + Sync>>>,
    pub out: GoChannel<Option<Box<dyn Shape + Send + Sync>>>,
}

impl Pipeline {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.r#in.clone();
        let __go_clone_1_0 = self.out.clone();
        Self {
            r#in: __go_clone_0_0,
            out: __go_clone_1_0,
        }
    }
}

impl std::fmt::Display for Pipeline {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


impl Rect {
    pub fn area(&self) -> i64 {
        return { let __tmp_x = (*self.w.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.h.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y };
    }

    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some("rect".to_string())))
    }
}

impl Shape for Rect {
    fn area(&self) -> i64 {
        Rect::area(self)
    }
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        Rect::name(self)
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Shape + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_shape(&self, other: &(dyn Shape + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Rect>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct RectPtr(pub Arc<Mutex<Option<Rect>>>);

impl std::fmt::Display for RectPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Shape for RectPtr {
    fn area(&self) -> i64 {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Rect::area(__recv)
    }
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Rect::name(__recv)
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Shape + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_shape(&self, other: &(dyn Shape + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<RectPtr>() {
            let __go_left_nil = { self.0.lock().unwrap().is_none() };
            let __go_right_nil = { __other.0.lock().unwrap().is_none() };
            if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&self.0, &__other.0) }
        } else {
            false
        }
    }
}

impl Square {
    pub fn area(&self) -> i64 {
        return { let __tmp_x = (*self.s.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.s.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y };
    }

    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some("square".to_string())))
    }
}

impl Shape for Square {
    fn area(&self) -> i64 {
        Square::area(self)
    }
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        Square::name(self)
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Shape + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_shape(&self, other: &(dyn Shape + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Square>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct SquarePtr(pub Arc<Mutex<Option<Square>>>);

impl std::fmt::Display for SquarePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Shape for SquarePtr {
    fn area(&self) -> i64 {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Square::area(__recv)
    }
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Square::name(__recv)
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Shape + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_shape(&self, other: &(dyn Shape + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SquarePtr>() {
            let __go_left_nil = { self.0.lock().unwrap().is_none() };
            let __go_right_nil = { __other.0.lock().unwrap().is_none() };
            if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&self.0, &__other.0) }
        } else {
            false
        }
    }
}

pub fn describe(s: Arc<Mutex<Option<Box<dyn Shape + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
    if { let __nil_result = (*s.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some("<nil shape>".to_string())));
    }
    Arc::new(Mutex::new(Some(format!("{}:{}", format!("{}", (*(*s.lock().unwrap().as_ref().unwrap()).name().lock().unwrap().as_ref().unwrap())), format!("{}", { let __arg_value = (*s.lock().unwrap().as_ref().unwrap()).area(); __arg_value })))))
}

pub fn receive(ch: GoChannel<Option<Box<dyn Shape + Send + Sync>>>) -> Arc<Mutex<Option<Box<dyn Shape + Send + Sync>>>> {
    Arc::new(Mutex::new(ch.recv().unwrap_or_default())).clone()
}

pub fn receive_named(ch: GoChannel<Option<Box<dyn Shape + Send + Sync>>>) -> Arc<Mutex<Option<Box<dyn Shape + Send + Sync>>>> {
    let mut s: Arc<Mutex<Option<Box<dyn Shape + Send + Sync>>>> = Arc::new(Mutex::new(None));

    { let new_val = ch.recv().unwrap_or_default(); *s.lock().unwrap() = new_val; };
    s.clone()
}

pub fn send(ch: GoChannel<Option<Box<dyn Shape + Send + Sync>>>, s: Arc<Mutex<Option<Box<dyn Shape + Send + Sync>>>>) {
    ch.send({ let __iface_handle = { let __iface_value = s.lock().unwrap().clone(); Arc::new(Mutex::new(__iface_value)) }; let __iface_value = (*__iface_handle.lock().unwrap()).clone(); __iface_value });
}

fn main() {
    let mut ch = GoChannel::<Option<Box<dyn Shape + Send + Sync>>>::new_buffered(8 as usize);
    let mut nothing: Arc<Mutex<Option<Box<dyn Shape + Send + Sync>>>> = Arc::new(Mutex::new(None));
    let mut sq = Arc::new(Mutex::new(Some(Square { s: Arc::new(Mutex::new(Some(3))), ..Default::default() })));

    ch.send({ let __iface_handle = Arc::new(Mutex::new(Some(Box::new(Rect { w: Arc::new(Mutex::new(Some(2))), h: Arc::new(Mutex::new(Some(3))), ..Default::default() }) as Box<dyn Shape + Send + Sync>))); let __iface_value = (*__iface_handle.lock().unwrap()).clone(); __iface_value });
    ch.send({ let __iface_handle = Arc::new(Mutex::new(Some(Box::new(SquarePtr(sq.clone())) as Box<dyn Shape + Send + Sync>))); let __iface_value = (*__iface_handle.lock().unwrap()).clone(); __iface_value });
    ch.send(None::<Box<dyn Shape + Send + Sync>>);
    ch.send({ let __iface_handle = { let __iface_value = nothing.lock().unwrap().clone(); Arc::new(Mutex::new(__iface_value)) }; let __iface_value = (*__iface_handle.lock().unwrap()).clone(); __iface_value });
    send(
        ch.clone(),
        Arc::new(Mutex::new(Some(Box::new(Rect { w: Arc::new(Mutex::new(Some(1))), h: Arc::new(Mutex::new(Some(1))), ..Default::default() }) as Box<dyn Shape + Send + Sync>)))
    );

    let mut first = Arc::new(Mutex::new(ch.recv().unwrap_or_default()));
    println!("{}", format!("{}", (*describe(first.clone()).lock().unwrap().as_ref().unwrap())));
    let mut second: Arc<Mutex<Option<Box<dyn Shape + Send + Sync>>>> = Arc::new(Mutex::new(None));
    { let new_val = ch.recv().unwrap_or_default(); *second.lock().unwrap() = new_val; };
    println!("{}", format!("{}", (*describe(second.clone()).lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", (*describe(Arc::new(Mutex::new(ch.recv().unwrap_or_default())).clone()).lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", { let __nil_result = (*Arc::new(Mutex::new(ch.recv().unwrap_or_default())).lock().unwrap()).is_none(); __nil_result }));
    println!("{}", format!("{}", (*describe(receive(ch.clone()).clone()).lock().unwrap().as_ref().unwrap())));

        // The value sent is the interface value at the time of the send.
    let mut cur = Arc::new(Mutex::new(Some(Box::new(Rect { w: Arc::new(Mutex::new(Some(4))), h: Arc::new(Mutex::new(Some(4))), ..Default::default() }) as Box<dyn Shape + Send + Sync>)));
    ch.send({ let __iface_handle = { let __iface_value = cur.lock().unwrap().clone(); Arc::new(Mutex::new(__iface_value)) }; let __iface_value = (*__iface_handle.lock().unwrap()).clone(); __iface_value });
    { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(Rect { w: Arc::new(Mutex::new(Some(5))), h: Arc::new(Mutex::new(Some(5))), ..Default::default() }) as Box<dyn Shape + Send + Sync>))); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *cur.lock().unwrap() = __iface_value; };
    let mut got = Arc::new(Mutex::new(ch.recv().unwrap_or_default()));
    println!("{} {}", format!("{}", (*describe(got.clone()).lock().unwrap().as_ref().unwrap())), format!("{}", (*describe(cur.clone()).lock().unwrap().as_ref().unwrap())));

        // A pointer inside the value still points at the same struct.
    ch.send({ let __iface_handle = Arc::new(Mutex::new(Some(Box::new(SquarePtr(sq.clone())) as Box<dyn Shape + Send + Sync>))); let __iface_value = (*__iface_handle.lock().unwrap()).clone(); __iface_value });
    { let new_val = 10; *(*sq.lock().unwrap().as_ref().unwrap()).s.lock().unwrap() = Some(new_val); };
    println!("{}", format!("{}", (*describe(receive_named(ch.clone()).clone()).lock().unwrap().as_ref().unwrap())));

        // Comma-ok, and a closed channel gives nil.
    ch.send({ let __iface_handle = Arc::new(Mutex::new(Some(Box::new(Rect { w: Arc::new(Mutex::new(Some(6))), h: Arc::new(Mutex::new(Some(1))), ..Default::default() }) as Box<dyn Shape + Send + Sync>))); let __iface_value = (*__iface_handle.lock().unwrap()).clone(); __iface_value });
    ch.close();
    let (mut s, mut ok) = match ch.recv() { Some(v) => (Arc::new(Mutex::new(v)), Arc::new(Mutex::new(Some(true)))), None => (Arc::new(Mutex::new(None::<Box<dyn Shape + Send + Sync>>)), Arc::new(Mutex::new(Some(false)))) };
    println!("{} {}", format!("{}", (*describe(s.clone()).lock().unwrap().as_ref().unwrap())), format!("{}", { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    match ch.recv() { Some(v) => { *s.lock().unwrap() = v; *ok.lock().unwrap() = Some(true); }, None => { *s.lock().unwrap() = None::<Box<dyn Shape + Send + Sync>>; *ok.lock().unwrap() = Some(false); } };
    println!("{} {}", format!("{}", (*describe(s.clone()).lock().unwrap().as_ref().unwrap())), format!("{}", { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    let mut last = Arc::new(Mutex::new(ch.recv().unwrap_or_default()));
    println!("{}", format!("{}", { let __nil_result = (*last.lock().unwrap()).is_none(); __nil_result }));

        // Range, select and a channel in a struct field.
    let mut p = Arc::new(Mutex::new(Some(Pipeline { r#in: GoChannel::<Option<Box<dyn Shape + Send + Sync>>>::new_buffered(4 as usize), out: GoChannel::<Option<Box<dyn Shape + Send + Sync>>>::new_buffered(4 as usize), ..Default::default() })));
    { let __channel = (*p.lock().unwrap().as_ref().unwrap()).r#in.clone(); __channel }.send({ let __iface_handle = Arc::new(Mutex::new(Some(Box::new(Rect { w: Arc::new(Mutex::new(Some(1))), h: Arc::new(Mutex::new(Some(2))), ..Default::default() }) as Box<dyn Shape + Send + Sync>))); let __iface_value = (*__iface_handle.lock().unwrap()).clone(); __iface_value });
    { let __channel = (*p.lock().unwrap().as_ref().unwrap()).r#in.clone(); __channel }.send({ let __iface_handle = Arc::new(Mutex::new(Some(Box::new(SquarePtr(Arc::new(Mutex::new(Some(Square { s: Arc::new(Mutex::new(Some(2))), ..Default::default() }))).clone())) as Box<dyn Shape + Send + Sync>))); let __iface_value = (*__iface_handle.lock().unwrap()).clone(); __iface_value });
    { let __channel = (*p.lock().unwrap().as_ref().unwrap()).r#in.clone(); __channel }.send(None::<Box<dyn Shape + Send + Sync>>);
    { let __channel = (*p.lock().unwrap().as_ref().unwrap()).r#in.clone(); __channel }.close();
    let mut total = Arc::new(Mutex::new(Some(0i64)));
    for __iface_value in { let __channel = (*p.lock().unwrap().as_ref().unwrap()).r#in.clone(); __channel } {
        let mut shape = Arc::new(Mutex::new(__iface_value));
        if { let __nil_result = (*shape.lock().unwrap()).is_none(); __nil_result } {
        println!("{}", format!("{}", "skip nil".to_string()));
        continue
    }
        { let __rhs = (*shape.lock().unwrap().as_ref().unwrap()).area(); let mut guard = total.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __channel = (*p.lock().unwrap().as_ref().unwrap()).out.clone(); __channel }.send({ let __iface_handle = { let __iface_value = shape.lock().unwrap().clone(); Arc::new(Mutex::new(__iface_value)) }; let __iface_value = (*__iface_handle.lock().unwrap()).clone(); __iface_value });
    }
    println!("{} {}", format!("{}", "total".to_string()), format!("{}", { let __v = (*total.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x < __tmp_y } {
        loop {
        if let Some(__select_recv) = { let __channel = (*p.lock().unwrap().as_ref().unwrap()).out.clone(); __channel }.try_recv_select() {
            let __select_ok = __select_recv.is_some();
            let shape = match __select_recv { Some(__v) => __v, None => Default::default() };
            let mut shape = Arc::new(Mutex::new(shape));
            println!("{} {}", format!("{}", "selected".to_string()), format!("{}", (*describe(shape.clone()).lock().unwrap().as_ref().unwrap())));
            break;
        }
        println!("{}", format!("{}", "nothing ready".to_string()));
        break;
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    loop {
        if { let __channel = (*p.lock().unwrap().as_ref().unwrap()).out.clone(); __channel }.try_send({ let __iface_handle = Arc::new(Mutex::new(Some(Box::new(Rect { w: Arc::new(Mutex::new(Some(7))), h: Arc::new(Mutex::new(Some(7))), ..Default::default() }) as Box<dyn Shape + Send + Sync>))); let __iface_value = (*__iface_handle.lock().unwrap()).clone(); __iface_value }) {
            println!("{}", format!("{}", "sent".to_string()));
            break;
        }
        println!("{}", format!("{}", "full".to_string()));
        break;
    }
    println!("{}", format!("{}", (*describe(Arc::new(Mutex::new({ let __channel = (*p.lock().unwrap().as_ref().unwrap()).out.clone(); __channel }.recv().unwrap_or_default())).clone()).lock().unwrap().as_ref().unwrap())));

    let mut shapes = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Box::new(Rect { w: Arc::new(Mutex::new(Some(1))), h: Arc::new(Mutex::new(Some(3))), ..Default::default() }) as Box<dyn Shape + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(SquarePtr(Arc::new(Mutex::new(Some(Square { s: Arc::new(Mutex::new(Some(4))), ..Default::default() }))).clone())) as Box<dyn Shape + Send + Sync>)))])));
    { let __range_holder = shapes.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for shape in __range_values.iter() {
        { let __channel = (*p.lock().unwrap().as_ref().unwrap()).out.clone(); __channel }.send({ let __iface_handle = { let __iface_value = (*shape).lock().unwrap().clone(); Arc::new(Mutex::new(__iface_value)) }; let __iface_value = (*__iface_handle.lock().unwrap()).clone(); __iface_value });
    } }
    { let __range_holder = shapes.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for _ in __range_values.iter() {
        println!("{}", format!("{}", (*describe(Arc::new(Mutex::new({ let __channel = (*p.lock().unwrap().as_ref().unwrap()).out.clone(); __channel }.recv().unwrap_or_default())).clone()).lock().unwrap().as_ref().unwrap())));
    } }
}