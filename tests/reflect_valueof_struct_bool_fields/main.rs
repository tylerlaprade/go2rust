#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

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


#[derive(Debug, Clone, Default)]
struct GoReflectStructTag {
    raw: Rc<RefCell<Option<String>>>,
}

impl GoReflectStructTag {
    fn get(&self, key: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {
        let raw = (*self.raw.borrow().as_ref().unwrap()).clone();
        let key = (*key.borrow().as_ref().unwrap()).clone();
        Rc::new(RefCell::new(Some(go_reflect_tag_get(&raw, &key))))
    }
}

#[derive(Debug, Clone, Default)]
struct GoReflectField {
    name: Rc<RefCell<Option<String>>>,
    tag: Rc<RefCell<Option<GoReflectStructTag>>>,
}

#[derive(Debug, Clone, Default)]
struct GoReflectType {
    name: Rc<RefCell<Option<String>>>,
    kind: Rc<RefCell<Option<reflect_Kind>>>,
    elem: Rc<RefCell<Option<Box<GoReflectType>>>>,
    fields: Rc<RefCell<Option<Vec<GoReflectField>>>>,
}

impl std::fmt::Display for GoReflectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.borrow().as_ref().unwrap())
    }
}

impl PartialEq for GoReflectType {
    fn eq(&self, other: &Self) -> bool {
        *self.name.borrow() == *other.name.borrow() &&
            *self.kind.borrow() == *other.kind.borrow()
    }
}

impl Eq for GoReflectType {}

impl GoReflectType {
    fn string(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some((*self.name.borrow().as_ref().unwrap()).clone())))
    }

    fn kind(&self) -> Rc<RefCell<Option<reflect_Kind>>> {
        self.kind.clone()
    }

    fn elem(&self) -> Rc<RefCell<Option<GoReflectType>>> {
        let elem_guard = self.elem.borrow();
        let elem = elem_guard.as_ref().expect("reflect.Type.Elem requires an element type").as_ref().clone();
        Rc::new(RefCell::new(Some(elem)))
    }

    fn num_field(&self) -> i64 {
        self.fields.borrow().as_ref().unwrap().len() as i64
    }

    fn field(&self, index: Rc<RefCell<Option<i64>>>) -> Rc<RefCell<Option<GoReflectField>>> {
        let index = *index.borrow().as_ref().unwrap() as usize;
        Rc::new(RefCell::new(Some(self.fields.borrow().as_ref().unwrap().as_slice()[index].clone())))
    }
}

type GoReflectBoolGetter = Box<dyn Fn() -> bool>;
type GoReflectBoolSetter = Box<dyn FnMut(Rc<RefCell<Option<bool>>>) -> ()>;

#[derive(Clone)]
struct GoReflectValue {
    typ: Rc<RefCell<Option<GoReflectType>>>,
    fields: Rc<RefCell<Option<Vec<GoReflectValue>>>>,
    bool_getter: Rc<RefCell<Option<GoReflectBoolGetter>>>,
    bool_setter: Rc<RefCell<Option<GoReflectBoolSetter>>>,
    unsupported: Option<&'static str>,
}

impl GoReflectValue {
    fn panic_if_unsupported(&self, op: &str) {
        if let Some(message) = self.unsupported {
            panic!("{}: {}", op, message);
        }
    }

    fn elem(&self) -> Rc<RefCell<Option<GoReflectValue>>> {
        self.panic_if_unsupported("reflect.Value.Elem");
        Rc::new(RefCell::new(Some(self.clone())))
    }

    fn r#type(&self) -> Rc<RefCell<Option<GoReflectType>>> {
        self.panic_if_unsupported("reflect.Value.Type");
        self.typ.clone()
    }

    fn kind(&self) -> Rc<RefCell<Option<reflect_Kind>>> {
        self.panic_if_unsupported("reflect.Value.Kind");
        self.typ.borrow().as_ref().unwrap().kind()
    }

    fn field(&self, index: i64) -> Rc<RefCell<Option<GoReflectValue>>> {
        self.panic_if_unsupported("reflect.Value.Field");
        let index = index as usize;
        Rc::new(RefCell::new(Some(self.fields.borrow().as_ref().unwrap().as_slice()[index].clone())))
    }

    fn set<T>(&self, _value: T) {
        self.panic_if_unsupported("reflect.Value.Set");
        panic!("reflect.Value.Set requires typed lowering")
    }

    fn set_bool(&mut self, value: Rc<RefCell<Option<bool>>>) {
        self.panic_if_unsupported("reflect.Value.SetBool");
        let mut setter_guard = self.bool_setter.borrow_mut();
        let setter = setter_guard.as_mut().expect("reflect.Value.SetBool requires a settable bool field");
        setter(value);
    }

    fn bool(&self) -> bool {
        self.panic_if_unsupported("reflect.Value.Bool");
        let getter_guard = self.bool_getter.borrow();
        let getter = getter_guard.as_ref().expect("reflect.Value.Bool requires a bool field");
        getter()
    }
}

fn go_reflect_tag_get(raw: &str, key: &str) -> String {
    let prefix = format!("{}:\"", key);
    let Some(start) = raw.find(&prefix) else {
        return String::new();
    };
    let rest = &raw[start + prefix.len()..];
    let mut value = String::new();
    let mut escaped = false;
    for ch in rest.chars() {
        if escaped {
            value.push(ch);
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else if ch == '"' {
            break;
        } else {
            value.push(ch);
        }
    }
    value
}

fn __go_next_external_interface_id() -> usize {
    static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}



#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct reflect_Kind(pub u64);

impl PartialEq<u64> for reflect_Kind {
    fn eq(&self, other: &u64) -> bool {
        self.0 == *other
    }
}

impl PartialEq<reflect_Kind> for u64 {
    fn eq(&self, other: &reflect_Kind) -> bool {
        *self == other.0
    }
}

impl std::ops::BitAnd for reflect_Kind {
    type Output = reflect_Kind;
    fn bitand(self, other: Self) -> reflect_Kind {
        reflect_Kind(self.0 & other.0)
    }
}

impl std::ops::BitOr for reflect_Kind {
    type Output = reflect_Kind;
    fn bitor(self, other: Self) -> reflect_Kind {
        reflect_Kind(self.0 | other.0)
    }
}

impl std::fmt::Display for reflect_Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<reflect_Kind>")
    }
}


impl reflect_Kind {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Clone)]
pub struct reflect_Type {
    pub __go_id: usize,
    pub __go_value: Rc<dyn std::any::Any>,
}

impl reflect_Type {
    pub fn __go_from<T: 'static>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_value: Rc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
}

impl Default for reflect_Type {
    fn default() -> Self {
        Self { __go_id: 0, __go_value: Rc::new(()) }
    }
}

impl std::fmt::Debug for reflect_Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<reflect_Type>")
    }
}

impl std::fmt::Display for reflect_Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<reflect_Type>")
    }
}

impl PartialEq for reflect_Type {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for reflect_Type {}

impl PartialOrd for reflect_Type {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ord::cmp(self, other))
    }
}

impl Ord for reflect_Type {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct reflect_Value;

impl std::fmt::Display for reflect_Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<reflect_Value>")
    }
}


impl reflect_Value {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn elem(&self) -> Rc<RefCell<Option<reflect_Value>>> {
        panic!("reflect_Value.elem bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
    pub fn field<T0>(&self, _arg0: T0) -> Rc<RefCell<Option<reflect_Value>>> {
        panic!("reflect_Value.field bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
    pub fn r#type(&self) -> Rc<RefCell<Option<reflect_Type>>> {
        panic!("reflect_Value.r#type bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone)]
pub struct Flags {
    pub alpha: Rc<RefCell<Option<bool>>>,
    pub beta: Rc<RefCell<Option<bool>>>,
}

impl Flags {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.alpha.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.beta.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            alpha: __go_clone_0_0,
            beta: __go_clone_1_0,
        }
    }
}


impl Default for Flags {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(false)));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(false)));
        Self {
            alpha: __go_default_0_0,
            beta: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.alpha.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.beta.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


fn main() {
    let mut flags = Rc::new(RefCell::new(Some(Flags { alpha: Rc::new(RefCell::new(Some(false))), beta: Rc::new(RefCell::new(Some(false))) })));
    let mut names = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<bool>>>) -> ()>>>>>::from([]))));

    let mut rv = { let __recv = { let __reflect_target = flags.clone(); Rc::new(RefCell::new(Some(GoReflectValue { typ: Rc::new(RefCell::new(Some(GoReflectType { name: Rc::new(RefCell::new(Some("main.Flags".to_string()))), kind: Rc::new(RefCell::new(Some(reflect_Kind(25)))), elem: Rc::new(RefCell::new(None::<Box<GoReflectType>>)), fields: Rc::new(RefCell::new(Some(vec![GoReflectField { name: Rc::new(RefCell::new(Some("Alpha".to_string()))), tag: Rc::new(RefCell::new(Some(GoReflectStructTag { raw: Rc::new(RefCell::new(Some("".to_string()))) }))) }, GoReflectField { name: Rc::new(RefCell::new(Some("Beta".to_string()))), tag: Rc::new(RefCell::new(Some(GoReflectStructTag { raw: Rc::new(RefCell::new(Some("".to_string()))) }))) }]))) }))), fields: Rc::new(RefCell::new(Some(vec![GoReflectValue { typ: Rc::new(RefCell::new(Some(GoReflectType { name: Rc::new(RefCell::new(Some("bool".to_string()))), kind: Rc::new(RefCell::new(Some(reflect_Kind(1)))), elem: Rc::new(RefCell::new(None::<Box<GoReflectType>>)), fields: Rc::new(RefCell::new(Some(vec![]))) }))), fields: Rc::new(RefCell::new(Some(vec![]))), bool_getter: Rc::new(RefCell::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move || -> bool { let __target_guard = __field_target.borrow(); let __target_value = __target_guard.as_ref().expect("reflect.Value.Bool requires a struct value"); let __field_value = { let __field_guard = __target_value.alpha.borrow(); (*__field_guard.as_ref().unwrap()).clone() }; __field_value }) as GoReflectBoolGetter }))), bool_setter: Rc::new(RefCell::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move |__value: Rc<RefCell<Option<bool>>>| { let __new_value = (*__value.borrow().as_ref().unwrap()).clone(); let mut __target_guard = __field_target.borrow_mut(); let __target_value = __target_guard.as_mut().expect("reflect.Value.SetBool requires a settable struct value"); *__target_value.alpha.borrow_mut() = Some(__new_value); }) as GoReflectBoolSetter }))), unsupported: None }, GoReflectValue { typ: Rc::new(RefCell::new(Some(GoReflectType { name: Rc::new(RefCell::new(Some("bool".to_string()))), kind: Rc::new(RefCell::new(Some(reflect_Kind(1)))), elem: Rc::new(RefCell::new(None::<Box<GoReflectType>>)), fields: Rc::new(RefCell::new(Some(vec![]))) }))), fields: Rc::new(RefCell::new(Some(vec![]))), bool_getter: Rc::new(RefCell::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move || -> bool { let __target_guard = __field_target.borrow(); let __target_value = __target_guard.as_ref().expect("reflect.Value.Bool requires a struct value"); let __field_value = { let __field_guard = __target_value.beta.borrow(); (*__field_guard.as_ref().unwrap()).clone() }; __field_value }) as GoReflectBoolGetter }))), bool_setter: Rc::new(RefCell::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move |__value: Rc<RefCell<Option<bool>>>| { let __new_value = (*__value.borrow().as_ref().unwrap()).clone(); let mut __target_guard = __field_target.borrow_mut(); let __target_value = __target_guard.as_mut().expect("reflect.Value.SetBool requires a settable struct value"); *__target_value.beta.borrow_mut() = Some(__new_value); }) as GoReflectBoolSetter }))), unsupported: None }]))), bool_getter: Rc::new(RefCell::new(None)), bool_setter: Rc::new(RefCell::new(None)), unsupported: None }))) }; let __result = (*__recv.borrow().as_ref().unwrap()).elem(); __result };
    let mut rt = (*rv.borrow().as_ref().unwrap()).r#type();
    let mut i = Rc::new(RefCell::new(Some(0i64)));
    while (*i.borrow().as_ref().unwrap()) < (*rt.borrow().as_ref().unwrap()).num_field() {
        let mut field = (*rv.borrow().as_ref().unwrap()).field({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() });
        { let __map_key = { let __map_key_holder = Rc::new(RefCell::new(Some({ let __s = (*(*(*rt.borrow().as_ref().unwrap()).field(Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })))).borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone(); __s.to_lowercase() }))).clone(); let __map_key_guard = __map_key_holder.borrow(); let __cloned = (*__map_key_guard.as_ref().unwrap()).clone(); drop(__map_key_guard); __cloned }; let __map_value = Rc::new(RefCell::new(Some({ let mut __recv = (*field.borrow().as_ref().unwrap()).clone(); Box::new(move |__arg0: Rc<RefCell<Option<bool>>>| { __recv.set_bool(__arg0) }) as Box<dyn FnMut(Rc<RefCell<Option<bool>>>) -> ()> }))); (*names.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    { let __f_holder = { let __map_holder = names.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"alpha".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }; let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<bool>>>) -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<bool>>>) -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(true)))) };
    println!("{} {}", format!("{}", (*(*flags.borrow().as_ref().unwrap()).alpha.borrow().as_ref().unwrap())), format!("{}", (*(*flags.borrow().as_ref().unwrap()).beta.borrow().as_ref().unwrap())));
}