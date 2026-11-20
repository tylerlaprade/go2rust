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
// it shares, instead of to one Vec<T> per slice value (DEV-36). The mode is
// opt-in while the emission sites move over.
func goSliceModeEnabled() bool {
	return os.Getenv("GO2RUST_GOSLICE") == "1"
}

// goSliceImportNames are the names every module imports from the helper that
// defines GoSlice. Vec and vec! shadow the prelude's, so the slice types and
// literals the translator writes as Vec<T> and vec![..] are Go slices.
var goSliceImportNames = []string{"GoSlice", "GoSliceAppend", "GoSliceAppendBoxed", "GoSliceFromSlice", "GoSliceHeader", "Vec", "vec"}

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
		"String::from_utf8(self.clone())", "String::from_utf8(self.to_vec())",
		"String::from_utf8(out)", "String::from_utf8(out.to_vec())",
	).Replace(text)
}

func generateGoSliceHelper(out *strings.Builder) {
	out.WriteString(goSliceRuntime)
}

// goSliceRuntime is the Rust definition of GoSlice. The backing is an
// UnsafeCell so that every header can read and write the elements it shares
// with the others, as Go's slices do; Go programs that race on a slice race
// here too.
const goSliceRuntime = `/// A Go slice: an offset, a length and a capacity over a backing array that
/// every slice made from it shares.
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

#[macro_export]
macro_rules! __go_slice_vec {
    () => ($crate::GoSlice::new());
    ($elem:expr; $n:expr) => ($crate::GoSlice::from(std::vec::from_elem($elem, $n)));
    ($($x:expr),+ $(,)?) => ($crate::GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($x),+]))));
}
pub use __go_slice_vec as vec;

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

    pub fn with_capacity(cap: usize) -> Self {
        GoSlice { backing: std::sync::Arc::new(std::cell::UnsafeCell::new(std::vec::Vec::with_capacity(cap))), off: 0, len: 0, cap }
    }

    fn backing(&self) -> &std::vec::Vec<T> {
        unsafe { &*self.backing.get() }
    }

    #[allow(clippy::mut_from_ref)]
    fn backing_mut(&self) -> &mut std::vec::Vec<T> {
        unsafe { &mut *self.backing.get() }
    }

    pub fn as_slice(&self) -> &[T] {
        &self.backing()[self.off..self.off + self.len]
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        let (off, len) = (self.off, self.len);
        &mut self.backing_mut()[off..off + len]
    }

    /// Go's cap(s).
    pub fn capacity(&self) -> usize {
        self.cap
    }

    /// Whether two slices share their backing array.
    pub fn shares_backing_with(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.backing, &other.backing)
    }

    /// Go's s[lo:hi].
    pub fn reslice(&self, lo: usize, hi: usize) -> Self {
        if lo > hi || hi > self.cap {
            panic!("runtime error: slice bounds out of range [{}:{}] with capacity {}", lo, hi, self.cap);
        }
        if self.off + hi > self.backing().len() {
            panic!("slice [{}:{}] reaches past the elements GoSlice has stored", lo, hi);
        }
        GoSlice { backing: self.backing.clone(), off: self.off + lo, len: hi - lo, cap: self.cap - lo }
    }

    /// Go's s[lo:hi:max].
    pub fn reslice3(&self, lo: usize, hi: usize, max: usize) -> Self {
        if max > self.cap || hi > max {
            panic!("runtime error: slice bounds out of range [{}:{}:{}] with capacity {}", lo, hi, max, self.cap);
        }
        let mut s = self.reslice(lo, hi);
        s.cap = max - lo;
        s
    }

    /// Go's s = s[:n] when n is at most len(s).
    pub fn truncate(&mut self, n: usize) {
        if n < self.len {
            self.len = n;
        }
    }

    /// Go's s = s[:0].
    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn reserve(&mut self, _additional: usize) {}

    /// Keeps the elements f accepts, in order, at the front of the slice.
    pub fn retain<F: FnMut(&T) -> bool>(&mut self, mut f: F) {
        let mut kept = 0;
        {
            let window = self.as_mut_slice();
            for i in 0..window.len() {
                if f(&window[i]) {
                    window.swap(kept, i);
                    kept += 1;
                }
            }
        }
        self.len = kept;
    }

    /// Moves the slice's elements out when no other slice shares them.
    fn take_unique(&mut self) -> Option<std::vec::Vec<T>> {
        let (off, len) = (self.off, self.len);
        let backing = std::sync::Arc::get_mut(&mut self.backing)?;
        let mut elems = std::mem::take(backing.get_mut());
        elems.truncate(off + len);
        elems.drain(..off);
        Some(elems)
    }

    fn fresh(elems: std::vec::Vec<T>, cap: usize) -> Self {
        let len = elems.len();
        GoSlice { backing: std::sync::Arc::new(std::cell::UnsafeCell::new(elems)), off: 0, len, cap: cap.max(len) }
    }

    /// The capacity Go's append gives a slice that must grow to hold needed.
    fn grown_cap(&self, needed: usize) -> usize {
        let old = self.cap;
        let doubled = old + old;
        if needed > doubled {
            return needed;
        }
        if old < 256 {
            return doubled.max(1);
        }
        let mut cap = old;
        while cap < needed {
            cap += (cap + 3 * 256) / 4;
        }
        cap
    }

    /// Stores v at index len(s) of the backing array: the in-place half of
    /// Go's append.
    fn store_at_end(&mut self, v: T) {
        let at = self.off + self.len;
        let backing = self.backing_mut();
        if at < backing.len() {
            backing[at] = v;
        } else {
            backing.push(v);
        }
        self.len += 1;
    }
}

impl<T: Clone> GoSlice<T> {
    /// Removes and returns the last element: Go's v := s[len(s)-1];
    /// s = s[:len(s)-1].
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        let v = self.as_slice()[self.len - 1].clone();
        self.len -= 1;
        Some(v)
    }

    pub fn insert(&mut self, index: usize, v: T) {
        if index > self.len {
            panic!("insertion index (is {}) should be <= len (is {})", index, self.len);
        }
        GoSliceAppend::push(self, v);
        self.as_mut_slice()[index..].rotate_right(1);
    }

    pub fn remove(&mut self, index: usize) -> T {
        let v = self.as_slice()[index].clone();
        self.as_mut_slice()[index..].rotate_left(1);
        self.len -= 1;
        v
    }

    pub fn split_off(&mut self, at: usize) -> Self {
        let tail = Self::from(self.as_slice()[at..].to_vec());
        self.len = at;
        tail
    }

    pub fn drain<R: std::ops::RangeBounds<usize>>(&mut self, range: R) -> std::vec::IntoIter<T> {
        let mut elems = self.as_slice().to_vec();
        let drained: std::vec::Vec<T> = elems.drain(range).collect();
        let cap = self.cap;
        *self = Self::fresh(elems, cap);
        drained.into_iter()
    }

    pub fn resize(&mut self, n: usize, v: T) {
        if n <= self.len {
            self.len = n;
        } else {
            let extra = std::vec::from_elem(v, n - self.len);
            GoSliceAppend::extend_from_slice(self, &extra);
        }
    }

    pub fn resize_with<F: FnMut() -> T>(&mut self, n: usize, mut f: F) {
        if n <= self.len {
            self.len = n;
        } else {
            let extra: std::vec::Vec<T> = (self.len..n).map(|_| f()).collect();
            GoSliceAppend::extend_from_slice(self, &extra);
        }
    }

    pub fn into_std_vec(self) -> std::vec::Vec<T> {
        self.as_slice().to_vec()
    }

    pub fn into_boxed_slice(self) -> Box<[T]> {
        self.as_slice().to_vec().into_boxed_slice()
    }
}

/// Go's append for elements that can be copied: in place while there is
/// capacity, else into a new backing array.
pub trait GoSliceAppend<T> {
    /// s = append(s, v)
    fn push(&mut self, v: T);
    /// s = append(s, xs...)
    fn extend_from_slice(&mut self, xs: &[T]);
    /// s = append(s, xs...) from values
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I);
    /// append(s, v), leaving s as it was
    fn appended(&self, v: T) -> Self;
}

impl<T: Clone> GoSliceAppend<T> for GoSlice<T> {
    fn push(&mut self, v: T) {
        if self.len < self.cap {
            self.store_at_end(v);
            return;
        }
        let cap = self.grown_cap(self.len + 1);
        let mut elems = match self.take_unique() {
            Some(elems) => elems,
            None => self.as_slice().to_vec(),
        };
        elems.reserve(cap.saturating_sub(elems.len()));
        elems.push(v);
        *self = Self::fresh(elems, cap);
    }

    fn extend_from_slice(&mut self, xs: &[T]) {
        if self.len + xs.len() <= self.cap {
            for x in xs {
                self.store_at_end(x.clone());
            }
            return;
        }
        let cap = self.grown_cap(self.len + xs.len());
        let mut elems = match self.take_unique() {
            Some(elems) => elems,
            None => self.as_slice().to_vec(),
        };
        elems.reserve(cap.saturating_sub(elems.len()));
        elems.extend_from_slice(xs);
        *self = Self::fresh(elems, cap);
    }

    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let items: std::vec::Vec<T> = iter.into_iter().collect();
        GoSliceAppend::extend_from_slice(self, &items);
    }

    fn appended(&self, v: T) -> Self {
        let mut out = self.clone();
        GoSliceAppend::push(&mut out, v);
        out
    }
}

/// Go's append for interface values, which have no Clone. When the slice
/// is full and shares its backing array, the new element goes at the end of
/// that array instead of into a copy.
pub trait GoSliceAppendBoxed<T> {
    fn push(&mut self, v: T);
    fn extend<I: GoSliceBoxedValues<T>>(&mut self, values: I);
}

impl<T> GoSlice<T> {
    fn push_without_copy(&mut self, v: T) {
        if self.len < self.cap {
            self.store_at_end(v);
            return;
        }
        let cap = self.grown_cap(self.len + 1);
        if let Some(mut elems) = self.take_unique() {
            elems.reserve(cap.saturating_sub(elems.len()));
            elems.push(v);
            *self = Self::fresh(elems, cap);
            return;
        }
        self.store_at_end(v);
        self.cap = self.len;
    }
}

/// The values appended by append(s, xs...) to a slice of interface values.
/// They are always a slice built for the append, which nothing else holds.
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
            Some(elems) => elems,
            None => panic!("append of interface values held by another slice"),
        }
    }
}

impl GoSliceAppendBoxed<Box<dyn std::any::Any>> for GoSlice<Box<dyn std::any::Any>> {
    fn push(&mut self, v: Box<dyn std::any::Any>) {
        self.push_without_copy(v);
    }
    fn extend<I: GoSliceBoxedValues<Box<dyn std::any::Any>>>(&mut self, values: I) {
        for v in values.into_boxed_values() {
            self.push_without_copy(v);
        }
    }
}

impl GoSliceAppendBoxed<Box<dyn std::any::Any + Send + Sync>> for GoSlice<Box<dyn std::any::Any + Send + Sync>> {
    fn push(&mut self, v: Box<dyn std::any::Any + Send + Sync>) {
        self.push_without_copy(v);
    }
    fn extend<I: GoSliceBoxedValues<Box<dyn std::any::Any + Send + Sync>>>(&mut self, values: I) {
        for v in values.into_boxed_values() {
            self.push_without_copy(v);
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
    fn from(v: std::vec::Vec<T>) -> Self {
        let len = v.len();
        GoSlice { backing: std::sync::Arc::new(std::cell::UnsafeCell::new(v)), off: 0, len, cap: len }
    }
}

impl<T: Clone> From<&[T]> for GoSlice<T> {
    fn from(v: &[T]) -> Self {
        Self::from(v.to_vec())
    }
}

impl<T: Clone> From<&mut [T]> for GoSlice<T> {
    fn from(v: &mut [T]) -> Self {
        Self::from(v.to_vec())
    }
}

impl<T, const N: usize> From<[T; N]> for GoSlice<T> {
    fn from(v: [T; N]) -> Self {
        Self::from(std::vec::Vec::from(v))
    }
}

impl From<&str> for GoSlice<u8> {
    fn from(s: &str) -> Self {
        Self::from(s.as_bytes().to_vec())
    }
}

impl From<String> for GoSlice<u8> {
    fn from(s: String) -> Self {
        Self::from(s.into_bytes())
    }
}

impl<T: Clone> From<GoSlice<T>> for std::vec::Vec<T> {
    fn from(s: GoSlice<T>) -> Self {
        s.as_slice().to_vec()
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
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::from(iter.into_iter().collect::<std::vec::Vec<T>>())
    }
}

impl<T: Clone> IntoIterator for GoSlice<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    fn into_iter(mut self) -> Self::IntoIter {
        match self.take_unique() {
            Some(elems) => elems.into_iter(),
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_slice().fmt(f)
    }
}

/// Go's %v of a slice: the elements' %v between brackets.
impl<T: std::fmt::Display> std::fmt::Display for GoSlice<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        for (i, v) in self.as_slice().iter().enumerate() {
            if i > 0 {
                f.write_str(" ")?;
            }
            write!(f, "{}", v)?;
        }
        f.write_str("]")
    }
}

impl<T: PartialEq> PartialEq for GoSlice<T> {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T: PartialEq> PartialEq<[T]> for GoSlice<T> {
    fn eq(&self, other: &[T]) -> bool {
        self.as_slice() == other
    }
}

impl<T: PartialEq> PartialEq<std::vec::Vec<T>> for GoSlice<T> {
    fn eq(&self, other: &std::vec::Vec<T>) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T: Eq> Eq for GoSlice<T> {}

impl<T: PartialOrd> PartialOrd for GoSlice<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_slice().partial_cmp(other.as_slice())
    }
}

impl<T: Ord> Ord for GoSlice<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

impl<T: std::hash::Hash> std::hash::Hash for GoSlice<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state)
    }
}

impl std::io::Write for GoSlice<u8> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        GoSliceAppend::extend_from_slice(self, buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

`

// writeGoSliceParamHeaderCopies gives each slice parameter that the function
// assigns to, or takes the address of, a cell of its own holding a copy of
// the caller's header. Go passes a slice by its header, so `xs = nil` or
// `xs = append(xs, v)` in the callee must not reach the caller's variable;
// the elements stay shared through the backing array.
func writeGoSliceParamHeaderCopies(out *strings.Builder, fn *ast.FuncDecl, indent string) {
	if !goSliceModeEnabled() || fn == nil || fn.Type == nil || fn.Type.Params == nil || fn.Body == nil {
		return
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return
	}
	for _, field := range fn.Type.Params.List {
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
			if !funcBodyReassignsVar(fn.Body, obj, typeInfo) {
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
	if _, ok := goSliceLocalVariable(s.Lhs[0]); !ok {
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
	if _, ok := goSliceLocalVariable(expr); !ok {
		return false
	}
	out.WriteString("{ let __slice_header = GoSliceHeader::go_slice_header(&")
	TranspileExpressionContext(out, expr, LValue)
	out.WriteString("); ")
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
	if len(s.Rhs) == 1 && len(s.Lhs) == 1 && goSliceValueIsFresh(s.Rhs[0]) {
		return
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
