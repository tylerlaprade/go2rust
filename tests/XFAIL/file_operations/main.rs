#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use std::any::Any;
use std::cell::{RefCell};
use std::error::Error as StdError;
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


thread_local! {
    static __GO_RECOVER_PAYLOAD: RefCell<Option<Box<dyn Any + Send + Sync>>> = RefCell::new(None);
}

fn go_recover() -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
    __GO_RECOVER_PAYLOAD.with(|slot| Arc::new(Mutex::new(slot.borrow_mut().take())))
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
                        __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(payload));
                        return;
                    }
                }
            }
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<String>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(go_runtime_error_text(*value)) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<&'static str>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(go_runtime_error_text(value.to_string())) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<i32>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(*value) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<i64>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(*value) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(payload) => payload,
    };
    let _payload = match payload.downcast::<bool>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(*value) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(_payload) => _payload,
    };
    panic!("recover: unsupported Rust panic payload; emit panic_any with a Go any payload instead")
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

thread_local! {
    static __GO_PANIC_HOOK_SILENCE: std::cell::Cell<usize> = std::cell::Cell::new(0);
}

fn go_silence_panic_hook() -> usize {
    static __GO_PANIC_HOOK: std::sync::Once = std::sync::Once::new();
    __GO_PANIC_HOOK.call_once(|| {
        let __previous = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |__info| {
            if __GO_PANIC_HOOK_SILENCE.with(|__depth| __depth.get()) == 0 {
                __previous(__info);
            }
        }));
    });
    __GO_PANIC_HOOK_SILENCE.with(|__depth| {
        let __previous = __depth.get();
        __depth.set(__previous + 1);
        __previous
    })
}

fn go_restore_panic_hook(__previous: usize) {
    __GO_PANIC_HOOK_SILENCE.with(|__depth| __depth.set(__previous));
}

fn go_resume_unrecovered_panic() {
    if let Some(payload) = __GO_RECOVER_PAYLOAD.with(|slot| slot.borrow_mut().take()) {
        std::panic::panic_any(payload);
    }
}

struct GoFile {
    file: Option<std::fs::File>,
}

impl GoFile {
    fn create(path: &str) -> Result<Self, std::io::Error> {
        std::fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path)
            .map(|file| GoFile { file: Some(file) })
    }

    fn empty() -> Self {
        GoFile { file: None }
    }

    fn write_string(&mut self, text: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<std::sync::Arc<dyn std::error::Error + Send + Sync>>>>) {
        let text = (*text.lock().unwrap().as_ref().unwrap()).clone();
        match self.file.as_mut() {
            Some(file) => match std::io::Write::write_all(file, text.as_bytes()) {
                Ok(()) => (Arc::new(Mutex::new(Some(text.len() as i64))), Arc::new(Mutex::new(None))),
                Err(e) => (Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(std::sync::Arc::<dyn std::error::Error + Send + Sync>::from(Box::<dyn std::error::Error + Send + Sync>::from(e)))))),
            },
            None => (
                Arc::new(Mutex::new(Some(0))),
                Arc::new(Mutex::new(Some(std::sync::Arc::<dyn std::error::Error + Send + Sync>::from(Box::<dyn std::error::Error + Send + Sync>::from(std::io::Error::new(std::io::ErrorKind::Other, "invalid file")))))),
            ),
        }
    }

    fn close(&mut self) -> Arc<Mutex<Option<std::sync::Arc<dyn std::error::Error + Send + Sync>>>> {
        self.file = None;
        Arc::new(Mutex::new(None))
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

fn __go_next_external_interface_id() -> usize {
    static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}



#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct bufio_Scanner;

impl std::fmt::Display for bufio_Scanner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<bufio_Scanner>")
    }
}


impl bufio_Scanner {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn err(&self) -> Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> {
        panic!("bufio_Scanner.err bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
    pub fn scan(&self) -> bool {
        panic!("bufio_Scanner.scan bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
    pub fn text(&self) -> Arc<Mutex<Option<String>>> {
        panic!("bufio_Scanner.text bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct fs_FileInfo {
    pub name: String,
    pub is_dir: bool,
    pub size: i64,
}

impl std::fmt::Display for fs_FileInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<fs_FileInfo>")
    }
}


impl fs_FileInfo {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some::<String>(self.name.clone())))
    }
    pub fn size(&self) -> i64 {
        self.size
    }
    pub fn is_dir(&self) -> bool {
        self.is_dir
    }
    pub fn mode(&self) -> Arc<Mutex<Option<fs_FileMode>>> {
        if self.is_dir {
            Arc::new(Mutex::new(Some::<fs_FileMode>(fs_FileMode(1u32 << 31))))
        } else {
            Arc::new(Mutex::new(Some::<fs_FileMode>(fs_FileMode(0))))
        }
    }
    pub fn mod_time(&self) -> Arc<Mutex<Option<GoTime>>> {
        panic!("fs_FileInfo.mod_time bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct fs_FileMode(pub u32);

impl PartialEq<u32> for fs_FileMode {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<fs_FileMode> for u32 {
    fn eq(&self, other: &fs_FileMode) -> bool {
        *self == other.0
    }
}

impl std::ops::BitAnd for fs_FileMode {
    type Output = fs_FileMode;
    fn bitand(self, other: Self) -> fs_FileMode {
        fs_FileMode(self.0 & other.0)
    }
}

impl std::ops::BitOr for fs_FileMode {
    type Output = fs_FileMode;
    fn bitor(self, other: Self) -> fs_FileMode {
        fs_FileMode(self.0 | other.0)
    }
}

impl std::fmt::Display for fs_FileMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<fs_FileMode>")
    }
}


impl fs_FileMode {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone)]
pub struct os_File {
    pub __go_data: std::sync::Arc<std::sync::Mutex<Vec<u8>>>,
    pub __go_closed: std::sync::Arc<std::sync::atomic::AtomicBool>,
    pub __go_wait_for_close: bool,
}

impl Default for os_File {
    fn default() -> Self {
        Self {
            __go_data: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
            __go_closed: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            __go_wait_for_close: false,
        }
    }
}

impl std::fmt::Display for os_File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<os_File>")
    }
}

impl PartialEq for os_File {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__go_data, &other.__go_data)
    }
}

impl Eq for os_File {}

impl os_File {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }

    pub fn __go_write_bytes(&self, data: &[u8]) {
        self.__go_data.lock().unwrap().extend_from_slice(data);
    }

    pub fn __go_read_all(&self) -> Vec<u8> {
        self.__go_data.lock().unwrap().clone()
    }

    pub fn close(&self) -> Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> {
        self.__go_closed.store(true, std::sync::atomic::Ordering::SeqCst);
        Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>))
    }

    pub fn write<T0: 'static>(&self, arg0: T0) -> (Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        let bytes = if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Vec<u8>>() {
            v.clone()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Arc<Mutex<Option<Vec<u8>>>>>() {
            v.lock().unwrap().as_ref().cloned().unwrap_or_default()
        } else {
            Vec::new()
        };
        let n = bytes.len() as i64;
        self.__go_write_bytes(&bytes);
        (Arc::new(Mutex::new(Some::<i64>(n))), Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>)))
    }

    pub fn write_string<T0: 'static>(&self, arg0: T0) -> (Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        let value = if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<String>() {
            v.clone()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<&str>() {
            (*v).to_string()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Arc<Mutex<Option<String>>>>() {
            v.lock().unwrap().as_ref().cloned().unwrap_or_default()
        } else {
            String::new()
        };
        let bytes = Vec::from(value.into_bytes());
        let n = bytes.len() as i64;
        self.__go_write_bytes(&bytes);
        (Arc::new(Mutex::new(Some::<i64>(n))), Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>)))
    }

    pub fn read<T0>(&self, _arg0: T0) -> (Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        (Arc::new(Mutex::new(Some::<i64>(0))), Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>)))
    }

    pub fn read_at<T0: 'static, T1: 'static>(&self, arg0: T0, arg1: T1) -> (Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        let offset = if let Some(v) = (&arg1 as &dyn std::any::Any).downcast_ref::<i64>() {
            *v
        } else if let Some(v) = (&arg1 as &dyn std::any::Any).downcast_ref::<Arc<Mutex<Option<i64>>>>() {
            v.lock().unwrap().as_ref().copied().unwrap_or_default()
        } else {
            0
        };
        let data = self.__go_read_all();
        let mut n = 0i64;
        if offset >= 0 {
            let start = offset as usize;
            if start < data.len() {
                if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Arc<Mutex<Option<Vec<u8>>>>>() {
                    let mut guard = v.lock().unwrap();
                    if let Some(target) = guard.as_mut() {
                        let count = std::cmp::min(target.len(), data.len() - start);
                        target[..count].copy_from_slice(&data[start..start + count]);
                        n = count as i64;
                    }
                }
            }
        }
        (Arc::new(Mutex::new(Some::<i64>(n))), Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>)))
    }
}


pub mod bufio {
    use super::*;
    pub fn new_scanner<T0>(_arg0: T0) -> Arc<Mutex<Option<bufio_Scanner>>> {
        panic!("new_scanner bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


pub mod io {
    use super::*;
    pub fn copy<T0, T1>(_arg0: T0, _arg1: T1) -> (i64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        panic!("copy bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


pub mod os {
    use super::*;
    use std::path::Path;

    pub trait GoStringArg {
        fn into_go_string(self) -> String;
    }

    impl GoStringArg for String {
        fn into_go_string(self) -> String {
            self
        }
    }

    impl<'a> GoStringArg for &'a str {
        fn into_go_string(self) -> String {
            self.to_string()
        }
    }

    impl<'a> GoStringArg for &'a String {
        fn into_go_string(self) -> String {
            self.clone()
        }
    }

    impl GoStringArg for Arc<Mutex<Option<String>>> {
        fn into_go_string(self) -> String {
            self.lock().unwrap().as_ref().cloned().unwrap_or_default()
        }
    }

    type GoError = Arc<Mutex<Option<std::sync::Arc<dyn std::error::Error + Send + Sync>>>>;

    fn no_error() -> GoError {
        Arc::new(Mutex::new(None))
    }

    fn io_error(err: std::io::Error) -> GoError {
        Arc::new(Mutex::new(Some(std::sync::Arc::new(err) as std::sync::Arc<dyn std::error::Error + Send + Sync>)))
    }

    pub fn is_not_exist<T0>(_arg0: T0) -> bool {
        panic!("is_not_exist bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn open<T0: GoStringArg>(_arg0: T0) -> (Arc<Mutex<Option<os_File>>>, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        let path = _arg0.into_go_string();
        match std::fs::read(&path) {
            Ok(data) => {
                let file = os_File { __go_data: std::sync::Arc::new(std::sync::Mutex::new(Vec::from(data))), __go_closed: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), __go_wait_for_close: false };
                (Arc::new(Mutex::new(Some::<os_File>(file))), no_error())
            }
            Err(err) => (Arc::new(Mutex::new(None::<os_File>)), io_error(err)),
        }
    }

    pub fn open_file<T0, T1, T2>(_arg0: T0, _arg1: T1, _arg2: T2) -> (Arc<Mutex<Option<os_File>>>, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        panic!("open_file bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn read_file<T0: GoStringArg>(_arg0: T0) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        let path = _arg0.into_go_string();
        match std::fs::read(&path) {
            Ok(data) => (Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(data)))), no_error()),
            Err(err) => (Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(Vec::new())))), io_error(err)),
        }
    }

    pub fn stat<T0: GoStringArg>(_arg0: T0) -> (Arc<Mutex<Option<fs_FileInfo>>>, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        let path = _arg0.into_go_string();
        match std::fs::metadata(&path) {
            Ok(metadata) => {
                let name = Path::new(&path).file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_else(|| path.clone());
                (Arc::new(Mutex::new(Some::<fs_FileInfo>(fs_FileInfo { name, is_dir: metadata.is_dir(), size: metadata.len() as i64 }))), no_error())
            }
            Err(err) => (Arc::new(Mutex::new(Some::<fs_FileInfo>(fs_FileInfo::default()))), io_error(err)),
        }
    }
}


fn main() {
    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    let __go_previous_panic_hook = go_silence_panic_hook();
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // Create a temporary file for testing
        println!("{}", format!("{}", "=== File Operations Test ===".to_string()));

        let mut filename = Arc::new(Mutex::new(Some("test_file.txt".to_string())));

                // Write to file
        println!("{}", format!("{}", "\n--- Writing to file ---".to_string()));

        let (mut file, mut err) = { let __path = (*filename.lock().unwrap().as_ref().unwrap()).clone(); match GoFile::create(&__path) { Ok(file) => (Arc::new(Mutex::new(Some(file))), Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>))), Err(e) => (Arc::new(Mutex::new(Some(GoFile::empty()))), Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(e)))))) } };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        print!("Error creating file: {}\n", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }

        let mut content = Arc::new(Mutex::new(Some(vec!["Hello, World!".to_string(), "This is line 2".to_string(), "Go file operations".to_string(), "Line 4 with numbers: 123".to_string(), "Final line".to_string()])));

        { let __range_holder = content.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, line) in __range_values.iter().enumerate() {
        let (_, mut err) = { let __recv = file.clone(); let __recv_ptr: *mut os_File = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut os_File }; let __result = unsafe { &mut *__recv_ptr }.write_string(Arc::new(Mutex::new(Some(format!("{}{}", line, "\n".to_string()))))); __result };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        print!("Error writing line {}: {}\n", format!("{}", { let __tmp_x = i as i64; let __tmp_y = 1; __tmp_x + __tmp_y }), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
        { let __recv = file.clone(); let __recv_ptr: *mut os_File = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut os_File }; let __result = unsafe { &mut *__recv_ptr }.close(); __result };
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
        print!("Wrote: {}\n", line);
    } }

        { let __recv = file.clone(); let __recv_ptr: *mut os_File = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut os_File }; let __result = unsafe { &mut *__recv_ptr }.close(); __result };
        print!("File '{}' created successfully\n", { let __v = (*filename.lock().unwrap().as_ref().unwrap()).clone(); __v });

                // Read entire file
        println!("{}", format!("{}", "\n--- Reading entire file ---".to_string()));

        let (mut data, __tmp_1) = os::read_file({ let __arg_holder = filename.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1;;
        let mut data = { let __slice_header = GoSliceHeader::go_slice_header(&data); Arc::new(Mutex::new(__slice_header)) };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        print!("Error reading file: {}\n", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }

        print!("File contents:\n{}", (*Arc::new(Mutex::new(Some(String::from_utf8(std::vec::Vec::from((*data.lock().unwrap().as_ref().unwrap()).clone())).unwrap()))).lock().unwrap().as_ref().unwrap()));

                // Read file line by line
        println!("{}", format!("{}", "\n--- Reading file line by line ---".to_string()));

        { let (__tmp_0, __tmp_1) = os::open({ let __arg_holder = filename.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }); file = __tmp_0.clone(); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        print!("Error opening file: {}\n", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
        let file_defer_captured = file.clone(); __defer_stack.push(Box::new(move || {
        { let __recv = file_defer_captured.clone(); let __recv_ptr: *mut os_File = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut os_File }; let __result = unsafe { &mut *__recv_ptr }.close(); __result };
    }));

        let mut scanner = bufio::new_scanner(file.clone());
        let mut lineNum = Arc::new(Mutex::new(Some(1i64)));

        while { let __recv = scanner.clone(); let __recv_ptr: *mut bufio_Scanner = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bufio_Scanner }; let __result = unsafe { &mut *__recv_ptr }.scan(); __result } {
        let mut line = { let __recv = scanner.clone(); let __recv_ptr: *mut bufio_Scanner = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bufio_Scanner }; let __result = unsafe { &mut *__recv_ptr }.text(); __result };
        print!("Line {}: {}\n", format!("{}", { let __v = (*lineNum.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*line.lock().unwrap().as_ref().unwrap()).clone(); __v }));
        { let mut guard = lineNum.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

        {
        let mut err = { let __recv = scanner.clone(); let __recv_ptr: *mut bufio_Scanner = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bufio_Scanner }; let __result = unsafe { &mut *__recv_ptr }.err(); __result };;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            print!("Error reading file: {}\n", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });;
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    };
        }
    }

                // Append to file
        println!("{}", format!("{}", "\n--- Appending to file ---".to_string()));

        { let (__tmp_0, __tmp_1) = os::open_file({ let __arg_holder = filename.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, 1025, fs_FileMode(0o644 as u32)); file = __tmp_0.clone(); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        print!("Error opening file for append: {}\n", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }

        let mut appendContent = Arc::new(Mutex::new(Some(vec!["Appended line 1".to_string(), "Appended line 2".to_string()])));

        { let __range_holder = appendContent.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for line in __range_values.iter() {
        let (_, mut err) = { let __recv = file.clone(); let __recv_ptr: *mut os_File = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut os_File }; let __result = unsafe { &mut *__recv_ptr }.write_string(Arc::new(Mutex::new(Some(format!("{}{}", line, "\n".to_string()))))); __result };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        print!("Error appending: {}\n", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
        { let __recv = file.clone(); let __recv_ptr: *mut os_File = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut os_File }; let __result = unsafe { &mut *__recv_ptr }.close(); __result };
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
        print!("Appended: {}\n", line);
    } }

        { let __recv = file.clone(); let __recv_ptr: *mut os_File = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut os_File }; let __result = unsafe { &mut *__recv_ptr }.close(); __result };

                // Read updated file
        println!("{}", format!("{}", "\n--- Reading updated file ---".to_string()));

        { let (__tmp_0, __tmp_1) = os::read_file({ let __arg_holder = filename.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *data.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        print!("Error reading updated file: {}\n", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }

        print!("Updated file contents:\n{}", (*Arc::new(Mutex::new(Some(String::from_utf8(std::vec::Vec::from((*data.lock().unwrap().as_ref().unwrap()).clone())).unwrap()))).lock().unwrap().as_ref().unwrap()));

                // File information
        println!("{}", format!("{}", "\n--- File information ---".to_string()));

        let (mut fileInfo, __tmp_1) = os::stat({ let __arg_holder = filename.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1;;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        print!("Error getting file info: {}\n", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }

        print!("File name: {}\n", (*(*fileInfo.lock().unwrap().as_ref().unwrap()).name().lock().unwrap().as_ref().unwrap()));
        print!("File size: {} bytes\n", (*fileInfo.lock().unwrap().as_ref().unwrap()).size());
        print!("File mode: {}\n", (*(*fileInfo.lock().unwrap().as_ref().unwrap()).mode().lock().unwrap().as_ref().unwrap()));

                // Keep the fixture deterministic; actual modification times vary per run.
        let _ = (*fileInfo.lock().unwrap().as_ref().unwrap()).mod_time();
        println!("{}", format!("{}", "Modified time: <deterministic>".to_string()));
        print!("Is directory: {}\n", (*fileInfo.lock().unwrap().as_ref().unwrap()).is_dir());

                // Copy file
        println!("{}", format!("{}", "\n--- Copying file ---".to_string()));

        let mut copyFilename = Arc::new(Mutex::new(Some("test_file_copy.txt".to_string())));

        let (mut sourceFile, __tmp_1) = os::open({ let __arg_holder = filename.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1;;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        print!("Error opening source file: {}\n", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
        let sourceFile_defer_captured = sourceFile.clone(); __defer_stack.push(Box::new(move || {
        { let __recv = sourceFile_defer_captured.clone(); let __recv_ptr: *mut os_File = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut os_File }; let __result = unsafe { &mut *__recv_ptr }.close(); __result };
    }));

        let (mut destFile, __tmp_1) = { let __path = (*copyFilename.lock().unwrap().as_ref().unwrap()).clone(); match GoFile::create(&__path) { Ok(file) => (Arc::new(Mutex::new(Some(file))), Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>))), Err(e) => (Arc::new(Mutex::new(Some(GoFile::empty()))), Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(e)))))) } }; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1;;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        print!("Error creating destination file: {}\n", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
        let destFile_defer_captured = destFile.clone(); __defer_stack.push(Box::new(move || {
        { let __recv = destFile_defer_captured.clone(); let __recv_ptr: *mut os_File = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut os_File }; let __result = unsafe { &mut *__recv_ptr }.close(); __result };
    }));

        let (mut bytesWritten, __tmp_1) = io::copy(destFile.clone(), sourceFile.clone()); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1;;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        print!("Error copying file: {}\n", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }

        print!("Copied {} bytes to '{}'\n", format!("{}", bytesWritten), format!("{}", { let __v = (*copyFilename.lock().unwrap().as_ref().unwrap()).clone(); __v }));

                // Read and process file content
        println!("{}", format!("{}", "\n--- Processing file content ---".to_string()));

        { let (__tmp_0, __tmp_1) = os::open({ let __arg_holder = filename.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }); file = __tmp_0.clone(); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        print!("Error opening file: {}\n", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
        let file_defer_captured = file.clone(); __defer_stack.push(Box::new(move || {
        { let __recv = file_defer_captured.clone(); let __recv_ptr: *mut os_File = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut os_File }; let __result = unsafe { &mut *__recv_ptr }.close(); __result };
    }));

        { let new_val = bufio::new_scanner(file.clone()).clone(); scanner = new_val; };
        let mut wordCount = Arc::new(Mutex::new(Some(0i64)));
        let mut lineCount = Arc::new(Mutex::new(Some(0i64)));
        let mut charCount = Arc::new(Mutex::new(Some(0i64)));

        while { let __recv = scanner.clone(); let __recv_ptr: *mut bufio_Scanner = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bufio_Scanner }; let __result = unsafe { &mut *__recv_ptr }.scan(); __result } {
        let mut line = { let __recv = scanner.clone(); let __recv_ptr: *mut bufio_Scanner = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bufio_Scanner }; let __result = unsafe { &mut *__recv_ptr }.text(); __result };
        { let mut guard = lineCount.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let __rhs = ({ let __len = (*line.lock().unwrap().as_ref().unwrap()).len(); __len }) as i64; let mut guard = charCount.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };

        let mut words = Arc::new(Mutex::new(Some({ let __s = (*line.lock().unwrap().as_ref().unwrap()).clone(); __s.split_whitespace().map(|__part| __part.to_string()).collect::<Vec<String>>() })));
        let mut words = { let __slice_header = GoSliceHeader::go_slice_header(&words); Arc::new(Mutex::new(__slice_header)) };
        { let __rhs = ({ let __len = (*words.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64; let mut guard = wordCount.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };

                // Process lines containing numbers
        if (*Arc::new(Mutex::new(Some({ let __s = (*line.lock().unwrap().as_ref().unwrap()).clone(); let __arg = "123".to_string(); __s.contains(&__arg) }))).lock().unwrap().as_ref().unwrap()) {
        print!("Found line with numbers: {}\n", { let __v = (*line.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }
    }

                // Process lines containing numbers
        print!("Statistics:\n");
        print!("  Lines: {}\n", { let __v = (*lineCount.lock().unwrap().as_ref().unwrap()).clone(); __v });
        print!("  Words: {}\n", { let __v = (*wordCount.lock().unwrap().as_ref().unwrap()).clone(); __v });
        print!("  Characters: {}\n", { let __v = (*charCount.lock().unwrap().as_ref().unwrap()).clone(); __v });

                // Write formatted data
        println!("{}", format!("{}", "\n--- Writing formatted data ---".to_string()));

        let mut dataFile = Arc::new(Mutex::new(Some("data.txt".to_string())));
        { let (__tmp_0, __tmp_1) = { let __path = (*dataFile.lock().unwrap().as_ref().unwrap()).clone(); match GoFile::create(&__path) { Ok(file) => (Arc::new(Mutex::new(Some(file))), Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>))), Err(e) => (Arc::new(Mutex::new(Some(GoFile::empty()))), Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(e)))))) } }; file = __tmp_0.clone(); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        print!("Error creating data file: {}\n", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
        let file_defer_captured = file.clone(); __defer_stack.push(Box::new(move || {
        { let __recv = file_defer_captured.clone(); let __recv_ptr: *mut os_File = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut os_File }; let __result = unsafe { &mut *__recv_ptr }.close(); __result };
    }));

                // Write structured data
        { let __s = format!("Name: {}\n", "John Doe".to_string()); (*file.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
        { let __s = format!("Age: {}\n", 30); (*file.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
        { let __s = format!("Score: {:.2}\n", 95.5); (*file.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
        { let __s = format!("Active: {}\n", true); (*file.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };

        print!("Formatted data written to '{}'\n", { let __v = (*dataFile.lock().unwrap().as_ref().unwrap()).clone(); __v });

                // Read and parse formatted data
        println!("{}", format!("{}", "\n--- Reading formatted data ---".to_string()));

        { let (__tmp_0, __tmp_1) = os::read_file({ let __arg_holder = dataFile.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *data.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        print!("Error reading data file: {}\n", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }

        print!("Data file contents:\n{}", (*Arc::new(Mutex::new(Some(String::from_utf8(std::vec::Vec::from((*data.lock().unwrap().as_ref().unwrap()).clone())).unwrap()))).lock().unwrap().as_ref().unwrap()));

                // Check if files exist
        println!("{}", format!("{}", "\n--- Checking file existence ---".to_string()));

        let mut files = Arc::new(Mutex::new(Some(vec![({ let __v = (*filename.lock().unwrap().as_ref().unwrap()).clone(); __v }), ({ let __v = (*copyFilename.lock().unwrap().as_ref().unwrap()).clone(); __v }), ({ let __v = (*dataFile.lock().unwrap().as_ref().unwrap()).clone(); __v }), "nonexistent.txt".to_string()])));

        { let __range_holder = files.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for f in __range_values.iter() {
        {
        let (_, mut err) = os::stat((*f).clone());;
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
            print!("File '{}' exists\n", f);;
        } else if os::is_not_exist({ let __err_source = err.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }) {
        print!("File '{}' does not exist\n", f);
    } else {
        print!("Error checking file '{}': {}\n", format!("{}", f), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    }
    }
    } }

                // Clean up - remove test files
        println!("{}", format!("{}", "\n--- Cleaning up ---".to_string()));

        let mut filesToRemove = Arc::new(Mutex::new(Some(vec![({ let __v = (*filename.lock().unwrap().as_ref().unwrap()).clone(); __v }), ({ let __v = (*copyFilename.lock().unwrap().as_ref().unwrap()).clone(); __v }), ({ let __v = (*dataFile.lock().unwrap().as_ref().unwrap()).clone(); __v })])));

        { let __range_holder = filesToRemove.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for f in __range_values.iter() {
        let mut err = { let __path = (*f).clone(); match std::fs::remove_file(&__path) { Ok(()) => Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>)), Err(e) => Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(e))))) } };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        print!("Error removing file '{}': {}\n", format!("{}", f), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    } else {
        print!("Removed file '{}'\n", f);
    }
    } }

                // Verify cleanup
        println!("{}", format!("{}", "\n--- Verifying cleanup ---".to_string()));

        { let __range_holder = filesToRemove.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for f in __range_values.iter() {
        {
        let (_, mut err) = os::stat((*f).clone());;
        if os::is_not_exist({ let __err_source = err.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }) {
            print!("File '{}' successfully removed\n", f);;
        } else {
            print!("File '{}' still exists\n", f);;
        }
    }
    } }

        println!("{}", format!("{}", "\nFile operations test completed!".to_string()));

        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
    }));
    match __go_panic_result {
        Ok(__go_value) => {
            go_restore_panic_hook(__go_previous_panic_hook);
            __go_value
        }
        Err(__go_panic_payload) => {
            go_store_panic_payload(__go_panic_payload);
            while let Some(f) = __defer_stack.pop() {
                if let Err(__go_defer_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
                    go_store_panic_payload(__go_defer_panic);
                }
            }
            go_restore_panic_hook(__go_previous_panic_hook);
            go_resume_unrecovered_panic();
            ()
        }
    }
}