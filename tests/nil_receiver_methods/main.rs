#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

#[allow(unused_macros)]
macro_rules! print {
    ($($__arg:tt)*) => {{
        std::print!($($__arg)*);
        std::io::Write::flush(&mut std::io::stdout()).ok();
    }};
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
    go_panic_value_text(__payload)
}

fn go_panic_value_text(__payload: &dyn Any) -> String {
    // A boxed payload holds the Go value itself, so its string is printed
    // as it is.
    if let Some(__text) = __payload.downcast_ref::<String>() {
        return __text.clone();
    }
    if let Some(__text) = __payload.downcast_ref::<&'static str>() {
        return __text.to_string();
    }
    if let Some(__inner) = __payload.downcast_ref::<Box<dyn Any + Send + Sync>>() {
        let __text = go_panic_value_text(&**__inner);
        return __text;
    }
    if let Some(__inner) = __payload.downcast_ref::<Box<dyn Any + Send>>() {
        return go_panic_value_text(&**__inner);
    }
    if let Some(__inner) = __payload.downcast_ref::<Box<dyn Any>>() {
        return go_panic_value_text(&**__inner);
    }
    if let Some(__err) = __payload.downcast_ref::<std::sync::Arc<std::sync::Mutex<Option<std::sync::Arc<dyn std::error::Error + Send + Sync>>>>>() {
        return __err.lock().unwrap().as_ref().map(|__err| __err.to_string()).unwrap_or_else(|| String::from("nil"));
    }
    macro_rules! go_panic_display_text {
        ($($t:ty),*) => {
            $(if let Some(__value) = __payload.downcast_ref::<$t>() {
                return __value.to_string();
            })*
        };
    }
    go_panic_display_text!(i64, i32, i16, i8, u64, u32, u16, u8, usize, isize, bool, char);
    // Go prints a float in a panic as +1.500000e+000; a float32 is widened
    // to float64 first, as Go's print does.
    let __float = __payload
        .downcast_ref::<f64>()
        .copied()
        .or_else(|| __payload.downcast_ref::<f32>().map(|__value| *__value as f64));
    if let Some(__value) = __float {
        if __value.is_nan() {
            return String::from("NaN");
        }
        if __value.is_infinite() {
            return String::from(if __value > 0.0 { "+Inf" } else { "-Inf" });
        }
        let __text = format!("{:+.6e}", __value);
        if let Some((__mantissa, __exponent)) = __text.split_once('e') {
            let __exponent: i32 = __exponent.parse().unwrap_or(0);
            return format!("{}e{}{:03}", __mantissa, if __exponent < 0 { '-' } else { '+' }, __exponent.abs());
        }
        return __text;
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

struct GoWriteBack<F: FnMut()>(F);

impl<F: FnMut()> Drop for GoWriteBack<F> {
    fn drop(&mut self) {
        (self.0)()
    }
}

thread_local! {
    static __GO_RECEIVER_HANDLES: std::cell::RefCell<std::vec::Vec<(usize, Box<dyn std::any::Any>)>> = std::cell::RefCell::new(std::vec::Vec::new());
}

fn go_set_receiver_handle<H: Clone + 'static>(handle: &H, address: usize) {
    __GO_RECEIVER_HANDLES.with(|__handles| {
        let mut __handles = __handles.borrow_mut();
        if __handles.len() >= 16 {
            __handles.remove(0);
        }
        __handles.push((address, Box::new(handle.clone())));
    });
}

fn go_hint_receiver_mut<T: 'static>(handle: &std::sync::Arc<std::sync::Mutex<Option<T>>>) -> *mut T {
    let __ptr = handle.lock().unwrap().as_mut().unwrap() as *mut T;
    go_set_receiver_handle(handle, __ptr as *const () as usize);
    __ptr
}

fn go_hint_receiver<T: 'static>(handle: &std::sync::Arc<std::sync::Mutex<Option<T>>>) -> *const T {
    let __ptr = handle.lock().unwrap().as_ref().unwrap() as *const T;
    go_set_receiver_handle(handle, __ptr as *const () as usize);
    __ptr
}

fn go_receiver_ptr_mut<T>(handle: &std::sync::Arc<std::sync::Mutex<Option<T>>>) -> *mut T {
    handle.lock().unwrap().as_mut().unwrap() as *mut T
}

fn go_receiver_ptr<T>(handle: &std::sync::Arc<std::sync::Mutex<Option<T>>>) -> *const T {
    handle.lock().unwrap().as_ref().unwrap() as *const T
}

fn go_take_receiver_handle<T: 'static>(address: usize) -> Option<std::sync::Arc<std::sync::Mutex<Option<T>>>> {
    __GO_RECEIVER_HANDLES.with(|__handles| {
        let mut __handles = __handles.borrow_mut();
        let __at = __handles.iter().rposition(|(__address, __handle)| *__address == address && __handle.is::<std::sync::Arc<std::sync::Mutex<Option<T>>>>())?;
        let (_, __handle) = __handles.remove(__at);
        __handle.downcast::<std::sync::Arc<std::sync::Mutex<Option<T>>>>().ok().map(|__handle| *__handle)
    })
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
pub struct Node {
    pub value: Arc<Mutex<Option<i64>>>,
    pub next: Arc<Mutex<Option<Node>>>,
}

impl Node {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.value.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.next.clone();
        Self {
            value: __go_clone_0_0,
            next: __go_clone_1_0,
        }
    }
}


impl Default for Node {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        Self {
            value: __go_default_0_0,
            next: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.value.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { let __guard = self.next.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct Tree {
    pub left: Arc<Mutex<Option<Tree>>>,
    pub right: Arc<Mutex<Option<Tree>>>,
    pub val: Arc<Mutex<Option<i64>>>,
}

impl Tree {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.left.clone();
        let __go_clone_0_1 = self.right.clone();
        let __go_clone_1_0 = { let __guard = self.val.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            left: __go_clone_0_0,
            right: __go_clone_0_1,
            val: __go_clone_1_0,
        }
    }
}


impl Default for Tree {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_0_1 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            left: __go_default_0_0,
            right: __go_default_0_1,
            val: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.left.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", { let __guard = self.right.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_2 = format!("{}", (*self.val.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


#[derive(Debug, Clone)]
pub struct List {
    pub head: Arc<Mutex<Option<Elem>>>,
    pub size: Arc<Mutex<Option<i64>>>,
}

impl List {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.head.clone();
        let __go_clone_1_0 = { let __guard = self.size.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            head: __go_clone_0_0,
            size: __go_clone_1_0,
        }
    }
}


impl Default for List {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            head: __go_default_0_0,
            size: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.head.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", (*self.size.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct Elem {
    pub val: Arc<Mutex<Option<String>>>,
    pub next: Arc<Mutex<Option<Elem>>>,
}

impl Elem {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.val.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.next.clone();
        Self {
            val: __go_clone_0_0,
            next: __go_clone_1_0,
        }
    }
}


impl Default for Elem {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        Self {
            val: __go_default_0_0,
            next: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Elem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}
impl PartialEq for Elem {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.val.lock().unwrap().as_ref().cloned() }; let __right = { other.val.lock().unwrap().as_ref().cloned() }; __left == __right }
                && { let __left_some = self.next.lock().unwrap().is_some(); let __right_some = other.next.lock().unwrap().is_some(); (!__left_some && !__right_some) || (__left_some && __right_some && Arc::ptr_eq(&self.next, &other.next)) }
        )
    }
}


pub trait Stringer: std::fmt::Display + Any {
    fn __go_clone_box_stringer(&self) -> Box<dyn Stringer + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_print(&self, plus: bool) -> String { if plus { format!("{:#}", self) } else { format!("{}", self) } }
    fn __go_eq_stringer(&self, other: &(dyn Stringer + Send + Sync)) -> bool;
    fn string(&self) -> Arc<Mutex<Option<String>>>;
}

impl Clone for Box<dyn Stringer + Send + Sync> {
    fn clone(&self) -> Self {
        Stringer::__go_clone_box_stringer(self.as_ref())
    }
}

#[derive(Debug, Clone)]
pub struct Counter {
    pub hits: Arc<Mutex<Option<i64>>>,
}

impl Counter {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.hits.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            hits: __go_clone_0_0,
        }
    }
}


impl Default for Counter {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            hits: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.hits.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Node {
    pub fn len(&self) -> i64 {
        if false {
        return 0;
    }
        return { let __tmp_x = 1; let __tmp_y = __go_nil_recv_node_len({ let __field = self.next.clone(); __field }); __tmp_x + __tmp_y };
    }

    pub fn sum(&self) -> i64 {
        if false {
        return 0;
    }
        return { let __go_rhs_1 = Arc::new(Mutex::new(Some(__go_nil_recv_node_sum({ let __field = self.next.clone(); __field })))); let __go_binary_value = { let __tmp_x = (*self.value.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*__go_rhs_1.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; __go_binary_value };
    }
}

impl Tree {
    pub fn insert(&mut self, v: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<Tree>>> {
        let (mut t, __self_copy) = match go_take_receiver_handle::<Self>(self as *const Self as usize) {
            Some(__handle) => (__handle, None),
            None => {
                let __copy = Arc::new(Mutex::new(Some(self.clone())));
                (__copy.clone(), Some(__copy))
            }
        };
        let __self_ptr = self as *mut Self;
        let __self_write_back = GoWriteBack(move || if let Some(__copy) = &__self_copy { unsafe { *__self_ptr = __copy.lock().unwrap().as_ref().unwrap().clone(); } });
        if { let __nil_result = (*t.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some(Tree { val: Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
    }
        if { let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).val.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let new_val = __go_nil_recv_tree_insert({ let __field = (*t.lock().unwrap().as_ref().unwrap()).left.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); (*t.lock().unwrap().as_mut().unwrap()).left = new_val; };
    } else {
        { let new_val = __go_nil_recv_tree_insert({ let __field = (*t.lock().unwrap().as_ref().unwrap()).right.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); (*t.lock().unwrap().as_mut().unwrap()).right = new_val; };
    }
        return t.clone();
    }

    pub fn in_order(&self, visit: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<i64>>>) -> () + Send + Sync>>>>) {
        if false {
        return;
    }
        __go_nil_recv_tree_in_order({ let __field = self.left.clone(); __field }, Arc::new(Mutex::new(GoFuncValue::go_func_value(&(visit)))));
        {
            let __f: Arc<dyn Fn(Arc<Mutex<Option<i64>>>) -> () + Send + Sync> = visit.lock().unwrap().as_ref().unwrap().clone();
            (__f)(
                Arc::new(Mutex::new(Some({ let __selector_holder = self.val.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned })))
            )
        };
        __go_nil_recv_tree_in_order({ let __field = self.right.clone(); __field }, Arc::new(Mutex::new(GoFuncValue::go_func_value(&(visit)))));
    }

    pub fn height(&self) -> i64 {
        if false {
        return 0;
    }
        let (mut l, mut r) = (__go_nil_recv_tree_height({ let __field = self.left.clone(); __field }), __go_nil_recv_tree_height({ let __field = self.right.clone(); __field }));
        if { let __tmp_x = l; let __tmp_y = r; __tmp_x > __tmp_y } {
        return { let __tmp_x = l; let __tmp_y = 1; __tmp_x + __tmp_y };
    }
        return { let __tmp_x = r; let __tmp_y = 1; __tmp_x + __tmp_y };
    }

    pub fn contains(&mut self, v: Arc<Mutex<Option<i64>>>) -> bool {
        let (mut t, __self_copy) = match go_take_receiver_handle::<Self>(self as *const Self as usize) {
            Some(__handle) => (__handle, None),
            None => {
                let __copy = Arc::new(Mutex::new(Some(self.clone())));
                (__copy.clone(), Some(__copy))
            }
        };
        let __self_ptr = self as *mut Self;
        let __self_write_back = GoWriteBack(move || if let Some(__copy) = &__self_copy { unsafe { *__self_ptr = __copy.lock().unwrap().as_ref().unwrap().clone(); } });
        while { let __nil_result = (*t.lock().unwrap()).is_some(); __nil_result } {
        if { let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).val.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } {
        return true;
    }
        if { let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).val.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let new_val = (*t.lock().unwrap().as_ref().unwrap()).left.clone(); t = new_val; };
    } else {
        { let new_val = (*t.lock().unwrap().as_ref().unwrap()).right.clone(); t = new_val; };
    }
    }
        false
    }
}

impl Elem {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        if false {
        return Arc::new(Mutex::new(Some("nil".to_string())));
    }
        return Arc::new(Mutex::new(Some({ let __go_rhs_2 = __go_nil_recv_elem_string({ let __field = self.next.clone(); __field }); let __go_binary_value = {
            let mut __s = String::new();
            __s.push_str(&format!("{}", (*self.val.clone().lock().unwrap().as_ref().unwrap())));
            __s.push_str(&format!("{}", "->".to_string()));
            __s.push_str(&format!("{}", { let __v = (*__go_rhs_2.lock().unwrap().as_ref().unwrap()).clone(); __v }));
            __s
        }; __go_binary_value })));
    }

    pub fn append(&mut self, v: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Elem>>> {
        let (mut e, __self_copy) = match go_take_receiver_handle::<Self>(self as *const Self as usize) {
            Some(__handle) => (__handle, None),
            None => {
                let __copy = Arc::new(Mutex::new(Some(self.clone())));
                (__copy.clone(), Some(__copy))
            }
        };
        let __self_ptr = self as *mut Self;
        let __self_write_back = GoWriteBack(move || if let Some(__copy) = &__self_copy { unsafe { *__self_ptr = __copy.lock().unwrap().as_ref().unwrap().clone(); } });
        if { let __nil_result = (*e.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some(Elem { val: Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
    }
        { let new_val = __go_nil_recv_elem_append({ let __field = (*e.lock().unwrap().as_ref().unwrap()).next.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); (*e.lock().unwrap().as_mut().unwrap()).next = new_val; };
        return e.clone();
    }
}

impl Stringer for Elem {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        Elem::string(self)
    }
    fn __go_clone_box_stringer(&self) -> Box<dyn Stringer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stringer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_stringer(&self, other: &(dyn Stringer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Elem>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct ElemPtr(pub Arc<Mutex<Option<Elem>>>);

impl std::fmt::Display for ElemPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Stringer for ElemPtr {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Elem::string(__recv)
    }
    fn __go_clone_box_stringer(&self) -> Box<dyn Stringer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stringer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => format!("{}", __v), None => "<nil>".to_string() }
    }
    fn __go_eq_stringer(&self, other: &(dyn Stringer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ElemPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl List {
    pub fn push(&mut self, v: Arc<Mutex<Option<String>>>) {
        { let new_val = __go_nil_recv_elem_append({ let __field = self.head.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); self.head = new_val; };
        { let __target = self.size.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

impl Counter {
    pub fn hit(&mut self) -> i64 {
        if false {
        return -(1);
    }
        { let __target = self.hits.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return (*self.hits.lock().unwrap().as_ref().unwrap());
    }
}

pub fn describe(s: Arc<Mutex<Option<Box<dyn Stringer + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
    return Arc::new(Mutex::new(Some({
        let mut __s = String::new();
        __s.push_str(&format!("{}", "[".to_string()));
        __s.push_str(&format!("{}", (*(*s.lock().unwrap().as_ref().unwrap()).string().lock().unwrap().as_ref().unwrap())));
        __s.push_str(&format!("{}", "]".to_string()));
        __s
    })));
}

pub fn more() {
    let mut l: Arc<Mutex<Option<List>>> = Arc::new(Mutex::new(Some(Default::default())));
    { let __range_holder = strings::fields(Arc::new(Mutex::new(Some("a b c".to_string())))).clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for w in __range_values.iter() {
        (*l.lock().unwrap().as_mut().unwrap()).push(Arc::new(Mutex::new(Some((*w).clone()))));
    } }
    println!("{} {}", format!("{}", (*__go_nil_recv_elem_string({ let __field = (*l.lock().unwrap().as_ref().unwrap()).head.clone(); __field }).lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __field = (*l.lock().unwrap().as_ref().unwrap()).size.clone(); __field }.lock().unwrap().as_ref().unwrap())));
    let mut none: Arc<Mutex<Option<Elem>>> = Arc::new(Mutex::new(None));
    println!("{} {}", format!("{}", (*__go_nil_recv_elem_string(none.clone()).lock().unwrap().as_ref().unwrap())), format!("{}", (*describe(Arc::new(Mutex::new(Some({ let __arg_value = Box::new(ElemPtr((*l.lock().unwrap().as_ref().unwrap()).head.clone())) as Box<dyn Stringer + Send + Sync>; __arg_value })))).lock().unwrap().as_ref().unwrap())));
    let mut c = Arc::new(Mutex::new(Some(Counter { hits: Arc::new(Mutex::new(Some(0))) })));
    __go_nil_recv_counter_hit(c.clone());
    __go_nil_recv_counter_hit(c.clone());
    let mut missing: Arc<Mutex<Option<Counter>>> = Arc::new(Mutex::new(None));
    { let __go_print_arg_1 = __go_nil_recv_counter_hit(c.clone()); let __go_print_arg_2 = __go_nil_recv_counter_hit(missing.clone()); let __go_print_value = println!("{} {} {}", format!("{}", __go_print_arg_1), format!("{}", (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).hits.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{}", __go_print_arg_2)); __go_print_value };
    let mut cs = Arc::new(Mutex::new(Some(vec![c.clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Counter { ..Default::default() })))])));
    { let __range_holder = cs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for x in __range_values.iter() {
        print!("{}{}", format!("{}", __go_nil_recv_counter_hit(x.clone())), format!("{}", " ".to_string()));
    } }
    println!();
    let mut byName = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Counter>>>>::from([("a".to_string(), c.clone())]))));
    println!("{} {}", format!("{}", __go_nil_recv_counter_hit({ let __map = { let __map_holder = byName.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) })), format!("{}", __go_nil_recv_counter_hit({ let __map = { let __map_holder = byName.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"zz".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) })));
    let mut done = GoChannel::<i64>::new();
    let c_thread = c.clone(); let done_thread = done.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        done_thread.send(__go_nil_recv_counter_hit(c_thread.clone()));;;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    println!("{}", format!("{}", done.recv().unwrap_or_default()));
}

fn main() {
    go_install_panic_exit();
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut list = Arc::new(Mutex::new(Some(Node {
        value: Arc::new(Mutex::new(Some(1))),
        next: Arc::new(Mutex::new(Some(Node {
            value: Arc::new(Mutex::new(Some(2))),
            next: Arc::new(Mutex::new(Some(Node { value: Arc::new(Mutex::new(Some(3))), next: Default::default(), ..Default::default() }))).clone(),
            ..Default::default()
        }))).clone(),
        ..Default::default()
    })));
    println!("{} {}", format!("{}", __go_nil_recv_node_len(list.clone())), format!("{}", __go_nil_recv_node_sum(list.clone())));
    let mut empty: Arc<Mutex<Option<Node>>> = Arc::new(Mutex::new(None));
    println!("{} {}", format!("{}", __go_nil_recv_node_len(empty.clone())), format!("{}", __go_nil_recv_node_sum(empty.clone())));
    let mut root: Arc<Mutex<Option<Tree>>> = Arc::new(Mutex::new(None));
    for v in vec![5i64, 3i64, 8i64, 1i64, 4i64, 9i64].iter().copied() {
        { let new_val = __go_nil_recv_tree_insert(root.clone(), Arc::new(Mutex::new(Some(v.clone())))).clone(); root = new_val; };
    }
    let mut out: Arc<Mutex<Option<Vec<i64>>>> = Arc::new(Mutex::new(None));
    let mut out_closure_clone = out.clone(); __go_nil_recv_tree_in_order(root.clone(), Arc::new(Mutex::new(Some(Arc::new(move |v: Arc<Mutex<Option<i64>>>| {
        let mut out_closure_clone = out_closure_clone.clone();
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = out_closure_clone.clone(); let __append_value = ({ let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }); let mut __appended: Vec<i64> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *out_closure_clone.lock().unwrap() = __slice_header; };
    }) as Arc<dyn Fn(Arc<Mutex<Option<i64>>>) -> () + Send + Sync>))));
    { let __go_print_arg_3 = __go_nil_recv_tree_height(root.clone()); let __go_print_arg_4 = __go_nil_recv_tree_contains(root.clone(), Arc::new(Mutex::new(Some(4)))); let __go_print_arg_5 = __go_nil_recv_tree_contains(root.clone(), Arc::new(Mutex::new(Some(7)))); let __go_print_value = println!("{} {} {} {}", format!("{}", format_slice(&out)), format!("{}", __go_print_arg_3), format!("{}", __go_print_arg_4), format!("{}", __go_print_arg_5)); __go_print_value };
    more();
}

pub fn __go_nil_recv_counter_hit(c: Arc<Mutex<Option<Counter>>>) -> i64 {
    if { let __nil_result = (*c.lock().unwrap()).is_none(); __nil_result } {
        return -(1);
    }
    { let __target = (*c.lock().unwrap().as_ref().unwrap()).hits.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    return (*(*c.lock().unwrap().as_ref().unwrap()).hits.lock().unwrap().as_ref().unwrap());
}

pub fn __go_nil_recv_elem_append(e: Arc<Mutex<Option<Elem>>>, v: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Elem>>> {
    if { let __nil_result = (*e.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some(Elem { val: Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
    }
    { let new_val = __go_nil_recv_elem_append({ let __field = (*e.lock().unwrap().as_ref().unwrap()).next.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); (*e.lock().unwrap().as_mut().unwrap()).next = new_val; };
    e.clone()
}

pub fn __go_nil_recv_elem_string(e: Arc<Mutex<Option<Elem>>>) -> Arc<Mutex<Option<String>>> {
    if { let __nil_result = (*e.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some("nil".to_string())));
    }
    return Arc::new(Mutex::new(Some({ let __go_rhs_3 = __go_nil_recv_elem_string({ let __field = (*e.lock().unwrap().as_ref().unwrap()).next.clone(); __field }); let __go_binary_value = {
        let mut __s = String::new();
        __s.push_str(&format!("{}", (*{ let __field = (*e.lock().unwrap().as_ref().unwrap()).val.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()));
        __s.push_str(&format!("{}", "->".to_string()));
        __s.push_str(&format!("{}", { let __v = (*__go_rhs_3.lock().unwrap().as_ref().unwrap()).clone(); __v }));
        __s
    }; __go_binary_value })));
}

pub fn __go_nil_recv_node_len(n: Arc<Mutex<Option<Node>>>) -> i64 {
    if { let __nil_result = (*n.lock().unwrap()).is_none(); __nil_result } {
        return 0;
    }
    return { let __tmp_x = 1; let __tmp_y = __go_nil_recv_node_len({ let __field = (*n.lock().unwrap().as_ref().unwrap()).next.clone(); __field }); __tmp_x + __tmp_y };
}

pub fn __go_nil_recv_node_sum(n: Arc<Mutex<Option<Node>>>) -> i64 {
    if { let __nil_result = (*n.lock().unwrap()).is_none(); __nil_result } {
        return 0;
    }
    return { let __go_rhs_4 = Arc::new(Mutex::new(Some(__go_nil_recv_node_sum({ let __field = (*n.lock().unwrap().as_ref().unwrap()).next.clone(); __field })))); let __go_binary_value = { let __tmp_x = (*{ let __field = (*n.lock().unwrap().as_ref().unwrap()).value.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*__go_rhs_4.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; __go_binary_value };
}

pub fn __go_nil_recv_tree_contains(mut t: Arc<Mutex<Option<Tree>>>, v: Arc<Mutex<Option<i64>>>) -> bool {
    while { let __nil_result = (*t.lock().unwrap()).is_some(); __nil_result } {
        if { let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).val.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } {
        return true;
    }
        if { let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).val.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let new_val = (*t.lock().unwrap().as_ref().unwrap()).left.clone(); t = new_val; };
    } else {
        { let new_val = (*t.lock().unwrap().as_ref().unwrap()).right.clone(); t = new_val; };
    }
    }
    false
}

pub fn __go_nil_recv_tree_height(t: Arc<Mutex<Option<Tree>>>) -> i64 {
    if { let __nil_result = (*t.lock().unwrap()).is_none(); __nil_result } {
        return 0;
    }
    let (mut l, mut r) = ({ let __tuple_value = __go_nil_recv_tree_height({ let __field = (*t.lock().unwrap().as_ref().unwrap()).left.clone(); __field }); __tuple_value }, { let __tuple_value = __go_nil_recv_tree_height({ let __field = (*t.lock().unwrap().as_ref().unwrap()).right.clone(); __field }); __tuple_value });
    if { let __tmp_x = l; let __tmp_y = r; __tmp_x > __tmp_y } {
        return { let __tmp_x = l; let __tmp_y = 1; __tmp_x + __tmp_y };
    }
    return { let __tmp_x = r; let __tmp_y = 1; __tmp_x + __tmp_y };
}

pub fn __go_nil_recv_tree_in_order(t: Arc<Mutex<Option<Tree>>>, visit: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<i64>>>) -> () + Send + Sync>>>>) {
    if { let __nil_result = (*t.lock().unwrap()).is_none(); __nil_result } {
        return;
    }
    __go_nil_recv_tree_in_order({ let __field = (*t.lock().unwrap().as_ref().unwrap()).left.clone(); __field }, Arc::new(Mutex::new(GoFuncValue::go_func_value(&(visit)))));
    {
        let __f: Arc<dyn Fn(Arc<Mutex<Option<i64>>>) -> () + Send + Sync> = visit.lock().unwrap().as_ref().unwrap().clone();
        (__f)(
            Arc::new(Mutex::new(Some({ let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).val.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned })))
        )
    };
    __go_nil_recv_tree_in_order({ let __field = (*t.lock().unwrap().as_ref().unwrap()).right.clone(); __field }, Arc::new(Mutex::new(GoFuncValue::go_func_value(&(visit)))));
}

pub fn __go_nil_recv_tree_insert(t: Arc<Mutex<Option<Tree>>>, v: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<Tree>>> {
    if { let __nil_result = (*t.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some(Tree { val: Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
    }
    if { let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).val.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let new_val = __go_nil_recv_tree_insert({ let __field = (*t.lock().unwrap().as_ref().unwrap()).left.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); (*t.lock().unwrap().as_mut().unwrap()).left = new_val; };
    } else {
        { let new_val = __go_nil_recv_tree_insert({ let __field = (*t.lock().unwrap().as_ref().unwrap()).right.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); (*t.lock().unwrap().as_mut().unwrap()).right = new_val; };
    }
    t.clone()
}

impl GoValueClone for Node {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Tree {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for List {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Elem {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Counter {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
