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

pub trait Shape: std::fmt::Display + Any {
    fn __go_clone_box_shape(&self) -> Box<dyn Shape + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_print(&self, plus: bool) -> String { if plus { format!("{:#}", self) } else { format!("{}", self) } }
    fn __go_eq_shape(&self, other: &(dyn Shape + Send + Sync)) -> bool;
    fn area(&self) -> f64;
}

impl Clone for Box<dyn Shape + Send + Sync> {
    fn clone(&self) -> Self {
        Shape::__go_clone_box_shape(self.as_ref())
    }
}

pub trait Named: std::fmt::Display + Any {
    fn __go_clone_box_named(&self) -> Box<dyn Named + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_print(&self, plus: bool) -> String { if plus { format!("{:#}", self) } else { format!("{}", self) } }
    fn __go_eq_named(&self, other: &(dyn Named + Send + Sync)) -> bool;
    fn name(&self) -> Arc<Mutex<Option<String>>>;
}

impl Clone for Box<dyn Named + Send + Sync> {
    fn clone(&self) -> Self {
        Named::__go_clone_box_named(self.as_ref())
    }
}

/// NamedShape embeds two interfaces. Each of them can recover the concrete
/// value behind it, so the combined interface has that ability twice.
pub trait NamedShape: Shape + Named + std::fmt::Display + Any {
    fn __go_clone_box_named_shape(&self) -> Box<dyn NamedShape + Send + Sync>;
    fn __go_eq_named_shape(&self, other: &(dyn NamedShape + Send + Sync)) -> bool;
}

impl Clone for Box<dyn NamedShape + Send + Sync> {
    fn clone(&self) -> Self {
        NamedShape::__go_clone_box_named_shape(self.as_ref())
    }
}

impl Shape for Box<dyn NamedShape + Send + Sync> {
    fn __go_clone_box_shape(&self) -> Box<dyn Shape + Send + Sync> {
        Box::new((*self).clone()) as Box<dyn Shape + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        Shape::__go_as_any(&**self)
    }
    fn __go_eq_shape(&self, other: &(dyn Shape + Send + Sync)) -> bool {
        (**self).__go_eq_shape(other)
    }
    fn area(&self) -> f64 {
        (**self).area()
    }
}

impl Named for Box<dyn NamedShape + Send + Sync> {
    fn __go_clone_box_named(&self) -> Box<dyn Named + Send + Sync> {
        Box::new((*self).clone()) as Box<dyn Named + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        Shape::__go_as_any(&**self)
    }
    fn __go_eq_named(&self, other: &(dyn Named + Send + Sync)) -> bool {
        (**self).__go_eq_named(other)
    }
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        (**self).name()
    }
}

#[derive(Debug, Clone)]
pub struct Rect {
    pub w: Arc<Mutex<Option<f64>>>,
    pub h: Arc<Mutex<Option<f64>>>,
}

impl Rect {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.w.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.h.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            w: __go_clone_0_0,
            h: __go_clone_0_1,
        }
    }
}


impl Default for Rect {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0.0)));
        let __go_default_0_1 = Arc::new(Mutex::new(Some(0.0)));
        Self {
            w: __go_default_0_0,
            h: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = go_float_string(((*self.w.lock().unwrap().as_ref().unwrap())) as f64, false);
        let __go_fmt_1 = go_float_string(((*self.h.lock().unwrap().as_ref().unwrap())) as f64, false);
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}
impl PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.w.lock().unwrap().as_ref().cloned() }; let __right = { other.w.lock().unwrap().as_ref().cloned() }; __left == __right }
                && { let __left = { self.h.lock().unwrap().as_ref().cloned() }; let __right = { other.h.lock().unwrap().as_ref().cloned() }; __left == __right }
        )
    }
}


#[derive(Debug, Clone)]
pub struct Square {
    pub s: Arc<Mutex<Option<f64>>>,
}

impl Square {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.s.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            s: __go_clone_0_0,
        }
    }
}


impl Default for Square {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0.0)));
        Self {
            s: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = go_float_string(((*self.s.lock().unwrap().as_ref().unwrap())) as f64, false);
        write!(f, "{{{}}}", __go_fmt_0)
    }
}
impl PartialEq for Square {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.s.lock().unwrap().as_ref().cloned() }; let __right = { other.s.lock().unwrap().as_ref().cloned() }; __left == __right }
        )
    }
}


impl Rect {
    pub fn area(&self) -> f64 {
        return { let __tmp_x = (*self.w.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.h.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y };
    }

    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some("rect".to_string())))
    }
}

impl Named for Rect {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        Rect::name(self)
    }
    fn __go_clone_box_named(&self) -> Box<dyn Named + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Named + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_named(&self, other: &(dyn Named + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Rect>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct RectPtr(pub Arc<Mutex<Option<Rect>>>);

impl std::fmt::Display for RectPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Named for RectPtr {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Rect::name(__recv)
    }
    fn __go_clone_box_named(&self) -> Box<dyn Named + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Named + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => if plus { format!("&{:#}", __v) } else { format!("&{}", __v) }, None => "<nil>".to_string() }
    }
    fn __go_eq_named(&self, other: &(dyn Named + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<RectPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl NamedShape for Rect {
    fn __go_clone_box_named_shape(&self) -> Box<dyn NamedShape + Send + Sync> {
        Box::new(self.clone()) as Box<dyn NamedShape + Send + Sync>
    }
    fn __go_eq_named_shape(&self, other: &(dyn NamedShape + Send + Sync)) -> bool {
        if let Some(__other) = Shape::__go_as_any(other).downcast_ref::<Rect>() {
            self == __other
        } else {
            false
        }
    }
}

impl NamedShape for RectPtr {
    fn __go_clone_box_named_shape(&self) -> Box<dyn NamedShape + Send + Sync> {
        Box::new(self.clone()) as Box<dyn NamedShape + Send + Sync>
    }
    fn __go_eq_named_shape(&self, other: &(dyn NamedShape + Send + Sync)) -> bool {
        if let Some(__other) = Shape::__go_as_any(other).downcast_ref::<RectPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Shape for Rect {
    fn area(&self) -> f64 {
        Rect::area(self)
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Shape + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_shape(&self, other: &(dyn Shape + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Rect>() {
            self == __other
        } else {
            false
        }
    }
}

impl Shape for RectPtr {
    fn area(&self) -> f64 {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Rect::area(__recv)
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Shape + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => if plus { format!("&{:#}", __v) } else { format!("&{}", __v) }, None => "<nil>".to_string() }
    }
    fn __go_eq_shape(&self, other: &(dyn Shape + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<RectPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Square {
    pub fn area(&self) -> f64 {
        return { let __tmp_x = (*self.s.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.s.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y };
    }

    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some("square".to_string())))
    }
}

impl Named for Square {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        Square::name(self)
    }
    fn __go_clone_box_named(&self) -> Box<dyn Named + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Named + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_named(&self, other: &(dyn Named + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Square>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct SquarePtr(pub Arc<Mutex<Option<Square>>>);

impl std::fmt::Display for SquarePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Named for SquarePtr {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Square::name(__recv)
    }
    fn __go_clone_box_named(&self) -> Box<dyn Named + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Named + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => if plus { format!("&{:#}", __v) } else { format!("&{}", __v) }, None => "<nil>".to_string() }
    }
    fn __go_eq_named(&self, other: &(dyn Named + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SquarePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl NamedShape for Square {
    fn __go_clone_box_named_shape(&self) -> Box<dyn NamedShape + Send + Sync> {
        Box::new(self.clone()) as Box<dyn NamedShape + Send + Sync>
    }
    fn __go_eq_named_shape(&self, other: &(dyn NamedShape + Send + Sync)) -> bool {
        if let Some(__other) = Shape::__go_as_any(other).downcast_ref::<Square>() {
            self == __other
        } else {
            false
        }
    }
}

impl NamedShape for SquarePtr {
    fn __go_clone_box_named_shape(&self) -> Box<dyn NamedShape + Send + Sync> {
        Box::new(self.clone()) as Box<dyn NamedShape + Send + Sync>
    }
    fn __go_eq_named_shape(&self, other: &(dyn NamedShape + Send + Sync)) -> bool {
        if let Some(__other) = Shape::__go_as_any(other).downcast_ref::<SquarePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        Square::area(self)
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Shape + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_shape(&self, other: &(dyn Shape + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Square>() {
            self == __other
        } else {
            false
        }
    }
}

impl Shape for SquarePtr {
    fn area(&self) -> f64 {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Square::area(__recv)
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Shape + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => if plus { format!("&{:#}", __v) } else { format!("&{}", __v) }, None => "<nil>".to_string() }
    }
    fn __go_eq_shape(&self, other: &(dyn Shape + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SquarePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn total_area(shapes: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Shape + Send + Sync>>>>>>>>) -> f64 {
    let mut t = Arc::new(Mutex::new(Some(0.0)));
    { let __range_holder = shapes.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for s in __range_values.iter() {
        { let __rhs = (*s.lock().unwrap().as_ref().unwrap()).area(); let mut guard = t.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }
    return { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn describe(n: Arc<Mutex<Option<Box<dyn Named + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
    return Arc::new(Mutex::new(Some({
        let mut __s = String::new();
        __s.push_str(&format!("{}", "<".to_string()));
        __s.push_str(&format!("{}", (*(*n.lock().unwrap().as_ref().unwrap()).name().lock().unwrap().as_ref().unwrap())));
        __s.push_str(&format!("{}", ">".to_string()));
        __s
    })));
}

fn main() {
    go_install_panic_exit();
    ::math_bits::__go_init_all();
    ::sort::__go_init_all();

    let mut started = GoChannel::<bool>::new();
    let started_thread = started.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        started_thread.send(true);;;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    started.recv().unwrap_or_default();
    let mut items = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Box::new(Rect { w: Arc::new(Mutex::new(Some(3.0))), h: Arc::new(Mutex::new(Some(4.0))), ..Default::default() }) as Box<dyn NamedShape + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(SquarePtr(Arc::new(Mutex::new(Some(Square { s: Arc::new(Mutex::new(Some(2.0))), ..Default::default() }))).clone())) as Box<dyn NamedShape + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(Rect { w: Arc::new(Mutex::new(Some(1.0))), h: Arc::new(Mutex::new(Some(1.0))), ..Default::default() }) as Box<dyn NamedShape + Send + Sync>)))])));
    let items_closure_clone = items.clone(); { let __sort_target = items_closure_clone.clone(); let __sort_less = Arc::new(Mutex::new(Some(Arc::new(move |i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>| -> bool {
        return {
            let __tmp_x = {
                let __recv = {
                    let __seq = { let __seq_holder = items_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned };
                    __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone()
                };
                let __result = (*__recv.lock().unwrap().as_ref().unwrap()).area();
                __result
            };
            let __tmp_y = {
                let __recv = {
                    let __seq = { let __seq_holder = items_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned };
                    __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone()
                };
                let __result = (*__recv.lock().unwrap().as_ref().unwrap()).area();
                __result
            };
            __tmp_x < __tmp_y
        };
    }) as Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> bool + Send + Sync>))); let __sort_len = { let __sort_guard = __sort_target.lock().unwrap(); __sort_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; for __sort_i in 1..__sort_len { let mut __sort_j = __sort_i; while __sort_j > 0 { let __should_swap = { let mut __less_guard = __sort_less.lock().unwrap(); let __less = __less_guard.as_mut().expect("sort.Slice less function is nil"); __less(Arc::new(Mutex::new(Some(__sort_j as i64))), Arc::new(Mutex::new(Some((__sort_j - 1) as i64)))) }; if !__should_swap { break; } { let mut __sort_guard = __sort_target.lock().unwrap(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.swap(__sort_j, __sort_j - 1); } } __sort_j -= 1; } } };
    let mut plain: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Shape + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));
    { let __range_holder = items.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for it in __range_values.iter() {
        println!("{} {} {}", format!("{}", (*(*it.lock().unwrap().as_ref().unwrap()).name().lock().unwrap().as_ref().unwrap())), { let __text = go_float_string(((*it.lock().unwrap().as_ref().unwrap()).area()) as f64, false); __text }, format!("{}", (*describe({ let __inner: Option<Box<dyn Named + Send + Sync>> = (*it.lock().unwrap()).clone().map(|__v| -> Box<dyn Named + Send + Sync> { __v }); Arc::new(Mutex::new(__inner)) }).lock().unwrap().as_ref().unwrap())));
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = plain.clone(); let __append_value = { let __inner: Option<Box<dyn Shape + Send + Sync>> = (*it.lock().unwrap()).clone().map(|__v| -> Box<dyn Shape + Send + Sync> { __v }); Arc::new(Mutex::new(__inner)) }; let mut __appended: Vec<Arc<Mutex<Option<Box<dyn Shape + Send + Sync>>>>> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *plain.lock().unwrap() = __slice_header; };
    } }
    println!("{}", { let __text = go_float_string((total_area(plain.clone())) as f64, false); __text });
    { let __range_holder = items.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for it in __range_values.iter() {
        {
    let _ts_subject = it.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    std::mem::drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = Shape::__go_as_any(__v.as_ref());
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn NamedShape + Send + Sync>>() {
            Shape::__go_as_any(__boxed.as_ref())
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<Rect>()).is_some() {
        let v = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<Rect>()).unwrap().__go_value_clone())));
        println!("{} {} {}", format!("{}", "rect".to_string()), { let __text = go_float_string(((*{ let __field = (*v.lock().unwrap().as_ref().unwrap()).w.clone(); __field }.lock().unwrap().as_ref().unwrap())) as f64, false); __text }, { let __text = go_float_string(((*{ let __field = (*v.lock().unwrap().as_ref().unwrap()).h.clone(); __field }.lock().unwrap().as_ref().unwrap())) as f64, false); __text });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<SquarePtr>()).is_some() {
        let v = _ts_val.and_then(|__v| __v.downcast_ref::<SquarePtr>()).unwrap().0.clone();
        println!("{} {}", format!("{}", "square".to_string()), { let __text = go_float_string(((*{ let __field = (*v.lock().unwrap().as_ref().unwrap()).s.clone(); __field }.lock().unwrap().as_ref().unwrap())) as f64, false); __text });;
    }
    }
        {
        let (mut r, mut ok) = ({
        let val = it.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = Shape::__go_as_any(any_val.as_ref()).downcast_ref::<Rect>() {
            (Arc::new(Mutex::new(Some(typed_val.__go_value_clone()))), true)
            } else {
                (Arc::new(Mutex::new(Some(Default::default()))), false)
            }
        } else {
            (Arc::new(Mutex::new(Some(Default::default()))), false)
        }
    });;
        if ok {
            println!("{} {}", format!("{}", "is rect".to_string()), { let __text = go_float_string(((*r.lock().unwrap().as_ref().unwrap()).area()) as f64, false); __text });;
        }
    }
    } }
    let mut a: Arc<Mutex<Option<Box<dyn NamedShape + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(Rect { w: Arc::new(Mutex::new(Some(1.0))), h: Arc::new(Mutex::new(Some(2.0))), ..Default::default() }) as Box<dyn NamedShape + Send + Sync>)));let mut b: Arc<Mutex<Option<Box<dyn NamedShape + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(Rect { w: Arc::new(Mutex::new(Some(1.0))), h: Arc::new(Mutex::new(Some(2.0))), ..Default::default() }) as Box<dyn NamedShape + Send + Sync>)));
    println!("{} {}", format!("{}", { let __left_holder = a.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn NamedShape + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = b.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn NamedShape + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_named_shape(__right), _ => false }; __eq }), format!("{}", { let __left_holder = a.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn NamedShape + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = { let __seq = { let __seq_holder = items.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn NamedShape + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_named_shape(__right), _ => false }; __eq }));
    let mut same = { let __iface_value = a.lock().unwrap().clone(); Arc::new(Mutex::new(__iface_value)) };
    println!("{}", format!("{}", { let __left_holder = same.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn NamedShape + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = a.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn NamedShape + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_named_shape(__right), _ => false }; __eq }));
    let mut sq = Arc::new(Mutex::new(Some(Square { s: Arc::new(Mutex::new(Some(5.0))), ..Default::default() })));
    let mut c: Arc<Mutex<Option<Box<dyn NamedShape + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(SquarePtr(sq.clone())) as Box<dyn NamedShape + Send + Sync>)));let mut d: Arc<Mutex<Option<Box<dyn NamedShape + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(SquarePtr(sq.clone())) as Box<dyn NamedShape + Send + Sync>)));
    println!("{} {}", format!("{}", { let __left_holder = c.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn NamedShape + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = d.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn NamedShape + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_named_shape(__right), _ => false }; __eq }), { let __text = go_float_string(((*(*({
        let val = c.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = Shape::__go_as_any(any_val.as_ref()).downcast_ref::<SquarePtr>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).lock().unwrap().as_ref().unwrap()).s.lock().unwrap().as_ref().unwrap())) as f64, false); __text });
    let mut none: Arc<Mutex<Option<Box<dyn NamedShape + Send + Sync>>>> = Arc::new(Mutex::new(None));
    let mut withNil = { let __append_target = Arc::new(Mutex::new(Some(Vec::<Arc<Mutex<Option<Box<dyn Shape + Send + Sync>>>>>::new()))).clone(); let __append_value = { let __inner: Option<Box<dyn Shape + Send + Sync>> = (*none.lock().unwrap()).clone().map(|__v| -> Box<dyn Shape + Send + Sync> { __v }); Arc::new(Mutex::new(__inner)) }; let mut __appended: Vec<Arc<Mutex<Option<Box<dyn Shape + Send + Sync>>>>> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) };
    println!("{} {}", format!("{}", ({ let __len = (*withNil.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", { let __nil_result = (*{ let __seq = { let __seq_holder = withNil.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap()).is_none(); __nil_result }));
    let mut byName = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Box<dyn Shape + Send + Sync>>>>>::from([]))));
    { let __range_holder = items.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for it in __range_values.iter() {
        { let __map_key = { let __map_key_holder = (*it.lock().unwrap().as_ref().unwrap()).name().clone(); let __map_key_guard = __map_key_holder.lock().unwrap(); let __cloned = (*__map_key_guard.as_ref().unwrap()).clone(); std::mem::drop(__map_key_guard); __cloned }; let __map_value = { let __inner: Option<Box<dyn Shape + Send + Sync>> = (*it.lock().unwrap()).clone().map(|__v| -> Box<dyn Shape + Send + Sync> { __v }); Arc::new(Mutex::new(__inner)) }; (*byName.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    } }
    println!("{} {}", format!("{}", ({ let __len = (*byName.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), { let __text = go_float_string(({ let __recv = { let __map = { let __map_holder = byName.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"square".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).area(); __result }) as f64, false); __text });
}

impl GoValueClone for Rect {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Square {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
