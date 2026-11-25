#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use std::any::Any;
use std::cell::{RefCell};
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
    /// Whether this is the nil slice. A slice variable's nil is the None in
    /// its cell; this one is for slices held bare, such as the elements of
    /// a [][]byte, whose zero value is nil too.
    nil: bool,
}

unsafe impl<T: Send> Send for GoSlice<T> {}
unsafe impl<T: Send + Sync> Sync for GoSlice<T> {}

/// The prelude's Vec and vec! are Go slices in generated code.
pub type Vec<T> = GoSlice<T>;

impl<T> Clone for GoSlice<T> {
    /// Copies the header. The copy shares the elements, as a Go slice
    /// assignment does.
    fn clone(&self) -> Self {
        GoSlice { backing: self.backing.clone(), off: self.off, len: self.len, cap: self.cap, nil: self.nil }
    }
}

impl<T> GoSlice<T> {
    pub fn new() -> Self {
        Self::from(std::vec::Vec::new())
    }

    pub fn with_capacity(__cap: usize) -> Self {
        GoSlice { backing: std::sync::Arc::from(std::cell::UnsafeCell::new(std::vec::Vec::with_capacity(__cap))), off: 0, len: 0, cap: __cap, nil: false }
    }

    /// The nil slice: no elements, no backing array of its own.
    pub fn nil() -> Self {
        GoSlice { backing: std::sync::Arc::from(std::cell::UnsafeCell::new(std::vec::Vec::new())), off: 0, len: 0, cap: 0, nil: true }
    }

    /// Go's s == nil.
    pub fn is_nil(&self) -> bool {
        self.nil
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
        GoSlice { backing: self.backing.clone(), off: self.off + __lo, len: __hi - __lo, cap: self.cap - __lo, nil: self.nil }
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
        GoSlice { backing: std::sync::Arc::from(std::cell::UnsafeCell::new(__elems)), off: 0, len: __len, cap: __cap.max(__len), nil: false }
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
        self.nil = false;
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

impl<T: Clone + Default> GoSlice<T> {
    /// Go's s[lo:hi:max] for an element type with a zero value. Up to the
    /// capacity the backing array may not have stored the elements, after an
    /// append grew it; in Go they are zero, so the zeros are stored first.
    pub fn reslice3_zeroed(&self, __lo: usize, __hi: usize, __max: usize) -> Self {
        if __hi <= self.cap && __max <= self.cap {
            let __need = self.off + __hi;
            let __backing = self.backing_mut();
            if __need > __backing.len() {
                __backing.resize(__need, T::default());
            }
        }
        self.reslice3(__lo, __hi, __max)
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
    /// A slice's zero value is nil.
    fn default() -> Self {
        Self::nil()
    }
}

impl<T> From<std::vec::Vec<T>> for GoSlice<T> {
    fn from(__elems: std::vec::Vec<T>) -> Self {
        let __len = __elems.len();
        GoSlice { backing: std::sync::Arc::from(std::cell::UnsafeCell::new(__elems)), off: 0, len: __len, cap: __len, nil: false }
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
fn format_nested_slice<T, C, Inner>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[Inner]>,
    Inner: AsRef<[T]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        format_nested_slice_values(s.as_ref())
    } else {
        "[]".to_string()
    }
}
fn format_nested_slice_values<T, Inner>(slice: &[Inner]) -> String
where
    Inner: AsRef<[T]>,
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|inner| format_slice_values(inner.as_ref())).collect();
    format!("[{}]", formatted.join(" "))
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
    // A nil pointer's cell is empty, so reading through it unwraps None.
    if __text == "called `Option::unwrap()` on a `None` value" || __text == "nil pointer dereference" || __text == "invalid memory address or nil pointer dereference" {
        return "runtime error: invalid memory address or nil pointer dereference".to_string();
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

fn go_float_string(__value: f64, __single: bool) -> String {
    // The names are prefixed: this helper is at the crate root, where a Go
    // package variable of the same name would be a static it cannot shadow.
    if __value.is_nan() {
        return "NaN".to_string();
    }
    if __value.is_infinite() {
        return if __value < 0.0 { "-Inf".to_string() } else { "+Inf".to_string() };
    }
    let __sign = if __value.is_sign_negative() { "-" } else { "" };
    let __shortest = if __single { format!("{:e}", __value.abs() as f32) } else { format!("{:e}", __value.abs()) };
    let (__mantissa, __exponent_text) = __shortest.split_once('e').unwrap();
    let __digits: String = __mantissa.chars().filter(|__c| __c.is_ascii_digit()).collect();
    let __exponent: i32 = __exponent_text.parse().unwrap();
    if __exponent < -4 || __exponent >= 6 {
        let mut __text = String::from(__sign);
        __text.push_str(&__digits[..1]);
        if __digits.len() > 1 {
            __text.push('.');
            __text.push_str(&__digits[1..]);
        }
        __text.push('e');
        __text.push(if __exponent < 0 { '-' } else { '+' });
        __text.push_str(&format!("{:02}", __exponent.abs()));
        return __text;
    }
    if __exponent < 0 {
        return format!("{}0.{}{}", __sign, "0".repeat((-__exponent - 1) as usize), __digits);
    }
    let __whole = (__exponent + 1) as usize;
    if __digits.len() <= __whole {
        return format!("{}{}{}", __sign, __digits, "0".repeat(__whole - __digits.len()));
    }
    format!("{}{}.{}", __sign, &__digits[..__whole], &__digits[__whole..])
}

// A value inside a slice or a map prints as fmt prints it: a float through
// go_float_string, anything else through its Display. The element type may
// borrow, so it is recognized by name rather than by TypeId.
fn go_display_value<T: Display>(__value: &T) -> String {
    let __name = std::any::type_name::<T>();
    if __name == "f64" {
        // T is f64.
        return go_float_string(unsafe { *(__value as *const T as *const f64) }, false);
    }
    if __name == "f32" {
        // T is f32.
        return go_float_string(unsafe { *(__value as *const T as *const f32) } as f64, true);
    }
    __value.to_string()
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub alive: Arc<Mutex<Option<bool>>>,
    pub age: Arc<Mutex<Option<i64>>>,
}

impl Cell {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.alive.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.age.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            alive: __go_clone_0_0,
            age: __go_clone_1_0,
        }
    }
}


impl Default for Cell {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            alive: __go_default_0_0,
            age: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.alive.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.age.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct Board {
    pub cells: Arc<Mutex<Option<Vec<Vec<Cell>>>>>,
    pub w: Arc<Mutex<Option<i64>>>,
    pub h: Arc<Mutex<Option<i64>>>,
}

impl Board {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.cells.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.w.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_1 = { let __guard = self.h.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            cells: __go_clone_0_0,
            w: __go_clone_1_0,
            h: __go_clone_1_1,
        }
    }
}


impl Default for Board {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_1 = Arc::new(Mutex::new(Some(0)));
        Self {
            cells: __go_default_0_0,
            w: __go_default_1_0,
            h: __go_default_1_1,
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_nested_slice(&self.cells));
        let __go_fmt_1 = format!("{}", (*self.w.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.h.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


impl Board {
    pub fn neighbors(&self, x: Arc<Mutex<Option<i64>>>, y: Arc<Mutex<Option<i64>>>) -> i64 {
        let mut n = Arc::new(Mutex::new(Some(0i64)));
        let mut dy = Arc::new(Mutex::new(Some((-1i64))));
    while { let __tmp_x = { let __v = (*dy.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x <= __tmp_y } {
        let mut dx = Arc::new(Mutex::new(Some((-1i64))));
    while { let __tmp_x = { let __v = (*dx.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x <= __tmp_y } {
        if { let __tmp_x = { let __v = (*dx.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*dy.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let mut guard = dx.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue
    }
        let (mut xx, mut yy) = ({ let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*dx.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))); __tuple_value }, { let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*dy.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))); __tuple_value });
        if {
            let __go_cond_0 = {
                let __go_cond_1 = {
                    let __go_cond_2 = {
                        let __go_cond_3 = { let __tmp_x = { let __v = (*xx.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y };
                        if __go_cond_3 {
                            let __go_cond_4 = { let __tmp_x = { let __v = (*xx.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.w.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y };
                            __go_cond_4
                        } else {
                            false
                        }
                    };
                    if __go_cond_2 {
                        let __go_cond_5 = { let __tmp_x = { let __v = (*yy.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y };
                        __go_cond_5
                    } else {
                        false
                    }
                };
                if __go_cond_1 {
                    let __go_cond_6 = { let __tmp_x = { let __v = (*yy.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.h.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y };
                    __go_cond_6
                } else {
                    false
                }
            };
            if __go_cond_0 {
                let __go_cond_7 = (*({ let __seq = { let __seq_holder = self.cells.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*yy.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })[({ let __v = (*xx.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone().alive.lock().unwrap().as_ref().unwrap());
                __go_cond_7
            } else {
                false
            }
        } {
        { let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let mut guard = dx.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let mut guard = dy.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        return { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }

    pub fn step(&self) -> Arc<Mutex<Option<Board>>> {
        let mut next = new_board(Arc::new(Mutex::new(Some({ let __selector_holder = self.w.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = self.h.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }))));
        let mut y = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.h.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        let mut x = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.w.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        let mut n = self.neighbors(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut c = Arc::new(Mutex::new(Some(({ let __seq = { let __seq_holder = self.cells.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })[({ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone().__go_value_clone())));
        if {
            let __go_cond_0 = {
                let __go_cond_1 = (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).alive.clone(); __field }.lock().unwrap().as_ref().unwrap());
                if __go_cond_1 {
                    let __go_cond_2 = {
                        let __go_cond_3 = { let __tmp_x = n; let __tmp_y = 2; __tmp_x == __tmp_y };
                        if __go_cond_3 {
                            true
                        } else {
                            let __go_cond_4 = { let __tmp_x = n; let __tmp_y = 3; __tmp_x == __tmp_y };
                            __go_cond_4
                        }
                    };
                    __go_cond_2
                } else {
                    false
                }
            };
            if __go_cond_0 {
                true
            } else {
                let __go_cond_5 = {
                    let __go_cond_6 = !(*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).alive.clone(); __field }.lock().unwrap().as_ref().unwrap());
                    if __go_cond_6 {
                        let __go_cond_7 = { let __tmp_x = n; let __tmp_y = 3; __tmp_x == __tmp_y };
                        __go_cond_7
                    } else {
                        false
                    }
                };
                __go_cond_5
            }
        } {
        { let __outer_idx = ({ let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __idx = ({ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __rhs = Cell { alive: Arc::new(Mutex::new(Some(true))), age: Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).age.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x + __tmp_y }))), ..Default::default() }; (*(*next.lock().unwrap().as_ref().unwrap()).cells.lock().unwrap().as_mut().unwrap())[__outer_idx][__idx] = __rhs; };
    }
        { let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let mut guard = y.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        return next.clone();
    }

    pub fn render(&self) -> Arc<Mutex<Option<Vec<String>>>> {
        let mut lines = Arc::new(Mutex::new(Some({ let mut v: Vec<String> = (0..(*self.h.clone().lock().unwrap().as_ref().unwrap()) as usize).map(|_| "".to_string()).collect(); v.truncate((0) as usize); v })));
        { let __range_holder = self.cells.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for row in __range_values.iter() {
        let mut buf = Arc::new(Mutex::new(Some(vec![0i32; (row.len()) as usize])));
        for (x, c) in row.iter().enumerate() {
        let c = &c.__go_value_clone();
        { let __idx = (x) as usize; let __rhs = ('.' as i32); (*buf.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
        if (*c.alive.lock().unwrap().as_ref().unwrap()) {
        { let __idx = (x) as usize; let __rhs = ('#' as i32); (*buf.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    }
    }
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = lines.clone(); let __append_value = (*Arc::new(Mutex::new(Some((*buf.lock().unwrap().as_ref().unwrap()).iter().map(|&c| char::from_u32(c as u32).unwrap_or('\u{FFFD}')).collect::<String>()))).lock().unwrap().as_ref().unwrap()).clone(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *lines.lock().unwrap() = __slice_header; };
    } }
        return lines.clone();
    }
}

pub fn new_board(w: Arc<Mutex<Option<i64>>>, h: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<Board>>> {
    let mut cells = Arc::new(Mutex::new(Some(vec![<Vec<Cell>>::nil(); ({ let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize])));
    for y in 0..(({ let __range_holder = cells.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        { let __idx = (y) as usize; let __rhs = (*Arc::new(Mutex::new(Some((0..({ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize).map(|_| Default::default()).collect::<Vec<Cell>>()))).lock().unwrap().as_ref().unwrap()).clone(); (*cells.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    }
    return Arc::new(Mutex::new(Some(Board { cells: { let __slice_header = GoSliceHeader::go_slice_header(&cells); Arc::new(Mutex::new(__slice_header)) }, w: Arc::new(Mutex::new(Some({ let __arg_holder = w.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), h: Arc::new(Mutex::new(Some({ let __arg_holder = h.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
}

fn main() {
    let mut wg = WaitGroup::new();
    let mut rows = Arc::new(Mutex::new(Some(vec![<Vec<i32>>::nil(); (2) as usize])));
    for i in 0..(({ let __range_holder = rows.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        wg.add(1);
        let rows_thread = rows.clone(); let wg_thread = wg.clone(); let __go_arg_0 = Arc::new(Mutex::new(Some(i as i64))); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();
        let mut __closure = move |i: Arc<Mutex<Option<i64>>>| {
            let __go_previous_panic_hook = go_silence_panic_hook();
let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
let wg_defer_captured = wg_thread.clone(); __defer_stack.push(Box::new(move || {
        wg_defer_captured.done();
    }));;
            { let __idx = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __rhs = (*Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(format!("{}{}", format!("{}", "row".to_string()), format!("{}", { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }))))).lock().unwrap().as_ref().unwrap()).chars().map(|c| c as i32).collect::<Vec<_>>()))).lock().unwrap().as_ref().unwrap()).clone(); (*rows_thread.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };;
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

        };
        __closure(__go_arg_0);
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    }
    wg.wait();
    { let __range_holder = rows.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for row in __range_values.iter() {
        println!("{}", format!("{}", (*Arc::new(Mutex::new(Some((row).iter().map(|&c| char::from_u32(c as u32).unwrap_or('\u{FFFD}')).collect::<String>()))).lock().unwrap().as_ref().unwrap())));
    } }

    let mut b = new_board(Arc::new(Mutex::new(Some(5))), Arc::new(Mutex::new(Some(5))));
    { let new_val = true; *({ let __seq = { let __seq_holder = (*b.lock().unwrap().as_ref().unwrap()).cells.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(2) as usize].clone() })[(1) as usize].clone().alive.lock().unwrap() = Some(new_val); };
    { let new_val = true; *({ let __seq = { let __seq_holder = (*b.lock().unwrap().as_ref().unwrap()).cells.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(2) as usize].clone() })[(2) as usize].clone().alive.lock().unwrap() = Some(new_val); };
    { let new_val = true; *({ let __seq = { let __seq_holder = (*b.lock().unwrap().as_ref().unwrap()).cells.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(2) as usize].clone() })[(3) as usize].clone().alive.lock().unwrap() = Some(new_val); };
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x < __tmp_y } {
        { let __range_holder = { let __recv = b.clone(); let __recv_ptr: *const Board = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Board }; let __result = unsafe { &*__recv_ptr }.render(); __result }.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for line in __range_values.iter() {
        println!("{}", format!("{}", line));
    } }
        println!();
        { let new_val = { let __recv = b.clone(); let __recv_ptr: *const Board = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Board }; let __result = unsafe { &*__recv_ptr }.step(); __result }.clone(); b = new_val; };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    println!("{} {}", format!("{}", (*({ let __seq = { let __seq_holder = (*b.lock().unwrap().as_ref().unwrap()).cells.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() })[(2) as usize].clone().age.lock().unwrap().as_ref().unwrap())), format!("{}", (*({ let __seq = { let __seq_holder = (*b.lock().unwrap().as_ref().unwrap()).cells.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(2) as usize].clone() })[(2) as usize].clone().age.lock().unwrap().as_ref().unwrap())));

    let mut board = Arc::new(Mutex::new(Some(vec![<Vec<i32>>::nil(); (3) as usize])));
    for i in 0..(({ let __range_holder = board.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        { let __idx = (i) as usize; let __rhs = (*Arc::new(Mutex::new(Some(("...".to_string()).chars().map(|c| c as i32).collect::<Vec<_>>()))).lock().unwrap().as_ref().unwrap()).clone(); (*board.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    }
    { let __outer_idx = (1) as usize; let __idx = (1) as usize; let __rhs = ('X' as i32); (*board.lock().unwrap().as_mut().unwrap())[__outer_idx][__idx] = __rhs; };
    { let __range_holder = board.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for row in __range_values.iter() {
        println!("{}", format!("{}", (*Arc::new(Mutex::new(Some((row).iter().map(|&c| char::from_u32(c as u32).unwrap_or('\u{FFFD}')).collect::<String>()))).lock().unwrap().as_ref().unwrap())));
    } }
    println!("{} {} {}", format!("{}", (*Arc::new(Mutex::new(Some(({ let __seq = { let __seq_holder = board.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }).iter().map(|&c| char::from_u32(c as u32).unwrap_or('\u{FFFD}')).collect::<String>()))).lock().unwrap().as_ref().unwrap())), format!("{}", (*Arc::new(Mutex::new(Some({ let __rune_slice_holder = Arc::new(Mutex::new(Some({
        let mut __seq = { let __seq = { let __seq_holder = board.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() };
        let __low = (1) as usize;
        let __high = __seq.len();
        let __max = __seq.capacity();
        __seq.reslice3_zeroed(__low, __high, __max)
    }))).clone(); let __rune_slice_guard = __rune_slice_holder.lock().unwrap(); (*__rune_slice_guard.as_ref().unwrap()).iter().map(|&c| char::from_u32(c as u32).unwrap_or('\u{FFFD}')).collect::<String>() }))).lock().unwrap().as_ref().unwrap())), format!("{}", ({ let __len = (*Arc::new(Mutex::new(Some(({ let __seq = { let __seq_holder = board.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(2) as usize].clone() }).iter().map(|&c| char::from_u32(c as u32).unwrap_or('\u{FFFD}')).collect::<String>()))).lock().unwrap().as_ref().unwrap()).len(); __len })));

    let mut bufs = Arc::new(Mutex::new(Some(vec![{ let __slice_holder = Arc::new(Mutex::new(Some(("ab".to_string()).as_bytes().to_go_slice()))).clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }, { let __slice_holder = Arc::new(Mutex::new(Some(("cd".to_string()).as_bytes().to_go_slice()))).clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }])));
    { let __range_holder = bufs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for buf in __range_values.iter() {
        print!("{}{}", format!("{}", (*Arc::new(Mutex::new(Some(String::from_utf8(std::vec::Vec::from((buf).clone())).unwrap()))).lock().unwrap().as_ref().unwrap())), format!("{}", " ".to_string()));
    } }
    println!("{}", format!("{}", (*Arc::new(Mutex::new(Some(String::from_utf8(std::vec::Vec::from(({ let __seq = { let __seq_holder = bufs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }).clone())).unwrap()))).lock().unwrap().as_ref().unwrap())));

    let mut empty = Arc::new(Mutex::new(Some(vec![<Vec<Cell>>::nil(); (2) as usize])));
    println!("{} {} {}", format!("{}", ({ let __len = (*empty.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", { let __nil_element = { let __seq = { let __seq_holder = empty.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; __nil_element.is_nil() }), format!("{}", { let __seq = { let __seq_holder = empty.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.len()));
}