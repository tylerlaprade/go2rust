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


/// A nil interface value in a slot that holds a Box<dyn Any>, such as a
/// []any element: it copies, prints and compares as nil.
#[derive(Clone, Copy, Debug, PartialEq)]
struct GoNilAny;

fn go_any_try_clone(value: &dyn Any) -> Option<Box<dyn Any>> {
    if let Some(v) = value.downcast_ref::<i32>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<i64>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<i8>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<i16>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<u32>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<u64>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<u8>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<u16>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<usize>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<isize>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<f64>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<f32>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<&'static str>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<bool>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<char>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<String>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<GoNilAny>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<notFound>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<notFound>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    None
}

fn go_any_clone(value: &dyn Any) -> Box<dyn Any> {
    go_any_try_clone(value).unwrap_or_else(|| panic!("go_any_clone: unsupported dynamic type; add typed lowering instead of cloning Box<dyn Any>"))
}

#[allow(dead_code)]
fn go_any_handle_copy(handle: &Rc<RefCell<Option<Box<dyn Any>>>>) -> Rc<RefCell<Option<Box<dyn Any>>>> {
    let __copy = match handle.borrow().as_ref() {
        None => None,
        Some(value) => match go_any_try_clone(&**value) {
            Some(copy) => Some(copy),
            None => return handle.clone(),
        },
    };
    Rc::new(RefCell::new(__copy))
}

/// Stores Go's copy of source's value in target's cell. A source that no
/// one else holds gives up its value; a value whose type has no copy comes
/// back for the caller to share instead.
#[allow(dead_code)]
fn go_any_store(target: &Rc<RefCell<Option<Box<dyn Any>>>>, source: Rc<RefCell<Option<Box<dyn Any>>>>) -> Option<Rc<RefCell<Option<Box<dyn Any>>>>> {
    if Rc::ptr_eq(target, &source) {
        return None;
    }
    let __value = if Rc::strong_count(&source) == 1 {
        source.borrow_mut().take()
    } else {
        let __copy = match source.borrow().as_ref() {
            None => None,
            Some(value) => match go_any_try_clone(&**value) {
                Some(copy) => Some(copy),
                None => return Some(source.clone()),
            },
        };
        __copy
    };
    *target.borrow_mut() = __value;
    None
}

struct GoPointerError<T> {
    handle: Rc<RefCell<Option<T>>>,
}

impl<T> GoPointerError<T> {
    fn new(handle: Rc<RefCell<Option<T>>>) -> Self { Self { handle } }
}

impl<T: std::fmt::Display> std::fmt::Display for GoPointerError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let guard = self.handle.borrow();
        match guard.as_ref() {
            Some(value) => write!(f, "{}", value),
            None => Ok(()),
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Debug for GoPointerError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl<T: std::fmt::Display + 'static> std::error::Error for GoPointerError<T> {}

fn __go_type_name(val: &dyn Any) -> &'static str {
    if val.is::<i64>() { return "int" }
    if val.is::<i32>() { return "int32" }
    if val.is::<char>() { return "int32" }
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
    if val.is::<GoNilAny>() { return "<nil>" }
    if val.is::<Rc<RefCell<Option<notFound>>>>() { return "*main.notFound" }
    if val.is::<Rc<RefCell<Option<pointerError>>>>() { return "*main.pointerError" }
    if val.is::<notFound>() { return "main.notFound" }
    std::any::type_name_of_val(val)
}

#[derive(Debug, Clone)]
pub struct notFound {
    pub name: Rc<RefCell<Option<String>>>,
}

impl notFound {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for notFound {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for notFound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct pointerError {
    pub code: Rc<RefCell<Option<i64>>>,
}

impl pointerError {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.code.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            code: __go_clone_0_0,
        }
    }
}


impl Default for pointerError {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            code: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for pointerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}


impl notFound {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some(format!("{}{}", (*self.name.clone().borrow().as_ref().unwrap()), " not found".to_string()))));
    }
}

impl StdError for notFound {}


impl pointerError {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("pointer {}", (*self.code.borrow().as_ref().unwrap())))))
    }
}

impl StdError for pointerError {}


pub fn error_text(v: Rc<RefCell<Option<Box<dyn Any>>>>) -> (bool, Rc<RefCell<Option<String>>>) {
    {
    let _ts_subject = v.clone();
    let _ts_guard = _ts_subject.borrow();
    let _ts_is_nil = _ts_guard.as_ref().map_or(true, |__v| (**__v).is::<GoNilAny>());
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| {
        let mut __any = __v.as_ref() as &dyn Any;
        while let Some(__boxed) = __any.downcast_ref::<Box<dyn Any>>() {
            __any = __boxed.as_ref() as &dyn Any;
        }
        __any
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<notFound>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<pointerError>>>>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<notFound>>>>()).is_some() {
        let err = if let Some(__err_handle) = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>()) { { let __err_source = __err_handle; let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) } } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<notFound>()) { Rc::new(RefCell::new(Some(Rc::new(typed_val.clone()) as Rc<dyn StdError>))) } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<pointerError>>>>()) { Rc::new(RefCell::new(Some(Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError>))) } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<notFound>>>>()) { Rc::new(RefCell::new(Some(Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError>))) } else { unreachable!("type switch error case matched no error payload") };
        std::mem::drop(_ts_guard);
        return (true, Rc::new(RefCell::new(Some(format!("{}", err.borrow().as_ref().unwrap())))));;
    } else {
        let err = _ts_subject.clone();
        std::mem::drop(_ts_guard);
        return (false, Rc::new(RefCell::new(Some("".to_string()))));;
    }
    }
    unreachable!()
}

pub fn replace_then_error_text(v: Rc<RefCell<Option<Box<dyn Any>>>>, p: Rc<RefCell<Option<pointerError>>>) -> Rc<RefCell<Option<String>>> {
    {
    let _ts_subject = v.clone();
    let _ts_guard = _ts_subject.borrow();
    let _ts_is_nil = _ts_guard.as_ref().map_or(true, |__v| (**__v).is::<GoNilAny>());
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| {
        let mut __any = __v.as_ref() as &dyn Any;
        while let Some(__boxed) = __any.downcast_ref::<Box<dyn Any>>() {
            __any = __boxed.as_ref() as &dyn Any;
        }
        __any
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<notFound>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<pointerError>>>>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<notFound>>>>()).is_some() {
        let err = if let Some(__err_handle) = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>()) { { let __err_source = __err_handle; let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) } } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<notFound>()) { Rc::new(RefCell::new(Some(Rc::new(typed_val.clone()) as Rc<dyn StdError>))) } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<pointerError>>>>()) { Rc::new(RefCell::new(Some(Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError>))) } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<notFound>>>>()) { Rc::new(RefCell::new(Some(Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError>))) } else { unreachable!("type switch error case matched no error payload") };
        std::mem::drop(_ts_guard);
        { let new_val = pointerError { code: Rc::new(RefCell::new(Some(10))), ..Default::default() }; *p.borrow_mut() = Some(new_val); };;
        return Rc::new(RefCell::new(Some(format!("{}", err.borrow().as_ref().unwrap()))));;
    } else {
        let err = _ts_subject.clone();
        std::mem::drop(_ts_guard);
        return Rc::new(RefCell::new(Some("not error".to_string())));;
    }
    }
    unreachable!()
}

pub fn is_error(v: Rc<RefCell<Option<Box<dyn Any>>>>) -> bool {
    {
    let _ts_subject = v.clone();
    let _ts_guard = _ts_subject.borrow();
    let _ts_is_nil = _ts_guard.as_ref().map_or(true, |__v| (**__v).is::<GoNilAny>());
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| {
        let mut __any = __v.as_ref() as &dyn Any;
        while let Some(__boxed) = __any.downcast_ref::<Box<dyn Any>>() {
            __any = __boxed.as_ref() as &dyn Any;
        }
        __any
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<notFound>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<pointerError>>>>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<notFound>>>>()).is_some() {
        std::mem::drop(_ts_guard);
        return true;;
    } else {
        std::mem::drop(_ts_guard);
        return false;;
    }
    }
    unreachable!()
}

fn main() {
    let mut value: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(notFound { name: Rc::new(RefCell::new(Some("key".to_string()))), ..Default::default() }) as Box<dyn Any>)));
    let (mut valueErr, mut valueOK) = ({
        let val = value.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() {
                ({ let __err_source = typed_val; let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }, true)
            } else if let Some(typed_val) = any_val.downcast_ref::<notFound>() {
                (Rc::new(RefCell::new(Some(Rc::new(typed_val.clone()) as Rc<dyn StdError>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<pointerError>>>>() {
                (Rc::new(RefCell::new(Some(Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<notFound>>>>() {
                (Rc::new(RefCell::new(Some(Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError>))), true)
            } else {
                (Rc::new(RefCell::new(None::<Rc<dyn StdError>>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<Rc<dyn StdError>>)), false)
        }
    });
    let (mut valueCase, mut valueText) = error_text(go_any_handle_copy(&value));
    println!("{} {} {} {}", format!("{}", valueOK), format!("{}", valueCase), format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", valueErr.borrow().as_ref().unwrap())))).borrow().as_ref().unwrap())), format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", ({
        let val = value.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            { if let Some(__err_handle) = any_val.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() { (*__err_handle.borrow()).clone().expect("type assertion failed") } else if let Some(typed_val) = any_val.downcast_ref::<notFound>() { Rc::new(typed_val.clone()) as Rc<dyn StdError> } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<pointerError>>>>() { Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError> } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<notFound>>>>() { Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError> } else { panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "error") } }
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "error")
        }
    }))))).borrow().as_ref().unwrap())));

    let mut pointerValue = Rc::new(RefCell::new(Some(pointerError { code: Rc::new(RefCell::new(Some(7))), ..Default::default() })));
    let mut pointer: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(pointerValue.clone()) as Box<dyn Any>)));
    let (mut pointerErr, mut pointerOK) = ({
        let val = pointer.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() {
                ({ let __err_source = typed_val; let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }, true)
            } else if let Some(typed_val) = any_val.downcast_ref::<notFound>() {
                (Rc::new(RefCell::new(Some(Rc::new(typed_val.clone()) as Rc<dyn StdError>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<pointerError>>>>() {
                (Rc::new(RefCell::new(Some(Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<notFound>>>>() {
                (Rc::new(RefCell::new(Some(Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError>))), true)
            } else {
                (Rc::new(RefCell::new(None::<Rc<dyn StdError>>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<Rc<dyn StdError>>)), false)
        }
    });
    let (mut pointerCase, mut pointerText) = error_text(go_any_handle_copy(&pointer));
    println!("{} {} {} {}", format!("{}", pointerOK), format!("{}", pointerCase), format!("{}", { let __v = (*pointerText.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", ({
        let val = pointer.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            { if let Some(__err_handle) = any_val.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() { (*__err_handle.borrow()).clone().expect("type assertion failed") } else if let Some(typed_val) = any_val.downcast_ref::<notFound>() { Rc::new(typed_val.clone()) as Rc<dyn StdError> } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<pointerError>>>>() { Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError> } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<notFound>>>>() { Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError> } else { panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "error") } }
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "error")
        }
    }))))).borrow().as_ref().unwrap())));
    { let new_val = pointerError { code: Rc::new(RefCell::new(Some(8))), ..Default::default() }; *pointerValue.borrow_mut() = Some(new_val); };
    println!("{} {}", format!("{}", "assertion replacement".to_string()), format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", pointerErr.borrow().as_ref().unwrap())))).borrow().as_ref().unwrap())));
    println!("{} {}", format!("{}", "switch replacement".to_string()), format!("{}", (*replace_then_error_text(go_any_handle_copy(&pointer), pointerValue.clone()).borrow().as_ref().unwrap())));

    let mut valuePointer: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(notFound { name: Rc::new(RefCell::new(Some("pointer key".to_string()))), ..Default::default() }))).clone()) as Box<dyn Any>)));
    let (mut valuePointerErr, mut valuePointerOK) = ({
        let val = valuePointer.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() {
                ({ let __err_source = typed_val; let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }, true)
            } else if let Some(typed_val) = any_val.downcast_ref::<notFound>() {
                (Rc::new(RefCell::new(Some(Rc::new(typed_val.clone()) as Rc<dyn StdError>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<pointerError>>>>() {
                (Rc::new(RefCell::new(Some(Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<notFound>>>>() {
                (Rc::new(RefCell::new(Some(Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError>))), true)
            } else {
                (Rc::new(RefCell::new(None::<Rc<dyn StdError>>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<Rc<dyn StdError>>)), false)
        }
    });
    let (mut valuePointerCase, _) = error_text(go_any_handle_copy(&valuePointer));
    println!("{} {} {}", format!("{}", valuePointerOK), format!("{}", valuePointerCase), format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", valuePointerErr.borrow().as_ref().unwrap())))).borrow().as_ref().unwrap())));

    let mut nilPointer: Rc<RefCell<Option<pointerError>>> = Rc::new(RefCell::new(None));
    let mut typedNil: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(nilPointer.clone()) as Box<dyn Any>)));
    let (mut typedNilErr, mut typedNilOK) = ({
        let val = typedNil.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() {
                ({ let __err_source = typed_val; let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }, true)
            } else if let Some(typed_val) = any_val.downcast_ref::<notFound>() {
                (Rc::new(RefCell::new(Some(Rc::new(typed_val.clone()) as Rc<dyn StdError>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<pointerError>>>>() {
                (Rc::new(RefCell::new(Some(Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<notFound>>>>() {
                (Rc::new(RefCell::new(Some(Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError>))), true)
            } else {
                (Rc::new(RefCell::new(None::<Rc<dyn StdError>>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<Rc<dyn StdError>>)), false)
        }
    });
    let mut typedNilCase = is_error(go_any_handle_copy(&typedNil));
    println!("{} {} {}", format!("{}", typedNilOK), format!("{}", typedNilCase), format!("{}", { let __nil_result = (*typedNilErr.borrow()).is_none(); __nil_result }));

    let mut text: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new("plain".to_string()) as Box<dyn Any>)));
    let (_, mut textOK) = ({
        let val = text.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() {
                ({ let __err_source = typed_val; let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }, true)
            } else if let Some(typed_val) = any_val.downcast_ref::<notFound>() {
                (Rc::new(RefCell::new(Some(Rc::new(typed_val.clone()) as Rc<dyn StdError>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<pointerError>>>>() {
                (Rc::new(RefCell::new(Some(Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<notFound>>>>() {
                (Rc::new(RefCell::new(Some(Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError>))), true)
            } else {
                (Rc::new(RefCell::new(None::<Rc<dyn StdError>>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<Rc<dyn StdError>>)), false)
        }
    });
    let (mut textCase, _) = error_text(go_any_handle_copy(&text));
    let (mut textErr, mut textErrOK) = ({
        let val = text.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() {
                ({ let __err_source = typed_val; let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }, true)
            } else if let Some(typed_val) = any_val.downcast_ref::<notFound>() {
                (Rc::new(RefCell::new(Some(Rc::new(typed_val.clone()) as Rc<dyn StdError>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<pointerError>>>>() {
                (Rc::new(RefCell::new(Some(Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<notFound>>>>() {
                (Rc::new(RefCell::new(Some(Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError>))), true)
            } else {
                (Rc::new(RefCell::new(None::<Rc<dyn StdError>>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<Rc<dyn StdError>>)), false)
        }
    });
    println!("{} {} {} {}", format!("{}", textOK), format!("{}", textCase), format!("{}", textErrOK), format!("{}", { let __nil_result = (*textErr.borrow()).is_none(); __nil_result }));

    let _ = &valueText;
    let _ = &pointerErr;
}