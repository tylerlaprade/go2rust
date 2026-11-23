#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use std::cell::{RefCell};
use std::cmp::Ord;
use std::collections::BTreeMap;
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

fn format_map<K: Display + Ord + Clone, V>(map: &Rc<RefCell<Option<BTreeMap<K, Rc<RefCell<Option<V>>>>>>>) -> String
where
    V: Display,
{
    let guard = map.borrow();
    if let Some(ref m) = *guard {
        let mut items: Vec<_> = m.iter().collect();
        items.sort_by_key(|(k, _)| (*k).clone());

        let formatted: Vec<String> = items
            .into_iter()
            .map(|(k, v)| {
                let v_guard = v.borrow();
                if let Some(ref val) = *v_guard {
                    format!("{}:{}", k, val)
                } else {
                    format!("{}:<nil>", k)
                }
            })
            .collect();

        format!("map[{}]", formatted.join(" "))
    } else {
        "map[]".to_string()
    }
}
fn format_slice<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[T]>,
    T: Display,
{
    let guard = slice.borrow();
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

fn format_slice_wrapped<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[Rc<RefCell<Option<T>>>]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| {
            let inner = v.borrow();
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
fn format_nested_slice<T, C, Inner>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[Inner]>,
    Inner: AsRef<[T]>,
    T: Display,
{
    let guard = slice.borrow();
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

#[derive(Debug, Clone, Default)]
pub struct Grid(pub Rc<RefCell<Option<Vec<Vec<i64>>>>>);


#[derive(Debug, Clone, Default)]
pub struct Sheet {
    pub cells: Rc<RefCell<Option<Vec<Vec<f64>>>>>,
    pub names: Rc<RefCell<Option<Vec<String>>>>,
}

impl Sheet {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.cells.clone();
        let __go_clone_1_0 = self.names.clone();
        Self {
            cells: __go_clone_0_0,
            names: __go_clone_1_0,
        }
    }
}

impl std::fmt::Display for Sheet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_nested_slice(&self.cells));
        let __go_fmt_1 = format!("{}", format_slice(&self.names));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl Sheet {
    pub fn bump(&mut self, i: Rc<RefCell<Option<i64>>>) {
        { let __outer_idx = ((*i.borrow().as_ref().unwrap())) as usize; let __idx = ((*i.borrow().as_ref().unwrap())) as usize; let __rhs = 0.5; let __seq_holder = self.cells.clone(); let mut __seq_guard = __seq_holder.borrow_mut(); let __seq = &mut __seq_guard.as_mut().unwrap()[__outer_idx]; __seq[__idx] = __seq[__idx] + __rhs; };
        { let __idx = ((*i.borrow().as_ref().unwrap())) as usize; let __rhs = "!".to_string(); let __seq_holder = self.names.clone(); let mut __seq_guard = __seq_holder.borrow_mut(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx].clone() + &__rhs; };
    }
}

fn main() {
    let mut a = Rc::new(RefCell::new(Some(vec![vec![1i64, 2i64], vec![3i64, 4i64]])));
    { let __outer_idx = (0) as usize; let __idx = (1) as usize; let __rhs = 10; let mut __seq_guard = a.borrow_mut(); let __seq = &mut __seq_guard.as_mut().unwrap()[__outer_idx]; __seq[__idx] = __seq[__idx] + __rhs; };
    {
        let __go_lhs_sequence_0 = a.clone();
        let __go_lhs_outer_index_0_0 = (1) as usize;
        let __go_lhs_index_0 = (0) as usize;
        let __go_lhs_sequence_1 = a.clone();
        let __go_lhs_outer_index_1_0 = (1) as usize;
        let __go_lhs_index_1 = (1) as usize;
        let __tmp_0 = (*a.borrow().as_ref().unwrap())[(1) as usize].clone()[(1) as usize].clone();
        let __tmp_1 = (*a.borrow().as_ref().unwrap())[(1) as usize].clone()[(0) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_outer_index_0_0][__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_outer_index_1_0][__go_lhs_index_1] = __tmp_1;
    };
    { let __outer_idx = (0) as usize; let __idx = (0) as usize; let mut __seq_guard = a.borrow_mut(); let __seq = &mut __seq_guard.as_mut().unwrap()[__outer_idx]; __seq[__idx] = __seq[__idx] + 1; }
    { let __outer_idx = (1) as usize; let __idx = (1) as usize; let __rhs = 2; let mut __seq_guard = a.borrow_mut(); let __seq = &mut __seq_guard.as_mut().unwrap()[__outer_idx]; __seq[__idx] = __seq[__idx] - __rhs; };
    let mut g = Rc::new(RefCell::new(Some(Grid(Rc::new(RefCell::new(Some(vec![vec![1i64, 2i64], vec![3i64, 4i64]])))))));
    { let __outer_idx = (0) as usize; let __idx = (1) as usize; let __rhs = 7; let __seq_holder = { let __named_slice = (*g.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let mut __seq_guard = __seq_holder.borrow_mut(); let __seq = &mut __seq_guard.as_mut().unwrap()[__outer_idx]; __seq[__idx] = __seq[__idx] * __rhs; };
    {
        let __go_lhs_sequence_0 = { let __named_slice = (*g.borrow().as_ref().unwrap()).0.clone(); __named_slice }.clone();
        let __go_lhs_outer_index_0_0 = (1) as usize;
        let __go_lhs_index_0 = (0) as usize;
        let __go_lhs_sequence_1 = { let __named_slice = (*g.borrow().as_ref().unwrap()).0.clone(); __named_slice }.clone();
        let __go_lhs_outer_index_1_0 = (1) as usize;
        let __go_lhs_index_1 = (1) as usize;
        let __tmp_0 = { let __seq_holder = { let __named_slice = (*g.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_index = (1) as usize; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() }[(1) as usize].clone();
        let __tmp_1 = { let __seq_holder = { let __named_slice = (*g.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_index = (1) as usize; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() }[(0) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_outer_index_0_0][__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_outer_index_1_0][__go_lhs_index_1] = __tmp_1;
    };
    { let __outer_idx = (0) as usize; let __idx = (0) as usize; let __seq_holder = { let __named_slice = (*g.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let mut __seq_guard = __seq_holder.borrow_mut(); let __seq = &mut __seq_guard.as_mut().unwrap()[__outer_idx]; __seq[__idx] = __seq[__idx] - 1; }
    println!("{} {}", format!("{}", format_nested_slice(&a)), format!("{}", format_nested_slice(&{ let __named_slice = (*g.borrow().as_ref().unwrap()).0.clone(); __named_slice })));

    let mut cube = Rc::new(RefCell::new(Some(vec![vec![vec![1i64, 2i64], vec![3i64]], vec![vec![4i64]]])));
    { let __outer_idx = (0) as usize; let __outer_idx_1 = (1) as usize; let __idx = (0) as usize; let __rhs = 30; (*cube.borrow_mut().as_mut().unwrap())[__outer_idx][__outer_idx_1][__idx] = __rhs; };
    { let __outer_idx = (1) as usize; let __outer_idx_1 = (0) as usize; let __idx = (0) as usize; let __rhs = 5; let mut __seq_guard = cube.borrow_mut(); let __seq = &mut __seq_guard.as_mut().unwrap()[__outer_idx][__outer_idx_1]; __seq[__idx] = __seq[__idx] * __rhs; };
    { let __outer_idx = (0) as usize; let __outer_idx_1 = (0) as usize; let __idx = (1) as usize; let mut __seq_guard = cube.borrow_mut(); let __seq = &mut __seq_guard.as_mut().unwrap()[__outer_idx][__outer_idx_1]; __seq[__idx] = __seq[__idx] - 1; }
    println!("{} {}", format!("{}", { let __formatted = format_nested_slice_values(&(*cube.borrow().as_ref().unwrap())[(0) as usize].clone()); __formatted }), format!("{}", (*cube.borrow().as_ref().unwrap())[(1) as usize].clone()[(0) as usize].clone()[(0) as usize].clone()));

    let mut board: Rc<RefCell<Option<[[String; 3]; 3]>>> = Rc::new(RefCell::new(Some(std::array::from_fn::<_, 3, _>(|_| std::array::from_fn::<_, 3, _>(|_| String::new())))));
    let mut i = Rc::new(RefCell::new(Some(0i64)));
    while (*i.borrow().as_ref().unwrap()) < 3 {
        let mut j = Rc::new(RefCell::new(Some(0i64)));
    while (*j.borrow().as_ref().unwrap()) < 3 {
        { let __outer_idx = ((*i.borrow().as_ref().unwrap())) as usize; let __idx = ((*j.borrow().as_ref().unwrap())) as usize; let __rhs = ".".to_string(); (*board.borrow_mut().as_mut().unwrap())[__outer_idx][__idx] = __rhs; };
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    { let __outer_idx = (1) as usize; let __idx = (1) as usize; let __rhs = "X".to_string(); (*board.borrow_mut().as_mut().unwrap())[__outer_idx][__idx] = __rhs; };
    { let __outer_idx = (0) as usize; let __idx = (2) as usize; let __rhs = "O".to_string(); let mut __seq_guard = board.borrow_mut(); let __seq = &mut __seq_guard.as_mut().unwrap()[__outer_idx]; __seq[__idx] = __seq[__idx].clone() + &__rhs; };
    println!("{}", format!("{}", format_nested_slice(&board)));

    let mut grid = Rc::new(RefCell::new(Some([std::array::from_fn::<_, 3, _>(|_| 0), std::array::from_fn::<_, 3, _>(|_| 0), std::array::from_fn::<_, 3, _>(|_| 0)])));
    let mut i = Rc::new(RefCell::new(Some(0i64)));
    while (*i.borrow().as_ref().unwrap()) < 3 {
        { let __outer_idx = ((*i.borrow().as_ref().unwrap())) as usize; let __idx = ((*i.borrow().as_ref().unwrap())) as usize; let __rhs = (*i.borrow().as_ref().unwrap()) + 1; (*grid.borrow_mut().as_mut().unwrap())[__outer_idx][__idx] = __rhs; };
        { let __outer_idx = ((*i.borrow().as_ref().unwrap())) as usize; let __idx = (2 - (*i.borrow().as_ref().unwrap())) as usize; let __rhs = 10; let mut __seq_guard = grid.borrow_mut(); let __seq = &mut __seq_guard.as_mut().unwrap()[__outer_idx]; __seq[__idx] = __seq[__idx] + __rhs; };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    println!("{}", format!("{}", format_nested_slice(&grid)));

    let mut paths = Rc::new(RefCell::new(Some(vec![vec![]; (4) as usize])));
    for i in 0..(({ let __range_holder = paths.clone(); let __range_guard = __range_holder.borrow(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        { let __idx = (i) as usize; let __rhs = (*Rc::new(RefCell::new(Some(vec![0; (4) as usize]))).borrow().as_ref().unwrap()).clone(); (*paths.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
        { let __outer_idx = (i) as usize; let __idx = (0) as usize; let __rhs = 1; (*paths.borrow_mut().as_mut().unwrap())[__outer_idx][__idx] = __rhs; };
    }
    let mut i = Rc::new(RefCell::new(Some(1i64)));
    while (*i.borrow().as_ref().unwrap()) < 4 {
        { let __outer_idx = (0) as usize; let __idx = ((*i.borrow().as_ref().unwrap())) as usize; let __rhs = 1; (*paths.borrow_mut().as_mut().unwrap())[__outer_idx][__idx] = __rhs; };
        let mut j = Rc::new(RefCell::new(Some(1i64)));
    while (*j.borrow().as_ref().unwrap()) < 4 {
        { let __outer_idx = ((*i.borrow().as_ref().unwrap())) as usize; let __idx = ((*j.borrow().as_ref().unwrap())) as usize; let __rhs = (*paths.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap()) - 1) as usize].clone()[((*j.borrow().as_ref().unwrap())) as usize].clone() + (*paths.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone()[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone(); (*paths.borrow_mut().as_mut().unwrap())[__outer_idx][__idx] = __rhs; };
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    println!("{} {}", format!("{}", (*paths.borrow().as_ref().unwrap())[(3) as usize].clone()[(3) as usize].clone()), format!("{}", format_nested_slice(&paths)));

    let mut s = Rc::new(RefCell::new(Some(Sheet { cells: Rc::new(RefCell::new(Some(vec![vec![1.0, 2.0], vec![3.0, 4.0]]))), names: Rc::new(RefCell::new(Some(vec!["a".to_string(), "b".to_string()]))), ..Default::default() })));
    (*s.borrow_mut().as_mut().unwrap()).bump(Rc::new(RefCell::new(Some(0))));
    (*s.borrow_mut().as_mut().unwrap()).bump(Rc::new(RefCell::new(Some(1))));
    {
        let __go_lhs_sequence_0 = (*s.borrow().as_ref().unwrap()).cells.clone();
        let __go_lhs_outer_index_0_0 = (1) as usize;
        let __go_lhs_index_0 = (0) as usize;
        let __go_lhs_sequence_1 = (*s.borrow().as_ref().unwrap()).cells.clone();
        let __go_lhs_outer_index_1_0 = (1) as usize;
        let __go_lhs_index_1 = (1) as usize;
        let __tmp_0 = (*(*s.borrow().as_ref().unwrap()).cells.borrow().as_ref().unwrap())[(1) as usize].clone()[(1) as usize].clone();
        let __tmp_1 = (*(*s.borrow().as_ref().unwrap()).cells.borrow().as_ref().unwrap())[(1) as usize].clone()[(0) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_outer_index_0_0][__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_outer_index_1_0][__go_lhs_index_1] = __tmp_1;
    };
    let mut words = Rc::new(RefCell::new(Some(vec!["x".to_string(), "y".to_string()])));
    { let __idx = (0) as usize; let __rhs = "z".to_string(); let mut __seq_guard = words.borrow_mut(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx].clone() + &__rhs; };
    { let __idx = (1) as usize; let __rhs = (*words.borrow().as_ref().unwrap())[(0) as usize].clone(); let mut __seq_guard = words.borrow_mut(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx].clone() + &__rhs; };
    let mut tags = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<String>>>>::from([("k".to_string(), Rc::new(RefCell::new(Some("v".to_string()))))]))));
    { let __map_holder = tags.clone(); let __key = "k".to_string(); let __rhs = "w".to_string(); let mut __map_guard = __map_holder.borrow_mut(); let __map = __map_guard.as_mut().unwrap(); let __entry = __map.entry(__key).or_insert_with(|| Rc::new(RefCell::new(Some(String::new())))); let mut __value = __entry.borrow_mut(); * __value = Some((*__value.as_ref().unwrap()).clone() + &__rhs); };
    let mut score = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<f64>>>>::from([]))));
    { let __map_holder = score.clone(); let __key = "a".to_string(); let mut __map_guard = __map_holder.borrow_mut(); let __map = __map_guard.as_mut().unwrap(); let __entry = __map.entry(__key).or_insert_with(|| Rc::new(RefCell::new(Some(0.0)))); let mut __value = __entry.borrow_mut(); * __value = Some((*__value.as_ref().unwrap()).clone() + 1.0); }
    { let __map_holder = score.clone(); let __key = "a".to_string(); let __rhs = 1.5; let mut __map_guard = __map_holder.borrow_mut(); let __map = __map_guard.as_mut().unwrap(); let __entry = __map.entry(__key).or_insert_with(|| Rc::new(RefCell::new(Some(0.0)))); let mut __value = __entry.borrow_mut(); * __value = Some((*__value.as_ref().unwrap()).clone() + __rhs); };
    println!("{} {} {} {} {}", format!("{}", { let __formatted = format_nested_slice_values(&(*(*s.borrow().as_ref().unwrap()).cells.borrow().as_ref().unwrap()).clone()); __formatted }), format!("{}", format_slice_values(&(*(*s.borrow().as_ref().unwrap()).names.borrow().as_ref().unwrap()).clone())), format!("{}", format_slice(&words)), format!("{}", format_map(&tags)), format!("{}", format_map(&score)));
}