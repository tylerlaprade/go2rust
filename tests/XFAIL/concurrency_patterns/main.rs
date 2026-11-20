#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use std::any::Any;
use std::cell::{RefCell};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

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


struct GoPanicHookSuppression;

thread_local! {
    static __GO_PANIC_HOOK_SUPPRESSION_DEPTH: std::cell::Cell<usize> = std::cell::Cell::new(0);
    static __GO_FUNCTION_DEPTH: std::cell::Cell<usize> = std::cell::Cell::new(0);
}

static __GO_PANIC_HOOK_INIT: std::sync::Once = std::sync::Once::new();

struct GoFunctionDepthGuard;

fn go_enter_function() -> GoFunctionDepthGuard {
    __GO_FUNCTION_DEPTH.with(|depth| depth.set(depth.get() + 1));
    GoFunctionDepthGuard
}

fn go_current_function_depth() -> usize {
    __GO_FUNCTION_DEPTH.with(|depth| depth.get())
}

impl Drop for GoFunctionDepthGuard {
    fn drop(&mut self) {
        __GO_FUNCTION_DEPTH.with(|depth| depth.set(depth.get() - 1));
    }
}

fn go_suppress_panic_hook() -> GoPanicHookSuppression {
    __GO_PANIC_HOOK_INIT.call_once(|| {
        let previous = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            let suppressed = __GO_PANIC_HOOK_SUPPRESSION_DEPTH.with(|depth| depth.get() != 0);
            if !suppressed {
                previous(info);
            }
        }));
    });
    __GO_PANIC_HOOK_SUPPRESSION_DEPTH.with(|depth| depth.set(depth.get() + 1));
    GoPanicHookSuppression
}

impl Drop for GoPanicHookSuppression {
    fn drop(&mut self) {
        __GO_PANIC_HOOK_SUPPRESSION_DEPTH.with(|depth| depth.set(depth.get() - 1));
    }
}

struct GoPanicFrame {
    payload: Option<Box<dyn Any + Send + Sync>>,
    recover_depth: Option<usize>,
}

thread_local! {
    static __GO_RECOVER_PAYLOADS: RefCell<std::vec::Vec<GoPanicFrame>> = RefCell::new(std::vec::Vec::new());
}

fn go_recover() -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
    let payload = __GO_RECOVER_PAYLOADS.with(|stack| {
        stack.borrow_mut().last_mut().and_then(|frame| {
            if frame.recover_depth == Some(go_current_function_depth()) { frame.payload.take() } else { None }
        })
    });
    Arc::new(Mutex::new(payload))
}

fn go_push_panic_payload(payload: Box<dyn Any + Send + Sync>) {
    __GO_RECOVER_PAYLOADS.with(|stack| stack.borrow_mut().push(GoPanicFrame {
        payload: Some(payload),
        recover_depth: None,
    }));
}

fn go_call_deferred(f: Box<dyn FnOnce()>, recoverable: bool) -> Result<(), Box<dyn Any + Send>> {
    let previous = __GO_RECOVER_PAYLOADS.with(|stack| {
        stack.borrow_mut().last_mut().map(|frame| {
            let previous = frame.recover_depth;
            frame.recover_depth = if recoverable { Some(go_current_function_depth() + 1) } else { None };
            previous
        })
    });
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
    if let Some(previous) = previous {
        __GO_RECOVER_PAYLOADS.with(|stack| {
            if let Some(frame) = stack.borrow_mut().last_mut() {
                frame.recover_depth = previous;
            }
        });
    }
    result
}

fn go_store_panic_payload(payload: Box<dyn Any + Send>) {
    let payload = match payload.downcast::<Box<dyn Any + Send + Sync>>() {
        Ok(boxed) => {
            let mut payload = *boxed;
            loop {
                match payload.downcast::<Box<dyn Any + Send + Sync>>() {
                    Ok(boxed) => {
                        payload = *boxed;
                    }
                    Err(payload) => {
                        go_push_panic_payload(payload);
                        return;
                    }
                }
            }
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<String>() {
        Ok(value) => {
            go_push_panic_payload(Box::new(go_runtime_error_text(*value)) as Box<dyn Any + Send + Sync>);
            return;
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<&'static str>() {
        Ok(value) => {
            go_push_panic_payload(Box::new(go_runtime_error_text(value.to_string())) as Box<dyn Any + Send + Sync>);
            return;
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<i32>() {
        Ok(value) => {
            go_push_panic_payload(Box::new(*value) as Box<dyn Any + Send + Sync>);
            return;
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<i64>() {
        Ok(value) => {
            go_push_panic_payload(Box::new(*value) as Box<dyn Any + Send + Sync>);
            return;
        }
        Err(payload) => payload,
    };
    let _payload = match payload.downcast::<bool>() {
        Ok(value) => {
            go_push_panic_payload(Box::new(*value) as Box<dyn Any + Send + Sync>);
            return;
        }
        Err(_payload) => _payload,
    };
    panic!("recover: unsupported Rust panic payload; emit panic_any with a Go any payload instead")
}

fn go_replace_panic_payload(payload: Box<dyn Any + Send>) {
    __GO_RECOVER_PAYLOADS.with(|stack| { stack.borrow_mut().pop(); });
    go_store_panic_payload(payload);
}

fn go_runtime_error_text(__text: String) -> String {
    if __text == "attempt to divide by zero" || __text == "attempt to calculate the remainder with a divisor of zero" {
        return "runtime error: integer divide by zero".to_string();
    }
    if let Some(__rest) = __text.strip_prefix("index out of bounds: the len is ") {
        if let Some((__len, __index)) = __rest.split_once(" but the index is ") {
            return format!("runtime error: index out of range [{}] with length {}", __index, __len);
        }
    }
    __text
}

fn go_resume_unrecovered_panic() {
    if let Some(GoPanicFrame { payload: Some(payload), .. }) = __GO_RECOVER_PAYLOADS.with(|stack| stack.borrow_mut().pop()) {
        std::panic::panic_any(payload);
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

struct WaitGroup {
    count: std::sync::Arc<(std::sync::Mutex<i64>, std::sync::Condvar)>,
}

impl WaitGroup {
    fn new() -> Self {
        WaitGroup {
            count: std::sync::Arc::new((std::sync::Mutex::new(0), std::sync::Condvar::new())),
        }
    }

    fn add(&self, n: i64) {
        let (lock, _) = &*self.count;
        let mut count = lock.lock().unwrap();
        *count += n;
    }

    fn done(&self) {
        let (lock, cvar) = &*self.count;
        let mut count = lock.lock().unwrap();
        *count -= 1;
        if *count <= 0 {
            cvar.notify_all();
        }
    }

    fn wait(&self) {
        let (lock, cvar) = &*self.count;
        let mut count = lock.lock().unwrap();
        while *count > 0 {
            count = cvar.wait(count).unwrap();
        }
    }
}

impl Clone for WaitGroup {
    fn clone(&self) -> Self {
        WaitGroup {
            count: self.count.clone(),
        }
    }
}

impl Default for WaitGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for WaitGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WaitGroup")
    }
}

struct GoMutex {
    inner: std::sync::Arc<(std::sync::Mutex<bool>, std::sync::Condvar)>,
}

struct GoMutexGuard {
    mutex: GoMutex,
    active: bool,
}

impl GoMutex {
    fn new() -> Self {
        GoMutex {
            inner: std::sync::Arc::new((std::sync::Mutex::new(false), std::sync::Condvar::new())),
        }
    }

    fn lock(&self) {
        let (state, ready) = &*self.inner;
        let mut locked = state.lock().unwrap();
        while *locked {
            locked = ready.wait(locked).unwrap();
        }
        *locked = true;
    }

    fn unlock(&self) {
        let (state, ready) = &*self.inner;
        let mut locked = state.lock().unwrap();
        if !*locked {
            panic!("sync.Mutex: unlock of unlocked mutex");
        }
        *locked = false;
        ready.notify_one();
    }

    fn guard(&self) -> GoMutexGuard {
        self.lock();
        GoMutexGuard {
            mutex: self.clone(),
            active: true,
        }
    }
}

impl Drop for GoMutexGuard {
    fn drop(&mut self) {
        if self.active {
            self.mutex.unlock();
            self.active = false;
        }
    }
}

impl Default for GoMutex {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for GoMutex {
    fn clone(&self) -> Self {
        GoMutex {
            inner: self.inner.clone(),
        }
    }
}

impl std::fmt::Debug for GoMutex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Mutex")
    }
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

    pub fn __go_value_clone(&self) -> Self {
        self.clone()
    }

    fn add(&self, duration: Arc<Mutex<Option<std::time::Duration>>>) -> Arc<Mutex<Option<GoTime>>> {
        let duration = *duration.lock().unwrap().as_ref().unwrap();
        Arc::new(Mutex::new(Some(GoTime::from_unix(
            self.seconds + duration.as_secs() as i64,
            self.nanos as i64 + duration.subsec_nanos() as i64,
        ))))
    }

    fn u_t_c(&self) -> Arc<Mutex<Option<GoTime>>> {
        Arc::new(Mutex::new(Some(self.clone())))
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

    fn format(&self, _layout: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(self.to_string())))
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

fn go_channel_after(duration: std::time::Duration) -> GoChannel<GoTime> {
    let channel = GoChannel::<GoTime>::new_buffered(1);
    let thread_channel = channel.clone();
    std::thread::spawn(move || {
        std::thread::sleep(duration);
        thread_channel.send(GoTime::now());
        thread_channel.close();
    });
    channel
}

fn main() {
        // Worker pool pattern
    println!("{}", format!("{}", "=== Worker Pool Pattern ===".to_string()));

    let mut jobs = GoChannel::<i64>::new_buffered(100 as usize);
    let mut results = GoChannel::<i64>::new_buffered(100 as usize);

        // Start workers
    let mut wg = WaitGroup::new();
    let mut numWorkers = Arc::new(Mutex::new(Some(3i64)));

    let mut w = Arc::new(Mutex::new(Some(1i64)));
    while { let __tmp_x = { let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*numWorkers.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } {
        wg.add(1);
        let jobs_thread = jobs.clone(); let results_thread = results.clone(); let wg_thread = wg.clone(); let __go_arg_0 = Arc::new(Mutex::new(Some({ let __arg_holder = w.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); std::thread::spawn(move || {
        let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();
        let mut __closure = move |id: Arc<Mutex<Option<i64>>>| {
            let __go_panic_hook_guard = go_suppress_panic_hook();
            let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let wg_defer_captured = wg_thread.clone(); __defer_stack.push(Box::new(move || {
        wg_defer_captured.done();
    }));;
                for job in jobs_thread.clone() {
        print!("Worker {} processing job {}\n", format!("{}", { let __v = (*id.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", job));
        std::thread::sleep(std::time::Duration::from_millis(10));
        results_thread.send({ let __tmp_x = job; let __tmp_y = 2; __tmp_x * __tmp_y });
    };
                while let Some(f) = __defer_stack.pop() {
                    if let Err(__go_defer_panic) = go_call_deferred(f, false) {
                        std::panic::resume_unwind(__go_defer_panic);
                    }
                }
            }));
            match __go_panic_result {
                Ok(__go_value) => {
                    drop(__go_panic_hook_guard);
                    __go_value
                }
                Err(__go_panic_payload) => {
                    go_store_panic_payload(__go_panic_payload);
                    while let Some(f) = __defer_stack.pop() {
                        if let Err(__go_defer_panic) = go_call_deferred(f, true) {
                            go_replace_panic_payload(__go_defer_panic);
                        }
                    }
                    drop(__go_panic_hook_guard);
                    go_resume_unrecovered_panic();
                    ()
                }
            }

        };
        __closure(__go_arg_0);
    });
        { let mut guard = w.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

        // Send jobs
    let mut numJobs = Arc::new(Mutex::new(Some(9i64)));
    let mut j = Arc::new(Mutex::new(Some(1i64)));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*numJobs.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } {
        jobs.send((*j.lock().unwrap().as_ref().unwrap()));
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    jobs.close();

        // Wait for workers to finish
    let results_thread = results.clone(); let wg_thread = wg.clone(); std::thread::spawn(move || {
        wg_thread.wait();;
        results_thread.close();;;
    });

        // Collect results
    for result in results.clone() {
        print!("Result: {}\n", result);
    }

        // Producer-Consumer pattern
    println!("{}", format!("{}", "\n=== Producer-Consumer Pattern ===".to_string()));

    let mut buffer = GoChannel::<String>::new_buffered(5 as usize);
    let mut done = GoChannel::<bool>::new();

        // Producer
    let buffer_thread = buffer.clone(); std::thread::spawn(move || {
        let mut items = Arc::new(Mutex::new(Some(vec!["apple".to_string(), "banana".to_string(), "cherry".to_string(), "date".to_string(), "elderberry".to_string()])));;
        { let __range_holder = items.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for item in __range_values.iter() {
        print!("Producing: {}\n", item);
        buffer_thread.send(item);
        std::thread::sleep(std::time::Duration::from_millis(50));
    } };
        buffer_thread.close();;;
    });

        // Consumer
    let buffer_thread = buffer.clone(); let done_thread = done.clone(); std::thread::spawn(move || {
        for item in buffer_thread.clone() {
        print!("Consuming: {}\n", item);
        std::thread::sleep(std::time::Duration::from_millis(100));
    };
        done_thread.send(true);;;
    });

    done.recv().unwrap_or_default();

        // Fan-out/Fan-in pattern
    println!("{}", format!("{}", "\n=== Fan-out/Fan-in Pattern ===".to_string()));

    let mut input = GoChannel::<i64>::new();

        // Fan-out: distribute work to multiple goroutines
    let mut c1 = fan_out(input.clone());
    let mut c2 = fan_out(input.clone());
    let mut c3 = fan_out(input.clone());

        // Fan-in: combine results from multiple goroutines
    let mut output = fan_in(Arc::new(Mutex::new(Some(vec![c1, c2, c3]))));

        // Send input
    let input_thread = input.clone(); std::thread::spawn(move || {
        let mut i = Arc::new(Mutex::new(Some(1i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 6; __tmp_x <= __tmp_y } {
        input_thread.send((*i.lock().unwrap().as_ref().unwrap()));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    };
        input_thread.close();;;
    });

        // Collect output
    for result in output.clone() {
        print!("Fan-in result: {}\n", result);
    }

        // Pipeline pattern
    println!("{}", format!("{}", "\n=== Pipeline Pattern ===".to_string()));

        // Stage 1: Generate numbers
    let mut numbers = GoChannel::<i64>::new();
    let numbers_thread = numbers.clone(); std::thread::spawn(move || {
        let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();
        let __go_panic_hook_guard = go_suppress_panic_hook();
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let numbers_defer_captured = numbers_thread.clone(); __defer_stack.push(Box::new(move || {
        numbers_defer_captured.close();
    }));;
            let mut i = Arc::new(Mutex::new(Some(1i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 5; __tmp_x <= __tmp_y } {
        numbers_thread.send((*i.lock().unwrap().as_ref().unwrap()));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    };
            while let Some(f) = __defer_stack.pop() {
                if let Err(__go_defer_panic) = go_call_deferred(f, false) {
                    std::panic::resume_unwind(__go_defer_panic);
                }
            }
        }));
        match __go_panic_result {
            Ok(__go_value) => {
                drop(__go_panic_hook_guard);
                __go_value
            }
            Err(__go_panic_payload) => {
                go_store_panic_payload(__go_panic_payload);
                while let Some(f) = __defer_stack.pop() {
                    if let Err(__go_defer_panic) = go_call_deferred(f, true) {
                        go_replace_panic_payload(__go_defer_panic);
                    }
                }
                drop(__go_panic_hook_guard);
                go_resume_unrecovered_panic();
                ()
            }
        }
;
    });

        // Stage 2: Square numbers
    let mut squares = GoChannel::<i64>::new();
    let numbers_thread = numbers.clone(); let squares_thread = squares.clone(); std::thread::spawn(move || {
        let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();
        let __go_panic_hook_guard = go_suppress_panic_hook();
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let squares_defer_captured = squares_thread.clone(); __defer_stack.push(Box::new(move || {
        squares_defer_captured.close();
    }));;
            for n in numbers_thread.clone() {
        squares_thread.send({ let __tmp_x = n; let __tmp_y = n; __tmp_x * __tmp_y });
    };
            while let Some(f) = __defer_stack.pop() {
                if let Err(__go_defer_panic) = go_call_deferred(f, false) {
                    std::panic::resume_unwind(__go_defer_panic);
                }
            }
        }));
        match __go_panic_result {
            Ok(__go_value) => {
                drop(__go_panic_hook_guard);
                __go_value
            }
            Err(__go_panic_payload) => {
                go_store_panic_payload(__go_panic_payload);
                while let Some(f) = __defer_stack.pop() {
                    if let Err(__go_defer_panic) = go_call_deferred(f, true) {
                        go_replace_panic_payload(__go_defer_panic);
                    }
                }
                drop(__go_panic_hook_guard);
                go_resume_unrecovered_panic();
                ()
            }
        }
;
    });

        // Stage 3: Add 10 to each
    let mut r#final = GoChannel::<i64>::new();
    let final_thread = final.clone(); let squares_thread = squares.clone(); std::thread::spawn(move || {
        let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();
        let __go_panic_hook_guard = go_suppress_panic_hook();
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let final_defer_captured = final_thread.clone(); __defer_stack.push(Box::new(move || {
        final_defer_captured.close();
    }));;
            for s in squares_thread.clone() {
        final_thread.send({ let __tmp_x = s; let __tmp_y = 10; __tmp_x + __tmp_y });
    };
            while let Some(f) = __defer_stack.pop() {
                if let Err(__go_defer_panic) = go_call_deferred(f, false) {
                    std::panic::resume_unwind(__go_defer_panic);
                }
            }
        }));
        match __go_panic_result {
            Ok(__go_value) => {
                drop(__go_panic_hook_guard);
                __go_value
            }
            Err(__go_panic_payload) => {
                go_store_panic_payload(__go_panic_payload);
                while let Some(f) = __defer_stack.pop() {
                    if let Err(__go_defer_panic) = go_call_deferred(f, true) {
                        go_replace_panic_payload(__go_defer_panic);
                    }
                }
                drop(__go_panic_hook_guard);
                go_resume_unrecovered_panic();
                ()
            }
        }
;
    });

        // Consume final results
    for result in r#final.clone() {
        print!("Pipeline result: {}\n", result);
    }

        // Mutex and shared state
    println!("{}", format!("{}", "\n=== Mutex and Shared State ===".to_string()));

    let mut counter: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut mutex = GoMutex::new();
    let mut wg2 = WaitGroup::new();

    let mut counter_closure_clone = counter.clone(); let mutex_closure_clone = mutex.clone(); let wg2_closure_clone = wg2.clone(); let mut increment = Arc::new(Mutex::new(Some(Box::new(move |id: Arc<Mutex<Option<i64>>>| {
        let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();
        let __go_panic_hook_guard = go_suppress_panic_hook();
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let wg2_defer_captured = wg2_closure_clone.clone(); __defer_stack.push(Box::new(move || {
        wg2_defer_captured.done();
    }));
        let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1000; __tmp_x < __tmp_y } {
        mutex_closure_clone.lock();
        { let mut guard = counter_closure_clone.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        mutex_closure_clone.unlock();
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        print!("Goroutine {} finished\n", { let __v = (*id.lock().unwrap().as_ref().unwrap()).clone(); __v });
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        }));
        match __go_panic_result {
            Ok(__go_value) => {
                drop(__go_panic_hook_guard);
                __go_value
            }
            Err(__go_panic_payload) => {
                go_store_panic_payload(__go_panic_payload);
                while let Some(f) = __defer_stack.pop() {
                    if let Err(__go_defer_panic) = go_call_deferred(f, true) {
                        go_replace_panic_payload(__go_defer_panic);
                    }
                }
                drop(__go_panic_hook_guard);
                go_resume_unrecovered_panic();
                ()
            }
        }
    }) as Box<dyn FnMut(Arc<Mutex<Option<i64>>>) -> () + Send + Sync>)));

        // Start multiple goroutines
    let mut i = Arc::new(Mutex::new(Some(1i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x <= __tmp_y } {
        wg2.add(1);
        let i_thread = Arc::new(Mutex::new(Some((*i.lock().unwrap().as_ref().unwrap()).clone()))); let increment_thread = increment.clone(); std::thread::spawn(move || {
        {
            let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i64>>>) -> () + Send + Sync> = {
                let mut __f_guard = increment_thread.lock().unwrap();
                __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i64>>>) -> () + Send + Sync>
            };
            let __f = unsafe { &mut *__f_ptr };
            (*__f)(
                i_thread.clone()
            )
        };
    });
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    wg2.wait();
    print!("Final counter value: {}\n", { let __v = (*counter.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Channel-based synchronization
    println!("{}", format!("{}", "\n=== Channel-based Synchronization ===".to_string()));

    let mut wg3 = WaitGroup::new();
    let mut barrier = GoChannel::<bool>::new_buffered(3 as usize);

    let barrier_closure_clone = barrier.clone(); let wg3_closure_clone = wg3.clone(); let mut worker = Arc::new(Mutex::new(Some(Box::new(move |id: Arc<Mutex<Option<i64>>>| {
        let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();
        let __go_panic_hook_guard = go_suppress_panic_hook();
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let wg3_defer_captured = wg3_closure_clone.clone(); __defer_stack.push(Box::new(move || {
        wg3_defer_captured.done();
    }));
        print!("Worker {}: Phase 1 complete\n", { let __v = (*id.lock().unwrap().as_ref().unwrap()).clone(); __v });
        barrier_closure_clone.send(true);
        barrier_closure_clone.recv().unwrap_or_default();
        barrier_closure_clone.recv().unwrap_or_default();
        barrier_closure_clone.recv().unwrap_or_default();
        print!("Worker {}: Phase 2 complete\n", { let __v = (*id.lock().unwrap().as_ref().unwrap()).clone(); __v });
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        }));
        match __go_panic_result {
            Ok(__go_value) => {
                drop(__go_panic_hook_guard);
                __go_value
            }
            Err(__go_panic_payload) => {
                go_store_panic_payload(__go_panic_payload);
                while let Some(f) = __defer_stack.pop() {
                    if let Err(__go_defer_panic) = go_call_deferred(f, true) {
                        go_replace_panic_payload(__go_defer_panic);
                    }
                }
                drop(__go_panic_hook_guard);
                go_resume_unrecovered_panic();
                ()
            }
        }
    }) as Box<dyn FnMut(Arc<Mutex<Option<i64>>>) -> () + Send + Sync>)));

        // Do some work
        // Signal completion of phase 1
        // Wait for all workers to complete phase 1
        // Do phase 2 work
    let mut i = Arc::new(Mutex::new(Some(1i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x <= __tmp_y } {
        wg3.add(1);
        let i_thread = Arc::new(Mutex::new(Some((*i.lock().unwrap().as_ref().unwrap()).clone()))); let worker_thread = worker.clone(); std::thread::spawn(move || {
        {
            let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i64>>>) -> () + Send + Sync> = {
                let mut __f_guard = worker_thread.lock().unwrap();
                __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i64>>>) -> () + Send + Sync>
            };
            let __f = unsafe { &mut *__f_ptr };
            (*__f)(
                i_thread.clone()
            )
        };
    });
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    wg3.wait();

        // Timeout pattern
    println!("{}", format!("{}", "\n=== Timeout Pattern ===".to_string()));

    let mut slowOperation = Arc::new(Mutex::new(Some(Box::new(move || -> GoChannel<String> {
        let mut result = GoChannel::<String>::new();
        let result_thread = result.clone(); std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(200));;
        result_thread.send("Operation completed".to_string());;;
    });
        result.clone()
    }) as Box<dyn FnMut() -> GoChannel<String> + Send + Sync>)));

    loop {
        if let Some(__select_recv) = {
            let __f_ptr: *mut Box<dyn FnMut() -> GoChannel<String> + Send + Sync> = {
                let mut __f_guard = slowOperation.lock().unwrap();
                __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> GoChannel<String> + Send + Sync>
            };
            let __f = unsafe { &mut *__f_ptr };
            (*__f)(

            )
        }.try_recv_select() {
            let __select_ok = __select_recv.is_some();
            let result = match __select_recv { Some(__v) => __v, None => Default::default() };
            let mut result = Arc::new(Mutex::new(Some(result)));
            print!("Success: {}\n", { let __v = (*result.lock().unwrap().as_ref().unwrap()).clone(); __v });
            break;
        }
        if let Some(_) = go_channel_after(std::time::Duration::from_millis(100)).try_recv_select() {
            println!("{}", format!("{}", "Timeout: Operation took too long".to_string()));
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }

        // Try again with longer timeout
    loop {
        if let Some(__select_recv) = {
            let __f_ptr: *mut Box<dyn FnMut() -> GoChannel<String> + Send + Sync> = {
                let mut __f_guard = slowOperation.lock().unwrap();
                __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> GoChannel<String> + Send + Sync>
            };
            let __f = unsafe { &mut *__f_ptr };
            (*__f)(

            )
        }.try_recv_select() {
            let __select_ok = __select_recv.is_some();
            let result = match __select_recv { Some(__v) => __v, None => Default::default() };
            let mut result = Arc::new(Mutex::new(Some(result)));
            print!("Success: {}\n", { let __v = (*result.lock().unwrap().as_ref().unwrap()).clone(); __v });
            break;
        }
        if let Some(_) = go_channel_after(std::time::Duration::from_millis(300)).try_recv_select() {
            println!("{}", format!("{}", "Timeout: Operation took too long".to_string()));
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}

/// Helper functions for fan-out/fan-in
pub fn fan_out(input: GoChannel<i64>) -> GoChannel<i64> {
    let mut output = GoChannel::<i64>::new();
    let input_thread = input.clone(); let output_thread = output.clone(); std::thread::spawn(move || {
        let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();
        let __go_panic_hook_guard = go_suppress_panic_hook();
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let output_defer_captured = output_thread.clone(); __defer_stack.push(Box::new(move || {
        output_defer_captured.close();
    }));;
            for n in input_thread.clone() {
        output_thread.send({ let __tmp_x = n; let __tmp_y = n; __tmp_x * __tmp_y });
    };
            while let Some(f) = __defer_stack.pop() {
                if let Err(__go_defer_panic) = go_call_deferred(f, false) {
                    std::panic::resume_unwind(__go_defer_panic);
                }
            }
        }));
        match __go_panic_result {
            Ok(__go_value) => {
                drop(__go_panic_hook_guard);
                __go_value
            }
            Err(__go_panic_payload) => {
                go_store_panic_payload(__go_panic_payload);
                while let Some(f) = __defer_stack.pop() {
                    if let Err(__go_defer_panic) = go_call_deferred(f, true) {
                        go_replace_panic_payload(__go_defer_panic);
                    }
                }
                drop(__go_panic_hook_guard);
                go_resume_unrecovered_panic();
                ()
            }
        }
;
    });
        // Square the number
    output.clone()
}

pub fn fan_in(inputs: Arc<Mutex<Option<Vec<GoChannel<i64>>>>>) -> GoChannel<i64> {
    let mut output = GoChannel::<i64>::new();
    let mut wg = WaitGroup::new();

    { let __range_holder = inputs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for input in __range_values.iter() {
        wg.add(1);
        let output_thread = output.clone(); let wg_thread = wg.clone(); let __go_arg_0 = Arc::new(Mutex::new(Some((*input).clone()))); std::thread::spawn(move || {
        let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();
        let mut __closure = move |ch: GoChannel<i64>| {
            let __go_panic_hook_guard = go_suppress_panic_hook();
            let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let wg_defer_captured = wg_thread.clone(); __defer_stack.push(Box::new(move || {
        wg_defer_captured.done();
    }));;
                for n in ch.clone() {
        output_thread.send(n);
    };
                while let Some(f) = __defer_stack.pop() {
                    if let Err(__go_defer_panic) = go_call_deferred(f, false) {
                        std::panic::resume_unwind(__go_defer_panic);
                    }
                }
            }));
            match __go_panic_result {
                Ok(__go_value) => {
                    drop(__go_panic_hook_guard);
                    __go_value
                }
                Err(__go_panic_payload) => {
                    go_store_panic_payload(__go_panic_payload);
                    while let Some(f) = __defer_stack.pop() {
                        if let Err(__go_defer_panic) = go_call_deferred(f, true) {
                            go_replace_panic_payload(__go_defer_panic);
                        }
                    }
                    drop(__go_panic_hook_guard);
                    go_resume_unrecovered_panic();
                    ()
                }
            }

        };
        __closure(__go_arg_0);
    });
    } }

    let output_thread = output.clone(); let wg_thread = wg.clone(); std::thread::spawn(move || {
        wg_thread.wait();;
        output_thread.close();;;
    });

    output.clone()
}