#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::cell::{RefCell};
use std::cmp::Ord;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! print {
    ($($__arg:tt)*) => {{
        std::print!($($__arg)*);
        std::io::Write::flush(&mut std::io::stdout()).ok();
    }};
}
fn format_map<K: Display + Ord + Clone, V>(map: &Rc<RefCell<Option<BTreeMap<K, Rc<RefCell<Option<V>>>>>>>) -> String
where
    V: Display + Clone,
{
    format_map_nil(map, "<nil>")
}

// format_map_nil prints a value the map holds no value for as __nil_text:
// [] for a nil slice, as Go prints one.
fn format_map_nil<K: Display + Ord + Clone, V>(map: &Rc<RefCell<Option<BTreeMap<K, Rc<RefCell<Option<V>>>>>>>, __nil_text: &str) -> String
where
    V: Display + Clone,
{
    // The entries are copied out first, in key order, so a value's String
    // method that reads this map does not wait on the lock printing holds.
    let items: Option<Vec<(K, Rc<RefCell<Option<V>>>)>> = { let guard = map.borrow(); guard.as_ref().map(|m| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect()) };
    if let Some(items) = items {
        let formatted: Vec<String> = items
            .iter()
            .map(|(k, v)| {
                // The value is copied out too, so its String method may
                // read its own entry.
                let value = { let v_guard = v.borrow(); v_guard.clone() };
                if let Some(ref val) = value {
                    format!("{}:{}", go_display_value(k), go_display_value(val))
                } else {
                    format!("{}:{}", go_display_value(k), __nil_text)
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

/// The Go func value a func-typed expression holds, or None for a nil func.
/// A copy of a func value is a clone of the same closure.
trait GoFuncValue {
    type Func: ?Sized;
    fn go_func_value(&self) -> Option<std::rc::Rc<Self::Func>>;
}

macro_rules! impl_go_func_value {
    ($(($($arg:ident),*)),* $(,)?) => { $(
        impl<R: 'static, $($arg: 'static),*> GoFuncValue for std::rc::Rc<dyn Fn($($arg),*) -> R> {
            type Func = dyn Fn($($arg),*) -> R;
            fn go_func_value(&self) -> Option<std::rc::Rc<Self::Func>> { Some(self.clone()) }
        }
        impl<R: 'static, $($arg: 'static),*> GoFuncValue for Option<std::rc::Rc<dyn Fn($($arg),*) -> R>> {
            type Func = dyn Fn($($arg),*) -> R;
            fn go_func_value(&self) -> Option<std::rc::Rc<Self::Func>> { self.clone() }
        }
        impl<R: 'static, $($arg: 'static),*> GoFuncValue for std::rc::Rc<std::cell::RefCell<Option<std::rc::Rc<dyn Fn($($arg),*) -> R>>>> {
            type Func = dyn Fn($($arg),*) -> R;
            fn go_func_value(&self) -> Option<std::rc::Rc<Self::Func>> { self.borrow().clone() }
        }
    )* };
}
impl_go_func_value!((), (A0), (A0, A1), (A0, A1, A2), (A0, A1, A2, A3), (A0, A1, A2, A3, A4), (A0, A1, A2, A3, A4, A5), (A0, A1, A2, A3, A4, A5, A6), (A0, A1, A2, A3, A4, A5, A6, A7), (A0, A1, A2, A3, A4, A5, A6, A7, A8), (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9), (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10), (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11));

fn go_install_panic_exit() {
    // Go lets a program call main again; the hook goes in once, so a
    // later call keeps the hook that go_silence_panic_hook wrapped around it.
    static __GO_PANIC_EXIT: std::sync::Once = std::sync::Once::new();
    __GO_PANIC_EXIT.call_once(|| {
        std::panic::set_hook(Box::new(|__info| {
            let __text = go_panic_payload_text(__info.payload());
            {
                use std::io::Write;
                std::io::stdout().flush().ok();
            }
            eprintln!("panic: {}", __text);
            std::process::exit(2);
        }));
    });
}

fn go_panic_payload_text(__payload: &dyn Any) -> String {
    if let Some(__text) = __payload.downcast_ref::<&'static str>() {
        return go_runtime_error_text(__text.to_string());
    }
    if let Some(__text) = __payload.downcast_ref::<String>() {
        return go_runtime_error_text_of_string(__text.clone());
    }
    String::from(GO_UNPRINTABLE_PANIC_VALUE)
}

const GO_UNPRINTABLE_PANIC_VALUE: &str = "(unprintable panic value)";

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

fn go_runtime_error_text_of_string(__text: String) -> String {
    // panic(s) in Go raises a String, and so does a runtime error whose text
    // Rust formats: an index out of range, or a nil pointer read through
    // expect(). The constant runtime errors arrive as a &'static str, so a
    // Go string that reads like one keeps its text.
    if __text.starts_with("index out of bounds: ") || __text == "nil pointer dereference" || __text == "invalid memory address or nil pointer dereference" {
        return go_runtime_error_text(__text);
    }
    __text
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

#[derive(Clone)]
pub struct GoSliceElemPtr<T> {
    slice: Rc<RefCell<Option<Vec<T>>>>,
    index: usize,
    /// The element's identity when the pointer was made with the slice
    /// free to borrow; otherwise identity reads it from the slice.
    id: Option<(usize, usize)>,
}

pub struct GoSliceElemRef<T: Clone> {
    value: Option<T>,
}

pub struct GoSliceElemMutRef<T: Clone> {
    slice: Rc<RefCell<Option<Vec<T>>>>,
    index: usize,
    value: Option<T>,
}

pub trait GoArrayElemBacking<T: Clone + 'static, const N: usize> {
    fn borrow_at(&self, index: usize) -> Option<T>;
    fn assign_at(&self, index: usize, value: Option<T>);
    fn identity_at(&self, index: usize) -> (*const (), usize);
}

#[derive(Clone)]
pub struct GoDirectArrayElemBacking<T: Clone + 'static, const N: usize> {
    array: Rc<RefCell<Option<[T; N]>>>,
}

impl<T: Clone + 'static, const N: usize> GoArrayElemBacking<T, N> for GoDirectArrayElemBacking<T, N> {
    fn borrow_at(&self, index: usize) -> Option<T> {
        let guard = self.array.borrow();
        guard.as_ref().and_then(|values| values.get(index).cloned())
    }

    fn assign_at(&self, index: usize, value: Option<T>) {
        if let Some(value) = value {
            if let Some(values) = self.array.borrow_mut().as_mut() {
                values[index] = value;
            }
        }
    }

    fn identity_at(&self, index: usize) -> (*const (), usize) {
        (Rc::as_ptr(&self.array) as *const (), index)
    }
}

#[derive(Clone)]
pub struct GoNestedArrayElemBacking<T: Clone + 'static, const N: usize, const OUT: usize> {
    outer: Rc<RefCell<Option<[[T; N]; OUT]>>>,
    outer_index: usize,
}

impl<T: Clone + 'static, const N: usize, const OUT: usize> GoArrayElemBacking<T, N> for GoNestedArrayElemBacking<T, N, OUT> {
    fn borrow_at(&self, index: usize) -> Option<T> {
        let guard = self.outer.borrow();
        guard.as_ref().and_then(|values| values.get(self.outer_index)).and_then(|inner| inner.get(index)).cloned()
    }

    fn assign_at(&self, index: usize, value: Option<T>) {
        if let Some(value) = value {
            if let Some(values) = self.outer.borrow_mut().as_mut() {
                values[self.outer_index][index] = value;
            }
        }
    }

    fn identity_at(&self, index: usize) -> (*const (), usize) {
        (Rc::as_ptr(&self.outer) as *const (), self.outer_index.wrapping_mul(N).wrapping_add(index))
    }
}

#[derive(Clone)]
pub struct GoArrayElemFromElemBacking<T: Clone + 'static, const N: usize, const OUT: usize> {
    parent: GoArrayElemPtr<[T; N], OUT>,
}

impl<T: Clone + 'static, const N: usize, const OUT: usize> GoArrayElemBacking<T, N> for GoArrayElemFromElemBacking<T, N, OUT> {
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
pub struct GoArrayElemFromGoPtrBacking<T: Clone + 'static, const N: usize> {
    parent: GoPtr<[T; N]>,
}

impl<T: Clone + 'static, const N: usize> GoArrayElemBacking<T, N> for GoArrayElemFromGoPtrBacking<T, N> {
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
pub struct GoArrayElemPtr<T: Clone + 'static, const N: usize> {
    backing: Rc<dyn GoArrayElemBacking<T, N>>,
    index: usize,
}

pub struct GoArrayElemRef<T: Clone> {
    value: Option<T>,
}

pub struct GoArrayElemMutRef<T: Clone + 'static, const N: usize> {
    backing: Rc<dyn GoArrayElemBacking<T, N>>,
    index: usize,
    value: Option<T>,
}

impl<T> GoSliceElemPtr<T> {
    pub fn new(slice: Rc<RefCell<Option<Vec<T>>>>, index: usize) -> Self {
        // &s[i] panics, as indexing does, when i is not below len(s). A
        // slice already borrowed for the expression is not checked.
        if let Ok(__guard) = slice.try_borrow() {
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
            return GoSliceElemPtr { slice: std::rc::Rc::new(std::cell::RefCell::new(__header)), index, id: __id };
        }
        GoSliceElemPtr { slice, index, id: None }
    }

    /// The array and element the pointer names, for == between pointers.
    pub fn identity(&self) -> (usize, usize) {
        if let Some(__id) = self.id {
            return __id;
        }
        if let Ok(__guard) = self.slice.try_borrow() {
            if let Some(__slice) = __guard.as_ref() {
                let __base = __slice.as_slice().as_ptr() as usize;
                return (__base.wrapping_add(self.index.wrapping_mul(std::mem::size_of::<T>())), 0);
            }
        }
        (Rc::as_ptr(&self.slice) as *const () as usize, self.index)
    }

    pub fn slice_handle(&self) -> Rc<RefCell<Option<Vec<T>>>> {
        self.slice.clone()
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

impl<T: Clone> GoSliceElemPtr<T> {
    pub fn borrow(&self) -> GoSliceElemRef<T> {
        let guard = self.slice.borrow();
        GoSliceElemRef {
            value: guard.as_ref().and_then(|values| values.get(self.index).cloned()),
        }
    }

    pub fn borrow_mut(&self) -> GoSliceElemMutRef<T> {
        let guard = self.slice.borrow();
        GoSliceElemMutRef {
            slice: self.slice.clone(),
            index: self.index,
            value: guard.as_ref().and_then(|values| values.get(self.index).cloned()),
        }
    }
}

impl<T: Clone + 'static, const N: usize> GoArrayElemPtr<T, N> {
    pub fn new(array: Rc<RefCell<Option<[T; N]>>>, index: usize) -> Self {
        GoArrayElemPtr {
            backing: Rc::new(GoDirectArrayElemBacking { array }),
            index,
        }
    }

    pub fn nested<const OUT: usize>(outer: Rc<RefCell<Option<[[T; N]; OUT]>>>, outer_index: usize, index: usize) -> Self {
        GoArrayElemPtr {
            backing: Rc::new(GoNestedArrayElemBacking { outer, outer_index }),
            index,
        }
    }

    pub fn from_array_elem<const OUT: usize>(parent: GoArrayElemPtr<[T; N], OUT>, index: usize) -> Self {
        GoArrayElemPtr {
            backing: Rc::new(GoArrayElemFromElemBacking { parent }),
            index,
        }
    }

    pub fn from_go_ptr(parent: GoPtr<[T; N]>, index: usize) -> Self {
        GoArrayElemPtr {
            backing: Rc::new(GoArrayElemFromGoPtrBacking { parent }),
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
            if let Some(values) = self.slice.borrow_mut().as_mut() {
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

impl<T: Clone + 'static, const N: usize> std::ops::Deref for GoArrayElemMutRef<T, N> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Clone + 'static, const N: usize> std::ops::DerefMut for GoArrayElemMutRef<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T: Clone + 'static, const N: usize> Drop for GoArrayElemMutRef<T, N> {
    fn drop(&mut self) {
        self.backing.assign_at(self.index, self.value.clone());
    }
}

pub trait GoArrayElemPtrDyn<T: 'static> {
    fn borrow_dyn(&self) -> Option<T>;
    fn assign_dyn(&self, value: Option<T>);
    fn with_mut_dyn(&self, f: &mut dyn FnMut(&mut T));
    fn identity_dyn(&self) -> (*const (), usize);
}

pub struct GoForeignArrayElemPtrDyn<T: 'static> {
    borrow: Rc<dyn Fn() -> Option<T>>,
    assign: Rc<dyn Fn(Option<T>)>,
    with_mut: Rc<dyn Fn(&mut dyn FnMut(&mut T))>,
    identity: Rc<dyn Fn() -> (*const (), usize)>,
}

impl<T: 'static> GoArrayElemPtrDyn<T> for GoForeignArrayElemPtrDyn<T> {
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

impl<T: Clone + 'static, const N: usize> GoArrayElemPtrDyn<T> for GoArrayElemPtr<T, N> {
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

pub enum GoPtr<T: 'static> {
    Nil,
    Raw(usize),
    Local(Rc<RefCell<Option<T>>>),
    SliceElem(GoSliceElemPtr<T>),
    ArrayElem(Rc<dyn GoArrayElemPtrDyn<T>>),
}

impl<T: 'static> Clone for GoPtr<T> {
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

impl<T: 'static> GoPtr<T> {
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

    pub fn local(value: Rc<RefCell<Option<T>>>) -> Self {
        if value.borrow().is_none() {
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
        GoPtr::ArrayElem(Rc::new(value))
    }

    pub fn array_elem_opt<const N: usize>(value: Option<GoArrayElemPtr<T, N>>) -> Self
    where
        T: Clone,
    {
        match value {
            Some(value) => GoPtr::ArrayElem(Rc::new(value)),
            None => GoPtr::Nil,
        }
    }

    pub fn array_elem_foreign(
        borrow: Rc<dyn Fn() -> Option<T>>,
        assign: Rc<dyn Fn(Option<T>)>,
        with_mut: Rc<dyn Fn(&mut dyn FnMut(&mut T))>,
        identity: Rc<dyn Fn() -> (*const (), usize)>,
    ) -> Self {
        GoPtr::ArrayElem(Rc::new(GoForeignArrayElemPtrDyn {
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
            GoPtr::Local(value) => value.borrow().is_none(),
            GoPtr::SliceElem(value) => {
                let guard = value.slice.borrow();
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
                let mut guard = slot.borrow_mut();
                f(guard.as_mut().unwrap())
            }
            GoPtr::SliceElem(slot) => {
                let mut guard = slot.slice.borrow_mut();
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
            (GoPtr::Local(left), GoPtr::Local(right)) => Rc::ptr_eq(left, right),
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
            GoPtr::Local(value) => Rc::as_ptr(value) as usize,
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

impl<T: Clone + 'static> GoPtr<T> {
    pub fn borrow(&self) -> Option<T> {
        match self {
            GoPtr::Nil => None,
            GoPtr::Raw(_) => panic!("raw unsafe pointer dereference requires unsafe pointee support"),
            GoPtr::Local(value) => (*value.borrow()).clone(),
            GoPtr::SliceElem(value) => (*value.borrow()).clone(),
            GoPtr::ArrayElem(value) => value.borrow_dyn(),
        }
    }

    pub fn assign(&self, value: Option<T>) {
        match self {
            GoPtr::Nil => panic!("nil pointer dereference"),
            GoPtr::Raw(_) => panic!("raw unsafe pointer assignment requires unsafe pointee support"),
            GoPtr::Local(slot) => *slot.borrow_mut() = value,
            GoPtr::SliceElem(slot) => *slot.borrow_mut() = value,
            GoPtr::ArrayElem(slot) => slot.assign_dyn(value),
        }
    }
}

impl<T: 'static> Default for GoPtr<T> {
    fn default() -> Self {
        GoPtr::Nil
    }
}

impl<T: 'static> std::fmt::Debug for GoPtr<T> {
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
pub struct Celsius(pub Rc<RefCell<Option<f64>>>);

impl Clone for Celsius {
    fn clone(&self) -> Self {
        let __value = (*self.0.borrow()).clone();
        Celsius(Rc::new(RefCell::new(__value)))
    }
}

impl Default for Celsius {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(Some(Default::default()))))
    }
}

impl Display for Celsius {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", go_float_string(*self.0.borrow().as_ref().unwrap() as f64, false))
    }
}

impl PartialEq for Celsius {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<f64> for Celsius {
    fn eq(&self, other: &f64) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Celsius {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<f64> for Celsius {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Celsius> for f64 {
    fn eq(&self, other: &Celsius) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<Celsius> for f64 {
    fn partial_cmp(&self, other: &Celsius) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for Celsius {
    type Output = Celsius;
    fn add(self, other: Self) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<f64> for Celsius {
    type Output = Celsius;
    fn add(self, other: f64) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Celsius> for f64 {
    type Output = Celsius;
    fn add(self, other: Celsius) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Celsius {
    type Output = Celsius;
    fn sub(self, other: Self) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<f64> for Celsius {
    type Output = Celsius;
    fn sub(self, other: f64) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Celsius> for f64 {
    type Output = Celsius;
    fn sub(self, other: Celsius) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Celsius {
    type Output = Celsius;
    fn mul(self, other: Self) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<f64> for Celsius {
    type Output = Celsius;
    fn mul(self, other: f64) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Celsius> for f64 {
    type Output = Celsius;
    fn mul(self, other: Celsius) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(self * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Celsius {
    type Output = Celsius;
    fn div(self, other: Self) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div<f64> for Celsius {
    type Output = Celsius;
    fn div(self, other: f64) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Celsius> for f64 {
    type Output = Celsius;
    fn div(self, other: Celsius) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(self / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for Celsius {
    type Output = Celsius;
    fn neg(self) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(-*self.0.borrow().as_ref().unwrap()))))
    }
}


#[derive(Debug, Clone, Default)]
pub struct IDs(pub Rc<RefCell<Option<Vec<i64>>>>);

impl Display for IDs {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice(&self.0))
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub name: Rc<RefCell<Option<String>>>,
    pub price: Rc<RefCell<Option<i64>>>,
}

impl Item {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.price.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
            price: __go_clone_1_0,
        }
    }
}


impl Default for Item {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            name: __go_default_0_0,
            price: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.price.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct Task {
    pub name: Rc<RefCell<Option<String>>>,
    pub pri: Rc<RefCell<Option<i64>>>,
}

impl Task {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.pri.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
            pri: __go_clone_1_0,
        }
    }
}


impl Default for Task {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            name: __go_default_0_0,
            pri: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.pri.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug)]
pub struct __slices_xorshift(pub Rc<RefCell<Option<u64>>>);

impl Clone for __slices_xorshift {
    fn clone(&self) -> Self {
        let __value = (*self.0.borrow()).clone();
        __slices_xorshift(Rc::new(RefCell::new(__value)))
    }
}

impl Default for __slices_xorshift {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(Some(Default::default()))))
    }
}

impl Display for __slices_xorshift {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for __slices_xorshift {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u64> for __slices_xorshift {
    fn eq(&self, other: &u64) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for __slices_xorshift {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u64> for __slices_xorshift {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<__slices_xorshift> for u64 {
    fn eq(&self, other: &__slices_xorshift) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<__slices_xorshift> for u64 {
    fn partial_cmp(&self, other: &__slices_xorshift) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for __slices_xorshift {
    type Output = __slices_xorshift;
    fn add(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn add(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn add(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for __slices_xorshift {
    type Output = __slices_xorshift;
    fn sub(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn sub(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn sub(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for __slices_xorshift {
    type Output = __slices_xorshift;
    fn mul(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn mul(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn mul(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(self * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div for __slices_xorshift {
    type Output = __slices_xorshift;
    fn div(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn div(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn div(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(self / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for __slices_xorshift {
    type Output = __slices_xorshift;
    fn rem(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn rem(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn rem(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(self % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for __slices_xorshift {
    type Output = __slices_xorshift;
    fn bitand(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn bitand(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn bitand(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(self & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for __slices_xorshift {
    type Output = __slices_xorshift;
    fn bitor(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn bitor(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn bitor(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(self | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for __slices_xorshift {
    type Output = __slices_xorshift;
    fn bitxor(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn bitxor(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn bitxor(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(self ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Not for __slices_xorshift {
    type Output = __slices_xorshift;
    fn not(self) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(!*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (*other.0.borrow().as_ref().unwrap()) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i32> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: i32) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i8> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: i8) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i16> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: i16) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: i64) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u32> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: u32) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u8> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: u8) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u16> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: u16) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<usize> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: usize) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (*other.0.borrow().as_ref().unwrap()) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i32> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: i32) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i8> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: i8) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i16> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: i16) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: i64) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u32> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: u32) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u8> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: u8) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u16> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: u16) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<usize> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: usize) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl Eq for __slices_xorshift {}

impl Ord for __slices_xorshift {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug)]
pub struct __slices_sortedHint(pub Rc<RefCell<Option<i64>>>);

impl Clone for __slices_sortedHint {
    fn clone(&self) -> Self {
        let __value = (*self.0.borrow()).clone();
        __slices_sortedHint(Rc::new(RefCell::new(__value)))
    }
}

impl Default for __slices_sortedHint {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(Some(Default::default()))))
    }
}

impl Display for __slices_sortedHint {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for __slices_sortedHint {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i64> for __slices_sortedHint {
    fn eq(&self, other: &i64) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for __slices_sortedHint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i64> for __slices_sortedHint {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<__slices_sortedHint> for i64 {
    fn eq(&self, other: &__slices_sortedHint) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<__slices_sortedHint> for i64 {
    fn partial_cmp(&self, other: &__slices_sortedHint) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn add(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn add(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn add(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn sub(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn sub(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn sub(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn mul(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn mul(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn mul(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(self * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn div(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn div(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn div(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(self / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn neg(self) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(-*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn rem(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn rem(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn rem(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(self % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn bitand(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn bitand(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn bitand(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(self & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn bitor(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn bitor(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn bitor(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(self | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn bitxor(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn bitxor(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn bitxor(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(self ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Not for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn not(self) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(!*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (*other.0.borrow().as_ref().unwrap()) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i32> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: i32) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i8> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: i8) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i16> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: i16) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u32> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: u32) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u8> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: u8) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u16> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: u16) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: u64) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<usize> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: usize) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (*other.0.borrow().as_ref().unwrap()) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i32> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: i32) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i8> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: i8) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i16> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: i16) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u32> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: u32) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u8> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: u8) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u16> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: u16) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: u64) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<usize> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: usize) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl Eq for __slices_sortedHint {}

impl Ord for __slices_sortedHint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


impl __slices_xorshift {
    pub fn next(&mut self) -> u64 {
        { let __rhs = (*((*self).clone() << 13i32).0.borrow().as_ref().unwrap()).clone(); let mut guard = self.0.borrow_mut(); *guard = Some(guard.as_ref().unwrap().clone() ^ __rhs); };
        { let __rhs = (*((*self).clone() >> 7i32).0.borrow().as_ref().unwrap()).clone(); let mut guard = self.0.borrow_mut(); *guard = Some(guard.as_ref().unwrap().clone() ^ __rhs); };
        { let __rhs = (*((*self).clone() << 17i32).0.borrow().as_ref().unwrap()).clone(); let mut guard = self.0.borrow_mut(); *guard = Some(guard.as_ref().unwrap().clone() ^ __rhs); };
        (*Rc::new(RefCell::new(Some((*self.0.borrow().as_ref().unwrap()) as u64))).borrow().as_ref().unwrap())
    }
}

pub fn lower(s: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {
    let mut b = Rc::new(RefCell::new(Some(((*s.borrow().as_ref().unwrap())).as_bytes().to_go_slice())));
    { let __range_holder = b.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for (i, c) in __range_values.iter().copied().enumerate() {
        if ('A' as u8) <= c && c <= ('Z' as u8) {
        { let __idx = (i) as usize; let __rhs = c + ('a' as u8) - ('A' as u8); (*b.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    }
    } }
    return Rc::new(RefCell::new(Some(String::from_utf8(std::vec::Vec::from((*b.borrow().as_ref().unwrap()).clone())).unwrap())));
}

fn main() {
    go_install_panic_exit();
    ::cmp::__go_init_all();
    ::math_bits::__go_init_all();

    let mut s = Rc::new(RefCell::new(Some({ let mut v: Vec<i64> = (0..(10) as usize).map(|_| 0i64).collect(); v.truncate((5) as usize); v })));
    for i in 0..(({ let __range_holder = s.clone(); let __range_guard = __range_holder.borrow(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        { let __idx = (i) as usize; let __rhs = i as i64 + 1; (*s.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    }
    { let __slice_header = GoSliceHeader::go_slice_header(&(__slices__insert__slice_int__int(s.clone(), Rc::new(RefCell::new(Some(2))), Rc::new(RefCell::new(Some(vec![10, 11])))))); *s.borrow_mut() = __slice_header; };
    println!("{} {} {}", format!("{}", format_slice(&s)), format!("{}", ({ let __len = (*s.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", (*s.borrow()).as_ref().map(|__v| __v.capacity()).unwrap_or(0)));
    { let __slice_header = GoSliceHeader::go_slice_header(&(__slices__insert__slice_int__int(s.clone(), Rc::new(RefCell::new(Some(1))), Rc::new(RefCell::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = (4) as usize; let __high = (6) as usize; let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) })))))); *s.borrow_mut() = __slice_header; };
    println!("{}", format!("{}", format_slice(&s)));
    { let __slice_header = GoSliceHeader::go_slice_header(&(__slices__replace__slice_int__int(s.clone(), Rc::new(RefCell::new(Some(1))), Rc::new(RefCell::new(Some(3))), Rc::new(RefCell::new(Some(vec![7, 8, 9])))))); *s.borrow_mut() = __slice_header; };
    println!("{}", format!("{}", format_slice(&s)));
    { let __slice_header = GoSliceHeader::go_slice_header(&(__slices__replace__slice_int__int(s.clone(), Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(2))), Rc::new(RefCell::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = (3) as usize; let __high = (7) as usize; let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) })))))); *s.borrow_mut() = __slice_header; };
    println!("{}", format!("{}", format_slice(&s)));
    let mut ins = __slices__insert__slice_int__int(Rc::new(RefCell::new(Some(vec![1i64, 4i64]))), Rc::new(RefCell::new(Some(1))), Rc::new(RefCell::new(Some(vec![2, 3]))));
    let mut ins = { let __slice_header = GoSliceHeader::go_slice_header(&ins); Rc::new(RefCell::new(__slice_header)) };
    let mut del = __slices__delete__slice_int__int(ins.clone(), Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(2))));
    let mut del = { let __slice_header = GoSliceHeader::go_slice_header(&del); Rc::new(RefCell::new(__slice_header)) };
    println!("{} {}", format!("{}", format_slice(&ins)), format!("{}", format_slice(&del)));
    println!("{} {}", format!("{}", format_slice(&__slices__compact__slice_int__int(Rc::new(RefCell::new(Some(vec![1i64, 1i64, 2i64, 2i64, 2i64, 3i64, 1i64])))))), format!("{}", format_slice(&__slices__compact_func__slice_string__string(Rc::new(RefCell::new(Some(vec!["a".to_string(), "A".to_string(), "b".to_string()]))), Rc::new(RefCell::new(Some(Rc::new(move |a: Rc<RefCell<Option<String>>>, b: Rc<RefCell<Option<String>>>| -> bool {
        (*lower(Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })))).borrow().as_ref().unwrap()).clone() == (*lower(Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })))).borrow().as_ref().unwrap()).clone()
    }) as Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> bool>)))))));
    println!("{} {}", format!("{}", format_slice(&__slices__concat__slice_int__int(Rc::new(RefCell::new(Some(vec![{ let __slice_holder = Rc::new(RefCell::new(Some(vec![1i64]))).clone(); let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }, Vec::new(), { let __slice_holder = Rc::new(RefCell::new(Some(vec![2i64, 3i64]))).clone(); let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }])))))), format!("{}", format_slice(&__slices__repeat__slice_string__string(Rc::new(RefCell::new(Some(vec!["x".to_string(), "y".to_string()]))), Rc::new(RefCell::new(Some(3)))))));
    let mut g = __slices__grow__slice_int__int(Rc::new(RefCell::new(Some(vec![1i64]))), Rc::new(RefCell::new(Some(10))));
    let mut g = { let __slice_header = GoSliceHeader::go_slice_header(&g); Rc::new(RefCell::new(__slice_header)) };
    println!("{} {} {} {}", format!("{}", ({ let __len = (*g.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", ((*g.borrow()).as_ref().map(|__v| __v.capacity()).unwrap_or(0) as i64) >= (11 as i64)), format!("{}", ({ let __len = (*__slices__clip__slice_int__int(g.clone()).borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", (*__slices__clip__slice_int__int(g.clone()).borrow()).as_ref().map(|__v| __v.capacity()).unwrap_or(0)));
    let mut __rf_done_1: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));
    { let __f_holder = __slices__chunk__slice_int__int(Rc::new(RefCell::new(Some(vec![1i64, 2i64, 3i64, 4i64, 5i64]))), Rc::new(RefCell::new(Some(2)))); let __f: Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Vec<i64>>>>) -> bool>>>>) -> ()> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(Rc::new(move |__rf_param_1_0: Rc<RefCell<Option<Vec<i64>>>>| -> bool {
        let mut c = Rc::new(RefCell::new(__rf_param_1_0.borrow().clone()));
        print!("{}{}", format!("{}", format_slice(&c)), format!("{}", " ".to_string()));
        true
    }) as Rc<dyn Fn(Rc<RefCell<Option<Vec<i64>>>>) -> bool>)))) };
    if (*__rf_done_1.borrow().as_ref().unwrap()) {
        return;
    }
    println!();
    let mut __rf_done_2: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));
    { let __f_holder = __slices__backward__slice_string__string(Rc::new(RefCell::new(Some(vec!["a".to_string(), "b".to_string(), "c".to_string()])))); let __f: Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<String>>>) -> bool>>>>) -> ()> = __f_holder.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(Rc::new(move |__rf_param_2_0: Rc<RefCell<Option<i64>>>, __rf_param_2_1: Rc<RefCell<Option<String>>>| -> bool {
        let (mut i, mut v) = ({ let __owned = __rf_param_2_0.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) }, { let __owned = __rf_param_2_1.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) });
        print!("{}{}{}", format!("{}", { let __v = (*i.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*v.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", " ".to_string()));
        true
    }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<String>>>) -> bool>)))) };
    if (*__rf_done_2.borrow().as_ref().unwrap()) {
        return;
    }
    println!();
    println!("{} {}", format!("{}", __slices__compare__slice_int__int(Rc::new(RefCell::new(Some(vec![1i64, 2i64]))), Rc::new(RefCell::new(Some(vec![1i64, 3i64]))))), format!("{}", __slices__compare__slice_string__string(Rc::new(RefCell::new(Some(vec!["b".to_string()]))), Rc::new(RefCell::new(Some(vec!["a".to_string(), "z".to_string()]))))));
    println!("{}", format!("{}", __slices__equal_func__slice_int__slice_string__int__string(Rc::new(RefCell::new(Some(vec![1i64, 2i64]))), Rc::new(RefCell::new(Some(vec!["1".to_string(), "2".to_string()]))), Rc::new(RefCell::new(Some(Rc::new(move |a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<String>>>| -> bool {
        (*Rc::new(RefCell::new(Some(format!("{}", format!("{}", { let __v = (*a.borrow().as_ref().unwrap()).clone(); __v }))))).borrow().as_ref().unwrap()).clone() == (*b.borrow().as_ref().unwrap()).clone()
    }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<String>>>) -> bool>))))));
    let mut items = Rc::new(RefCell::new(Some(vec![Item { name: Rc::new(RefCell::new(Some("pen".to_string()))), price: Rc::new(RefCell::new(Some(3))), ..Default::default() }, Item { name: Rc::new(RefCell::new(Some("cup".to_string()))), price: Rc::new(RefCell::new(Some(5))), ..Default::default() }, Item { name: Rc::new(RefCell::new(Some("ink".to_string()))), price: Rc::new(RefCell::new(Some(3))), ..Default::default() }, Item { name: Rc::new(RefCell::new(Some("box".to_string()))), price: Rc::new(RefCell::new(Some(1))), ..Default::default() }])));
    __slices__sort_stable_func__slice__item___item(items.clone(), Rc::new(RefCell::new(Some(Rc::new(move |a: Rc<RefCell<Option<Item>>>, b: Rc<RefCell<Option<Item>>>| -> i64 {
        __cmp__compare__int(Rc::new(RefCell::new(Some({ let __selector_holder = (*a.borrow().as_ref().unwrap()).price.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }))), Rc::new(RefCell::new(Some({ let __selector_holder = (*b.borrow().as_ref().unwrap()).price.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }))))
    }) as Rc<dyn Fn(Rc<RefCell<Option<Item>>>, Rc<RefCell<Option<Item>>>) -> i64>))));
    { let __go_print_arg_1 = __slices__is_sorted_func__slice__item___item(items.clone(), Rc::new(RefCell::new(Some(Rc::new(move |a: Rc<RefCell<Option<Item>>>, b: Rc<RefCell<Option<Item>>>| -> i64 {
        return (*(*a.borrow().as_ref().unwrap()).price.borrow().as_ref().unwrap()) - (*(*b.borrow().as_ref().unwrap()).price.borrow().as_ref().unwrap());
    }) as Rc<dyn Fn(Rc<RefCell<Option<Item>>>, Rc<RefCell<Option<Item>>>) -> i64>)))); let __go_print_value = println!("{} {}", format!("{}", format_slice(&items)), format!("{}", __go_print_arg_1)); __go_print_value };
    let (mut n, mut ok) = __slices__binary_search_func__slice__item___item__int(items.clone(), Rc::new(RefCell::new(Some(5))), Rc::new(RefCell::new(Some(Rc::new(move |it: Rc<RefCell<Option<Item>>>, p: Rc<RefCell<Option<i64>>>| -> i64 {
        __cmp__compare__int(Rc::new(RefCell::new(Some({ let __selector_holder = (*it.borrow().as_ref().unwrap()).price.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }))), Rc::new(RefCell::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }) as Rc<dyn Fn(Rc<RefCell<Option<Item>>>, Rc<RefCell<Option<i64>>>) -> i64>))));
    println!("{} {}", format!("{}", n), format!("{}", ok));
    println!("{} {}", format!("{}", (*__slices__min_func__slice__item___item(items.clone(), Rc::new(RefCell::new(Some(Rc::new(move |a: Rc<RefCell<Option<Item>>>, b: Rc<RefCell<Option<Item>>>| -> i64 {
        __cmp__compare__string(Rc::new(RefCell::new(Some({ let __selector_holder = (*a.borrow().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }))), Rc::new(RefCell::new(Some({ let __selector_holder = (*b.borrow().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }))))
    }) as Rc<dyn Fn(Rc<RefCell<Option<Item>>>, Rc<RefCell<Option<Item>>>) -> i64>)))).borrow().as_ref().unwrap())), format!("{}", (*__slices__max_func__slice__item___item(items.clone(), Rc::new(RefCell::new(Some(Rc::new(move |a: Rc<RefCell<Option<Item>>>, b: Rc<RefCell<Option<Item>>>| -> i64 {
        return (*(*a.borrow().as_ref().unwrap()).price.borrow().as_ref().unwrap()) - (*(*b.borrow().as_ref().unwrap()).price.borrow().as_ref().unwrap());
    }) as Rc<dyn Fn(Rc<RefCell<Option<Item>>>, Rc<RefCell<Option<Item>>>) -> i64>)))).borrow().as_ref().unwrap())));
    { let __slice_header = GoSliceHeader::go_slice_header(&(__slices__delete_func__slice__item___item(items.clone(), Rc::new(RefCell::new(Some(Rc::new(move |it: Rc<RefCell<Option<Item>>>| -> bool {
        return (*(*it.borrow().as_ref().unwrap()).price.borrow().as_ref().unwrap()) == 3 as i64;
    }) as Rc<dyn Fn(Rc<RefCell<Option<Item>>>) -> bool>)))))); *items.borrow_mut() = __slice_header; };
    { let __go_print_arg_2 = __slices__index__slice__item___item(items.clone(), Rc::new(RefCell::new(Some(Item { name: Rc::new(RefCell::new(Some("cup".to_string()))), price: Rc::new(RefCell::new(Some(5))), ..Default::default() })))); let __go_print_arg_3 = __slices__contains__slice__item___item(items.clone(), Rc::new(RefCell::new(Some(Item { name: Rc::new(RefCell::new(Some("pen".to_string()))), price: Rc::new(RefCell::new(Some(3))), ..Default::default() })))); let __go_print_value = println!("{} {} {}", format!("{}", format_slice(&items)), format!("{}", __go_print_arg_2), format!("{}", __go_print_arg_3)); __go_print_value };
    let mut tasks = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(Task { name: Rc::new(RefCell::new(Some("write".to_string()))), pri: Rc::new(RefCell::new(Some(2))), ..Default::default() }))), Rc::new(RefCell::new(Some(Task { name: Rc::new(RefCell::new(Some("test".to_string()))), pri: Rc::new(RefCell::new(Some(1))), ..Default::default() }))), Rc::new(RefCell::new(Some(Task { name: Rc::new(RefCell::new(Some("ship".to_string()))), pri: Rc::new(RefCell::new(Some(3))), ..Default::default() })))])));
    __slices__sort_func__slice_ptr__task__ptr__task(tasks.clone(), Rc::new(RefCell::new(Some(Rc::new(move |a: Rc<RefCell<Option<Task>>>, b: Rc<RefCell<Option<Task>>>| -> i64 {
        __cmp__compare__int(Rc::new(RefCell::new(Some({ let __selector_holder = (*a.borrow().as_ref().unwrap()).pri.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }))), Rc::new(RefCell::new(Some({ let __selector_holder = (*b.borrow().as_ref().unwrap()).pri.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }))))
    }) as Rc<dyn Fn(Rc<RefCell<Option<Task>>>, Rc<RefCell<Option<Task>>>) -> i64>))));
    { let __range_holder = tasks.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for t in __range_values.iter() {
        print!("{}{}", format!("{}", (*(*t.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone()), format!("{}", " ".to_string()));
    } }
    println!();
    let mut i = __slices__index_func__slice_ptr__task__ptr__task(tasks.clone(), Rc::new(RefCell::new(Some(Rc::new(move |t: Rc<RefCell<Option<Task>>>| -> bool {
        return { let __selector_holder = (*t.borrow().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned } == "ship".to_string();
    }) as Rc<dyn Fn(Rc<RefCell<Option<Task>>>) -> bool>))));
    { let new_val = 10; *(*(*tasks.borrow().as_ref().unwrap())[(0) as usize].clone().borrow().as_ref().unwrap()).pri.borrow_mut() = Some(new_val); };
    { let __go_print_arg_4 = __slices__contains_func__slice_ptr__task__ptr__task(tasks.clone(), Rc::new(RefCell::new(Some(Rc::new(move |t: Rc<RefCell<Option<Task>>>| -> bool {
        return (*(*t.borrow().as_ref().unwrap()).pri.borrow().as_ref().unwrap()) == 10 as i64;
    }) as Rc<dyn Fn(Rc<RefCell<Option<Task>>>) -> bool>)))); let __go_print_arg_5 = __slices__index__slice_ptr__task__ptr__task(tasks.clone(), (*tasks.borrow().as_ref().unwrap())[(1) as usize].clone()); let __go_print_value = println!("{} {} {} {}", format!("{}", i), format!("{}", (*(*(*tasks.borrow().as_ref().unwrap())[(i) as usize].clone().borrow().as_ref().unwrap()).pri.borrow().as_ref().unwrap())), format!("{}", __go_print_arg_4), format!("{}", __go_print_arg_5)); __go_print_value };
    let mut temps = Rc::new(RefCell::new(Some(vec![Celsius(Rc::new(RefCell::new(Some(21.5)))), Celsius(Rc::new(RefCell::new(Some((-3.0))))), Celsius(Rc::new(RefCell::new(Some(37.0)))), Celsius(Rc::new(RefCell::new(Some(0.0))))])));
    __slices__sort__slice__celsius___celsius(temps.clone());
    { let __go_print_arg_6 = __slices__max__slice__celsius___celsius(temps.clone()); let __go_print_arg_7 = __slices__min__slice__celsius___celsius(temps.clone()); let __go_print_arg_8 = __cmp__less___celsius(Rc::new(RefCell::new(Some({ let __arg_value = (*temps.borrow().as_ref().unwrap())[(0) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*temps.borrow().as_ref().unwrap())[(1) as usize].clone(); __arg_value })))); let __go_print_value = println!("{} {} {} {}", format!("{}", format_slice(&temps)), format!("{}", { let __v = (*__go_print_arg_6.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*__go_print_arg_7.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", __go_print_arg_8)); __go_print_value };
    let mut ids = Rc::new(RefCell::new(Some(IDs(Rc::new(RefCell::new(Some(vec![9i64, 4i64, 7i64])))))));
    __slices__sort___i_ds__int(Rc::new(RefCell::new(Some({ let __arg_holder = ids.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let (mut idx, mut found) = __slices__binary_search___i_ds__int(Rc::new(RefCell::new(Some({ let __arg_holder = ids.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some(7))));
    { let __go_print_arg_9 = __slices__clone___i_ds__int(Rc::new(RefCell::new(Some({ let __arg_holder = ids.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __go_print_value = print!("{} {} {} {}\n", format_slice(&{ let __named_slice = (*ids.borrow().as_ref().unwrap()).0.clone(); __named_slice }), "main.IDs", idx, found); __go_print_value };
    let mut zero = Rc::new(RefCell::new(Some(0.0)));
    let (mut nan, mut negInf) = (Rc::new(RefCell::new(Some({ let __bin_zero = (*zero.borrow().as_ref().unwrap()).clone(); __bin_zero / __bin_zero }))), Rc::new(RefCell::new(Some(-1.0 / (*zero.borrow().as_ref().unwrap())))));
    let mut fl = Rc::new(RefCell::new(Some(vec![3.0, (*nan.borrow().as_ref().unwrap()).clone(), 1.0, (*negInf.borrow().as_ref().unwrap()).clone()])));
    __slices__sort__slice_float64__float64(fl.clone());
    println!("{} {} {} {}", format!("{}", format_slice(&fl)), format!("{}", __cmp__compare__float64(Rc::new(RefCell::new(Some({ let __arg_holder = nan.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some(1.0))))), format!("{}", __cmp__or__int(Rc::new(RefCell::new(Some(vec![0, 0, 4, 2]))))), format!("{}", (*__cmp__or__string(Rc::new(RefCell::new(Some(vec!["".to_string(), "x".to_string()])))).borrow().as_ref().unwrap())));
    let mut big = Rc::new(RefCell::new(Some(vec![0i64; (200) as usize])));
    for i in 0..(({ let __range_holder = big.clone(); let __range_guard = __range_holder.borrow(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        { let __idx = (i) as usize; let __rhs = (i as i64 * 7919) % 200; (*big.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    }
    __slices__sort__slice_int__int(big.clone());
    println!("{} {} {}", format!("{}", format_slice(&Rc::new(RefCell::new(Some({ let __seq_holder = big.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = 0; let __high = (5) as usize; let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) }))))), format!("{}", format_slice(&Rc::new(RefCell::new(Some({ let __seq_holder = big.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = (195) as usize; let __high = __seq.len(); let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) }))))), format!("{}", __slices__is_sorted__slice_int__int(big.clone())));
    __slices__sort_func__slice_int__int(big.clone(), Rc::new(RefCell::new(Some(Rc::new(move |a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>| -> i64 {
        (*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap())
    }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>))));
    println!("{}", format!("{}", format_slice(&Rc::new(RefCell::new(Some({ let __seq_holder = big.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = 0; let __high = (3) as usize; let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) }))))));
    __slices__reverse__slice_int__int(big.clone());
    println!("{} {}", format!("{}", format_slice(&Rc::new(RefCell::new(Some({ let __seq_holder = big.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = 0; let __high = (3) as usize; let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) }))))), format!("{}", ({ let __len = (*big.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })));
    let mut m = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i64>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(1i64)))), ("b".to_string(), Rc::new(RefCell::new(Some(2i64)))), ("c".to_string(), Rc::new(RefCell::new(Some(3i64))))]))));
    println!("{} {}", format!("{}", format_slice(&__slices__sorted__string(__maps__keys__map_string_int__string__int(m.clone())))), format!("{}", format_slice(&__slices__sorted__int(__maps__values__map_string_int__string__int(m.clone())))));
    println!("{}", format!("{}", format_map(&__maps__collect__int__string(__slices__all__slice_string__string(Rc::new(RefCell::new(Some(vec!["x".to_string(), "y".to_string()]))))))));
    __maps__copy__map_string_int__map_string_int__string__int(m.clone(), Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i64>>>>::from([("d".to_string(), Rc::new(RefCell::new(Some(4i64)))), ("a".to_string(), Rc::new(RefCell::new(Some(10i64))))])))));
    __maps__delete_func__map_string_int__string__int(m.clone(), Rc::new(RefCell::new(Some(Rc::new(move |k: Rc<RefCell<Option<String>>>, v: Rc<RefCell<Option<i64>>>| -> bool {
        (*v.borrow().as_ref().unwrap()) % 2 == 0
    }) as Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<i64>>>) -> bool>))));
    { let __go_print_arg_10 = __maps__equal_func__map_string_int__map_string_int__string__int__int(m.clone(), Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i64>>>>::from([("c".to_string(), Rc::new(RefCell::new(Some(3i64))))])))), Rc::new(RefCell::new(Some(Rc::new(move |a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>| -> bool {
        (*a.borrow().as_ref().unwrap()) == (*b.borrow().as_ref().unwrap())
    }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> bool>)))); let __go_print_value = println!("{} {}", format!("{}", format_map(&m)), format!("{}", __go_print_arg_10)); __go_print_value };
    __maps__insert__map_string_int__string__int(m.clone(), __maps__all__map_string_int__string__int(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i64>>>>::from([("z".to_string(), Rc::new(RefCell::new(Some(26i64))))]))))));
    let mut m2 = __maps__clone__map_string_int__string__int(m.clone());
    { let __map_key = "y".to_string(); let __map_value = Rc::new(RefCell::new(Some(25))); (*m2.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    { let __go_print_arg_11 = Rc::new(RefCell::new(Some(({ let __len = (*m.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64))); let __go_print_arg_12 = Rc::new(RefCell::new(Some(({ let __len = (*m2.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64))); let __go_print_arg_13 = __maps__equal__map_string_int__map_string_int__string__int(m.clone(), m2.clone()); let __go_print_value = println!("{} {} {} {}", format!("{}", { let __v = (*__go_print_arg_11.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*__go_print_arg_12.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __map_holder = m2.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"z".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", __go_print_arg_13)); __go_print_value };
    let mut nilMap: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>> = Rc::new(RefCell::new(None));
    println!("{} {}", format!("{}", { let __nil_result = (*__maps__clone__map_string_int__string__int(nilMap.clone()).borrow()).is_none(); __nil_result }), format!("{}", { let __nil_result = (*__slices__collect__string(__maps__keys__map_string_int__string__int(nilMap.clone())).borrow()).is_none(); __nil_result }));
}

pub fn __slices__insert__slice_int__int(mut s: Rc<RefCell<Option<Vec<i64>>>>, i: Rc<RefCell<Option<i64>>>, v: Rc<RefCell<Option<Vec<i64>>>>) -> Rc<RefCell<Option<Vec<i64>>>> {
    let mut s = Rc::new(RefCell::new((*s.borrow()).clone()));
    let _ = Rc::new(RefCell::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = ((*i.borrow().as_ref().unwrap())) as usize; let __high = __seq.len(); let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) })));
    let mut m = Rc::new(RefCell::new(Some(({ let __len = (*v.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64)));
    if (*m.borrow().as_ref().unwrap()) == 0 {
        return s.clone();
    }
    let mut n = Rc::new(RefCell::new(Some(({ let __len = (*s.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64)));
    if (*i.borrow().as_ref().unwrap()) == (*n.borrow().as_ref().unwrap()) {
        return { let __append_target = s.clone(); let __append_values: Vec<_> = { let __slice_holder = v.clone(); let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned().collect(); let mut __appended: Vec<i64> = (*__append_target.borrow()).clone().unwrap_or_default(); __appended.extend(__append_values); let __append_was_nil = (*__append_target.borrow()).is_none(); Rc::new(RefCell::new(if __append_was_nil && __appended.len() == 0 { None } else { Some(__appended) })) };
    }
    if ((*n.borrow().as_ref().unwrap()) + (*m.borrow().as_ref().unwrap()) as i64) > ((*s.borrow()).as_ref().map(|__v| __v.capacity()).unwrap_or(0) as i64) {
        let mut s2 = { let __append_target = Rc::new(RefCell::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = 0; let __high = ((*i.borrow().as_ref().unwrap())) as usize; let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) }))).clone(); let __append_values: Vec<_> = { let __slice_holder = Rc::new(RefCell::new(Some(vec![0i64; ((*n.borrow().as_ref().unwrap()) + (*m.borrow().as_ref().unwrap()) - (*i.borrow().as_ref().unwrap())) as usize]))).clone(); let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned().collect(); let mut __appended: Vec<i64> = (*__append_target.borrow()).clone().unwrap_or_default(); __appended.extend(__append_values); let __append_was_nil = (*__append_target.borrow()).is_none(); Rc::new(RefCell::new(if __append_was_nil && __appended.len() == 0 { None } else { Some(__appended) })) };
        {
            let _dst_start = ((*i.borrow().as_ref().unwrap())) as usize;
            let _dst_len = (*s2.borrow().as_ref().unwrap()).len() - _dst_start;
            let _src = { let __copy_src_holder = v.clone(); let __copy_src_guard = __copy_src_holder.borrow(); __copy_src_guard.as_ref().map(|__s| __s.to_vec()).unwrap_or_default() };
            let _n = std::cmp::min(_dst_len, _src.len());
            for _i in 0.._n {
                (*s2.borrow_mut().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
            }
            Rc::new(RefCell::new(Some(_n as i64)))
        };
        {
            let _dst_start = ((*i.borrow().as_ref().unwrap()) + (*m.borrow().as_ref().unwrap())) as usize;
            let _dst_len = (*s2.borrow().as_ref().unwrap()).len() - _dst_start;
            let _src = (*Rc::new(RefCell::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = ((*i.borrow().as_ref().unwrap())) as usize; let __high = __seq.len(); let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) }))).borrow().as_ref().unwrap()).to_vec();
            let _n = std::cmp::min(_dst_len, _src.len());
            for _i in 0.._n {
                (*s2.borrow_mut().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
            }
            Rc::new(RefCell::new(Some(_n as i64)))
        };
        return s2.clone();
    }
    { let __slice_header = GoSliceHeader::go_slice_header(&(Rc::new(RefCell::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = 0; let __high = ((*n.borrow().as_ref().unwrap()) + (*m.borrow().as_ref().unwrap())) as usize; let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) }))))); *s.borrow_mut() = __slice_header; };
    if !__slices_overlaps__int(v.clone(), Rc::new(RefCell::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = ((*i.borrow().as_ref().unwrap()) + (*m.borrow().as_ref().unwrap())) as usize; let __high = __seq.len(); let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) })))) {
        {
            let _dst_start = ((*i.borrow().as_ref().unwrap()) + (*m.borrow().as_ref().unwrap())) as usize;
            let _dst_len = (*s.borrow().as_ref().unwrap()).len() - _dst_start;
            let _src = (*Rc::new(RefCell::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = ((*i.borrow().as_ref().unwrap())) as usize; let __high = __seq.len(); let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) }))).borrow().as_ref().unwrap()).to_vec();
            let _n = std::cmp::min(_dst_len, _src.len());
            for _i in 0.._n {
                (*s.borrow_mut().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
            }
            Rc::new(RefCell::new(Some(_n as i64)))
        };
        {
            let _dst_start = ((*i.borrow().as_ref().unwrap())) as usize;
            let _dst_len = (*s.borrow().as_ref().unwrap()).len() - _dst_start;
            let _src = { let __copy_src_holder = v.clone(); let __copy_src_guard = __copy_src_holder.borrow(); __copy_src_guard.as_ref().map(|__s| __s.to_vec()).unwrap_or_default() };
            let _n = std::cmp::min(_dst_len, _src.len());
            for _i in 0.._n {
                (*s.borrow_mut().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
            }
            Rc::new(RefCell::new(Some(_n as i64)))
        };
        return s.clone();
    }
    {
        let _dst_start = ((*n.borrow().as_ref().unwrap())) as usize;
        let _dst_len = (*s.borrow().as_ref().unwrap()).len() - _dst_start;
        let _src = { let __copy_src_holder = v.clone(); let __copy_src_guard = __copy_src_holder.borrow(); __copy_src_guard.as_ref().map(|__s| __s.to_vec()).unwrap_or_default() };
        let _n = std::cmp::min(_dst_len, _src.len());
        for _i in 0.._n {
            (*s.borrow_mut().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
        }
        Rc::new(RefCell::new(Some(_n as i64)))
    };
    __slices_rotate_right__int(Rc::new(RefCell::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = ((*i.borrow().as_ref().unwrap())) as usize; let __high = __seq.len(); let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) }))), Rc::new(RefCell::new(Some({ let __arg_holder = m.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    return s.clone();
}

pub fn __slices__replace__slice_int__int(s: Rc<RefCell<Option<Vec<i64>>>>, i: Rc<RefCell<Option<i64>>>, j: Rc<RefCell<Option<i64>>>, v: Rc<RefCell<Option<Vec<i64>>>>) -> Rc<RefCell<Option<Vec<i64>>>> {
    let _ = Rc::new(RefCell::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = ((*i.borrow().as_ref().unwrap())) as usize; let __high = ((*j.borrow().as_ref().unwrap())) as usize; let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) })));
    if (*i.borrow().as_ref().unwrap()) == (*j.borrow().as_ref().unwrap()) {
        return __slices__insert__slice_int__int(s.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), v.clone());
    }
    if ((*j.borrow().as_ref().unwrap()) as i64) == (({ let __len = (*s.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) {
        let mut s2 = { let __append_target = Rc::new(RefCell::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = 0; let __high = ((*i.borrow().as_ref().unwrap())) as usize; let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) }))).clone(); let __append_values: Vec<_> = { let __slice_holder = v.clone(); let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned().collect(); let mut __appended: Vec<i64> = (*__append_target.borrow()).clone().unwrap_or_default(); __appended.extend(__append_values); let __append_was_nil = (*__append_target.borrow()).is_none(); Rc::new(RefCell::new(if __append_was_nil && __appended.len() == 0 { None } else { Some(__appended) })) };
        if (({ let __len = (*s2.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) < (({ let __len = (*s.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) {
        { let __clear_start = (({ let __len = (*s2.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })) as usize; let __clear_end = { let __clear_len_holder = s.clone(); let __clear_len_guard = __clear_len_holder.borrow(); __clear_len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; let __clear_holder = s.clone(); let mut __clear_guard = __clear_holder.borrow_mut(); if let Some(__clear_seq) = __clear_guard.as_mut() { let __clear_cap = __clear_seq.capacity(); let mut __clear_window = __clear_seq.reslice3_zeroed(__clear_start, __clear_end, __clear_cap); for __clear_elem in __clear_window.as_mut_slice().iter_mut() { *__clear_elem = 0; } } };
    }
        return s2.clone();
    }
    let mut tot = Rc::new(RefCell::new(Some(((({ let __len = (*Rc::new(RefCell::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = 0; let __high = ((*i.borrow().as_ref().unwrap())) as usize; let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) }))).borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) + (({ let __len = (*v.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) as i64) + (({ let __len = (*Rc::new(RefCell::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = ((*j.borrow().as_ref().unwrap())) as usize; let __high = __seq.len(); let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) }))).borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64))));
    if ((*tot.borrow().as_ref().unwrap()) as i64) > ((*s.borrow()).as_ref().map(|__v| __v.capacity()).unwrap_or(0) as i64) {
        let mut s2 = { let __append_target = Rc::new(RefCell::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = 0; let __high = ((*i.borrow().as_ref().unwrap())) as usize; let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) }))).clone(); let __append_values: Vec<_> = { let __slice_holder = Rc::new(RefCell::new(Some(vec![0i64; ((*tot.borrow().as_ref().unwrap()) - (*i.borrow().as_ref().unwrap())) as usize]))).clone(); let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned().collect(); let mut __appended: Vec<i64> = (*__append_target.borrow()).clone().unwrap_or_default(); __appended.extend(__append_values); let __append_was_nil = (*__append_target.borrow()).is_none(); Rc::new(RefCell::new(if __append_was_nil && __appended.len() == 0 { None } else { Some(__appended) })) };
        {
            let _dst_start = ((*i.borrow().as_ref().unwrap())) as usize;
            let _dst_len = (*s2.borrow().as_ref().unwrap()).len() - _dst_start;
            let _src = { let __copy_src_holder = v.clone(); let __copy_src_guard = __copy_src_holder.borrow(); __copy_src_guard.as_ref().map(|__s| __s.to_vec()).unwrap_or_default() };
            let _n = std::cmp::min(_dst_len, _src.len());
            for _i in 0.._n {
                (*s2.borrow_mut().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
            }
            Rc::new(RefCell::new(Some(_n as i64)))
        };
        {
            let _dst_start = (((*i.borrow().as_ref().unwrap()) as i64) + (({ let __len = (*v.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64)) as usize;
            let _dst_len = (*s2.borrow().as_ref().unwrap()).len() - _dst_start;
            let _src = (*Rc::new(RefCell::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = ((*j.borrow().as_ref().unwrap())) as usize; let __high = __seq.len(); let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) }))).borrow().as_ref().unwrap()).to_vec();
            let _n = std::cmp::min(_dst_len, _src.len());
            for _i in 0.._n {
                (*s2.borrow_mut().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
            }
            Rc::new(RefCell::new(Some(_n as i64)))
        };
        return s2.clone();
    }
    let mut r = Rc::new(RefCell::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = 0; let __high = ((*tot.borrow().as_ref().unwrap())) as usize; let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) })));
    if (((*i.borrow().as_ref().unwrap()) as i64) + (({ let __len = (*v.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) as i64) <= ((*j.borrow().as_ref().unwrap()) as i64) {
        {
            let _dst_start = ((*i.borrow().as_ref().unwrap())) as usize;
            let _dst_len = (*r.borrow().as_ref().unwrap()).len() - _dst_start;
            let _src = { let __copy_src_holder = v.clone(); let __copy_src_guard = __copy_src_holder.borrow(); __copy_src_guard.as_ref().map(|__s| __s.to_vec()).unwrap_or_default() };
            let _n = std::cmp::min(_dst_len, _src.len());
            for _i in 0.._n {
                (*r.borrow_mut().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
            }
            Rc::new(RefCell::new(Some(_n as i64)))
        };
        {
            let _dst_start = (((*i.borrow().as_ref().unwrap()) as i64) + (({ let __len = (*v.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64)) as usize;
            let _dst_len = (*r.borrow().as_ref().unwrap()).len() - _dst_start;
            let _src = (*Rc::new(RefCell::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = ((*j.borrow().as_ref().unwrap())) as usize; let __high = __seq.len(); let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) }))).borrow().as_ref().unwrap()).to_vec();
            let _n = std::cmp::min(_dst_len, _src.len());
            for _i in 0.._n {
                (*r.borrow_mut().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
            }
            Rc::new(RefCell::new(Some(_n as i64)))
        };
        { let __clear_start = ((*tot.borrow().as_ref().unwrap())) as usize; let __clear_end = { let __clear_len_holder = s.clone(); let __clear_len_guard = __clear_len_holder.borrow(); __clear_len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; let __clear_holder = s.clone(); let mut __clear_guard = __clear_holder.borrow_mut(); if let Some(__clear_seq) = __clear_guard.as_mut() { let __clear_cap = __clear_seq.capacity(); let mut __clear_window = __clear_seq.reslice3_zeroed(__clear_start, __clear_end, __clear_cap); for __clear_elem in __clear_window.as_mut_slice().iter_mut() { *__clear_elem = 0; } } };
        return r.clone();
    }
    if !__slices_overlaps__int(Rc::new(RefCell::new(Some({ let __seq_holder = r.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = (((*i.borrow().as_ref().unwrap()) as i64) + (({ let __len = (*v.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64)) as usize; let __high = __seq.len(); let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) }))), v.clone()) {
        {
            let _dst_start = (((*i.borrow().as_ref().unwrap()) as i64) + (({ let __len = (*v.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64)) as usize;
            let _dst_len = (*r.borrow().as_ref().unwrap()).len() - _dst_start;
            let _src = (*Rc::new(RefCell::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = ((*j.borrow().as_ref().unwrap())) as usize; let __high = __seq.len(); let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) }))).borrow().as_ref().unwrap()).to_vec();
            let _n = std::cmp::min(_dst_len, _src.len());
            for _i in 0.._n {
                (*r.borrow_mut().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
            }
            Rc::new(RefCell::new(Some(_n as i64)))
        };
        {
            let _dst_start = ((*i.borrow().as_ref().unwrap())) as usize;
            let _dst_len = (*r.borrow().as_ref().unwrap()).len() - _dst_start;
            let _src = { let __copy_src_holder = v.clone(); let __copy_src_guard = __copy_src_holder.borrow(); __copy_src_guard.as_ref().map(|__s| __s.to_vec()).unwrap_or_default() };
            let _n = std::cmp::min(_dst_len, _src.len());
            for _i in 0.._n {
                (*r.borrow_mut().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
            }
            Rc::new(RefCell::new(Some(_n as i64)))
        };
        return r.clone();
    }
    let mut y = Rc::new(RefCell::new(Some((({ let __len = (*v.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) - (((*j.borrow().as_ref().unwrap()) - (*i.borrow().as_ref().unwrap())) as i64))));
    if !__slices_overlaps__int(Rc::new(RefCell::new(Some({ let __seq_holder = r.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = ((*i.borrow().as_ref().unwrap())) as usize; let __high = ((*j.borrow().as_ref().unwrap())) as usize; let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) }))), v.clone()) {
        {
            let _dst_start = ((*i.borrow().as_ref().unwrap())) as usize;
            let _dst_len = (((*j.borrow().as_ref().unwrap())) as usize) - _dst_start;
            let _src = (*Rc::new(RefCell::new(Some({ let __seq_holder = v.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = ((*y.borrow().as_ref().unwrap())) as usize; let __high = __seq.len(); let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) }))).borrow().as_ref().unwrap()).to_vec();
            let _n = std::cmp::min(_dst_len, _src.len());
            for _i in 0.._n {
                (*r.borrow_mut().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
            }
            Rc::new(RefCell::new(Some(_n as i64)))
        };
        {
            let _dst_start = (({ let __len = (*s.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })) as usize;
            let _dst_len = (*r.borrow().as_ref().unwrap()).len() - _dst_start;
            let _src = (*Rc::new(RefCell::new(Some({ let __seq_holder = v.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = 0; let __high = ((*y.borrow().as_ref().unwrap())) as usize; let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) }))).borrow().as_ref().unwrap()).to_vec();
            let _n = std::cmp::min(_dst_len, _src.len());
            for _i in 0.._n {
                (*r.borrow_mut().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
            }
            Rc::new(RefCell::new(Some(_n as i64)))
        };
        __slices_rotate_right__int(Rc::new(RefCell::new(Some({ let __seq_holder = r.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = ((*i.borrow().as_ref().unwrap())) as usize; let __high = __seq.len(); let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) }))), Rc::new(RefCell::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return r.clone();
    }
    if !__slices_overlaps__int(Rc::new(RefCell::new(Some({ let __seq_holder = r.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = (({ let __len = (*s.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })) as usize; let __high = __seq.len(); let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) }))), v.clone()) {
        {
            let _dst_start = (({ let __len = (*s.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })) as usize;
            let _dst_len = (*r.borrow().as_ref().unwrap()).len() - _dst_start;
            let _src = (*Rc::new(RefCell::new(Some({ let __seq_holder = v.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = 0; let __high = ((*y.borrow().as_ref().unwrap())) as usize; let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) }))).borrow().as_ref().unwrap()).to_vec();
            let _n = std::cmp::min(_dst_len, _src.len());
            for _i in 0.._n {
                (*r.borrow_mut().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
            }
            Rc::new(RefCell::new(Some(_n as i64)))
        };
        {
            let _dst_start = ((*i.borrow().as_ref().unwrap())) as usize;
            let _dst_len = (((*j.borrow().as_ref().unwrap())) as usize) - _dst_start;
            let _src = (*Rc::new(RefCell::new(Some({ let __seq_holder = v.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = ((*y.borrow().as_ref().unwrap())) as usize; let __high = __seq.len(); let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) }))).borrow().as_ref().unwrap()).to_vec();
            let _n = std::cmp::min(_dst_len, _src.len());
            for _i in 0.._n {
                (*r.borrow_mut().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
            }
            Rc::new(RefCell::new(Some(_n as i64)))
        };
        __slices_rotate_right__int(Rc::new(RefCell::new(Some({ let __seq_holder = r.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = ((*i.borrow().as_ref().unwrap())) as usize; let __high = __seq.len(); let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) }))), Rc::new(RefCell::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return r.clone();
    }
    let mut k = __slices_start_idx__int(v.clone(), Rc::new(RefCell::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = ((*j.borrow().as_ref().unwrap())) as usize; let __high = __seq.len(); let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) }))));
    {
        let _dst_start = ((*i.borrow().as_ref().unwrap())) as usize;
        let _dst_len = (*r.borrow().as_ref().unwrap()).len() - _dst_start;
        let _src = { let __copy_src_holder = v.clone(); let __copy_src_guard = __copy_src_holder.borrow(); __copy_src_guard.as_ref().map(|__s| __s.to_vec()).unwrap_or_default() };
        let _n = std::cmp::min(_dst_len, _src.len());
        for _i in 0.._n {
            (*r.borrow_mut().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
        }
        Rc::new(RefCell::new(Some(_n as i64)))
    };
    {
        let _dst_start = (((*i.borrow().as_ref().unwrap()) as i64) + (({ let __len = (*v.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64)) as usize;
        let _dst_len = (*r.borrow().as_ref().unwrap()).len() - _dst_start;
        let _src = (*Rc::new(RefCell::new(Some({ let __seq_holder = r.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = ((*i.borrow().as_ref().unwrap()) + k) as usize; let __high = __seq.len(); let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) }))).borrow().as_ref().unwrap()).to_vec();
        let _n = std::cmp::min(_dst_len, _src.len());
        for _i in 0.._n {
            (*r.borrow_mut().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
        }
        Rc::new(RefCell::new(Some(_n as i64)))
    };
    return r.clone();
}

pub fn __slices__delete__slice_int__int(mut s: Rc<RefCell<Option<Vec<i64>>>>, i: Rc<RefCell<Option<i64>>>, j: Rc<RefCell<Option<i64>>>) -> Rc<RefCell<Option<Vec<i64>>>> {
    let mut s = Rc::new(RefCell::new((*s.borrow()).clone()));
    let _ = Rc::new(RefCell::new(Some({ let mut __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; let __low = ((*i.borrow().as_ref().unwrap())) as usize; let __high = ((*j.borrow().as_ref().unwrap())) as usize; let __max = (({ let __len = (*s.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })) as usize; __seq.reslice3_zeroed(__low, __high, __max) })));
    if (*i.borrow().as_ref().unwrap()) == (*j.borrow().as_ref().unwrap()) {
        return s.clone();
    }
    let mut oldlen = Rc::new(RefCell::new(Some(({ let __len = (*s.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64)));
    { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = Rc::new(RefCell::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = 0; let __high = ((*i.borrow().as_ref().unwrap())) as usize; let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) }))).clone(); let __append_values: Vec<_> = { let __slice_holder = Rc::new(RefCell::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = ((*j.borrow().as_ref().unwrap())) as usize; let __high = __seq.len(); let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) }))).clone(); let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned().collect(); let mut __appended: Vec<i64> = (*__append_target.borrow()).clone().unwrap_or_default(); __appended.extend(__append_values); let __append_was_nil = (*__append_target.borrow()).is_none(); Rc::new(RefCell::new(if __append_was_nil && __appended.len() == 0 { None } else { Some(__appended) })) })); *s.borrow_mut() = __slice_header; };
    { let __clear_start = (({ let __len = (*s.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })) as usize; let __clear_end = ((*oldlen.borrow().as_ref().unwrap())) as usize; let __clear_holder = s.clone(); let mut __clear_guard = __clear_holder.borrow_mut(); if let Some(__clear_seq) = __clear_guard.as_mut() { let __clear_cap = __clear_seq.capacity(); let mut __clear_window = __clear_seq.reslice3_zeroed(__clear_start, __clear_end, __clear_cap); for __clear_elem in __clear_window.as_mut_slice().iter_mut() { *__clear_elem = 0; } } };
    return s.clone();
}

pub fn __slices__compact__slice_int__int(s: Rc<RefCell<Option<Vec<i64>>>>) -> Rc<RefCell<Option<Vec<i64>>>> {
    if (({ let __len = (*s.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) < (2 as i64) {
        return s.clone();
    }
    let mut k = Rc::new(RefCell::new(Some(1i64)));
    while ((*k.borrow().as_ref().unwrap()) as i64) < (({ let __len = (*s.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) {
        if (*s.borrow().as_ref().unwrap())[((*k.borrow().as_ref().unwrap())) as usize].clone() == (*s.borrow().as_ref().unwrap())[((*k.borrow().as_ref().unwrap()) - 1) as usize].clone() {
        let mut s2 = Rc::new(RefCell::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = ((*k.borrow().as_ref().unwrap())) as usize; let __high = __seq.len(); let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) })));
        let mut k2 = Rc::new(RefCell::new(Some(1i64)));
    while ((*k2.borrow().as_ref().unwrap()) as i64) < (({ let __len = (*s2.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) {
        if (*s2.borrow().as_ref().unwrap())[((*k2.borrow().as_ref().unwrap())) as usize].clone() != (*s2.borrow().as_ref().unwrap())[((*k2.borrow().as_ref().unwrap()) - 1) as usize].clone() {
        { let __idx = ((*k.borrow().as_ref().unwrap())) as usize; let __rhs = (*s2.borrow().as_ref().unwrap())[((*k2.borrow().as_ref().unwrap())) as usize].clone(); (*s.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
        { let mut guard = k.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let mut guard = k2.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let __clear_start = ((*k.borrow().as_ref().unwrap())) as usize; let __clear_end = { let __clear_len_holder = s.clone(); let __clear_len_guard = __clear_len_holder.borrow(); __clear_len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; let __clear_holder = s.clone(); let mut __clear_guard = __clear_holder.borrow_mut(); if let Some(__clear_seq) = __clear_guard.as_mut() { let __clear_cap = __clear_seq.capacity(); let mut __clear_window = __clear_seq.reslice3_zeroed(__clear_start, __clear_end, __clear_cap); for __clear_elem in __clear_window.as_mut_slice().iter_mut() { *__clear_elem = 0; } } };
        return Rc::new(RefCell::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = 0; let __high = ((*k.borrow().as_ref().unwrap())) as usize; let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) })));
    }
        { let mut guard = k.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    s.clone()
}

pub fn __slices__compact_func__slice_string__string(s: Rc<RefCell<Option<Vec<String>>>>, eq: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> bool>>>>) -> Rc<RefCell<Option<Vec<String>>>> {
    if (({ let __len = (*s.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) < (2 as i64) {
        return s.clone();
    }
    let mut k = Rc::new(RefCell::new(Some(1i64)));
    while ((*k.borrow().as_ref().unwrap()) as i64) < (({ let __len = (*s.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) {
        if { let __f: Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> bool> = eq.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*s.borrow().as_ref().unwrap())[((*k.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*s.borrow().as_ref().unwrap())[((*k.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value })))) } {
        let mut s2 = Rc::new(RefCell::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = ((*k.borrow().as_ref().unwrap())) as usize; let __high = __seq.len(); let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) })));
        let mut k2 = Rc::new(RefCell::new(Some(1i64)));
    while ((*k2.borrow().as_ref().unwrap()) as i64) < (({ let __len = (*s2.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) {
        if !{ let __f: Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> bool> = eq.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*s2.borrow().as_ref().unwrap())[((*k2.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*s2.borrow().as_ref().unwrap())[((*k2.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value })))) } {
        { let __idx = ((*k.borrow().as_ref().unwrap())) as usize; let __rhs = (*s2.borrow().as_ref().unwrap())[((*k2.borrow().as_ref().unwrap())) as usize].clone(); (*s.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
        { let mut guard = k.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let mut guard = k2.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let __clear_start = ((*k.borrow().as_ref().unwrap())) as usize; let __clear_end = { let __clear_len_holder = s.clone(); let __clear_len_guard = __clear_len_holder.borrow(); __clear_len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; let __clear_holder = s.clone(); let mut __clear_guard = __clear_holder.borrow_mut(); if let Some(__clear_seq) = __clear_guard.as_mut() { let __clear_cap = __clear_seq.capacity(); let mut __clear_window = __clear_seq.reslice3_zeroed(__clear_start, __clear_end, __clear_cap); for __clear_elem in __clear_window.as_mut_slice().iter_mut() { *__clear_elem = String::new(); } } };
        return Rc::new(RefCell::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = 0; let __high = ((*k.borrow().as_ref().unwrap())) as usize; let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) })));
    }
        { let mut guard = k.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    s.clone()
}

pub fn __slices__concat__slice_int__int(slices: Rc<RefCell<Option<Vec<Vec<i64>>>>>) -> Rc<RefCell<Option<Vec<i64>>>> {
    let mut size = Rc::new(RefCell::new(Some(0i64)));
    { let __range_holder = slices.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for s in __range_values.iter() {
        { let __rhs = s.len() as i64; let mut guard = size.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        if (*size.borrow().as_ref().unwrap()) < 0 {
        std::panic::panic_any(format!("len out of range"));
    }
    } }
    let mut newslice = __slices__grow__slice_int__int(Rc::new(RefCell::new(None)), Rc::new(RefCell::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let mut newslice = { let __slice_header = GoSliceHeader::go_slice_header(&newslice); Rc::new(RefCell::new(__slice_header)) };
    { let __range_holder = slices.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for s in __range_values.iter() {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = newslice.clone(); let __append_values: Vec<_> = s.iter().cloned().collect(); let mut __appended: Vec<i64> = (*__append_target.borrow()).clone().unwrap_or_default(); __appended.extend(__append_values); let __append_was_nil = (*__append_target.borrow()).is_none(); Rc::new(RefCell::new(if __append_was_nil && __appended.len() == 0 { None } else { Some(__appended) })) })); *newslice.borrow_mut() = __slice_header; };
    } }
    return newslice.clone();
}

pub fn __slices__repeat__slice_string__string(x: Rc<RefCell<Option<Vec<String>>>>, count: Rc<RefCell<Option<i64>>>) -> Rc<RefCell<Option<Vec<String>>>> {
    if (*count.borrow().as_ref().unwrap()) < 0 {
        std::panic::panic_any(format!("cannot be negative"));
    }
    const maxInt: u64 = !(0 as u64) >> 1;

    let (mut hi, mut lo) = math_bits::mul(Rc::new(RefCell::new(Some({ let __arg_value = ({ let __len = (*x.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as u64; __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*count.borrow().as_ref().unwrap()) as u64; __arg_value }))));
    if hi > 0 as u64 || lo > maxInt as u64 {
        std::panic::panic_any(format!("the result of (len(x) * count) overflows"));
    }
    let mut newslice = Rc::new(RefCell::new(Some(vec!["".to_string(); (*Rc::new(RefCell::new(Some(lo as i64))).borrow().as_ref().unwrap()) as usize])));
    let mut n = {
        let _src = { let __copy_src_holder = x.clone(); let __copy_src_guard = __copy_src_holder.borrow(); __copy_src_guard.as_ref().map(|__s| __s.to_vec()).unwrap_or_default() };
        let _n = std::cmp::min((*newslice.borrow().as_ref().unwrap()).len(), _src.len());
        for _i in 0.._n {
            (*newslice.borrow_mut().as_mut().unwrap())[_i] = _src[_i].clone();
        }
        Rc::new(RefCell::new(Some(_n as i64)))
    };
    while ((*n.borrow().as_ref().unwrap()) as i64) < (({ let __len = (*newslice.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) {
        { let __rhs = (*{
            let _dst_start = ((*n.borrow().as_ref().unwrap())) as usize;
            let _dst_len = (*newslice.borrow().as_ref().unwrap()).len() - _dst_start;
            let _src = (*Rc::new(RefCell::new(Some({ let __seq_holder = newslice.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = 0; let __high = ((*n.borrow().as_ref().unwrap())) as usize; let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) }))).borrow().as_ref().unwrap()).to_vec();
            let _n = std::cmp::min(_dst_len, _src.len());
            for _i in 0.._n {
                (*newslice.borrow_mut().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
            }
            Rc::new(RefCell::new(Some(_n as i64)))
        }.borrow().as_ref().unwrap()); let mut guard = n.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    return newslice.clone();
}

pub fn __slices__grow__slice_int__int(mut s: Rc<RefCell<Option<Vec<i64>>>>, mut n: Rc<RefCell<Option<i64>>>) -> Rc<RefCell<Option<Vec<i64>>>> {
    let mut s = Rc::new(RefCell::new((*s.borrow()).clone()));
    if (*n.borrow().as_ref().unwrap()) < 0 {
        std::panic::panic_any(format!("cannot be negative"));
    }
    {
        { let __rhs = ((*s.borrow()).as_ref().map(|__v| __v.capacity()).unwrap_or(0) as i64) - (({ let __len = (*s.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); let mut guard = n.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - __rhs); };;
        if (*n.borrow().as_ref().unwrap()) > 0 {
            { let __slice_header = GoSliceHeader::go_slice_header(&(Rc::new(RefCell::new(Some({ let __seq_holder = { let __append_target = Rc::new(RefCell::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = 0; let __high = ((*s.borrow()).as_ref().map(|__v| __v.capacity()).unwrap_or(0)) as usize; let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) }))).clone(); let __append_values: Vec<_> = { let __slice_holder = Rc::new(RefCell::new(Some(vec![0i64; ((*n.borrow().as_ref().unwrap())) as usize]))).clone(); let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned().collect(); let mut __appended: Vec<i64> = (*__append_target.borrow()).clone().unwrap_or_default(); __appended.extend(__append_values); let __append_was_nil = (*__append_target.borrow()).is_none(); Rc::new(RefCell::new(if __append_was_nil && __appended.len() == 0 { None } else { Some(__appended) })) }.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = 0; let __high = (({ let __len = (*s.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })) as usize; let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) }))))); *s.borrow_mut() = __slice_header; };;
        }
    }
    return s.clone();
}

pub fn __slices__clip__slice_int__int(s: Rc<RefCell<Option<Vec<i64>>>>) -> Rc<RefCell<Option<Vec<i64>>>> {
    Rc::new(RefCell::new(Some({ let mut __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; let __low = 0; let __high = (({ let __len = (*s.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })) as usize; let __max = (({ let __len = (*s.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })) as usize; __seq.reslice3_zeroed(__low, __high, __max) })))
}

pub fn __slices__chunk__slice_int__int(s: Rc<RefCell<Option<Vec<i64>>>>, n: Rc<RefCell<Option<i64>>>) -> Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Vec<i64>>>>) -> bool>>>>) -> ()>>>> {
    let mut s = Rc::new(RefCell::new((*s.borrow()).clone()));
    if (*n.borrow().as_ref().unwrap()) < 1 {
        std::panic::panic_any(format!("cannot be less than 1"));
    }
    let n_closure_clone = n.clone(); let s_closure_clone = s.clone(); return Rc::new(RefCell::new(Some(Rc::new(move |r#yield: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Vec<i64>>>>) -> bool>>>>| {
        let mut i = Rc::new(RefCell::new(Some(0i64)));
    while ((*i.borrow().as_ref().unwrap()) as i64) < (({ let __len = (*s_closure_clone.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) {
        let mut end = Rc::new(RefCell::new(Some(std::cmp::min(((*n_closure_clone.borrow().as_ref().unwrap()) as i64), (({ let __len = (*Rc::new(RefCell::new(Some({ let __seq_holder = s_closure_clone.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = ((*i.borrow().as_ref().unwrap())) as usize; let __high = __seq.len(); let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) }))).borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64)))));
        if !{ let __f: Rc<dyn Fn(Rc<RefCell<Option<Vec<i64>>>>) -> bool> = r#yield.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let mut __seq = { let __seq_holder = s_closure_clone.clone(); let __seq_guard = __seq_holder.borrow(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; let __low = ((*i.borrow().as_ref().unwrap())) as usize; let __high = ((*i.borrow().as_ref().unwrap()) + (*end.borrow().as_ref().unwrap())) as usize; let __max = ((*i.borrow().as_ref().unwrap()) + (*end.borrow().as_ref().unwrap())) as usize; __seq.reslice3_zeroed(__low, __high, __max) })))) } {
        return;
    }
        { let __rhs = (*n_closure_clone.borrow().as_ref().unwrap()); let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    }) as Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Vec<i64>>>>) -> bool>>>>) -> ()>)));
}

pub fn __slices__backward__slice_string__string(s: Rc<RefCell<Option<Vec<String>>>>) -> Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<String>>>) -> bool>>>>) -> ()>>>> {
    let mut s = Rc::new(RefCell::new((*s.borrow()).clone()));
    let s_closure_clone = s.clone(); return Rc::new(RefCell::new(Some(Rc::new(move |r#yield: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<String>>>) -> bool>>>>| {
        let mut i = Rc::new(RefCell::new(Some((({ let __len = (*s_closure_clone.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) - (1 as i64))));
    while (*i.borrow().as_ref().unwrap()) >= 0 {
        if !{ let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<String>>>) -> bool> = r#yield.borrow().as_ref().unwrap().clone(); (__f)(i.clone(), Rc::new(RefCell::new(Some({ let __arg_value = (*s_closure_clone.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) } {
        return;
    }
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    }) as Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<String>>>) -> bool>>>>) -> ()>)));
}

pub fn __slices__compare__slice_int__int(s1: Rc<RefCell<Option<Vec<i64>>>>, s2: Rc<RefCell<Option<Vec<i64>>>>) -> i64 {
    { let __range_holder = s1.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for (i, v1) in __range_values.iter().copied().enumerate() {
        if (i as i64) >= (({ let __len = (*s2.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) {
        return 1;
    }
        let mut v2 = Rc::new(RefCell::new(Some((*s2.borrow().as_ref().unwrap())[(i) as usize].clone())));
        {
        let mut c = __cmp__compare__int(Rc::new(RefCell::new(Some(v1))), Rc::new(RefCell::new(Some({ let __arg_holder = v2.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if c != 0 {
            return c;;
        }
    }
    } }
    if (({ let __len = (*s1.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) < (({ let __len = (*s2.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) {
        return -(1);
    }
    0
}

pub fn __slices__compare__slice_string__string(s1: Rc<RefCell<Option<Vec<String>>>>, s2: Rc<RefCell<Option<Vec<String>>>>) -> i64 {
    { let __range_holder = s1.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for (i, v1) in __range_values.iter().enumerate() {
        if (i as i64) >= (({ let __len = (*s2.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) {
        return 1;
    }
        let mut v2 = Rc::new(RefCell::new(Some((*s2.borrow().as_ref().unwrap())[(i) as usize].clone())));
        {
        let mut c = __cmp__compare__string(Rc::new(RefCell::new(Some((*v1).clone()))), Rc::new(RefCell::new(Some({ let __arg_holder = v2.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if c != 0 {
            return c;;
        }
    }
    } }
    if (({ let __len = (*s1.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) < (({ let __len = (*s2.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) {
        return -(1);
    }
    0
}

pub fn __slices__equal_func__slice_int__slice_string__int__string(s1: Rc<RefCell<Option<Vec<i64>>>>, s2: Rc<RefCell<Option<Vec<String>>>>, eq: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<String>>>) -> bool>>>>) -> bool {
    if (({ let __len = (*s1.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) != (({ let __len = (*s2.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) {
        return false;
    }
    { let __range_holder = s1.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for (i, v1) in __range_values.iter().copied().enumerate() {
        let mut v2 = Rc::new(RefCell::new(Some((*s2.borrow().as_ref().unwrap())[(i) as usize].clone())));
        if !{ let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<String>>>) -> bool> = eq.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(v1))), Rc::new(RefCell::new(Some({ let __arg_holder = v2.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })))) } {
        return false;
    }
    } }
    true
}

pub fn __slices__sort_stable_func__slice__item___item(x: Rc<RefCell<Option<Vec<Item>>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Item>>>, Rc<RefCell<Option<Item>>>) -> i64>>>>) {
    __slices_stable_cmp_func___item(x.clone(), Rc::new(RefCell::new(Some({ let __arg_value = ({ let __len = (*x.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64; __arg_value }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
}

pub fn __cmp__compare__int(x: Rc<RefCell<Option<i64>>>, y: Rc<RefCell<Option<i64>>>) -> i64 {
    let mut xNaN = __cmp_is_na_n__int(Rc::new(RefCell::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let mut yNaN = __cmp_is_na_n__int(Rc::new(RefCell::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    if xNaN {
        if yNaN {
        return 0;
    }
        return -(1);
    }
    if yNaN {
        return 1;
    }
    if (*x.borrow().as_ref().unwrap()) < (*y.borrow().as_ref().unwrap()) {
        return -(1);
    }
    if (*x.borrow().as_ref().unwrap()) > (*y.borrow().as_ref().unwrap()) {
        return 1;
    }
    0
}

pub fn __cmp__compare__string(x: Rc<RefCell<Option<String>>>, y: Rc<RefCell<Option<String>>>) -> i64 {
    let mut xNaN = __cmp_is_na_n__string(Rc::new(RefCell::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let mut yNaN = __cmp_is_na_n__string(Rc::new(RefCell::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    if xNaN {
        if yNaN {
        return 0;
    }
        return -(1);
    }
    if yNaN {
        return 1;
    }
    if (*x.borrow().as_ref().unwrap()) < (*y.borrow().as_ref().unwrap()) {
        return -(1);
    }
    if (*x.borrow().as_ref().unwrap()) > (*y.borrow().as_ref().unwrap()) {
        return 1;
    }
    0
}

pub fn __cmp__compare__float64(x: Rc<RefCell<Option<f64>>>, y: Rc<RefCell<Option<f64>>>) -> i64 {
    let mut xNaN = __cmp_is_na_n__float64(Rc::new(RefCell::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let mut yNaN = __cmp_is_na_n__float64(Rc::new(RefCell::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    if xNaN {
        if yNaN {
        return 0;
    }
        return -(1);
    }
    if yNaN {
        return 1;
    }
    if (*x.borrow().as_ref().unwrap()) < (*y.borrow().as_ref().unwrap()) {
        return -(1);
    }
    if (*x.borrow().as_ref().unwrap()) > (*y.borrow().as_ref().unwrap()) {
        return 1;
    }
    0
}

pub fn __slices__is_sorted_func__slice__item___item(x: Rc<RefCell<Option<Vec<Item>>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Item>>>, Rc<RefCell<Option<Item>>>) -> i64>>>>) -> bool {
    let mut i = Rc::new(RefCell::new(Some((({ let __len = (*x.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) - (1 as i64))));
    while (*i.borrow().as_ref().unwrap()) > 0 {
        if { let __f: Rc<dyn Fn(Rc<RefCell<Option<Item>>>, Rc<RefCell<Option<Item>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*x.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*x.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value })))) } < 0 {
        return false;
    }
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    true
}

pub fn __slices__binary_search_func__slice__item___item__int(x: Rc<RefCell<Option<Vec<Item>>>>, target: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Item>>>, Rc<RefCell<Option<i64>>>) -> i64>>>>) -> (i64, bool) {
    let mut n = Rc::new(RefCell::new(Some(({ let __len = (*x.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64)));
    let (mut i, mut j) = (Rc::new(RefCell::new(Some(0i64))), { let __owned = n.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) });
    while (*i.borrow().as_ref().unwrap()) < (*j.borrow().as_ref().unwrap()) {
        let mut h = Rc::new(RefCell::new(Some((((*Rc::new(RefCell::new(Some((((*i.borrow().as_ref().unwrap()) + (*j.borrow().as_ref().unwrap()))) as u64))).borrow().as_ref().unwrap()) >> 1)) as i64)));
        if { let __f: Rc<dyn Fn(Rc<RefCell<Option<Item>>>, Rc<RefCell<Option<i64>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*x.borrow().as_ref().unwrap())[((*h.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), target.clone()) } < 0 {
        { let new_val = (*h.borrow().as_ref().unwrap()) + 1; *i.borrow_mut() = Some(new_val); };
    } else {
        { let new_val = h.borrow().as_ref().unwrap().clone(); *j.borrow_mut() = Some(new_val); };
    }
    }
    return ((*i.borrow().as_ref().unwrap()), (*i.borrow().as_ref().unwrap()) < (*n.borrow().as_ref().unwrap()) && { let __f: Rc<dyn Fn(Rc<RefCell<Option<Item>>>, Rc<RefCell<Option<i64>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*x.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), target.clone()) } == 0);
}

pub fn __slices__min_func__slice__item___item(x: Rc<RefCell<Option<Vec<Item>>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Item>>>, Rc<RefCell<Option<Item>>>) -> i64>>>>) -> Rc<RefCell<Option<Item>>> {
    if (({ let __len = (*x.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) < (1 as i64) {
        std::panic::panic_any(format!("slices.MinFunc: empty list"));
    }
    let mut m = Rc::new(RefCell::new(Some((*x.borrow().as_ref().unwrap())[(0) as usize].clone().__go_value_clone())));
    let mut i = Rc::new(RefCell::new(Some(1i64)));
    while ((*i.borrow().as_ref().unwrap()) as i64) < (({ let __len = (*x.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) {
        if { let __f: Rc<dyn Fn(Rc<RefCell<Option<Item>>>, Rc<RefCell<Option<Item>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*x.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), m.clone()) } < 0 {
        { let new_val = (*x.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone().__go_value_clone(); *m.borrow_mut() = Some(new_val); };
    }
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return { let __owned = m.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) };
}

pub fn __slices__max_func__slice__item___item(x: Rc<RefCell<Option<Vec<Item>>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Item>>>, Rc<RefCell<Option<Item>>>) -> i64>>>>) -> Rc<RefCell<Option<Item>>> {
    if (({ let __len = (*x.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) < (1 as i64) {
        std::panic::panic_any(format!("slices.MaxFunc: empty list"));
    }
    let mut m = Rc::new(RefCell::new(Some((*x.borrow().as_ref().unwrap())[(0) as usize].clone().__go_value_clone())));
    let mut i = Rc::new(RefCell::new(Some(1i64)));
    while ((*i.borrow().as_ref().unwrap()) as i64) < (({ let __len = (*x.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) {
        if { let __f: Rc<dyn Fn(Rc<RefCell<Option<Item>>>, Rc<RefCell<Option<Item>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*x.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), m.clone()) } > 0 {
        { let new_val = (*x.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone().__go_value_clone(); *m.borrow_mut() = Some(new_val); };
    }
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return { let __owned = m.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) };
}

pub fn __slices__delete_func__slice__item___item(s: Rc<RefCell<Option<Vec<Item>>>>, del: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Item>>>) -> bool>>>>) -> Rc<RefCell<Option<Vec<Item>>>> {
    let mut i = __slices__index_func__slice__item___item(s.clone(), Rc::new(RefCell::new({ let __func_value = (*del.borrow()).clone(); __func_value })));
    if i == -1 {
        return s.clone();
    }
    let mut j = Rc::new(RefCell::new(Some(i + 1)));
    while ((*j.borrow().as_ref().unwrap()) as i64) < (({ let __len = (*s.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) {
        {
        let mut v = Rc::new(RefCell::new(Some((*s.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone().__go_value_clone())));;
        if !{ let __f: Rc<dyn Fn(Rc<RefCell<Option<Item>>>) -> bool> = del.borrow().as_ref().unwrap().clone(); (__f)(v.clone()) } {
            { let __idx = (i) as usize; let __rhs = (*v.borrow().as_ref().unwrap()).__go_value_clone(); (*s.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };;
            { i += 1; };
        }
    }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    { let __clear_start = (i) as usize; let __clear_end = { let __clear_len_holder = s.clone(); let __clear_len_guard = __clear_len_holder.borrow(); __clear_len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; let __clear_holder = s.clone(); let mut __clear_guard = __clear_holder.borrow_mut(); if let Some(__clear_seq) = __clear_guard.as_mut() { let __clear_cap = __clear_seq.capacity(); let mut __clear_window = __clear_seq.reslice3_zeroed(__clear_start, __clear_end, __clear_cap); for __clear_elem in __clear_window.as_mut_slice().iter_mut() { *__clear_elem = Default::default(); } } };
    Rc::new(RefCell::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = 0; let __high = (i) as usize; let __max = __source_cap; __seq.reslice3(__low, __high, __max) })))
}

pub fn __slices__index__slice__item___item(s: Rc<RefCell<Option<Vec<Item>>>>, v: Rc<RefCell<Option<Item>>>) -> i64 {
    for i in 0..(({ let __range_holder = s.clone(); let __range_guard = __range_holder.borrow(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        if (*v.borrow().as_ref().unwrap()) == (*s.borrow().as_ref().unwrap())[(i) as usize].clone() {
        return i as i64;
    }
    }
    -(1)
}

pub fn __slices__index__slice_ptr__task__ptr__task(s: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Task>>>>>>>, v: Rc<RefCell<Option<Task>>>) -> i64 {
    for i in 0..(({ let __range_holder = s.clone(); let __range_guard = __range_holder.borrow(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        if { let __left = v.clone(); let __right = (*s.borrow().as_ref().unwrap())[(i) as usize].clone(); let __both_nil = (*__left.borrow()).is_none() && (*__right.borrow()).is_none(); let __eq = __both_nil || Rc::ptr_eq(&__left, &__right) || { let __left_id = __left.borrow().as_ref().map(|__v| Rc::as_ptr(&__v.name) as *const () as usize); let __right_id = __right.borrow().as_ref().map(|__v| Rc::as_ptr(&__v.name) as *const () as usize); __left_id.is_some() && __left_id == __right_id }; __eq } {
        return i as i64;
    }
    }
    -(1)
}

pub fn __slices__contains__slice__item___item(s: Rc<RefCell<Option<Vec<Item>>>>, v: Rc<RefCell<Option<Item>>>) -> bool {
    __slices__index__slice__item___item(s.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() })))) >= 0
}

pub fn __slices__sort_func__slice_ptr__task__ptr__task(x: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Task>>>>>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Task>>>, Rc<RefCell<Option<Task>>>) -> i64>>>>) {
    let mut n = Rc::new(RefCell::new(Some(({ let __len = (*x.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64)));
    __slices_pdqsort_cmp_func__ptr__task(x.clone(), Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_value = math_bits::len(Rc::new(RefCell::new(Some({ let __arg_value = (*n.borrow().as_ref().unwrap()) as u64; __arg_value })))); __arg_value }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
}

pub fn __slices__sort_func__slice_int__int(x: Rc<RefCell<Option<Vec<i64>>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>>>>) {
    let mut n = Rc::new(RefCell::new(Some(({ let __len = (*x.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64)));
    __slices_pdqsort_cmp_func__int(x.clone(), Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_value = math_bits::len(Rc::new(RefCell::new(Some({ let __arg_value = (*n.borrow().as_ref().unwrap()) as u64; __arg_value })))); __arg_value }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
}

pub fn __slices__index_func__slice_ptr__task__ptr__task(s: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Task>>>>>>>, f: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Task>>>) -> bool>>>>) -> i64 {
    for i in 0..(({ let __range_holder = s.clone(); let __range_guard = __range_holder.borrow(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        if { let __f: Rc<dyn Fn(Rc<RefCell<Option<Task>>>) -> bool> = f.borrow().as_ref().unwrap().clone(); (__f)((*s.borrow().as_ref().unwrap())[(i) as usize].clone()) } {
        return i as i64;
    }
    }
    -(1)
}

pub fn __slices__index_func__slice__item___item(s: Rc<RefCell<Option<Vec<Item>>>>, f: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Item>>>) -> bool>>>>) -> i64 {
    for i in 0..(({ let __range_holder = s.clone(); let __range_guard = __range_holder.borrow(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        if { let __f: Rc<dyn Fn(Rc<RefCell<Option<Item>>>) -> bool> = f.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*s.borrow().as_ref().unwrap())[(i) as usize].clone(); __arg_value })))) } {
        return i as i64;
    }
    }
    -(1)
}

pub fn __slices__contains_func__slice_ptr__task__ptr__task(s: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Task>>>>>>>, f: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Task>>>) -> bool>>>>) -> bool {
    __slices__index_func__slice_ptr__task__ptr__task(s.clone(), Rc::new(RefCell::new({ let __func_value = (*f.borrow()).clone(); __func_value }))) >= 0
}

pub fn __slices__sort__slice__celsius___celsius(x: Rc<RefCell<Option<Vec<Celsius>>>>) {
    let mut n = Rc::new(RefCell::new(Some(({ let __len = (*x.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64)));
    __slices_pdqsort_ordered___celsius(x.clone(), Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_value = math_bits::len(Rc::new(RefCell::new(Some({ let __arg_value = (*n.borrow().as_ref().unwrap()) as u64; __arg_value })))); __arg_value }))));
}

pub fn __slices__sort___i_ds__int(x: Rc<RefCell<Option<IDs>>>) {
    let mut n = Rc::new(RefCell::new(Some({ let __slice_holder = { let __named_slice = (*x.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64)));
    __slices_pdqsort_ordered__int({ let __named_slice = (*x.borrow().as_ref().unwrap()).0.clone(); __named_slice }, Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_value = math_bits::len(Rc::new(RefCell::new(Some({ let __arg_value = (*n.borrow().as_ref().unwrap()) as u64; __arg_value })))); __arg_value }))));
}

pub fn __slices__sort__slice_float64__float64(x: Rc<RefCell<Option<Vec<f64>>>>) {
    let mut n = Rc::new(RefCell::new(Some(({ let __len = (*x.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64)));
    __slices_pdqsort_ordered__float64(x.clone(), Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_value = math_bits::len(Rc::new(RefCell::new(Some({ let __arg_value = (*n.borrow().as_ref().unwrap()) as u64; __arg_value })))); __arg_value }))));
}

pub fn __slices__sort__slice_int__int(x: Rc<RefCell<Option<Vec<i64>>>>) {
    let mut n = Rc::new(RefCell::new(Some(({ let __len = (*x.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64)));
    __slices_pdqsort_ordered__int(x.clone(), Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_value = math_bits::len(Rc::new(RefCell::new(Some({ let __arg_value = (*n.borrow().as_ref().unwrap()) as u64; __arg_value })))); __arg_value }))));
}

pub fn __slices__sort__slice_string__string(x: Rc<RefCell<Option<Vec<String>>>>) {
    let mut n = Rc::new(RefCell::new(Some(({ let __len = (*x.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64)));
    __slices_pdqsort_ordered__string(x.clone(), Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_value = math_bits::len(Rc::new(RefCell::new(Some({ let __arg_value = (*n.borrow().as_ref().unwrap()) as u64; __arg_value })))); __arg_value }))));
}

pub fn __slices__max__slice__celsius___celsius(x: Rc<RefCell<Option<Vec<Celsius>>>>) -> Rc<RefCell<Option<Celsius>>> {
    if (({ let __len = (*x.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) < (1 as i64) {
        std::panic::panic_any(format!("slices.Max: empty list"));
    }
    let mut m = Rc::new(RefCell::new(Some((*x.borrow().as_ref().unwrap())[(0) as usize].clone())));
    let mut i = Rc::new(RefCell::new(Some(1i64)));
    while ((*i.borrow().as_ref().unwrap()) as i64) < (({ let __len = (*x.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) {
        { let new_val = Celsius(Rc::new(RefCell::new(Some({ let __l = (*((*m.borrow().as_ref().unwrap())).0.borrow().as_ref().unwrap()).clone(); let __r = (*((*x.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone()).0.borrow().as_ref().unwrap()).clone(); if __l.is_nan() || __r.is_nan() { __l + __r } else if __r > __l || (__r == __l && __r.is_sign_positive()) { __r } else { __l } })))); *m.borrow_mut() = Some(new_val); };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return { let __owned = m.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
}

pub fn __slices__min__slice__celsius___celsius(x: Rc<RefCell<Option<Vec<Celsius>>>>) -> Rc<RefCell<Option<Celsius>>> {
    if (({ let __len = (*x.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) < (1 as i64) {
        std::panic::panic_any(format!("slices.Min: empty list"));
    }
    let mut m = Rc::new(RefCell::new(Some((*x.borrow().as_ref().unwrap())[(0) as usize].clone())));
    let mut i = Rc::new(RefCell::new(Some(1i64)));
    while ((*i.borrow().as_ref().unwrap()) as i64) < (({ let __len = (*x.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) {
        { let new_val = Celsius(Rc::new(RefCell::new(Some({ let __l = (*((*m.borrow().as_ref().unwrap())).0.borrow().as_ref().unwrap()).clone(); let __r = (*((*x.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone()).0.borrow().as_ref().unwrap()).clone(); if __l.is_nan() || __r.is_nan() { __l + __r } else if __r < __l || (__r == __l && __r.is_sign_negative()) { __r } else { __l } })))); *m.borrow_mut() = Some(new_val); };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return { let __owned = m.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
}

pub fn __cmp__less___celsius(x: Rc<RefCell<Option<Celsius>>>, y: Rc<RefCell<Option<Celsius>>>) -> bool {
    (__cmp_is_na_n___celsius(Rc::new(RefCell::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })))) && !__cmp_is_na_n___celsius(Rc::new(RefCell::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))))) || (*x.borrow().as_ref().unwrap()) < (*y.borrow().as_ref().unwrap())
}

pub fn __cmp__less__int(x: Rc<RefCell<Option<i64>>>, y: Rc<RefCell<Option<i64>>>) -> bool {
    (__cmp_is_na_n__int(Rc::new(RefCell::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })))) && !__cmp_is_na_n__int(Rc::new(RefCell::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))))) || (*x.borrow().as_ref().unwrap()) < (*y.borrow().as_ref().unwrap())
}

pub fn __cmp__less__float64(x: Rc<RefCell<Option<f64>>>, y: Rc<RefCell<Option<f64>>>) -> bool {
    (__cmp_is_na_n__float64(Rc::new(RefCell::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })))) && !__cmp_is_na_n__float64(Rc::new(RefCell::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))))) || (*x.borrow().as_ref().unwrap()) < (*y.borrow().as_ref().unwrap())
}

pub fn __cmp__less__string(x: Rc<RefCell<Option<String>>>, y: Rc<RefCell<Option<String>>>) -> bool {
    (__cmp_is_na_n__string(Rc::new(RefCell::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })))) && !__cmp_is_na_n__string(Rc::new(RefCell::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))))) || (*x.borrow().as_ref().unwrap()) < (*y.borrow().as_ref().unwrap())
}

pub fn __slices__binary_search___i_ds__int(x: Rc<RefCell<Option<IDs>>>, target: Rc<RefCell<Option<i64>>>) -> (i64, bool) {
    let mut n = Rc::new(RefCell::new(Some({ let __slice_holder = { let __named_slice = (*x.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64)));
    let (mut i, mut j) = (Rc::new(RefCell::new(Some(0i64))), { let __owned = n.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) });
    while (*i.borrow().as_ref().unwrap()) < (*j.borrow().as_ref().unwrap()) {
        let mut h = Rc::new(RefCell::new(Some((((*Rc::new(RefCell::new(Some((((*i.borrow().as_ref().unwrap()) + (*j.borrow().as_ref().unwrap()))) as u64))).borrow().as_ref().unwrap()) >> 1)) as i64)));
        if __cmp__less__int(Rc::new(RefCell::new(Some({ let __seq_holder = { let __named_slice = (*x.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_index = ((*h.borrow().as_ref().unwrap())) as usize; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = target.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        { let new_val = (*h.borrow().as_ref().unwrap()) + 1; *i.borrow_mut() = Some(new_val); };
    } else {
        { let new_val = h.borrow().as_ref().unwrap().clone(); *j.borrow_mut() = Some(new_val); };
    }
    }
    return ((*i.borrow().as_ref().unwrap()), (*i.borrow().as_ref().unwrap()) < (*n.borrow().as_ref().unwrap()) && ({ let __seq_holder = { let __named_slice = (*x.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_index = ((*i.borrow().as_ref().unwrap())) as usize; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() } == (*target.borrow().as_ref().unwrap()) || (__slices_is_na_n__int(Rc::new(RefCell::new(Some({ let __seq_holder = { let __named_slice = (*x.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_index = ((*i.borrow().as_ref().unwrap())) as usize; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() })))) && __slices_is_na_n__int(Rc::new(RefCell::new(Some({ let __arg_holder = target.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })))))));
}

pub fn __slices__clone___i_ds__int(s: Rc<RefCell<Option<IDs>>>) -> Rc<RefCell<Option<IDs>>> {
    if { let __slice_holder = { let __named_slice = s.borrow().as_ref().map(|__v| __v.0.clone()).unwrap_or_else(|| Rc::new(RefCell::new(None))); __named_slice }; let __slice_guard = __slice_holder.borrow(); (*__slice_guard).is_none() } {
        return Rc::new(RefCell::new(None));
    }
    { let __base = { let __named_slice = IDs(Rc::new(RefCell::new(Some(vec![])))); __named_slice.0.clone() }; let __base_guard = __base.borrow(); let mut __values = __base_guard.as_ref().cloned().unwrap_or_else(Vec::new); std::mem::drop(__base_guard); let __src = { let __named_slice = (*s.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __src_guard = __src.borrow(); if let Some(__src_values) = __src_guard.as_ref() { __values.extend(__src_values.iter().cloned()); }; Rc::new(RefCell::new(Some(IDs(Rc::new(RefCell::new(Some(__values))))))) }
}

pub fn __cmp__or__int(vals: Rc<RefCell<Option<Vec<i64>>>>) -> i64 {
    let mut zero: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));
    { let __range_holder = vals.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for val in __range_values.iter().copied() {
        if val != (*zero.borrow().as_ref().unwrap()) {
        return val;
    }
    } }
    return (*zero.borrow().as_ref().unwrap());
}

pub fn __cmp__or__string(vals: Rc<RefCell<Option<Vec<String>>>>) -> Rc<RefCell<Option<String>>> {
    let mut zero: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(Some(String::new())));
    { let __range_holder = vals.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for val in __range_values.iter() {
        if (*val).clone() != (*zero.borrow().as_ref().unwrap()) {
        return Rc::new(RefCell::new(Some((*val).clone())));
    }
    } }
    return { let __owned = zero.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
}

pub fn __slices__is_sorted__slice_int__int(x: Rc<RefCell<Option<Vec<i64>>>>) -> bool {
    let mut i = Rc::new(RefCell::new(Some((({ let __len = (*x.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) - (1 as i64))));
    while (*i.borrow().as_ref().unwrap()) > 0 {
        if __cmp__less__int(Rc::new(RefCell::new(Some({ let __arg_value = (*x.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*x.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value })))) {
        return false;
    }
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    true
}

pub fn __slices__reverse__slice_int__int(s: Rc<RefCell<Option<Vec<i64>>>>) {
    let (mut i, mut j) = (Rc::new(RefCell::new(Some(0i64))), Rc::new(RefCell::new(Some((({ let __len = (*s.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) - (1 as i64)))));
    while (*i.borrow().as_ref().unwrap()) < (*j.borrow().as_ref().unwrap()) {
        {
            let __go_lhs_sequence_0 = s.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = s.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*s.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*s.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        {
            let __tmp_0 = (*i.borrow().as_ref().unwrap()) + 1;
            let __tmp_1 = (*j.borrow().as_ref().unwrap()) - 1;
            *i.borrow_mut() = Some(__tmp_0);
            *j.borrow_mut() = Some(__tmp_1);
        };
    }
}

pub fn __slices__sorted__string(seq: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<String>>>) -> bool>>>>) -> ()>>>>) -> Rc<RefCell<Option<Vec<String>>>> {
    let mut s = __slices__collect__string(Rc::new(RefCell::new({ let __func_value = (*seq.borrow()).clone(); __func_value })));
    let mut s = { let __slice_header = GoSliceHeader::go_slice_header(&s); Rc::new(RefCell::new(__slice_header)) };
    __slices__sort__slice_string__string(s.clone());
    return s.clone();
}

pub fn __slices__sorted__int(seq: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> bool>>>>) -> ()>>>>) -> Rc<RefCell<Option<Vec<i64>>>> {
    let mut s = __slices__collect__int(Rc::new(RefCell::new({ let __func_value = (*seq.borrow()).clone(); __func_value })));
    let mut s = { let __slice_header = GoSliceHeader::go_slice_header(&s); Rc::new(RefCell::new(__slice_header)) };
    __slices__sort__slice_int__int(s.clone());
    return s.clone();
}

pub fn __maps__keys__map_string_int__string__int(m: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>) -> Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<String>>>) -> bool>>>>) -> ()>>>> {
    let m_closure_clone = m.clone(); return Rc::new(RefCell::new(Some(Rc::new(move |r#yield: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<String>>>) -> bool>>>>| {
        for (k, _) in { let __range_holder = m_closure_clone.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        if !{ let __f: Rc<dyn Fn(Rc<RefCell<Option<String>>>) -> bool> = r#yield.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(k.clone())))) } {
        return;
    }
    }
    }) as Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<String>>>) -> bool>>>>) -> ()>)));
}

pub fn __maps__values__map_string_int__string__int(m: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>) -> Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> bool>>>>) -> ()>>>> {
    let m_closure_clone = m.clone(); return Rc::new(RefCell::new(Some(Rc::new(move |r#yield: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> bool>>>>| {
        for (_, v) in { let __range_holder = m_closure_clone.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        if !{ let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> bool> = r#yield.borrow().as_ref().unwrap().clone(); (__f)(v.clone()) } {
        return;
    }
    }
    }) as Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> bool>>>>) -> ()>)));
}

pub fn __maps__collect__int__string(seq: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<String>>>) -> bool>>>>) -> ()>>>>) -> Rc<RefCell<Option<BTreeMap<i64, Rc<RefCell<Option<String>>>>>>> {
    let mut m = Rc::new(RefCell::new(Some(BTreeMap::<i64, Rc<RefCell<Option<String>>>>::new())));
    __maps__insert__map_int_string__int__string(m.clone(), Rc::new(RefCell::new({ let __func_value = (*seq.borrow()).clone(); __func_value })));
    return m.clone();
}

pub fn __slices__all__slice_string__string(s: Rc<RefCell<Option<Vec<String>>>>) -> Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<String>>>) -> bool>>>>) -> ()>>>> {
    let mut s = Rc::new(RefCell::new((*s.borrow()).clone()));
    let s_closure_clone = s.clone(); return Rc::new(RefCell::new(Some(Rc::new(move |r#yield: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<String>>>) -> bool>>>>| {
        { let __range_holder = s_closure_clone.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for (i, v) in __range_values.iter().enumerate() {
        if !{ let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<String>>>) -> bool> = r#yield.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(i as i64))), Rc::new(RefCell::new(Some((*v).clone())))) } {
        return;
    }
    } }
    }) as Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<String>>>) -> bool>>>>) -> ()>)));
}

pub fn __maps__copy__map_string_int__map_string_int__string__int(dst: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>, src: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>) {
    for (k, v) in { let __range_holder = src.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __map_key = k.clone(); let __map_value = Rc::new(RefCell::new(Some((*v.borrow().as_ref().unwrap()).clone()))); (*dst.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    }
}

pub fn __maps__delete_func__map_string_int__string__int(m: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>, del: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<i64>>>) -> bool>>>>) {
    for (k, v) in { let __range_holder = m.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        if { let __f: Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<i64>>>) -> bool> = del.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(k.clone()))), v.clone()) } {
        { let __map_handle = m.clone(); let __delete_key = &k; let mut __map_guard = __map_handle.borrow_mut(); if let Some(__map) = __map_guard.as_mut() { __map.remove(__delete_key); } };
    }
    }
}

pub fn __maps__equal_func__map_string_int__map_string_int__string__int__int(m1: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>, m2: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>, eq: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> bool>>>>) -> bool {
    if (({ let __len = (*m1.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) != (({ let __len = (*m2.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) {
        return false;
    }
    for (k, v1) in { let __range_holder = m1.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        {
        let (mut v2, mut ok) = { let __map_holder = m2.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&k)) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().clone()))), true), None => (Rc::new(RefCell::new(Some(0))), false) } };;
        if !ok || !{ let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> bool> = eq.borrow().as_ref().unwrap().clone(); (__f)(v1.clone(), v2.clone()) } {
            return false;;
        }
    }
    }
    true
}

pub fn __maps__insert__map_string_int__string__int(m: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>, seq: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<i64>>>) -> bool>>>>) -> ()>>>>) {
    let mut __rf_done_3: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));
    let m_closure_clone = m.clone(); { let __f: Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<i64>>>) -> bool>>>>) -> ()> = seq.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(Rc::new(move |__rf_param_3_0: Rc<RefCell<Option<String>>>, __rf_param_3_1: Rc<RefCell<Option<i64>>>| -> bool {
        let (mut k, mut v) = ({ let __owned = __rf_param_3_0.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) }, { let __owned = __rf_param_3_1.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) });
        { let __map_key = (*k.borrow().as_ref().unwrap()).clone(); let __map_value = Rc::new(RefCell::new(Some((*v.borrow().as_ref().unwrap()).clone()))); (*m_closure_clone.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
        true
    }) as Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<i64>>>) -> bool>)))) };
    if (*__rf_done_3.borrow().as_ref().unwrap()) {
        return;
    }
}

pub fn __maps__insert__map_int_string__int__string(m: Rc<RefCell<Option<BTreeMap<i64, Rc<RefCell<Option<String>>>>>>>, seq: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<String>>>) -> bool>>>>) -> ()>>>>) {
    let mut __rf_done_4: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));
    let m_closure_clone = m.clone(); { let __f: Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<String>>>) -> bool>>>>) -> ()> = seq.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(Rc::new(move |__rf_param_4_0: Rc<RefCell<Option<i64>>>, __rf_param_4_1: Rc<RefCell<Option<String>>>| -> bool {
        let (mut k, mut v) = ({ let __owned = __rf_param_4_0.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) }, { let __owned = __rf_param_4_1.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) });
        { let __map_key = (*k.borrow().as_ref().unwrap()); let __map_value = Rc::new(RefCell::new(Some((*v.borrow().as_ref().unwrap()).clone()))); (*m_closure_clone.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
        true
    }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<String>>>) -> bool>)))) };
    if (*__rf_done_4.borrow().as_ref().unwrap()) {
        return;
    }
}

pub fn __maps__all__map_string_int__string__int(m: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>) -> Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<i64>>>) -> bool>>>>) -> ()>>>> {
    let m_closure_clone = m.clone(); return Rc::new(RefCell::new(Some(Rc::new(move |r#yield: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<i64>>>) -> bool>>>>| {
        for (k, v) in { let __range_holder = m_closure_clone.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        if !{ let __f: Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<i64>>>) -> bool> = r#yield.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(k.clone()))), v.clone()) } {
        return;
    }
    }
    }) as Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<i64>>>) -> bool>>>>) -> ()>)));
}

pub fn __maps__clone__map_string_int__string__int(m: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>) -> Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>> {
    if { let __nil_result = (*m.borrow()).is_none(); __nil_result } {
        return Rc::new(RefCell::new(None));
    }
    let mut r = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i64>>>>::new())));
    for (k, v) in { let __range_holder = m.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __map_key = k.clone(); let __map_value = Rc::new(RefCell::new(Some((*v.borrow().as_ref().unwrap()).clone()))); (*r.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    }
    return r.clone();
}

pub fn __maps__equal__map_string_int__map_string_int__string__int(m1: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>, m2: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>) -> bool {
    if (({ let __len = (*m1.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) != (({ let __len = (*m2.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) {
        return false;
    }
    for (k, v1) in { let __range_holder = m1.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        {
        let (mut v2, mut ok) = { let __map_holder = m2.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&k)) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().clone()))), true), None => (Rc::new(RefCell::new(Some(0))), false) } };;
        if !ok || (*v1.borrow_mut().as_mut().unwrap()) != (*v2.borrow().as_ref().unwrap()) {
            return false;;
        }
    }
    }
    true
}

pub fn __slices__collect__string(seq: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<String>>>) -> bool>>>>) -> ()>>>>) -> Rc<RefCell<Option<Vec<String>>>> {
    __slices__append_seq__slice_string__string(Rc::new(RefCell::new(None::<Vec<String>>)), Rc::new(RefCell::new({ let __func_value = (*seq.borrow()).clone(); __func_value })))
}

pub fn __slices__collect__int(seq: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> bool>>>>) -> ()>>>>) -> Rc<RefCell<Option<Vec<i64>>>> {
    __slices__append_seq__slice_int__int(Rc::new(RefCell::new(None::<Vec<i64>>)), Rc::new(RefCell::new({ let __func_value = (*seq.borrow()).clone(); __func_value })))
}

pub fn __slices_overlaps__int(a: Rc<RefCell<Option<Vec<i64>>>>, b: Rc<RefCell<Option<Vec<i64>>>>) -> bool {
    if (({ let __len = (*a.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) == (0 as i64) || (({ let __len = (*b.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) == (0 as i64) {
        return false;
    }
    for i in 0..(({ let __range_holder = a.clone(); let __range_guard = __range_holder.borrow(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        if { let __left = GoSliceElemPtr::new(a.clone(), (i) as usize); let __right = GoSliceElemPtr::new(b.clone(), (0) as usize); let __eq = __left.identity() == __right.identity(); __eq } {
        return true;
    }
    }
    for i in 0..(({ let __range_holder = b.clone(); let __range_guard = __range_holder.borrow(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        if { let __left = GoSliceElemPtr::new(b.clone(), (i) as usize); let __right = GoSliceElemPtr::new(a.clone(), (0) as usize); let __eq = __left.identity() == __right.identity(); __eq } {
        return true;
    }
    }
    false
}

pub fn __slices_rotate_right__int(s: Rc<RefCell<Option<Vec<i64>>>>, r: Rc<RefCell<Option<i64>>>) {
    __slices_rotate_left__int(s.clone(), Rc::new(RefCell::new(Some({ let __arg_value = (({ let __len = (*s.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) - ((*r.borrow().as_ref().unwrap()) as i64); __arg_value }))));
}

pub fn __slices_start_idx__int(haystack: Rc<RefCell<Option<Vec<i64>>>>, needle: Rc<RefCell<Option<Vec<i64>>>>) -> i64 {
    let mut p: Option<GoSliceElemPtr<i64>> = Some(GoSliceElemPtr::new(needle.clone(), (0) as usize));
    for i in 0..(({ let __range_holder = haystack.clone(); let __range_guard = __range_holder.borrow(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        if { let __left = p.clone(); let __right = Some(GoSliceElemPtr::new(haystack.clone(), (i) as usize)); let __eq = match (&__left, &__right) { (None, None) => true, (Some(__l), Some(__r)) => __l.identity() == __r.identity(), _ => false }; __eq } {
        return i as i64;
    }
    }
    std::panic::panic_any(format!("needle not found"));
}

pub fn __slices_stable_cmp_func___item(data: Rc<RefCell<Option<Vec<Item>>>>, n: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Item>>>, Rc<RefCell<Option<Item>>>) -> i64>>>>) {
    let mut blockSize = Rc::new(RefCell::new(Some(20i64)));
    let (mut a, mut b) = (Rc::new(RefCell::new(Some(0i64))), { let __owned = blockSize.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) });
    while (*b.borrow().as_ref().unwrap()) <= (*n.borrow().as_ref().unwrap()) {
        __slices_insertion_sort_cmp_func___item(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        { let new_val = b.borrow().as_ref().unwrap().clone(); *a.borrow_mut() = Some(new_val); };
        { let __rhs = (*blockSize.borrow().as_ref().unwrap()); let mut guard = b.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    __slices_insertion_sort_cmp_func___item(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
    while (*blockSize.borrow().as_ref().unwrap()) < (*n.borrow().as_ref().unwrap()) {
        {
            let __tmp_0 = 0;
            let __tmp_1 = 2 * (*blockSize.borrow().as_ref().unwrap());
            *a.borrow_mut() = Some(__tmp_0);
            *b.borrow_mut() = Some(__tmp_1);
        };
        while (*b.borrow().as_ref().unwrap()) <= (*n.borrow().as_ref().unwrap()) {
        __slices_sym_merge_cmp_func___item(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_value = (*a.borrow().as_ref().unwrap()) + (*blockSize.borrow().as_ref().unwrap()); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        { let new_val = b.borrow().as_ref().unwrap().clone(); *a.borrow_mut() = Some(new_val); };
        { let __rhs = 2 * (*blockSize.borrow().as_ref().unwrap()); let mut guard = b.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        {
        let mut m = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + (*blockSize.borrow().as_ref().unwrap()))));;
        if (*m.borrow().as_ref().unwrap()) < (*n.borrow().as_ref().unwrap()) {
            __slices_sym_merge_cmp_func___item(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = m.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));;
        }
    }
        { let __rhs = 2; let mut guard = blockSize.borrow_mut(); *guard = Some(guard.as_ref().unwrap() * __rhs); };
    }
}

pub fn __cmp_is_na_n__int(x: Rc<RefCell<Option<i64>>>) -> bool {
    { let __bin_x = (*x.borrow().as_ref().unwrap()).clone(); __bin_x != __bin_x }
}

pub fn __cmp_is_na_n__string(x: Rc<RefCell<Option<String>>>) -> bool {
    { let __bin_x = (*x.borrow().as_ref().unwrap()).clone(); __bin_x != __bin_x }
}

pub fn __cmp_is_na_n___celsius(x: Rc<RefCell<Option<Celsius>>>) -> bool {
    { let __bin_x = (*x.borrow().as_ref().unwrap()).clone(); __bin_x.clone() != __bin_x }
}

pub fn __cmp_is_na_n__float64(x: Rc<RefCell<Option<f64>>>) -> bool {
    { let __bin_x = (*x.borrow().as_ref().unwrap()).clone(); __bin_x != __bin_x }
}

pub fn __slices_pdqsort_cmp_func__ptr__task(data: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Task>>>>>>>, mut a: Rc<RefCell<Option<i64>>>, mut b: Rc<RefCell<Option<i64>>>, mut limit: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Task>>>, Rc<RefCell<Option<Task>>>) -> i64>>>>) {
    const maxInsertion: i64 = 12;

    let mut wasBalanced = Rc::new(RefCell::new(Some(true)));let mut wasPartitioned = Rc::new(RefCell::new(Some(true)));
    loop {
        let mut length = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
        if (*length.borrow().as_ref().unwrap()) <= 12 {
        __slices_insertion_sort_cmp_func__ptr__task(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        return;
    }
        if (*limit.borrow().as_ref().unwrap()) == 0 {
        __slices_heap_sort_cmp_func__ptr__task(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        return;
    }
        if !(*wasBalanced.borrow().as_ref().unwrap()) {
        __slices_break_patterns_cmp_func__ptr__task(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        { let mut guard = limit.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        let (mut pivot, mut hint) = __slices_choose_pivot_cmp_func__ptr__task(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        if (*hint.borrow().as_ref().unwrap()) == __slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_DECREASING_HINT as i64)))) {
        __slices_reverse_range_cmp_func__ptr__task(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        { let new_val = ((*b.borrow().as_ref().unwrap()) - 1) - (pivot - (*a.borrow().as_ref().unwrap())); pivot = new_val; };
        { let new_val = __slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_INCREASING_HINT as i64)))); *hint.borrow_mut() = Some(new_val); };
    }
        if (*wasBalanced.borrow().as_ref().unwrap()) && (*wasPartitioned.borrow().as_ref().unwrap()) && (*hint.borrow().as_ref().unwrap()) == __slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_INCREASING_HINT as i64)))) {
        if __slices_partial_insertion_sort_cmp_func__ptr__task(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value }))) {
        return;
    }
    }
        if (*a.borrow().as_ref().unwrap()) > 0 && !({ let __f: Rc<dyn Fn(Rc<RefCell<Option<Task>>>, Rc<RefCell<Option<Task>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)((*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap()) - 1) as usize].clone(), (*data.borrow().as_ref().unwrap())[(pivot) as usize].clone()) } < 0) {
        let mut mid = __slices_partition_equal_cmp_func__ptr__task(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some(pivot))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        { let new_val = mid; *a.borrow_mut() = Some(new_val); };
        continue
    }
        let (mut mid, mut alreadyPartitioned) = __slices_partition_cmp_func__ptr__task(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some(pivot))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        { let new_val = alreadyPartitioned; *wasPartitioned.borrow_mut() = Some(new_val); };
        let (mut leftLen, mut rightLen) = (Rc::new(RefCell::new(Some(mid - (*a.borrow().as_ref().unwrap())))), Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - mid))));
        let mut balanceThreshold = Rc::new(RefCell::new(Some((*length.borrow().as_ref().unwrap()) / 8)));
        if (*leftLen.borrow().as_ref().unwrap()) < (*rightLen.borrow().as_ref().unwrap()) {
        { let new_val = (*leftLen.borrow().as_ref().unwrap()) >= (*balanceThreshold.borrow().as_ref().unwrap()); *wasBalanced.borrow_mut() = Some(new_val); };
        __slices_pdqsort_cmp_func__ptr__task(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some(mid))), Rc::new(RefCell::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        { let new_val = mid + 1; *a.borrow_mut() = Some(new_val); };
    } else {
        { let new_val = (*rightLen.borrow().as_ref().unwrap()) >= (*balanceThreshold.borrow().as_ref().unwrap()); *wasBalanced.borrow_mut() = Some(new_val); };
        __slices_pdqsort_cmp_func__ptr__task(data.clone(), Rc::new(RefCell::new(Some(mid + 1))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        { let new_val = mid; *b.borrow_mut() = Some(new_val); };
    }
    }
}

pub fn __slices_pdqsort_cmp_func__int(data: Rc<RefCell<Option<Vec<i64>>>>, mut a: Rc<RefCell<Option<i64>>>, mut b: Rc<RefCell<Option<i64>>>, mut limit: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>>>>) {
    const maxInsertion: i64 = 12;

    let mut wasBalanced = Rc::new(RefCell::new(Some(true)));let mut wasPartitioned = Rc::new(RefCell::new(Some(true)));
    loop {
        let mut length = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
        if (*length.borrow().as_ref().unwrap()) <= 12 {
        __slices_insertion_sort_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        return;
    }
        if (*limit.borrow().as_ref().unwrap()) == 0 {
        __slices_heap_sort_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        return;
    }
        if !(*wasBalanced.borrow().as_ref().unwrap()) {
        __slices_break_patterns_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        { let mut guard = limit.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        let (mut pivot, mut hint) = __slices_choose_pivot_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        if (*hint.borrow().as_ref().unwrap()) == __slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_DECREASING_HINT as i64)))) {
        __slices_reverse_range_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        { let new_val = ((*b.borrow().as_ref().unwrap()) - 1) - (pivot - (*a.borrow().as_ref().unwrap())); pivot = new_val; };
        { let new_val = __slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_INCREASING_HINT as i64)))); *hint.borrow_mut() = Some(new_val); };
    }
        if (*wasBalanced.borrow().as_ref().unwrap()) && (*wasPartitioned.borrow().as_ref().unwrap()) && (*hint.borrow().as_ref().unwrap()) == __slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_INCREASING_HINT as i64)))) {
        if __slices_partial_insertion_sort_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value }))) {
        return;
    }
    }
        if (*a.borrow().as_ref().unwrap()) > 0 && !({ let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[(pivot) as usize].clone(); __arg_value })))) } < 0) {
        let mut mid = __slices_partition_equal_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some(pivot))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        { let new_val = mid; *a.borrow_mut() = Some(new_val); };
        continue
    }
        let (mut mid, mut alreadyPartitioned) = __slices_partition_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some(pivot))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        { let new_val = alreadyPartitioned; *wasPartitioned.borrow_mut() = Some(new_val); };
        let (mut leftLen, mut rightLen) = (Rc::new(RefCell::new(Some(mid - (*a.borrow().as_ref().unwrap())))), Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - mid))));
        let mut balanceThreshold = Rc::new(RefCell::new(Some((*length.borrow().as_ref().unwrap()) / 8)));
        if (*leftLen.borrow().as_ref().unwrap()) < (*rightLen.borrow().as_ref().unwrap()) {
        { let new_val = (*leftLen.borrow().as_ref().unwrap()) >= (*balanceThreshold.borrow().as_ref().unwrap()); *wasBalanced.borrow_mut() = Some(new_val); };
        __slices_pdqsort_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some(mid))), Rc::new(RefCell::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        { let new_val = mid + 1; *a.borrow_mut() = Some(new_val); };
    } else {
        { let new_val = (*rightLen.borrow().as_ref().unwrap()) >= (*balanceThreshold.borrow().as_ref().unwrap()); *wasBalanced.borrow_mut() = Some(new_val); };
        __slices_pdqsort_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some(mid + 1))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        { let new_val = mid; *b.borrow_mut() = Some(new_val); };
    }
    }
}

pub fn __slices_pdqsort_ordered___celsius(data: Rc<RefCell<Option<Vec<Celsius>>>>, mut a: Rc<RefCell<Option<i64>>>, mut b: Rc<RefCell<Option<i64>>>, mut limit: Rc<RefCell<Option<i64>>>) {
    const maxInsertion: i64 = 12;

    let mut wasBalanced = Rc::new(RefCell::new(Some(true)));let mut wasPartitioned = Rc::new(RefCell::new(Some(true)));
    loop {
        let mut length = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
        if (*length.borrow().as_ref().unwrap()) <= 12 {
        __slices_insertion_sort_ordered___celsius(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return;
    }
        if (*limit.borrow().as_ref().unwrap()) == 0 {
        __slices_heap_sort_ordered___celsius(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return;
    }
        if !(*wasBalanced.borrow().as_ref().unwrap()) {
        __slices_break_patterns_ordered___celsius(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let mut guard = limit.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        let (mut pivot, mut hint) = __slices_choose_pivot_ordered___celsius(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if (*hint.borrow().as_ref().unwrap()) == __slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_DECREASING_HINT as i64)))) {
        __slices_reverse_range_ordered___celsius(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = ((*b.borrow().as_ref().unwrap()) - 1) - (pivot - (*a.borrow().as_ref().unwrap())); pivot = new_val; };
        { let new_val = __slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_INCREASING_HINT as i64)))); *hint.borrow_mut() = Some(new_val); };
    }
        if (*wasBalanced.borrow().as_ref().unwrap()) && (*wasPartitioned.borrow().as_ref().unwrap()) && (*hint.borrow().as_ref().unwrap()) == __slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_INCREASING_HINT as i64)))) {
        if __slices_partial_insertion_sort_ordered___celsius(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return;
    }
    }
        if (*a.borrow().as_ref().unwrap()) > 0 && !__cmp__less___celsius(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[(pivot) as usize].clone(); __arg_value })))) {
        let mut mid = __slices_partition_equal_ordered___celsius(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some(pivot))));
        { let new_val = mid; *a.borrow_mut() = Some(new_val); };
        continue
    }
        let (mut mid, mut alreadyPartitioned) = __slices_partition_ordered___celsius(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some(pivot))));
        { let new_val = alreadyPartitioned; *wasPartitioned.borrow_mut() = Some(new_val); };
        let (mut leftLen, mut rightLen) = (Rc::new(RefCell::new(Some(mid - (*a.borrow().as_ref().unwrap())))), Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - mid))));
        let mut balanceThreshold = Rc::new(RefCell::new(Some((*length.borrow().as_ref().unwrap()) / 8)));
        if (*leftLen.borrow().as_ref().unwrap()) < (*rightLen.borrow().as_ref().unwrap()) {
        { let new_val = (*leftLen.borrow().as_ref().unwrap()) >= (*balanceThreshold.borrow().as_ref().unwrap()); *wasBalanced.borrow_mut() = Some(new_val); };
        __slices_pdqsort_ordered___celsius(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some(mid))), Rc::new(RefCell::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = mid + 1; *a.borrow_mut() = Some(new_val); };
    } else {
        { let new_val = (*rightLen.borrow().as_ref().unwrap()) >= (*balanceThreshold.borrow().as_ref().unwrap()); *wasBalanced.borrow_mut() = Some(new_val); };
        __slices_pdqsort_ordered___celsius(data.clone(), Rc::new(RefCell::new(Some(mid + 1))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = mid; *b.borrow_mut() = Some(new_val); };
    }
    }
}

pub fn __slices_pdqsort_ordered__int(data: Rc<RefCell<Option<Vec<i64>>>>, mut a: Rc<RefCell<Option<i64>>>, mut b: Rc<RefCell<Option<i64>>>, mut limit: Rc<RefCell<Option<i64>>>) {
    const maxInsertion: i64 = 12;

    let mut wasBalanced = Rc::new(RefCell::new(Some(true)));let mut wasPartitioned = Rc::new(RefCell::new(Some(true)));
    loop {
        let mut length = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
        if (*length.borrow().as_ref().unwrap()) <= 12 {
        __slices_insertion_sort_ordered__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return;
    }
        if (*limit.borrow().as_ref().unwrap()) == 0 {
        __slices_heap_sort_ordered__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return;
    }
        if !(*wasBalanced.borrow().as_ref().unwrap()) {
        __slices_break_patterns_ordered__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let mut guard = limit.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        let (mut pivot, mut hint) = __slices_choose_pivot_ordered__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if (*hint.borrow().as_ref().unwrap()) == __slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_DECREASING_HINT as i64)))) {
        __slices_reverse_range_ordered__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = ((*b.borrow().as_ref().unwrap()) - 1) - (pivot - (*a.borrow().as_ref().unwrap())); pivot = new_val; };
        { let new_val = __slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_INCREASING_HINT as i64)))); *hint.borrow_mut() = Some(new_val); };
    }
        if (*wasBalanced.borrow().as_ref().unwrap()) && (*wasPartitioned.borrow().as_ref().unwrap()) && (*hint.borrow().as_ref().unwrap()) == __slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_INCREASING_HINT as i64)))) {
        if __slices_partial_insertion_sort_ordered__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return;
    }
    }
        if (*a.borrow().as_ref().unwrap()) > 0 && !__cmp__less__int(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[(pivot) as usize].clone(); __arg_value })))) {
        let mut mid = __slices_partition_equal_ordered__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some(pivot))));
        { let new_val = mid; *a.borrow_mut() = Some(new_val); };
        continue
    }
        let (mut mid, mut alreadyPartitioned) = __slices_partition_ordered__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some(pivot))));
        { let new_val = alreadyPartitioned; *wasPartitioned.borrow_mut() = Some(new_val); };
        let (mut leftLen, mut rightLen) = (Rc::new(RefCell::new(Some(mid - (*a.borrow().as_ref().unwrap())))), Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - mid))));
        let mut balanceThreshold = Rc::new(RefCell::new(Some((*length.borrow().as_ref().unwrap()) / 8)));
        if (*leftLen.borrow().as_ref().unwrap()) < (*rightLen.borrow().as_ref().unwrap()) {
        { let new_val = (*leftLen.borrow().as_ref().unwrap()) >= (*balanceThreshold.borrow().as_ref().unwrap()); *wasBalanced.borrow_mut() = Some(new_val); };
        __slices_pdqsort_ordered__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some(mid))), Rc::new(RefCell::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = mid + 1; *a.borrow_mut() = Some(new_val); };
    } else {
        { let new_val = (*rightLen.borrow().as_ref().unwrap()) >= (*balanceThreshold.borrow().as_ref().unwrap()); *wasBalanced.borrow_mut() = Some(new_val); };
        __slices_pdqsort_ordered__int(data.clone(), Rc::new(RefCell::new(Some(mid + 1))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = mid; *b.borrow_mut() = Some(new_val); };
    }
    }
}

pub fn __slices_pdqsort_ordered__float64(data: Rc<RefCell<Option<Vec<f64>>>>, mut a: Rc<RefCell<Option<i64>>>, mut b: Rc<RefCell<Option<i64>>>, mut limit: Rc<RefCell<Option<i64>>>) {
    const maxInsertion: i64 = 12;

    let mut wasBalanced = Rc::new(RefCell::new(Some(true)));let mut wasPartitioned = Rc::new(RefCell::new(Some(true)));
    loop {
        let mut length = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
        if (*length.borrow().as_ref().unwrap()) <= 12 {
        __slices_insertion_sort_ordered__float64(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return;
    }
        if (*limit.borrow().as_ref().unwrap()) == 0 {
        __slices_heap_sort_ordered__float64(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return;
    }
        if !(*wasBalanced.borrow().as_ref().unwrap()) {
        __slices_break_patterns_ordered__float64(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let mut guard = limit.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        let (mut pivot, mut hint) = __slices_choose_pivot_ordered__float64(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if (*hint.borrow().as_ref().unwrap()) == __slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_DECREASING_HINT as i64)))) {
        __slices_reverse_range_ordered__float64(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = ((*b.borrow().as_ref().unwrap()) - 1) - (pivot - (*a.borrow().as_ref().unwrap())); pivot = new_val; };
        { let new_val = __slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_INCREASING_HINT as i64)))); *hint.borrow_mut() = Some(new_val); };
    }
        if (*wasBalanced.borrow().as_ref().unwrap()) && (*wasPartitioned.borrow().as_ref().unwrap()) && (*hint.borrow().as_ref().unwrap()) == __slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_INCREASING_HINT as i64)))) {
        if __slices_partial_insertion_sort_ordered__float64(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return;
    }
    }
        if (*a.borrow().as_ref().unwrap()) > 0 && !__cmp__less__float64(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[(pivot) as usize].clone(); __arg_value })))) {
        let mut mid = __slices_partition_equal_ordered__float64(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some(pivot))));
        { let new_val = mid; *a.borrow_mut() = Some(new_val); };
        continue
    }
        let (mut mid, mut alreadyPartitioned) = __slices_partition_ordered__float64(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some(pivot))));
        { let new_val = alreadyPartitioned; *wasPartitioned.borrow_mut() = Some(new_val); };
        let (mut leftLen, mut rightLen) = (Rc::new(RefCell::new(Some(mid - (*a.borrow().as_ref().unwrap())))), Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - mid))));
        let mut balanceThreshold = Rc::new(RefCell::new(Some((*length.borrow().as_ref().unwrap()) / 8)));
        if (*leftLen.borrow().as_ref().unwrap()) < (*rightLen.borrow().as_ref().unwrap()) {
        { let new_val = (*leftLen.borrow().as_ref().unwrap()) >= (*balanceThreshold.borrow().as_ref().unwrap()); *wasBalanced.borrow_mut() = Some(new_val); };
        __slices_pdqsort_ordered__float64(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some(mid))), Rc::new(RefCell::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = mid + 1; *a.borrow_mut() = Some(new_val); };
    } else {
        { let new_val = (*rightLen.borrow().as_ref().unwrap()) >= (*balanceThreshold.borrow().as_ref().unwrap()); *wasBalanced.borrow_mut() = Some(new_val); };
        __slices_pdqsort_ordered__float64(data.clone(), Rc::new(RefCell::new(Some(mid + 1))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = mid; *b.borrow_mut() = Some(new_val); };
    }
    }
}

pub fn __slices_pdqsort_ordered__string(data: Rc<RefCell<Option<Vec<String>>>>, mut a: Rc<RefCell<Option<i64>>>, mut b: Rc<RefCell<Option<i64>>>, mut limit: Rc<RefCell<Option<i64>>>) {
    const maxInsertion: i64 = 12;

    let mut wasBalanced = Rc::new(RefCell::new(Some(true)));let mut wasPartitioned = Rc::new(RefCell::new(Some(true)));
    loop {
        let mut length = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
        if (*length.borrow().as_ref().unwrap()) <= 12 {
        __slices_insertion_sort_ordered__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return;
    }
        if (*limit.borrow().as_ref().unwrap()) == 0 {
        __slices_heap_sort_ordered__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return;
    }
        if !(*wasBalanced.borrow().as_ref().unwrap()) {
        __slices_break_patterns_ordered__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let mut guard = limit.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        let (mut pivot, mut hint) = __slices_choose_pivot_ordered__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if (*hint.borrow().as_ref().unwrap()) == __slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_DECREASING_HINT as i64)))) {
        __slices_reverse_range_ordered__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = ((*b.borrow().as_ref().unwrap()) - 1) - (pivot - (*a.borrow().as_ref().unwrap())); pivot = new_val; };
        { let new_val = __slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_INCREASING_HINT as i64)))); *hint.borrow_mut() = Some(new_val); };
    }
        if (*wasBalanced.borrow().as_ref().unwrap()) && (*wasPartitioned.borrow().as_ref().unwrap()) && (*hint.borrow().as_ref().unwrap()) == __slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_INCREASING_HINT as i64)))) {
        if __slices_partial_insertion_sort_ordered__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return;
    }
    }
        if (*a.borrow().as_ref().unwrap()) > 0 && !__cmp__less__string(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[(pivot) as usize].clone(); __arg_value })))) {
        let mut mid = __slices_partition_equal_ordered__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some(pivot))));
        { let new_val = mid; *a.borrow_mut() = Some(new_val); };
        continue
    }
        let (mut mid, mut alreadyPartitioned) = __slices_partition_ordered__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some(pivot))));
        { let new_val = alreadyPartitioned; *wasPartitioned.borrow_mut() = Some(new_val); };
        let (mut leftLen, mut rightLen) = (Rc::new(RefCell::new(Some(mid - (*a.borrow().as_ref().unwrap())))), Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - mid))));
        let mut balanceThreshold = Rc::new(RefCell::new(Some((*length.borrow().as_ref().unwrap()) / 8)));
        if (*leftLen.borrow().as_ref().unwrap()) < (*rightLen.borrow().as_ref().unwrap()) {
        { let new_val = (*leftLen.borrow().as_ref().unwrap()) >= (*balanceThreshold.borrow().as_ref().unwrap()); *wasBalanced.borrow_mut() = Some(new_val); };
        __slices_pdqsort_ordered__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some(mid))), Rc::new(RefCell::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = mid + 1; *a.borrow_mut() = Some(new_val); };
    } else {
        { let new_val = (*rightLen.borrow().as_ref().unwrap()) >= (*balanceThreshold.borrow().as_ref().unwrap()); *wasBalanced.borrow_mut() = Some(new_val); };
        __slices_pdqsort_ordered__string(data.clone(), Rc::new(RefCell::new(Some(mid + 1))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = mid; *b.borrow_mut() = Some(new_val); };
    }
    }
}

pub fn __slices_is_na_n__int(x: Rc<RefCell<Option<i64>>>) -> bool {
    { let __bin_x = (*x.borrow().as_ref().unwrap()).clone(); __bin_x != __bin_x }
}

pub fn __slices__append_seq__slice_string__string(mut s: Rc<RefCell<Option<Vec<String>>>>, seq: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<String>>>) -> bool>>>>) -> ()>>>>) -> Rc<RefCell<Option<Vec<String>>>> {
    let mut s = Rc::new(RefCell::new((*s.borrow()).clone()));
    let mut __rf_done_5: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));
    let mut __rf_result_5_0: Rc<RefCell<Option<[Vec<String>; 1]>>> = Rc::new(RefCell::new(Some(std::array::from_fn::<_, 1, _>(|_| Vec::nil()))));
    let mut s_closure_clone = s.clone(); { let __f: Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<String>>>) -> bool>>>>) -> ()> = seq.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(Rc::new(move |__rf_param_5_0: Rc<RefCell<Option<String>>>| -> bool {
        let mut s_closure_clone = s_closure_clone.clone();
        let mut v = { let __owned = __rf_param_5_0.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = s_closure_clone.clone(); let __append_value = (*v.borrow().as_ref().unwrap()).clone(); let mut __appended: Vec<String> = (*__append_target.borrow()).clone().unwrap_or_default(); __appended.push(__append_value); Rc::new(RefCell::new(Some(__appended))) })); *s_closure_clone.borrow_mut() = __slice_header; };
        true
    }) as Rc<dyn Fn(Rc<RefCell<Option<String>>>) -> bool>)))) };
    if (*__rf_done_5.borrow().as_ref().unwrap()) {
        return Rc::new(RefCell::new(Some((*__rf_result_5_0.borrow().as_ref().unwrap())[(0) as usize].clone())));
    }
    return s.clone();
}

pub fn __slices__append_seq__slice_int__int(mut s: Rc<RefCell<Option<Vec<i64>>>>, seq: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> bool>>>>) -> ()>>>>) -> Rc<RefCell<Option<Vec<i64>>>> {
    let mut s = Rc::new(RefCell::new((*s.borrow()).clone()));
    let mut __rf_done_6: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));
    let mut __rf_result_6_0: Rc<RefCell<Option<[Vec<i64>; 1]>>> = Rc::new(RefCell::new(Some(std::array::from_fn::<_, 1, _>(|_| Vec::nil()))));
    let mut s_closure_clone = s.clone(); { let __f: Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> bool>>>>) -> ()> = seq.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(Rc::new(move |__rf_param_6_0: Rc<RefCell<Option<i64>>>| -> bool {
        let mut s_closure_clone = s_closure_clone.clone();
        let mut v = { let __owned = __rf_param_6_0.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = s_closure_clone.clone(); let __append_value = (*v.borrow().as_ref().unwrap()).clone(); let mut __appended: Vec<i64> = (*__append_target.borrow()).clone().unwrap_or_default(); __appended.push(__append_value); Rc::new(RefCell::new(Some(__appended))) })); *s_closure_clone.borrow_mut() = __slice_header; };
        true
    }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> bool>)))) };
    if (*__rf_done_6.borrow().as_ref().unwrap()) {
        return Rc::new(RefCell::new(Some((*__rf_result_6_0.borrow().as_ref().unwrap())[(0) as usize].clone())));
    }
    return s.clone();
}

pub fn __slices_rotate_left__int(s: Rc<RefCell<Option<Vec<i64>>>>, r: Rc<RefCell<Option<i64>>>) {
    __slices__reverse__slice_int__int(Rc::new(RefCell::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = 0; let __high = ((*r.borrow().as_ref().unwrap())) as usize; let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) }))));
    __slices__reverse__slice_int__int(Rc::new(RefCell::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = ((*r.borrow().as_ref().unwrap())) as usize; let __high = __seq.len(); let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) }))));
    __slices__reverse__slice_int__int(s.clone());
}

pub fn __slices_insertion_sort_cmp_func___item(data: Rc<RefCell<Option<Vec<Item>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Item>>>, Rc<RefCell<Option<Item>>>) -> i64>>>>) {
    let mut i = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1)));
    while (*i.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap()) {
        let mut j = { let __owned = i.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    while (*j.borrow().as_ref().unwrap()) > (*a.borrow().as_ref().unwrap()) && ({ let __f: Rc<dyn Fn(Rc<RefCell<Option<Item>>>, Rc<RefCell<Option<Item>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value })))) } < 0) {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

pub fn __slices_insertion_sort_cmp_func__ptr__task(data: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Task>>>>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Task>>>, Rc<RefCell<Option<Task>>>) -> i64>>>>) {
    let mut i = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1)));
    while (*i.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap()) {
        let mut j = { let __owned = i.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    while (*j.borrow().as_ref().unwrap()) > (*a.borrow().as_ref().unwrap()) && ({ let __f: Rc<dyn Fn(Rc<RefCell<Option<Task>>>, Rc<RefCell<Option<Task>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)((*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(), (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone()) } < 0) {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

pub fn __slices_insertion_sort_cmp_func__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>>>>) {
    let mut i = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1)));
    while (*i.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap()) {
        let mut j = { let __owned = i.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    while (*j.borrow().as_ref().unwrap()) > (*a.borrow().as_ref().unwrap()) && ({ let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value })))) } < 0) {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

pub fn __slices_sym_merge_cmp_func___item(data: Rc<RefCell<Option<Vec<Item>>>>, a: Rc<RefCell<Option<i64>>>, m: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Item>>>, Rc<RefCell<Option<Item>>>) -> i64>>>>) {
    if (*m.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()) == 1 {
        let mut i = { let __owned = m.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
        let mut j = { let __owned = b.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
        while (*i.borrow().as_ref().unwrap()) < (*j.borrow().as_ref().unwrap()) {
        let mut h = Rc::new(RefCell::new(Some((((*Rc::new(RefCell::new(Some((((*i.borrow().as_ref().unwrap()) + (*j.borrow().as_ref().unwrap()))) as u64))).borrow().as_ref().unwrap()) >> 1)) as i64)));
        if { let __f: Rc<dyn Fn(Rc<RefCell<Option<Item>>>, Rc<RefCell<Option<Item>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*h.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) } < 0 {
        { let new_val = (*h.borrow().as_ref().unwrap()) + 1; *i.borrow_mut() = Some(new_val); };
    } else {
        { let new_val = h.borrow().as_ref().unwrap().clone(); *j.borrow_mut() = Some(new_val); };
    }
    }
        let mut k = { let __owned = a.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    while (*k.borrow().as_ref().unwrap()) < (*i.borrow().as_ref().unwrap()) - 1 {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*k.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*k.borrow().as_ref().unwrap()) + 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*k.borrow().as_ref().unwrap()) + 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*k.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = k.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        return;
    }
    if (*b.borrow().as_ref().unwrap()) - (*m.borrow().as_ref().unwrap()) == 1 {
        let mut i = { let __owned = a.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
        let mut j = { let __owned = m.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
        while (*i.borrow().as_ref().unwrap()) < (*j.borrow().as_ref().unwrap()) {
        let mut h = Rc::new(RefCell::new(Some((((*Rc::new(RefCell::new(Some((((*i.borrow().as_ref().unwrap()) + (*j.borrow().as_ref().unwrap()))) as u64))).borrow().as_ref().unwrap()) >> 1)) as i64)));
        if !({ let __f: Rc<dyn Fn(Rc<RefCell<Option<Item>>>, Rc<RefCell<Option<Item>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*m.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*h.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) } < 0) {
        { let new_val = (*h.borrow().as_ref().unwrap()) + 1; *i.borrow_mut() = Some(new_val); };
    } else {
        { let new_val = h.borrow().as_ref().unwrap().clone(); *j.borrow_mut() = Some(new_val); };
    }
    }
        let mut k = { let __owned = m.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    while (*k.borrow().as_ref().unwrap()) > (*i.borrow().as_ref().unwrap()) {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*k.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*k.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*k.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*k.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = k.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        return;
    }
    let mut mid = Rc::new(RefCell::new(Some((((*Rc::new(RefCell::new(Some((((*a.borrow().as_ref().unwrap()) + (*b.borrow().as_ref().unwrap()))) as u64))).borrow().as_ref().unwrap()) >> 1)) as i64)));
    let mut n = Rc::new(RefCell::new(Some((*mid.borrow().as_ref().unwrap()) + (*m.borrow().as_ref().unwrap()))));
    let mut start: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));let mut r: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));
    if (*m.borrow().as_ref().unwrap()) > (*mid.borrow().as_ref().unwrap()) {
        { let new_val = (*n.borrow().as_ref().unwrap()) - (*b.borrow().as_ref().unwrap()); *start.borrow_mut() = Some(new_val); };
        { let new_val = mid.borrow().as_ref().unwrap().clone(); *r.borrow_mut() = Some(new_val); };
    } else {
        { let new_val = a.borrow().as_ref().unwrap().clone(); *start.borrow_mut() = Some(new_val); };
        { let new_val = m.borrow().as_ref().unwrap().clone(); *r.borrow_mut() = Some(new_val); };
    }
    let mut p = Rc::new(RefCell::new(Some((*n.borrow().as_ref().unwrap()) - 1)));
    while (*start.borrow().as_ref().unwrap()) < (*r.borrow().as_ref().unwrap()) {
        let mut c = Rc::new(RefCell::new(Some((((*Rc::new(RefCell::new(Some((((*start.borrow().as_ref().unwrap()) + (*r.borrow().as_ref().unwrap()))) as u64))).borrow().as_ref().unwrap()) >> 1)) as i64)));
        if !({ let __f: Rc<dyn Fn(Rc<RefCell<Option<Item>>>, Rc<RefCell<Option<Item>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*p.borrow().as_ref().unwrap()) - (*c.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*c.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) } < 0) {
        { let new_val = (*c.borrow().as_ref().unwrap()) + 1; *start.borrow_mut() = Some(new_val); };
    } else {
        { let new_val = c.borrow().as_ref().unwrap().clone(); *r.borrow_mut() = Some(new_val); };
    }
    }
    let mut end = Rc::new(RefCell::new(Some((*n.borrow().as_ref().unwrap()) - (*start.borrow().as_ref().unwrap()))));
    if (*start.borrow().as_ref().unwrap()) < (*m.borrow().as_ref().unwrap()) && (*m.borrow().as_ref().unwrap()) < (*end.borrow().as_ref().unwrap()) {
        __slices_rotate_cmp_func___item(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = start.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = m.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = end.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
    }
    if (*a.borrow().as_ref().unwrap()) < (*start.borrow().as_ref().unwrap()) && (*start.borrow().as_ref().unwrap()) < (*mid.borrow().as_ref().unwrap()) {
        __slices_sym_merge_cmp_func___item(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = start.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = mid.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
    }
    if (*mid.borrow().as_ref().unwrap()) < (*end.borrow().as_ref().unwrap()) && (*end.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap()) {
        __slices_sym_merge_cmp_func___item(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = mid.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = end.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
    }
}

pub fn __slices_heap_sort_cmp_func__ptr__task(data: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Task>>>>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Task>>>, Rc<RefCell<Option<Task>>>) -> i64>>>>) {
    let mut first = { let __owned = a.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    let mut lo = Rc::new(RefCell::new(Some(0i64)));
    let mut hi = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
    let mut i = Rc::new(RefCell::new(Some(((*hi.borrow().as_ref().unwrap()) - 1) / 2)));
    while (*i.borrow().as_ref().unwrap()) >= 0 {
        __slices_sift_down_cmp_func__ptr__task(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = hi.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    let mut i = Rc::new(RefCell::new(Some((*hi.borrow().as_ref().unwrap()) - 1)));
    while (*i.borrow().as_ref().unwrap()) >= 0 {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*first.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*first.borrow().as_ref().unwrap()) + (*i.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*i.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        __slices_sift_down_cmp_func__ptr__task(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = lo.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

pub fn __slices_heap_sort_cmp_func__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>>>>) {
    let mut first = { let __owned = a.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    let mut lo = Rc::new(RefCell::new(Some(0i64)));
    let mut hi = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
    let mut i = Rc::new(RefCell::new(Some(((*hi.borrow().as_ref().unwrap()) - 1) / 2)));
    while (*i.borrow().as_ref().unwrap()) >= 0 {
        __slices_sift_down_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = hi.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    let mut i = Rc::new(RefCell::new(Some((*hi.borrow().as_ref().unwrap()) - 1)));
    while (*i.borrow().as_ref().unwrap()) >= 0 {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*first.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*first.borrow().as_ref().unwrap()) + (*i.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*i.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        __slices_sift_down_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = lo.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

pub fn __slices_break_patterns_cmp_func__ptr__task(data: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Task>>>>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Task>>>, Rc<RefCell<Option<Task>>>) -> i64>>>>) {
    let mut length = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
    if (*length.borrow().as_ref().unwrap()) >= 8 {
        let mut random = Rc::new(RefCell::new(Some(__slices_xorshift(Rc::new(RefCell::new(Some((*length.borrow().as_ref().unwrap()) as u64)))))));
        let mut modulus = __slices_next_power_of_two(Rc::new(RefCell::new(Some({ let __arg_holder = length.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut idx = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + ((*length.borrow().as_ref().unwrap()) / 4) * 2 - 1)));
    while (*idx.borrow().as_ref().unwrap()) <= (*a.borrow().as_ref().unwrap()) + ((*length.borrow().as_ref().unwrap()) / 4) * 2 + 1 {
        let mut other = Rc::new(RefCell::new(Some((((*Rc::new(RefCell::new(Some((*random.borrow_mut().as_mut().unwrap()).next() as u64))).borrow().as_ref().unwrap()) & (modulus - 1 as u64))) as i64)));
        if (*other.borrow().as_ref().unwrap()) >= (*length.borrow().as_ref().unwrap()) {
        { let __rhs = (*length.borrow().as_ref().unwrap()); let mut guard = other.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*idx.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*a.borrow().as_ref().unwrap()) + (*other.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap()) + (*other.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*idx.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = idx.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
}

pub fn __slices_break_patterns_cmp_func__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>>>>) {
    let mut length = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
    if (*length.borrow().as_ref().unwrap()) >= 8 {
        let mut random = Rc::new(RefCell::new(Some(__slices_xorshift(Rc::new(RefCell::new(Some((*length.borrow().as_ref().unwrap()) as u64)))))));
        let mut modulus = __slices_next_power_of_two(Rc::new(RefCell::new(Some({ let __arg_holder = length.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut idx = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + ((*length.borrow().as_ref().unwrap()) / 4) * 2 - 1)));
    while (*idx.borrow().as_ref().unwrap()) <= (*a.borrow().as_ref().unwrap()) + ((*length.borrow().as_ref().unwrap()) / 4) * 2 + 1 {
        let mut other = Rc::new(RefCell::new(Some((((*Rc::new(RefCell::new(Some((*random.borrow_mut().as_mut().unwrap()).next() as u64))).borrow().as_ref().unwrap()) & (modulus - 1 as u64))) as i64)));
        if (*other.borrow().as_ref().unwrap()) >= (*length.borrow().as_ref().unwrap()) {
        { let __rhs = (*length.borrow().as_ref().unwrap()); let mut guard = other.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*idx.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*a.borrow().as_ref().unwrap()) + (*other.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap()) + (*other.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*idx.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = idx.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
}

pub fn __slices_choose_pivot_cmp_func__ptr__task(data: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Task>>>>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Task>>>, Rc<RefCell<Option<Task>>>) -> i64>>>>) -> (i64, Rc<RefCell<Option<__slices_sortedHint>>>) {
    let mut pivot: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));
    let mut hint: Rc<RefCell<Option<__slices_sortedHint>>> = Rc::new(RefCell::new(Some(Default::default())));

    const shortestNinther: i64 = 50;
const maxSwaps: i64 = 4 * 3;

    let mut l = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
    let mut swaps: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));let mut i = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + (*l.borrow().as_ref().unwrap()) / 4 * 1)));let mut j = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + (*l.borrow().as_ref().unwrap()) / 4 * 2)));let mut k = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + (*l.borrow().as_ref().unwrap()) / 4 * 3)));
    if (*l.borrow().as_ref().unwrap()) >= 8 {
        if (*l.borrow().as_ref().unwrap()) >= 50 {
        { let new_val = __slices_median_adjacent_cmp_func__ptr__task(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone(), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value }))); *i.borrow_mut() = Some(new_val); };
        { let new_val = __slices_median_adjacent_cmp_func__ptr__task(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone(), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value }))); *j.borrow_mut() = Some(new_val); };
        { let new_val = __slices_median_adjacent_cmp_func__ptr__task(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone(), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value }))); *k.borrow_mut() = Some(new_val); };
    }
        { let new_val = __slices_median_cmp_func__ptr__task(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone(), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value }))); *j.borrow_mut() = Some(new_val); };
    }
    { let _switch_val = (*swaps.borrow().as_ref().unwrap());
    if _switch_val == (0) {
            return ((*j.borrow().as_ref().unwrap()), Rc::new(RefCell::new(Some(__slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_INCREASING_HINT as i64))))))));
        } else if _switch_val == (12) {
            return ((*j.borrow().as_ref().unwrap()), Rc::new(RefCell::new(Some(__slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_DECREASING_HINT as i64))))))));
        } else {
            return ((*j.borrow().as_ref().unwrap()), Rc::new(RefCell::new(Some(__slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_UNKNOWN_HINT as i64))))))));
        }
    }
}

pub fn __slices_choose_pivot_cmp_func__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>>>>) -> (i64, Rc<RefCell<Option<__slices_sortedHint>>>) {
    let mut pivot: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));
    let mut hint: Rc<RefCell<Option<__slices_sortedHint>>> = Rc::new(RefCell::new(Some(Default::default())));

    const shortestNinther: i64 = 50;
const maxSwaps: i64 = 4 * 3;

    let mut l = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
    let mut swaps: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));let mut i = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + (*l.borrow().as_ref().unwrap()) / 4 * 1)));let mut j = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + (*l.borrow().as_ref().unwrap()) / 4 * 2)));let mut k = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + (*l.borrow().as_ref().unwrap()) / 4 * 3)));
    if (*l.borrow().as_ref().unwrap()) >= 8 {
        if (*l.borrow().as_ref().unwrap()) >= 50 {
        { let new_val = __slices_median_adjacent_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone(), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value }))); *i.borrow_mut() = Some(new_val); };
        { let new_val = __slices_median_adjacent_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone(), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value }))); *j.borrow_mut() = Some(new_val); };
        { let new_val = __slices_median_adjacent_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone(), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value }))); *k.borrow_mut() = Some(new_val); };
    }
        { let new_val = __slices_median_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone(), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value }))); *j.borrow_mut() = Some(new_val); };
    }
    { let _switch_val = (*swaps.borrow().as_ref().unwrap());
    if _switch_val == (0) {
            return ((*j.borrow().as_ref().unwrap()), Rc::new(RefCell::new(Some(__slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_INCREASING_HINT as i64))))))));
        } else if _switch_val == (12) {
            return ((*j.borrow().as_ref().unwrap()), Rc::new(RefCell::new(Some(__slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_DECREASING_HINT as i64))))))));
        } else {
            return ((*j.borrow().as_ref().unwrap()), Rc::new(RefCell::new(Some(__slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_UNKNOWN_HINT as i64))))))));
        }
    }
}

pub fn __slices_reverse_range_cmp_func__ptr__task(data: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Task>>>>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Task>>>, Rc<RefCell<Option<Task>>>) -> i64>>>>) {
    let mut i = { let __owned = a.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    let mut j = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - 1)));
    while (*i.borrow().as_ref().unwrap()) < (*j.borrow().as_ref().unwrap()) {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

pub fn __slices_reverse_range_cmp_func__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>>>>) {
    let mut i = { let __owned = a.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    let mut j = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - 1)));
    while (*i.borrow().as_ref().unwrap()) < (*j.borrow().as_ref().unwrap()) {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

pub fn __slices_partial_insertion_sort_cmp_func__ptr__task(data: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Task>>>>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Task>>>, Rc<RefCell<Option<Task>>>) -> i64>>>>) -> bool {
    const maxSteps: i64 = 5;
const shortestShifting: i64 = 50;

    let mut i = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1)));
    let mut j = Rc::new(RefCell::new(Some(0i64)));
    while (*j.borrow().as_ref().unwrap()) < 5 {
        while (*i.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap()) && !({ let __f: Rc<dyn Fn(Rc<RefCell<Option<Task>>>, Rc<RefCell<Option<Task>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)((*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(), (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap()) - 1) as usize].clone()) } < 0) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if (*i.borrow().as_ref().unwrap()) == (*b.borrow().as_ref().unwrap()) {
        return true;
    }
        if (*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()) < 50 {
        return false;
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*i.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        if (*i.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()) >= 2 {
        let mut j = Rc::new(RefCell::new(Some((*i.borrow().as_ref().unwrap()) - 1)));
    while (*j.borrow().as_ref().unwrap()) >= 1 {
        if !({ let __f: Rc<dyn Fn(Rc<RefCell<Option<Task>>>, Rc<RefCell<Option<Task>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)((*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(), (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone()) } < 0) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    }
        if (*b.borrow().as_ref().unwrap()) - (*i.borrow().as_ref().unwrap()) >= 2 {
        let mut j = Rc::new(RefCell::new(Some((*i.borrow().as_ref().unwrap()) + 1)));
    while (*j.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap()) {
        if !({ let __f: Rc<dyn Fn(Rc<RefCell<Option<Task>>>, Rc<RefCell<Option<Task>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)((*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(), (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone()) } < 0) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    false
}

pub fn __slices_partial_insertion_sort_cmp_func__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>>>>) -> bool {
    const maxSteps: i64 = 5;
const shortestShifting: i64 = 50;

    let mut i = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1)));
    let mut j = Rc::new(RefCell::new(Some(0i64)));
    while (*j.borrow().as_ref().unwrap()) < 5 {
        while (*i.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap()) && !({ let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value })))) } < 0) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if (*i.borrow().as_ref().unwrap()) == (*b.borrow().as_ref().unwrap()) {
        return true;
    }
        if (*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()) < 50 {
        return false;
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*i.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        if (*i.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()) >= 2 {
        let mut j = Rc::new(RefCell::new(Some((*i.borrow().as_ref().unwrap()) - 1)));
    while (*j.borrow().as_ref().unwrap()) >= 1 {
        if !({ let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value })))) } < 0) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    }
        if (*b.borrow().as_ref().unwrap()) - (*i.borrow().as_ref().unwrap()) >= 2 {
        let mut j = Rc::new(RefCell::new(Some((*i.borrow().as_ref().unwrap()) + 1)));
    while (*j.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap()) {
        if !({ let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value })))) } < 0) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    false
}

pub fn __slices_partition_equal_cmp_func__ptr__task(data: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Task>>>>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, pivot: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Task>>>, Rc<RefCell<Option<Task>>>) -> i64>>>>) -> i64 {
    let mut newpivot: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));

    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*a.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*pivot.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*pivot.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    let (mut i, mut j) = (Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1))), Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - 1))));
    loop {
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && !({ let __f: Rc<dyn Fn(Rc<RefCell<Option<Task>>>, Rc<RefCell<Option<Task>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)((*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(), (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone()) } < 0) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && ({ let __f: Rc<dyn Fn(Rc<RefCell<Option<Task>>>, Rc<RefCell<Option<Task>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)((*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(), (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone()) } < 0) {
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if (*i.borrow().as_ref().unwrap()) > (*j.borrow().as_ref().unwrap()) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    return (*i.borrow().as_ref().unwrap());
}

pub fn __slices_partition_equal_cmp_func__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, pivot: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>>>>) -> i64 {
    let mut newpivot: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));

    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*a.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*pivot.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*pivot.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    let (mut i, mut j) = (Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1))), Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - 1))));
    loop {
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && !({ let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) } < 0) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && ({ let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) } < 0) {
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if (*i.borrow().as_ref().unwrap()) > (*j.borrow().as_ref().unwrap()) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    return (*i.borrow().as_ref().unwrap());
}

pub fn __slices_partition_cmp_func__ptr__task(data: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Task>>>>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, pivot: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Task>>>, Rc<RefCell<Option<Task>>>) -> i64>>>>) -> (i64, bool) {
    let mut newpivot: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));
    let mut alreadyPartitioned: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));

    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*a.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*pivot.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*pivot.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    let (mut i, mut j) = (Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1))), Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - 1))));
    while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && ({ let __f: Rc<dyn Fn(Rc<RefCell<Option<Task>>>, Rc<RefCell<Option<Task>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)((*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(), (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone()) } < 0) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && !({ let __f: Rc<dyn Fn(Rc<RefCell<Option<Task>>>, Rc<RefCell<Option<Task>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)((*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(), (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone()) } < 0) {
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    if (*i.borrow().as_ref().unwrap()) > (*j.borrow().as_ref().unwrap()) {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*a.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        return ((*j.borrow().as_ref().unwrap()), true);
    }
    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    loop {
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && ({ let __f: Rc<dyn Fn(Rc<RefCell<Option<Task>>>, Rc<RefCell<Option<Task>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)((*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(), (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone()) } < 0) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && !({ let __f: Rc<dyn Fn(Rc<RefCell<Option<Task>>>, Rc<RefCell<Option<Task>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)((*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(), (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone()) } < 0) {
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if (*i.borrow().as_ref().unwrap()) > (*j.borrow().as_ref().unwrap()) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*a.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    return ((*j.borrow().as_ref().unwrap()), false);
}

pub fn __slices_partition_cmp_func__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, pivot: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>>>>) -> (i64, bool) {
    let mut newpivot: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));
    let mut alreadyPartitioned: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));

    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*a.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*pivot.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*pivot.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    let (mut i, mut j) = (Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1))), Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - 1))));
    while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && ({ let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) } < 0) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && !({ let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) } < 0) {
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    if (*i.borrow().as_ref().unwrap()) > (*j.borrow().as_ref().unwrap()) {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*a.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        return ((*j.borrow().as_ref().unwrap()), true);
    }
    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    loop {
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && ({ let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) } < 0) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && !({ let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) } < 0) {
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if (*i.borrow().as_ref().unwrap()) > (*j.borrow().as_ref().unwrap()) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*a.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    return ((*j.borrow().as_ref().unwrap()), false);
}

pub fn __slices_insertion_sort_ordered___celsius(data: Rc<RefCell<Option<Vec<Celsius>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) {
    let mut i = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1)));
    while (*i.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap()) {
        let mut j = { let __owned = i.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    while (*j.borrow().as_ref().unwrap()) > (*a.borrow().as_ref().unwrap()) && __cmp__less___celsius(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value })))) {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

pub fn __slices_insertion_sort_ordered__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) {
    let mut i = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1)));
    while (*i.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap()) {
        let mut j = { let __owned = i.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    while (*j.borrow().as_ref().unwrap()) > (*a.borrow().as_ref().unwrap()) && __cmp__less__int(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value })))) {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

pub fn __slices_insertion_sort_ordered__float64(data: Rc<RefCell<Option<Vec<f64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) {
    let mut i = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1)));
    while (*i.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap()) {
        let mut j = { let __owned = i.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    while (*j.borrow().as_ref().unwrap()) > (*a.borrow().as_ref().unwrap()) && __cmp__less__float64(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value })))) {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

pub fn __slices_insertion_sort_ordered__string(data: Rc<RefCell<Option<Vec<String>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) {
    let mut i = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1)));
    while (*i.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap()) {
        let mut j = { let __owned = i.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    while (*j.borrow().as_ref().unwrap()) > (*a.borrow().as_ref().unwrap()) && __cmp__less__string(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value })))) {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

pub fn __slices_heap_sort_ordered___celsius(data: Rc<RefCell<Option<Vec<Celsius>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) {
    let mut first = { let __owned = a.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    let mut lo = Rc::new(RefCell::new(Some(0i64)));
    let mut hi = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
    let mut i = Rc::new(RefCell::new(Some(((*hi.borrow().as_ref().unwrap()) - 1) / 2)));
    while (*i.borrow().as_ref().unwrap()) >= 0 {
        __slices_sift_down_ordered___celsius(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = hi.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    let mut i = Rc::new(RefCell::new(Some((*hi.borrow().as_ref().unwrap()) - 1)));
    while (*i.borrow().as_ref().unwrap()) >= 0 {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*first.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*first.borrow().as_ref().unwrap()) + (*i.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*i.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        __slices_sift_down_ordered___celsius(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = lo.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

pub fn __slices_heap_sort_ordered__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) {
    let mut first = { let __owned = a.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    let mut lo = Rc::new(RefCell::new(Some(0i64)));
    let mut hi = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
    let mut i = Rc::new(RefCell::new(Some(((*hi.borrow().as_ref().unwrap()) - 1) / 2)));
    while (*i.borrow().as_ref().unwrap()) >= 0 {
        __slices_sift_down_ordered__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = hi.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    let mut i = Rc::new(RefCell::new(Some((*hi.borrow().as_ref().unwrap()) - 1)));
    while (*i.borrow().as_ref().unwrap()) >= 0 {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*first.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*first.borrow().as_ref().unwrap()) + (*i.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*i.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        __slices_sift_down_ordered__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = lo.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

pub fn __slices_heap_sort_ordered__float64(data: Rc<RefCell<Option<Vec<f64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) {
    let mut first = { let __owned = a.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    let mut lo = Rc::new(RefCell::new(Some(0i64)));
    let mut hi = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
    let mut i = Rc::new(RefCell::new(Some(((*hi.borrow().as_ref().unwrap()) - 1) / 2)));
    while (*i.borrow().as_ref().unwrap()) >= 0 {
        __slices_sift_down_ordered__float64(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = hi.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    let mut i = Rc::new(RefCell::new(Some((*hi.borrow().as_ref().unwrap()) - 1)));
    while (*i.borrow().as_ref().unwrap()) >= 0 {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*first.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*first.borrow().as_ref().unwrap()) + (*i.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*i.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        __slices_sift_down_ordered__float64(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = lo.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

pub fn __slices_heap_sort_ordered__string(data: Rc<RefCell<Option<Vec<String>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) {
    let mut first = { let __owned = a.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    let mut lo = Rc::new(RefCell::new(Some(0i64)));
    let mut hi = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
    let mut i = Rc::new(RefCell::new(Some(((*hi.borrow().as_ref().unwrap()) - 1) / 2)));
    while (*i.borrow().as_ref().unwrap()) >= 0 {
        __slices_sift_down_ordered__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = hi.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    let mut i = Rc::new(RefCell::new(Some((*hi.borrow().as_ref().unwrap()) - 1)));
    while (*i.borrow().as_ref().unwrap()) >= 0 {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*first.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*first.borrow().as_ref().unwrap()) + (*i.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*i.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        __slices_sift_down_ordered__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = lo.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

pub fn __slices_break_patterns_ordered___celsius(data: Rc<RefCell<Option<Vec<Celsius>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) {
    let mut length = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
    if (*length.borrow().as_ref().unwrap()) >= 8 {
        let mut random = Rc::new(RefCell::new(Some(__slices_xorshift(Rc::new(RefCell::new(Some((*length.borrow().as_ref().unwrap()) as u64)))))));
        let mut modulus = __slices_next_power_of_two(Rc::new(RefCell::new(Some({ let __arg_holder = length.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut idx = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + ((*length.borrow().as_ref().unwrap()) / 4) * 2 - 1)));
    while (*idx.borrow().as_ref().unwrap()) <= (*a.borrow().as_ref().unwrap()) + ((*length.borrow().as_ref().unwrap()) / 4) * 2 + 1 {
        let mut other = Rc::new(RefCell::new(Some((((*Rc::new(RefCell::new(Some((*random.borrow_mut().as_mut().unwrap()).next() as u64))).borrow().as_ref().unwrap()) & (modulus - 1 as u64))) as i64)));
        if (*other.borrow().as_ref().unwrap()) >= (*length.borrow().as_ref().unwrap()) {
        { let __rhs = (*length.borrow().as_ref().unwrap()); let mut guard = other.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*idx.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*a.borrow().as_ref().unwrap()) + (*other.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap()) + (*other.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*idx.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = idx.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
}

pub fn __slices_break_patterns_ordered__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) {
    let mut length = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
    if (*length.borrow().as_ref().unwrap()) >= 8 {
        let mut random = Rc::new(RefCell::new(Some(__slices_xorshift(Rc::new(RefCell::new(Some((*length.borrow().as_ref().unwrap()) as u64)))))));
        let mut modulus = __slices_next_power_of_two(Rc::new(RefCell::new(Some({ let __arg_holder = length.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut idx = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + ((*length.borrow().as_ref().unwrap()) / 4) * 2 - 1)));
    while (*idx.borrow().as_ref().unwrap()) <= (*a.borrow().as_ref().unwrap()) + ((*length.borrow().as_ref().unwrap()) / 4) * 2 + 1 {
        let mut other = Rc::new(RefCell::new(Some((((*Rc::new(RefCell::new(Some((*random.borrow_mut().as_mut().unwrap()).next() as u64))).borrow().as_ref().unwrap()) & (modulus - 1 as u64))) as i64)));
        if (*other.borrow().as_ref().unwrap()) >= (*length.borrow().as_ref().unwrap()) {
        { let __rhs = (*length.borrow().as_ref().unwrap()); let mut guard = other.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*idx.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*a.borrow().as_ref().unwrap()) + (*other.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap()) + (*other.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*idx.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = idx.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
}

pub fn __slices_break_patterns_ordered__float64(data: Rc<RefCell<Option<Vec<f64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) {
    let mut length = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
    if (*length.borrow().as_ref().unwrap()) >= 8 {
        let mut random = Rc::new(RefCell::new(Some(__slices_xorshift(Rc::new(RefCell::new(Some((*length.borrow().as_ref().unwrap()) as u64)))))));
        let mut modulus = __slices_next_power_of_two(Rc::new(RefCell::new(Some({ let __arg_holder = length.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut idx = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + ((*length.borrow().as_ref().unwrap()) / 4) * 2 - 1)));
    while (*idx.borrow().as_ref().unwrap()) <= (*a.borrow().as_ref().unwrap()) + ((*length.borrow().as_ref().unwrap()) / 4) * 2 + 1 {
        let mut other = Rc::new(RefCell::new(Some((((*Rc::new(RefCell::new(Some((*random.borrow_mut().as_mut().unwrap()).next() as u64))).borrow().as_ref().unwrap()) & (modulus - 1 as u64))) as i64)));
        if (*other.borrow().as_ref().unwrap()) >= (*length.borrow().as_ref().unwrap()) {
        { let __rhs = (*length.borrow().as_ref().unwrap()); let mut guard = other.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*idx.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*a.borrow().as_ref().unwrap()) + (*other.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap()) + (*other.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*idx.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = idx.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
}

pub fn __slices_break_patterns_ordered__string(data: Rc<RefCell<Option<Vec<String>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) {
    let mut length = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
    if (*length.borrow().as_ref().unwrap()) >= 8 {
        let mut random = Rc::new(RefCell::new(Some(__slices_xorshift(Rc::new(RefCell::new(Some((*length.borrow().as_ref().unwrap()) as u64)))))));
        let mut modulus = __slices_next_power_of_two(Rc::new(RefCell::new(Some({ let __arg_holder = length.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut idx = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + ((*length.borrow().as_ref().unwrap()) / 4) * 2 - 1)));
    while (*idx.borrow().as_ref().unwrap()) <= (*a.borrow().as_ref().unwrap()) + ((*length.borrow().as_ref().unwrap()) / 4) * 2 + 1 {
        let mut other = Rc::new(RefCell::new(Some((((*Rc::new(RefCell::new(Some((*random.borrow_mut().as_mut().unwrap()).next() as u64))).borrow().as_ref().unwrap()) & (modulus - 1 as u64))) as i64)));
        if (*other.borrow().as_ref().unwrap()) >= (*length.borrow().as_ref().unwrap()) {
        { let __rhs = (*length.borrow().as_ref().unwrap()); let mut guard = other.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*idx.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*a.borrow().as_ref().unwrap()) + (*other.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap()) + (*other.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*idx.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = idx.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
}

pub fn __slices_choose_pivot_ordered___celsius(data: Rc<RefCell<Option<Vec<Celsius>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) -> (i64, Rc<RefCell<Option<__slices_sortedHint>>>) {
    let mut pivot: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));
    let mut hint: Rc<RefCell<Option<__slices_sortedHint>>> = Rc::new(RefCell::new(Some(Default::default())));

    const shortestNinther: i64 = 50;
const maxSwaps: i64 = 4 * 3;

    let mut l = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
    let mut swaps: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));let mut i = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + (*l.borrow().as_ref().unwrap()) / 4 * 1)));let mut j = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + (*l.borrow().as_ref().unwrap()) / 4 * 2)));let mut k = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + (*l.borrow().as_ref().unwrap()) / 4 * 3)));
    if (*l.borrow().as_ref().unwrap()) >= 8 {
        if (*l.borrow().as_ref().unwrap()) >= 50 {
        { let new_val = __slices_median_adjacent_ordered___celsius(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *i.borrow_mut() = Some(new_val); };
        { let new_val = __slices_median_adjacent_ordered___celsius(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *j.borrow_mut() = Some(new_val); };
        { let new_val = __slices_median_adjacent_ordered___celsius(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *k.borrow_mut() = Some(new_val); };
    }
        { let new_val = __slices_median_ordered___celsius(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *j.borrow_mut() = Some(new_val); };
    }
    { let _switch_val = (*swaps.borrow().as_ref().unwrap());
    if _switch_val == (0) {
            return ((*j.borrow().as_ref().unwrap()), Rc::new(RefCell::new(Some(__slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_INCREASING_HINT as i64))))))));
        } else if _switch_val == (12) {
            return ((*j.borrow().as_ref().unwrap()), Rc::new(RefCell::new(Some(__slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_DECREASING_HINT as i64))))))));
        } else {
            return ((*j.borrow().as_ref().unwrap()), Rc::new(RefCell::new(Some(__slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_UNKNOWN_HINT as i64))))))));
        }
    }
}

pub fn __slices_choose_pivot_ordered__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) -> (i64, Rc<RefCell<Option<__slices_sortedHint>>>) {
    let mut pivot: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));
    let mut hint: Rc<RefCell<Option<__slices_sortedHint>>> = Rc::new(RefCell::new(Some(Default::default())));

    const shortestNinther: i64 = 50;
const maxSwaps: i64 = 4 * 3;

    let mut l = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
    let mut swaps: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));let mut i = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + (*l.borrow().as_ref().unwrap()) / 4 * 1)));let mut j = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + (*l.borrow().as_ref().unwrap()) / 4 * 2)));let mut k = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + (*l.borrow().as_ref().unwrap()) / 4 * 3)));
    if (*l.borrow().as_ref().unwrap()) >= 8 {
        if (*l.borrow().as_ref().unwrap()) >= 50 {
        { let new_val = __slices_median_adjacent_ordered__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *i.borrow_mut() = Some(new_val); };
        { let new_val = __slices_median_adjacent_ordered__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *j.borrow_mut() = Some(new_val); };
        { let new_val = __slices_median_adjacent_ordered__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *k.borrow_mut() = Some(new_val); };
    }
        { let new_val = __slices_median_ordered__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *j.borrow_mut() = Some(new_val); };
    }
    { let _switch_val = (*swaps.borrow().as_ref().unwrap());
    if _switch_val == (0) {
            return ((*j.borrow().as_ref().unwrap()), Rc::new(RefCell::new(Some(__slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_INCREASING_HINT as i64))))))));
        } else if _switch_val == (12) {
            return ((*j.borrow().as_ref().unwrap()), Rc::new(RefCell::new(Some(__slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_DECREASING_HINT as i64))))))));
        } else {
            return ((*j.borrow().as_ref().unwrap()), Rc::new(RefCell::new(Some(__slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_UNKNOWN_HINT as i64))))))));
        }
    }
}

pub fn __slices_choose_pivot_ordered__float64(data: Rc<RefCell<Option<Vec<f64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) -> (i64, Rc<RefCell<Option<__slices_sortedHint>>>) {
    let mut pivot: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));
    let mut hint: Rc<RefCell<Option<__slices_sortedHint>>> = Rc::new(RefCell::new(Some(Default::default())));

    const shortestNinther: i64 = 50;
const maxSwaps: i64 = 4 * 3;

    let mut l = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
    let mut swaps: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));let mut i = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + (*l.borrow().as_ref().unwrap()) / 4 * 1)));let mut j = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + (*l.borrow().as_ref().unwrap()) / 4 * 2)));let mut k = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + (*l.borrow().as_ref().unwrap()) / 4 * 3)));
    if (*l.borrow().as_ref().unwrap()) >= 8 {
        if (*l.borrow().as_ref().unwrap()) >= 50 {
        { let new_val = __slices_median_adjacent_ordered__float64(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *i.borrow_mut() = Some(new_val); };
        { let new_val = __slices_median_adjacent_ordered__float64(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *j.borrow_mut() = Some(new_val); };
        { let new_val = __slices_median_adjacent_ordered__float64(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *k.borrow_mut() = Some(new_val); };
    }
        { let new_val = __slices_median_ordered__float64(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *j.borrow_mut() = Some(new_val); };
    }
    { let _switch_val = (*swaps.borrow().as_ref().unwrap());
    if _switch_val == (0) {
            return ((*j.borrow().as_ref().unwrap()), Rc::new(RefCell::new(Some(__slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_INCREASING_HINT as i64))))))));
        } else if _switch_val == (12) {
            return ((*j.borrow().as_ref().unwrap()), Rc::new(RefCell::new(Some(__slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_DECREASING_HINT as i64))))))));
        } else {
            return ((*j.borrow().as_ref().unwrap()), Rc::new(RefCell::new(Some(__slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_UNKNOWN_HINT as i64))))))));
        }
    }
}

pub fn __slices_choose_pivot_ordered__string(data: Rc<RefCell<Option<Vec<String>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) -> (i64, Rc<RefCell<Option<__slices_sortedHint>>>) {
    let mut pivot: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));
    let mut hint: Rc<RefCell<Option<__slices_sortedHint>>> = Rc::new(RefCell::new(Some(Default::default())));

    const shortestNinther: i64 = 50;
const maxSwaps: i64 = 4 * 3;

    let mut l = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
    let mut swaps: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));let mut i = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + (*l.borrow().as_ref().unwrap()) / 4 * 1)));let mut j = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + (*l.borrow().as_ref().unwrap()) / 4 * 2)));let mut k = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + (*l.borrow().as_ref().unwrap()) / 4 * 3)));
    if (*l.borrow().as_ref().unwrap()) >= 8 {
        if (*l.borrow().as_ref().unwrap()) >= 50 {
        { let new_val = __slices_median_adjacent_ordered__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *i.borrow_mut() = Some(new_val); };
        { let new_val = __slices_median_adjacent_ordered__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *j.borrow_mut() = Some(new_val); };
        { let new_val = __slices_median_adjacent_ordered__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *k.borrow_mut() = Some(new_val); };
    }
        { let new_val = __slices_median_ordered__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *j.borrow_mut() = Some(new_val); };
    }
    { let _switch_val = (*swaps.borrow().as_ref().unwrap());
    if _switch_val == (0) {
            return ((*j.borrow().as_ref().unwrap()), Rc::new(RefCell::new(Some(__slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_INCREASING_HINT as i64))))))));
        } else if _switch_val == (12) {
            return ((*j.borrow().as_ref().unwrap()), Rc::new(RefCell::new(Some(__slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_DECREASING_HINT as i64))))))));
        } else {
            return ((*j.borrow().as_ref().unwrap()), Rc::new(RefCell::new(Some(__slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_UNKNOWN_HINT as i64))))))));
        }
    }
}

pub fn __slices_reverse_range_ordered___celsius(data: Rc<RefCell<Option<Vec<Celsius>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) {
    let mut i = { let __owned = a.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    let mut j = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - 1)));
    while (*i.borrow().as_ref().unwrap()) < (*j.borrow().as_ref().unwrap()) {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

pub fn __slices_reverse_range_ordered__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) {
    let mut i = { let __owned = a.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    let mut j = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - 1)));
    while (*i.borrow().as_ref().unwrap()) < (*j.borrow().as_ref().unwrap()) {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

pub fn __slices_reverse_range_ordered__float64(data: Rc<RefCell<Option<Vec<f64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) {
    let mut i = { let __owned = a.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    let mut j = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - 1)));
    while (*i.borrow().as_ref().unwrap()) < (*j.borrow().as_ref().unwrap()) {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

pub fn __slices_reverse_range_ordered__string(data: Rc<RefCell<Option<Vec<String>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) {
    let mut i = { let __owned = a.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    let mut j = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - 1)));
    while (*i.borrow().as_ref().unwrap()) < (*j.borrow().as_ref().unwrap()) {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

pub fn __slices_partial_insertion_sort_ordered___celsius(data: Rc<RefCell<Option<Vec<Celsius>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) -> bool {
    const maxSteps: i64 = 5;
const shortestShifting: i64 = 50;

    let mut i = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1)));
    let mut j = Rc::new(RefCell::new(Some(0i64)));
    while (*j.borrow().as_ref().unwrap()) < 5 {
        while (*i.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap()) && !__cmp__less___celsius(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value })))) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if (*i.borrow().as_ref().unwrap()) == (*b.borrow().as_ref().unwrap()) {
        return true;
    }
        if (*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()) < 50 {
        return false;
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*i.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        if (*i.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()) >= 2 {
        let mut j = Rc::new(RefCell::new(Some((*i.borrow().as_ref().unwrap()) - 1)));
    while (*j.borrow().as_ref().unwrap()) >= 1 {
        if !__cmp__less___celsius(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value })))) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    }
        if (*b.borrow().as_ref().unwrap()) - (*i.borrow().as_ref().unwrap()) >= 2 {
        let mut j = Rc::new(RefCell::new(Some((*i.borrow().as_ref().unwrap()) + 1)));
    while (*j.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap()) {
        if !__cmp__less___celsius(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value })))) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    false
}

pub fn __slices_partial_insertion_sort_ordered__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) -> bool {
    const maxSteps: i64 = 5;
const shortestShifting: i64 = 50;

    let mut i = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1)));
    let mut j = Rc::new(RefCell::new(Some(0i64)));
    while (*j.borrow().as_ref().unwrap()) < 5 {
        while (*i.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap()) && !__cmp__less__int(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value })))) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if (*i.borrow().as_ref().unwrap()) == (*b.borrow().as_ref().unwrap()) {
        return true;
    }
        if (*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()) < 50 {
        return false;
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*i.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        if (*i.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()) >= 2 {
        let mut j = Rc::new(RefCell::new(Some((*i.borrow().as_ref().unwrap()) - 1)));
    while (*j.borrow().as_ref().unwrap()) >= 1 {
        if !__cmp__less__int(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value })))) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    }
        if (*b.borrow().as_ref().unwrap()) - (*i.borrow().as_ref().unwrap()) >= 2 {
        let mut j = Rc::new(RefCell::new(Some((*i.borrow().as_ref().unwrap()) + 1)));
    while (*j.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap()) {
        if !__cmp__less__int(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value })))) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    false
}

pub fn __slices_partial_insertion_sort_ordered__float64(data: Rc<RefCell<Option<Vec<f64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) -> bool {
    const maxSteps: i64 = 5;
const shortestShifting: i64 = 50;

    let mut i = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1)));
    let mut j = Rc::new(RefCell::new(Some(0i64)));
    while (*j.borrow().as_ref().unwrap()) < 5 {
        while (*i.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap()) && !__cmp__less__float64(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value })))) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if (*i.borrow().as_ref().unwrap()) == (*b.borrow().as_ref().unwrap()) {
        return true;
    }
        if (*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()) < 50 {
        return false;
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*i.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        if (*i.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()) >= 2 {
        let mut j = Rc::new(RefCell::new(Some((*i.borrow().as_ref().unwrap()) - 1)));
    while (*j.borrow().as_ref().unwrap()) >= 1 {
        if !__cmp__less__float64(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value })))) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    }
        if (*b.borrow().as_ref().unwrap()) - (*i.borrow().as_ref().unwrap()) >= 2 {
        let mut j = Rc::new(RefCell::new(Some((*i.borrow().as_ref().unwrap()) + 1)));
    while (*j.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap()) {
        if !__cmp__less__float64(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value })))) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    false
}

pub fn __slices_partial_insertion_sort_ordered__string(data: Rc<RefCell<Option<Vec<String>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) -> bool {
    const maxSteps: i64 = 5;
const shortestShifting: i64 = 50;

    let mut i = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1)));
    let mut j = Rc::new(RefCell::new(Some(0i64)));
    while (*j.borrow().as_ref().unwrap()) < 5 {
        while (*i.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap()) && !__cmp__less__string(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value })))) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if (*i.borrow().as_ref().unwrap()) == (*b.borrow().as_ref().unwrap()) {
        return true;
    }
        if (*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()) < 50 {
        return false;
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*i.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        if (*i.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()) >= 2 {
        let mut j = Rc::new(RefCell::new(Some((*i.borrow().as_ref().unwrap()) - 1)));
    while (*j.borrow().as_ref().unwrap()) >= 1 {
        if !__cmp__less__string(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value })))) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    }
        if (*b.borrow().as_ref().unwrap()) - (*i.borrow().as_ref().unwrap()) >= 2 {
        let mut j = Rc::new(RefCell::new(Some((*i.borrow().as_ref().unwrap()) + 1)));
    while (*j.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap()) {
        if !__cmp__less__string(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value })))) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    false
}

pub fn __slices_partition_equal_ordered___celsius(data: Rc<RefCell<Option<Vec<Celsius>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, pivot: Rc<RefCell<Option<i64>>>) -> i64 {
    let mut newpivot: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));

    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*a.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*pivot.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*pivot.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    let (mut i, mut j) = (Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1))), Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - 1))));
    loop {
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && !__cmp__less___celsius(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && __cmp__less___celsius(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) {
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if (*i.borrow().as_ref().unwrap()) > (*j.borrow().as_ref().unwrap()) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    return (*i.borrow().as_ref().unwrap());
}

pub fn __slices_partition_equal_ordered__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, pivot: Rc<RefCell<Option<i64>>>) -> i64 {
    let mut newpivot: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));

    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*a.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*pivot.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*pivot.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    let (mut i, mut j) = (Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1))), Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - 1))));
    loop {
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && !__cmp__less__int(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && __cmp__less__int(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) {
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if (*i.borrow().as_ref().unwrap()) > (*j.borrow().as_ref().unwrap()) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    return (*i.borrow().as_ref().unwrap());
}

pub fn __slices_partition_equal_ordered__float64(data: Rc<RefCell<Option<Vec<f64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, pivot: Rc<RefCell<Option<i64>>>) -> i64 {
    let mut newpivot: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));

    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*a.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*pivot.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*pivot.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    let (mut i, mut j) = (Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1))), Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - 1))));
    loop {
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && !__cmp__less__float64(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && __cmp__less__float64(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) {
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if (*i.borrow().as_ref().unwrap()) > (*j.borrow().as_ref().unwrap()) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    return (*i.borrow().as_ref().unwrap());
}

pub fn __slices_partition_equal_ordered__string(data: Rc<RefCell<Option<Vec<String>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, pivot: Rc<RefCell<Option<i64>>>) -> i64 {
    let mut newpivot: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));

    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*a.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*pivot.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*pivot.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    let (mut i, mut j) = (Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1))), Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - 1))));
    loop {
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && !__cmp__less__string(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && __cmp__less__string(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) {
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if (*i.borrow().as_ref().unwrap()) > (*j.borrow().as_ref().unwrap()) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    return (*i.borrow().as_ref().unwrap());
}

pub fn __slices_partition_ordered___celsius(data: Rc<RefCell<Option<Vec<Celsius>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, pivot: Rc<RefCell<Option<i64>>>) -> (i64, bool) {
    let mut newpivot: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));
    let mut alreadyPartitioned: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));

    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*a.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*pivot.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*pivot.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    let (mut i, mut j) = (Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1))), Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - 1))));
    while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && __cmp__less___celsius(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && !__cmp__less___celsius(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) {
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    if (*i.borrow().as_ref().unwrap()) > (*j.borrow().as_ref().unwrap()) {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*a.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        return ((*j.borrow().as_ref().unwrap()), true);
    }
    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    loop {
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && __cmp__less___celsius(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && !__cmp__less___celsius(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) {
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if (*i.borrow().as_ref().unwrap()) > (*j.borrow().as_ref().unwrap()) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*a.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    return ((*j.borrow().as_ref().unwrap()), false);
}

pub fn __slices_partition_ordered__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, pivot: Rc<RefCell<Option<i64>>>) -> (i64, bool) {
    let mut newpivot: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));
    let mut alreadyPartitioned: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));

    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*a.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*pivot.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*pivot.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    let (mut i, mut j) = (Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1))), Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - 1))));
    while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && __cmp__less__int(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && !__cmp__less__int(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) {
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    if (*i.borrow().as_ref().unwrap()) > (*j.borrow().as_ref().unwrap()) {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*a.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        return ((*j.borrow().as_ref().unwrap()), true);
    }
    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    loop {
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && __cmp__less__int(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && !__cmp__less__int(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) {
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if (*i.borrow().as_ref().unwrap()) > (*j.borrow().as_ref().unwrap()) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*a.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    return ((*j.borrow().as_ref().unwrap()), false);
}

pub fn __slices_partition_ordered__float64(data: Rc<RefCell<Option<Vec<f64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, pivot: Rc<RefCell<Option<i64>>>) -> (i64, bool) {
    let mut newpivot: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));
    let mut alreadyPartitioned: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));

    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*a.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*pivot.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*pivot.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    let (mut i, mut j) = (Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1))), Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - 1))));
    while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && __cmp__less__float64(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && !__cmp__less__float64(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) {
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    if (*i.borrow().as_ref().unwrap()) > (*j.borrow().as_ref().unwrap()) {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*a.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        return ((*j.borrow().as_ref().unwrap()), true);
    }
    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    loop {
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && __cmp__less__float64(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && !__cmp__less__float64(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) {
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if (*i.borrow().as_ref().unwrap()) > (*j.borrow().as_ref().unwrap()) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*a.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    return ((*j.borrow().as_ref().unwrap()), false);
}

pub fn __slices_partition_ordered__string(data: Rc<RefCell<Option<Vec<String>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, pivot: Rc<RefCell<Option<i64>>>) -> (i64, bool) {
    let mut newpivot: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));
    let mut alreadyPartitioned: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));

    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*a.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*pivot.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*pivot.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    let (mut i, mut j) = (Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1))), Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - 1))));
    while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && __cmp__less__string(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && !__cmp__less__string(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) {
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    if (*i.borrow().as_ref().unwrap()) > (*j.borrow().as_ref().unwrap()) {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*a.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        return ((*j.borrow().as_ref().unwrap()), true);
    }
    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    loop {
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && __cmp__less__string(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && !__cmp__less__string(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) {
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if (*i.borrow().as_ref().unwrap()) > (*j.borrow().as_ref().unwrap()) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*a.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    return ((*j.borrow().as_ref().unwrap()), false);
}

pub fn __slices_rotate_cmp_func___item(data: Rc<RefCell<Option<Vec<Item>>>>, a: Rc<RefCell<Option<i64>>>, m: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Item>>>, Rc<RefCell<Option<Item>>>) -> i64>>>>) {
    let mut i = Rc::new(RefCell::new(Some((*m.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
    let mut j = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*m.borrow().as_ref().unwrap()))));
    while (*i.borrow().as_ref().unwrap()) != (*j.borrow().as_ref().unwrap()) {
        if (*i.borrow().as_ref().unwrap()) > (*j.borrow().as_ref().unwrap()) {
        __slices_swap_range_cmp_func___item(data.clone(), Rc::new(RefCell::new(Some({ let __arg_value = (*m.borrow().as_ref().unwrap()) - (*i.borrow().as_ref().unwrap()); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_holder = m.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        { let __rhs = (*j.borrow().as_ref().unwrap()); let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    } else {
        __slices_swap_range_cmp_func___item(data.clone(), Rc::new(RefCell::new(Some({ let __arg_value = (*m.borrow().as_ref().unwrap()) - (*i.borrow().as_ref().unwrap()); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*m.borrow().as_ref().unwrap()) + (*j.borrow().as_ref().unwrap()) - (*i.borrow().as_ref().unwrap()); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        { let __rhs = (*i.borrow().as_ref().unwrap()); let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
    }
    __slices_swap_range_cmp_func___item(data.clone(), Rc::new(RefCell::new(Some({ let __arg_value = (*m.borrow().as_ref().unwrap()) - (*i.borrow().as_ref().unwrap()); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_holder = m.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
}

pub fn __slices_sift_down_cmp_func__ptr__task(data: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Task>>>>>>>, lo: Rc<RefCell<Option<i64>>>, hi: Rc<RefCell<Option<i64>>>, first: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Task>>>, Rc<RefCell<Option<Task>>>) -> i64>>>>) {
    let mut root = { let __owned = lo.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    loop {
        let mut child = Rc::new(RefCell::new(Some(2 * (*root.borrow().as_ref().unwrap()) + 1)));
        if (*child.borrow().as_ref().unwrap()) >= (*hi.borrow().as_ref().unwrap()) {
        break
    }
        if (*child.borrow().as_ref().unwrap()) + 1 < (*hi.borrow().as_ref().unwrap()) && ({ let __f: Rc<dyn Fn(Rc<RefCell<Option<Task>>>, Rc<RefCell<Option<Task>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)((*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize].clone(), (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap()) + 1) as usize].clone()) } < 0) {
        { let mut guard = child.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if !({ let __f: Rc<dyn Fn(Rc<RefCell<Option<Task>>>, Rc<RefCell<Option<Task>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)((*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*root.borrow().as_ref().unwrap())) as usize].clone(), (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize].clone()) } < 0) {
        return;
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*first.borrow().as_ref().unwrap()) + (*root.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*root.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let new_val = child.borrow().as_ref().unwrap().clone(); *root.borrow_mut() = Some(new_val); };
    }
}

pub fn __slices_sift_down_cmp_func__int(data: Rc<RefCell<Option<Vec<i64>>>>, lo: Rc<RefCell<Option<i64>>>, hi: Rc<RefCell<Option<i64>>>, first: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>>>>) {
    let mut root = { let __owned = lo.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    loop {
        let mut child = Rc::new(RefCell::new(Some(2 * (*root.borrow().as_ref().unwrap()) + 1)));
        if (*child.borrow().as_ref().unwrap()) >= (*hi.borrow().as_ref().unwrap()) {
        break
    }
        if (*child.borrow().as_ref().unwrap()) + 1 < (*hi.borrow().as_ref().unwrap()) && ({ let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap()) + 1) as usize].clone(); __arg_value })))) } < 0) {
        { let mut guard = child.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if !({ let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*root.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) } < 0) {
        return;
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*first.borrow().as_ref().unwrap()) + (*root.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*root.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let new_val = child.borrow().as_ref().unwrap().clone(); *root.borrow_mut() = Some(new_val); };
    }
}

pub fn __slices_next_power_of_two(length: Rc<RefCell<Option<i64>>>) -> u64 {
    { let __shift_x = 1u64; let __shift_c = (math_bits::len(Rc::new(RefCell::new(Some({ let __arg_value = (*length.borrow().as_ref().unwrap()) as u64; __arg_value }))))) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }
}

pub fn __slices_median_adjacent_cmp_func__ptr__task(data: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Task>>>>>>>, a: Rc<RefCell<Option<i64>>>, swaps: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Task>>>, Rc<RefCell<Option<Task>>>) -> i64>>>>) -> i64 {
    __slices_median_cmp_func__ptr__task(data.clone(), Rc::new(RefCell::new(Some({ let __arg_value = (*a.borrow().as_ref().unwrap()) - 1; __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_value = (*a.borrow().as_ref().unwrap()) + 1; __arg_value }))), swaps.clone(), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })))
}

pub fn __slices_median_adjacent_cmp_func__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, swaps: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>>>>) -> i64 {
    __slices_median_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_value = (*a.borrow().as_ref().unwrap()) - 1; __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_value = (*a.borrow().as_ref().unwrap()) + 1; __arg_value }))), swaps.clone(), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })))
}

pub fn __slices_median_cmp_func__ptr__task(data: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Task>>>>>>>, mut a: Rc<RefCell<Option<i64>>>, mut b: Rc<RefCell<Option<i64>>>, mut c: Rc<RefCell<Option<i64>>>, swaps: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Task>>>, Rc<RefCell<Option<Task>>>) -> i64>>>>) -> i64 {
    { let (__tmp_0, __tmp_1) = __slices_order2_cmp_func__ptr__task(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone(), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value }))); *a.borrow_mut() = Some(__tmp_0); *b.borrow_mut() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = __slices_order2_cmp_func__ptr__task(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone(), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value }))); *b.borrow_mut() = Some(__tmp_0); *c.borrow_mut() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = __slices_order2_cmp_func__ptr__task(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone(), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value }))); *a.borrow_mut() = Some(__tmp_0); *b.borrow_mut() = Some(__tmp_1); };
    return (*b.borrow().as_ref().unwrap());
}

pub fn __slices_median_cmp_func__int(data: Rc<RefCell<Option<Vec<i64>>>>, mut a: Rc<RefCell<Option<i64>>>, mut b: Rc<RefCell<Option<i64>>>, mut c: Rc<RefCell<Option<i64>>>, swaps: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>>>>) -> i64 {
    { let (__tmp_0, __tmp_1) = __slices_order2_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone(), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value }))); *a.borrow_mut() = Some(__tmp_0); *b.borrow_mut() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = __slices_order2_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone(), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value }))); *b.borrow_mut() = Some(__tmp_0); *c.borrow_mut() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = __slices_order2_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone(), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value }))); *a.borrow_mut() = Some(__tmp_0); *b.borrow_mut() = Some(__tmp_1); };
    return (*b.borrow().as_ref().unwrap());
}

pub fn __slices_sift_down_ordered___celsius(data: Rc<RefCell<Option<Vec<Celsius>>>>, lo: Rc<RefCell<Option<i64>>>, hi: Rc<RefCell<Option<i64>>>, first: Rc<RefCell<Option<i64>>>) {
    let mut root = { let __owned = lo.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    loop {
        let mut child = Rc::new(RefCell::new(Some(2 * (*root.borrow().as_ref().unwrap()) + 1)));
        if (*child.borrow().as_ref().unwrap()) >= (*hi.borrow().as_ref().unwrap()) {
        break
    }
        if (*child.borrow().as_ref().unwrap()) + 1 < (*hi.borrow().as_ref().unwrap()) && __cmp__less___celsius(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap()) + 1) as usize].clone(); __arg_value })))) {
        { let mut guard = child.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if !__cmp__less___celsius(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*root.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) {
        return;
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*first.borrow().as_ref().unwrap()) + (*root.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*root.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let new_val = child.borrow().as_ref().unwrap().clone(); *root.borrow_mut() = Some(new_val); };
    }
}

pub fn __slices_sift_down_ordered__int(data: Rc<RefCell<Option<Vec<i64>>>>, lo: Rc<RefCell<Option<i64>>>, hi: Rc<RefCell<Option<i64>>>, first: Rc<RefCell<Option<i64>>>) {
    let mut root = { let __owned = lo.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    loop {
        let mut child = Rc::new(RefCell::new(Some(2 * (*root.borrow().as_ref().unwrap()) + 1)));
        if (*child.borrow().as_ref().unwrap()) >= (*hi.borrow().as_ref().unwrap()) {
        break
    }
        if (*child.borrow().as_ref().unwrap()) + 1 < (*hi.borrow().as_ref().unwrap()) && __cmp__less__int(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap()) + 1) as usize].clone(); __arg_value })))) {
        { let mut guard = child.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if !__cmp__less__int(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*root.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) {
        return;
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*first.borrow().as_ref().unwrap()) + (*root.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*root.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let new_val = child.borrow().as_ref().unwrap().clone(); *root.borrow_mut() = Some(new_val); };
    }
}

pub fn __slices_sift_down_ordered__float64(data: Rc<RefCell<Option<Vec<f64>>>>, lo: Rc<RefCell<Option<i64>>>, hi: Rc<RefCell<Option<i64>>>, first: Rc<RefCell<Option<i64>>>) {
    let mut root = { let __owned = lo.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    loop {
        let mut child = Rc::new(RefCell::new(Some(2 * (*root.borrow().as_ref().unwrap()) + 1)));
        if (*child.borrow().as_ref().unwrap()) >= (*hi.borrow().as_ref().unwrap()) {
        break
    }
        if (*child.borrow().as_ref().unwrap()) + 1 < (*hi.borrow().as_ref().unwrap()) && __cmp__less__float64(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap()) + 1) as usize].clone(); __arg_value })))) {
        { let mut guard = child.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if !__cmp__less__float64(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*root.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) {
        return;
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*first.borrow().as_ref().unwrap()) + (*root.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*root.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let new_val = child.borrow().as_ref().unwrap().clone(); *root.borrow_mut() = Some(new_val); };
    }
}

pub fn __slices_sift_down_ordered__string(data: Rc<RefCell<Option<Vec<String>>>>, lo: Rc<RefCell<Option<i64>>>, hi: Rc<RefCell<Option<i64>>>, first: Rc<RefCell<Option<i64>>>) {
    let mut root = { let __owned = lo.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    loop {
        let mut child = Rc::new(RefCell::new(Some(2 * (*root.borrow().as_ref().unwrap()) + 1)));
        if (*child.borrow().as_ref().unwrap()) >= (*hi.borrow().as_ref().unwrap()) {
        break
    }
        if (*child.borrow().as_ref().unwrap()) + 1 < (*hi.borrow().as_ref().unwrap()) && __cmp__less__string(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap()) + 1) as usize].clone(); __arg_value })))) {
        { let mut guard = child.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if !__cmp__less__string(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*root.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) {
        return;
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*first.borrow().as_ref().unwrap()) + (*root.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*root.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let new_val = child.borrow().as_ref().unwrap().clone(); *root.borrow_mut() = Some(new_val); };
    }
}

pub fn __slices_median_adjacent_ordered___celsius(data: Rc<RefCell<Option<Vec<Celsius>>>>, a: Rc<RefCell<Option<i64>>>, swaps: Rc<RefCell<Option<i64>>>) -> i64 {
    __slices_median_ordered___celsius(data.clone(), Rc::new(RefCell::new(Some({ let __arg_value = (*a.borrow().as_ref().unwrap()) - 1; __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_value = (*a.borrow().as_ref().unwrap()) + 1; __arg_value }))), swaps.clone())
}

pub fn __slices_median_adjacent_ordered__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, swaps: Rc<RefCell<Option<i64>>>) -> i64 {
    __slices_median_ordered__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_value = (*a.borrow().as_ref().unwrap()) - 1; __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_value = (*a.borrow().as_ref().unwrap()) + 1; __arg_value }))), swaps.clone())
}

pub fn __slices_median_adjacent_ordered__float64(data: Rc<RefCell<Option<Vec<f64>>>>, a: Rc<RefCell<Option<i64>>>, swaps: Rc<RefCell<Option<i64>>>) -> i64 {
    __slices_median_ordered__float64(data.clone(), Rc::new(RefCell::new(Some({ let __arg_value = (*a.borrow().as_ref().unwrap()) - 1; __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_value = (*a.borrow().as_ref().unwrap()) + 1; __arg_value }))), swaps.clone())
}

pub fn __slices_median_adjacent_ordered__string(data: Rc<RefCell<Option<Vec<String>>>>, a: Rc<RefCell<Option<i64>>>, swaps: Rc<RefCell<Option<i64>>>) -> i64 {
    __slices_median_ordered__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_value = (*a.borrow().as_ref().unwrap()) - 1; __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_value = (*a.borrow().as_ref().unwrap()) + 1; __arg_value }))), swaps.clone())
}

pub fn __slices_median_ordered___celsius(data: Rc<RefCell<Option<Vec<Celsius>>>>, mut a: Rc<RefCell<Option<i64>>>, mut b: Rc<RefCell<Option<i64>>>, mut c: Rc<RefCell<Option<i64>>>, swaps: Rc<RefCell<Option<i64>>>) -> i64 {
    { let (__tmp_0, __tmp_1) = __slices_order2_ordered___celsius(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *a.borrow_mut() = Some(__tmp_0); *b.borrow_mut() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = __slices_order2_ordered___celsius(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *b.borrow_mut() = Some(__tmp_0); *c.borrow_mut() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = __slices_order2_ordered___celsius(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *a.borrow_mut() = Some(__tmp_0); *b.borrow_mut() = Some(__tmp_1); };
    return (*b.borrow().as_ref().unwrap());
}

pub fn __slices_median_ordered__int(data: Rc<RefCell<Option<Vec<i64>>>>, mut a: Rc<RefCell<Option<i64>>>, mut b: Rc<RefCell<Option<i64>>>, mut c: Rc<RefCell<Option<i64>>>, swaps: Rc<RefCell<Option<i64>>>) -> i64 {
    { let (__tmp_0, __tmp_1) = __slices_order2_ordered__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *a.borrow_mut() = Some(__tmp_0); *b.borrow_mut() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = __slices_order2_ordered__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *b.borrow_mut() = Some(__tmp_0); *c.borrow_mut() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = __slices_order2_ordered__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *a.borrow_mut() = Some(__tmp_0); *b.borrow_mut() = Some(__tmp_1); };
    return (*b.borrow().as_ref().unwrap());
}

pub fn __slices_median_ordered__float64(data: Rc<RefCell<Option<Vec<f64>>>>, mut a: Rc<RefCell<Option<i64>>>, mut b: Rc<RefCell<Option<i64>>>, mut c: Rc<RefCell<Option<i64>>>, swaps: Rc<RefCell<Option<i64>>>) -> i64 {
    { let (__tmp_0, __tmp_1) = __slices_order2_ordered__float64(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *a.borrow_mut() = Some(__tmp_0); *b.borrow_mut() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = __slices_order2_ordered__float64(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *b.borrow_mut() = Some(__tmp_0); *c.borrow_mut() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = __slices_order2_ordered__float64(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *a.borrow_mut() = Some(__tmp_0); *b.borrow_mut() = Some(__tmp_1); };
    return (*b.borrow().as_ref().unwrap());
}

pub fn __slices_median_ordered__string(data: Rc<RefCell<Option<Vec<String>>>>, mut a: Rc<RefCell<Option<i64>>>, mut b: Rc<RefCell<Option<i64>>>, mut c: Rc<RefCell<Option<i64>>>, swaps: Rc<RefCell<Option<i64>>>) -> i64 {
    { let (__tmp_0, __tmp_1) = __slices_order2_ordered__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *a.borrow_mut() = Some(__tmp_0); *b.borrow_mut() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = __slices_order2_ordered__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *b.borrow_mut() = Some(__tmp_0); *c.borrow_mut() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = __slices_order2_ordered__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *a.borrow_mut() = Some(__tmp_0); *b.borrow_mut() = Some(__tmp_1); };
    return (*b.borrow().as_ref().unwrap());
}

pub fn __slices_swap_range_cmp_func___item(data: Rc<RefCell<Option<Vec<Item>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, n: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Item>>>, Rc<RefCell<Option<Item>>>) -> i64>>>>) {
    let mut i = Rc::new(RefCell::new(Some(0i64)));
    while (*i.borrow().as_ref().unwrap()) < (*n.borrow().as_ref().unwrap()) {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*a.borrow().as_ref().unwrap()) + (*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*b.borrow().as_ref().unwrap()) + (*i.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*b.borrow().as_ref().unwrap()) + (*i.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap()) + (*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

pub fn __slices_order2_cmp_func__ptr__task(data: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Task>>>>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, swaps: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<Task>>>, Rc<RefCell<Option<Task>>>) -> i64>>>>) -> (i64, i64) {
    if { let __f: Rc<dyn Fn(Rc<RefCell<Option<Task>>>, Rc<RefCell<Option<Task>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)((*data.borrow().as_ref().unwrap())[((*b.borrow().as_ref().unwrap())) as usize].clone(), (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone()) } < 0 {
        { let mut guard = swaps.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return ((*b.borrow().as_ref().unwrap()), (*a.borrow().as_ref().unwrap()));
    }
    ((*a.borrow().as_ref().unwrap()), (*b.borrow().as_ref().unwrap()))
}

pub fn __slices_order2_cmp_func__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, swaps: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>>>>) -> (i64, i64) {
    if { let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*b.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) } < 0 {
        { let mut guard = swaps.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return ((*b.borrow().as_ref().unwrap()), (*a.borrow().as_ref().unwrap()));
    }
    ((*a.borrow().as_ref().unwrap()), (*b.borrow().as_ref().unwrap()))
}

pub fn __slices_order2_ordered___celsius(data: Rc<RefCell<Option<Vec<Celsius>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, swaps: Rc<RefCell<Option<i64>>>) -> (i64, i64) {
    if __cmp__less___celsius(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*b.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) {
        { let mut guard = swaps.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return ((*b.borrow().as_ref().unwrap()), (*a.borrow().as_ref().unwrap()));
    }
    ((*a.borrow().as_ref().unwrap()), (*b.borrow().as_ref().unwrap()))
}

pub fn __slices_order2_ordered__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, swaps: Rc<RefCell<Option<i64>>>) -> (i64, i64) {
    if __cmp__less__int(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*b.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) {
        { let mut guard = swaps.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return ((*b.borrow().as_ref().unwrap()), (*a.borrow().as_ref().unwrap()));
    }
    ((*a.borrow().as_ref().unwrap()), (*b.borrow().as_ref().unwrap()))
}

pub fn __slices_order2_ordered__float64(data: Rc<RefCell<Option<Vec<f64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, swaps: Rc<RefCell<Option<i64>>>) -> (i64, i64) {
    if __cmp__less__float64(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*b.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) {
        { let mut guard = swaps.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return ((*b.borrow().as_ref().unwrap()), (*a.borrow().as_ref().unwrap()));
    }
    ((*a.borrow().as_ref().unwrap()), (*b.borrow().as_ref().unwrap()))
}

pub fn __slices_order2_ordered__string(data: Rc<RefCell<Option<Vec<String>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, swaps: Rc<RefCell<Option<i64>>>) -> (i64, i64) {
    if __cmp__less__string(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*b.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) {
        { let mut guard = swaps.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return ((*b.borrow().as_ref().unwrap()), (*a.borrow().as_ref().unwrap()));
    }
    ((*a.borrow().as_ref().unwrap()), (*b.borrow().as_ref().unwrap()))
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
