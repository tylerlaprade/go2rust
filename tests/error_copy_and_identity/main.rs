#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use std::any::Any;
use std::cell::{RefCell};
use std::collections::BTreeMap;
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
    } else if let Some(v) = value.downcast_ref::<Missing>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() {
        match (*v.borrow()).as_ref() { Some(__err) => format!("{}", __err), None => "<nil>".to_string() }
    } else {
        panic!("go2rust: cannot print this value held in an interface: its type has no formatting case yet")
    }
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


#[derive(Debug, Clone, PartialEq)]
pub struct Missing {
    pub name: Rc<RefCell<Option<String>>>,
}

impl Missing {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for Missing {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Missing {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}


#[derive(Clone)]
pub struct Reader {
    pub err: Rc<RefCell<Option<Rc<dyn StdError>>>>,
    pub read: Rc<RefCell<Option<i64>>>,
}

impl Reader {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.err.clone();
        let __go_clone_1_0 = { let __guard = self.read.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            err: __go_clone_0_0,
            read: __go_clone_1_0,
        }
    }
}


impl Default for Reader {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(None));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            err: __go_default_0_0,
            read: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Reader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", match self.err.borrow().as_ref() { Some(__v) => format!("{}", __v), None => "<nil>".to_string() });
        let __go_fmt_1 = format!("{}", (*self.read.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
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

pub static ErrClosed: GoGlobal<Rc<dyn StdError>> = GoGlobal::new();


fn __go_init_globals() {
    *ErrNotFound.borrow_mut() = None;
    *ErrClosed.borrow_mut() = None;
    { let __rhs_holder = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("not found".to_string()))))).clone(); let new_val = (*__rhs_holder.borrow()).clone(); *ErrNotFound.borrow_mut() = new_val; }
    { let __rhs_holder = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("closed".to_string()))))).clone(); let new_val = (*__rhs_holder.borrow()).clone(); *ErrClosed.borrow_mut() = new_val; }
}


impl Missing {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("{}{}", "missing ".to_string(), (*self.name.clone().borrow().as_ref().unwrap())))))
    }
}

impl StdError for Missing {}

#[derive(Debug, Clone)]
pub struct __go_error_ptr_Missing(pub Rc<RefCell<Option<Missing>>>);

impl std::fmt::Display for __go_error_ptr_Missing {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => std::fmt::Display::fmt(__v, f), None => write!(f, "<nil>") }
    }
}

impl StdError for __go_error_ptr_Missing {}


impl Reader {
    pub fn err(&self) -> Rc<RefCell<Option<Rc<dyn StdError>>>> {
        { let __err_source = { let __field = self.err.clone(); __field }; let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }
    }

    pub fn fail(&mut self, err: Rc<RefCell<Option<Rc<dyn StdError>>>>) {
        { let __rhs_holder = err.clone(); let new_val = (*__rhs_holder.borrow()).clone(); *self.err.borrow_mut() = new_val; };
    }
}

pub fn run(fail: Rc<RefCell<Option<bool>>>) -> Rc<RefCell<Option<Rc<dyn StdError>>>> {
    if (*fail.borrow().as_ref().unwrap()) {
        return Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("boom".to_string())))));
    }
    Rc::new(RefCell::new(None))
}

pub fn find(key: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<Rc<dyn StdError>>>> {
    if (*key.borrow().as_ref().unwrap()).clone() == "".to_string() {
        return { let __err_source = ErrNotFound.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) };
    }
    if (*key.borrow().as_ref().unwrap()).clone() == "closed".to_string() {
        return { let __err_source = ErrClosed.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) };
    }
    Rc::new(RefCell::new(None))
}

pub fn lookup(key: Rc<RefCell<Option<String>>>) -> (i64, Rc<RefCell<Option<Rc<dyn StdError>>>>) {
    {
        let mut err = find(Rc::new(RefCell::new(Some({ let __arg_holder = key.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if { let __nil_result = (*err.borrow()).is_some(); __nil_result } {
            return (0, { let __err_source = err.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) });;
        }
    }
    ((*key.borrow().as_ref().unwrap()).len() as i64, Rc::new(RefCell::new(None)))
}

pub fn report(mut err: Rc<RefCell<Option<Rc<dyn StdError>>>>) {
    println!("{} {}", format!("{}", "report:".to_string()), format!("{}", { let __go_print_value = { let __go_print_handle = &(err); let __go_guard = __go_print_handle.borrow(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    *err.borrow_mut() = None;
}

pub fn classify(err: Rc<RefCell<Option<Rc<dyn StdError>>>>) -> Rc<RefCell<Option<String>>> {
    { let _switch_val = { let __err_source = err.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) };
    if (*_switch_val.borrow()).is_none() {
            return Rc::new(RefCell::new(Some("ok".to_string())));
        } else if { let __left_holder = _switch_val.clone(); let __left_value = { (*__left_holder.borrow()).clone() }; let __right_holder = { let __err_source = ErrNotFound.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __right_value = { (*__right_holder.borrow()).clone() }; match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__left), Some(__right)) => if let (Some(__go_left), Some(__go_right)) = (__left.downcast_ref::<Missing>(), __right.downcast_ref::<Missing>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__left.downcast_ref::<__go_error_ptr_Missing>(), __right.downcast_ref::<__go_error_ptr_Missing>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__left, &**__right) { true } else { false }, (None, None) => true, _ => false } } {
            return Rc::new(RefCell::new(Some("not found".to_string())));
        } else if { let __left_holder = _switch_val.clone(); let __left_value = { (*__left_holder.borrow()).clone() }; let __right_holder = { let __err_source = ErrClosed.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __right_value = { (*__right_holder.borrow()).clone() }; match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__left), Some(__right)) => if let (Some(__go_left), Some(__go_right)) = (__left.downcast_ref::<Missing>(), __right.downcast_ref::<Missing>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__left.downcast_ref::<__go_error_ptr_Missing>(), __right.downcast_ref::<__go_error_ptr_Missing>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__left, &**__right) { true } else { false }, (None, None) => true, _ => false } } {
            return Rc::new(RefCell::new(Some("closed".to_string())));
        }
    }
    Rc::new(RefCell::new(Some(format!("{}{}", "other: ".to_string(), (*Rc::new(RefCell::new(Some(format!("{}", err.borrow().as_ref().unwrap())))).borrow().as_ref().unwrap())))))
}

/// work replaces its own result in a deferred function.
pub fn work(fail: Rc<RefCell<Option<bool>>>) -> Rc<RefCell<Option<Rc<dyn StdError>>>> {
    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    let mut err: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(None));

    let mut err_defer_captured = err.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        if { let __nil_result = (*err_defer_captured.borrow()).is_some(); __nil_result } {
        { let __rhs_holder = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(format!("wrapped: {}", { let __go_print_value = { let __go_print_handle = &(err_defer_captured); let __go_guard = __go_print_handle.borrow(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text })))))).clone(); let new_val = (*__rhs_holder.borrow()).clone(); *err_defer_captured.borrow_mut() = new_val; };
    }
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
    if (*fail.borrow().as_ref().unwrap()) {
        {
        { let __rhs_holder = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("inner".to_string()))))).clone(); let new_val = (*__rhs_holder.borrow()).clone(); *err.borrow_mut() = new_val; };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return err.clone();
    }
    }
    {
        *err.borrow_mut() = None;;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return err.clone();
    }
}

fn main() {
    __go_init_all();
        // Assigning an error leaves the one it was assigned from.
    let mut last: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(None));
    let mut err = run(Rc::new(RefCell::new(Some(true))));
    if { let __nil_result = (*err.borrow()).is_some(); __nil_result } {
        { let __rhs_holder = err.clone(); let new_val = (*__rhs_holder.borrow()).clone(); *last.borrow_mut() = new_val; };
    }
    println!("{} {}", format!("{}", { let __nil_result = (*err.borrow()).is_some(); __nil_result }), format!("{}", { let __nil_result = (*last.borrow()).is_some(); __nil_result }));
    println!("{} {}", format!("{}", { let __go_print_value = { let __go_print_handle = &(err); let __go_guard = __go_print_handle.borrow(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __go_print_value = { let __go_print_handle = &(last); let __go_guard = __go_print_handle.borrow(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

        // A copy is a variable of its own.
    let mut e1 = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("first".to_string())))));
    let mut e2 = { let __err_source = e1.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) };
    { let __rhs_holder = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("second".to_string()))))).clone(); let new_val = (*__rhs_holder.borrow()).clone(); *e1.borrow_mut() = new_val; };
    println!("{} {}", format!("{}", { let __go_print_value = { let __go_print_handle = &(e1); let __go_guard = __go_print_handle.borrow(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __go_print_value = { let __go_print_handle = &(e2); let __go_guard = __go_print_handle.borrow(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

        // A package-level error can be returned, compared and returned again.
    println!("{} {} {}", format!("{}", { let __go_print_value = { let __go_print_handle = &(find(Rc::new(RefCell::new(Some("".to_string()))))); let __go_guard = __go_print_handle.borrow(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __go_print_value = { let __go_print_handle = &(find(Rc::new(RefCell::new(Some("closed".to_string()))))); let __go_guard = __go_print_handle.borrow(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __go_print_value = { let __go_print_handle = &(find(Rc::new(RefCell::new(Some("x".to_string()))))); let __go_guard = __go_print_handle.borrow(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    println!("{} {} {}", format!("{}", { let __left = find(Rc::new(RefCell::new(Some("".to_string())))).clone(); let __right = ErrNotFound.clone(); let __left_value = { (*__left.borrow()).clone() }; let __right_value = { (*__right.borrow()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<Missing>(), __r.downcast_ref::<Missing>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_Missing>(), __r.downcast_ref::<__go_error_ptr_Missing>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq }), format!("{}", { let __left = find(Rc::new(RefCell::new(Some("".to_string())))).clone(); let __right = ErrClosed.clone(); let __left_value = { (*__left.borrow()).clone() }; let __right_value = { (*__right.borrow()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<Missing>(), __r.downcast_ref::<Missing>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_Missing>(), __r.downcast_ref::<__go_error_ptr_Missing>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq }), format!("{}", { let __nil_result = (*find(Rc::new(RefCell::new(Some("x".to_string())))).borrow()).is_none(); __nil_result }));
    println!("{} {} {} {}", format!("{}", (*classify(find(Rc::new(RefCell::new(Some("".to_string()))))).borrow().as_ref().unwrap())), format!("{}", (*classify(find(Rc::new(RefCell::new(Some("closed".to_string()))))).borrow().as_ref().unwrap())), format!("{}", (*classify(find(Rc::new(RefCell::new(Some("x".to_string()))))).borrow().as_ref().unwrap())), format!("{}", (*classify({ let __err_source = err.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }).borrow().as_ref().unwrap())));
    {
        let (mut n, mut err) = lookup(Rc::new(RefCell::new(Some("".to_string()))));;
        if { let __left = err.clone(); let __right = ErrNotFound.clone(); let __left_value = { (*__left.borrow()).clone() }; let __right_value = { (*__right.borrow()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<Missing>(), __r.downcast_ref::<Missing>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_Missing>(), __r.downcast_ref::<__go_error_ptr_Missing>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq } {
            println!("{} {} {}", format!("{}", "lookup:".to_string()), format!("{}", n), format!("{}", { let __go_print_value = { let __go_print_handle = &(err); let __go_guard = __go_print_handle.borrow(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));;
        }
    }
    let mut got = find(Rc::new(RefCell::new(Some("".to_string()))));
    *got.borrow_mut() = None;
    println!("{} {} {}", format!("{}", { let __nil_result = (*got.borrow()).is_none(); __nil_result }), format!("{}", { let __go_print_value = { let __go_print_handle = &(ErrNotFound); let __go_guard = __go_print_handle.borrow(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __left = find(Rc::new(RefCell::new(Some("".to_string())))).clone(); let __right = ErrNotFound.clone(); let __left_value = { (*__left.borrow()).clone() }; let __right_value = { (*__right.borrow()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<Missing>(), __r.downcast_ref::<Missing>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_Missing>(), __r.downcast_ref::<__go_error_ptr_Missing>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq }));

        // Two errors with the same text are two errors. A copy is the same error.
    let mut a = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("same".to_string())))));
    let mut b = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("same".to_string())))));
    let mut c = { let __err_source = a.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) };
    println!("{} {} {} {}", format!("{}", { let __left = a.clone(); let __right = b.clone(); let __left_value = { (*__left.borrow()).clone() }; let __right_value = { (*__right.borrow()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<Missing>(), __r.downcast_ref::<Missing>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_Missing>(), __r.downcast_ref::<__go_error_ptr_Missing>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq }), format!("{}", { let __left = a.clone(); let __right = c.clone(); let __left_value = { (*__left.borrow()).clone() }; let __right_value = { (*__right.borrow()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<Missing>(), __r.downcast_ref::<Missing>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_Missing>(), __r.downcast_ref::<__go_error_ptr_Missing>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq }), format!("{}", { let __left = a.clone(); let __right = b.clone(); let __left_value = { (*__left.borrow()).clone() }; let __right_value = { (*__right.borrow()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<Missing>(), __r.downcast_ref::<Missing>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_Missing>(), __r.downcast_ref::<__go_error_ptr_Missing>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; !__eq }), format!("{}", { let __left = c.clone(); let __right = a.clone(); let __left_value = { (*__left.borrow()).clone() }; let __right_value = { (*__right.borrow()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<Missing>(), __r.downcast_ref::<Missing>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_Missing>(), __r.downcast_ref::<__go_error_ptr_Missing>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq }));

        // Passing an error does not give the variable away, and the function
        // cannot change it.
    report({ let __err_source = err.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) });
    report({ let __err_source = err.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) });
    println!("{}", format!("{}", { let __go_print_value = { let __go_print_handle = &(err); let __go_guard = __go_print_handle.borrow(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

        // An error in a struct field.
    let mut r = Rc::new(RefCell::new(Some(Reader { err: Default::default(), read: Rc::new(RefCell::new(Some(0))) })));
    (*r.borrow_mut().as_mut().unwrap()).fail({ let __err_source = err.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) });
    let mut held = (*r.borrow().as_ref().unwrap()).err();
    *held.borrow_mut() = None;
    println!("{} {} {} {}", format!("{}", { let __nil_result = (*held.borrow()).is_none(); __nil_result }), format!("{}", { let __go_print_value = { let __go_print_handle = &((*r.borrow().as_ref().unwrap()).err()); let __go_guard = __go_print_handle.borrow(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __left = (*r.borrow().as_ref().unwrap()).err.clone(); let __right = err.clone(); let __left_value = { (*__left.borrow()).clone() }; let __right_value = { (*__right.borrow()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<Missing>(), __r.downcast_ref::<Missing>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_Missing>(), __r.downcast_ref::<__go_error_ptr_Missing>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq }), format!("{}", { let __go_print_value = { let __go_print_handle = &(err); let __go_guard = __go_print_handle.borrow(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    { let __rhs_holder = ErrClosed.clone(); let new_val = (*__rhs_holder.borrow()).clone(); *(*r.borrow().as_ref().unwrap()).err.borrow_mut() = new_val; };
    println!("{} {}", format!("{}", { let __left = (*r.borrow().as_ref().unwrap()).err().clone(); let __right = ErrClosed.clone(); let __left_value = { (*__left.borrow()).clone() }; let __right_value = { (*__right.borrow()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<Missing>(), __r.downcast_ref::<Missing>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_Missing>(), __r.downcast_ref::<__go_error_ptr_Missing>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq }), format!("{}", (*classify((*r.borrow().as_ref().unwrap()).err()).borrow().as_ref().unwrap())));

        // Errors in a slice and in a map.
    let mut list = Rc::new(RefCell::new(Some(vec![{ let __err_source = err.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }, { let __err_source = last.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }, { let __err_source = ErrNotFound.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }, Rc::new(RefCell::new(None::<Rc<dyn StdError>>))])));
    { let __range_holder = list.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, item) in __range_values.iter().cloned().enumerate() {
        println!("{} {} {}", format!("{}", i), format!("{}", { let __go_print_value = { let __go_print_handle = &(item); let __go_guard = __go_print_handle.borrow(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __left = item.clone(); let __right = err.clone(); let __left_value = { (*__left.borrow()).clone() }; let __right_value = { (*__right.borrow()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<Missing>(), __r.downcast_ref::<Missing>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_Missing>(), __r.downcast_ref::<__go_error_ptr_Missing>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq }));
    } }
    { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = list.clone(); let __append_value = { let __err_source = e2.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let mut __appended: Vec<Rc<RefCell<Option<Rc<dyn StdError>>>>> = (*__append_target.borrow()).clone().unwrap_or_default(); __appended.push(__append_value); Rc::new(RefCell::new(Some(__appended))) })); *list.borrow_mut() = __slice_header; };
    println!("{} {} {}", format!("{}", (*list.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", { let __go_print_value = { let __go_print_handle = &((*list.borrow().as_ref().unwrap())[(4) as usize].clone()); let __go_guard = __go_print_handle.borrow(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __go_print_value = { let __go_print_handle = &(err); let __go_guard = __go_print_handle.borrow(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    let mut byName = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Rc<dyn StdError>>>>>::from([("a".to_string(), { let __err_source = err.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }), ("b".to_string(), { let __err_source = ErrClosed.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) })]))));
    println!("{} {} {}", format!("{}", { let __go_print_value = { let __go_print_handle = &({ let __map_holder = byName.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }); let __go_guard = __go_print_handle.borrow(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __left = { let __map_holder = byName.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"b".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __right = ErrClosed.clone(); let __left_value = { (*__left.borrow()).clone() }; let __right_value = { (*__right.borrow()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<Missing>(), __r.downcast_ref::<Missing>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_Missing>(), __r.downcast_ref::<__go_error_ptr_Missing>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq }), format!("{}", { let __nil_result = (*{ let __map_holder = byName.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"c".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.borrow()).is_none(); __nil_result }));

        // A custom error type.
    let mut m: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(Some(Rc::new(Missing { name: Rc::new(RefCell::new(Some("key".to_string()))), ..Default::default() }) as Rc<dyn StdError>)));
    let mut n = { let __err_source = m.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) };
    *m.borrow_mut() = None;
    println!("{} {}", format!("{}", { let __nil_result = (*m.borrow()).is_none(); __nil_result }), format!("{}", { let __go_print_value = { let __go_print_handle = &(n); let __go_guard = __go_print_handle.borrow(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

    *last.borrow_mut() = None;
    println!("{} {}", format!("{}", { let __nil_result = (*last.borrow()).is_none(); __nil_result }), format!("{}", { let __nil_result = (*err.borrow()).is_some(); __nil_result }));

        // An interface value holds a copy as well: it does not follow the
        // variable it was made from.
    let mut boxed: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some({ let __err_holder = err.clone(); let __err_guard = __err_holder.borrow(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<Missing>() { go_box_any_with_metadata(typed_val.clone(), "struct", true) } else { Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any> } } })));
    { let __rhs_holder = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("changed".to_string()))))).clone(); let new_val = (*__rhs_holder.borrow()).clone(); *err.borrow_mut() = new_val; };
    println!("{} {}", format!("{}", match boxed.borrow().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }), format!("{}", { let __go_print_value = { let __go_print_handle = &(err); let __go_guard = __go_print_handle.borrow(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

        // So does what a closure returns.
    let err_closure_clone = err.clone(); let mut check = Rc::new(RefCell::new(Some(Box::new(move || -> Rc<RefCell<Option<Rc<dyn StdError>>>> {
        return { let __err_source = err_closure_clone.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) };
    }) as Box<dyn FnMut() -> Rc<RefCell<Option<Rc<dyn StdError>>>>>)));
    let mut fromClosure = { let __f_ptr: *mut Box<dyn FnMut() -> Rc<RefCell<Option<Rc<dyn StdError>>>>> = { let mut __f_guard = check.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> Rc<RefCell<Option<Rc<dyn StdError>>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    *fromClosure.borrow_mut() = None;
    println!("{} {} {}", format!("{}", { let __nil_result = (*fromClosure.borrow()).is_none(); __nil_result }), format!("{}", { let __go_print_value = { let __go_print_handle = &(err); let __go_guard = __go_print_handle.borrow(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __left = { let __f_ptr: *mut Box<dyn FnMut() -> Rc<RefCell<Option<Rc<dyn StdError>>>>> = { let mut __f_guard = check.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> Rc<RefCell<Option<Rc<dyn StdError>>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }.clone(); let __right = err.clone(); let __left_value = { (*__left.borrow()).clone() }; let __right_value = { (*__right.borrow()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<Missing>(), __r.downcast_ref::<Missing>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_Missing>(), __r.downcast_ref::<__go_error_ptr_Missing>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq }));

        // A deferred function can still replace a named result.
    println!("{} {}", format!("{}", { let __go_print_value = { let __go_print_handle = &(work(Rc::new(RefCell::new(Some(true))))); let __go_guard = __go_print_handle.borrow(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __go_print_value = { let __go_print_handle = &(work(Rc::new(RefCell::new(Some(false))))); let __go_guard = __go_print_handle.borrow(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
