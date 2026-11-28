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


fn format_slice_wrapped_stringer<T, C>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[Arc<Mutex<Option<T>>>]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        format_slice_wrapped_stringer_values(s.as_ref())
    } else {
        "[]".to_string()
    }
}

fn format_slice_wrapped_stringer_values<T>(slice: &[Arc<Mutex<Option<T>>>]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| {
        let inner = v.lock().unwrap();
        match inner.as_ref() {
            Some(value) => value.to_string(),
            None => "<nil>".to_string(),
        }
    }).collect();
    format!("[{}]", formatted.join(" "))
}

/// Runs an append, __store, for elements whose Go copy is not their Clone,
/// such as a struct of cells. When the append moves the slice to a new
/// backing array, the elements it carried over are replaced with __copy of
/// them, so the old array and the new one share nothing.
trait GoSliceAppendCopying<T> {
    fn go_append_copying(&mut self, __store: impl FnOnce(&mut Self), __copy: impl Fn(&T) -> T);
}

impl<T: Clone> GoSliceAppendCopying<T> for GoSlice<T> {
    fn go_append_copying(&mut self, __store: impl FnOnce(&mut Self), __copy: impl Fn(&T) -> T) {
        let __before = self.clone();
        let __len = self.len();
        __store(self);
        if !self.shares_backing_with(&__before) {
            for __i in 0..__len.min(self.len()) {
                let __value = __copy(&self.as_slice()[__i]);
                self.as_mut_slice()[__i] = __value;
            }
        }
    }
}

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

/// Interface for drawing
pub trait Drawable: std::fmt::Display + Any {
    fn __go_clone_box_drawable(&self) -> Box<dyn Drawable + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_print(&self, plus: bool) -> String { if plus { format!("{:#}", self) } else { format!("{}", self) } }
    fn __go_eq_drawable(&self, other: &(dyn Drawable + Send + Sync)) -> bool;
    fn draw(&self) -> Arc<Mutex<Option<String>>>;
}

impl Clone for Box<dyn Drawable + Send + Sync> {
    fn clone(&self) -> Self {
        Drawable::__go_clone_box_drawable(self.as_ref())
    }
}

/// Shape types
#[derive(Debug, Clone)]
pub struct Circle {
    pub radius: Arc<Mutex<Option<f64>>>,
}

impl Circle {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.radius.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            radius: __go_clone_0_0,
        }
    }
}


impl Default for Circle {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0.0)));
        Self {
            radius: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = go_float_string(((*self.radius.lock().unwrap().as_ref().unwrap())) as f64, false);
        write!(f, "{{{}}}", __go_fmt_0)
    }
}
impl PartialEq for Circle {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.radius.lock().unwrap().as_ref().cloned() }; let __right = { other.radius.lock().unwrap().as_ref().cloned() }; __left == __right }
        )
    }
}


#[derive(Debug, Clone)]
pub struct Rectangle {
    pub width: Arc<Mutex<Option<f64>>>,
    pub height: Arc<Mutex<Option<f64>>>,
}

impl Rectangle {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.width.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.height.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            width: __go_clone_0_0,
            height: __go_clone_0_1,
        }
    }
}


impl Default for Rectangle {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0.0)));
        let __go_default_0_1 = Arc::new(Mutex::new(Some(0.0)));
        Self {
            width: __go_default_0_0,
            height: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = go_float_string(((*self.width.lock().unwrap().as_ref().unwrap())) as f64, false);
        let __go_fmt_1 = go_float_string(((*self.height.lock().unwrap().as_ref().unwrap())) as f64, false);
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}
impl PartialEq for Rectangle {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.width.lock().unwrap().as_ref().cloned() }; let __right = { other.width.lock().unwrap().as_ref().cloned() }; __left == __right }
                && { let __left = { self.height.lock().unwrap().as_ref().cloned() }; let __right = { other.height.lock().unwrap().as_ref().cloned() }; __left == __right }
        )
    }
}


#[derive(Clone)]
pub struct Canvas {
    pub name: Arc<Mutex<Option<String>>>,
    pub shapes: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Drawable + Send + Sync>>>>>>>>,
}

impl Canvas {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.shapes.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
            shapes: __go_clone_1_0,
        }
    }
}


impl Default for Canvas {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        Self {
            name: __go_default_0_0,
            shapes: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Canvas {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice_wrapped_stringer(&self.shapes));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


/// Nested struct definitions
#[derive(Debug, Clone)]
pub struct Address {
    pub street: Arc<Mutex<Option<String>>>,
    pub city: Arc<Mutex<Option<String>>>,
    pub state: Arc<Mutex<Option<String>>>,
    pub zip_code: Arc<Mutex<Option<String>>>,
    pub country: Arc<Mutex<Option<String>>>,
}

impl Address {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.street.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.city.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.state.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.zip_code.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.country.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            street: __go_clone_0_0,
            city: __go_clone_1_0,
            state: __go_clone_2_0,
            zip_code: __go_clone_3_0,
            country: __go_clone_4_0,
        }
    }
}


impl Default for Address {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            street: __go_default_0_0,
            city: __go_default_1_0,
            state: __go_default_2_0,
            zip_code: __go_default_3_0,
            country: __go_default_4_0,
        }
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.street.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.city.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.state.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.zip_code.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.country.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4)
    }
}


#[derive(Debug, Clone)]
pub struct Contact {
    pub email: Arc<Mutex<Option<String>>>,
    pub phone: Arc<Mutex<Option<String>>>,
}

impl Contact {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.email.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.phone.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            email: __go_clone_0_0,
            phone: __go_clone_1_0,
        }
    }
}


impl Default for Contact {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            email: __go_default_0_0,
            phone: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Contact {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.email.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.phone.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct Person {
    pub name: Arc<Mutex<Option<String>>>,
    pub age: Arc<Mutex<Option<i64>>>,
    pub address: Arc<Mutex<Option<Address>>>,
    pub contact: Arc<Mutex<Option<Contact>>>,
}

impl Person {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.age.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.address.lock().unwrap(); Arc::new(Mutex::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        let __go_clone_3_0 = { let __guard = self.contact.lock().unwrap(); Arc::new(Mutex::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        Self {
            name: __go_clone_0_0,
            age: __go_clone_1_0,
            address: __go_clone_2_0,
            contact: __go_clone_3_0,
        }
    }
}


impl Default for Person {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(Address::default())));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(Contact::default())));
        Self {
            name: __go_default_0_0,
            age: __go_default_1_0,
            address: __go_default_2_0,
            contact: __go_default_3_0,
        }
    }
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.age.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.address.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.contact.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}


#[derive(Debug, Clone)]
pub struct Department {
    pub name: Arc<Mutex<Option<String>>>,
    pub manager: Arc<Mutex<Option<Person>>>,
    pub employees: Arc<Mutex<Option<Vec<Person>>>>,
    pub budget: Arc<Mutex<Option<f64>>>,
}

impl Department {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.manager.lock().unwrap(); Arc::new(Mutex::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        let __go_clone_2_0 = { let __guard = self.employees.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.budget.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
            manager: __go_clone_1_0,
            employees: __go_clone_2_0,
            budget: __go_clone_3_0,
        }
    }
}


impl Default for Department {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(Person::default())));
        let __go_default_2_0 = Arc::new(Mutex::new(None));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0.0)));
        Self {
            name: __go_default_0_0,
            manager: __go_default_1_0,
            employees: __go_default_2_0,
            budget: __go_default_3_0,
        }
    }
}

impl std::fmt::Display for Department {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.manager.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", format_slice(&self.employees));
        let __go_fmt_3 = go_float_string(((*self.budget.lock().unwrap().as_ref().unwrap())) as f64, false);
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}


#[derive(Debug, Clone)]
pub struct Company {
    pub name: Arc<Mutex<Option<String>>>,
    pub departments: Arc<Mutex<Option<Vec<Department>>>>,
    pub headquarters: Arc<Mutex<Option<Address>>>,
}

impl Company {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.departments.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.headquarters.lock().unwrap(); Arc::new(Mutex::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        Self {
            name: __go_clone_0_0,
            departments: __go_clone_1_0,
            headquarters: __go_clone_2_0,
        }
    }
}


impl Default for Company {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(Address::default())));
        Self {
            name: __go_default_0_0,
            departments: __go_default_1_0,
            headquarters: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for Company {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice(&self.departments));
        let __go_fmt_2 = format!("{}", (*self.headquarters.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


impl Circle {
    pub fn draw(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(format!("Circle(r={:.1})", (*self.radius.lock().unwrap().as_ref().unwrap())))))
    }
}

impl Drawable for Circle {
    fn draw(&self) -> Arc<Mutex<Option<String>>> {
        Circle::draw(self)
    }
    fn __go_clone_box_drawable(&self) -> Box<dyn Drawable + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Drawable + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_drawable(&self, other: &(dyn Drawable + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Circle>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct CirclePtr(pub Arc<Mutex<Option<Circle>>>);

impl std::fmt::Display for CirclePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Drawable for CirclePtr {
    fn draw(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_ptr: *const Circle = { let __recv_guard = self.0.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Circle };
        let __recv = unsafe { &*__recv_ptr };
        Circle::draw(__recv)
    }
    fn __go_clone_box_drawable(&self) -> Box<dyn Drawable + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Drawable + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => if plus { format!("&{:#}", __v) } else { format!("&{}", __v) }, None => "<nil>".to_string() }
    }
    fn __go_eq_drawable(&self, other: &(dyn Drawable + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<CirclePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Rectangle {
    pub fn draw(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(format!("Rectangle({}x{})", format!("{:.1}", (*self.width.lock().unwrap().as_ref().unwrap())), format!("{:.1}", (*self.height.lock().unwrap().as_ref().unwrap()))))))
    }
}

impl Drawable for Rectangle {
    fn draw(&self) -> Arc<Mutex<Option<String>>> {
        Rectangle::draw(self)
    }
    fn __go_clone_box_drawable(&self) -> Box<dyn Drawable + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Drawable + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_drawable(&self, other: &(dyn Drawable + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Rectangle>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct RectanglePtr(pub Arc<Mutex<Option<Rectangle>>>);

impl std::fmt::Display for RectanglePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Drawable for RectanglePtr {
    fn draw(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_ptr: *const Rectangle = { let __recv_guard = self.0.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Rectangle };
        let __recv = unsafe { &*__recv_ptr };
        Rectangle::draw(__recv)
    }
    fn __go_clone_box_drawable(&self) -> Box<dyn Drawable + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Drawable + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => if plus { format!("&{:#}", __v) } else { format!("&{}", __v) }, None => "<nil>".to_string() }
    }
    fn __go_eq_drawable(&self, other: &(dyn Drawable + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<RectanglePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

fn main() {
    go_install_panic_exit();
    ::math_bits::__go_init_all();
    ::sort::__go_init_all();

    println!("{}", format!("{}", "=== Creating nested structures ===".to_string()));
    let mut hq = Arc::new(Mutex::new(Some(Address { street: Arc::new(Mutex::new(Some("123 Corporate Blvd".to_string()))), city: Arc::new(Mutex::new(Some("Tech City".to_string()))), state: Arc::new(Mutex::new(Some("CA".to_string()))), zip_code: Arc::new(Mutex::new(Some("90210".to_string()))), country: Arc::new(Mutex::new(Some("USA".to_string()))), ..Default::default() })));
    let mut managerAddr = Arc::new(Mutex::new(Some(Address { street: Arc::new(Mutex::new(Some("456 Manager St".to_string()))), city: Arc::new(Mutex::new(Some("Suburb".to_string()))), state: Arc::new(Mutex::new(Some("CA".to_string()))), zip_code: Arc::new(Mutex::new(Some("90211".to_string()))), country: Arc::new(Mutex::new(Some("USA".to_string()))), ..Default::default() })));
    let mut emp1Addr = Arc::new(Mutex::new(Some(Address { street: Arc::new(Mutex::new(Some("789 Employee Ave".to_string()))), city: Arc::new(Mutex::new(Some("Hometown".to_string()))), state: Arc::new(Mutex::new(Some("CA".to_string()))), zip_code: Arc::new(Mutex::new(Some("90212".to_string()))), country: Arc::new(Mutex::new(Some("USA".to_string()))), ..Default::default() })));
    let mut emp2Addr = Arc::new(Mutex::new(Some(Address { street: Arc::new(Mutex::new(Some("321 Worker Way".to_string()))), city: Arc::new(Mutex::new(Some("Village".to_string()))), state: Arc::new(Mutex::new(Some("CA".to_string()))), zip_code: Arc::new(Mutex::new(Some("90213".to_string()))), country: Arc::new(Mutex::new(Some("USA".to_string()))), ..Default::default() })));
    let mut managerContact = Arc::new(Mutex::new(Some(Contact { email: Arc::new(Mutex::new(Some("manager@company.com".to_string()))), phone: Arc::new(Mutex::new(Some("555-0001".to_string()))), ..Default::default() })));
    let mut emp1Contact = Arc::new(Mutex::new(Some(Contact { email: Arc::new(Mutex::new(Some("emp1@company.com".to_string()))), phone: Arc::new(Mutex::new(Some("555-0002".to_string()))), ..Default::default() })));
    let mut emp2Contact = Arc::new(Mutex::new(Some(Contact { email: Arc::new(Mutex::new(Some("emp2@company.com".to_string()))), phone: Arc::new(Mutex::new(Some("555-0003".to_string()))), ..Default::default() })));
    let mut manager = Arc::new(Mutex::new(Some(Person { name: Arc::new(Mutex::new(Some("Alice Manager".to_string()))), age: Arc::new(Mutex::new(Some(45))), address: Arc::new(Mutex::new(Some({ let __arg_holder = managerAddr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))), contact: Arc::new(Mutex::new(Some({ let __arg_holder = managerContact.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))), ..Default::default() })));
    let mut employee1 = Arc::new(Mutex::new(Some(Person { name: Arc::new(Mutex::new(Some("Bob Employee".to_string()))), age: Arc::new(Mutex::new(Some(30))), address: Arc::new(Mutex::new(Some({ let __arg_holder = emp1Addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))), contact: Arc::new(Mutex::new(Some({ let __arg_holder = emp1Contact.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))), ..Default::default() })));
    let mut employee2 = Arc::new(Mutex::new(Some(Person { name: Arc::new(Mutex::new(Some("Carol Worker".to_string()))), age: Arc::new(Mutex::new(Some(28))), address: Arc::new(Mutex::new(Some({ let __arg_holder = emp2Addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))), contact: Arc::new(Mutex::new(Some({ let __arg_holder = emp2Contact.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))), ..Default::default() })));
    let mut engineering = Arc::new(Mutex::new(Some(Department {
        name: Arc::new(Mutex::new(Some("Engineering".to_string()))),
        manager: Arc::new(Mutex::new(Some({ let __arg_holder = manager.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))),
        employees: Arc::new(Mutex::new(Some(vec![({ let __v = (*employee1.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }), ({ let __v = (*employee2.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v })]))),
        budget: Arc::new(Mutex::new(Some(1e+06))),
        ..Default::default()
    })));
    let mut company = Arc::new(Mutex::new(Some(Company { name: Arc::new(Mutex::new(Some("TechCorp Inc".to_string()))), departments: Arc::new(Mutex::new(Some(vec![({ let __v = (*engineering.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v })]))), headquarters: Arc::new(Mutex::new(Some({ let __arg_holder = hq.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))), ..Default::default() })));
    println!("{}", format!("{}", "\n=== Accessing nested data ===".to_string()));
    print!("Company: {}\n", (*{ let __field = (*company.lock().unwrap().as_ref().unwrap()).name.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone());
    print!("HQ Address: {}, {}, {} {}\n", format!("{}", (*(*(*company.lock().unwrap().as_ref().unwrap()).headquarters.lock().unwrap().as_ref().unwrap()).street.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", (*(*(*company.lock().unwrap().as_ref().unwrap()).headquarters.lock().unwrap().as_ref().unwrap()).city.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", (*(*(*company.lock().unwrap().as_ref().unwrap()).headquarters.lock().unwrap().as_ref().unwrap()).state.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", (*(*(*company.lock().unwrap().as_ref().unwrap()).headquarters.lock().unwrap().as_ref().unwrap()).zip_code.lock().unwrap().as_ref().unwrap()).clone()));
    print!("Department: {}\n", (*{ let __seq = { let __seq_holder = (*company.lock().unwrap().as_ref().unwrap()).departments.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.name.lock().unwrap().as_ref().unwrap()).clone());
    print!("Department Budget: ${:.2}\n", (*{ let __seq = { let __seq_holder = (*company.lock().unwrap().as_ref().unwrap()).departments.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.budget.lock().unwrap().as_ref().unwrap()));
    print!("Manager: {} (Age: {})\n", format!("{}", (*(*{ let __seq = { let __seq_holder = (*company.lock().unwrap().as_ref().unwrap()).departments.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.manager.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", (*(*{ let __seq = { let __seq_holder = (*company.lock().unwrap().as_ref().unwrap()).departments.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.manager.lock().unwrap().as_ref().unwrap()).age.lock().unwrap().as_ref().unwrap())));
    print!("Manager Email: {}\n", (*(*(*{ let __seq = { let __seq_holder = (*company.lock().unwrap().as_ref().unwrap()).departments.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.manager.lock().unwrap().as_ref().unwrap()).contact.lock().unwrap().as_ref().unwrap()).email.lock().unwrap().as_ref().unwrap()).clone());
    print!("Manager Address: {}, {}\n", format!("{}", (*(*(*{ let __seq = { let __seq_holder = (*company.lock().unwrap().as_ref().unwrap()).departments.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.manager.lock().unwrap().as_ref().unwrap()).address.lock().unwrap().as_ref().unwrap()).city.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", (*(*(*{ let __seq = { let __seq_holder = (*company.lock().unwrap().as_ref().unwrap()).departments.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.manager.lock().unwrap().as_ref().unwrap()).address.lock().unwrap().as_ref().unwrap()).state.lock().unwrap().as_ref().unwrap()).clone()));
    println!("{}", format!("{}", "\n=== Department employees ===".to_string()));
    { let __range_holder = { let __seq = { let __seq_holder = (*company.lock().unwrap().as_ref().unwrap()).departments.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.employees.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for (i, emp) in __range_values.iter().enumerate() {
        let emp = &emp.__go_value_clone();
        print!("Employee {}: {}\n", format!("{}", { let __tmp_x = i as i64; let __tmp_y = 1; __tmp_x + __tmp_y }), format!("{}", (*emp.name.lock().unwrap().as_ref().unwrap()).clone()));
        print!("  Age: {}\n", (*emp.age.lock().unwrap().as_ref().unwrap()));
        print!("  Email: {}\n", (*(*emp.contact.lock().unwrap().as_ref().unwrap()).email.lock().unwrap().as_ref().unwrap()).clone());
        print!("  Phone: {}\n", (*(*emp.contact.lock().unwrap().as_ref().unwrap()).phone.lock().unwrap().as_ref().unwrap()).clone());
        print!("  Address: {}, {}, {}\n", format!("{}", (*(*emp.address.lock().unwrap().as_ref().unwrap()).street.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", (*(*emp.address.lock().unwrap().as_ref().unwrap()).city.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", (*(*emp.address.lock().unwrap().as_ref().unwrap()).state.lock().unwrap().as_ref().unwrap()).clone()));
        println!();
    } }
    println!("{}", format!("{}", "=== Nested maps ===".to_string()));
    let mut inventory = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>>::from([("electronics".to_string(), Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("laptops".to_string(), Arc::new(Mutex::new(Some(50i64)))), ("phones".to_string(), Arc::new(Mutex::new(Some(100i64)))), ("tablets".to_string(), Arc::new(Mutex::new(Some(25i64))))]))))), ("furniture".to_string(), Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("chairs".to_string(), Arc::new(Mutex::new(Some(200i64)))), ("desks".to_string(), Arc::new(Mutex::new(Some(75i64)))), ("lamps".to_string(), Arc::new(Mutex::new(Some(150i64))))]))))), ("supplies".to_string(), Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("pens".to_string(), Arc::new(Mutex::new(Some(1000i64)))), ("paper".to_string(), Arc::new(Mutex::new(Some(500i64)))), ("folders".to_string(), Arc::new(Mutex::new(Some(300i64))))])))))]))));
    println!("{}", format!("{}", "Inventory:".to_string()));
    let mut categories: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
    for (category, _) in { let __range_holder = inventory.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = categories.clone(); let __append_value = category.clone(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *categories.lock().unwrap() = __slice_header; };
    }
    sort::strings(categories.clone());
    { let __range_holder = categories.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for category in __range_values.iter() {
        let mut items = { let __map = { let __map_holder = inventory.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(category)).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) };
        print!("  {}:\n", category);
        let mut itemNames: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
        for (item, _) in { let __range_holder = items.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = itemNames.clone(); let __append_value = item.clone(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *itemNames.lock().unwrap() = __slice_header; };
    }
        sort::strings(itemNames.clone());
        { let __range_holder = itemNames.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for item in __range_values.iter() {
        let mut count = Arc::new(Mutex::new(Some({ let __map = { let __map_holder = items.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(item)).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) })));
        print!("    {}: {}\n", format!("{}", item), format!("{}", { let __v = (*count.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    } }
    } }
    let mut laptopCount = Arc::new(Mutex::new(Some({ let __map = { let __map_holder = { let __map = { let __map_holder = inventory.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"electronics".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"laptops".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) })));
    print!("Laptop count: {}\n", { let __v = (*laptopCount.lock().unwrap().as_ref().unwrap()).clone(); __v });
    println!("{}", format!("{}", "\n=== Nested slices ===".to_string()));
    let mut matrix = Arc::new(Mutex::new(Some(vec![vec![1i64, 2i64, 3i64], vec![4i64, 5i64, 6i64], vec![7i64, 8i64, 9i64]])));
    println!("{}", format!("{}", "Matrix:".to_string()));
    { let __range_holder = matrix.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for (i, row) in __range_values.iter().enumerate() {
        print!("Row {}: ", i);
        for (j, val) in row.iter().copied().enumerate() {
        print!("{} ", val);
        if { let __tmp_x = (j as i64); let __tmp_y = ({ let __tmp_x = (row.len() as i64); let __tmp_y = 1; __tmp_x - __tmp_y } as i64); __tmp_x < __tmp_y } {
        print!("{}", format!("{}", " ".to_string()));
    }
    }
        println!();
    } }
    let mut centerElement = Arc::new(Mutex::new(Some(({ let __seq = { let __seq_holder = matrix.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() })[(1) as usize].clone())));
    print!("Center element: {}\n", { let __v = (*centerElement.lock().unwrap().as_ref().unwrap()).clone(); __v });
    let mut cube = Arc::new(Mutex::new(Some(vec![vec![vec![1i64, 2i64], vec![3i64, 4i64]], vec![vec![5i64, 6i64], vec![7i64, 8i64]]])));
    println!("{}", format!("{}", "\n3D Cube:".to_string()));
    { let __range_holder = cube.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for (i, layer) in __range_values.iter().enumerate() {
        print!("Layer {}:\n", i);
        for (j, row) in layer.iter().enumerate() {
        print!("  Row {}: ", j);
        for val in row.iter().copied() {
        print!("{} ", val);
    }
        println!();
    }
    } }
    println!("{}", format!("{}", "\n=== Complex nested with interfaces ===".to_string()));
    let mut canvas = Arc::new(Mutex::new(Some(Canvas { name: Arc::new(Mutex::new(Some("My Drawing".to_string()))), shapes: Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Box::new(Circle { radius: Arc::new(Mutex::new(Some(5.0))), ..Default::default() }) as Box<dyn Drawable + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(Rectangle { width: Arc::new(Mutex::new(Some(10.0))), height: Arc::new(Mutex::new(Some(8.0))), ..Default::default() }) as Box<dyn Drawable + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(Circle { radius: Arc::new(Mutex::new(Some(3.0))), ..Default::default() }) as Box<dyn Drawable + Send + Sync>)))]))), ..Default::default() })));
    print!("Canvas: {}\n", (*{ let __field = (*canvas.lock().unwrap().as_ref().unwrap()).name.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone());
    { let __range_holder = (*canvas.lock().unwrap().as_ref().unwrap()).shapes.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for (i, shape) in __range_values.iter().enumerate() {
        print!("Shape {}: {}\n", format!("{}", { let __tmp_x = i as i64; let __tmp_y = 1; __tmp_x + __tmp_y }), format!("{}", (*(*shape.lock().unwrap().as_ref().unwrap()).draw().lock().unwrap().as_ref().unwrap())));
    } }
    println!("{}", format!("{}", "\n=== Modifying nested structures ===".to_string()));
    { let new_val = "bob.new@company.com".to_string(); *(*{ let __seq = { let __seq_holder = { let __seq = { let __seq_holder = (*company.lock().unwrap().as_ref().unwrap()).departments.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.employees.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.contact.lock().unwrap().as_ref().unwrap()).email.lock().unwrap() = Some(new_val); };
    print!("Updated employee email: {}\n", (*(*{ let __seq = { let __seq_holder = { let __seq = { let __seq_holder = (*company.lock().unwrap().as_ref().unwrap()).departments.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.employees.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.contact.lock().unwrap().as_ref().unwrap()).email.lock().unwrap().as_ref().unwrap()).clone());
    let mut newEmployee = Arc::new(Mutex::new(Some(Person {
        name: Arc::new(Mutex::new(Some("Dave Newbie".to_string()))),
        age: Arc::new(Mutex::new(Some(25))),
        address: Arc::new(Mutex::new(Some(Address { street: Arc::new(Mutex::new(Some("999 New St".to_string()))), city: Arc::new(Mutex::new(Some("Newtown".to_string()))), state: Arc::new(Mutex::new(Some("CA".to_string()))), zip_code: Arc::new(Mutex::new(Some("90214".to_string()))), country: Arc::new(Mutex::new(Some("USA".to_string()))), ..Default::default() }))),
        contact: Arc::new(Mutex::new(Some(Contact { email: Arc::new(Mutex::new(Some("dave@company.com".to_string()))), phone: Arc::new(Mutex::new(Some("555-0004".to_string()))), ..Default::default() }))),
        ..Default::default()
    })));
    { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = { let __seq = { let __seq_holder = (*company.lock().unwrap().as_ref().unwrap()).departments.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.employees.clone(); let __append_value = ({ let __v = (*newEmployee.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }); let mut __appended: Vec<Person> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.go_append_copying(|__go_slice| { __go_slice.push(__append_value); }, |__go_elem| __go_elem.__go_value_clone()); Arc::new(Mutex::new(Some(__appended))) })); *{ let __seq = { let __seq_holder = (*company.lock().unwrap().as_ref().unwrap()).departments.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.employees.lock().unwrap() = __slice_header; };
    print!("Added new employee: {}\n", (*{ let __field = (*newEmployee.lock().unwrap().as_ref().unwrap()).name.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone());
    print!("Total employees now: {}\n", ({ let __len = ({ let __len_target = { let __field = { let __seq = { let __seq_holder = (*company.lock().unwrap().as_ref().unwrap()).departments.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.employees.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }); __len }));
}

impl GoValueClone for Circle {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Rectangle {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Canvas {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Address {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Contact {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Person {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Department {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Company {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
