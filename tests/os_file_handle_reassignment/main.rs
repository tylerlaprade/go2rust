#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use std::cell::{RefCell};
use std::error::Error as StdError;
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


#[derive(Debug, Clone, Default)]
pub struct fs_PathError {
    pub err: Rc<RefCell<Option<Rc<dyn StdError>>>>,
    pub op: Rc<RefCell<Option<String>>>,
    pub path: Rc<RefCell<Option<String>>>,
}

impl std::fmt::Display for fs_PathError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __op_guard = self.op.borrow();
        let __path_guard = self.path.borrow();
        let __err_guard = self.err.borrow();
        let __op = __op_guard.as_deref().unwrap_or("");
        let __path = __path_guard.as_deref().unwrap_or("");
        let __err = __err_guard.as_ref().map(|e| e.to_string()).unwrap_or_default();
        if __op.is_empty() {
            write!(f, "{}: {}", __path, __err)
        } else {
            write!(f, "{} {}: {}", __op, __path, __err)
        }
    }
}

impl std::error::Error for fs_PathError {}

impl fs_PathError {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some::<String>(self.to_string())))
    }
    pub fn unwrap(&self) -> Rc<RefCell<Option<Rc<dyn StdError>>>> {
        self.err.clone()
    }
}


#[derive(Debug, Clone)]
pub struct os_File {
    pub __go_data: std::sync::Arc<std::sync::Mutex<Vec<u8>>>,
    pub __go_closed: std::sync::Arc<std::sync::atomic::AtomicBool>,
    pub __go_wait_for_close: bool,
    /// How far Read has read __go_data; shared by every copy of the File.
    pub __go_read_pos: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    /// os.Stdin: Read reads the process's standard input.
    pub __go_stdin: bool,
    /// The file os.Create opened: reads and writes go to it, and Close
    /// closes it.
    pub __go_disk: std::sync::Arc<std::sync::Mutex<Option<std::fs::File>>>,
    /// The name the File was opened with, for its errors.
    pub __go_name: String,
}

impl Default for os_File {
    fn default() -> Self {
        Self {
            __go_data: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
            __go_closed: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            __go_wait_for_close: false,
            __go_read_pos: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            __go_stdin: false,
            __go_disk: std::sync::Arc::new(std::sync::Mutex::new(None)),
            __go_name: String::new(),
        }
    }
}

/// The end of a File's input until a source-translated io registers its
/// io.EOF (__go_register_io_eof), the value callers compare with.
#[derive(Debug)]
pub struct GoStubEOF;

impl std::fmt::Display for GoStubEOF {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "EOF")
    }
}

impl std::error::Error for GoStubEOF {}

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
        let _ = self.__go_write_bytes_result(data);
    }

    /// Writes data to the disk file os.Create opened, or else to the
    /// File's bytes. A closed File takes no writes, as in Go.
    pub fn __go_write_bytes_result(&self, data: &[u8]) -> std::io::Result<()> {
        if self.__go_closed.load(std::sync::atomic::Ordering::SeqCst) {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("write {}: file already closed", self.__go_name)));
        }
        if let Some(__disk) = self.__go_disk.lock().unwrap().as_mut() {
            return std::io::Write::write_all(__disk, data)
                .map_err(|__err| std::io::Error::new(__err.kind(), format!("write {}: {}", self.__go_name, __err)));
        }
        self.__go_data.lock().unwrap().extend_from_slice(data);
        Ok(())
    }

    pub fn __go_read_all(&self) -> Vec<u8> {
        self.__go_data.lock().unwrap().clone()
    }

    pub fn close(&self) -> Rc<RefCell<Option<Rc<dyn StdError>>>> {
        self.__go_closed.store(true, std::sync::atomic::Ordering::SeqCst);
        self.__go_disk.lock().unwrap().take();
        Rc::new(RefCell::new(None::<Rc<dyn StdError>>))
    }

    pub fn write<T0: 'static>(&self, arg0: T0) -> (i64, Rc<RefCell<Option<Rc<dyn StdError>>>>) {
        let bytes = if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Vec<u8>>() {
            v.clone()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Rc<RefCell<Option<Vec<u8>>>>>() {
            v.borrow().as_ref().cloned().unwrap_or_default()
        } else {
            Vec::new()
        };
        let n = bytes.len() as i64;
        if let Err(__err) = self.__go_write_bytes_result(&bytes) {
            let n = 0i64;
            return (n as i64, Rc::new(RefCell::new(Some::<Rc<dyn StdError>>(std::rc::Rc::new(__err) as std::rc::Rc<dyn std::error::Error>))));
        }
        (n as i64, Rc::new(RefCell::new(None::<Rc<dyn StdError>>)))
    }

    pub fn write_string<T0: 'static>(&self, arg0: T0) -> (i64, Rc<RefCell<Option<Rc<dyn StdError>>>>) {
        let value = if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<String>() {
            v.clone()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<&str>() {
            (*v).to_string()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Rc<RefCell<Option<String>>>>() {
            v.borrow().as_ref().cloned().unwrap_or_default()
        } else {
            String::new()
        };
        let bytes = Vec::from(value.into_bytes());
        let n = bytes.len() as i64;
        if let Err(__err) = self.__go_write_bytes_result(&bytes) {
            let n = 0i64;
            return (n as i64, Rc::new(RefCell::new(Some::<Rc<dyn StdError>>(std::rc::Rc::new(__err) as std::rc::Rc<dyn std::error::Error>))));
        }
        (n as i64, Rc::new(RefCell::new(None::<Rc<dyn StdError>>)))
    }

    pub fn read<T0: 'static>(&self, arg0: T0) -> (i64, Rc<RefCell<Option<Rc<dyn StdError>>>>) {
        // Go's Read: fill as much of p as the input has, or report the end
        // of the input with no bytes read.
        let (__n, __eof): (usize, bool) = if let Some(__buf) = (&arg0 as &dyn std::any::Any).downcast_ref::<Rc<RefCell<Option<Vec<u8>>>>>() {
            let mut __guard = __buf.borrow_mut();
            match __guard.as_mut() {
                Some(__dst) => {
                    let __dst = __dst.as_mut_slice();
                    if __dst.is_empty() {
                        (0, false)
                    } else if self.__go_stdin {
                        match std::io::Read::read(&mut std::io::stdin(), __dst) {
                            Ok(0) | Err(_) => (0, true),
                            Ok(__count) => (__count, false),
                        }
                    } else if let Some(__disk) = self.__go_disk.lock().unwrap().as_mut() {
                        match std::io::Read::read(__disk, __dst) {
                            Ok(0) | Err(_) => (0, true),
                            Ok(__count) => (__count, false),
                        }
                    } else {
                        let __data = self.__go_data.lock().unwrap();
                        let __pos = self.__go_read_pos.load(std::sync::atomic::Ordering::SeqCst).min(__data.len());
                        let __count = (__data.len() - __pos).min(__dst.len());
                        __dst[..__count].copy_from_slice(&__data[__pos..__pos + __count]);
                        self.__go_read_pos.store(__pos + __count, std::sync::atomic::Ordering::SeqCst);
                        (__count, __count == 0)
                    }
                }
                None => (0, false),
            }
        } else {
            (0, false)
        };
        if __eof {
            (0, __go_stub_eof())
        } else {
            let n = __n;
            (n as i64, Rc::new(RefCell::new(None::<Rc<dyn StdError>>)))
        }
    }

    pub fn read_at<T0: 'static, T1: 'static>(&self, arg0: T0, arg1: T1) -> (i64, Rc<RefCell<Option<Rc<dyn StdError>>>>) {
        let offset = if let Some(v) = (&arg1 as &dyn std::any::Any).downcast_ref::<i64>() {
            *v
        } else if let Some(v) = (&arg1 as &dyn std::any::Any).downcast_ref::<Rc<RefCell<Option<i64>>>>() {
            v.borrow().as_ref().copied().unwrap_or_default()
        } else {
            0
        };
        let mut n = 0i64;
        #[cfg(unix)]
        if offset >= 0 {
            if let Some(__disk) = self.__go_disk.lock().unwrap().as_ref() {
                // ReadAt reads at offset without moving the File's own
                // position, as pread does.
                if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Rc<RefCell<Option<Vec<u8>>>>>() {
                    let mut guard = v.borrow_mut();
                    if let Some(target) = guard.as_mut() {
                        let mut __done = 0usize;
                        while __done < target.len() {
                            match std::os::unix::fs::FileExt::read_at(__disk, &mut target[__done..], offset as u64 + __done as u64) {
                                Ok(0) | Err(_) => break,
                                Ok(__count) => __done += __count,
                            }
                        }
                        n = __done as i64;
                    }
                }
                return (n as i64, Rc::new(RefCell::new(None::<Rc<dyn StdError>>)));
            }
        }
        let data = self.__go_read_all();
        if offset >= 0 {
            let start = offset as usize;
            if start < data.len() {
                if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Rc<RefCell<Option<Vec<u8>>>>>() {
                    let mut guard = v.borrow_mut();
                    if let Some(target) = guard.as_mut() {
                        let count = std::cmp::min(target.len(), data.len() - start);
                        target[..count].copy_from_slice(&data[start..start + count]);
                        n = count as i64;
                    }
                }
            }
        }
        (n as i64, Rc::new(RefCell::new(None::<Rc<dyn StdError>>)))
    }
}

thread_local! {
    static __GO_STUB_IO_EOF: std::cell::RefCell<Option<Rc<dyn StdError>>> = std::cell::RefCell::new(None);
}

/// io's init hands over io.EOF's value.
pub fn __go_register_io_eof(eof: Option<Rc<dyn StdError>>) {
    if eof.is_some() {
        __GO_STUB_IO_EOF.with(|__slot| *__slot.borrow_mut() = eof);
    }
}

fn __go_stub_eof() -> Rc<RefCell<Option<Rc<dyn StdError>>>> {
    __GO_STUB_IO_EOF.with(|__slot| match __slot.borrow().as_ref() {
        Some(__eof) => Rc::new(RefCell::new(Some::<Rc<dyn StdError>>(__eof.clone()))),
        None => Rc::new(RefCell::new(Some::<Rc<dyn StdError>>(std::rc::Rc::new(GoStubEOF) as Rc<dyn StdError>))),
    })
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

    impl GoStringArg for Rc<RefCell<Option<String>>> {
        fn into_go_string(self) -> String {
            self.borrow().as_ref().cloned().unwrap_or_default()
        }
    }

    type GoError = Rc<RefCell<Option<std::rc::Rc<dyn std::error::Error>>>>;

    fn no_error() -> GoError {
        Rc::new(RefCell::new(None))
    }

    fn io_error(err: std::io::Error) -> GoError {
        Rc::new(RefCell::new(Some(std::rc::Rc::new(err) as std::rc::Rc<dyn std::error::Error>)))
    }

    fn path_error(op: &str, path: String, err: std::io::Error) -> GoError {
        let path_error = fs_PathError { err: Rc::new(RefCell::new(Some::<Rc<dyn StdError>>(std::rc::Rc::new(err) as std::rc::Rc<dyn std::error::Error>))), op: Rc::new(RefCell::new(Some::<String>(op.to_string()))), path: Rc::new(RefCell::new(Some::<String>(path))) };
        Rc::new(RefCell::new(Some(std::rc::Rc::new(path_error) as std::rc::Rc<dyn std::error::Error>)))
    }

    pub fn create<T0: GoStringArg>(_arg0: T0) -> (Rc<RefCell<Option<os_File>>>, Rc<RefCell<Option<Rc<dyn StdError>>>>) {
        let path = _arg0.into_go_string();
        match std::fs::OpenOptions::new().read(true).write(true).create(true).truncate(true).open(&path) {
            Ok(disk) => {
                let file = os_File { __go_disk: std::sync::Arc::new(std::sync::Mutex::new(Some(disk))), __go_name: path.clone(), ..Default::default() };
                (Rc::new(RefCell::new(Some::<os_File>(file))), no_error())
            }
            Err(err) => (Rc::new(RefCell::new(None::<os_File>)), path_error("open", path, err)),
        }
    }

    pub fn open<T0: GoStringArg>(_arg0: T0) -> (Rc<RefCell<Option<os_File>>>, Rc<RefCell<Option<Rc<dyn StdError>>>>) {
        let path = _arg0.into_go_string();
        match std::fs::read(&path) {
            Ok(data) => {
                let file = os_File { __go_data: std::sync::Arc::new(std::sync::Mutex::new(Vec::from(data))), __go_closed: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), __go_wait_for_close: false, ..Default::default() };
                (Rc::new(RefCell::new(Some::<os_File>(file))), no_error())
            }
            Err(err) => (Rc::new(RefCell::new(None::<os_File>)), path_error("open", path, err)),
        }
    }

    pub fn open_file<T0: GoStringArg, T1: 'static, T2>(_arg0: T0, _arg1: T1, _arg2: T2) -> (Rc<RefCell<Option<os_File>>>, Rc<RefCell<Option<Rc<dyn StdError>>>>) {
        let path = _arg0.into_go_string();
        let flag: i64 = if let Some(v) = (&_arg1 as &dyn std::any::Any).downcast_ref::<i64>() {
            *v
        } else if let Some(v) = (&_arg1 as &dyn std::any::Any).downcast_ref::<i32>() {
            *v as i64
        } else if let Some(v) = (&_arg1 as &dyn std::any::Any).downcast_ref::<Rc<RefCell<Option<i64>>>>() {
            v.borrow().as_ref().copied().unwrap_or_default()
        } else {
            0
        };
        let access = flag & 3;
        let mut options = std::fs::OpenOptions::new();
        options.read(access == 0 || access == 2).write(access == 1 || access == 2);
        if flag & 1024 != 0 { options.append(true); }
        if flag & 64 != 0 { if flag & 128 != 0 { options.create_new(true); } else { options.create(true); } }
        if flag & 512 != 0 { options.truncate(true); }
        match options.open(&path) {
            Ok(disk) => {
                let file = os_File { __go_disk: std::sync::Arc::new(std::sync::Mutex::new(Some(disk))), __go_name: path.clone(), ..Default::default() };
                (Rc::new(RefCell::new(Some::<os_File>(file))), no_error())
            }
            Err(err) => (Rc::new(RefCell::new(None::<os_File>)), path_error("open", path, err)),
        }
    }

    pub fn read_file<T0: GoStringArg>(_arg0: T0) -> (Rc<RefCell<Option<Vec<u8>>>>, Rc<RefCell<Option<Rc<dyn StdError>>>>) {
        let path = _arg0.into_go_string();
        match std::fs::read(&path) {
            Ok(data) => (Rc::new(RefCell::new(Some::<Vec<u8>>(Vec::from(data)))), no_error()),
            Err(err) => (Rc::new(RefCell::new(Some::<Vec<u8>>(Vec::from(Vec::new())))), io_error(err)),
        }
    }
}


fn main() {
    const path: &'static str = "go2rust-file-handle.txt";


    let (mut file, mut err) = os::create("go2rust-file-handle.txt".to_string());
    if { let __nil_result = (*err.borrow()).is_some(); __nil_result } {
        println!("{}", format!("{}", false));
        return;
    }
    { let (__tmp_0, __tmp_1) = (*file.borrow_mut().as_mut().unwrap()).write_string(Rc::new(RefCell::new(Some("first".to_string())))); let __moved_tmp_1 = { let mut __guard = __tmp_1.borrow_mut(); __guard.take() }; *err.borrow_mut() = __moved_tmp_1; };
    println!("{}", format!("{}", { let __nil_result = (*err.borrow()).is_none(); __nil_result } && { let __nil_result = (*(*file.borrow_mut().as_mut().unwrap()).close().borrow()).is_none(); __nil_result }));

    { let (__tmp_0, __tmp_1) = os::open("go2rust-file-handle.txt".to_string()); file = __tmp_0.clone(); let __moved_tmp_1 = { let mut __guard = __tmp_1.borrow_mut(); __guard.take() }; *err.borrow_mut() = __moved_tmp_1; };
    println!("{}", format!("{}", { let __nil_result = (*err.borrow()).is_none(); __nil_result } && { let __nil_result = (*(*file.borrow_mut().as_mut().unwrap()).close().borrow()).is_none(); __nil_result }));

    { let (__tmp_0, __tmp_1) = os::open_file("go2rust-file-handle.txt".to_string(), 1025, fs_FileMode(0o600 as u32)); file = __tmp_0.clone(); let __moved_tmp_1 = { let mut __guard = __tmp_1.borrow_mut(); __guard.take() }; *err.borrow_mut() = __moved_tmp_1; };
    if { let __nil_result = (*err.borrow()).is_some(); __nil_result } {
        println!("{}", format!("{}", false));
        return;
    }
    { let (__tmp_0, __tmp_1) = (*file.borrow_mut().as_mut().unwrap()).write_string(Rc::new(RefCell::new(Some("-second".to_string())))); let __moved_tmp_1 = { let mut __guard = __tmp_1.borrow_mut(); __guard.take() }; *err.borrow_mut() = __moved_tmp_1; };
    println!("{}", format!("{}", { let __nil_result = (*err.borrow()).is_none(); __nil_result } && { let __nil_result = (*(*file.borrow_mut().as_mut().unwrap()).close().borrow()).is_none(); __nil_result }));

    let (mut contents, __tmp_1) = os::read_file("go2rust-file-handle.txt".to_string()); let __moved_tmp_1 = { let mut __guard = __tmp_1.borrow_mut(); __guard.take() }; *err.borrow_mut() = __moved_tmp_1;;
    let mut contents = { let __slice_header = GoSliceHeader::go_slice_header(&contents); Rc::new(RefCell::new(__slice_header)) };
    println!("{} {}", format!("{}", (*Rc::new(RefCell::new(Some(String::from_utf8(std::vec::Vec::from((*contents.borrow().as_ref().unwrap()).clone())).unwrap()))).borrow().as_ref().unwrap())), format!("{}", { let __nil_result = (*err.borrow()).is_none(); __nil_result }));
    println!("{}", format!("{}", { let __nil_result = (*{ let __path = path; match std::fs::remove_file(&__path) { Ok(()) => Rc::new(RefCell::new(None::<Rc<dyn StdError>>)), Err(e) => Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(e))))) } }.borrow()).is_none(); __nil_result }));
}