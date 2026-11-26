#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::cmp::Ord;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

fn format_map<K: Display + Ord + Clone, V>(map: &Arc<Mutex<Option<BTreeMap<K, Arc<Mutex<Option<V>>>>>>>) -> String
where
    V: Display,
{
    let guard = map.lock().unwrap();
    if let Some(ref m) = *guard {
        let mut items: Vec<_> = m.iter().collect();
        items.sort_by_key(|(k, _)| (*k).clone());

        let formatted: Vec<String> = items
            .into_iter()
            .map(|(k, v)| {
                let v_guard = v.lock().unwrap();
                if let Some(ref val) = *v_guard {
                    format!("{}:{}", go_display_value(k), go_display_value(val))
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

/// The Go func value a func-typed expression holds, or None for a nil func.
/// A copy of a func value is a clone of the same closure.
trait GoFuncValue {
    type Func: ?Sized;
    fn go_func_value(&self) -> Option<std::sync::Arc<Self::Func>>;
}

macro_rules! impl_go_func_value {
    ($(($($arg:ident),*)),* $(,)?) => { $(
        impl<R: 'static, $($arg: 'static),*> GoFuncValue for std::sync::Arc<dyn Fn($($arg),*) -> R + Send + Sync> {
            type Func = dyn Fn($($arg),*) -> R + Send + Sync;
            fn go_func_value(&self) -> Option<std::sync::Arc<Self::Func>> { Some(self.clone()) }
        }
        impl<R: 'static, $($arg: 'static),*> GoFuncValue for Option<std::sync::Arc<dyn Fn($($arg),*) -> R + Send + Sync>> {
            type Func = dyn Fn($($arg),*) -> R + Send + Sync;
            fn go_func_value(&self) -> Option<std::sync::Arc<Self::Func>> { self.clone() }
        }
        impl<R: 'static, $($arg: 'static),*> GoFuncValue for std::sync::Arc<std::sync::Mutex<Option<std::sync::Arc<dyn Fn($($arg),*) -> R + Send + Sync>>>> {
            type Func = dyn Fn($($arg),*) -> R + Send + Sync;
            fn go_func_value(&self) -> Option<std::sync::Arc<Self::Func>> { self.lock().unwrap().clone() }
        }
    )* };
}
impl_go_func_value!((), (A0), (A0, A1), (A0, A1, A2), (A0, A1, A2, A3), (A0, A1, A2, A3, A4), (A0, A1, A2, A3, A4, A5), (A0, A1, A2, A3, A4, A5, A6), (A0, A1, A2, A3, A4, A5, A6, A7), (A0, A1, A2, A3, A4, A5, A6, A7, A8), (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9), (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10), (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11));

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

#[derive(Clone)]
pub struct GoSliceElemPtr<T> {
    slice: Arc<Mutex<Option<Vec<T>>>>,
    index: usize,
    /// The element's identity when the pointer was made with the slice
    /// free to borrow; otherwise identity reads it from the slice.
    id: Option<(usize, usize)>,
}

pub struct GoSliceElemRef<T: Clone> {
    value: Option<T>,
}

pub struct GoSliceElemMutRef<T: Clone> {
    slice: Arc<Mutex<Option<Vec<T>>>>,
    index: usize,
    value: Option<T>,
}

pub trait GoArrayElemBacking<T: Clone + Send + Sync + 'static, const N: usize>: Send + Sync {
    fn borrow_at(&self, index: usize) -> Option<T>;
    fn assign_at(&self, index: usize, value: Option<T>);
    fn identity_at(&self, index: usize) -> (*const (), usize);
}

#[derive(Clone)]
pub struct GoDirectArrayElemBacking<T: Clone + Send + Sync + 'static, const N: usize> {
    array: Arc<Mutex<Option<[T; N]>>>,
}

impl<T: Clone + Send + Sync + 'static, const N: usize> GoArrayElemBacking<T, N> for GoDirectArrayElemBacking<T, N> {
    fn borrow_at(&self, index: usize) -> Option<T> {
        let guard = self.array.lock().unwrap();
        guard.as_ref().and_then(|values| values.get(index).cloned())
    }

    fn assign_at(&self, index: usize, value: Option<T>) {
        if let Some(value) = value {
            if let Some(values) = self.array.lock().unwrap().as_mut() {
                values[index] = value;
            }
        }
    }

    fn identity_at(&self, index: usize) -> (*const (), usize) {
        (Arc::as_ptr(&self.array) as *const (), index)
    }
}

#[derive(Clone)]
pub struct GoNestedArrayElemBacking<T: Clone + Send + Sync + 'static, const N: usize, const OUT: usize> {
    outer: Arc<Mutex<Option<[[T; N]; OUT]>>>,
    outer_index: usize,
}

impl<T: Clone + Send + Sync + 'static, const N: usize, const OUT: usize> GoArrayElemBacking<T, N> for GoNestedArrayElemBacking<T, N, OUT> {
    fn borrow_at(&self, index: usize) -> Option<T> {
        let guard = self.outer.lock().unwrap();
        guard.as_ref().and_then(|values| values.get(self.outer_index)).and_then(|inner| inner.get(index)).cloned()
    }

    fn assign_at(&self, index: usize, value: Option<T>) {
        if let Some(value) = value {
            if let Some(values) = self.outer.lock().unwrap().as_mut() {
                values[self.outer_index][index] = value;
            }
        }
    }

    fn identity_at(&self, index: usize) -> (*const (), usize) {
        (Arc::as_ptr(&self.outer) as *const (), self.outer_index.wrapping_mul(N).wrapping_add(index))
    }
}

#[derive(Clone)]
pub struct GoArrayElemFromElemBacking<T: Clone + Send + Sync + 'static, const N: usize, const OUT: usize> {
    parent: GoArrayElemPtr<[T; N], OUT>,
}

impl<T: Clone + Send + Sync + 'static, const N: usize, const OUT: usize> GoArrayElemBacking<T, N> for GoArrayElemFromElemBacking<T, N, OUT> {
    fn borrow_at(&self, index: usize) -> Option<T> {
        let inner = self.parent.borrow();
        inner.as_ref().and_then(|values| values.get(index).cloned())
    }

    fn assign_at(&self, index: usize, value: Option<T>) {
        if let Some(value) = value {
            let mut inner = self.parent.borrow_mut();
            if let Some(values) = inner.as_mut() {
                values[index] = value;
            }
        }
    }

    fn identity_at(&self, index: usize) -> (*const (), usize) {
        let (base, outer_index) = self.parent.identity();
        (base, outer_index.wrapping_mul(N).wrapping_add(index))
    }
}

#[derive(Clone)]
pub struct GoArrayElemFromGoPtrBacking<T: Clone + Send + Sync + 'static, const N: usize> {
    parent: GoPtr<[T; N]>,
}

impl<T: Clone + Send + Sync + 'static, const N: usize> GoArrayElemBacking<T, N> for GoArrayElemFromGoPtrBacking<T, N> {
    fn borrow_at(&self, index: usize) -> Option<T> {
        let inner = self.parent.borrow();
        inner.as_ref().and_then(|values| values.get(index).cloned())
    }

    fn assign_at(&self, index: usize, value: Option<T>) {
        if let Some(value) = value {
            self.parent.with_mut(|values| {
                values[index] = value;
            });
        }
    }

    fn identity_at(&self, index: usize) -> (*const (), usize) {
        (self.parent.addr() as *const (), index)
    }
}

#[derive(Clone)]
pub struct GoArrayElemPtr<T: Clone + Send + Sync + 'static, const N: usize> {
    backing: Arc<dyn GoArrayElemBacking<T, N> + Send + Sync>,
    index: usize,
}

pub struct GoArrayElemRef<T: Clone> {
    value: Option<T>,
}

pub struct GoArrayElemMutRef<T: Clone + Send + Sync + 'static, const N: usize> {
    backing: Arc<dyn GoArrayElemBacking<T, N> + Send + Sync>,
    index: usize,
    value: Option<T>,
}

impl<T> GoSliceElemPtr<T> {
    pub fn new(slice: Arc<Mutex<Option<Vec<T>>>>, index: usize) -> Self {
        // &s[i] panics, as indexing does, when i is not below len(s). A
        // slice already borrowed for the expression is not checked.
        if let Ok(__guard) = slice.try_lock() {
            let __len = __guard.as_ref().map_or(0, |__values| __values.len());
            if index >= __len {
                drop(__guard);
                panic!("index out of bounds: the len is {} but the index is {}", __len, index);
            }
            // Hold the backing array the slice has now, not the variable.
            let __header = (*__guard).clone();
            drop(__guard);
            // The element's address, taken now: nothing else can lock
            // the copy, and == need not wait for it.
            let __id = __header.as_ref().map(|__slice| ((__slice.as_slice().as_ptr() as usize).wrapping_add(index.wrapping_mul(std::mem::size_of::<T>())), 0));
            return GoSliceElemPtr { slice: std::sync::Arc::new(std::sync::Mutex::new(__header)), index, id: __id };
        }
        GoSliceElemPtr { slice, index, id: None }
    }

    /// The array and element the pointer names, for == between pointers.
    pub fn identity(&self) -> (usize, usize) {
        if let Some(__id) = self.id {
            return __id;
        }
        if let Ok(__guard) = self.slice.try_lock() {
            if let Some(__slice) = __guard.as_ref() {
                let __base = __slice.as_slice().as_ptr() as usize;
                return (__base.wrapping_add(self.index.wrapping_mul(std::mem::size_of::<T>())), 0);
            }
        }
        (Arc::as_ptr(&self.slice) as *const () as usize, self.index)
    }

    pub fn slice_handle(&self) -> Arc<Mutex<Option<Vec<T>>>> {
        self.slice.clone()
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

impl<T: Clone> GoSliceElemPtr<T> {
    pub fn borrow(&self) -> GoSliceElemRef<T> {
        let guard = self.slice.lock().unwrap();
        GoSliceElemRef {
            value: guard.as_ref().and_then(|values| values.get(self.index).cloned()),
        }
    }

    pub fn borrow_mut(&self) -> GoSliceElemMutRef<T> {
        let guard = self.slice.lock().unwrap();
        GoSliceElemMutRef {
            slice: self.slice.clone(),
            index: self.index,
            value: guard.as_ref().and_then(|values| values.get(self.index).cloned()),
        }
    }
}

impl<T: Clone + Send + Sync + 'static, const N: usize> GoArrayElemPtr<T, N> {
    pub fn new(array: Arc<Mutex<Option<[T; N]>>>, index: usize) -> Self {
        GoArrayElemPtr {
            backing: Arc::new(GoDirectArrayElemBacking { array }),
            index,
        }
    }

    pub fn nested<const OUT: usize>(outer: Arc<Mutex<Option<[[T; N]; OUT]>>>, outer_index: usize, index: usize) -> Self {
        GoArrayElemPtr {
            backing: Arc::new(GoNestedArrayElemBacking { outer, outer_index }),
            index,
        }
    }

    pub fn from_array_elem<const OUT: usize>(parent: GoArrayElemPtr<[T; N], OUT>, index: usize) -> Self {
        GoArrayElemPtr {
            backing: Arc::new(GoArrayElemFromElemBacking { parent }),
            index,
        }
    }

    pub fn from_go_ptr(parent: GoPtr<[T; N]>, index: usize) -> Self {
        GoArrayElemPtr {
            backing: Arc::new(GoArrayElemFromGoPtrBacking { parent }),
            index,
        }
    }

    pub fn borrow(&self) -> GoArrayElemRef<T> {
        GoArrayElemRef {
            value: self.backing.borrow_at(self.index),
        }
    }

    pub fn borrow_mut(&self) -> GoArrayElemMutRef<T, N> {
        GoArrayElemMutRef {
            backing: self.backing.clone(),
            index: self.index,
            value: self.backing.borrow_at(self.index),
        }
    }

    /// The element as the guard of a cell, for code written against a
    /// Mutex-held value: sync/atomic's Load and Store on &a[i].
    #[allow(dead_code)]
    pub fn lock(&self) -> Result<GoArrayElemMutRef<T, N>, std::convert::Infallible> {
        Ok(self.borrow_mut())
    }

    pub fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut value = self.backing.borrow_at(self.index).expect("nil pointer dereference");
        let result = f(&mut value);
        self.backing.assign_at(self.index, Some(value));
        result
    }

    pub fn identity(&self) -> (*const (), usize) {
        self.backing.identity_at(self.index)
    }
}

impl<T: Clone> std::ops::Deref for GoSliceElemRef<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Clone> std::ops::Deref for GoSliceElemMutRef<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Clone> std::ops::DerefMut for GoSliceElemMutRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T: Clone> Drop for GoSliceElemMutRef<T> {
    fn drop(&mut self) {
        if let Some(value) = self.value.clone() {
            if let Some(values) = self.slice.lock().unwrap().as_mut() {
                values[self.index] = value;
            }
        }
    }
}

impl<T: Clone> std::ops::Deref for GoArrayElemRef<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Clone + Send + Sync + 'static, const N: usize> std::ops::Deref for GoArrayElemMutRef<T, N> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Clone + Send + Sync + 'static, const N: usize> std::ops::DerefMut for GoArrayElemMutRef<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T: Clone + Send + Sync + 'static, const N: usize> Drop for GoArrayElemMutRef<T, N> {
    fn drop(&mut self) {
        self.backing.assign_at(self.index, self.value.clone());
    }
}

pub trait GoArrayElemPtrDyn<T: Send + Sync + 'static>: Send + Sync {
    fn borrow_dyn(&self) -> Option<T>;
    fn assign_dyn(&self, value: Option<T>);
    fn with_mut_dyn(&self, f: &mut dyn FnMut(&mut T));
    fn identity_dyn(&self) -> (*const (), usize);
}

pub struct GoForeignArrayElemPtrDyn<T: Send + Sync + 'static> {
    borrow: Arc<dyn Fn() -> Option<T> + Send + Sync>,
    assign: Arc<dyn Fn(Option<T>) + Send + Sync>,
    with_mut: Arc<dyn Fn(&mut dyn FnMut(&mut T)) + Send + Sync>,
    identity: Arc<dyn Fn() -> (*const (), usize) + Send + Sync>,
}

impl<T: Send + Sync + 'static> GoArrayElemPtrDyn<T> for GoForeignArrayElemPtrDyn<T> {
    fn borrow_dyn(&self) -> Option<T> {
        (self.borrow)()
    }

    fn assign_dyn(&self, value: Option<T>) {
        (self.assign)(value)
    }

    fn with_mut_dyn(&self, f: &mut dyn FnMut(&mut T)) {
        (self.with_mut)(f)
    }

    fn identity_dyn(&self) -> (*const (), usize) {
        (self.identity)()
    }
}

impl<T: Clone + Send + Sync + 'static, const N: usize> GoArrayElemPtrDyn<T> for GoArrayElemPtr<T, N> {
    fn borrow_dyn(&self) -> Option<T> {
        (*self.borrow()).clone()
    }

    fn assign_dyn(&self, value: Option<T>) {
        *self.borrow_mut() = value;
    }

    fn with_mut_dyn(&self, f: &mut dyn FnMut(&mut T)) {
        self.with_mut(|value| f(value));
    }

    fn identity_dyn(&self) -> (*const (), usize) {
        self.identity()
    }
}

pub enum GoPtr<T: Send + Sync + 'static> {
    Nil,
    Raw(usize),
    Local(Arc<Mutex<Option<T>>>),
    SliceElem(GoSliceElemPtr<T>),
    ArrayElem(Arc<dyn GoArrayElemPtrDyn<T> + Send + Sync>),
}

impl<T: Send + Sync + 'static> Clone for GoPtr<T> {
    fn clone(&self) -> Self {
        match self {
            GoPtr::Nil => GoPtr::Nil,
            GoPtr::Raw(addr) => GoPtr::Raw(*addr),
            GoPtr::Local(value) => GoPtr::Local(value.clone()),
            GoPtr::SliceElem(value) => GoPtr::SliceElem(GoSliceElemPtr { slice: value.slice.clone(), index: value.index, id: value.id }),
            GoPtr::ArrayElem(value) => GoPtr::ArrayElem(value.clone()),
        }
    }
}

impl<T: Send + Sync + 'static> GoPtr<T> {
    pub fn nil() -> Self {
        GoPtr::Nil
    }

    pub fn raw(addr: usize) -> Self {
        if addr == 0 {
            GoPtr::Nil
        } else {
            GoPtr::Raw(addr)
        }
    }

    pub fn local(value: Arc<Mutex<Option<T>>>) -> Self {
        if value.lock().unwrap().is_none() {
            GoPtr::Nil
        } else {
            GoPtr::Local(value)
        }
    }

    pub fn slice_elem(value: GoSliceElemPtr<T>) -> Self {
        GoPtr::SliceElem(value)
    }

    pub fn slice_elem_opt(value: Option<GoSliceElemPtr<T>>) -> Self {
        match value {
            Some(value) => GoPtr::SliceElem(value),
            None => GoPtr::Nil,
        }
    }

    pub fn array_elem<const N: usize>(value: GoArrayElemPtr<T, N>) -> Self
    where
        T: Clone,
    {
        GoPtr::ArrayElem(Arc::new(value))
    }

    pub fn array_elem_opt<const N: usize>(value: Option<GoArrayElemPtr<T, N>>) -> Self
    where
        T: Clone,
    {
        match value {
            Some(value) => GoPtr::ArrayElem(Arc::new(value)),
            None => GoPtr::Nil,
        }
    }

    pub fn array_elem_foreign(
        borrow: Arc<dyn Fn() -> Option<T> + Send + Sync>,
        assign: Arc<dyn Fn(Option<T>) + Send + Sync>,
        with_mut: Arc<dyn Fn(&mut dyn FnMut(&mut T)) + Send + Sync>,
        identity: Arc<dyn Fn() -> (*const (), usize) + Send + Sync>,
    ) -> Self {
        GoPtr::ArrayElem(Arc::new(GoForeignArrayElemPtrDyn {
            borrow,
            assign,
            with_mut,
            identity,
        }))
    }

    pub fn is_nil(&self) -> bool {
        match self {
            GoPtr::Nil => true,
            GoPtr::Raw(addr) => *addr == 0,
            GoPtr::Local(value) => value.lock().unwrap().is_none(),
            GoPtr::SliceElem(value) => {
                let guard = value.slice.lock().unwrap();
                guard.as_ref().and_then(|values| values.get(value.index)).is_none()
            }
            GoPtr::ArrayElem(value) => value.identity_dyn().0.is_null(),
        }
    }

    pub fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        match self {
            GoPtr::Nil => panic!("nil pointer dereference"),
            GoPtr::Raw(_) => panic!("raw unsafe pointer mutable borrow requires unsafe pointee support"),
            GoPtr::Local(slot) => {
                let mut guard = slot.lock().unwrap();
                f(guard.as_mut().unwrap())
            }
            GoPtr::SliceElem(slot) => {
                let mut guard = slot.slice.lock().unwrap();
                let values = guard.as_mut().expect("nil pointer dereference");
                f(values.get_mut(slot.index).expect("nil pointer dereference"))
            }
            GoPtr::ArrayElem(slot) => {
                let mut result = None;
                let mut callback = Some(f);
                slot.with_mut_dyn(&mut |value| {
                    let f = callback.take().expect("array element pointer mutable borrow called twice");
                    result = Some(f(value));
                });
                result.expect("nil pointer dereference")
            }
        }
    }

    pub fn ptr_eq(left: &Self, right: &Self) -> bool {
        match (left, right) {
            (GoPtr::Nil, GoPtr::Nil) => true,
            (GoPtr::Raw(_), _) | (_, GoPtr::Raw(_)) => left.addr() == right.addr(),
            (GoPtr::Local(left), GoPtr::Local(right)) => Arc::ptr_eq(left, right),
            (GoPtr::SliceElem(left), GoPtr::SliceElem(right)) => {
                left.identity() == right.identity()
            }
            (GoPtr::ArrayElem(left), GoPtr::ArrayElem(right)) => left.identity_dyn() == right.identity_dyn(),
            _ => false,
        }
    }

    pub fn addr(&self) -> usize {
        match self {
            GoPtr::Nil => 0,
            GoPtr::Raw(addr) => *addr,
            GoPtr::Local(value) => Arc::as_ptr(value) as usize,
            GoPtr::SliceElem(value) => {
                let (base, index) = value.identity();
                base.wrapping_add(index)
            }
            GoPtr::ArrayElem(value) => {
                let (base, index) = value.identity_dyn();
                (base as usize).wrapping_add(index)
            }
        }
    }
}

impl<T: Clone + Send + Sync + 'static> GoPtr<T> {
    /// The pointee, read the way the generated code reads a cell in a
    /// program with goroutines, ptr.lock().unwrap(), as borrow() is in one
    /// without: a []*T or map[K]*T element read through the cell path.
    pub fn lock(&self) -> Result<Option<T>, std::convert::Infallible> {
        Ok(self.borrow())
    }

    pub fn borrow(&self) -> Option<T> {
        match self {
            GoPtr::Nil => None,
            GoPtr::Raw(_) => panic!("raw unsafe pointer dereference requires unsafe pointee support"),
            GoPtr::Local(value) => (*value.lock().unwrap()).clone(),
            GoPtr::SliceElem(value) => (*value.borrow()).clone(),
            GoPtr::ArrayElem(value) => value.borrow_dyn(),
        }
    }

    pub fn assign(&self, value: Option<T>) {
        match self {
            GoPtr::Nil => panic!("nil pointer dereference"),
            GoPtr::Raw(_) => panic!("raw unsafe pointer assignment requires unsafe pointee support"),
            GoPtr::Local(slot) => *slot.lock().unwrap() = value,
            GoPtr::SliceElem(slot) => *slot.borrow_mut() = value,
            GoPtr::ArrayElem(slot) => slot.assign_dyn(value),
        }
    }
}

impl<T: Send + Sync + 'static> Default for GoPtr<T> {
    fn default() -> Self {
        GoPtr::Nil
    }
}

impl<T: Send + Sync + 'static> std::fmt::Debug for GoPtr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_nil() {
            write!(f, "<nil>")
        } else {
            write!(f, "<ptr>")
        }
    }
}

pub(crate) const __SLICES_UNKNOWN_HINT: i64 = 0;
pub(crate) const __SLICES_INCREASING_HINT: i64 = 1;
pub(crate) const __SLICES_DECREASING_HINT: i64 = 2;


#[derive(Debug)]
pub struct Celsius(pub Arc<Mutex<Option<f64>>>);

impl Clone for Celsius {
    fn clone(&self) -> Self {
        let __value = (*self.0.lock().unwrap()).clone();
        Celsius(Arc::new(Mutex::new(__value)))
    }
}

impl Default for Celsius {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(Some(Default::default()))))
    }
}

impl Display for Celsius {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", go_float_string(*self.0.lock().unwrap().as_ref().unwrap() as f64, false))
    }
}

impl PartialEq for Celsius {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<f64> for Celsius {
    fn eq(&self, other: &f64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Celsius {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<f64> for Celsius {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Celsius> for f64 {
    fn eq(&self, other: &Celsius) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<Celsius> for f64 {
    fn partial_cmp(&self, other: &Celsius) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for Celsius {
    type Output = Celsius;
    fn add(self, other: Self) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<f64> for Celsius {
    type Output = Celsius;
    fn add(self, other: f64) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Celsius> for f64 {
    type Output = Celsius;
    fn add(self, other: Celsius) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Celsius {
    type Output = Celsius;
    fn sub(self, other: Self) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<f64> for Celsius {
    type Output = Celsius;
    fn sub(self, other: f64) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Celsius> for f64 {
    type Output = Celsius;
    fn sub(self, other: Celsius) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Celsius {
    type Output = Celsius;
    fn mul(self, other: Self) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<f64> for Celsius {
    type Output = Celsius;
    fn mul(self, other: f64) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Celsius> for f64 {
    type Output = Celsius;
    fn mul(self, other: Celsius) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Celsius {
    type Output = Celsius;
    fn div(self, other: Self) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<f64> for Celsius {
    type Output = Celsius;
    fn div(self, other: f64) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Celsius> for f64 {
    type Output = Celsius;
    fn div(self, other: Celsius) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for Celsius {
    type Output = Celsius;
    fn neg(self) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}


#[derive(Debug, Clone, Default)]
pub struct IDs(pub Arc<Mutex<Option<Vec<i64>>>>);

impl Display for IDs {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice(&self.0))
    }
}


#[derive(Debug, Clone)]
pub struct Item {
    pub name: Arc<Mutex<Option<String>>>,
    pub price: Arc<Mutex<Option<i64>>>,
}

impl Item {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.price.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
            price: __go_clone_1_0,
        }
    }
}


impl Default for Item {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            name: __go_default_0_0,
            price: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.price.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}
impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.name.lock().unwrap().as_ref().cloned() }; let __right = { other.name.lock().unwrap().as_ref().cloned() }; __left == __right }
                && { let __left = { self.price.lock().unwrap().as_ref().cloned() }; let __right = { other.price.lock().unwrap().as_ref().cloned() }; __left == __right }
        )
    }
}


#[derive(Debug, Clone)]
pub struct Task {
    pub name: Arc<Mutex<Option<String>>>,
    pub pri: Arc<Mutex<Option<i64>>>,
}

impl Task {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.pri.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
            pri: __go_clone_1_0,
        }
    }
}


impl Default for Task {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            name: __go_default_0_0,
            pri: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.pri.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug)]
pub struct __slices_xorshift(pub Arc<Mutex<Option<u64>>>);

impl Clone for __slices_xorshift {
    fn clone(&self) -> Self {
        let __value = (*self.0.lock().unwrap()).clone();
        __slices_xorshift(Arc::new(Mutex::new(__value)))
    }
}

impl Default for __slices_xorshift {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(Some(Default::default()))))
    }
}

impl Display for __slices_xorshift {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for __slices_xorshift {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u64> for __slices_xorshift {
    fn eq(&self, other: &u64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for __slices_xorshift {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u64> for __slices_xorshift {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<__slices_xorshift> for u64 {
    fn eq(&self, other: &__slices_xorshift) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<__slices_xorshift> for u64 {
    fn partial_cmp(&self, other: &__slices_xorshift) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for __slices_xorshift {
    type Output = __slices_xorshift;
    fn add(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn add(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn add(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for __slices_xorshift {
    type Output = __slices_xorshift;
    fn sub(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn sub(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn sub(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for __slices_xorshift {
    type Output = __slices_xorshift;
    fn mul(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn mul(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn mul(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for __slices_xorshift {
    type Output = __slices_xorshift;
    fn div(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn div(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn div(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for __slices_xorshift {
    type Output = __slices_xorshift;
    fn rem(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn rem(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn rem(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for __slices_xorshift {
    type Output = __slices_xorshift;
    fn bitand(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn bitand(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn bitand(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for __slices_xorshift {
    type Output = __slices_xorshift;
    fn bitor(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn bitor(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn bitor(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for __slices_xorshift {
    type Output = __slices_xorshift;
    fn bitxor(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn bitxor(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn bitxor(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for __slices_xorshift {
    type Output = __slices_xorshift;
    fn not(self) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (*other.0.lock().unwrap().as_ref().unwrap()) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i32> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: i32) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i8> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: i8) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i16> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: i16) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: i64) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u32> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: u32) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u8> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: u8) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u16> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: u16) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<usize> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: usize) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (*other.0.lock().unwrap().as_ref().unwrap()) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i32> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: i32) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i8> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: i8) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i16> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: i16) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: i64) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u32> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: u32) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u8> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: u8) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u16> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: u16) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<usize> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: usize) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl Eq for __slices_xorshift {}

impl Ord for __slices_xorshift {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug)]
pub struct __slices_sortedHint(pub Arc<Mutex<Option<i64>>>);

impl Clone for __slices_sortedHint {
    fn clone(&self) -> Self {
        let __value = (*self.0.lock().unwrap()).clone();
        __slices_sortedHint(Arc::new(Mutex::new(__value)))
    }
}

impl Default for __slices_sortedHint {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(Some(Default::default()))))
    }
}

impl Display for __slices_sortedHint {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for __slices_sortedHint {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i64> for __slices_sortedHint {
    fn eq(&self, other: &i64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for __slices_sortedHint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i64> for __slices_sortedHint {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<__slices_sortedHint> for i64 {
    fn eq(&self, other: &__slices_sortedHint) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<__slices_sortedHint> for i64 {
    fn partial_cmp(&self, other: &__slices_sortedHint) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn add(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn add(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn add(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn sub(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn sub(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn sub(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn mul(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn mul(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn mul(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn div(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn div(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn div(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn neg(self) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn rem(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn rem(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn rem(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn bitand(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn bitand(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn bitand(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn bitor(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn bitor(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn bitor(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn bitxor(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn bitxor(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn bitxor(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn not(self) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (*other.0.lock().unwrap().as_ref().unwrap()) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i32> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: i32) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i8> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: i8) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i16> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: i16) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u32> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: u32) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u8> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: u8) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u16> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: u16) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: u64) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<usize> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: usize) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (*other.0.lock().unwrap().as_ref().unwrap()) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i32> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: i32) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i8> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: i8) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i16> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: i16) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u32> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: u32) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u8> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: u8) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u16> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: u16) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: u64) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<usize> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: usize) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl Eq for __slices_sortedHint {}

impl Ord for __slices_sortedHint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


impl __slices_xorshift {
    pub fn next(&mut self) -> u64 {
        { let __rhs = (*({ let __tmp_x = (*self).clone(); let __tmp_y = 13i32; __tmp_x << __tmp_y }).0.lock().unwrap().as_ref().unwrap()).clone(); let mut guard = self.0.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() ^ __rhs); };
        { let __rhs = (*({ let __tmp_x = (*self).clone(); let __tmp_y = 7i32; __tmp_x >> __tmp_y }).0.lock().unwrap().as_ref().unwrap()).clone(); let mut guard = self.0.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() ^ __rhs); };
        { let __rhs = (*({ let __tmp_x = (*self).clone(); let __tmp_y = 17i32; __tmp_x << __tmp_y }).0.lock().unwrap().as_ref().unwrap()).clone(); let mut guard = self.0.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() ^ __rhs); };
        (*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap())
    }
}

pub fn lower(s: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    let mut b = Arc::new(Mutex::new(Some(({ let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }).as_bytes().to_go_slice())));
    { let __range_holder = b.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for (i, c) in __range_values.iter().copied().enumerate() {
        if { let __tmp_x = (('A' as u8) as u8); let __tmp_y = c; __tmp_x <= __tmp_y } && { let __tmp_x = c; let __tmp_y = ('Z' as u8) as u8; __tmp_x <= __tmp_y } {
        { let __idx = (i) as usize; let __rhs = { let __tmp_x = { let __tmp_x = c; let __tmp_y = ('a' as u8) as u8; __tmp_x + __tmp_y }; let __tmp_y = ('A' as u8) as u8; __tmp_x - __tmp_y }; (*b.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    }
    } }
    return Arc::new(Mutex::new(Some(String::from_utf8(std::vec::Vec::from((*b.lock().unwrap().as_ref().unwrap()).clone())).unwrap())));
}

fn main() {
    ::cmp::__go_init_all();
    ::math_bits::__go_init_all();

    let mut s = Arc::new(Mutex::new(Some({ let mut v: Vec<i64> = (0..(10) as usize).map(|_| 0i64).collect(); v.truncate((5) as usize); v })));
    for i in 0..(({ let __range_holder = s.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        { let __idx = (i) as usize; let __rhs = { let __tmp_x = i as i64; let __tmp_y = 1; __tmp_x + __tmp_y }; (*s.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    }
    { let __slice_header = GoSliceHeader::go_slice_header(&(__slices__insert__slice_int__int(s.clone(), Arc::new(Mutex::new(Some(2))), Arc::new(Mutex::new(Some(vec![10, 11])))))); *s.lock().unwrap() = __slice_header; };
    println!("{} {} {}", format!("{}", format_slice(&s)), format!("{}", ({ let __len = (*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", (*s.lock().unwrap()).as_ref().map(|__v| __v.capacity()).unwrap_or(0)));
    { let __slice_header = GoSliceHeader::go_slice_header(&(__slices__insert__slice_int__int(s.clone(), Arc::new(Mutex::new(Some(1))), Arc::new(Mutex::new(Some({
        let __seq_holder = s.clone();
        let __seq_guard = __seq_holder.lock().unwrap();
        let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
        let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
        std::mem::drop(__seq_guard);
        let __low = (4) as usize;
        let __high = (6) as usize;
        let __max = __source_cap;
        __seq.reslice3_zeroed(__low, __high, __max)
    })))))); *s.lock().unwrap() = __slice_header; };
    println!("{}", format!("{}", format_slice(&s)));
    { let __slice_header = GoSliceHeader::go_slice_header(&(__slices__replace__slice_int__int(s.clone(), Arc::new(Mutex::new(Some(1))), Arc::new(Mutex::new(Some(3))), Arc::new(Mutex::new(Some(vec![7, 8, 9])))))); *s.lock().unwrap() = __slice_header; };
    println!("{}", format!("{}", format_slice(&s)));
    { let __slice_header = GoSliceHeader::go_slice_header(&(__slices__replace__slice_int__int(s.clone(), Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(2))), Arc::new(Mutex::new(Some({
        let __seq_holder = s.clone();
        let __seq_guard = __seq_holder.lock().unwrap();
        let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
        let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
        std::mem::drop(__seq_guard);
        let __low = (3) as usize;
        let __high = (7) as usize;
        let __max = __source_cap;
        __seq.reslice3_zeroed(__low, __high, __max)
    })))))); *s.lock().unwrap() = __slice_header; };
    println!("{}", format!("{}", format_slice(&s)));
    let mut ins = __slices__insert__slice_int__int(Arc::new(Mutex::new(Some(vec![1i64, 4i64]))), Arc::new(Mutex::new(Some(1))), Arc::new(Mutex::new(Some(vec![2, 3]))));
    let mut ins = { let __slice_header = GoSliceHeader::go_slice_header(&ins); Arc::new(Mutex::new(__slice_header)) };
    let mut del = __slices__delete__slice_int__int(ins.clone(), Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(2))));
    let mut del = { let __slice_header = GoSliceHeader::go_slice_header(&del); Arc::new(Mutex::new(__slice_header)) };
    println!("{} {}", format!("{}", format_slice(&ins)), format!("{}", format_slice(&del)));
    println!("{} {}", format!("{}", format_slice(&__slices__compact__slice_int__int(Arc::new(Mutex::new(Some(vec![1i64, 1i64, 2i64, 2i64, 2i64, 3i64, 1i64])))))), format!("{}", format_slice(&__slices__compact_func__slice_string__string(
        Arc::new(Mutex::new(Some(vec!["a".to_string(), "A".to_string(), "b".to_string()]))),
        Arc::new(Mutex::new(Some(Arc::new(move |a: Arc<Mutex<Option<String>>>, b: Arc<Mutex<Option<String>>>| -> bool {
        return { let __tmp_x = (*lower(Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*lower(Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y };
    }) as Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> bool + Send + Sync>)))
    ))));
    println!("{} {}", format!("{}", format_slice(&__slices__concat__slice_int__int(Arc::new(Mutex::new(Some(vec![{ let __slice_holder = Arc::new(Mutex::new(Some(vec![1i64]))).clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }, Vec::new(), { let __slice_holder = Arc::new(Mutex::new(Some(vec![2i64, 3i64]))).clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }])))))), format!("{}", format_slice(&__slices__repeat__slice_string__string(
        Arc::new(Mutex::new(Some(vec!["x".to_string(), "y".to_string()]))),
        Arc::new(Mutex::new(Some(3)))
    ))));
    let mut g = __slices__grow__slice_int__int(
        Arc::new(Mutex::new(Some(vec![1i64]))),
        Arc::new(Mutex::new(Some(10)))
    );
    let mut g = { let __slice_header = GoSliceHeader::go_slice_header(&g); Arc::new(Mutex::new(__slice_header)) };
    println!("{} {} {} {}", format!("{}", ({ let __len = (*g.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", { let __tmp_x = ((*g.lock().unwrap()).as_ref().map(|__v| __v.capacity()).unwrap_or(0) as i64); let __tmp_y = 11; __tmp_x >= __tmp_y }), format!("{}", ({ let __len = (*__slices__clip__slice_int__int(g.clone()).lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", (*__slices__clip__slice_int__int(g.clone()).lock().unwrap()).as_ref().map(|__v| __v.capacity()).unwrap_or(0)));
    let mut __rf_done_1: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    { let __f_holder = __slices__chunk__slice_int__int(
        Arc::new(Mutex::new(Some(vec![1i64, 2i64, 3i64, 4i64, 5i64]))),
        Arc::new(Mutex::new(Some(2)))
    ); let __f: Arc<dyn Fn(Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Vec<i64>>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)(Arc::new(Mutex::new(Some(Arc::new(move |__rf_param_1_0: Arc<Mutex<Option<Vec<i64>>>>| -> bool {
        let mut c = Arc::new(Mutex::new(__rf_param_1_0.lock().unwrap().clone()));
        print!("{}{}", format!("{}", format_slice(&c)), format!("{}", " ".to_string()));
        true
    }) as Arc<dyn Fn(Arc<Mutex<Option<Vec<i64>>>>) -> bool + Send + Sync>)))) };
    if { let __v = (*__rf_done_1.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return;
    }
    println!();
    let mut __rf_done_2: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    { let __f_holder = __slices__backward__slice_string__string(Arc::new(Mutex::new(Some(vec!["a".to_string(), "b".to_string(), "c".to_string()])))); let __f: Arc<dyn Fn(Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<String>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)(Arc::new(Mutex::new(Some(Arc::new(move |__rf_param_2_0: Arc<Mutex<Option<i64>>>, __rf_param_2_1: Arc<Mutex<Option<String>>>| -> bool {
        let (mut i, mut v) = ({ let __tuple_value = { let __owned = __rf_param_2_0.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }; __tuple_value }, { let __tuple_value = { let __owned = __rf_param_2_1.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }; __tuple_value });
        print!("{}{}{}", format!("{}", { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", " ".to_string()));
        true
    }) as Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<String>>>) -> bool + Send + Sync>)))) };
    if { let __v = (*__rf_done_2.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return;
    }
    println!();
    println!("{} {}", format!("{}", __slices__compare__slice_int__int(
        Arc::new(Mutex::new(Some(vec![1i64, 2i64]))),
        Arc::new(Mutex::new(Some(vec![1i64, 3i64])))
    )), format!("{}", __slices__compare__slice_string__string(
        Arc::new(Mutex::new(Some(vec!["b".to_string()]))),
        Arc::new(Mutex::new(Some(vec!["a".to_string(), "z".to_string()])))
    )));
    println!("{}", format!("{}", __slices__equal_func__slice_int__slice_string__int__string(
        Arc::new(Mutex::new(Some(vec![1i64, 2i64]))),
        Arc::new(Mutex::new(Some(vec!["1".to_string(), "2".to_string()]))),
        Arc::new(Mutex::new(Some(Arc::new(move |a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<String>>>| -> bool {
        return { let __tmp_x = (*Arc::new(Mutex::new(Some(format!("{}", format!("{}", { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }))))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = ({ let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }); __tmp_x == __tmp_y };
    }) as Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<String>>>) -> bool + Send + Sync>)))
    )));
    let mut items = Arc::new(Mutex::new(Some(vec![
        Item { name: Arc::new(Mutex::new(Some("pen".to_string()))), price: Arc::new(Mutex::new(Some(3))), ..Default::default() },
        Item { name: Arc::new(Mutex::new(Some("cup".to_string()))), price: Arc::new(Mutex::new(Some(5))), ..Default::default() },
        Item { name: Arc::new(Mutex::new(Some("ink".to_string()))), price: Arc::new(Mutex::new(Some(3))), ..Default::default() },
        Item { name: Arc::new(Mutex::new(Some("box".to_string()))), price: Arc::new(Mutex::new(Some(1))), ..Default::default() },
    ])));
    __slices__sort_stable_func__slice__item___item(
        items.clone(),
        Arc::new(Mutex::new(Some(Arc::new(move |a: Arc<Mutex<Option<Item>>>, b: Arc<Mutex<Option<Item>>>| -> i64 {
        __cmp__compare__int(Arc::new(Mutex::new(Some({ let __selector_holder = (*a.lock().unwrap().as_ref().unwrap()).price.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*b.lock().unwrap().as_ref().unwrap()).price.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }))))
    }) as Arc<dyn Fn(Arc<Mutex<Option<Item>>>, Arc<Mutex<Option<Item>>>) -> i64 + Send + Sync>)))
    );
    { let __go_print_arg_1 = Arc::new(Mutex::new(Some(__slices__is_sorted_func__slice__item___item(
        items.clone(),
        Arc::new(Mutex::new(Some(Arc::new(move |a: Arc<Mutex<Option<Item>>>, b: Arc<Mutex<Option<Item>>>| -> i64 {
        return { let __tmp_x = (*{ let __field = (*a.lock().unwrap().as_ref().unwrap()).price.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*b.lock().unwrap().as_ref().unwrap()).price.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y };
    }) as Arc<dyn Fn(Arc<Mutex<Option<Item>>>, Arc<Mutex<Option<Item>>>) -> i64 + Send + Sync>)))
    )))); let __go_print_value = println!("{} {}", format!("{}", format_slice(&items)), format!("{}", { let __v = (*__go_print_arg_1.lock().unwrap().as_ref().unwrap()).clone(); __v })); __go_print_value };
    let (mut n, mut ok) = __slices__binary_search_func__slice__item___item__int(
        items.clone(),
        Arc::new(Mutex::new(Some(5))),
        Arc::new(Mutex::new(Some(Arc::new(move |it: Arc<Mutex<Option<Item>>>, p: Arc<Mutex<Option<i64>>>| -> i64 {
        __cmp__compare__int(Arc::new(Mutex::new(Some({ let __selector_holder = (*it.lock().unwrap().as_ref().unwrap()).price.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }) as Arc<dyn Fn(Arc<Mutex<Option<Item>>>, Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync>)))
    );
    println!("{} {}", format!("{}", n), format!("{}", ok));
    println!("{} {}", format!("{}", (*__slices__min_func__slice__item___item(
        items.clone(),
        Arc::new(Mutex::new(Some(Arc::new(move |a: Arc<Mutex<Option<Item>>>, b: Arc<Mutex<Option<Item>>>| -> i64 {
        __cmp__compare__string(Arc::new(Mutex::new(Some({ let __selector_holder = (*a.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*b.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }))))
    }) as Arc<dyn Fn(Arc<Mutex<Option<Item>>>, Arc<Mutex<Option<Item>>>) -> i64 + Send + Sync>)))
    ).lock().unwrap().as_ref().unwrap())), format!("{}", (*__slices__max_func__slice__item___item(
        items.clone(),
        Arc::new(Mutex::new(Some(Arc::new(move |a: Arc<Mutex<Option<Item>>>, b: Arc<Mutex<Option<Item>>>| -> i64 {
        return { let __tmp_x = (*{ let __field = (*a.lock().unwrap().as_ref().unwrap()).price.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*b.lock().unwrap().as_ref().unwrap()).price.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y };
    }) as Arc<dyn Fn(Arc<Mutex<Option<Item>>>, Arc<Mutex<Option<Item>>>) -> i64 + Send + Sync>)))
    ).lock().unwrap().as_ref().unwrap())));
    { let __slice_header = GoSliceHeader::go_slice_header(&(__slices__delete_func__slice__item___item(
        items.clone(),
        Arc::new(Mutex::new(Some(Arc::new(move |it: Arc<Mutex<Option<Item>>>| -> bool {
        return { let __tmp_x = (*{ let __field = (*it.lock().unwrap().as_ref().unwrap()).price.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 3; __tmp_x == __tmp_y };
    }) as Arc<dyn Fn(Arc<Mutex<Option<Item>>>) -> bool + Send + Sync>)))
    ))); *items.lock().unwrap() = __slice_header; };
    { let __go_print_arg_2 = Arc::new(Mutex::new(Some(__slices__index__slice__item___item(
        items.clone(),
        Arc::new(Mutex::new(Some(Item { name: Arc::new(Mutex::new(Some("cup".to_string()))), price: Arc::new(Mutex::new(Some(5))), ..Default::default() })))
    )))); let __go_print_arg_3 = Arc::new(Mutex::new(Some(__slices__contains__slice__item___item(
        items.clone(),
        Arc::new(Mutex::new(Some(Item { name: Arc::new(Mutex::new(Some("pen".to_string()))), price: Arc::new(Mutex::new(Some(3))), ..Default::default() })))
    )))); let __go_print_value = println!("{} {} {}", format!("{}", format_slice(&items)), format!("{}", { let __v = (*__go_print_arg_2.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*__go_print_arg_3.lock().unwrap().as_ref().unwrap()).clone(); __v })); __go_print_value };
    let mut tasks = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Task { name: Arc::new(Mutex::new(Some("write".to_string()))), pri: Arc::new(Mutex::new(Some(2))), ..Default::default() }))), Arc::new(Mutex::new(Some(Task { name: Arc::new(Mutex::new(Some("test".to_string()))), pri: Arc::new(Mutex::new(Some(1))), ..Default::default() }))), Arc::new(Mutex::new(Some(Task { name: Arc::new(Mutex::new(Some("ship".to_string()))), pri: Arc::new(Mutex::new(Some(3))), ..Default::default() })))])));
    __slices__sort_func__slice_ptr__task__ptr__task(
        tasks.clone(),
        Arc::new(Mutex::new(Some(Arc::new(move |a: Arc<Mutex<Option<Task>>>, b: Arc<Mutex<Option<Task>>>| -> i64 {
        __cmp__compare__int(Arc::new(Mutex::new(Some({ let __selector_holder = (*a.lock().unwrap().as_ref().unwrap()).pri.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*b.lock().unwrap().as_ref().unwrap()).pri.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }))))
    }) as Arc<dyn Fn(Arc<Mutex<Option<Task>>>, Arc<Mutex<Option<Task>>>) -> i64 + Send + Sync>)))
    );
    { let __range_holder = tasks.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for t in __range_values.iter() {
        print!("{}{}", format!("{}", (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).name.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", " ".to_string()));
    } }
    println!();
    let mut i = __slices__index_func__slice_ptr__task__ptr__task(
        tasks.clone(),
        Arc::new(Mutex::new(Some(Arc::new(move |t: Arc<Mutex<Option<Task>>>| -> bool {
        return { let __tmp_x = { let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }; let __tmp_y = "ship".to_string(); __tmp_x == __tmp_y };
    }) as Arc<dyn Fn(Arc<Mutex<Option<Task>>>) -> bool + Send + Sync>)))
    );
    { let new_val = 10; *(*{ let __seq = { let __seq_holder = tasks.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap().as_ref().unwrap()).pri.lock().unwrap() = Some(new_val); };
    { let __go_print_arg_4 = Arc::new(Mutex::new(Some(__slices__contains_func__slice_ptr__task__ptr__task(
        tasks.clone(),
        Arc::new(Mutex::new(Some(Arc::new(move |t: Arc<Mutex<Option<Task>>>| -> bool {
        return { let __tmp_x = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).pri.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 10; __tmp_x == __tmp_y };
    }) as Arc<dyn Fn(Arc<Mutex<Option<Task>>>) -> bool + Send + Sync>)))
    )))); let __go_print_arg_5 = Arc::new(Mutex::new(Some(__slices__index__slice_ptr__task__ptr__task(
        tasks.clone(),
        { let __seq = { let __seq_holder = tasks.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }
    )))); let __go_print_value = println!("{} {} {} {}", format!("{}", i), format!("{}", (*(*{ let __seq = { let __seq_holder = tasks.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.lock().unwrap().as_ref().unwrap()).pri.lock().unwrap().as_ref().unwrap())), format!("{}", { let __v = (*__go_print_arg_4.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*__go_print_arg_5.lock().unwrap().as_ref().unwrap()).clone(); __v })); __go_print_value };
    let mut temps = Arc::new(Mutex::new(Some(vec![Celsius(Arc::new(Mutex::new(Some(21.5)))), Celsius(Arc::new(Mutex::new(Some((-3.0))))), Celsius(Arc::new(Mutex::new(Some(37.0)))), Celsius(Arc::new(Mutex::new(Some(0.0))))])));
    __slices__sort__slice__celsius___celsius(temps.clone());
    { let __go_print_arg_6 = __slices__max__slice__celsius___celsius(temps.clone()); let __go_print_arg_7 = __slices__min__slice__celsius___celsius(temps.clone()); let __go_print_arg_8 = Arc::new(Mutex::new(Some(__cmp__less___celsius(
        Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = temps.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }))),
        Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = temps.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() })))
    )))); let __go_print_value = println!("{} {} {} {}", format!("{}", format_slice(&temps)), format!("{}", { let __v = (*__go_print_arg_6.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*__go_print_arg_7.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*__go_print_arg_8.lock().unwrap().as_ref().unwrap()).clone(); __v })); __go_print_value };
    let mut ids = Arc::new(Mutex::new(Some(IDs(Arc::new(Mutex::new(Some(vec![9i64, 4i64, 7i64])))))));
    __slices__sort___i_ds__int(Arc::new(Mutex::new(Some({ let __arg_holder = ids.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let (mut idx, mut found) = __slices__binary_search___i_ds__int(Arc::new(Mutex::new(Some({ let __arg_holder = ids.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(7))));
    { let __go_print_arg_9 = __slices__clone___i_ds__int(Arc::new(Mutex::new(Some({ let __arg_holder = ids.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __go_print_value = print!("{} {} {} {}\n", format!("{}", format_slice(&{ let __named_slice = (*ids.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice })), format!("{}", "main.IDs"), format!("{}", idx), format!("{}", found)); __go_print_value };
    let mut zero = Arc::new(Mutex::new(Some(0.0)));
    let (mut nan, mut negInf) = ({ let __tuple_value = Arc::new(Mutex::new(Some({ let __bin_zero = ({ let __v = (*zero.lock().unwrap().as_ref().unwrap()).clone(); __v }); __bin_zero / __bin_zero }))); __tuple_value }, { let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = -1.0; let __tmp_y = { let __v = (*zero.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x / __tmp_y }))); __tuple_value });
    let mut fl = Arc::new(Mutex::new(Some(vec![3.0, ({ let __v = (*nan.lock().unwrap().as_ref().unwrap()).clone(); __v }), 1.0, ({ let __v = (*negInf.lock().unwrap().as_ref().unwrap()).clone(); __v })])));
    __slices__sort__slice_float64__float64(fl.clone());
    println!("{} {} {} {}", format!("{}", format_slice(&fl)), format!("{}", __cmp__compare__float64(Arc::new(Mutex::new(Some({ let __arg_holder = nan.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(1.0))))), format!("{}", __cmp__or__int(Arc::new(Mutex::new(Some(vec![0, 0, 4, 2]))))), format!("{}", (*__cmp__or__string(Arc::new(Mutex::new(Some(vec!["".to_string(), "x".to_string()])))).lock().unwrap().as_ref().unwrap())));
    let mut big = Arc::new(Mutex::new(Some(vec![0i64; (200) as usize])));
    for i in 0..(({ let __range_holder = big.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        { let __idx = (i) as usize; let __rhs = { let __tmp_x = ({ let __tmp_x = i as i64; let __tmp_y = 7919; __tmp_x * __tmp_y }); let __tmp_y = 200; __tmp_x % __tmp_y }; (*big.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    }
    __slices__sort__slice_int__int(big.clone());
    println!("{} {} {}", format!("{}", format_slice(&Arc::new(Mutex::new(Some({
        let __seq_holder = big.clone();
        let __seq_guard = __seq_holder.lock().unwrap();
        let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
        let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
        std::mem::drop(__seq_guard);
        let __low = 0;
        let __high = (5) as usize;
        let __max = __source_cap;
        __seq.reslice3_zeroed(__low, __high, __max)
    }))))), format!("{}", format_slice(&Arc::new(Mutex::new(Some({
        let __seq_holder = big.clone();
        let __seq_guard = __seq_holder.lock().unwrap();
        let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
        let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
        std::mem::drop(__seq_guard);
        let __low = (195) as usize;
        let __high = __seq.len();
        let __max = __source_cap;
        __seq.reslice3_zeroed(__low, __high, __max)
    }))))), format!("{}", __slices__is_sorted__slice_int__int(big.clone())));
    __slices__sort_func__slice_int__int(
        big.clone(),
        Arc::new(Mutex::new(Some(Arc::new(move |a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>| -> i64 {
        return { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y };
    }) as Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync>)))
    );
    println!("{}", format!("{}", format_slice(&Arc::new(Mutex::new(Some({
        let __seq_holder = big.clone();
        let __seq_guard = __seq_holder.lock().unwrap();
        let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
        let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
        std::mem::drop(__seq_guard);
        let __low = 0;
        let __high = (3) as usize;
        let __max = __source_cap;
        __seq.reslice3_zeroed(__low, __high, __max)
    }))))));
    __slices__reverse__slice_int__int(big.clone());
    println!("{} {}", format!("{}", format_slice(&Arc::new(Mutex::new(Some({
        let __seq_holder = big.clone();
        let __seq_guard = __seq_holder.lock().unwrap();
        let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
        let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
        std::mem::drop(__seq_guard);
        let __low = 0;
        let __high = (3) as usize;
        let __max = __source_cap;
        __seq.reslice3_zeroed(__low, __high, __max)
    }))))), format!("{}", ({ let __len = (*big.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })));
    let mut m = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some(1i64)))), ("b".to_string(), Arc::new(Mutex::new(Some(2i64)))), ("c".to_string(), Arc::new(Mutex::new(Some(3i64))))]))));
    println!("{} {}", format!("{}", format_slice(&__slices__sorted__string(__maps__keys__map_string_int__string__int(m.clone())))), format!("{}", format_slice(&__slices__sorted__int(__maps__values__map_string_int__string__int(m.clone())))));
    println!("{}", format!("{}", format_map(&__maps__collect__int__string(__slices__all__slice_string__string(Arc::new(Mutex::new(Some(vec!["x".to_string(), "y".to_string()]))))))));
    __maps__copy__map_string_int__map_string_int__string__int(
        m.clone(),
        Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("d".to_string(), Arc::new(Mutex::new(Some(4i64)))), ("a".to_string(), Arc::new(Mutex::new(Some(10i64))))]))))
    );
    __maps__delete_func__map_string_int__string__int(
        m.clone(),
        Arc::new(Mutex::new(Some(Arc::new(move |k: Arc<Mutex<Option<String>>>, v: Arc<Mutex<Option<i64>>>| -> bool {
        return { let __tmp_x = { let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x % __tmp_y }; let __tmp_y = 0; __tmp_x == __tmp_y };
    }) as Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<i64>>>) -> bool + Send + Sync>)))
    );
    { let __go_print_arg_10 = Arc::new(Mutex::new(Some(__maps__equal_func__map_string_int__map_string_int__string__int__int(
        m.clone(),
        Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("c".to_string(), Arc::new(Mutex::new(Some(3i64))))])))),
        Arc::new(Mutex::new(Some(Arc::new(move |a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>| -> bool {
        return { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y };
    }) as Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> bool + Send + Sync>)))
    )))); let __go_print_value = println!("{} {}", format!("{}", format_map(&m)), format!("{}", { let __v = (*__go_print_arg_10.lock().unwrap().as_ref().unwrap()).clone(); __v })); __go_print_value };
    __maps__insert__map_string_int__string__int(
        m.clone(),
        __maps__all__map_string_int__string__int(Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("z".to_string(), Arc::new(Mutex::new(Some(26i64))))])))))
    );
    let mut m2 = __maps__clone__map_string_int__string__int(m.clone());
    { let __map_key = "y".to_string(); let __map_value = Arc::new(Mutex::new(Some(25))); (*m2.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    { let __go_print_arg_11 = Arc::new(Mutex::new(Some(({ let __len = (*m.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64))); let __go_print_arg_12 = Arc::new(Mutex::new(Some(({ let __len = (*m2.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64))); let __go_print_arg_13 = Arc::new(Mutex::new(Some(__maps__equal__map_string_int__map_string_int__string__int(m.clone(), m2.clone())))); let __go_print_value = println!("{} {} {} {}", format!("{}", { let __v = (*__go_print_arg_11.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*__go_print_arg_12.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __map = { let __map_holder = m2.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"z".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", { let __v = (*__go_print_arg_13.lock().unwrap().as_ref().unwrap()).clone(); __v })); __go_print_value };
    let mut nilMap: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> = Arc::new(Mutex::new(None));
    println!("{} {}", format!("{}", { let __nil_result = (*__maps__clone__map_string_int__string__int(nilMap.clone()).lock().unwrap()).is_none(); __nil_result }), format!("{}", { let __nil_result = (*__slices__collect__string(__maps__keys__map_string_int__string__int(nilMap.clone())).lock().unwrap()).is_none(); __nil_result }));
    let mut done = GoChannel::<Vec<i64>>::new();
    let done_thread = done.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let mut w = Arc::new(Mutex::new(Some(vec![5i64, 3i64, 9i64])));;
        __slices__sort__slice_int__int(w.clone());;
        done_thread.send((*__slices__insert__slice_int__int(w.clone(), Arc::new(Mutex::new(Some(1))), Arc::new(Mutex::new(Some({
            let __seq_holder = w.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
            let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
            std::mem::drop(__seq_guard);
            let __low = (1) as usize;
            let __high = __seq.len();
            let __max = __source_cap;
            __seq.reslice3_zeroed(__low, __high, __max)
        })))).lock().unwrap().as_ref().unwrap()).clone());;;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    println!("{}", format!("{}", format_slice_values(&done.recv().unwrap_or_default())));
}

pub fn __slices__insert__slice_int__int(mut s: Arc<Mutex<Option<Vec<i64>>>>, i: Arc<Mutex<Option<i64>>>, v: Arc<Mutex<Option<Vec<i64>>>>) -> Arc<Mutex<Option<Vec<i64>>>> {
    let mut s = Arc::new(Mutex::new((*s.lock().unwrap()).clone()));
    let _ = Arc::new(Mutex::new(Some({
        let __seq_holder = s.clone();
        let __seq_guard = __seq_holder.lock().unwrap();
        let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
        let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
        std::mem::drop(__seq_guard);
        let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __high = __seq.len();
        let __max = __source_cap;
        __seq.reslice3_zeroed(__low, __high, __max)
    })));
    let mut m = Arc::new(Mutex::new(Some(({ let __len = (*v.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64)));
    if { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        return s.clone();
    }
    let mut n = Arc::new(Mutex::new(Some(({ let __len = (*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64)));
    if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        return { let __append_target = s.clone(); let __append_values: Vec<_> = { let __slice_holder = v.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned().collect(); let mut __appended: Vec<i64> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.extend(__append_values); let __append_was_nil = (*__append_target.lock().unwrap()).is_none(); Arc::new(Mutex::new(if __append_was_nil && __appended.len() == 0 { None } else { Some(__appended) })) };
    }
    if { let __tmp_x = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y } as i64); let __tmp_y = ((*s.lock().unwrap()).as_ref().map(|__v| __v.capacity()).unwrap_or(0) as i64); __tmp_x > __tmp_y } {
        let mut s2 = { let __append_target = Arc::new(Mutex::new(Some({
            let __seq_holder = s.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
            let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
            std::mem::drop(__seq_guard);
            let __low = 0;
            let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __max = __source_cap;
            __seq.reslice3_zeroed(__low, __high, __max)
        }))).clone(); let __append_values: Vec<_> = { let __slice_holder = Arc::new(Mutex::new(Some(vec![0i64; ({ let __tmp_x = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as usize]))).clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned().collect(); let mut __appended: Vec<i64> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.extend(__append_values); let __append_was_nil = (*__append_target.lock().unwrap()).is_none(); Arc::new(Mutex::new(if __append_was_nil && __appended.len() == 0 { None } else { Some(__appended) })) };
        {
            let _dst_start = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let _dst_len = (*s2.lock().unwrap().as_ref().unwrap()).len() - _dst_start;
            let _src = { let __copy_src_holder = v.clone(); let __copy_src_guard = __copy_src_holder.lock().unwrap(); __copy_src_guard.as_ref().map(|__s| __s.to_vec()).unwrap_or_default() };
            let _n = std::cmp::min(_dst_len, _src.len());
            for _i in 0.._n {
                (*s2.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
            }
            Arc::new(Mutex::new(Some(_n as i64)))
        };
        {
            let _dst_start = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
            let _dst_len = (*s2.lock().unwrap().as_ref().unwrap()).len() - _dst_start;
            let _src = (*Arc::new(Mutex::new(Some({
                let __seq_holder = s.clone();
                let __seq_guard = __seq_holder.lock().unwrap();
                let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
                let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
                std::mem::drop(__seq_guard);
                let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
                let __high = __seq.len();
                let __max = __source_cap;
                __seq.reslice3_zeroed(__low, __high, __max)
            }))).lock().unwrap().as_ref().unwrap()).to_vec();
            let _n = std::cmp::min(_dst_len, _src.len());
            for _i in 0.._n {
                (*s2.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
            }
            Arc::new(Mutex::new(Some(_n as i64)))
        };
        return s2.clone();
    }
    { let __slice_header = GoSliceHeader::go_slice_header(&(Arc::new(Mutex::new(Some({
        let __seq_holder = s.clone();
        let __seq_guard = __seq_holder.lock().unwrap();
        let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
        let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
        std::mem::drop(__seq_guard);
        let __low = 0;
        let __high = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
        let __max = __source_cap;
        __seq.reslice3_zeroed(__low, __high, __max)
    }))))); *s.lock().unwrap() = __slice_header; };
    if !__slices_overlaps__int(
        v.clone(),
        Arc::new(Mutex::new(Some({
            let __seq_holder = s.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
            let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
            std::mem::drop(__seq_guard);
            let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
            let __high = __seq.len();
            let __max = __source_cap;
            __seq.reslice3_zeroed(__low, __high, __max)
        })))
    ) {
        {
            let _dst_start = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
            let _dst_len = (*s.lock().unwrap().as_ref().unwrap()).len() - _dst_start;
            let _src = (*Arc::new(Mutex::new(Some({
                let __seq_holder = s.clone();
                let __seq_guard = __seq_holder.lock().unwrap();
                let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
                let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
                std::mem::drop(__seq_guard);
                let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
                let __high = __seq.len();
                let __max = __source_cap;
                __seq.reslice3_zeroed(__low, __high, __max)
            }))).lock().unwrap().as_ref().unwrap()).to_vec();
            let _n = std::cmp::min(_dst_len, _src.len());
            for _i in 0.._n {
                (*s.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
            }
            Arc::new(Mutex::new(Some(_n as i64)))
        };
        {
            let _dst_start = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let _dst_len = (*s.lock().unwrap().as_ref().unwrap()).len() - _dst_start;
            let _src = { let __copy_src_holder = v.clone(); let __copy_src_guard = __copy_src_holder.lock().unwrap(); __copy_src_guard.as_ref().map(|__s| __s.to_vec()).unwrap_or_default() };
            let _n = std::cmp::min(_dst_len, _src.len());
            for _i in 0.._n {
                (*s.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
            }
            Arc::new(Mutex::new(Some(_n as i64)))
        };
        return s.clone();
    }
    {
        let _dst_start = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let _dst_len = (*s.lock().unwrap().as_ref().unwrap()).len() - _dst_start;
        let _src = { let __copy_src_holder = v.clone(); let __copy_src_guard = __copy_src_holder.lock().unwrap(); __copy_src_guard.as_ref().map(|__s| __s.to_vec()).unwrap_or_default() };
        let _n = std::cmp::min(_dst_len, _src.len());
        for _i in 0.._n {
            (*s.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
        }
        Arc::new(Mutex::new(Some(_n as i64)))
    };
    __slices_rotate_right__int(
        Arc::new(Mutex::new(Some({
            let __seq_holder = s.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
            let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
            std::mem::drop(__seq_guard);
            let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __high = __seq.len();
            let __max = __source_cap;
            __seq.reslice3_zeroed(__low, __high, __max)
        }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = m.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
    );
    return s.clone();
}

pub fn __slices__replace__slice_int__int(s: Arc<Mutex<Option<Vec<i64>>>>, i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>, v: Arc<Mutex<Option<Vec<i64>>>>) -> Arc<Mutex<Option<Vec<i64>>>> {
    let _ = Arc::new(Mutex::new(Some({
        let __seq_holder = s.clone();
        let __seq_guard = __seq_holder.lock().unwrap();
        let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
        let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
        std::mem::drop(__seq_guard);
        let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __high = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __max = __source_cap;
        __seq.reslice3_zeroed(__low, __high, __max)
    })));
    if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        return __slices__insert__slice_int__int(s.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), v.clone());
    }
    if { let __tmp_x = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x == __tmp_y } {
        let mut s2 = { let __append_target = Arc::new(Mutex::new(Some({
            let __seq_holder = s.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
            let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
            std::mem::drop(__seq_guard);
            let __low = 0;
            let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __max = __source_cap;
            __seq.reslice3_zeroed(__low, __high, __max)
        }))).clone(); let __append_values: Vec<_> = { let __slice_holder = v.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned().collect(); let mut __appended: Vec<i64> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.extend(__append_values); let __append_was_nil = (*__append_target.lock().unwrap()).is_none(); Arc::new(Mutex::new(if __append_was_nil && __appended.len() == 0 { None } else { Some(__appended) })) };
        if { let __tmp_x = (({ let __len = (*s2.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); let __tmp_y = (({ let __len = (*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x < __tmp_y } {
        { let __clear_start = (({ let __len = (*s2.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })) as usize; let __clear_end = { let __clear_len_holder = s.clone(); let __clear_len_guard = __clear_len_holder.lock().unwrap(); __clear_len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; let __clear_holder = s.clone(); let mut __clear_guard = __clear_holder.lock().unwrap(); if let Some(__clear_seq) = __clear_guard.as_mut() { let __clear_cap = __clear_seq.capacity(); let mut __clear_window = __clear_seq.reslice3_zeroed(__clear_start, __clear_end, __clear_cap); for __clear_elem in __clear_window.as_mut_slice().iter_mut() { *__clear_elem = 0; } } };
    }
        return s2.clone();
    }
    let mut tot = Arc::new(Mutex::new(Some({
        let __tmp_x = ({ let __tmp_x = (({ let __len = (*Arc::new(Mutex::new(Some({
            let __seq_holder = s.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
            let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
            std::mem::drop(__seq_guard);
            let __low = 0;
            let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __max = __source_cap;
            __seq.reslice3_zeroed(__low, __high, __max)
        }))).lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); let __tmp_y = (({ let __len = (*v.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x + __tmp_y } as i64);
        let __tmp_y = (({ let __len = (*Arc::new(Mutex::new(Some({
            let __seq_holder = s.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
            let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
            std::mem::drop(__seq_guard);
            let __low = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __high = __seq.len();
            let __max = __source_cap;
            __seq.reslice3_zeroed(__low, __high, __max)
        }))).lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64);
        __tmp_x + __tmp_y
    })));
    if { let __tmp_x = ({ let __v = (*tot.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = ((*s.lock().unwrap()).as_ref().map(|__v| __v.capacity()).unwrap_or(0) as i64); __tmp_x > __tmp_y } {
        let mut s2 = { let __append_target = Arc::new(Mutex::new(Some({
            let __seq_holder = s.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
            let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
            std::mem::drop(__seq_guard);
            let __low = 0;
            let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __max = __source_cap;
            __seq.reslice3_zeroed(__low, __high, __max)
        }))).clone(); let __append_values: Vec<_> = { let __slice_holder = Arc::new(Mutex::new(Some(vec![0i64; ({ let __tmp_x = { let __v = (*tot.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as usize]))).clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned().collect(); let mut __appended: Vec<i64> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.extend(__append_values); let __append_was_nil = (*__append_target.lock().unwrap()).is_none(); Arc::new(Mutex::new(if __append_was_nil && __appended.len() == 0 { None } else { Some(__appended) })) };
        {
            let _dst_start = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let _dst_len = (*s2.lock().unwrap().as_ref().unwrap()).len() - _dst_start;
            let _src = { let __copy_src_holder = v.clone(); let __copy_src_guard = __copy_src_holder.lock().unwrap(); __copy_src_guard.as_ref().map(|__s| __s.to_vec()).unwrap_or_default() };
            let _n = std::cmp::min(_dst_len, _src.len());
            for _i in 0.._n {
                (*s2.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
            }
            Arc::new(Mutex::new(Some(_n as i64)))
        };
        {
            let _dst_start = ({ let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*v.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x + __tmp_y }) as usize;
            let _dst_len = (*s2.lock().unwrap().as_ref().unwrap()).len() - _dst_start;
            let _src = (*Arc::new(Mutex::new(Some({
                let __seq_holder = s.clone();
                let __seq_guard = __seq_holder.lock().unwrap();
                let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
                let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
                std::mem::drop(__seq_guard);
                let __low = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
                let __high = __seq.len();
                let __max = __source_cap;
                __seq.reslice3_zeroed(__low, __high, __max)
            }))).lock().unwrap().as_ref().unwrap()).to_vec();
            let _n = std::cmp::min(_dst_len, _src.len());
            for _i in 0.._n {
                (*s2.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
            }
            Arc::new(Mutex::new(Some(_n as i64)))
        };
        return s2.clone();
    }
    let mut r = Arc::new(Mutex::new(Some({
        let __seq_holder = s.clone();
        let __seq_guard = __seq_holder.lock().unwrap();
        let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
        let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
        std::mem::drop(__seq_guard);
        let __low = 0;
        let __high = ({ let __v = (*tot.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __max = __source_cap;
        __seq.reslice3_zeroed(__low, __high, __max)
    })));
    if { let __tmp_x = ({ let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*v.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x + __tmp_y } as i64); let __tmp_y = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); __tmp_x <= __tmp_y } {
        {
            let _dst_start = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let _dst_len = (*r.lock().unwrap().as_ref().unwrap()).len() - _dst_start;
            let _src = { let __copy_src_holder = v.clone(); let __copy_src_guard = __copy_src_holder.lock().unwrap(); __copy_src_guard.as_ref().map(|__s| __s.to_vec()).unwrap_or_default() };
            let _n = std::cmp::min(_dst_len, _src.len());
            for _i in 0.._n {
                (*r.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
            }
            Arc::new(Mutex::new(Some(_n as i64)))
        };
        {
            let _dst_start = ({ let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*v.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x + __tmp_y }) as usize;
            let _dst_len = (*r.lock().unwrap().as_ref().unwrap()).len() - _dst_start;
            let _src = (*Arc::new(Mutex::new(Some({
                let __seq_holder = s.clone();
                let __seq_guard = __seq_holder.lock().unwrap();
                let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
                let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
                std::mem::drop(__seq_guard);
                let __low = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
                let __high = __seq.len();
                let __max = __source_cap;
                __seq.reslice3_zeroed(__low, __high, __max)
            }))).lock().unwrap().as_ref().unwrap()).to_vec();
            let _n = std::cmp::min(_dst_len, _src.len());
            for _i in 0.._n {
                (*r.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
            }
            Arc::new(Mutex::new(Some(_n as i64)))
        };
        { let __clear_start = ({ let __v = (*tot.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __clear_end = { let __clear_len_holder = s.clone(); let __clear_len_guard = __clear_len_holder.lock().unwrap(); __clear_len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; let __clear_holder = s.clone(); let mut __clear_guard = __clear_holder.lock().unwrap(); if let Some(__clear_seq) = __clear_guard.as_mut() { let __clear_cap = __clear_seq.capacity(); let mut __clear_window = __clear_seq.reslice3_zeroed(__clear_start, __clear_end, __clear_cap); for __clear_elem in __clear_window.as_mut_slice().iter_mut() { *__clear_elem = 0; } } };
        return r.clone();
    }
    if !__slices_overlaps__int(
        Arc::new(Mutex::new(Some({
            let __seq_holder = r.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
            let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
            std::mem::drop(__seq_guard);
            let __low = ({ let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*v.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x + __tmp_y }) as usize;
            let __high = __seq.len();
            let __max = __source_cap;
            __seq.reslice3_zeroed(__low, __high, __max)
        }))),
        v.clone()
    ) {
        {
            let _dst_start = ({ let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*v.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x + __tmp_y }) as usize;
            let _dst_len = (*r.lock().unwrap().as_ref().unwrap()).len() - _dst_start;
            let _src = (*Arc::new(Mutex::new(Some({
                let __seq_holder = s.clone();
                let __seq_guard = __seq_holder.lock().unwrap();
                let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
                let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
                std::mem::drop(__seq_guard);
                let __low = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
                let __high = __seq.len();
                let __max = __source_cap;
                __seq.reslice3_zeroed(__low, __high, __max)
            }))).lock().unwrap().as_ref().unwrap()).to_vec();
            let _n = std::cmp::min(_dst_len, _src.len());
            for _i in 0.._n {
                (*r.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
            }
            Arc::new(Mutex::new(Some(_n as i64)))
        };
        {
            let _dst_start = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let _dst_len = (*r.lock().unwrap().as_ref().unwrap()).len() - _dst_start;
            let _src = { let __copy_src_holder = v.clone(); let __copy_src_guard = __copy_src_holder.lock().unwrap(); __copy_src_guard.as_ref().map(|__s| __s.to_vec()).unwrap_or_default() };
            let _n = std::cmp::min(_dst_len, _src.len());
            for _i in 0.._n {
                (*r.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
            }
            Arc::new(Mutex::new(Some(_n as i64)))
        };
        return r.clone();
    }
    let mut y = Arc::new(Mutex::new(Some({ let __tmp_x = (({ let __len = (*v.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); let __tmp_y = (({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as i64); __tmp_x - __tmp_y })));
    if !__slices_overlaps__int(
        Arc::new(Mutex::new(Some({
            let __seq_holder = r.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
            let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
            std::mem::drop(__seq_guard);
            let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __high = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __max = __source_cap;
            __seq.reslice3_zeroed(__low, __high, __max)
        }))),
        v.clone()
    ) {
        {
            let _dst_start = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let _dst_len = (({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize) - _dst_start;
            let _src = (*Arc::new(Mutex::new(Some({
                let __seq_holder = v.clone();
                let __seq_guard = __seq_holder.lock().unwrap();
                let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
                let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
                std::mem::drop(__seq_guard);
                let __low = ({ let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
                let __high = __seq.len();
                let __max = __source_cap;
                __seq.reslice3_zeroed(__low, __high, __max)
            }))).lock().unwrap().as_ref().unwrap()).to_vec();
            let _n = std::cmp::min(_dst_len, _src.len());
            for _i in 0.._n {
                (*r.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
            }
            Arc::new(Mutex::new(Some(_n as i64)))
        };
        {
            let _dst_start = (({ let __len = (*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })) as usize;
            let _dst_len = (*r.lock().unwrap().as_ref().unwrap()).len() - _dst_start;
            let _src = (*Arc::new(Mutex::new(Some({
                let __seq_holder = v.clone();
                let __seq_guard = __seq_holder.lock().unwrap();
                let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
                let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
                std::mem::drop(__seq_guard);
                let __low = 0;
                let __high = ({ let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
                let __max = __source_cap;
                __seq.reslice3_zeroed(__low, __high, __max)
            }))).lock().unwrap().as_ref().unwrap()).to_vec();
            let _n = std::cmp::min(_dst_len, _src.len());
            for _i in 0.._n {
                (*r.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
            }
            Arc::new(Mutex::new(Some(_n as i64)))
        };
        __slices_rotate_right__int(
            Arc::new(Mutex::new(Some({
                let __seq_holder = r.clone();
                let __seq_guard = __seq_holder.lock().unwrap();
                let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
                let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
                std::mem::drop(__seq_guard);
                let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
                let __high = __seq.len();
                let __max = __source_cap;
                __seq.reslice3_zeroed(__low, __high, __max)
            }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
        );
        return r.clone();
    }
    if !__slices_overlaps__int(
        Arc::new(Mutex::new(Some({
            let __seq_holder = r.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
            let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
            std::mem::drop(__seq_guard);
            let __low = (({ let __len = (*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })) as usize;
            let __high = __seq.len();
            let __max = __source_cap;
            __seq.reslice3_zeroed(__low, __high, __max)
        }))),
        v.clone()
    ) {
        {
            let _dst_start = (({ let __len = (*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })) as usize;
            let _dst_len = (*r.lock().unwrap().as_ref().unwrap()).len() - _dst_start;
            let _src = (*Arc::new(Mutex::new(Some({
                let __seq_holder = v.clone();
                let __seq_guard = __seq_holder.lock().unwrap();
                let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
                let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
                std::mem::drop(__seq_guard);
                let __low = 0;
                let __high = ({ let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
                let __max = __source_cap;
                __seq.reslice3_zeroed(__low, __high, __max)
            }))).lock().unwrap().as_ref().unwrap()).to_vec();
            let _n = std::cmp::min(_dst_len, _src.len());
            for _i in 0.._n {
                (*r.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
            }
            Arc::new(Mutex::new(Some(_n as i64)))
        };
        {
            let _dst_start = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let _dst_len = (({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize) - _dst_start;
            let _src = (*Arc::new(Mutex::new(Some({
                let __seq_holder = v.clone();
                let __seq_guard = __seq_holder.lock().unwrap();
                let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
                let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
                std::mem::drop(__seq_guard);
                let __low = ({ let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
                let __high = __seq.len();
                let __max = __source_cap;
                __seq.reslice3_zeroed(__low, __high, __max)
            }))).lock().unwrap().as_ref().unwrap()).to_vec();
            let _n = std::cmp::min(_dst_len, _src.len());
            for _i in 0.._n {
                (*r.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
            }
            Arc::new(Mutex::new(Some(_n as i64)))
        };
        __slices_rotate_right__int(
            Arc::new(Mutex::new(Some({
                let __seq_holder = r.clone();
                let __seq_guard = __seq_holder.lock().unwrap();
                let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
                let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
                std::mem::drop(__seq_guard);
                let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
                let __high = __seq.len();
                let __max = __source_cap;
                __seq.reslice3_zeroed(__low, __high, __max)
            }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
        );
        return r.clone();
    }
    let mut k = __slices_start_idx__int(
        v.clone(),
        Arc::new(Mutex::new(Some({
            let __seq_holder = s.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
            let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
            std::mem::drop(__seq_guard);
            let __low = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __high = __seq.len();
            let __max = __source_cap;
            __seq.reslice3_zeroed(__low, __high, __max)
        })))
    );
    {
        let _dst_start = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let _dst_len = (*r.lock().unwrap().as_ref().unwrap()).len() - _dst_start;
        let _src = { let __copy_src_holder = v.clone(); let __copy_src_guard = __copy_src_holder.lock().unwrap(); __copy_src_guard.as_ref().map(|__s| __s.to_vec()).unwrap_or_default() };
        let _n = std::cmp::min(_dst_len, _src.len());
        for _i in 0.._n {
            (*r.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
        }
        Arc::new(Mutex::new(Some(_n as i64)))
    };
    {
        let _dst_start = ({ let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*v.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x + __tmp_y }) as usize;
        let _dst_len = (*r.lock().unwrap().as_ref().unwrap()).len() - _dst_start;
        let _src = (*Arc::new(Mutex::new(Some({
            let __seq_holder = r.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
            let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
            std::mem::drop(__seq_guard);
            let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = k; __tmp_x + __tmp_y }) as usize;
            let __high = __seq.len();
            let __max = __source_cap;
            __seq.reslice3_zeroed(__low, __high, __max)
        }))).lock().unwrap().as_ref().unwrap()).to_vec();
        let _n = std::cmp::min(_dst_len, _src.len());
        for _i in 0.._n {
            (*r.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
        }
        Arc::new(Mutex::new(Some(_n as i64)))
    };
    return r.clone();
}

pub fn __slices__delete__slice_int__int(mut s: Arc<Mutex<Option<Vec<i64>>>>, i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<Vec<i64>>>> {
    let mut s = Arc::new(Mutex::new((*s.lock().unwrap()).clone()));
    let _ = Arc::new(Mutex::new(Some({
        let mut __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned };
        let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __high = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __max = (({ let __len = (*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })) as usize;
        __seq.reslice3_zeroed(__low, __high, __max)
    })));
    if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        return s.clone();
    }
    let mut oldlen = Arc::new(Mutex::new(Some(({ let __len = (*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64)));
    { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = Arc::new(Mutex::new(Some({
        let __seq_holder = s.clone();
        let __seq_guard = __seq_holder.lock().unwrap();
        let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
        let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
        std::mem::drop(__seq_guard);
        let __low = 0;
        let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __max = __source_cap;
        __seq.reslice3_zeroed(__low, __high, __max)
    }))).clone(); let __append_values: Vec<_> = { let __slice_holder = Arc::new(Mutex::new(Some({
        let __seq_holder = s.clone();
        let __seq_guard = __seq_holder.lock().unwrap();
        let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
        let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
        std::mem::drop(__seq_guard);
        let __low = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __high = __seq.len();
        let __max = __source_cap;
        __seq.reslice3_zeroed(__low, __high, __max)
    }))).clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned().collect(); let mut __appended: Vec<i64> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.extend(__append_values); let __append_was_nil = (*__append_target.lock().unwrap()).is_none(); Arc::new(Mutex::new(if __append_was_nil && __appended.len() == 0 { None } else { Some(__appended) })) })); *s.lock().unwrap() = __slice_header; };
    { let __clear_start = (({ let __len = (*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })) as usize; let __clear_end = ({ let __v = (*oldlen.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __clear_holder = s.clone(); let mut __clear_guard = __clear_holder.lock().unwrap(); if let Some(__clear_seq) = __clear_guard.as_mut() { let __clear_cap = __clear_seq.capacity(); let mut __clear_window = __clear_seq.reslice3_zeroed(__clear_start, __clear_end, __clear_cap); for __clear_elem in __clear_window.as_mut_slice().iter_mut() { *__clear_elem = 0; } } };
    return s.clone();
}

pub fn __slices__compact__slice_int__int(s: Arc<Mutex<Option<Vec<i64>>>>) -> Arc<Mutex<Option<Vec<i64>>>> {
    if { let __tmp_x = (({ let __len = (*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); let __tmp_y = 2; __tmp_x < __tmp_y } {
        return s.clone();
    }
    let mut k = Arc::new(Mutex::new(Some(1i64)));
    while { let __tmp_x = ({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x < __tmp_y } {
        if {
            let __tmp_x = { let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_y = { let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            __tmp_x == __tmp_y
        } {
        let mut s2 = Arc::new(Mutex::new(Some({
            let __seq_holder = s.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
            let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
            std::mem::drop(__seq_guard);
            let __low = ({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __high = __seq.len();
            let __max = __source_cap;
            __seq.reslice3_zeroed(__low, __high, __max)
        })));
        let mut k2 = Arc::new(Mutex::new(Some(1i64)));
    while { let __tmp_x = ({ let __v = (*k2.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*s2.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x < __tmp_y } {
        if {
            let __tmp_x = { let __seq = { let __seq_holder = s2.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*k2.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_y = { let __seq = { let __seq_holder = s2.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*k2.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            __tmp_x != __tmp_y
        } {
        { let __idx = ({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __rhs = { let __seq = { let __seq_holder = s2.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*k2.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; (*s.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
        { let mut guard = k.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let mut guard = k2.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let __clear_start = ({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __clear_end = { let __clear_len_holder = s.clone(); let __clear_len_guard = __clear_len_holder.lock().unwrap(); __clear_len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; let __clear_holder = s.clone(); let mut __clear_guard = __clear_holder.lock().unwrap(); if let Some(__clear_seq) = __clear_guard.as_mut() { let __clear_cap = __clear_seq.capacity(); let mut __clear_window = __clear_seq.reslice3_zeroed(__clear_start, __clear_end, __clear_cap); for __clear_elem in __clear_window.as_mut_slice().iter_mut() { *__clear_elem = 0; } } };
        return Arc::new(Mutex::new(Some({
            let __seq_holder = s.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
            let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
            std::mem::drop(__seq_guard);
            let __low = 0;
            let __high = ({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __max = __source_cap;
            __seq.reslice3_zeroed(__low, __high, __max)
        })));
    }
        { let mut guard = k.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return s.clone();
}

pub fn __slices__compact_func__slice_string__string(s: Arc<Mutex<Option<Vec<String>>>>, eq: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> bool + Send + Sync>>>>) -> Arc<Mutex<Option<Vec<String>>>> {
    if { let __tmp_x = (({ let __len = (*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); let __tmp_y = 2; __tmp_x < __tmp_y } {
        return s.clone();
    }
    let mut k = Arc::new(Mutex::new(Some(1i64)));
    while { let __tmp_x = ({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x < __tmp_y } {
        if {
            let __f: Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> bool + Send + Sync> = eq.lock().unwrap().as_ref().unwrap().clone();
            (__f)(
                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value })))
            )
        } {
        let mut s2 = Arc::new(Mutex::new(Some({
            let __seq_holder = s.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
            let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
            std::mem::drop(__seq_guard);
            let __low = ({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __high = __seq.len();
            let __max = __source_cap;
            __seq.reslice3_zeroed(__low, __high, __max)
        })));
        let mut k2 = Arc::new(Mutex::new(Some(1i64)));
    while { let __tmp_x = ({ let __v = (*k2.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*s2.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x < __tmp_y } {
        if !{
            let __f: Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> bool + Send + Sync> = eq.lock().unwrap().as_ref().unwrap().clone();
            (__f)(
                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = s2.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*k2.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = s2.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*k2.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value })))
            )
        } {
        { let __idx = ({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __rhs = { let __seq = { let __seq_holder = s2.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*k2.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; (*s.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
        { let mut guard = k.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let mut guard = k2.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let __clear_start = ({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __clear_end = { let __clear_len_holder = s.clone(); let __clear_len_guard = __clear_len_holder.lock().unwrap(); __clear_len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; let __clear_holder = s.clone(); let mut __clear_guard = __clear_holder.lock().unwrap(); if let Some(__clear_seq) = __clear_guard.as_mut() { let __clear_cap = __clear_seq.capacity(); let mut __clear_window = __clear_seq.reslice3_zeroed(__clear_start, __clear_end, __clear_cap); for __clear_elem in __clear_window.as_mut_slice().iter_mut() { *__clear_elem = String::new(); } } };
        return Arc::new(Mutex::new(Some({
            let __seq_holder = s.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
            let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
            std::mem::drop(__seq_guard);
            let __low = 0;
            let __high = ({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __max = __source_cap;
            __seq.reslice3_zeroed(__low, __high, __max)
        })));
    }
        { let mut guard = k.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return s.clone();
}

pub fn __slices__concat__slice_int__int(slices: Arc<Mutex<Option<Vec<Vec<i64>>>>>) -> Arc<Mutex<Option<Vec<i64>>>> {
    let mut size = Arc::new(Mutex::new(Some(0i64)));
    { let __range_holder = slices.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for s in __range_values.iter() {
        { let __rhs = s.len() as i64; let mut guard = size.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        if { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        std::panic::panic_any(Box::new("len out of range".to_string()) as Box<dyn Any + Send + Sync>);
    }
    } }
    let mut newslice = __slices__grow__slice_int__int(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let mut newslice = { let __slice_header = GoSliceHeader::go_slice_header(&newslice); Arc::new(Mutex::new(__slice_header)) };
    { let __range_holder = slices.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for s in __range_values.iter() {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = newslice.clone(); let __append_values: Vec<_> = s.iter().cloned().collect(); let mut __appended: Vec<i64> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.extend(__append_values); let __append_was_nil = (*__append_target.lock().unwrap()).is_none(); Arc::new(Mutex::new(if __append_was_nil && __appended.len() == 0 { None } else { Some(__appended) })) })); *newslice.lock().unwrap() = __slice_header; };
    } }
    return newslice.clone();
}

pub fn __slices__repeat__slice_string__string(x: Arc<Mutex<Option<Vec<String>>>>, count: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<Vec<String>>>> {
    if { let __tmp_x = { let __v = (*count.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        std::panic::panic_any(Box::new("cannot be negative".to_string()) as Box<dyn Any + Send + Sync>);
    }
    const maxInt: u64 = !(0 as u64) >> 1;

    let (mut hi, mut lo) = math_bits::mul(Arc::new(Mutex::new(Some({ let __arg_value = ({ let __len = (*x.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as u64; __arg_value }))), Arc::new(Mutex::new(Some({ let __arg_value = ({ let __v = (*count.lock().unwrap().as_ref().unwrap()); __v }) as u64; __arg_value }))));
    if { let __tmp_x = hi; let __tmp_y = 0 as u64; __tmp_x > __tmp_y } || { let __tmp_x = lo; let __tmp_y = maxInt as u64; __tmp_x > __tmp_y } {
        std::panic::panic_any(Box::new("the result of (len(x) * count) overflows".to_string()) as Box<dyn Any + Send + Sync>);
    }
    let mut newslice = Arc::new(Mutex::new(Some(vec!["".to_string(); (*Arc::new(Mutex::new(Some(lo as i64))).lock().unwrap().as_ref().unwrap()) as usize])));
    let mut n = {
        let _src = { let __copy_src_holder = x.clone(); let __copy_src_guard = __copy_src_holder.lock().unwrap(); __copy_src_guard.as_ref().map(|__s| __s.to_vec()).unwrap_or_default() };
        let _n = std::cmp::min((*newslice.lock().unwrap().as_ref().unwrap()).len(), _src.len());
        for _i in 0.._n {
            (*newslice.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone();
        }
        Arc::new(Mutex::new(Some(_n as i64)))
    };
    while { let __tmp_x = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*newslice.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x < __tmp_y } {
        { let __rhs = (*{
            let _dst_start = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let _dst_len = (*newslice.lock().unwrap().as_ref().unwrap()).len() - _dst_start;
            let _src = (*Arc::new(Mutex::new(Some({
                let __seq_holder = newslice.clone();
                let __seq_guard = __seq_holder.lock().unwrap();
                let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
                let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
                std::mem::drop(__seq_guard);
                let __low = 0;
                let __high = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
                let __max = __source_cap;
                __seq.reslice3_zeroed(__low, __high, __max)
            }))).lock().unwrap().as_ref().unwrap()).to_vec();
            let _n = std::cmp::min(_dst_len, _src.len());
            for _i in 0.._n {
                (*newslice.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
            }
            Arc::new(Mutex::new(Some(_n as i64)))
        }.lock().unwrap().as_ref().unwrap()); let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    return newslice.clone();
}

pub fn __slices__grow__slice_int__int(mut s: Arc<Mutex<Option<Vec<i64>>>>, mut n: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<Vec<i64>>>> {
    let mut s = Arc::new(Mutex::new((*s.lock().unwrap()).clone()));
    if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        std::panic::panic_any(Box::new("cannot be negative".to_string()) as Box<dyn Any + Send + Sync>);
    }
    {
        { let __rhs = { let __tmp_x = ((*s.lock().unwrap()).as_ref().map(|__v| __v.capacity()).unwrap_or(0) as i64); let __tmp_y = (({ let __len = (*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x - __tmp_y }; let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };;
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
            { let __slice_header = GoSliceHeader::go_slice_header(&(Arc::new(Mutex::new(Some({
                let __seq_holder = { let __append_target = Arc::new(Mutex::new(Some({
                    let __seq_holder = s.clone();
                    let __seq_guard = __seq_holder.lock().unwrap();
                    let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
                    let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
                    std::mem::drop(__seq_guard);
                    let __low = 0;
                    let __high = ((*s.lock().unwrap()).as_ref().map(|__v| __v.capacity()).unwrap_or(0)) as usize;
                    let __max = __source_cap;
                    __seq.reslice3_zeroed(__low, __high, __max)
                }))).clone(); let __append_values: Vec<_> = { let __slice_holder = Arc::new(Mutex::new(Some(vec![0i64; ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize]))).clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned().collect(); let mut __appended: Vec<i64> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.extend(__append_values); let __append_was_nil = (*__append_target.lock().unwrap()).is_none(); Arc::new(Mutex::new(if __append_was_nil && __appended.len() == 0 { None } else { Some(__appended) })) }.clone();
                let __seq_guard = __seq_holder.lock().unwrap();
                let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
                let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
                std::mem::drop(__seq_guard);
                let __low = 0;
                let __high = (({ let __len = (*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })) as usize;
                let __max = __source_cap;
                __seq.reslice3_zeroed(__low, __high, __max)
            }))))); *s.lock().unwrap() = __slice_header; };;
        }
    }
    return s.clone();
}

pub fn __slices__clip__slice_int__int(s: Arc<Mutex<Option<Vec<i64>>>>) -> Arc<Mutex<Option<Vec<i64>>>> {
    Arc::new(Mutex::new(Some({
        let mut __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned };
        let __low = 0;
        let __high = (({ let __len = (*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })) as usize;
        let __max = (({ let __len = (*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })) as usize;
        __seq.reslice3_zeroed(__low, __high, __max)
    })))
}

pub fn __slices__chunk__slice_int__int(s: Arc<Mutex<Option<Vec<i64>>>>, n: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Vec<i64>>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>>>> {
    let mut s = Arc::new(Mutex::new((*s.lock().unwrap()).clone()));
    if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x < __tmp_y } {
        std::panic::panic_any(Box::new("cannot be less than 1".to_string()) as Box<dyn Any + Send + Sync>);
    }
    let n_closure_clone = n.clone(); let s_closure_clone = s.clone(); return Arc::new(Mutex::new(Some(Arc::new(move |r#yield: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Vec<i64>>>>) -> bool + Send + Sync>>>>| {
        let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*s_closure_clone.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x < __tmp_y } {
        let mut end = Arc::new(Mutex::new(Some(std::cmp::min(({ let __v = (*n_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64), (({ let __len = (*Arc::new(Mutex::new(Some({
    let __seq_holder = s_closure_clone.clone();
    let __seq_guard = __seq_holder.lock().unwrap();
    let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
    let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
    std::mem::drop(__seq_guard);
    let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
    let __high = __seq.len();
    let __max = __source_cap;
    __seq.reslice3_zeroed(__low, __high, __max)
}))).lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64)))));
        if !{
            let __f: Arc<dyn Fn(Arc<Mutex<Option<Vec<i64>>>>) -> bool + Send + Sync> = r#yield.lock().unwrap().as_ref().unwrap().clone();
            (__f)(
                Arc::new(Mutex::new(Some({
                    let mut __seq = { let __seq_holder = s_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned };
                    let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
                    let __high = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*end.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
                    let __max = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*end.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
                    __seq.reslice3_zeroed(__low, __high, __max)
                })))
            )
        } {
        return;
    }
        { let __rhs = (*n_closure_clone.lock().unwrap().as_ref().unwrap()); let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    }) as Arc<dyn Fn(Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Vec<i64>>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>)));
}

pub fn __slices__backward__slice_string__string(s: Arc<Mutex<Option<Vec<String>>>>) -> Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<String>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>>>> {
    let mut s = Arc::new(Mutex::new((*s.lock().unwrap()).clone()));
    let s_closure_clone = s.clone(); return Arc::new(Mutex::new(Some(Arc::new(move |r#yield: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<String>>>) -> bool + Send + Sync>>>>| {
        let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = (({ let __len = (*s_closure_clone.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        if !{
            let __f: Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<String>>>) -> bool + Send + Sync> = r#yield.lock().unwrap().as_ref().unwrap().clone();
            (__f)(
                i.clone(),
                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = s_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
            )
        } {
        return;
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    }) as Arc<dyn Fn(Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<String>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>)));
}

pub fn __slices__compare__slice_int__int(s1: Arc<Mutex<Option<Vec<i64>>>>, s2: Arc<Mutex<Option<Vec<i64>>>>) -> i64 {
    { let __range_holder = s1.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for (i, v1) in __range_values.iter().copied().enumerate() {
        if { let __tmp_x = (i as i64); let __tmp_y = (({ let __len = (*s2.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x >= __tmp_y } {
        return 1;
    }
        let mut v2 = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = s2.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() })));
        {
        let mut c = __cmp__compare__int(Arc::new(Mutex::new(Some(v1))), Arc::new(Mutex::new(Some({ let __arg_holder = v2.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if { let __tmp_x = c; let __tmp_y = 0; __tmp_x != __tmp_y } {
            return c;;
        }
    }
    } }
    if { let __tmp_x = (({ let __len = (*s1.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); let __tmp_y = (({ let __len = (*s2.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x < __tmp_y } {
        return -(1);
    }
    0
}

pub fn __slices__compare__slice_string__string(s1: Arc<Mutex<Option<Vec<String>>>>, s2: Arc<Mutex<Option<Vec<String>>>>) -> i64 {
    { let __range_holder = s1.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for (i, v1) in __range_values.iter().enumerate() {
        if { let __tmp_x = (i as i64); let __tmp_y = (({ let __len = (*s2.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x >= __tmp_y } {
        return 1;
    }
        let mut v2 = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = s2.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() })));
        {
        let mut c = __cmp__compare__string(Arc::new(Mutex::new(Some((*v1).clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = v2.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if { let __tmp_x = c; let __tmp_y = 0; __tmp_x != __tmp_y } {
            return c;;
        }
    }
    } }
    if { let __tmp_x = (({ let __len = (*s1.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); let __tmp_y = (({ let __len = (*s2.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x < __tmp_y } {
        return -(1);
    }
    0
}

pub fn __slices__equal_func__slice_int__slice_string__int__string(s1: Arc<Mutex<Option<Vec<i64>>>>, s2: Arc<Mutex<Option<Vec<String>>>>, eq: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<String>>>) -> bool + Send + Sync>>>>) -> bool {
    if { let __tmp_x = (({ let __len = (*s1.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); let __tmp_y = (({ let __len = (*s2.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x != __tmp_y } {
        return false;
    }
    { let __range_holder = s1.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for (i, v1) in __range_values.iter().copied().enumerate() {
        let mut v2 = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = s2.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() })));
        if !{
            let __f: Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<String>>>) -> bool + Send + Sync> = eq.lock().unwrap().as_ref().unwrap().clone();
            (__f)(
                Arc::new(Mutex::new(Some(v1))),
                Arc::new(Mutex::new(Some({ let __arg_holder = v2.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
            )
        } {
        return false;
    }
    } }
    true
}

pub fn __slices__sort_stable_func__slice__item___item(x: Arc<Mutex<Option<Vec<Item>>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Item>>>, Arc<Mutex<Option<Item>>>) -> i64 + Send + Sync>>>>) {
    __slices_stable_cmp_func___item(
        x.clone(),
        Arc::new(Mutex::new(Some({ let __arg_value = ({ let __len = (*x.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64; __arg_value }))),
        Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
    );
}

pub fn __cmp__compare__int(x: Arc<Mutex<Option<i64>>>, y: Arc<Mutex<Option<i64>>>) -> i64 {
    let mut xNaN = __cmp_is_na_n__int(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let mut yNaN = __cmp_is_na_n__int(Arc::new(Mutex::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    if xNaN {
        if yNaN {
        return 0;
    }
        return -(1);
    }
    if yNaN {
        return 1;
    }
    if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        return -(1);
    }
    if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        return 1;
    }
    0
}

pub fn __cmp__compare__string(x: Arc<Mutex<Option<String>>>, y: Arc<Mutex<Option<String>>>) -> i64 {
    let mut xNaN = __cmp_is_na_n__string(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let mut yNaN = __cmp_is_na_n__string(Arc::new(Mutex::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    if xNaN {
        if yNaN {
        return 0;
    }
        return -(1);
    }
    if yNaN {
        return 1;
    }
    if { let __tmp_x = ({ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __tmp_y = ({ let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }); __tmp_x < __tmp_y } {
        return -(1);
    }
    if { let __tmp_x = ({ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __tmp_y = ({ let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }); __tmp_x > __tmp_y } {
        return 1;
    }
    0
}

pub fn __cmp__compare__float64(x: Arc<Mutex<Option<f64>>>, y: Arc<Mutex<Option<f64>>>) -> i64 {
    let mut xNaN = __cmp_is_na_n__float64(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let mut yNaN = __cmp_is_na_n__float64(Arc::new(Mutex::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    if xNaN {
        if yNaN {
        return 0;
    }
        return -(1);
    }
    if yNaN {
        return 1;
    }
    if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        return -(1);
    }
    if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        return 1;
    }
    0
}

pub fn __slices__is_sorted_func__slice__item___item(x: Arc<Mutex<Option<Vec<Item>>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Item>>>, Arc<Mutex<Option<Item>>>) -> i64 + Send + Sync>>>>) -> bool {
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = (({ let __len = (*x.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        if {
            let __tmp_x =
                {
                    let __f: Arc<dyn Fn(Arc<Mutex<Option<Item>>>, Arc<Mutex<Option<Item>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                    (__f)(
                        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value })))
                    )
                };
            let __tmp_y = 0;
            __tmp_x < __tmp_y
        } {
        return false;
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    true
}

pub fn __slices__binary_search_func__slice__item___item__int(x: Arc<Mutex<Option<Vec<Item>>>>, target: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Item>>>, Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync>>>>) -> (i64, bool) {
    let mut n = Arc::new(Mutex::new(Some(({ let __len = (*x.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64)));
    let (mut i, mut j) = (Arc::new(Mutex::new(Some(0i64))), { let __tuple_value = { let __owned = n.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }; __tuple_value });
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut h = Arc::new(Mutex::new(Some((({ let __tmp_x = (*Arc::new(Mutex::new(Some((({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x >> __tmp_y })) as i64)));
        if {
            let __tmp_x =
                {
                    let __f: Arc<dyn Fn(Arc<Mutex<Option<Item>>>, Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                    (__f)(
                        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                        target.clone()
                    )
                };
            let __tmp_y = 0;
            __tmp_x < __tmp_y
        } {
        { let new_val = { let __tmp_x = { let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }; *i.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = h.lock().unwrap().as_ref().unwrap().clone(); *j.lock().unwrap() = Some(new_val); };
    }
    }
    return (
        { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v },
        {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = {
                    let __tmp_x =
                        {
                            let __f: Arc<dyn Fn(Arc<Mutex<Option<Item>>>, Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                            (__f)(
                                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                                target.clone()
                            )
                        };
                    let __tmp_y = 0;
                    __tmp_x == __tmp_y
                };
                __go_cond_1
            } else {
                false
            }
        }
    );
}

pub fn __slices__min_func__slice__item___item(x: Arc<Mutex<Option<Vec<Item>>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Item>>>, Arc<Mutex<Option<Item>>>) -> i64 + Send + Sync>>>>) -> Arc<Mutex<Option<Item>>> {
    if { let __tmp_x = (({ let __len = (*x.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); let __tmp_y = 1; __tmp_x < __tmp_y } {
        std::panic::panic_any(Box::new("slices.MinFunc: empty list".to_string()) as Box<dyn Any + Send + Sync>);
    }
    let mut m = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.__go_value_clone())));
    let mut i = Arc::new(Mutex::new(Some(1i64)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*x.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x < __tmp_y } {
        if {
            let __tmp_x =
                {
                    let __f: Arc<dyn Fn(Arc<Mutex<Option<Item>>>, Arc<Mutex<Option<Item>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                    (__f)(
                        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                        m.clone()
                    )
                };
            let __tmp_y = 0;
            __tmp_x < __tmp_y
        } {
        { let new_val = { let __seq = { let __seq_holder = x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.__go_value_clone(); *m.lock().unwrap() = Some(new_val); };
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return { let __owned = m.lock().unwrap().as_ref().unwrap().__go_value_clone(); Arc::new(Mutex::new(Some(__owned))) };
}

pub fn __slices__max_func__slice__item___item(x: Arc<Mutex<Option<Vec<Item>>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Item>>>, Arc<Mutex<Option<Item>>>) -> i64 + Send + Sync>>>>) -> Arc<Mutex<Option<Item>>> {
    if { let __tmp_x = (({ let __len = (*x.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); let __tmp_y = 1; __tmp_x < __tmp_y } {
        std::panic::panic_any(Box::new("slices.MaxFunc: empty list".to_string()) as Box<dyn Any + Send + Sync>);
    }
    let mut m = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.__go_value_clone())));
    let mut i = Arc::new(Mutex::new(Some(1i64)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*x.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x < __tmp_y } {
        if {
            let __tmp_x =
                {
                    let __f: Arc<dyn Fn(Arc<Mutex<Option<Item>>>, Arc<Mutex<Option<Item>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                    (__f)(
                        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                        m.clone()
                    )
                };
            let __tmp_y = 0;
            __tmp_x > __tmp_y
        } {
        { let new_val = { let __seq = { let __seq_holder = x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.__go_value_clone(); *m.lock().unwrap() = Some(new_val); };
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return { let __owned = m.lock().unwrap().as_ref().unwrap().__go_value_clone(); Arc::new(Mutex::new(Some(__owned))) };
}

pub fn __slices__delete_func__slice__item___item(s: Arc<Mutex<Option<Vec<Item>>>>, del: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Item>>>) -> bool + Send + Sync>>>>) -> Arc<Mutex<Option<Vec<Item>>>> {
    let mut i = __slices__index_func__slice__item___item(s.clone(), Arc::new(Mutex::new(GoFuncValue::go_func_value(&(del)))));
    if { let __tmp_x = i; let __tmp_y = -1; __tmp_x == __tmp_y } {
        return s.clone();
    }
    let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = i; let __tmp_y = 1; __tmp_x + __tmp_y })));
    while { let __tmp_x = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x < __tmp_y } {
        {
        let mut v = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.__go_value_clone())));;
        if !{
            let __f: Arc<dyn Fn(Arc<Mutex<Option<Item>>>) -> bool + Send + Sync> = del.lock().unwrap().as_ref().unwrap().clone();
            (__f)(
                v.clone()
            )
        } {
            { let __idx = (i) as usize; let __rhs = { let __v = (*v.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }; (*s.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };;
            { i += 1; };
        }
    }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    { let __clear_start = (i) as usize; let __clear_end = { let __clear_len_holder = s.clone(); let __clear_len_guard = __clear_len_holder.lock().unwrap(); __clear_len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; let __clear_holder = s.clone(); let mut __clear_guard = __clear_holder.lock().unwrap(); if let Some(__clear_seq) = __clear_guard.as_mut() { let __clear_cap = __clear_seq.capacity(); let mut __clear_window = __clear_seq.reslice3_zeroed(__clear_start, __clear_end, __clear_cap); for __clear_elem in __clear_window.as_mut_slice().iter_mut() { *__clear_elem = Default::default(); } } };
    Arc::new(Mutex::new(Some({
        let __seq_holder = s.clone();
        let __seq_guard = __seq_holder.lock().unwrap();
        let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
        let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
        std::mem::drop(__seq_guard);
        let __low = 0;
        let __high = (i) as usize;
        let __max = __source_cap;
        __seq.reslice3(__low, __high, __max)
    })))
}

pub fn __slices__index__slice__item___item(s: Arc<Mutex<Option<Vec<Item>>>>, v: Arc<Mutex<Option<Item>>>) -> i64 {
    for i in 0..(({ let __range_holder = s.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        if {
            let __tmp_x = ({ let __v = (*v.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v });
            let __tmp_y = { let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() };
            __tmp_x == __tmp_y
        } {
        return i as i64;
    }
    }
    -(1)
}

pub fn __slices__index__slice_ptr__task__ptr__task(s: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Task>>>>>>>, v: Arc<Mutex<Option<Task>>>) -> i64 {
    for i in 0..(({ let __range_holder = s.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        if {
            let __left = v.clone();
            let __right = { let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() };
            let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
            let __eq = __both_nil || Arc::ptr_eq(&__left, &__right) || { let __left_id = __left.lock().unwrap().as_ref().map(|__v| Arc::as_ptr(&__v.name) as *const () as usize); let __right_id = __right.lock().unwrap().as_ref().map(|__v| Arc::as_ptr(&__v.name) as *const () as usize); __left_id.is_some() && __left_id == __right_id };
            __eq
        } {
        return i as i64;
    }
    }
    -(1)
}

pub fn __slices__contains__slice__item___item(s: Arc<Mutex<Option<Vec<Item>>>>, v: Arc<Mutex<Option<Item>>>) -> bool {
    return { let __tmp_x = __slices__index__slice__item___item(s.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() })))); let __tmp_y = 0; __tmp_x >= __tmp_y };
}

pub fn __slices__sort_func__slice_ptr__task__ptr__task(x: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Task>>>>>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Task>>>, Arc<Mutex<Option<Task>>>) -> i64 + Send + Sync>>>>) {
    let mut n = Arc::new(Mutex::new(Some(({ let __len = (*x.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64)));
    __slices_pdqsort_cmp_func__ptr__task(
        x.clone(),
        Arc::new(Mutex::new(Some(0))),
        Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_value = math_bits::len(Arc::new(Mutex::new(Some({ let __arg_value = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()); __v }) as u64; __arg_value })))); __arg_value }))),
        Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
    );
}

pub fn __slices__sort_func__slice_int__int(x: Arc<Mutex<Option<Vec<i64>>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync>>>>) {
    let mut n = Arc::new(Mutex::new(Some(({ let __len = (*x.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64)));
    __slices_pdqsort_cmp_func__int(
        x.clone(),
        Arc::new(Mutex::new(Some(0))),
        Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_value = math_bits::len(Arc::new(Mutex::new(Some({ let __arg_value = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()); __v }) as u64; __arg_value })))); __arg_value }))),
        Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
    );
}

pub fn __slices__index_func__slice_ptr__task__ptr__task(s: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Task>>>>>>>, f: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Task>>>) -> bool + Send + Sync>>>>) -> i64 {
    for i in 0..(({ let __range_holder = s.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        if {
            let __f: Arc<dyn Fn(Arc<Mutex<Option<Task>>>) -> bool + Send + Sync> = f.lock().unwrap().as_ref().unwrap().clone();
            (__f)(
                { let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }
            )
        } {
        return i as i64;
    }
    }
    -(1)
}

pub fn __slices__index_func__slice__item___item(s: Arc<Mutex<Option<Vec<Item>>>>, f: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Item>>>) -> bool + Send + Sync>>>>) -> i64 {
    for i in 0..(({ let __range_holder = s.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        if {
            let __f: Arc<dyn Fn(Arc<Mutex<Option<Item>>>) -> bool + Send + Sync> = f.lock().unwrap().as_ref().unwrap().clone();
            (__f)(
                Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() })))
            )
        } {
        return i as i64;
    }
    }
    -(1)
}

pub fn __slices__contains_func__slice_ptr__task__ptr__task(s: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Task>>>>>>>, f: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Task>>>) -> bool + Send + Sync>>>>) -> bool {
    return { let __tmp_x = __slices__index_func__slice_ptr__task__ptr__task(s.clone(), Arc::new(Mutex::new(GoFuncValue::go_func_value(&(f))))); let __tmp_y = 0; __tmp_x >= __tmp_y };
}

pub fn __slices__sort__slice__celsius___celsius(x: Arc<Mutex<Option<Vec<Celsius>>>>) {
    let mut n = Arc::new(Mutex::new(Some(({ let __len = (*x.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64)));
    __slices_pdqsort_ordered___celsius(
        x.clone(),
        Arc::new(Mutex::new(Some(0))),
        Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_value = math_bits::len(Arc::new(Mutex::new(Some({ let __arg_value = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()); __v }) as u64; __arg_value })))); __arg_value })))
    );
}

pub fn __slices__sort___i_ds__int(x: Arc<Mutex<Option<IDs>>>) {
    let mut n = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64)));
    __slices_pdqsort_ordered__int(
        { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice },
        Arc::new(Mutex::new(Some(0))),
        Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_value = math_bits::len(Arc::new(Mutex::new(Some({ let __arg_value = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()); __v }) as u64; __arg_value })))); __arg_value })))
    );
}

pub fn __slices__sort__slice_float64__float64(x: Arc<Mutex<Option<Vec<f64>>>>) {
    let mut n = Arc::new(Mutex::new(Some(({ let __len = (*x.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64)));
    __slices_pdqsort_ordered__float64(
        x.clone(),
        Arc::new(Mutex::new(Some(0))),
        Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_value = math_bits::len(Arc::new(Mutex::new(Some({ let __arg_value = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()); __v }) as u64; __arg_value })))); __arg_value })))
    );
}

pub fn __slices__sort__slice_int__int(x: Arc<Mutex<Option<Vec<i64>>>>) {
    let mut n = Arc::new(Mutex::new(Some(({ let __len = (*x.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64)));
    __slices_pdqsort_ordered__int(
        x.clone(),
        Arc::new(Mutex::new(Some(0))),
        Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_value = math_bits::len(Arc::new(Mutex::new(Some({ let __arg_value = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()); __v }) as u64; __arg_value })))); __arg_value })))
    );
}

pub fn __slices__sort__slice_string__string(x: Arc<Mutex<Option<Vec<String>>>>) {
    let mut n = Arc::new(Mutex::new(Some(({ let __len = (*x.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64)));
    __slices_pdqsort_ordered__string(
        x.clone(),
        Arc::new(Mutex::new(Some(0))),
        Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_value = math_bits::len(Arc::new(Mutex::new(Some({ let __arg_value = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()); __v }) as u64; __arg_value })))); __arg_value })))
    );
}

pub fn __slices__max__slice__celsius___celsius(x: Arc<Mutex<Option<Vec<Celsius>>>>) -> Arc<Mutex<Option<Celsius>>> {
    if { let __tmp_x = (({ let __len = (*x.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); let __tmp_y = 1; __tmp_x < __tmp_y } {
        std::panic::panic_any(Box::new("slices.Max: empty list".to_string()) as Box<dyn Any + Send + Sync>);
    }
    let mut m = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() })));
    let mut i = Arc::new(Mutex::new(Some(1i64)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*x.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x < __tmp_y } {
        { let new_val = Celsius(Arc::new(Mutex::new(Some({ let __l = (*({ let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }).0.lock().unwrap().as_ref().unwrap()).clone(); let __r = (*({ let __seq = { let __seq_holder = x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }).0.lock().unwrap().as_ref().unwrap()).clone(); if __l.is_nan() || __r.is_nan() { __l + __r } else if __r > __l || (__r == __l && __r.is_sign_positive()) { __r } else { __l } })))); *m.lock().unwrap() = Some(new_val); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return { let __owned = m.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}

pub fn __slices__min__slice__celsius___celsius(x: Arc<Mutex<Option<Vec<Celsius>>>>) -> Arc<Mutex<Option<Celsius>>> {
    if { let __tmp_x = (({ let __len = (*x.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); let __tmp_y = 1; __tmp_x < __tmp_y } {
        std::panic::panic_any(Box::new("slices.Min: empty list".to_string()) as Box<dyn Any + Send + Sync>);
    }
    let mut m = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() })));
    let mut i = Arc::new(Mutex::new(Some(1i64)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*x.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x < __tmp_y } {
        { let new_val = Celsius(Arc::new(Mutex::new(Some({ let __l = (*({ let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }).0.lock().unwrap().as_ref().unwrap()).clone(); let __r = (*({ let __seq = { let __seq_holder = x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }).0.lock().unwrap().as_ref().unwrap()).clone(); if __l.is_nan() || __r.is_nan() { __l + __r } else if __r < __l || (__r == __l && __r.is_sign_negative()) { __r } else { __l } })))); *m.lock().unwrap() = Some(new_val); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return { let __owned = m.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}

pub fn __cmp__less___celsius(x: Arc<Mutex<Option<Celsius>>>, y: Arc<Mutex<Option<Celsius>>>) -> bool {
    (__cmp_is_na_n___celsius(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) && !__cmp_is_na_n___celsius(Arc::new(Mutex::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))) || { let __tmp_x = ({ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __tmp_y = ({ let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }); __tmp_x < __tmp_y }
}

pub fn __cmp__less__int(x: Arc<Mutex<Option<i64>>>, y: Arc<Mutex<Option<i64>>>) -> bool {
    (__cmp_is_na_n__int(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) && !__cmp_is_na_n__int(Arc::new(Mutex::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))) || { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y }
}

pub fn __cmp__less__float64(x: Arc<Mutex<Option<f64>>>, y: Arc<Mutex<Option<f64>>>) -> bool {
    (__cmp_is_na_n__float64(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) && !__cmp_is_na_n__float64(Arc::new(Mutex::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))) || { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y }
}

pub fn __cmp__less__string(x: Arc<Mutex<Option<String>>>, y: Arc<Mutex<Option<String>>>) -> bool {
    (__cmp_is_na_n__string(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) && !__cmp_is_na_n__string(Arc::new(Mutex::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))) || { let __tmp_x = ({ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __tmp_y = ({ let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }); __tmp_x < __tmp_y }
}

pub fn __slices__binary_search___i_ds__int(x: Arc<Mutex<Option<IDs>>>, target: Arc<Mutex<Option<i64>>>) -> (i64, bool) {
    let mut n = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64)));
    let (mut i, mut j) = (Arc::new(Mutex::new(Some(0i64))), { let __tuple_value = { let __owned = n.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }; __tuple_value });
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut h = Arc::new(Mutex::new(Some((({ let __tmp_x = (*Arc::new(Mutex::new(Some((({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x >> __tmp_y })) as i64)));
        if __cmp__less__int(
            Arc::new(Mutex::new(Some({ let __seq_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_index = ({ let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = target.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
        ) {
        { let new_val = { let __tmp_x = { let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }; *i.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = h.lock().unwrap().as_ref().unwrap().clone(); *j.lock().unwrap() = Some(new_val); };
    }
    }
    return (
        { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v },
        {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = {
                    let __go_cond_2 = {
                        let __tmp_x = { let __seq_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_index = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() };
                        let __tmp_y = { let __v = (*target.lock().unwrap().as_ref().unwrap()).clone(); __v };
                        __tmp_x == __tmp_y
                    };
                    if __go_cond_2 {
                        true
                    } else {
                        let __go_cond_3 = {
                            let __go_cond_4 = __slices_is_na_n__int(Arc::new(Mutex::new(Some({ let __seq_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_index = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() }))));
                            if __go_cond_4 {
                                let __go_cond_5 = __slices_is_na_n__int(Arc::new(Mutex::new(Some({ let __arg_holder = target.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
                                __go_cond_5
                            } else {
                                false
                            }
                        };
                        __go_cond_3
                    }
                };
                __go_cond_1
            } else {
                false
            }
        }
    );
}

pub fn __slices__clone___i_ds__int(s: Arc<Mutex<Option<IDs>>>) -> Arc<Mutex<Option<IDs>>> {
    if { let __slice_holder = { let __named_slice = s.lock().unwrap().as_ref().map(|__v| __v.0.clone()).unwrap_or_else(|| Arc::new(Mutex::new(None))); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); (*__slice_guard).is_none() } {
        return Arc::new(Mutex::new(None));
    }
    { let __base = { let __named_slice = IDs(Arc::new(Mutex::new(Some(vec![])))); __named_slice.0.clone() }; let __base_guard = __base.lock().unwrap(); let mut __values = __base_guard.as_ref().cloned().unwrap_or_else(Vec::new); std::mem::drop(__base_guard); let __src = { let __named_slice = (*s.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __src_guard = __src.lock().unwrap(); if let Some(__src_values) = __src_guard.as_ref() { __values.extend(__src_values.iter().cloned()); }; Arc::new(Mutex::new(Some(IDs(Arc::new(Mutex::new(Some(__values))))))) }
}

pub fn __cmp__or__int(vals: Arc<Mutex<Option<Vec<i64>>>>) -> i64 {
    let mut zero: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    { let __range_holder = vals.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for val in __range_values.iter().copied() {
        if { let __tmp_x = val; let __tmp_y = { let __v = (*zero.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
        return val;
    }
    } }
    return { let __v = (*zero.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn __cmp__or__string(vals: Arc<Mutex<Option<Vec<String>>>>) -> Arc<Mutex<Option<String>>> {
    let mut zero: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    { let __range_holder = vals.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for val in __range_values.iter() {
        if { let __tmp_x = (*val).clone(); let __tmp_y = ({ let __v = (*zero.lock().unwrap().as_ref().unwrap()).clone(); __v }); __tmp_x != __tmp_y } {
        return Arc::new(Mutex::new(Some((*val).clone())));
    }
    } }
    return { let __owned = zero.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}

pub fn __slices__is_sorted__slice_int__int(x: Arc<Mutex<Option<Vec<i64>>>>) -> bool {
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = (({ let __len = (*x.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        if __cmp__less__int(
            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value })))
        ) {
        return false;
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    true
}

pub fn __slices__reverse__slice_int__int(s: Arc<Mutex<Option<Vec<i64>>>>) {
    let (mut i, mut j) = (Arc::new(Mutex::new(Some(0i64))), { let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = (({ let __len = (*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); let __tmp_y = 1; __tmp_x - __tmp_y }))); __tuple_value });
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        {
            let __go_lhs_sequence_0 = s.clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = s.clone();
            let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        {
            let __tmp_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y };
            let __tmp_1 = { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y };
            *i.lock().unwrap() = Some(__tmp_0);
            *j.lock().unwrap() = Some(__tmp_1);
        };
    }
}

pub fn __slices__sorted__string(seq: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<String>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>>>>) -> Arc<Mutex<Option<Vec<String>>>> {
    let mut s = __slices__collect__string(Arc::new(Mutex::new(GoFuncValue::go_func_value(&(seq)))));
    let mut s = { let __slice_header = GoSliceHeader::go_slice_header(&s); Arc::new(Mutex::new(__slice_header)) };
    __slices__sort__slice_string__string(s.clone());
    return s.clone();
}

pub fn __slices__sorted__int(seq: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<i64>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>>>>) -> Arc<Mutex<Option<Vec<i64>>>> {
    let mut s = __slices__collect__int(Arc::new(Mutex::new(GoFuncValue::go_func_value(&(seq)))));
    let mut s = { let __slice_header = GoSliceHeader::go_slice_header(&s); Arc::new(Mutex::new(__slice_header)) };
    __slices__sort__slice_int__int(s.clone());
    return s.clone();
}

pub fn __maps__keys__map_string_int__string__int(m: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>) -> Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<String>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>>>> {
    let m_closure_clone = m.clone(); return Arc::new(Mutex::new(Some(Arc::new(move |r#yield: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<String>>>) -> bool + Send + Sync>>>>| {
        for (k, _) in { let __range_holder = m_closure_clone.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        if !{
            let __f: Arc<dyn Fn(Arc<Mutex<Option<String>>>) -> bool + Send + Sync> = r#yield.lock().unwrap().as_ref().unwrap().clone();
            (__f)(
                Arc::new(Mutex::new(Some(k.clone())))
            )
        } {
        return;
    }
    }
    }) as Arc<dyn Fn(Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<String>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>)));
}

pub fn __maps__values__map_string_int__string__int(m: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>) -> Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<i64>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>>>> {
    let m_closure_clone = m.clone(); return Arc::new(Mutex::new(Some(Arc::new(move |r#yield: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<i64>>>) -> bool + Send + Sync>>>>| {
        for (_, v) in { let __range_holder = m_closure_clone.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        if !{
            let __f: Arc<dyn Fn(Arc<Mutex<Option<i64>>>) -> bool + Send + Sync> = r#yield.lock().unwrap().as_ref().unwrap().clone();
            (__f)(
                v.clone()
            )
        } {
        return;
    }
    }
    }) as Arc<dyn Fn(Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<i64>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>)));
}

pub fn __maps__collect__int__string(seq: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<String>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>>>>) -> Arc<Mutex<Option<BTreeMap<i64, Arc<Mutex<Option<String>>>>>>> {
    let mut m = Arc::new(Mutex::new(Some(BTreeMap::<i64, Arc<Mutex<Option<String>>>>::new())));
    __maps__insert__map_int_string__int__string(m.clone(), Arc::new(Mutex::new(GoFuncValue::go_func_value(&(seq)))));
    return m.clone();
}

pub fn __slices__all__slice_string__string(s: Arc<Mutex<Option<Vec<String>>>>) -> Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<String>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>>>> {
    let mut s = Arc::new(Mutex::new((*s.lock().unwrap()).clone()));
    let s_closure_clone = s.clone(); return Arc::new(Mutex::new(Some(Arc::new(move |r#yield: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<String>>>) -> bool + Send + Sync>>>>| {
        { let __range_holder = s_closure_clone.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for (i, v) in __range_values.iter().enumerate() {
        if !{
            let __f: Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<String>>>) -> bool + Send + Sync> = r#yield.lock().unwrap().as_ref().unwrap().clone();
            (__f)(
                Arc::new(Mutex::new(Some(i as i64))),
                Arc::new(Mutex::new(Some((*v).clone())))
            )
        } {
        return;
    }
    } }
    }) as Arc<dyn Fn(Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<String>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>)));
}

pub fn __maps__copy__map_string_int__map_string_int__string__int(dst: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>, src: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>) {
    for (k, v) in { let __range_holder = src.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __map_key = k.clone(); let __map_value = Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap()).clone()))); (*dst.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    }
}

pub fn __maps__delete_func__map_string_int__string__int(m: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>, del: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<i64>>>) -> bool + Send + Sync>>>>) {
    for (k, v) in { let __range_holder = m.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        if {
            let __f: Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<i64>>>) -> bool + Send + Sync> = del.lock().unwrap().as_ref().unwrap().clone();
            (__f)(
                Arc::new(Mutex::new(Some(k.clone()))),
                v.clone()
            )
        } {
        { let __map_handle = m.clone(); let mut __map_guard = __map_handle.lock().unwrap(); __map_guard.as_mut().unwrap().remove(&k); };
    }
    }
}

pub fn __maps__equal_func__map_string_int__map_string_int__string__int__int(m1: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>, m2: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>, eq: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> bool + Send + Sync>>>>) -> bool {
    if { let __tmp_x = (({ let __len = (*m1.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); let __tmp_y = (({ let __len = (*m2.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x != __tmp_y } {
        return false;
    }
    for (k, v1) in { let __range_holder = m1.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        {
        let (mut v2, mut ok) = { let __map = { let __map_holder = m2.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&k)) { /* MAP_COMMA_OK */ Some(v) => (Arc::new(Mutex::new(Some(v.lock().unwrap().as_ref().unwrap().clone()))), true), None => (Arc::new(Mutex::new(Some(0))), false) } };;
        if !ok || !{
            let __f: Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> bool + Send + Sync> = eq.lock().unwrap().as_ref().unwrap().clone();
            (__f)(
                v1.clone(),
                v2.clone()
            )
        } {
            return false;;
        }
    }
    }
    true
}

pub fn __maps__insert__map_string_int__string__int(m: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>, seq: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<i64>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>>>>) {
    let mut __rf_done_3: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    let m_closure_clone = m.clone(); {
        let __f: Arc<dyn Fn(Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<i64>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync> = seq.lock().unwrap().as_ref().unwrap().clone();
        (__f)(
            Arc::new(Mutex::new(Some(Arc::new(move |__rf_param_3_0: Arc<Mutex<Option<String>>>, __rf_param_3_1: Arc<Mutex<Option<i64>>>| -> bool {
        let (mut k, mut v) = ({ let __tuple_value = { let __owned = __rf_param_3_0.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }; __tuple_value }, { let __tuple_value = { let __owned = __rf_param_3_1.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }; __tuple_value });
        { let __map_key = ({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __map_value = Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap()).clone()))); (*m_closure_clone.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        true
    }) as Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<i64>>>) -> bool + Send + Sync>)))
        )
    };
    if { let __v = (*__rf_done_3.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return;
    }
}

pub fn __maps__insert__map_int_string__int__string(m: Arc<Mutex<Option<BTreeMap<i64, Arc<Mutex<Option<String>>>>>>>, seq: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<String>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>>>>) {
    let mut __rf_done_4: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    let m_closure_clone = m.clone(); {
        let __f: Arc<dyn Fn(Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<String>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync> = seq.lock().unwrap().as_ref().unwrap().clone();
        (__f)(
            Arc::new(Mutex::new(Some(Arc::new(move |__rf_param_4_0: Arc<Mutex<Option<i64>>>, __rf_param_4_1: Arc<Mutex<Option<String>>>| -> bool {
        let (mut k, mut v) = ({ let __tuple_value = { let __owned = __rf_param_4_0.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }; __tuple_value }, { let __tuple_value = { let __owned = __rf_param_4_1.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }; __tuple_value });
        { let __map_key = { let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __map_value = Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap()).clone()))); (*m_closure_clone.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        true
    }) as Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<String>>>) -> bool + Send + Sync>)))
        )
    };
    if { let __v = (*__rf_done_4.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return;
    }
}

pub fn __maps__all__map_string_int__string__int(m: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>) -> Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<i64>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>>>> {
    let m_closure_clone = m.clone(); return Arc::new(Mutex::new(Some(Arc::new(move |r#yield: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<i64>>>) -> bool + Send + Sync>>>>| {
        for (k, v) in { let __range_holder = m_closure_clone.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        if !{
            let __f: Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<i64>>>) -> bool + Send + Sync> = r#yield.lock().unwrap().as_ref().unwrap().clone();
            (__f)(
                Arc::new(Mutex::new(Some(k.clone()))),
                v.clone()
            )
        } {
        return;
    }
    }
    }) as Arc<dyn Fn(Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<i64>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>)));
}

pub fn __maps__clone__map_string_int__string__int(m: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>) -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> {
    if { let __nil_result = (*m.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(None));
    }
    let mut r = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::new())));
    for (k, v) in { let __range_holder = m.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __map_key = k.clone(); let __map_value = Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap()).clone()))); (*r.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    }
    return r.clone();
}

pub fn __maps__equal__map_string_int__map_string_int__string__int(m1: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>, m2: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>) -> bool {
    if { let __tmp_x = (({ let __len = (*m1.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); let __tmp_y = (({ let __len = (*m2.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); __tmp_x != __tmp_y } {
        return false;
    }
    for (k, v1) in { let __range_holder = m1.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        {
        let (mut v2, mut ok) = { let __map = { let __map_holder = m2.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&k)) { /* MAP_COMMA_OK */ Some(v) => (Arc::new(Mutex::new(Some(v.lock().unwrap().as_ref().unwrap().clone()))), true), None => (Arc::new(Mutex::new(Some(0))), false) } };;
        if !ok || { let __tmp_x = (*v1.lock().unwrap().as_mut().unwrap()); let __tmp_y = { let __v = (*v2.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
            return false;;
        }
    }
    }
    true
}

pub fn __slices__collect__string(seq: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<String>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>>>>) -> Arc<Mutex<Option<Vec<String>>>> {
    __slices__append_seq__slice_string__string(
        Arc::new(Mutex::new(None::<Vec<String>>)),
        Arc::new(Mutex::new(GoFuncValue::go_func_value(&(seq))))
    )
}

pub fn __slices__collect__int(seq: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<i64>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>>>>) -> Arc<Mutex<Option<Vec<i64>>>> {
    __slices__append_seq__slice_int__int(
        Arc::new(Mutex::new(None::<Vec<i64>>)),
        Arc::new(Mutex::new(GoFuncValue::go_func_value(&(seq))))
    )
}

pub fn __slices_overlaps__int(a: Arc<Mutex<Option<Vec<i64>>>>, b: Arc<Mutex<Option<Vec<i64>>>>) -> bool {
    if { let __tmp_x = (({ let __len = (*a.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); let __tmp_y = 0; __tmp_x == __tmp_y } || { let __tmp_x = (({ let __len = (*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return false;
    }
    for i in 0..(({ let __range_holder = a.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        if { let __left = GoSliceElemPtr::new(a.clone(), (i) as usize); let __right = GoSliceElemPtr::new(b.clone(), (0) as usize); let __eq = __left.identity() == __right.identity(); __eq } {
        return true;
    }
    }
    for i in 0..(({ let __range_holder = b.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        if { let __left = GoSliceElemPtr::new(b.clone(), (i) as usize); let __right = GoSliceElemPtr::new(a.clone(), (0) as usize); let __eq = __left.identity() == __right.identity(); __eq } {
        return true;
    }
    }
    false
}

pub fn __slices_rotate_right__int(s: Arc<Mutex<Option<Vec<i64>>>>, r: Arc<Mutex<Option<i64>>>) {
    __slices_rotate_left__int(
        s.clone(),
        Arc::new(Mutex::new(Some({ let __tmp_x = (({ let __len = (*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); let __tmp_y = ({ let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); __tmp_x - __tmp_y })))
    );
}

pub fn __slices_start_idx__int(haystack: Arc<Mutex<Option<Vec<i64>>>>, needle: Arc<Mutex<Option<Vec<i64>>>>) -> i64 {
    let mut p: Option<GoSliceElemPtr<i64>> = Some(GoSliceElemPtr::new(needle.clone(), (0) as usize));
    for i in 0..(({ let __range_holder = haystack.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        if { let __left = p.clone(); let __right = Some(GoSliceElemPtr::new(haystack.clone(), (i) as usize)); let __eq = match (&__left, &__right) { (None, None) => true, (Some(__l), Some(__r)) => __l.identity() == __r.identity(), _ => false }; __eq } {
        return i as i64;
    }
    }
    std::panic::panic_any(Box::new("needle not found".to_string()) as Box<dyn Any + Send + Sync>);
}

pub fn __slices_stable_cmp_func___item(data: Arc<Mutex<Option<Vec<Item>>>>, n: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Item>>>, Arc<Mutex<Option<Item>>>) -> i64 + Send + Sync>>>>) {
    let mut blockSize = Arc::new(Mutex::new(Some(20i64)));
    let (mut a, mut b) = (Arc::new(Mutex::new(Some(0i64))), { let __tuple_value = { let __owned = blockSize.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }; __tuple_value });
    while { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } {
        __slices_insertion_sort_cmp_func___item(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
        { let new_val = b.lock().unwrap().as_ref().unwrap().clone(); *a.lock().unwrap() = Some(new_val); };
        { let __rhs = (*blockSize.lock().unwrap().as_ref().unwrap()); let mut guard = b.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    __slices_insertion_sort_cmp_func___item(
        data.clone(),
        Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
    );
    while { let __tmp_x = { let __v = (*blockSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        {
            let __tmp_0 = 0;
            let __tmp_1 = { let __tmp_x = 2; let __tmp_y = { let __v = (*blockSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y };
            *a.lock().unwrap() = Some(__tmp_0);
            *b.lock().unwrap() = Some(__tmp_1);
        };
        while { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } {
        __slices_sym_merge_cmp_func___item(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*blockSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
        { let new_val = b.lock().unwrap().as_ref().unwrap().clone(); *a.lock().unwrap() = Some(new_val); };
        { let __rhs = { let __tmp_x = 2; let __tmp_y = { let __v = (*blockSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let mut guard = b.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        {
        let mut m = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*blockSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })));;
        if { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
            __slices_sym_merge_cmp_func___item(
                data.clone(),
                Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
                Arc::new(Mutex::new(Some({ let __arg_holder = m.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
                Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
                Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
            );;
        }
    }
        { let __rhs = 2; let mut guard = blockSize.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() * __rhs); };
    }
}

pub fn __cmp_is_na_n__int(x: Arc<Mutex<Option<i64>>>) -> bool {
    return { let __bin_x = ({ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }); __bin_x != __bin_x };
}

pub fn __cmp_is_na_n__string(x: Arc<Mutex<Option<String>>>) -> bool {
    return { let __bin_x = ({ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }); __bin_x != __bin_x };
}

pub fn __cmp_is_na_n___celsius(x: Arc<Mutex<Option<Celsius>>>) -> bool {
    return { let __bin_x = ({ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }); __bin_x.clone() != __bin_x };
}

pub fn __cmp_is_na_n__float64(x: Arc<Mutex<Option<f64>>>) -> bool {
    return { let __bin_x = ({ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }); __bin_x != __bin_x };
}

pub fn __slices_pdqsort_cmp_func__ptr__task(data: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Task>>>>>>>, mut a: Arc<Mutex<Option<i64>>>, mut b: Arc<Mutex<Option<i64>>>, mut limit: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Task>>>, Arc<Mutex<Option<Task>>>) -> i64 + Send + Sync>>>>) {
    const maxInsertion: i64 = 12;

    let mut wasBalanced = Arc::new(Mutex::new(Some(true)));let mut wasPartitioned = Arc::new(Mutex::new(Some(true)));
    loop {
        let mut length = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
        if { let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 12; __tmp_x <= __tmp_y } {
        __slices_insertion_sort_cmp_func__ptr__task(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
        return;
    }
        if { let __tmp_x = { let __v = (*limit.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        __slices_heap_sort_cmp_func__ptr__task(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
        return;
    }
        if !{ let __v = (*wasBalanced.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        __slices_break_patterns_cmp_func__ptr__task(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
        { let mut guard = limit.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        let (mut pivot, mut hint) = __slices_choose_pivot_cmp_func__ptr__task(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
        if { let __tmp_x = ({ let __v = (*hint.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __tmp_y = __slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_DECREASING_HINT as i64)))); __tmp_x == __tmp_y } {
        __slices_reverse_range_cmp_func__ptr__task(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
        { let new_val = { let __tmp_x = ({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }); let __tmp_y = ({ let __tmp_x = pivot; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); __tmp_x - __tmp_y }; pivot = new_val; };
        { let new_val = __slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_INCREASING_HINT as i64)))); *hint.lock().unwrap() = Some(new_val); };
    }
        if { let __v = (*wasBalanced.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __v = (*wasPartitioned.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = ({ let __v = (*hint.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __tmp_y = __slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_INCREASING_HINT as i64)))); __tmp_x == __tmp_y } {
        if __slices_partial_insertion_sort_cmp_func__ptr__task(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        ) {
        return;
    }
    }
        if {
            let __go_cond_0 = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = !({
                    let __tmp_x =
                        {
                            let __f: Arc<dyn Fn(Arc<Mutex<Option<Task>>>, Arc<Mutex<Option<Task>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                            (__f)(
                                { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() },
                                { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(pivot) as usize].clone() }
                            )
                        };
                    let __tmp_y = 0;
                    __tmp_x < __tmp_y
                });
                __go_cond_1
            } else {
                false
            }
        } {
        let mut mid = __slices_partition_equal_cmp_func__ptr__task(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some(pivot))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
        { let new_val = mid; *a.lock().unwrap() = Some(new_val); };
        continue
    }
        let (mut mid, mut alreadyPartitioned) = __slices_partition_cmp_func__ptr__task(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some(pivot))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
        { let new_val = alreadyPartitioned; *wasPartitioned.lock().unwrap() = Some(new_val); };
        let (mut leftLen, mut rightLen) = ({ let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = mid; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))); __tuple_value }, { let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = mid; __tmp_x - __tmp_y }))); __tuple_value });
        let mut balanceThreshold = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x / __tmp_y })));
        if { let __tmp_x = { let __v = (*leftLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*rightLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let new_val = { let __tmp_x = { let __v = (*leftLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*balanceThreshold.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y }; *wasBalanced.lock().unwrap() = Some(new_val); };
        __slices_pdqsort_cmp_func__ptr__task(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some(mid))),
            Arc::new(Mutex::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
        { let new_val = { let __tmp_x = mid; let __tmp_y = 1; __tmp_x + __tmp_y }; *a.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = { let __tmp_x = { let __v = (*rightLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*balanceThreshold.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y }; *wasBalanced.lock().unwrap() = Some(new_val); };
        __slices_pdqsort_cmp_func__ptr__task(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __tmp_x = mid; let __tmp_y = 1; __tmp_x + __tmp_y }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
        { let new_val = mid; *b.lock().unwrap() = Some(new_val); };
    }
    }
}

pub fn __slices_pdqsort_cmp_func__int(data: Arc<Mutex<Option<Vec<i64>>>>, mut a: Arc<Mutex<Option<i64>>>, mut b: Arc<Mutex<Option<i64>>>, mut limit: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync>>>>) {
    const maxInsertion: i64 = 12;

    let mut wasBalanced = Arc::new(Mutex::new(Some(true)));let mut wasPartitioned = Arc::new(Mutex::new(Some(true)));
    loop {
        let mut length = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
        if { let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 12; __tmp_x <= __tmp_y } {
        __slices_insertion_sort_cmp_func__int(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
        return;
    }
        if { let __tmp_x = { let __v = (*limit.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        __slices_heap_sort_cmp_func__int(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
        return;
    }
        if !{ let __v = (*wasBalanced.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        __slices_break_patterns_cmp_func__int(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
        { let mut guard = limit.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        let (mut pivot, mut hint) = __slices_choose_pivot_cmp_func__int(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
        if { let __tmp_x = ({ let __v = (*hint.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __tmp_y = __slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_DECREASING_HINT as i64)))); __tmp_x == __tmp_y } {
        __slices_reverse_range_cmp_func__int(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
        { let new_val = { let __tmp_x = ({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }); let __tmp_y = ({ let __tmp_x = pivot; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); __tmp_x - __tmp_y }; pivot = new_val; };
        { let new_val = __slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_INCREASING_HINT as i64)))); *hint.lock().unwrap() = Some(new_val); };
    }
        if { let __v = (*wasBalanced.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __v = (*wasPartitioned.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = ({ let __v = (*hint.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __tmp_y = __slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_INCREASING_HINT as i64)))); __tmp_x == __tmp_y } {
        if __slices_partial_insertion_sort_cmp_func__int(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        ) {
        return;
    }
    }
        if {
            let __go_cond_0 = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = !({
                    let __tmp_x =
                        {
                            let __f: Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                            (__f)(
                                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value }))),
                                Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(pivot) as usize].clone() })))
                            )
                        };
                    let __tmp_y = 0;
                    __tmp_x < __tmp_y
                });
                __go_cond_1
            } else {
                false
            }
        } {
        let mut mid = __slices_partition_equal_cmp_func__int(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some(pivot))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
        { let new_val = mid; *a.lock().unwrap() = Some(new_val); };
        continue
    }
        let (mut mid, mut alreadyPartitioned) = __slices_partition_cmp_func__int(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some(pivot))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
        { let new_val = alreadyPartitioned; *wasPartitioned.lock().unwrap() = Some(new_val); };
        let (mut leftLen, mut rightLen) = ({ let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = mid; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))); __tuple_value }, { let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = mid; __tmp_x - __tmp_y }))); __tuple_value });
        let mut balanceThreshold = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x / __tmp_y })));
        if { let __tmp_x = { let __v = (*leftLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*rightLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let new_val = { let __tmp_x = { let __v = (*leftLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*balanceThreshold.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y }; *wasBalanced.lock().unwrap() = Some(new_val); };
        __slices_pdqsort_cmp_func__int(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some(mid))),
            Arc::new(Mutex::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
        { let new_val = { let __tmp_x = mid; let __tmp_y = 1; __tmp_x + __tmp_y }; *a.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = { let __tmp_x = { let __v = (*rightLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*balanceThreshold.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y }; *wasBalanced.lock().unwrap() = Some(new_val); };
        __slices_pdqsort_cmp_func__int(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __tmp_x = mid; let __tmp_y = 1; __tmp_x + __tmp_y }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
        { let new_val = mid; *b.lock().unwrap() = Some(new_val); };
    }
    }
}

pub fn __slices_pdqsort_ordered___celsius(data: Arc<Mutex<Option<Vec<Celsius>>>>, mut a: Arc<Mutex<Option<i64>>>, mut b: Arc<Mutex<Option<i64>>>, mut limit: Arc<Mutex<Option<i64>>>) {
    const maxInsertion: i64 = 12;

    let mut wasBalanced = Arc::new(Mutex::new(Some(true)));let mut wasPartitioned = Arc::new(Mutex::new(Some(true)));
    loop {
        let mut length = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
        if { let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 12; __tmp_x <= __tmp_y } {
        __slices_insertion_sort_ordered___celsius(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return;
    }
        if { let __tmp_x = { let __v = (*limit.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        __slices_heap_sort_ordered___celsius(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return;
    }
        if !{ let __v = (*wasBalanced.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        __slices_break_patterns_ordered___celsius(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let mut guard = limit.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        let (mut pivot, mut hint) = __slices_choose_pivot_ordered___celsius(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = ({ let __v = (*hint.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __tmp_y = __slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_DECREASING_HINT as i64)))); __tmp_x == __tmp_y } {
        __slices_reverse_range_ordered___celsius(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = { let __tmp_x = ({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }); let __tmp_y = ({ let __tmp_x = pivot; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); __tmp_x - __tmp_y }; pivot = new_val; };
        { let new_val = __slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_INCREASING_HINT as i64)))); *hint.lock().unwrap() = Some(new_val); };
    }
        if { let __v = (*wasBalanced.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __v = (*wasPartitioned.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = ({ let __v = (*hint.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __tmp_y = __slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_INCREASING_HINT as i64)))); __tmp_x == __tmp_y } {
        if __slices_partial_insertion_sort_ordered___celsius(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return;
    }
    }
        if {
            let __go_cond_0 = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = !__cmp__less___celsius(
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value }))),
                    Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(pivot) as usize].clone() })))
                );
                __go_cond_1
            } else {
                false
            }
        } {
        let mut mid = __slices_partition_equal_ordered___celsius(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some(pivot)))
        );
        { let new_val = mid; *a.lock().unwrap() = Some(new_val); };
        continue
    }
        let (mut mid, mut alreadyPartitioned) = __slices_partition_ordered___celsius(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some(pivot)))
        );
        { let new_val = alreadyPartitioned; *wasPartitioned.lock().unwrap() = Some(new_val); };
        let (mut leftLen, mut rightLen) = ({ let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = mid; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))); __tuple_value }, { let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = mid; __tmp_x - __tmp_y }))); __tuple_value });
        let mut balanceThreshold = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x / __tmp_y })));
        if { let __tmp_x = { let __v = (*leftLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*rightLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let new_val = { let __tmp_x = { let __v = (*leftLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*balanceThreshold.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y }; *wasBalanced.lock().unwrap() = Some(new_val); };
        __slices_pdqsort_ordered___celsius(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some(mid))),
            Arc::new(Mutex::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
        );
        { let new_val = { let __tmp_x = mid; let __tmp_y = 1; __tmp_x + __tmp_y }; *a.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = { let __tmp_x = { let __v = (*rightLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*balanceThreshold.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y }; *wasBalanced.lock().unwrap() = Some(new_val); };
        __slices_pdqsort_ordered___celsius(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __tmp_x = mid; let __tmp_y = 1; __tmp_x + __tmp_y }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
        );
        { let new_val = mid; *b.lock().unwrap() = Some(new_val); };
    }
    }
}

pub fn __slices_pdqsort_ordered__int(data: Arc<Mutex<Option<Vec<i64>>>>, mut a: Arc<Mutex<Option<i64>>>, mut b: Arc<Mutex<Option<i64>>>, mut limit: Arc<Mutex<Option<i64>>>) {
    const maxInsertion: i64 = 12;

    let mut wasBalanced = Arc::new(Mutex::new(Some(true)));let mut wasPartitioned = Arc::new(Mutex::new(Some(true)));
    loop {
        let mut length = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
        if { let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 12; __tmp_x <= __tmp_y } {
        __slices_insertion_sort_ordered__int(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return;
    }
        if { let __tmp_x = { let __v = (*limit.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        __slices_heap_sort_ordered__int(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return;
    }
        if !{ let __v = (*wasBalanced.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        __slices_break_patterns_ordered__int(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let mut guard = limit.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        let (mut pivot, mut hint) = __slices_choose_pivot_ordered__int(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = ({ let __v = (*hint.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __tmp_y = __slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_DECREASING_HINT as i64)))); __tmp_x == __tmp_y } {
        __slices_reverse_range_ordered__int(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = { let __tmp_x = ({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }); let __tmp_y = ({ let __tmp_x = pivot; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); __tmp_x - __tmp_y }; pivot = new_val; };
        { let new_val = __slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_INCREASING_HINT as i64)))); *hint.lock().unwrap() = Some(new_val); };
    }
        if { let __v = (*wasBalanced.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __v = (*wasPartitioned.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = ({ let __v = (*hint.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __tmp_y = __slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_INCREASING_HINT as i64)))); __tmp_x == __tmp_y } {
        if __slices_partial_insertion_sort_ordered__int(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return;
    }
    }
        if {
            let __go_cond_0 = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = !__cmp__less__int(
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value }))),
                    Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(pivot) as usize].clone() })))
                );
                __go_cond_1
            } else {
                false
            }
        } {
        let mut mid = __slices_partition_equal_ordered__int(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some(pivot)))
        );
        { let new_val = mid; *a.lock().unwrap() = Some(new_val); };
        continue
    }
        let (mut mid, mut alreadyPartitioned) = __slices_partition_ordered__int(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some(pivot)))
        );
        { let new_val = alreadyPartitioned; *wasPartitioned.lock().unwrap() = Some(new_val); };
        let (mut leftLen, mut rightLen) = ({ let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = mid; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))); __tuple_value }, { let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = mid; __tmp_x - __tmp_y }))); __tuple_value });
        let mut balanceThreshold = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x / __tmp_y })));
        if { let __tmp_x = { let __v = (*leftLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*rightLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let new_val = { let __tmp_x = { let __v = (*leftLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*balanceThreshold.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y }; *wasBalanced.lock().unwrap() = Some(new_val); };
        __slices_pdqsort_ordered__int(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some(mid))),
            Arc::new(Mutex::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
        );
        { let new_val = { let __tmp_x = mid; let __tmp_y = 1; __tmp_x + __tmp_y }; *a.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = { let __tmp_x = { let __v = (*rightLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*balanceThreshold.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y }; *wasBalanced.lock().unwrap() = Some(new_val); };
        __slices_pdqsort_ordered__int(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __tmp_x = mid; let __tmp_y = 1; __tmp_x + __tmp_y }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
        );
        { let new_val = mid; *b.lock().unwrap() = Some(new_val); };
    }
    }
}

pub fn __slices_pdqsort_ordered__float64(data: Arc<Mutex<Option<Vec<f64>>>>, mut a: Arc<Mutex<Option<i64>>>, mut b: Arc<Mutex<Option<i64>>>, mut limit: Arc<Mutex<Option<i64>>>) {
    const maxInsertion: i64 = 12;

    let mut wasBalanced = Arc::new(Mutex::new(Some(true)));let mut wasPartitioned = Arc::new(Mutex::new(Some(true)));
    loop {
        let mut length = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
        if { let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 12; __tmp_x <= __tmp_y } {
        __slices_insertion_sort_ordered__float64(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return;
    }
        if { let __tmp_x = { let __v = (*limit.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        __slices_heap_sort_ordered__float64(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return;
    }
        if !{ let __v = (*wasBalanced.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        __slices_break_patterns_ordered__float64(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let mut guard = limit.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        let (mut pivot, mut hint) = __slices_choose_pivot_ordered__float64(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = ({ let __v = (*hint.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __tmp_y = __slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_DECREASING_HINT as i64)))); __tmp_x == __tmp_y } {
        __slices_reverse_range_ordered__float64(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = { let __tmp_x = ({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }); let __tmp_y = ({ let __tmp_x = pivot; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); __tmp_x - __tmp_y }; pivot = new_val; };
        { let new_val = __slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_INCREASING_HINT as i64)))); *hint.lock().unwrap() = Some(new_val); };
    }
        if { let __v = (*wasBalanced.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __v = (*wasPartitioned.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = ({ let __v = (*hint.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __tmp_y = __slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_INCREASING_HINT as i64)))); __tmp_x == __tmp_y } {
        if __slices_partial_insertion_sort_ordered__float64(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return;
    }
    }
        if {
            let __go_cond_0 = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = !__cmp__less__float64(
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value }))),
                    Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(pivot) as usize].clone() })))
                );
                __go_cond_1
            } else {
                false
            }
        } {
        let mut mid = __slices_partition_equal_ordered__float64(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some(pivot)))
        );
        { let new_val = mid; *a.lock().unwrap() = Some(new_val); };
        continue
    }
        let (mut mid, mut alreadyPartitioned) = __slices_partition_ordered__float64(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some(pivot)))
        );
        { let new_val = alreadyPartitioned; *wasPartitioned.lock().unwrap() = Some(new_val); };
        let (mut leftLen, mut rightLen) = ({ let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = mid; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))); __tuple_value }, { let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = mid; __tmp_x - __tmp_y }))); __tuple_value });
        let mut balanceThreshold = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x / __tmp_y })));
        if { let __tmp_x = { let __v = (*leftLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*rightLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let new_val = { let __tmp_x = { let __v = (*leftLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*balanceThreshold.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y }; *wasBalanced.lock().unwrap() = Some(new_val); };
        __slices_pdqsort_ordered__float64(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some(mid))),
            Arc::new(Mutex::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
        );
        { let new_val = { let __tmp_x = mid; let __tmp_y = 1; __tmp_x + __tmp_y }; *a.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = { let __tmp_x = { let __v = (*rightLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*balanceThreshold.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y }; *wasBalanced.lock().unwrap() = Some(new_val); };
        __slices_pdqsort_ordered__float64(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __tmp_x = mid; let __tmp_y = 1; __tmp_x + __tmp_y }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
        );
        { let new_val = mid; *b.lock().unwrap() = Some(new_val); };
    }
    }
}

pub fn __slices_pdqsort_ordered__string(data: Arc<Mutex<Option<Vec<String>>>>, mut a: Arc<Mutex<Option<i64>>>, mut b: Arc<Mutex<Option<i64>>>, mut limit: Arc<Mutex<Option<i64>>>) {
    const maxInsertion: i64 = 12;

    let mut wasBalanced = Arc::new(Mutex::new(Some(true)));let mut wasPartitioned = Arc::new(Mutex::new(Some(true)));
    loop {
        let mut length = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
        if { let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 12; __tmp_x <= __tmp_y } {
        __slices_insertion_sort_ordered__string(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return;
    }
        if { let __tmp_x = { let __v = (*limit.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        __slices_heap_sort_ordered__string(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return;
    }
        if !{ let __v = (*wasBalanced.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        __slices_break_patterns_ordered__string(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let mut guard = limit.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        let (mut pivot, mut hint) = __slices_choose_pivot_ordered__string(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = ({ let __v = (*hint.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __tmp_y = __slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_DECREASING_HINT as i64)))); __tmp_x == __tmp_y } {
        __slices_reverse_range_ordered__string(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = { let __tmp_x = ({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }); let __tmp_y = ({ let __tmp_x = pivot; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); __tmp_x - __tmp_y }; pivot = new_val; };
        { let new_val = __slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_INCREASING_HINT as i64)))); *hint.lock().unwrap() = Some(new_val); };
    }
        if { let __v = (*wasBalanced.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __v = (*wasPartitioned.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = ({ let __v = (*hint.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __tmp_y = __slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_INCREASING_HINT as i64)))); __tmp_x == __tmp_y } {
        if __slices_partial_insertion_sort_ordered__string(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return;
    }
    }
        if {
            let __go_cond_0 = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = !__cmp__less__string(
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value }))),
                    Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(pivot) as usize].clone() })))
                );
                __go_cond_1
            } else {
                false
            }
        } {
        let mut mid = __slices_partition_equal_ordered__string(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some(pivot)))
        );
        { let new_val = mid; *a.lock().unwrap() = Some(new_val); };
        continue
    }
        let (mut mid, mut alreadyPartitioned) = __slices_partition_ordered__string(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some(pivot)))
        );
        { let new_val = alreadyPartitioned; *wasPartitioned.lock().unwrap() = Some(new_val); };
        let (mut leftLen, mut rightLen) = ({ let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = mid; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))); __tuple_value }, { let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = mid; __tmp_x - __tmp_y }))); __tuple_value });
        let mut balanceThreshold = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x / __tmp_y })));
        if { let __tmp_x = { let __v = (*leftLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*rightLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let new_val = { let __tmp_x = { let __v = (*leftLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*balanceThreshold.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y }; *wasBalanced.lock().unwrap() = Some(new_val); };
        __slices_pdqsort_ordered__string(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some(mid))),
            Arc::new(Mutex::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
        );
        { let new_val = { let __tmp_x = mid; let __tmp_y = 1; __tmp_x + __tmp_y }; *a.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = { let __tmp_x = { let __v = (*rightLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*balanceThreshold.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y }; *wasBalanced.lock().unwrap() = Some(new_val); };
        __slices_pdqsort_ordered__string(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __tmp_x = mid; let __tmp_y = 1; __tmp_x + __tmp_y }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
        );
        { let new_val = mid; *b.lock().unwrap() = Some(new_val); };
    }
    }
}

pub fn __slices_is_na_n__int(x: Arc<Mutex<Option<i64>>>) -> bool {
    return { let __bin_x = ({ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }); __bin_x != __bin_x };
}

pub fn __slices__append_seq__slice_string__string(mut s: Arc<Mutex<Option<Vec<String>>>>, seq: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<String>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>>>>) -> Arc<Mutex<Option<Vec<String>>>> {
    let mut s = Arc::new(Mutex::new((*s.lock().unwrap()).clone()));
    let mut __rf_done_5: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    let mut __rf_result_5_0: Arc<Mutex<Option<[Vec<String>; 1]>>> = Arc::new(Mutex::new(Some(std::array::from_fn::<_, 1, _>(|_| Vec::nil()))));
    let mut s_closure_clone = s.clone(); {
        let __f: Arc<dyn Fn(Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<String>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync> = seq.lock().unwrap().as_ref().unwrap().clone();
        (__f)(
            Arc::new(Mutex::new(Some(Arc::new(move |__rf_param_5_0: Arc<Mutex<Option<String>>>| -> bool {
        let mut s_closure_clone = s_closure_clone.clone();
        let mut v = { let __owned = __rf_param_5_0.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = s_closure_clone.clone(); let __append_value = ({ let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *s_closure_clone.lock().unwrap() = __slice_header; };
        true
    }) as Arc<dyn Fn(Arc<Mutex<Option<String>>>) -> bool + Send + Sync>)))
        )
    };
    if { let __v = (*__rf_done_5.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = __rf_result_5_0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() })));
    }
    return s.clone();
}

pub fn __slices__append_seq__slice_int__int(mut s: Arc<Mutex<Option<Vec<i64>>>>, seq: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<i64>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>>>>) -> Arc<Mutex<Option<Vec<i64>>>> {
    let mut s = Arc::new(Mutex::new((*s.lock().unwrap()).clone()));
    let mut __rf_done_6: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    let mut __rf_result_6_0: Arc<Mutex<Option<[Vec<i64>; 1]>>> = Arc::new(Mutex::new(Some(std::array::from_fn::<_, 1, _>(|_| Vec::nil()))));
    let mut s_closure_clone = s.clone(); {
        let __f: Arc<dyn Fn(Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<i64>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync> = seq.lock().unwrap().as_ref().unwrap().clone();
        (__f)(
            Arc::new(Mutex::new(Some(Arc::new(move |__rf_param_6_0: Arc<Mutex<Option<i64>>>| -> bool {
        let mut s_closure_clone = s_closure_clone.clone();
        let mut v = { let __owned = __rf_param_6_0.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = s_closure_clone.clone(); let __append_value = ({ let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }); let mut __appended: Vec<i64> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *s_closure_clone.lock().unwrap() = __slice_header; };
        true
    }) as Arc<dyn Fn(Arc<Mutex<Option<i64>>>) -> bool + Send + Sync>)))
        )
    };
    if { let __v = (*__rf_done_6.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = __rf_result_6_0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() })));
    }
    return s.clone();
}

pub fn __slices_rotate_left__int(s: Arc<Mutex<Option<Vec<i64>>>>, r: Arc<Mutex<Option<i64>>>) {
    __slices__reverse__slice_int__int(Arc::new(Mutex::new(Some({
        let __seq_holder = s.clone();
        let __seq_guard = __seq_holder.lock().unwrap();
        let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
        let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
        std::mem::drop(__seq_guard);
        let __low = 0;
        let __high = ({ let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __max = __source_cap;
        __seq.reslice3_zeroed(__low, __high, __max)
    }))));
    __slices__reverse__slice_int__int(Arc::new(Mutex::new(Some({
        let __seq_holder = s.clone();
        let __seq_guard = __seq_holder.lock().unwrap();
        let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
        let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
        std::mem::drop(__seq_guard);
        let __low = ({ let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __high = __seq.len();
        let __max = __source_cap;
        __seq.reslice3_zeroed(__low, __high, __max)
    }))));
    __slices__reverse__slice_int__int(s.clone());
}

pub fn __slices_insertion_sort_cmp_func___item(data: Arc<Mutex<Option<Vec<Item>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Item>>>, Arc<Mutex<Option<Item>>>) -> i64 + Send + Sync>>>>) {
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut j = { let __owned = i.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    while {
        let __go_cond_0 = { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y };
        if __go_cond_0 {
            let __go_cond_1 = ({
                let __tmp_x =
                    {
                        let __f: Arc<dyn Fn(Arc<Mutex<Option<Item>>>, Arc<Mutex<Option<Item>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                        (__f)(
                            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value })))
                        )
                    };
                let __tmp_y = 0;
                __tmp_x < __tmp_y
            });
            __go_cond_1
        } else {
            false
        }
    } {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

pub fn __slices_insertion_sort_cmp_func__ptr__task(data: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Task>>>>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Task>>>, Arc<Mutex<Option<Task>>>) -> i64 + Send + Sync>>>>) {
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut j = { let __owned = i.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    while {
        let __go_cond_0 = { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y };
        if __go_cond_0 {
            let __go_cond_1 = ({
                let __tmp_x =
                    {
                        let __f: Arc<dyn Fn(Arc<Mutex<Option<Task>>>, Arc<Mutex<Option<Task>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                        (__f)(
                            { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() },
                            { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }
                        )
                    };
                let __tmp_y = 0;
                __tmp_x < __tmp_y
            });
            __go_cond_1
        } else {
            false
        }
    } {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

pub fn __slices_insertion_sort_cmp_func__int(data: Arc<Mutex<Option<Vec<i64>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync>>>>) {
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut j = { let __owned = i.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    while {
        let __go_cond_0 = { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y };
        if __go_cond_0 {
            let __go_cond_1 = ({
                let __tmp_x =
                    {
                        let __f: Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                        (__f)(
                            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value })))
                        )
                    };
                let __tmp_y = 0;
                __tmp_x < __tmp_y
            });
            __go_cond_1
        } else {
            false
        }
    } {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

pub fn __slices_sym_merge_cmp_func___item(data: Arc<Mutex<Option<Vec<Item>>>>, a: Arc<Mutex<Option<i64>>>, m: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Item>>>, Arc<Mutex<Option<Item>>>) -> i64 + Send + Sync>>>>) {
    if { let __tmp_x = { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = 1; __tmp_x == __tmp_y } {
        let mut i = { let __owned = m.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        let mut j = { let __owned = b.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut h = Arc::new(Mutex::new(Some((({ let __tmp_x = (*Arc::new(Mutex::new(Some((({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x >> __tmp_y })) as i64)));
        if {
            let __tmp_x =
                {
                    let __f: Arc<dyn Fn(Arc<Mutex<Option<Item>>>, Arc<Mutex<Option<Item>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                    (__f)(
                        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
                    )
                };
            let __tmp_y = 0;
            __tmp_x < __tmp_y
        } {
        { let new_val = { let __tmp_x = { let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }; *i.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = h.lock().unwrap().as_ref().unwrap().clone(); *j.lock().unwrap() = Some(new_val); };
    }
    }
        let mut k = { let __owned = a.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    while { let __tmp_x = { let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }; __tmp_x < __tmp_y } {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = k.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        return;
    }
    if { let __tmp_x = { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = 1; __tmp_x == __tmp_y } {
        let mut i = { let __owned = a.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        let mut j = { let __owned = m.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut h = Arc::new(Mutex::new(Some((({ let __tmp_x = (*Arc::new(Mutex::new(Some((({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x >> __tmp_y })) as i64)));
        if !({
            let __tmp_x =
                {
                    let __f: Arc<dyn Fn(Arc<Mutex<Option<Item>>>, Arc<Mutex<Option<Item>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                    (__f)(
                        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
                    )
                };
            let __tmp_y = 0;
            __tmp_x < __tmp_y
        }) {
        { let new_val = { let __tmp_x = { let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }; *i.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = h.lock().unwrap().as_ref().unwrap().clone(); *j.lock().unwrap() = Some(new_val); };
    }
    }
        let mut k = { let __owned = m.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    while { let __tmp_x = { let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = k.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        return;
    }
    let mut mid = Arc::new(Mutex::new(Some((({ let __tmp_x = (*Arc::new(Mutex::new(Some((({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x >> __tmp_y })) as i64)));
    let mut n = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*mid.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })));
    let mut start: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));let mut r: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    if { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*mid.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        { let new_val = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; *start.lock().unwrap() = Some(new_val); };
        { let new_val = mid.lock().unwrap().as_ref().unwrap().clone(); *r.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = a.lock().unwrap().as_ref().unwrap().clone(); *start.lock().unwrap() = Some(new_val); };
        { let new_val = m.lock().unwrap().as_ref().unwrap().clone(); *r.lock().unwrap() = Some(new_val); };
    }
    let mut p = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut c = Arc::new(Mutex::new(Some((({ let __tmp_x = (*Arc::new(Mutex::new(Some((({ let __tmp_x = { let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x >> __tmp_y })) as i64)));
        if !({
            let __tmp_x =
                {
                    let __f: Arc<dyn Fn(Arc<Mutex<Option<Item>>>, Arc<Mutex<Option<Item>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                    (__f)(
                        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value }))),
                        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
                    )
                };
            let __tmp_y = 0;
            __tmp_x < __tmp_y
        }) {
        { let new_val = { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }; *start.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = c.lock().unwrap().as_ref().unwrap().clone(); *r.lock().unwrap() = Some(new_val); };
    }
    }
    let mut end = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
    if { let __tmp_x = { let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } && { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*end.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        __slices_rotate_cmp_func___item(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = start.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = m.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = end.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
    }
    if { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } && { let __tmp_x = { let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*mid.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        __slices_sym_merge_cmp_func___item(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = start.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = mid.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
    }
    if { let __tmp_x = { let __v = (*mid.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*end.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } && { let __tmp_x = { let __v = (*end.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        __slices_sym_merge_cmp_func___item(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = mid.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = end.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
    }
}

pub fn __slices_heap_sort_cmp_func__ptr__task(data: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Task>>>>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Task>>>, Arc<Mutex<Option<Task>>>) -> i64 + Send + Sync>>>>) {
    let mut first = { let __owned = a.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    let mut lo = Arc::new(Mutex::new(Some(0i64)));
    let mut hi = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }); let __tmp_y = 2; __tmp_x / __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        __slices_sift_down_cmp_func__ptr__task(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = hi.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        __slices_sift_down_cmp_func__ptr__task(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = lo.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

pub fn __slices_heap_sort_cmp_func__int(data: Arc<Mutex<Option<Vec<i64>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync>>>>) {
    let mut first = { let __owned = a.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    let mut lo = Arc::new(Mutex::new(Some(0i64)));
    let mut hi = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }); let __tmp_y = 2; __tmp_x / __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        __slices_sift_down_cmp_func__int(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = hi.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        __slices_sift_down_cmp_func__int(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = lo.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

pub fn __slices_break_patterns_cmp_func__ptr__task(data: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Task>>>>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Task>>>, Arc<Mutex<Option<Task>>>) -> i64 + Send + Sync>>>>) {
    let mut length = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
    if { let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x >= __tmp_y } {
        let mut random = Arc::new(Mutex::new(Some(__slices_xorshift(Arc::new(Mutex::new(Some(({ let __v = (*length.lock().unwrap().as_ref().unwrap()); __v }) as u64)))))));
        let mut modulus = __slices_next_power_of_two(Arc::new(Mutex::new(Some({ let __arg_holder = length.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let __go_binary_0 = (*a.lock().unwrap().as_ref().unwrap());
let __go_binary_1 = (*length.lock().unwrap().as_ref().unwrap());
let __go_binary_2 = 4;
let __go_binary_3 = __go_binary_1 / __go_binary_2;
let __go_binary_4 = 2;
let __go_binary_5 = __go_binary_3 * __go_binary_4;
let __go_binary_6 = __go_binary_0 + __go_binary_5;
let __go_binary_7 = 1;
let __go_binary_8 = __go_binary_6 - __go_binary_7;
let mut idx = Arc::new(Mutex::new(Some(__go_binary_8)));
    while { let __tmp_x = { let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = ({ let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x / __tmp_y }); let __tmp_y = 2; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y }; __tmp_x <= __tmp_y } {
        let mut other = Arc::new(Mutex::new(Some((({ let __tmp_x = (*Arc::new(Mutex::new(Some((*random.lock().unwrap().as_mut().unwrap()).next() as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = modulus; let __tmp_y = 1 as u64; __tmp_x - __tmp_y }); __tmp_x & __tmp_y })) as i64)));
        if { let __tmp_x = { let __v = (*other.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        { let __rhs = (*length.lock().unwrap().as_ref().unwrap()); let mut guard = other.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*other.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*other.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = idx.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
}

pub fn __slices_break_patterns_cmp_func__int(data: Arc<Mutex<Option<Vec<i64>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync>>>>) {
    let mut length = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
    if { let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x >= __tmp_y } {
        let mut random = Arc::new(Mutex::new(Some(__slices_xorshift(Arc::new(Mutex::new(Some(({ let __v = (*length.lock().unwrap().as_ref().unwrap()); __v }) as u64)))))));
        let mut modulus = __slices_next_power_of_two(Arc::new(Mutex::new(Some({ let __arg_holder = length.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let __go_binary_0 = (*a.lock().unwrap().as_ref().unwrap());
let __go_binary_1 = (*length.lock().unwrap().as_ref().unwrap());
let __go_binary_2 = 4;
let __go_binary_3 = __go_binary_1 / __go_binary_2;
let __go_binary_4 = 2;
let __go_binary_5 = __go_binary_3 * __go_binary_4;
let __go_binary_6 = __go_binary_0 + __go_binary_5;
let __go_binary_7 = 1;
let __go_binary_8 = __go_binary_6 - __go_binary_7;
let mut idx = Arc::new(Mutex::new(Some(__go_binary_8)));
    while { let __tmp_x = { let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = ({ let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x / __tmp_y }); let __tmp_y = 2; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y }; __tmp_x <= __tmp_y } {
        let mut other = Arc::new(Mutex::new(Some((({ let __tmp_x = (*Arc::new(Mutex::new(Some((*random.lock().unwrap().as_mut().unwrap()).next() as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = modulus; let __tmp_y = 1 as u64; __tmp_x - __tmp_y }); __tmp_x & __tmp_y })) as i64)));
        if { let __tmp_x = { let __v = (*other.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        { let __rhs = (*length.lock().unwrap().as_ref().unwrap()); let mut guard = other.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*other.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*other.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = idx.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
}

pub fn __slices_choose_pivot_cmp_func__ptr__task(data: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Task>>>>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Task>>>, Arc<Mutex<Option<Task>>>) -> i64 + Send + Sync>>>>) -> (i64, Arc<Mutex<Option<__slices_sortedHint>>>) {
    let mut pivot: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut hint: Arc<Mutex<Option<__slices_sortedHint>>> = Arc::new(Mutex::new(Some(Default::default())));

    const shortestNinther: i64 = 50;
const maxSwaps: i64 = 4 * 3;

    let mut l = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
    let mut swaps: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x / __tmp_y }; let __tmp_y = 1; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x / __tmp_y }; let __tmp_y = 2; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));let mut k = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x / __tmp_y }; let __tmp_y = 3; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));
    if { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x >= __tmp_y } {
        if { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 50; __tmp_x >= __tmp_y } {
        { let new_val = __slices_median_adjacent_cmp_func__ptr__task(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            swaps.clone(),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        ); *i.lock().unwrap() = Some(new_val); };
        { let new_val = __slices_median_adjacent_cmp_func__ptr__task(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            swaps.clone(),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        ); *j.lock().unwrap() = Some(new_val); };
        { let new_val = __slices_median_adjacent_cmp_func__ptr__task(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            swaps.clone(),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        ); *k.lock().unwrap() = Some(new_val); };
    }
        { let new_val = __slices_median_cmp_func__ptr__task(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            swaps.clone(),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        ); *j.lock().unwrap() = Some(new_val); };
    }
    { let _switch_val = { let __v = (*swaps.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (0) {
            return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some(__slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_INCREASING_HINT as i64))))))));
        } else if _switch_val == (12) {
            return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some(__slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_DECREASING_HINT as i64))))))));
        } else {
            return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some(__slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_UNKNOWN_HINT as i64))))))));
        }
    }
}

pub fn __slices_choose_pivot_cmp_func__int(data: Arc<Mutex<Option<Vec<i64>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync>>>>) -> (i64, Arc<Mutex<Option<__slices_sortedHint>>>) {
    let mut pivot: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut hint: Arc<Mutex<Option<__slices_sortedHint>>> = Arc::new(Mutex::new(Some(Default::default())));

    const shortestNinther: i64 = 50;
const maxSwaps: i64 = 4 * 3;

    let mut l = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
    let mut swaps: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x / __tmp_y }; let __tmp_y = 1; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x / __tmp_y }; let __tmp_y = 2; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));let mut k = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x / __tmp_y }; let __tmp_y = 3; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));
    if { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x >= __tmp_y } {
        if { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 50; __tmp_x >= __tmp_y } {
        { let new_val = __slices_median_adjacent_cmp_func__int(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            swaps.clone(),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        ); *i.lock().unwrap() = Some(new_val); };
        { let new_val = __slices_median_adjacent_cmp_func__int(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            swaps.clone(),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        ); *j.lock().unwrap() = Some(new_val); };
        { let new_val = __slices_median_adjacent_cmp_func__int(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            swaps.clone(),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        ); *k.lock().unwrap() = Some(new_val); };
    }
        { let new_val = __slices_median_cmp_func__int(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            swaps.clone(),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        ); *j.lock().unwrap() = Some(new_val); };
    }
    { let _switch_val = { let __v = (*swaps.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (0) {
            return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some(__slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_INCREASING_HINT as i64))))))));
        } else if _switch_val == (12) {
            return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some(__slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_DECREASING_HINT as i64))))))));
        } else {
            return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some(__slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_UNKNOWN_HINT as i64))))))));
        }
    }
}

pub fn __slices_reverse_range_cmp_func__ptr__task(data: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Task>>>>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Task>>>, Arc<Mutex<Option<Task>>>) -> i64 + Send + Sync>>>>) {
    let mut i = { let __owned = a.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

pub fn __slices_reverse_range_cmp_func__int(data: Arc<Mutex<Option<Vec<i64>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync>>>>) {
    let mut i = { let __owned = a.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

pub fn __slices_partial_insertion_sort_cmp_func__ptr__task(data: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Task>>>>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Task>>>, Arc<Mutex<Option<Task>>>) -> i64 + Send + Sync>>>>) -> bool {
    const maxSteps: i64 = 5;
const shortestShifting: i64 = 50;

    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })));
    let mut j = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 5; __tmp_x < __tmp_y } {
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = !({
                    let __tmp_x =
                        {
                            let __f: Arc<dyn Fn(Arc<Mutex<Option<Task>>>, Arc<Mutex<Option<Task>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                            (__f)(
                                { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() },
                                { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }
                            )
                        };
                    let __tmp_y = 0;
                    __tmp_x < __tmp_y
                });
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        return true;
    }
        if { let __tmp_x = { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = 50; __tmp_x < __tmp_y } {
        return false;
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        if { let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = 2; __tmp_x >= __tmp_y } {
        let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x >= __tmp_y } {
        if !({
            let __tmp_x =
                {
                    let __f: Arc<dyn Fn(Arc<Mutex<Option<Task>>>, Arc<Mutex<Option<Task>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                    (__f)(
                        { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() },
                        { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }
                    )
                };
            let __tmp_y = 0;
            __tmp_x < __tmp_y
        }) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    }
        if { let __tmp_x = { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = 2; __tmp_x >= __tmp_y } {
        let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        if !({
            let __tmp_x =
                {
                    let __f: Arc<dyn Fn(Arc<Mutex<Option<Task>>>, Arc<Mutex<Option<Task>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                    (__f)(
                        { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() },
                        { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }
                    )
                };
            let __tmp_y = 0;
            __tmp_x < __tmp_y
        }) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    false
}

pub fn __slices_partial_insertion_sort_cmp_func__int(data: Arc<Mutex<Option<Vec<i64>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync>>>>) -> bool {
    const maxSteps: i64 = 5;
const shortestShifting: i64 = 50;

    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })));
    let mut j = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 5; __tmp_x < __tmp_y } {
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = !({
                    let __tmp_x =
                        {
                            let __f: Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                            (__f)(
                                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value })))
                            )
                        };
                    let __tmp_y = 0;
                    __tmp_x < __tmp_y
                });
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        return true;
    }
        if { let __tmp_x = { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = 50; __tmp_x < __tmp_y } {
        return false;
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        if { let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = 2; __tmp_x >= __tmp_y } {
        let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x >= __tmp_y } {
        if !({
            let __tmp_x =
                {
                    let __f: Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                    (__f)(
                        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value })))
                    )
                };
            let __tmp_y = 0;
            __tmp_x < __tmp_y
        }) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    }
        if { let __tmp_x = { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = 2; __tmp_x >= __tmp_y } {
        let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        if !({
            let __tmp_x =
                {
                    let __f: Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                    (__f)(
                        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value })))
                    )
                };
            let __tmp_y = 0;
            __tmp_x < __tmp_y
        }) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    false
}

pub fn __slices_partition_equal_cmp_func__ptr__task(data: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Task>>>>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, pivot: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Task>>>, Arc<Mutex<Option<Task>>>) -> i64 + Send + Sync>>>>) -> i64 {
    let mut newpivot: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));

    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ({ let __v = (*pivot.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*pivot.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    let (mut i, mut j) = ({ let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }))); __tuple_value }, { let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }))); __tuple_value });
    loop {
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = !({
                    let __tmp_x =
                        {
                            let __f: Arc<dyn Fn(Arc<Mutex<Option<Task>>>, Arc<Mutex<Option<Task>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                            (__f)(
                                { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() },
                                { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }
                            )
                        };
                    let __tmp_y = 0;
                    __tmp_x < __tmp_y
                });
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = ({
                    let __tmp_x =
                        {
                            let __f: Arc<dyn Fn(Arc<Mutex<Option<Task>>>, Arc<Mutex<Option<Task>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                            (__f)(
                                { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() },
                                { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }
                            )
                        };
                    let __tmp_y = 0;
                    __tmp_x < __tmp_y
                });
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    return { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn __slices_partition_equal_cmp_func__int(data: Arc<Mutex<Option<Vec<i64>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, pivot: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync>>>>) -> i64 {
    let mut newpivot: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));

    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ({ let __v = (*pivot.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*pivot.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    let (mut i, mut j) = ({ let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }))); __tuple_value }, { let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }))); __tuple_value });
    loop {
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = !({
                    let __tmp_x =
                        {
                            let __f: Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                            (__f)(
                                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
                            )
                        };
                    let __tmp_y = 0;
                    __tmp_x < __tmp_y
                });
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = ({
                    let __tmp_x =
                        {
                            let __f: Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                            (__f)(
                                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
                            )
                        };
                    let __tmp_y = 0;
                    __tmp_x < __tmp_y
                });
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    return { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn __slices_partition_cmp_func__ptr__task(data: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Task>>>>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, pivot: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Task>>>, Arc<Mutex<Option<Task>>>) -> i64 + Send + Sync>>>>) -> (i64, bool) {
    let mut newpivot: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut alreadyPartitioned: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ({ let __v = (*pivot.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*pivot.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    let (mut i, mut j) = ({ let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }))); __tuple_value }, { let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }))); __tuple_value });
    while {
        let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
        if __go_cond_0 {
            let __go_cond_1 = ({
                let __tmp_x =
                    {
                        let __f: Arc<dyn Fn(Arc<Mutex<Option<Task>>>, Arc<Mutex<Option<Task>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                        (__f)(
                            { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() },
                            { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }
                        )
                    };
                let __tmp_y = 0;
                __tmp_x < __tmp_y
            });
            __go_cond_1
        } else {
            false
        }
    } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    while {
        let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
        if __go_cond_0 {
            let __go_cond_1 = !({
                let __tmp_x =
                    {
                        let __f: Arc<dyn Fn(Arc<Mutex<Option<Task>>>, Arc<Mutex<Option<Task>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                        (__f)(
                            { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() },
                            { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }
                        )
                    };
                let __tmp_y = 0;
                __tmp_x < __tmp_y
            });
            __go_cond_1
        } else {
            false
        }
    } {
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, true);
    }
    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    loop {
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = ({
                    let __tmp_x =
                        {
                            let __f: Arc<dyn Fn(Arc<Mutex<Option<Task>>>, Arc<Mutex<Option<Task>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                            (__f)(
                                { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() },
                                { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }
                            )
                        };
                    let __tmp_y = 0;
                    __tmp_x < __tmp_y
                });
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = !({
                    let __tmp_x =
                        {
                            let __f: Arc<dyn Fn(Arc<Mutex<Option<Task>>>, Arc<Mutex<Option<Task>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                            (__f)(
                                { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() },
                                { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }
                            )
                        };
                    let __tmp_y = 0;
                    __tmp_x < __tmp_y
                });
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, false);
}

pub fn __slices_partition_cmp_func__int(data: Arc<Mutex<Option<Vec<i64>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, pivot: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync>>>>) -> (i64, bool) {
    let mut newpivot: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut alreadyPartitioned: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ({ let __v = (*pivot.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*pivot.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    let (mut i, mut j) = ({ let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }))); __tuple_value }, { let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }))); __tuple_value });
    while {
        let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
        if __go_cond_0 {
            let __go_cond_1 = ({
                let __tmp_x =
                    {
                        let __f: Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                        (__f)(
                            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
                        )
                    };
                let __tmp_y = 0;
                __tmp_x < __tmp_y
            });
            __go_cond_1
        } else {
            false
        }
    } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    while {
        let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
        if __go_cond_0 {
            let __go_cond_1 = !({
                let __tmp_x =
                    {
                        let __f: Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                        (__f)(
                            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
                        )
                    };
                let __tmp_y = 0;
                __tmp_x < __tmp_y
            });
            __go_cond_1
        } else {
            false
        }
    } {
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, true);
    }
    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    loop {
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = ({
                    let __tmp_x =
                        {
                            let __f: Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                            (__f)(
                                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
                            )
                        };
                    let __tmp_y = 0;
                    __tmp_x < __tmp_y
                });
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = !({
                    let __tmp_x =
                        {
                            let __f: Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                            (__f)(
                                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
                            )
                        };
                    let __tmp_y = 0;
                    __tmp_x < __tmp_y
                });
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, false);
}

pub fn __slices_insertion_sort_ordered___celsius(data: Arc<Mutex<Option<Vec<Celsius>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>) {
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut j = { let __owned = i.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    while {
        let __go_cond_0 = { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y };
        if __go_cond_0 {
            let __go_cond_1 = __cmp__less___celsius(
                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value })))
            );
            __go_cond_1
        } else {
            false
        }
    } {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

pub fn __slices_insertion_sort_ordered__int(data: Arc<Mutex<Option<Vec<i64>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>) {
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut j = { let __owned = i.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    while {
        let __go_cond_0 = { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y };
        if __go_cond_0 {
            let __go_cond_1 = __cmp__less__int(
                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value })))
            );
            __go_cond_1
        } else {
            false
        }
    } {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

pub fn __slices_insertion_sort_ordered__float64(data: Arc<Mutex<Option<Vec<f64>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>) {
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut j = { let __owned = i.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    while {
        let __go_cond_0 = { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y };
        if __go_cond_0 {
            let __go_cond_1 = __cmp__less__float64(
                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value })))
            );
            __go_cond_1
        } else {
            false
        }
    } {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

pub fn __slices_insertion_sort_ordered__string(data: Arc<Mutex<Option<Vec<String>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>) {
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut j = { let __owned = i.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    while {
        let __go_cond_0 = { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y };
        if __go_cond_0 {
            let __go_cond_1 = __cmp__less__string(
                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value })))
            );
            __go_cond_1
        } else {
            false
        }
    } {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

pub fn __slices_heap_sort_ordered___celsius(data: Arc<Mutex<Option<Vec<Celsius>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>) {
    let mut first = { let __owned = a.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    let mut lo = Arc::new(Mutex::new(Some(0i64)));
    let mut hi = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }); let __tmp_y = 2; __tmp_x / __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        __slices_sift_down_ordered___celsius(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = hi.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
        );
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        __slices_sift_down_ordered___celsius(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = lo.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
        );
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

pub fn __slices_heap_sort_ordered__int(data: Arc<Mutex<Option<Vec<i64>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>) {
    let mut first = { let __owned = a.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    let mut lo = Arc::new(Mutex::new(Some(0i64)));
    let mut hi = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }); let __tmp_y = 2; __tmp_x / __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        __slices_sift_down_ordered__int(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = hi.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
        );
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        __slices_sift_down_ordered__int(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = lo.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
        );
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

pub fn __slices_heap_sort_ordered__float64(data: Arc<Mutex<Option<Vec<f64>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>) {
    let mut first = { let __owned = a.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    let mut lo = Arc::new(Mutex::new(Some(0i64)));
    let mut hi = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }); let __tmp_y = 2; __tmp_x / __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        __slices_sift_down_ordered__float64(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = hi.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
        );
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        __slices_sift_down_ordered__float64(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = lo.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
        );
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

pub fn __slices_heap_sort_ordered__string(data: Arc<Mutex<Option<Vec<String>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>) {
    let mut first = { let __owned = a.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    let mut lo = Arc::new(Mutex::new(Some(0i64)));
    let mut hi = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }); let __tmp_y = 2; __tmp_x / __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        __slices_sift_down_ordered__string(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = hi.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
        );
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        __slices_sift_down_ordered__string(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = lo.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
        );
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

pub fn __slices_break_patterns_ordered___celsius(data: Arc<Mutex<Option<Vec<Celsius>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>) {
    let mut length = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
    if { let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x >= __tmp_y } {
        let mut random = Arc::new(Mutex::new(Some(__slices_xorshift(Arc::new(Mutex::new(Some(({ let __v = (*length.lock().unwrap().as_ref().unwrap()); __v }) as u64)))))));
        let mut modulus = __slices_next_power_of_two(Arc::new(Mutex::new(Some({ let __arg_holder = length.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let __go_binary_0 = (*a.lock().unwrap().as_ref().unwrap());
let __go_binary_1 = (*length.lock().unwrap().as_ref().unwrap());
let __go_binary_2 = 4;
let __go_binary_3 = __go_binary_1 / __go_binary_2;
let __go_binary_4 = 2;
let __go_binary_5 = __go_binary_3 * __go_binary_4;
let __go_binary_6 = __go_binary_0 + __go_binary_5;
let __go_binary_7 = 1;
let __go_binary_8 = __go_binary_6 - __go_binary_7;
let mut idx = Arc::new(Mutex::new(Some(__go_binary_8)));
    while { let __tmp_x = { let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = ({ let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x / __tmp_y }); let __tmp_y = 2; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y }; __tmp_x <= __tmp_y } {
        let mut other = Arc::new(Mutex::new(Some((({ let __tmp_x = (*Arc::new(Mutex::new(Some((*random.lock().unwrap().as_mut().unwrap()).next() as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = modulus; let __tmp_y = 1 as u64; __tmp_x - __tmp_y }); __tmp_x & __tmp_y })) as i64)));
        if { let __tmp_x = { let __v = (*other.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        { let __rhs = (*length.lock().unwrap().as_ref().unwrap()); let mut guard = other.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*other.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*other.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = idx.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
}

pub fn __slices_break_patterns_ordered__int(data: Arc<Mutex<Option<Vec<i64>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>) {
    let mut length = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
    if { let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x >= __tmp_y } {
        let mut random = Arc::new(Mutex::new(Some(__slices_xorshift(Arc::new(Mutex::new(Some(({ let __v = (*length.lock().unwrap().as_ref().unwrap()); __v }) as u64)))))));
        let mut modulus = __slices_next_power_of_two(Arc::new(Mutex::new(Some({ let __arg_holder = length.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let __go_binary_0 = (*a.lock().unwrap().as_ref().unwrap());
let __go_binary_1 = (*length.lock().unwrap().as_ref().unwrap());
let __go_binary_2 = 4;
let __go_binary_3 = __go_binary_1 / __go_binary_2;
let __go_binary_4 = 2;
let __go_binary_5 = __go_binary_3 * __go_binary_4;
let __go_binary_6 = __go_binary_0 + __go_binary_5;
let __go_binary_7 = 1;
let __go_binary_8 = __go_binary_6 - __go_binary_7;
let mut idx = Arc::new(Mutex::new(Some(__go_binary_8)));
    while { let __tmp_x = { let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = ({ let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x / __tmp_y }); let __tmp_y = 2; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y }; __tmp_x <= __tmp_y } {
        let mut other = Arc::new(Mutex::new(Some((({ let __tmp_x = (*Arc::new(Mutex::new(Some((*random.lock().unwrap().as_mut().unwrap()).next() as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = modulus; let __tmp_y = 1 as u64; __tmp_x - __tmp_y }); __tmp_x & __tmp_y })) as i64)));
        if { let __tmp_x = { let __v = (*other.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        { let __rhs = (*length.lock().unwrap().as_ref().unwrap()); let mut guard = other.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*other.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*other.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = idx.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
}

pub fn __slices_break_patterns_ordered__float64(data: Arc<Mutex<Option<Vec<f64>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>) {
    let mut length = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
    if { let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x >= __tmp_y } {
        let mut random = Arc::new(Mutex::new(Some(__slices_xorshift(Arc::new(Mutex::new(Some(({ let __v = (*length.lock().unwrap().as_ref().unwrap()); __v }) as u64)))))));
        let mut modulus = __slices_next_power_of_two(Arc::new(Mutex::new(Some({ let __arg_holder = length.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let __go_binary_0 = (*a.lock().unwrap().as_ref().unwrap());
let __go_binary_1 = (*length.lock().unwrap().as_ref().unwrap());
let __go_binary_2 = 4;
let __go_binary_3 = __go_binary_1 / __go_binary_2;
let __go_binary_4 = 2;
let __go_binary_5 = __go_binary_3 * __go_binary_4;
let __go_binary_6 = __go_binary_0 + __go_binary_5;
let __go_binary_7 = 1;
let __go_binary_8 = __go_binary_6 - __go_binary_7;
let mut idx = Arc::new(Mutex::new(Some(__go_binary_8)));
    while { let __tmp_x = { let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = ({ let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x / __tmp_y }); let __tmp_y = 2; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y }; __tmp_x <= __tmp_y } {
        let mut other = Arc::new(Mutex::new(Some((({ let __tmp_x = (*Arc::new(Mutex::new(Some((*random.lock().unwrap().as_mut().unwrap()).next() as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = modulus; let __tmp_y = 1 as u64; __tmp_x - __tmp_y }); __tmp_x & __tmp_y })) as i64)));
        if { let __tmp_x = { let __v = (*other.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        { let __rhs = (*length.lock().unwrap().as_ref().unwrap()); let mut guard = other.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*other.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*other.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = idx.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
}

pub fn __slices_break_patterns_ordered__string(data: Arc<Mutex<Option<Vec<String>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>) {
    let mut length = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
    if { let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x >= __tmp_y } {
        let mut random = Arc::new(Mutex::new(Some(__slices_xorshift(Arc::new(Mutex::new(Some(({ let __v = (*length.lock().unwrap().as_ref().unwrap()); __v }) as u64)))))));
        let mut modulus = __slices_next_power_of_two(Arc::new(Mutex::new(Some({ let __arg_holder = length.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let __go_binary_0 = (*a.lock().unwrap().as_ref().unwrap());
let __go_binary_1 = (*length.lock().unwrap().as_ref().unwrap());
let __go_binary_2 = 4;
let __go_binary_3 = __go_binary_1 / __go_binary_2;
let __go_binary_4 = 2;
let __go_binary_5 = __go_binary_3 * __go_binary_4;
let __go_binary_6 = __go_binary_0 + __go_binary_5;
let __go_binary_7 = 1;
let __go_binary_8 = __go_binary_6 - __go_binary_7;
let mut idx = Arc::new(Mutex::new(Some(__go_binary_8)));
    while { let __tmp_x = { let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = ({ let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x / __tmp_y }); let __tmp_y = 2; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y }; __tmp_x <= __tmp_y } {
        let mut other = Arc::new(Mutex::new(Some((({ let __tmp_x = (*Arc::new(Mutex::new(Some((*random.lock().unwrap().as_mut().unwrap()).next() as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = modulus; let __tmp_y = 1 as u64; __tmp_x - __tmp_y }); __tmp_x & __tmp_y })) as i64)));
        if { let __tmp_x = { let __v = (*other.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        { let __rhs = (*length.lock().unwrap().as_ref().unwrap()); let mut guard = other.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*other.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*other.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = idx.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
}

pub fn __slices_choose_pivot_ordered___celsius(data: Arc<Mutex<Option<Vec<Celsius>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>) -> (i64, Arc<Mutex<Option<__slices_sortedHint>>>) {
    let mut pivot: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut hint: Arc<Mutex<Option<__slices_sortedHint>>> = Arc::new(Mutex::new(Some(Default::default())));

    const shortestNinther: i64 = 50;
const maxSwaps: i64 = 4 * 3;

    let mut l = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
    let mut swaps: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x / __tmp_y }; let __tmp_y = 1; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x / __tmp_y }; let __tmp_y = 2; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));let mut k = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x / __tmp_y }; let __tmp_y = 3; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));
    if { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x >= __tmp_y } {
        if { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 50; __tmp_x >= __tmp_y } {
        { let new_val = __slices_median_adjacent_ordered___celsius(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *i.lock().unwrap() = Some(new_val); };
        { let new_val = __slices_median_adjacent_ordered___celsius(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *j.lock().unwrap() = Some(new_val); };
        { let new_val = __slices_median_adjacent_ordered___celsius(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *k.lock().unwrap() = Some(new_val); };
    }
        { let new_val = __slices_median_ordered___celsius(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            swaps.clone()
        ); *j.lock().unwrap() = Some(new_val); };
    }
    { let _switch_val = { let __v = (*swaps.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (0) {
            return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some(__slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_INCREASING_HINT as i64))))))));
        } else if _switch_val == (12) {
            return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some(__slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_DECREASING_HINT as i64))))))));
        } else {
            return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some(__slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_UNKNOWN_HINT as i64))))))));
        }
    }
}

pub fn __slices_choose_pivot_ordered__int(data: Arc<Mutex<Option<Vec<i64>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>) -> (i64, Arc<Mutex<Option<__slices_sortedHint>>>) {
    let mut pivot: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut hint: Arc<Mutex<Option<__slices_sortedHint>>> = Arc::new(Mutex::new(Some(Default::default())));

    const shortestNinther: i64 = 50;
const maxSwaps: i64 = 4 * 3;

    let mut l = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
    let mut swaps: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x / __tmp_y }; let __tmp_y = 1; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x / __tmp_y }; let __tmp_y = 2; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));let mut k = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x / __tmp_y }; let __tmp_y = 3; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));
    if { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x >= __tmp_y } {
        if { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 50; __tmp_x >= __tmp_y } {
        { let new_val = __slices_median_adjacent_ordered__int(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *i.lock().unwrap() = Some(new_val); };
        { let new_val = __slices_median_adjacent_ordered__int(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *j.lock().unwrap() = Some(new_val); };
        { let new_val = __slices_median_adjacent_ordered__int(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *k.lock().unwrap() = Some(new_val); };
    }
        { let new_val = __slices_median_ordered__int(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            swaps.clone()
        ); *j.lock().unwrap() = Some(new_val); };
    }
    { let _switch_val = { let __v = (*swaps.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (0) {
            return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some(__slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_INCREASING_HINT as i64))))))));
        } else if _switch_val == (12) {
            return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some(__slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_DECREASING_HINT as i64))))))));
        } else {
            return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some(__slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_UNKNOWN_HINT as i64))))))));
        }
    }
}

pub fn __slices_choose_pivot_ordered__float64(data: Arc<Mutex<Option<Vec<f64>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>) -> (i64, Arc<Mutex<Option<__slices_sortedHint>>>) {
    let mut pivot: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut hint: Arc<Mutex<Option<__slices_sortedHint>>> = Arc::new(Mutex::new(Some(Default::default())));

    const shortestNinther: i64 = 50;
const maxSwaps: i64 = 4 * 3;

    let mut l = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
    let mut swaps: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x / __tmp_y }; let __tmp_y = 1; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x / __tmp_y }; let __tmp_y = 2; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));let mut k = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x / __tmp_y }; let __tmp_y = 3; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));
    if { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x >= __tmp_y } {
        if { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 50; __tmp_x >= __tmp_y } {
        { let new_val = __slices_median_adjacent_ordered__float64(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *i.lock().unwrap() = Some(new_val); };
        { let new_val = __slices_median_adjacent_ordered__float64(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *j.lock().unwrap() = Some(new_val); };
        { let new_val = __slices_median_adjacent_ordered__float64(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *k.lock().unwrap() = Some(new_val); };
    }
        { let new_val = __slices_median_ordered__float64(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            swaps.clone()
        ); *j.lock().unwrap() = Some(new_val); };
    }
    { let _switch_val = { let __v = (*swaps.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (0) {
            return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some(__slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_INCREASING_HINT as i64))))))));
        } else if _switch_val == (12) {
            return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some(__slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_DECREASING_HINT as i64))))))));
        } else {
            return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some(__slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_UNKNOWN_HINT as i64))))))));
        }
    }
}

pub fn __slices_choose_pivot_ordered__string(data: Arc<Mutex<Option<Vec<String>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>) -> (i64, Arc<Mutex<Option<__slices_sortedHint>>>) {
    let mut pivot: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut hint: Arc<Mutex<Option<__slices_sortedHint>>> = Arc::new(Mutex::new(Some(Default::default())));

    const shortestNinther: i64 = 50;
const maxSwaps: i64 = 4 * 3;

    let mut l = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
    let mut swaps: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x / __tmp_y }; let __tmp_y = 1; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x / __tmp_y }; let __tmp_y = 2; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));let mut k = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x / __tmp_y }; let __tmp_y = 3; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));
    if { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x >= __tmp_y } {
        if { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 50; __tmp_x >= __tmp_y } {
        { let new_val = __slices_median_adjacent_ordered__string(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *i.lock().unwrap() = Some(new_val); };
        { let new_val = __slices_median_adjacent_ordered__string(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *j.lock().unwrap() = Some(new_val); };
        { let new_val = __slices_median_adjacent_ordered__string(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *k.lock().unwrap() = Some(new_val); };
    }
        { let new_val = __slices_median_ordered__string(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            swaps.clone()
        ); *j.lock().unwrap() = Some(new_val); };
    }
    { let _switch_val = { let __v = (*swaps.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (0) {
            return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some(__slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_INCREASING_HINT as i64))))))));
        } else if _switch_val == (12) {
            return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some(__slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_DECREASING_HINT as i64))))))));
        } else {
            return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some(__slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_UNKNOWN_HINT as i64))))))));
        }
    }
}

pub fn __slices_reverse_range_ordered___celsius(data: Arc<Mutex<Option<Vec<Celsius>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>) {
    let mut i = { let __owned = a.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

pub fn __slices_reverse_range_ordered__int(data: Arc<Mutex<Option<Vec<i64>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>) {
    let mut i = { let __owned = a.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

pub fn __slices_reverse_range_ordered__float64(data: Arc<Mutex<Option<Vec<f64>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>) {
    let mut i = { let __owned = a.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

pub fn __slices_reverse_range_ordered__string(data: Arc<Mutex<Option<Vec<String>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>) {
    let mut i = { let __owned = a.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

pub fn __slices_partial_insertion_sort_ordered___celsius(data: Arc<Mutex<Option<Vec<Celsius>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>) -> bool {
    const maxSteps: i64 = 5;
const shortestShifting: i64 = 50;

    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })));
    let mut j = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 5; __tmp_x < __tmp_y } {
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = !__cmp__less___celsius(
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value })))
                );
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        return true;
    }
        if { let __tmp_x = { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = 50; __tmp_x < __tmp_y } {
        return false;
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        if { let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = 2; __tmp_x >= __tmp_y } {
        let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x >= __tmp_y } {
        if !__cmp__less___celsius(
            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value })))
        ) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    }
        if { let __tmp_x = { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = 2; __tmp_x >= __tmp_y } {
        let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        if !__cmp__less___celsius(
            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value })))
        ) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    false
}

pub fn __slices_partial_insertion_sort_ordered__int(data: Arc<Mutex<Option<Vec<i64>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>) -> bool {
    const maxSteps: i64 = 5;
const shortestShifting: i64 = 50;

    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })));
    let mut j = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 5; __tmp_x < __tmp_y } {
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = !__cmp__less__int(
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value })))
                );
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        return true;
    }
        if { let __tmp_x = { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = 50; __tmp_x < __tmp_y } {
        return false;
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        if { let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = 2; __tmp_x >= __tmp_y } {
        let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x >= __tmp_y } {
        if !__cmp__less__int(
            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value })))
        ) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    }
        if { let __tmp_x = { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = 2; __tmp_x >= __tmp_y } {
        let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        if !__cmp__less__int(
            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value })))
        ) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    false
}

pub fn __slices_partial_insertion_sort_ordered__float64(data: Arc<Mutex<Option<Vec<f64>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>) -> bool {
    const maxSteps: i64 = 5;
const shortestShifting: i64 = 50;

    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })));
    let mut j = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 5; __tmp_x < __tmp_y } {
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = !__cmp__less__float64(
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value })))
                );
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        return true;
    }
        if { let __tmp_x = { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = 50; __tmp_x < __tmp_y } {
        return false;
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        if { let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = 2; __tmp_x >= __tmp_y } {
        let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x >= __tmp_y } {
        if !__cmp__less__float64(
            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value })))
        ) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    }
        if { let __tmp_x = { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = 2; __tmp_x >= __tmp_y } {
        let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        if !__cmp__less__float64(
            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value })))
        ) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    false
}

pub fn __slices_partial_insertion_sort_ordered__string(data: Arc<Mutex<Option<Vec<String>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>) -> bool {
    const maxSteps: i64 = 5;
const shortestShifting: i64 = 50;

    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })));
    let mut j = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 5; __tmp_x < __tmp_y } {
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = !__cmp__less__string(
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value })))
                );
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        return true;
    }
        if { let __tmp_x = { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = 50; __tmp_x < __tmp_y } {
        return false;
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        if { let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = 2; __tmp_x >= __tmp_y } {
        let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x >= __tmp_y } {
        if !__cmp__less__string(
            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value })))
        ) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    }
        if { let __tmp_x = { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = 2; __tmp_x >= __tmp_y } {
        let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        if !__cmp__less__string(
            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value })))
        ) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    false
}

pub fn __slices_partition_equal_ordered___celsius(data: Arc<Mutex<Option<Vec<Celsius>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, pivot: Arc<Mutex<Option<i64>>>) -> i64 {
    let mut newpivot: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));

    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ({ let __v = (*pivot.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*pivot.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    let (mut i, mut j) = ({ let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }))); __tuple_value }, { let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }))); __tuple_value });
    loop {
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = !__cmp__less___celsius(
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
                );
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = __cmp__less___celsius(
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
                );
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    return { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn __slices_partition_equal_ordered__int(data: Arc<Mutex<Option<Vec<i64>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, pivot: Arc<Mutex<Option<i64>>>) -> i64 {
    let mut newpivot: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));

    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ({ let __v = (*pivot.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*pivot.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    let (mut i, mut j) = ({ let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }))); __tuple_value }, { let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }))); __tuple_value });
    loop {
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = !__cmp__less__int(
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
                );
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = __cmp__less__int(
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
                );
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    return { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn __slices_partition_equal_ordered__float64(data: Arc<Mutex<Option<Vec<f64>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, pivot: Arc<Mutex<Option<i64>>>) -> i64 {
    let mut newpivot: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));

    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ({ let __v = (*pivot.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*pivot.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    let (mut i, mut j) = ({ let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }))); __tuple_value }, { let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }))); __tuple_value });
    loop {
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = !__cmp__less__float64(
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
                );
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = __cmp__less__float64(
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
                );
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    return { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn __slices_partition_equal_ordered__string(data: Arc<Mutex<Option<Vec<String>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, pivot: Arc<Mutex<Option<i64>>>) -> i64 {
    let mut newpivot: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));

    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ({ let __v = (*pivot.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*pivot.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    let (mut i, mut j) = ({ let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }))); __tuple_value }, { let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }))); __tuple_value });
    loop {
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = !__cmp__less__string(
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
                );
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = __cmp__less__string(
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
                );
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    return { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn __slices_partition_ordered___celsius(data: Arc<Mutex<Option<Vec<Celsius>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, pivot: Arc<Mutex<Option<i64>>>) -> (i64, bool) {
    let mut newpivot: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut alreadyPartitioned: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ({ let __v = (*pivot.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*pivot.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    let (mut i, mut j) = ({ let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }))); __tuple_value }, { let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }))); __tuple_value });
    while {
        let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
        if __go_cond_0 {
            let __go_cond_1 = __cmp__less___celsius(
                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
            );
            __go_cond_1
        } else {
            false
        }
    } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    while {
        let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
        if __go_cond_0 {
            let __go_cond_1 = !__cmp__less___celsius(
                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
            );
            __go_cond_1
        } else {
            false
        }
    } {
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, true);
    }
    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    loop {
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = __cmp__less___celsius(
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
                );
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = !__cmp__less___celsius(
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
                );
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, false);
}

pub fn __slices_partition_ordered__int(data: Arc<Mutex<Option<Vec<i64>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, pivot: Arc<Mutex<Option<i64>>>) -> (i64, bool) {
    let mut newpivot: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut alreadyPartitioned: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ({ let __v = (*pivot.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*pivot.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    let (mut i, mut j) = ({ let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }))); __tuple_value }, { let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }))); __tuple_value });
    while {
        let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
        if __go_cond_0 {
            let __go_cond_1 = __cmp__less__int(
                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
            );
            __go_cond_1
        } else {
            false
        }
    } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    while {
        let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
        if __go_cond_0 {
            let __go_cond_1 = !__cmp__less__int(
                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
            );
            __go_cond_1
        } else {
            false
        }
    } {
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, true);
    }
    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    loop {
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = __cmp__less__int(
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
                );
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = !__cmp__less__int(
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
                );
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, false);
}

pub fn __slices_partition_ordered__float64(data: Arc<Mutex<Option<Vec<f64>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, pivot: Arc<Mutex<Option<i64>>>) -> (i64, bool) {
    let mut newpivot: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut alreadyPartitioned: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ({ let __v = (*pivot.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*pivot.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    let (mut i, mut j) = ({ let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }))); __tuple_value }, { let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }))); __tuple_value });
    while {
        let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
        if __go_cond_0 {
            let __go_cond_1 = __cmp__less__float64(
                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
            );
            __go_cond_1
        } else {
            false
        }
    } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    while {
        let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
        if __go_cond_0 {
            let __go_cond_1 = !__cmp__less__float64(
                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
            );
            __go_cond_1
        } else {
            false
        }
    } {
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, true);
    }
    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    loop {
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = __cmp__less__float64(
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
                );
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = !__cmp__less__float64(
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
                );
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, false);
}

pub fn __slices_partition_ordered__string(data: Arc<Mutex<Option<Vec<String>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, pivot: Arc<Mutex<Option<i64>>>) -> (i64, bool) {
    let mut newpivot: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut alreadyPartitioned: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ({ let __v = (*pivot.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*pivot.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    let (mut i, mut j) = ({ let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }))); __tuple_value }, { let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }))); __tuple_value });
    while {
        let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
        if __go_cond_0 {
            let __go_cond_1 = __cmp__less__string(
                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
            );
            __go_cond_1
        } else {
            false
        }
    } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    while {
        let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
        if __go_cond_0 {
            let __go_cond_1 = !__cmp__less__string(
                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
            );
            __go_cond_1
        } else {
            false
        }
    } {
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, true);
    }
    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    loop {
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = __cmp__less__string(
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
                );
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = !__cmp__less__string(
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
                );
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, false);
}

pub fn __slices_rotate_cmp_func___item(data: Arc<Mutex<Option<Vec<Item>>>>, a: Arc<Mutex<Option<i64>>>, m: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Item>>>, Arc<Mutex<Option<Item>>>) -> i64 + Send + Sync>>>>) {
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
    let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        __slices_swap_range_cmp_func___item(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = m.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
        { let __rhs = (*j.lock().unwrap().as_ref().unwrap()); let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    } else {
        __slices_swap_range_cmp_func___item(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))),
            Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
        { let __rhs = (*i.lock().unwrap().as_ref().unwrap()); let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
    }
    __slices_swap_range_cmp_func___item(
        data.clone(),
        Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = m.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
    );
}

pub fn __slices_sift_down_cmp_func__ptr__task(data: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Task>>>>>>>, lo: Arc<Mutex<Option<i64>>>, hi: Arc<Mutex<Option<i64>>>, first: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Task>>>, Arc<Mutex<Option<Task>>>) -> i64 + Send + Sync>>>>) {
    let mut root = { let __owned = lo.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    loop {
        let mut child = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = 2; let __tmp_y = { let __v = (*root.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y })));
        if { let __tmp_x = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        break
    }
        if {
            let __go_cond_0 = { let __tmp_x = { let __tmp_x = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = ({
                    let __tmp_x =
                        {
                            let __f: Arc<dyn Fn(Arc<Mutex<Option<Task>>>, Arc<Mutex<Option<Task>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                            (__f)(
                                { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() },
                                { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize].clone() }
                            )
                        };
                    let __tmp_y = 0;
                    __tmp_x < __tmp_y
                });
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = child.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if !({
            let __tmp_x =
                {
                    let __f: Arc<dyn Fn(Arc<Mutex<Option<Task>>>, Arc<Mutex<Option<Task>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                    (__f)(
                        { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*root.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() },
                        { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() }
                    )
                };
            let __tmp_y = 0;
            __tmp_x < __tmp_y
        }) {
        return;
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*root.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*root.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let new_val = child.lock().unwrap().as_ref().unwrap().clone(); *root.lock().unwrap() = Some(new_val); };
    }
}

pub fn __slices_sift_down_cmp_func__int(data: Arc<Mutex<Option<Vec<i64>>>>, lo: Arc<Mutex<Option<i64>>>, hi: Arc<Mutex<Option<i64>>>, first: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync>>>>) {
    let mut root = { let __owned = lo.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    loop {
        let mut child = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = 2; let __tmp_y = { let __v = (*root.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y })));
        if { let __tmp_x = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        break
    }
        if {
            let __go_cond_0 = { let __tmp_x = { let __tmp_x = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = ({
                    let __tmp_x =
                        {
                            let __f: Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                            (__f)(
                                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() }; __arg_value }))),
                                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize].clone() }; __arg_value })))
                            )
                        };
                    let __tmp_y = 0;
                    __tmp_x < __tmp_y
                });
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = child.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if !({
            let __tmp_x =
                {
                    let __f: Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                    (__f)(
                        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*root.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() }; __arg_value }))),
                        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() }; __arg_value })))
                    )
                };
            let __tmp_y = 0;
            __tmp_x < __tmp_y
        }) {
        return;
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*root.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*root.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let new_val = child.lock().unwrap().as_ref().unwrap().clone(); *root.lock().unwrap() = Some(new_val); };
    }
}

pub fn __slices_next_power_of_two(length: Arc<Mutex<Option<i64>>>) -> u64 {
    return { let __shift_x = 1u64; let __shift_c = (math_bits::len(Arc::new(Mutex::new(Some({ let __arg_value = ({ let __v = (*length.lock().unwrap().as_ref().unwrap()); __v }) as u64; __arg_value }))))) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } };
}

pub fn __slices_median_adjacent_cmp_func__ptr__task(data: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Task>>>>>>>, a: Arc<Mutex<Option<i64>>>, swaps: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Task>>>, Arc<Mutex<Option<Task>>>) -> i64 + Send + Sync>>>>) -> i64 {
    __slices_median_cmp_func__ptr__task(
        data.clone(),
        Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }))),
        swaps.clone(),
        Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
    )
}

pub fn __slices_median_adjacent_cmp_func__int(data: Arc<Mutex<Option<Vec<i64>>>>, a: Arc<Mutex<Option<i64>>>, swaps: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync>>>>) -> i64 {
    __slices_median_cmp_func__int(
        data.clone(),
        Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }))),
        swaps.clone(),
        Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
    )
}

pub fn __slices_median_cmp_func__ptr__task(data: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Task>>>>>>>, mut a: Arc<Mutex<Option<i64>>>, mut b: Arc<Mutex<Option<i64>>>, mut c: Arc<Mutex<Option<i64>>>, swaps: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Task>>>, Arc<Mutex<Option<Task>>>) -> i64 + Send + Sync>>>>) -> i64 {
    { let (__tmp_0, __tmp_1) = __slices_order2_cmp_func__ptr__task(
        data.clone(),
        Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        swaps.clone(),
        Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
    ); *a.lock().unwrap() = Some(__tmp_0); *b.lock().unwrap() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = __slices_order2_cmp_func__ptr__task(
        data.clone(),
        Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        swaps.clone(),
        Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
    ); *b.lock().unwrap() = Some(__tmp_0); *c.lock().unwrap() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = __slices_order2_cmp_func__ptr__task(
        data.clone(),
        Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        swaps.clone(),
        Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
    ); *a.lock().unwrap() = Some(__tmp_0); *b.lock().unwrap() = Some(__tmp_1); };
    return { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn __slices_median_cmp_func__int(data: Arc<Mutex<Option<Vec<i64>>>>, mut a: Arc<Mutex<Option<i64>>>, mut b: Arc<Mutex<Option<i64>>>, mut c: Arc<Mutex<Option<i64>>>, swaps: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync>>>>) -> i64 {
    { let (__tmp_0, __tmp_1) = __slices_order2_cmp_func__int(
        data.clone(),
        Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        swaps.clone(),
        Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
    ); *a.lock().unwrap() = Some(__tmp_0); *b.lock().unwrap() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = __slices_order2_cmp_func__int(
        data.clone(),
        Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        swaps.clone(),
        Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
    ); *b.lock().unwrap() = Some(__tmp_0); *c.lock().unwrap() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = __slices_order2_cmp_func__int(
        data.clone(),
        Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        swaps.clone(),
        Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
    ); *a.lock().unwrap() = Some(__tmp_0); *b.lock().unwrap() = Some(__tmp_1); };
    return { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn __slices_sift_down_ordered___celsius(data: Arc<Mutex<Option<Vec<Celsius>>>>, lo: Arc<Mutex<Option<i64>>>, hi: Arc<Mutex<Option<i64>>>, first: Arc<Mutex<Option<i64>>>) {
    let mut root = { let __owned = lo.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    loop {
        let mut child = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = 2; let __tmp_y = { let __v = (*root.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y })));
        if { let __tmp_x = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        break
    }
        if {
            let __go_cond_0 = { let __tmp_x = { let __tmp_x = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = __cmp__less___celsius(
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() }; __arg_value }))),
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize].clone() }; __arg_value })))
                );
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = child.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if !__cmp__less___celsius(
            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*root.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() }; __arg_value }))),
            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() }; __arg_value })))
        ) {
        return;
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*root.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*root.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let new_val = child.lock().unwrap().as_ref().unwrap().clone(); *root.lock().unwrap() = Some(new_val); };
    }
}

pub fn __slices_sift_down_ordered__int(data: Arc<Mutex<Option<Vec<i64>>>>, lo: Arc<Mutex<Option<i64>>>, hi: Arc<Mutex<Option<i64>>>, first: Arc<Mutex<Option<i64>>>) {
    let mut root = { let __owned = lo.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    loop {
        let mut child = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = 2; let __tmp_y = { let __v = (*root.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y })));
        if { let __tmp_x = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        break
    }
        if {
            let __go_cond_0 = { let __tmp_x = { let __tmp_x = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = __cmp__less__int(
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() }; __arg_value }))),
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize].clone() }; __arg_value })))
                );
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = child.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if !__cmp__less__int(
            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*root.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() }; __arg_value }))),
            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() }; __arg_value })))
        ) {
        return;
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*root.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*root.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let new_val = child.lock().unwrap().as_ref().unwrap().clone(); *root.lock().unwrap() = Some(new_val); };
    }
}

pub fn __slices_sift_down_ordered__float64(data: Arc<Mutex<Option<Vec<f64>>>>, lo: Arc<Mutex<Option<i64>>>, hi: Arc<Mutex<Option<i64>>>, first: Arc<Mutex<Option<i64>>>) {
    let mut root = { let __owned = lo.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    loop {
        let mut child = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = 2; let __tmp_y = { let __v = (*root.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y })));
        if { let __tmp_x = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        break
    }
        if {
            let __go_cond_0 = { let __tmp_x = { let __tmp_x = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = __cmp__less__float64(
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() }; __arg_value }))),
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize].clone() }; __arg_value })))
                );
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = child.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if !__cmp__less__float64(
            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*root.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() }; __arg_value }))),
            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() }; __arg_value })))
        ) {
        return;
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*root.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*root.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let new_val = child.lock().unwrap().as_ref().unwrap().clone(); *root.lock().unwrap() = Some(new_val); };
    }
}

pub fn __slices_sift_down_ordered__string(data: Arc<Mutex<Option<Vec<String>>>>, lo: Arc<Mutex<Option<i64>>>, hi: Arc<Mutex<Option<i64>>>, first: Arc<Mutex<Option<i64>>>) {
    let mut root = { let __owned = lo.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    loop {
        let mut child = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = 2; let __tmp_y = { let __v = (*root.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y })));
        if { let __tmp_x = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        break
    }
        if {
            let __go_cond_0 = { let __tmp_x = { let __tmp_x = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = __cmp__less__string(
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() }; __arg_value }))),
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize].clone() }; __arg_value })))
                );
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = child.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if !__cmp__less__string(
            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*root.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() }; __arg_value }))),
            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() }; __arg_value })))
        ) {
        return;
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*root.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*root.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let new_val = child.lock().unwrap().as_ref().unwrap().clone(); *root.lock().unwrap() = Some(new_val); };
    }
}

pub fn __slices_median_adjacent_ordered___celsius(data: Arc<Mutex<Option<Vec<Celsius>>>>, a: Arc<Mutex<Option<i64>>>, swaps: Arc<Mutex<Option<i64>>>) -> i64 {
    __slices_median_ordered___celsius(
        data.clone(),
        Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }))),
        swaps.clone()
    )
}

pub fn __slices_median_adjacent_ordered__int(data: Arc<Mutex<Option<Vec<i64>>>>, a: Arc<Mutex<Option<i64>>>, swaps: Arc<Mutex<Option<i64>>>) -> i64 {
    __slices_median_ordered__int(
        data.clone(),
        Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }))),
        swaps.clone()
    )
}

pub fn __slices_median_adjacent_ordered__float64(data: Arc<Mutex<Option<Vec<f64>>>>, a: Arc<Mutex<Option<i64>>>, swaps: Arc<Mutex<Option<i64>>>) -> i64 {
    __slices_median_ordered__float64(
        data.clone(),
        Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }))),
        swaps.clone()
    )
}

pub fn __slices_median_adjacent_ordered__string(data: Arc<Mutex<Option<Vec<String>>>>, a: Arc<Mutex<Option<i64>>>, swaps: Arc<Mutex<Option<i64>>>) -> i64 {
    __slices_median_ordered__string(
        data.clone(),
        Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }))),
        swaps.clone()
    )
}

pub fn __slices_median_ordered___celsius(data: Arc<Mutex<Option<Vec<Celsius>>>>, mut a: Arc<Mutex<Option<i64>>>, mut b: Arc<Mutex<Option<i64>>>, mut c: Arc<Mutex<Option<i64>>>, swaps: Arc<Mutex<Option<i64>>>) -> i64 {
    { let (__tmp_0, __tmp_1) = __slices_order2_ordered___celsius(
        data.clone(),
        Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        swaps.clone()
    ); *a.lock().unwrap() = Some(__tmp_0); *b.lock().unwrap() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = __slices_order2_ordered___celsius(
        data.clone(),
        Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        swaps.clone()
    ); *b.lock().unwrap() = Some(__tmp_0); *c.lock().unwrap() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = __slices_order2_ordered___celsius(
        data.clone(),
        Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        swaps.clone()
    ); *a.lock().unwrap() = Some(__tmp_0); *b.lock().unwrap() = Some(__tmp_1); };
    return { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn __slices_median_ordered__int(data: Arc<Mutex<Option<Vec<i64>>>>, mut a: Arc<Mutex<Option<i64>>>, mut b: Arc<Mutex<Option<i64>>>, mut c: Arc<Mutex<Option<i64>>>, swaps: Arc<Mutex<Option<i64>>>) -> i64 {
    { let (__tmp_0, __tmp_1) = __slices_order2_ordered__int(
        data.clone(),
        Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        swaps.clone()
    ); *a.lock().unwrap() = Some(__tmp_0); *b.lock().unwrap() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = __slices_order2_ordered__int(
        data.clone(),
        Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        swaps.clone()
    ); *b.lock().unwrap() = Some(__tmp_0); *c.lock().unwrap() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = __slices_order2_ordered__int(
        data.clone(),
        Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        swaps.clone()
    ); *a.lock().unwrap() = Some(__tmp_0); *b.lock().unwrap() = Some(__tmp_1); };
    return { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn __slices_median_ordered__float64(data: Arc<Mutex<Option<Vec<f64>>>>, mut a: Arc<Mutex<Option<i64>>>, mut b: Arc<Mutex<Option<i64>>>, mut c: Arc<Mutex<Option<i64>>>, swaps: Arc<Mutex<Option<i64>>>) -> i64 {
    { let (__tmp_0, __tmp_1) = __slices_order2_ordered__float64(
        data.clone(),
        Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        swaps.clone()
    ); *a.lock().unwrap() = Some(__tmp_0); *b.lock().unwrap() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = __slices_order2_ordered__float64(
        data.clone(),
        Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        swaps.clone()
    ); *b.lock().unwrap() = Some(__tmp_0); *c.lock().unwrap() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = __slices_order2_ordered__float64(
        data.clone(),
        Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        swaps.clone()
    ); *a.lock().unwrap() = Some(__tmp_0); *b.lock().unwrap() = Some(__tmp_1); };
    return { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn __slices_median_ordered__string(data: Arc<Mutex<Option<Vec<String>>>>, mut a: Arc<Mutex<Option<i64>>>, mut b: Arc<Mutex<Option<i64>>>, mut c: Arc<Mutex<Option<i64>>>, swaps: Arc<Mutex<Option<i64>>>) -> i64 {
    { let (__tmp_0, __tmp_1) = __slices_order2_ordered__string(
        data.clone(),
        Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        swaps.clone()
    ); *a.lock().unwrap() = Some(__tmp_0); *b.lock().unwrap() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = __slices_order2_ordered__string(
        data.clone(),
        Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        swaps.clone()
    ); *b.lock().unwrap() = Some(__tmp_0); *c.lock().unwrap() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = __slices_order2_ordered__string(
        data.clone(),
        Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        swaps.clone()
    ); *a.lock().unwrap() = Some(__tmp_0); *b.lock().unwrap() = Some(__tmp_1); };
    return { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn __slices_swap_range_cmp_func___item(data: Arc<Mutex<Option<Vec<Item>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, n: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Item>>>, Arc<Mutex<Option<Item>>>) -> i64 + Send + Sync>>>>) {
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

pub fn __slices_order2_cmp_func__ptr__task(data: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Task>>>>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, swaps: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Task>>>, Arc<Mutex<Option<Task>>>) -> i64 + Send + Sync>>>>) -> (i64, i64) {
    if {
        let __tmp_x =
            {
                let __f: Arc<dyn Fn(Arc<Mutex<Option<Task>>>, Arc<Mutex<Option<Task>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                (__f)(
                    { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() },
                    { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }
                )
            };
        let __tmp_y = 0;
        __tmp_x < __tmp_y
    } {
        { let mut guard = swaps.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return ({ let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }
    return ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v });
}

pub fn __slices_order2_cmp_func__int(data: Arc<Mutex<Option<Vec<i64>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, swaps: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync>>>>) -> (i64, i64) {
    if {
        let __tmp_x =
            {
                let __f: Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                (__f)(
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
                )
            };
        let __tmp_y = 0;
        __tmp_x < __tmp_y
    } {
        { let mut guard = swaps.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return ({ let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }
    return ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v });
}

pub fn __slices_order2_ordered___celsius(data: Arc<Mutex<Option<Vec<Celsius>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, swaps: Arc<Mutex<Option<i64>>>) -> (i64, i64) {
    if __cmp__less___celsius(
        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
    ) {
        { let mut guard = swaps.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return ({ let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }
    return ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v });
}

pub fn __slices_order2_ordered__int(data: Arc<Mutex<Option<Vec<i64>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, swaps: Arc<Mutex<Option<i64>>>) -> (i64, i64) {
    if __cmp__less__int(
        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
    ) {
        { let mut guard = swaps.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return ({ let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }
    return ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v });
}

pub fn __slices_order2_ordered__float64(data: Arc<Mutex<Option<Vec<f64>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, swaps: Arc<Mutex<Option<i64>>>) -> (i64, i64) {
    if __cmp__less__float64(
        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
    ) {
        { let mut guard = swaps.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return ({ let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }
    return ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v });
}

pub fn __slices_order2_ordered__string(data: Arc<Mutex<Option<Vec<String>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, swaps: Arc<Mutex<Option<i64>>>) -> (i64, i64) {
    if __cmp__less__string(
        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
    ) {
        { let mut guard = swaps.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return ({ let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }
    return ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v });
}

impl GoValueClone for Item {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Task {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
