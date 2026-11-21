package main

import (
	"go/ast"
	"go/token"
	"go/types"
	"os"
	"strings"
)

// goSliceModeEnabled reports whether Go slices lower to GoSlice<T>, a header
// (offset, length, capacity) over a backing array that every slice made from
// it shares, instead of to one Vec<T> per slice value (DEV-36). It is the
// default; GO2RUST_GOSLICE=0 restores the old lowering while the last users
// of it move over.
func goSliceModeEnabled() bool {
	return os.Getenv("GO2RUST_GOSLICE") != "0"
}

// goSliceImportNames are the names every module imports from the helper that
// defines GoSlice. Vec and vec! shadow the prelude's, so the slice types and
// literals the translator writes as Vec<T> and vec![..] are Go slices.
var goSliceImportNames = []string{"GoSlice", "GoSliceAppendBoxed", "GoSliceFromSlice", "GoSliceHeader", "GoSliceView", "Vec"}

// goSliceVecMacro shadows the prelude's vec! with one that makes a GoSlice.
// A macro reached through an import, or defined in an include!d file, is
// ambiguous with the prelude's, so every generated file that is not
// include!d defines its own copy at the top: a macro_rules definition
// written in the file itself takes precedence over the prelude.
const goSliceVecMacro = `#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
`

// goSliceFromStdVec turns a Rust expression of the standard library's Vec
// into the Go slice value that the generated code expects.
func goSliceFromStdVec(expr string) string {
	if goSliceModeEnabled() {
		return "Vec::from(" + expr + ")"
	}
	return expr
}

// stdVecPath names the standard library's Vec, for scratch vectors that
// are not Go slices.
func stdVecPath() string {
	if goSliceModeEnabled() {
		return "std::vec::Vec"
	}
	return "Vec"
}

// sliceToVecMethod is the method that copies a Rust slice into a new Go
// slice value: to_vec, which in GoSlice mode would make the standard
// library's Vec, or to_go_slice.
func sliceToVecMethod() string {
	if goSliceModeEnabled() {
		return ".to_go_slice()"
	}
	return ".to_vec()"
}

// deferStackDecl declares a function's stack of deferred calls. The stack is
// not a Go slice, so in GoSlice mode it names the standard library's Vec.
func deferStackDecl() string {
	if goSliceModeEnabled() {
		return "let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();"
	}
	return "let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();"
}

// stringFromUTF8Open and stringFromUTF8Close surround a []byte value to make
// a string of it. String::from_utf8 takes the standard library's Vec.
func stringFromUTF8Open() string {
	if goSliceModeEnabled() {
		return "String::from_utf8(std::vec::Vec::from("
	}
	return "String::from_utf8("
}

func stringFromUTF8Close() string {
	if goSliceModeEnabled() {
		return ")).unwrap()"
	}
	return ").unwrap()"
}

// goSliceHelperText adapts the text of another helper to GoSlice mode, where
// Vec in that text is a GoSlice.
func goSliceHelperText(text string) string {
	if !goSliceModeEnabled() {
		return text
	}
	return strings.NewReplacer(
		"fn format_slice_values<T>(slice: &[T]) -> String\nwhere\n    T: Display,\n{\n    let formatted: Vec<String> = slice.iter()", "fn format_slice_values<T, C>(slice: &C) -> String\nwhere\n    C: GoSliceView<T> + ?Sized,\n    T: Display,\n{\n    let __view = slice.go_slice_view();\n    let formatted: std::vec::Vec<String> = __view.iter()",
		"fn format_nested_slice_values<T, Inner>(slice: &[Inner]) -> String\nwhere\n    Inner: AsRef<[T]>,\n    T: Display,\n{\n    let formatted: Vec<String> = slice.iter().map(|inner| format_slice_values(inner.as_ref())).collect();", "fn format_nested_slice_values<T, Inner, C>(slice: &C) -> String\nwhere\n    C: GoSliceView<Inner> + ?Sized,\n    Inner: GoSliceView<T>,\n    T: Display,\n{\n    let __view = slice.go_slice_view();\n    let formatted: std::vec::Vec<String> = __view.iter().map(|inner| { let __inner_view = inner.go_slice_view(); format_slice_values(__inner_view.as_ref()) }).collect();",
		"String::from_utf8(self.clone())", "String::from_utf8(self.to_vec())",
		"String::from_utf8(out)", "String::from_utf8(out.to_vec())",
		"GoValueClone for std::vec::Vec<T>", "GoValueClone for GoSlice<T>",
		"GoTypeParamFieldClone for std::vec::Vec<T>", "GoTypeParamFieldClone for GoSlice<T>",
		"let mut items: Vec<_>", "let mut items: std::vec::Vec<_>",
		"let formatted: Vec<String>", "let formatted: std::vec::Vec<String>",
		"C: AsRef<[T]>", "C: GoSliceView<T>",
		"C: AsRef<[Inner]>", "C: GoSliceView<Inner>",
		"Inner: AsRef<[T]>", "Inner: GoSliceView<T>",
		"C: AsRef<[Arc<Mutex<Option<T>>>]>", "C: GoSliceView<Arc<Mutex<Option<T>>>>",
		"C: AsRef<[Rc<RefCell<Option<T>>>]>", "C: GoSliceView<Rc<RefCell<Option<T>>>>",
		"Inner: AsRef<[Arc<Mutex<Option<T>>>]>", "Inner: GoSliceView<Arc<Mutex<Option<T>>>>",
		"Inner: AsRef<[Rc<RefCell<Option<T>>>]>", "Inner: GoSliceView<Rc<RefCell<Option<T>>>>",
		"C: AsRef<[Arc<Mutex<Option<Inner>>>]>", "C: GoSliceView<Arc<Mutex<Option<Inner>>>>",
		"C: AsRef<[Rc<RefCell<Option<Inner>>>]>", "C: GoSliceView<Rc<RefCell<Option<Inner>>>>",
		"s.as_ref().iter()", "s.go_slice_view().iter()",
		"s\n            .as_ref()\n            .iter()", "s\n            .go_slice_view()\n            .iter()",
		"format_nested_slice_values(s.as_ref())", "format_nested_slice_values(s.go_slice_view().as_ref())",
		"format_slice_values(inner.as_ref())", "format_slice_values(inner.go_slice_view().as_ref())",
		"format_slice_wrapped_values(inner.as_ref())", "format_slice_wrapped_values(inner.go_slice_view().as_ref())",
		"format_slice_wrapped_stringer_values(s.as_ref())", "{ let __view = s.go_slice_view(); format_slice_wrapped_stringer_values(__view.as_ref()) }",
		"format_slice_values(values.as_ref())", "format_slice_values(values.go_slice_view().as_ref())",
		"format_slice_wrapped_values(values.as_ref())", "format_slice_wrapped_values(values.go_slice_view().as_ref())",
		"self.__go_write_bytes(&bytes);", "{ let __bytes = bytes.as_slice(); self.__go_write_bytes(__bytes.as_ref()); }",
		"stdout.__go_write_bytes(&output.stdout);", "{ let __stdout = output.stdout.as_slice(); stdout.__go_write_bytes(__stdout.as_ref()); }",
		"stderr.__go_write_bytes(&output.stderr);", "{ let __stderr = output.stderr.as_slice(); stderr.__go_write_bytes(__stderr.as_ref()); }",
		"guard.as_ref().and_then(|values| values.get(self.index).cloned())", "guard.as_ref().map(|values| values.get_cloned(self.index))",
		"guard.as_ref().and_then(|values| values.get(value.index)).is_none()", "guard.as_ref().map(|values| value.index >= values.len()).unwrap_or(true)",
		"f(values.get_mut(slot.index).expect(\"nil pointer dereference\"))", "values.with_mut(slot.index, f)",
		"values[self.index] = value;", "values.set(self.index, value);",
	).Replace(text)
}

// goSliceStubText adapts the hand-written standard-library stubs to GoSlice
// mode: the bytes they make with std APIs become Go slices.
func goSliceStubText(text string) string {
	if !goSliceModeEnabled() {
		return text
	}
	return strings.NewReplacer(
		"self.into_bytes()", "Vec::from(self.into_bytes())",
		"self.as_bytes().to_vec()", "self.as_bytes().to_go_slice()",
		"self.to_vec()", "self.to_go_slice()",
		"value.as_bytes().to_vec()", "value.as_bytes().to_go_slice()",
		"std::sync::Mutex::new(data))", "std::sync::Mutex::new(Vec::from(data)))",
		"let bytes = value.into_bytes();", "let bytes = Vec::from(value.into_bytes());",
		"self.__go_write_bytes(&bytes);", "{ let __bytes = bytes.as_slice(); self.__go_write_bytes(__bytes.as_ref()); }",
		"stdout.__go_write_bytes(&output.stdout);", "{ let __stdout = output.stdout.as_slice(); stdout.__go_write_bytes(__stdout.as_ref()); }",
		"stderr.__go_write_bytes(&output.stderr);", "{ let __stderr = output.stderr.as_slice(); stderr.__go_write_bytes(__stderr.as_ref()); }",
		"target[..count].copy_from_slice(&data[start..start + count]);", "{ let mut __target = target.as_mut_slice(); let __data = data.as_slice(); __target[..count].copy_from_slice(&__data[start..start + count]); }",
	).Replace(text)
}

func generateGoSliceHelper(out *strings.Builder, includeBoxedImpls bool) {
	runtime := goSliceRuntime
	boxedImpls := ""
	if includeBoxedImpls {
		boxedImpls = goSliceBoxedImpl
		if NeedsConcurrentWrapper() {
			boxedImpls = goSliceBoxedConcurrentImpl
		}
	}
	out.WriteString(strings.Replace(runtime, "__GO_SLICE_BOXED_IMPLS__", boxedImpls, 1))
}

// goSliceRuntime is the Rust definition of GoSlice. Every header shares one
// backing-wide RwLock so references cannot outlive the guard that excludes a
// write through another header.
const goSliceRuntime = `/// A Go slice: an offset, a length and a capacity over a backing array that
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

__GO_SLICE_BOXED_IMPLS__

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

`

const goSliceBoxedImpl = `impl GoSliceBoxedValues<Box<dyn std::any::Any>> for GoSlice<Box<dyn std::any::Any>> {
    fn into_boxed_values(mut self) -> std::vec::Vec<Box<dyn std::any::Any>> {
        match self.take_unique() {
            Some(__elems) => __elems,
            None => self.as_slice().iter().map(|__value| go_any_clone(__value.as_ref())).collect(),
        }
    }
}

impl GoSlice<Box<dyn std::any::Any>> {
    fn ensure_boxed_capacity(&mut self, __additional: usize) {
        let __needed = self.len + __additional;
        if __needed <= self.cap {
            return;
        }
        let __cap = self.grown_cap(__needed);
        let mut __elems = match self.take_unique() {
            Some(__elems) => __elems,
            None => self.as_slice().iter().map(|__value| go_any_clone(__value.as_ref())).collect(),
        };
        __elems.reserve(__cap.saturating_sub(__elems.len()));
        *self = Self::fresh(__elems, __cap);
    }
}

impl GoSliceAppendBoxed<Box<dyn std::any::Any>> for GoSlice<Box<dyn std::any::Any>> {
    fn push(&mut self, __value: Box<dyn std::any::Any>) {
        self.ensure_boxed_capacity(1);
        self.store_at_end(__value);
    }
    fn extend<I: GoSliceBoxedValues<Box<dyn std::any::Any>>>(&mut self, __values: I) {
        let __values = __values.into_boxed_values();
        self.ensure_boxed_capacity(__values.len());
        for __value in __values {
            self.store_at_end(__value);
        }
    }
}
`

const goSliceBoxedConcurrentImpl = `impl GoSliceBoxedValues<Box<dyn std::any::Any + Send + Sync>> for GoSlice<Box<dyn std::any::Any + Send + Sync>> {
    fn into_boxed_values(mut self) -> std::vec::Vec<Box<dyn std::any::Any + Send + Sync>> {
        match self.take_unique() {
            Some(__elems) => __elems,
            None => self.as_slice().iter().map(|__value| go_any_clone(__value.as_ref())).collect(),
        }
    }
}

impl GoSlice<Box<dyn std::any::Any + Send + Sync>> {
    fn ensure_boxed_capacity(&mut self, __additional: usize) {
        let __needed = self.len + __additional;
        if __needed <= self.cap {
            return;
        }
        let __cap = self.grown_cap(__needed);
        let mut __elems = match self.take_unique() {
            Some(__elems) => __elems,
            None => self.as_slice().iter().map(|__value| go_any_clone(__value.as_ref())).collect(),
        };
        __elems.reserve(__cap.saturating_sub(__elems.len()));
        *self = Self::fresh(__elems, __cap);
    }
}

impl GoSliceAppendBoxed<Box<dyn std::any::Any + Send + Sync>> for GoSlice<Box<dyn std::any::Any + Send + Sync>> {
    fn push(&mut self, __value: Box<dyn std::any::Any + Send + Sync>) {
        self.ensure_boxed_capacity(1);
        self.store_at_end(__value);
    }
    fn extend<I: GoSliceBoxedValues<Box<dyn std::any::Any + Send + Sync>>>(&mut self, __values: I) {
        let __values = __values.into_boxed_values();
        self.ensure_boxed_capacity(__values.len());
        for __value in __values {
            self.store_at_end(__value);
        }
    }
}
`

// writeGoSliceParamHeaderCopies gives each slice parameter that the function
// assigns to, or takes the address of, a cell of its own holding a copy of
// the caller's header. Go passes a slice by its header, so `xs = nil` or
// `xs = append(xs, v)` in the callee must not reach the caller's variable;
// the elements stay shared through the backing array.
func writeGoSliceParamHeaderCopies(out *strings.Builder, fn *ast.FuncDecl, indent string) {
	if fn == nil {
		return
	}
	writeGoSliceParamHeaderCopiesFor(out, fn.Type, fn.Body, indent)
}

func writeGoSliceParamHeaderCopiesFor(out *strings.Builder, fnType *ast.FuncType, body *ast.BlockStmt, indent string) {
	if !goSliceModeEnabled() || fnType == nil || fnType.Params == nil || body == nil {
		return
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return
	}
	for _, field := range fnType.Params.List {
		rustType := GoTypeToRust(field.Type)
		if !strings.HasPrefix(rustType, GetOuterWrapperType()+"<"+GetInnerWrapperType()+"<Option<Vec<") {
			continue
		}
		for _, name := range field.Names {
			if name == nil || name.Name == "_" {
				continue
			}
			obj := typeInfo.info.Defs[name]
			if obj == nil {
				continue
			}
			if _, ok := types.Unalias(obj.Type()).(*types.Slice); !ok {
				continue
			}
			if !funcBodyReassignsVar(body, obj, typeInfo) {
				continue
			}
			rustName := RustLocalIdent(name.Name)
			out.WriteString(indent)
			out.WriteString("let mut ")
			out.WriteString(rustName)
			out.WriteString(" = ")
			out.WriteString(GetOuterWrapperType())
			out.WriteString("::new(")
			out.WriteString(GetInnerWrapperType())
			out.WriteString("::new((*")
			out.WriteString(rustName)
			WriteBorrowMethod(out, false)
			out.WriteString(").clone()));\n")
		}
	}
}

// funcBodyReassignsVar reports whether body assigns to obj or takes its
// address, anywhere including in function literals.
func funcBodyReassignsVar(body *ast.BlockStmt, obj types.Object, typeInfo *TypeInfo) bool {
	found := false
	isObj := func(expr ast.Expr) bool {
		ident, ok := unwrapParens(expr).(*ast.Ident)
		return ok && typeInfo.info.Uses[ident] == obj
	}
	ast.Inspect(body, func(node ast.Node) bool {
		if found {
			return false
		}
		switch n := node.(type) {
		case *ast.AssignStmt:
			for _, lhs := range n.Lhs {
				if isObj(lhs) {
					found = true
				}
			}
		case *ast.RangeStmt:
			if n.Tok == token.ASSIGN && (isObj(n.Key) || isObj(n.Value)) {
				found = true
			}
		case *ast.UnaryExpr:
			if n.Op == token.AND && isObj(n.X) {
				found = true
			}
		}
		return !found
	})
	return found
}

// goSliceLocalVariable reports whether expr names a local variable or
// parameter of an unnamed slice type held in the usual cell.
func goSliceLocalVariable(expr ast.Expr) (*ast.Ident, bool) {
	ident, ok := unwrapParens(expr).(*ast.Ident)
	if !ok || ident.Name == "_" {
		return nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || typeInfo.pkg == nil {
		return nil, false
	}
	obj, ok := typeInfo.info.Uses[ident].(*types.Var)
	if !ok || obj.Parent() == nil || obj.Parent() == typeInfo.pkg.Scope() || obj.IsField() {
		return nil, false
	}
	if _, ok := types.Unalias(obj.Type()).(*types.Slice); !ok {
		return nil, false
	}
	if _, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
		return nil, false
	}
	if info := lookupVarInfo(ident.Name); info != nil {
		if info.WrapLevel != WrapFull || info.PointerKind != PointerDefault || info.IsRef {
			return nil, false
		}
	}
	return ident, true
}

// writeGoSliceHeaderAssignment writes `xs = v` for a slice variable as a
// store of v's header into xs's own cell, and reports whether it did. The
// cell is never replaced by another one: a closure that captured xs holds
// the same cell, and the cell that v came from belongs to another variable,
// which must not see later assignments to xs.
func writeGoSliceHeaderAssignment(out *strings.Builder, s *ast.AssignStmt) bool {
	if !goSliceModeEnabled() || s.Tok != token.ASSIGN || len(s.Lhs) != 1 || len(s.Rhs) != 1 {
		return false
	}
	if _, ok := goSliceLocalVariable(s.Lhs[0]); !ok && !goSliceFieldSelfAppend(s) {
		return false
	}
	rhs := unwrapParens(s.Rhs[0])
	out.WriteString("{ let __slice_header = ")
	if ident, ok := rhs.(*ast.Ident); ok && ident.Name == "nil" {
		out.WriteString("None")
	} else if _, ok := goSliceLocalVariable(rhs); ok {
		// The variable's cell, so that a nil slice is read as nil.
		out.WriteString("GoSliceHeader::go_slice_header(&")
		TranspileExpressionContext(out, rhs, LValue)
		out.WriteString(")")
	} else {
		// The value is a cell, a header, or an Option of one, depending
		// on the expression; the trait reads the header from any of them.
		out.WriteString("GoSliceHeader::go_slice_header(&(")
		TranspileExpression(out, rhs)
		out.WriteString("))")
	}
	out.WriteString("; *")
	TranspileExpressionContext(out, s.Lhs[0], LValue)
	WriteBorrowMethod(out, true)
	out.WriteString(" = __slice_header; }")
	return true
}

// writeGoSliceFreshCellFromVariable writes a new cell holding a copy of the
// header of the slice variable expr, and reports whether expr is one. A
// slice stored in a struct field, a map or a result is a header of its own:
// later assignments to the variable must not reach it.
func writeGoSliceFreshCellFromVariable(out *strings.Builder, expr ast.Expr) bool {
	if !goSliceModeEnabled() {
		return false
	}
	if _, ok := goSliceLocalVariable(expr); ok {
		out.WriteString("{ let __slice_header = GoSliceHeader::go_slice_header(&")
		TranspileExpressionContext(out, expr, LValue)
		out.WriteString("); ")
	} else if goSliceStructField(expr) {
		// The field's cell, so that a nil slice is read as nil.
		out.WriteString("{ let __slice_header = GoSliceHeader::go_slice_header(&(")
		TranspileExpressionContext(out, expr, LValue)
		out.WriteString(")); ")
	} else {
		return false
	}
	out.WriteString(GetOuterWrapperType())
	out.WriteString("::new(")
	out.WriteString(GetInnerWrapperType())
	out.WriteString("::new(__slice_header)) }")
	return true
}

// writeGoSliceDefineShadows follows `xs := v` with a new cell for xs when v
// may be a cell that belongs to something else: the result of a call, which
// can be a struct field's cell, or a field read. Assignments to xs write
// into its cell, so it must be xs's own.
func writeGoSliceDefineShadows(out *strings.Builder, s *ast.AssignStmt, indent string) {
	if !goSliceModeEnabled() || s.Tok != token.DEFINE {
		return
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return
	}
	if len(s.Rhs) == 1 && len(s.Lhs) == 1 {
		if call, ok := unwrapParens(s.Rhs[0]).(*ast.CallExpr); ok && appendCallReturnsBareIndexedSlice(call) {
			// append(grid[i], ...) is a bare header; the variable is a cell.
			if ident, ok := s.Lhs[0].(*ast.Ident); ok && ident.Name != "_" {
				name := RustLocalIdent(ident.Name)
				out.WriteString("\n")
				out.WriteString(indent)
				out.WriteString("let mut ")
				out.WriteString(name)
				out.WriteString(" = ")
				WriteWrapperPrefix(out)
				out.WriteString(name)
				WriteWrapperSuffix(out)
				out.WriteString(";")
			}
			return
		}
		if goSliceValueIsFresh(s.Rhs[0]) {
			return
		}
	}
	if len(s.Rhs) == len(s.Lhs) && len(s.Lhs) > 1 {
		// Parallel definitions copy their values one by one.
		return
	}
	for _, lhs := range s.Lhs {
		ident, ok := lhs.(*ast.Ident)
		if !ok || ident.Name == "_" {
			continue
		}
		obj, ok := typeInfo.info.Defs[ident].(*types.Var)
		if !ok {
			continue
		}
		if _, ok := types.Unalias(obj.Type()).(*types.Slice); !ok {
			continue
		}
		if _, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
			continue
		}
		if info := lookupVarInfo(ident.Name); info == nil || info.WrapLevel != WrapFull || info.PointerKind != PointerDefault || info.IsRef {
			continue
		}
		name := RustLocalIdent(ident.Name)
		out.WriteString("\n")
		out.WriteString(indent)
		out.WriteString("let mut ")
		out.WriteString(name)
		out.WriteString(" = { let __slice_header = GoSliceHeader::go_slice_header(&")
		out.WriteString(name)
		out.WriteString("); ")
		out.WriteString(GetOuterWrapperType())
		out.WriteString("::new(")
		out.WriteString(GetInnerWrapperType())
		out.WriteString("::new(__slice_header)) };")
	}
}

// goSliceValueIsFresh reports whether the lowering of a slice-valued expr
// builds a cell that nothing else holds.
func goSliceValueIsFresh(expr ast.Expr) bool {
	switch e := unwrapParens(expr).(type) {
	case *ast.CompositeLit, *ast.SliceExpr, *ast.Ident, *ast.FuncLit:
		return true
	case *ast.CallExpr:
		if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.IsTypeConversion(e) {
			return true
		}
		if fn, ok := unwrapParens(e.Fun).(*ast.Ident); ok {
			if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.info != nil {
				if _, isBuiltin := typeInfo.info.Uses[fn].(*types.Builtin); isBuiltin {
					return true
				}
			}
		}
	}
	return false
}

// isUnnamedSliceType reports whether typ is a slice type with no name.
func isUnnamedSliceType(typ types.Type) bool {
	if typ == nil {
		return false
	}
	_, ok := types.Unalias(typ).(*types.Slice)
	return ok
}

// goSliceStructField reports whether expr selects a struct field of an
// unnamed slice type. A struct's Clone shares its field cells, so a method
// value or a pointer receiver can hold a second handle on the same field:
// the field's cell takes the new header rather than being replaced.
func goSliceStructField(expr ast.Expr) bool {
	sel, ok := unwrapParens(expr).(*ast.SelectorExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	selection, ok := typeInfo.info.Selections[sel]
	if !ok || selection.Kind() != types.FieldVal {
		return false
	}
	return isUnnamedSliceType(selection.Type())
}

// writeGoSliceTypeSwitchBindingCopy gives the variable of a type switch case
// for a slice type a cell of its own. The interface value holds the slice's
// cell, and the case variable is Go's copy of the header: an append to it
// must not reach the slice the interface was made from.
func writeGoSliceTypeSwitchBindingCopy(out *strings.Builder, typeInfo *TypeInfo, varName string, caseType ast.Expr) {
	if !goSliceModeEnabled() || typeInfo == nil || typeInfo.info == nil {
		return
	}
	if !isUnnamedSliceType(typeInfo.info.TypeOf(caseType)) {
		return
	}
	name := RustLocalIdent(varName)
	out.WriteString("        let mut ")
	out.WriteString(name)
	out.WriteString(" = { let __slice_header = GoSliceHeader::go_slice_header(&")
	out.WriteString(name)
	out.WriteString("); ")
	out.WriteString(GetOuterWrapperType())
	out.WriteString("::new(")
	out.WriteString(GetInnerWrapperType())
	out.WriteString("::new(__slice_header)) };\n")
}

// goSliceFieldSelfAppend reports whether s is `x.f = append(x.f, ...)` for a
// slice field f. That is the form in which a second handle on the struct (a
// method value, or a receiver copied out of its lock) must see the append,
// so the field's cell takes the new header. Other assignments to a field
// keep their own lowering.
func goSliceFieldSelfAppend(s *ast.AssignStmt) bool {
	if !goSliceStructField(s.Lhs[0]) {
		return false
	}
	call, ok := unwrapParens(s.Rhs[0]).(*ast.CallExpr)
	if !ok || len(call.Args) == 0 || !isBuiltinAppendCall(call) {
		return false
	}
	return types.ExprString(unwrapParens(call.Args[0])) == types.ExprString(unwrapParens(s.Lhs[0]))
}

// writeGoSliceElementHeaderFromVariable writes the header of the slice
// variable rhs for storing into an element of a slice or array of slices,
// which holds headers directly. The element shares the variable's backing
// array, as in Go.
func writeGoSliceElementHeaderFromVariable(out *strings.Builder, rhs ast.Expr, expected types.Type) bool {
	if !goSliceModeEnabled() || !isUnnamedSliceType(expected) {
		return false
	}
	if _, ok := goSliceLocalVariable(rhs); !ok {
		return false
	}
	out.WriteString("GoSliceHeader::go_slice_header(&")
	TranspileExpressionContext(out, rhs, LValue)
	out.WriteString(").unwrap_or_default()")
	return true
}

// stripGoSliceRuntime removes the GoSlice runtime and the vec! macro from
// generated code. Tests that look for what the translator wrote for a
// program use it, so that the fixed runtime text does not match their
// patterns.
func stripGoSliceRuntime(code string) string {
	code = strings.Replace(code, goSliceRuntime, "", -1)
	return strings.Replace(code, goSliceVecMacro, "", -1)
}
