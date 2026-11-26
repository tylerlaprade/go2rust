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
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[allow(unused_macros)]
macro_rules! print {
    ($($__arg:tt)*) => {{
        std::print!($($__arg)*);
        std::io::Write::flush(&mut std::io::stdout()).ok();
    }};
}

// Go expands a pointer to a struct only when the pointer is the value being
// printed. Inside a struct or a slice it prints the address. That is also
// what keeps a value that points back to itself from printing forever.
fn go_format_any_pointer(nested: bool, address: *const (), is_nil: impl FnOnce() -> bool, expand: impl FnOnce() -> String) -> String {
    if !nested {
        return expand();
    }
    if is_nil() { "<nil>".to_string() } else { format!("{:p}", address) }
}
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

fn format_any(value: &(dyn Any + Send + Sync)) -> String {
    format_any_at(value, false)
}

// Prints a value held inside the one being printed: a struct's interface
// field, an element of a []interface{}.
fn format_any_nested(value: &(dyn Any + Send + Sync)) -> String {
    format_any_at(value, true)
}

fn format_any_at(value: &(dyn Any + Send + Sync), nested: bool) -> String {
    if let Some(v) = value.downcast_ref::<i32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f64>() {
        go_float_string(*v, false)
    } else if let Some(v) = value.downcast_ref::<f32>() {
        go_float_string(*v as f64, true)
    } else if let Some(v) = value.downcast_ref::<String>() {
        v.clone()
    } else if let Some(v) = value.downcast_ref::<&str>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<bool>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i8>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i16>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<isize>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u8>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u16>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<usize>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<Point>>>>>>>>() {
        format_map(v)
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>>() {
        format_map(v)
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<BTreeMap<i64, Arc<Mutex<Option<String>>>>>>>>() {
        format_map(v)
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Node>>>>() {
        go_format_any_pointer(nested, Arc::as_ptr(v) as *const (), || v.try_lock().map(|__guard| __guard.is_none()).unwrap_or(false), || match (*v.lock().unwrap()).as_ref() { Some(__pointee) => format!("&{}", __pointee), None => "<nil>".to_string() })
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<NotFound>>>>() {
        match (*v.lock().unwrap()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() }
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Point>>>>() {
        go_format_any_pointer(nested, Arc::as_ptr(v) as *const (), || v.try_lock().map(|__guard| __guard.is_none()).unwrap_or(false), || match (*v.lock().unwrap()).as_ref() { Some(__pointee) => format!("&{}", __pointee), None => "<nil>".to_string() })
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Status>>>>() {
        match (*v.lock().unwrap()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() }
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Vec<Point>>>>>() {
        format_slice(v)
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Vec<String>>>>>() {
        format_slice(v)
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Vec<bool>>>>>() {
        format_slice(v)
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Vec<f64>>>>>() {
        format_slice(v)
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Vec<i64>>>>>() {
        format_slice(v)
    } else if let Some(v) = value.downcast_ref::<Label>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<Node>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<NotFound>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<Outcome>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<Point>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<Report>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<Status>() {
        format!("{}", v)
    } else {
        panic!("go2rust: cannot print this value held in an interface: its type has no formatting case yet")
    }
}

fn format_any_slice_values(slice: &Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>, format_value: fn(&(dyn Any + Send + Sync)) -> String) -> String {
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.iter().map(|v| format_value(v.as_ref())).collect();
        formatted.join(" ")
    } else {
        String::new()
    }
}

fn format_any_slice(slice: &Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> String {
    format!("[{}]", format_any_slice_values(slice, format_any_nested))
}

fn format_any_variadic(slice: &Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> String {
    format_any_slice_values(slice, format_any)
}

fn go_any_try_clone(value: &(dyn Any + Send + Sync)) -> Option<Box<dyn Any + Send + Sync>> {
    if let Some(v) = value.downcast_ref::<i32>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<i64>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<i8>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<i16>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<u32>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<u64>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<u8>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<u16>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<usize>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<isize>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<f64>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<f32>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<&'static str>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<bool>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<char>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<String>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<Point>>>>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<BTreeMap<i64, Arc<Mutex<Option<String>>>>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Node>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<NotFound>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Point>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Status>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Vec<Point>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Vec<String>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Vec<bool>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Vec<f64>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Vec<i64>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Label>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Node>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<NotFound>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Outcome>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Point>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Report>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Status>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    None
}

fn go_any_clone(value: &(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync> {
    go_any_try_clone(value).unwrap_or_else(|| panic!("go_any_clone: unsupported dynamic type; add typed lowering instead of cloning Box<dyn Any>"))
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
        if __text == GO_UNPRINTABLE_PANIC_VALUE {
            return format_any(&**__inner);
        }
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
    if let Some(__value) = __payload.downcast_ref::<f64>() {
        // Go prints a float in a panic as +1.500000e+000.
        if __value.is_nan() {
            return String::from("NaN");
        }
        if __value.is_infinite() {
            return String::from(if *__value > 0.0 { "+Inf" } else { "-Inf" });
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

#[derive(Debug, Clone)]
pub struct Point {
    pub x: Arc<Mutex<Option<i64>>>,
    pub y: Arc<Mutex<Option<i64>>>,
}

impl Point {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.y.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
            y: __go_clone_0_1,
        }
    }
}


impl Default for Point {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_0_1 = Arc::new(Mutex::new(Some(0)));
        Self {
            x: __go_default_0_0,
            y: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.y.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct Status {
    pub code: Arc<Mutex<Option<i64>>>,
}

impl Status {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.code.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            code: __go_clone_0_0,
        }
    }
}


impl Default for Status {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            code: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct NotFound {
    pub name: Arc<Mutex<Option<String>>>,
}

impl NotFound {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for NotFound {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for NotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}


/// Node can point at itself through its interface field.
#[derive(Clone)]
pub struct Node {
    pub name: Arc<Mutex<Option<String>>>,
    pub next: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>,
}

impl Node {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.next.clone();
        Self {
            name: __go_clone_0_0,
            next: __go_clone_1_0,
        }
    }
}


impl Default for Node {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        Self {
            name: __go_default_0_0,
            next: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", match self.next.lock().unwrap().as_ref() { Some(__v) => format_any_nested(__v.as_ref()), None => "<nil>".to_string() });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


/// Label prints itself by printing another value. That is a print call of its
/// own: the pointer it prints is at the top level of that call, wherever the
/// Label sits in the value being printed.
#[derive(Debug, Clone)]
pub struct Label {
    pub x: Arc<Mutex<Option<i64>>>,
}

impl Label {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
        }
    }
}


impl Default for Label {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            x: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}


/// Missing does the same from its Error method.
#[derive(Debug, Clone)]
pub struct Missing {
    pub line: Arc<Mutex<Option<i64>>>,
}

impl Missing {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.line.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            line: __go_clone_0_0,
        }
    }
}


impl Default for Missing {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            line: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Missing {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}


/// Outcome is the usual result struct: its error is nil unless something
/// failed.
#[derive(Clone)]
pub struct Outcome {
    pub value: Arc<Mutex<Option<i64>>>,
    pub err: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>,
}

impl Outcome {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.value.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.err.clone();
        Self {
            value: __go_clone_0_0,
            err: __go_clone_1_0,
        }
    }
}


impl Default for Outcome {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        Self {
            value: __go_default_0_0,
            err: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Outcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.value.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", match self.err.lock().unwrap().as_ref() { Some(__v) => format!("{}", __v), None => "<nil>".to_string() });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


/// Failure is another name for error. A field of this type is an error field.
pub type Failure = Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>;


#[derive(Clone, Default)]
pub struct Report {
    pub err: Failure,
}

impl Report {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.err.clone();
        Self {
            err: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", match self.err.lock().unwrap().as_ref() { Some(__v) => format!("{}", __v), None => "<nil>".to_string() });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Status {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(format!("status {}", (*self.code.lock().unwrap().as_ref().unwrap())))))
    }
}

impl NotFound {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("{}{}", (*self.name.clone().lock().unwrap().as_ref().unwrap()), " not found".to_string()))));
    }
}

impl StdError for NotFound {}


impl Label {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        render(Arc::new(Mutex::new(Some({ let __arg_value = Box::new(Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some({ let __selector_holder = self.x.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }))), ..Default::default() }))).clone()) as Box<dyn Any + Send + Sync>; __arg_value }))))
    }
}

impl Missing {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("{}{}", "missing at ".to_string(), (*render(Arc::new(Mutex::new(Some({ let __arg_value = Box::new(Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some({ let __selector_holder = self.line.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }))), ..Default::default() }))).clone()) as Box<dyn Any + Send + Sync>; __arg_value })))).lock().unwrap().as_ref().unwrap())))));
    }
}

impl StdError for Missing {}


pub fn render(v: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
    Arc::new(Mutex::new(Some(format!("{}", { let __text = match v.lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }))))
}

/// shape describes text around the addresses in it, which change from run to
/// run: whether it starts with prefix, and how many addresses it holds.
pub fn shape(text: Arc<Mutex<Option<String>>>, prefix: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    Arc::new(Mutex::new(Some(format!("{} {}", format!("{}", strings::has_prefix(Arc::new(Mutex::new(Some({ let __arg_holder = text.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = prefix.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))), format!("{}", { let __tmp_x = (({ let __len = (*strings::split(Arc::new(Mutex::new(Some({ let __arg_holder = text.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("0x".to_string())))).lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); let __tmp_y = 1; __tmp_x - __tmp_y })))))
}

/// show prints whatever it is given. The value arrives as an interface, so
/// the printer has to find out what it holds.
pub fn show(v: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) {
    println!("{}", format!("{}", { let __text = match v.lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }));
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

    show(Arc::new(Mutex::new(Some(Box::new(42i64) as Box<dyn Any + Send + Sync>))));
    show(Arc::new(Mutex::new(Some(Box::new("hi".to_string()) as Box<dyn Any + Send + Sync>))));
    show(Arc::new(Mutex::new(Some(Box::new(2.5) as Box<dyn Any + Send + Sync>))));
    show(Arc::new(Mutex::new(Some(Box::new(true) as Box<dyn Any + Send + Sync>))));
    show(Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(vec![1i64, 2i64, 3i64]))).clone()) as Box<dyn Any + Send + Sync>))));
    show(Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(vec!["a".to_string(), "b".to_string()]))).clone()) as Box<dyn Any + Send + Sync>))));
    show(Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(vec![true, false]))).clone()) as Box<dyn Any + Send + Sync>))));
    show(Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(vec![1.5, 2.0]))).clone()) as Box<dyn Any + Send + Sync>))));
    show(Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(vec![Point { x: Arc::new(Mutex::new(Some(1))), y: Arc::new(Mutex::new(Some(2))), ..Default::default() }, Point { x: Arc::new(Mutex::new(Some(3))), y: Arc::new(Mutex::new(Some(4))), ..Default::default() }]))).clone()) as Box<dyn Any + Send + Sync>))));
    show(Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("b".to_string(), Arc::new(Mutex::new(Some(2i64)))), ("a".to_string(), Arc::new(Mutex::new(Some(1i64))))])))).clone()) as Box<dyn Any + Send + Sync>))));
    show(Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(BTreeMap::<i64, Arc<Mutex<Option<String>>>>::from([(2i64, Arc::new(Mutex::new(Some("two".to_string())))), (1i64, Arc::new(Mutex::new(Some("one".to_string()))))])))).clone()) as Box<dyn Any + Send + Sync>))));
    show(Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Point>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(1))), y: Arc::new(Mutex::new(Some(2))), ..Default::default() }))))])))).clone()) as Box<dyn Any + Send + Sync>))));
    show(Arc::new(Mutex::new(Some(Box::new(Point { x: Arc::new(Mutex::new(Some(1))), y: Arc::new(Mutex::new(Some(2))), ..Default::default() }) as Box<dyn Any + Send + Sync>))));
    show(Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(3))), y: Arc::new(Mutex::new(Some(4))), ..Default::default() }))).clone()) as Box<dyn Any + Send + Sync>))));
    show(Arc::new(Mutex::new(Some(Box::new(Status { code: Arc::new(Mutex::new(Some(404))), ..Default::default() }) as Box<dyn Any + Send + Sync>))));
    show(Arc::new(Mutex::new(Some(Box::new(NotFound { name: Arc::new(Mutex::new(Some("x".to_string()))), ..Default::default() }) as Box<dyn Any + Send + Sync>))));
    let mut none: Arc<Mutex<Option<Vec<i64>>>> = Arc::new(Mutex::new(None));
    show(Arc::new(Mutex::new(Some(Box::new(none.clone()) as Box<dyn Any + Send + Sync>))));
    let mut nowhere: Arc<Mutex<Option<Point>>> = Arc::new(Mutex::new(None));
    show(Arc::new(Mutex::new(Some(Box::new(nowhere.clone()) as Box<dyn Any + Send + Sync>))));
    let mut xs = Arc::new(Mutex::new(Some(vec![4i64, 5i64])));
    let mut p = Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(6))), y: Arc::new(Mutex::new(Some(7))), ..Default::default() })));
    let mut boxed: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(xs.clone()) as Box<dyn Any + Send + Sync>)));
    println!("{} {} {}", format!("{}", "println:".to_string()), format!("{}", { let __text = match boxed.lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }), format!("{}", Point { x: Arc::new(Mutex::new(Some(5))), y: Arc::new(Mutex::new(Some(6))), ..Default::default() }));
    print!("printf: {} {}\n", format!("{}", { let __text = match boxed.lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }), format!("{}", 7));
    { let new_val = Box::new(p.clone()) as Box<dyn Any + Send + Sync>; *boxed.lock().unwrap() = Some(new_val); };
    print!("printf: {}\n", { let __text = match boxed.lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text });
    let mut items = Arc::new(Mutex::new(Some(vec![
        Box::new(Arc::new(Mutex::new(Some(vec![1i64, 2i64]))).clone()) as Box<dyn Any + Send + Sync>,
        Box::new(Point { x: Arc::new(Mutex::new(Some(7))), y: Arc::new(Mutex::new(Some(8))), ..Default::default() }) as Box<dyn Any + Send + Sync>,
        Box::new(Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("k".to_string(), Arc::new(Mutex::new(Some(1i64))))])))).clone()) as Box<dyn Any + Send + Sync>,
        Box::new("s".to_string()) as Box<dyn Any + Send + Sync>,
        Box::new(3i64) as Box<dyn Any + Send + Sync>,
    ])));
    { let __range_holder = items.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_len = __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); std::mem::drop(__range_guard); let __range_values = 0..__range_len; for item in __range_values.map(|__range_index| { let __range_guard = __range_holder.lock().unwrap(); go_any_clone(__range_guard.as_ref().unwrap()[__range_index].as_ref()) }) {
        println!("{} {}", format!("{}", "item:".to_string()), format!("{}", format_any(item.as_ref())));
    } }
    let mut names = Arc::new(Mutex::new(Some(vec!["x".to_string()])));
    let mut byName = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>>::from([("point".to_string(), Arc::new(Mutex::new(Some(Box::new(Point { x: Arc::new(Mutex::new(Some(9))), y: Arc::new(Mutex::new(Some(9))), ..Default::default() }) as Box<dyn Any + Send + Sync>)))), ("list".to_string(), Arc::new(Mutex::new(Some(Box::new(names.clone()) as Box<dyn Any + Send + Sync>))))]))));
    println!("{} {}", format!("{}", { let __text = match ({ let __map = { let __map_holder = byName.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"point".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }).lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }), format!("{}", { let __text = match ({ let __map = { let __map_holder = byName.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"list".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }).lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }));
    let mut r#loop = Arc::new(Mutex::new(Some(Node { name: Arc::new(Mutex::new(Some("loop".to_string()))), ..Default::default() })));
    { let new_val = Box::new(r#loop.clone()) as Box<dyn Any + Send + Sync>; *(*r#loop.lock().unwrap().as_ref().unwrap()).next.lock().unwrap() = Some(new_val); };
    let mut text = render(Arc::new(Mutex::new(Some(Box::new(r#loop.clone()) as Box<dyn Any + Send + Sync>))));
    println!("{} {} {} {}", format!("{}", "cycle:".to_string()), format!("{}", strings::has_prefix(Arc::new(Mutex::new(Some({ let __arg_holder = text.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("&{loop 0x".to_string()))))), format!("{}", strings::has_suffix(Arc::new(Mutex::new(Some({ let __arg_holder = text.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("}".to_string()))))), format!("{}", { let __tmp_x = (({ let __len = (*strings::split(Arc::new(Mutex::new(Some({ let __arg_holder = text.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("loop".to_string())))).lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); let __tmp_y = 1; __tmp_x - __tmp_y }));
    let mut tail = Arc::new(Mutex::new(Some(Node { name: Arc::new(Mutex::new(Some("tail".to_string()))), ..Default::default() })));
    { let new_val = render(Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(Node { name: Arc::new(Mutex::new(Some("head".to_string()))), next: Arc::new(Mutex::new(Some(Box::new(tail.clone()) as Box<dyn Any + Send + Sync>))), ..Default::default() }))).clone()) as Box<dyn Any + Send + Sync>)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *text.lock().unwrap() = __moved_val; };
    println!("{} {} {}", format!("{}", "chain:".to_string()), format!("{}", strings::has_prefix(Arc::new(Mutex::new(Some({ let __arg_holder = text.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("&{head 0x".to_string()))))), format!("{}", strings::contains(Arc::new(Mutex::new(Some({ let __arg_holder = text.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("tail".to_string()))))));
    println!("{} {}", format!("{}", "struct value, boxed:".to_string()), format!("{}", (*shape(
        render(Arc::new(Mutex::new(Some({ let __arg_value = Box::new({ let __v = (*r#loop.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }) as Box<dyn Any + Send + Sync>; __arg_value })))),
        Arc::new(Mutex::new(Some("{loop 0x".to_string())))
    ).lock().unwrap().as_ref().unwrap())));
    println!("{} {}", format!("{}", "struct value, typed:".to_string()), format!("{}", (*shape(
        Arc::new(Mutex::new(Some({ let __arg_value = format!("{}", { let __v = (*r#loop.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }); __arg_value }))),
        Arc::new(Mutex::new(Some("{loop 0x".to_string())))
    ).lock().unwrap().as_ref().unwrap())));
    println!("{} {}", format!("{}", "pointer, typed:".to_string()), format!("{}", (*shape(
        Arc::new(Mutex::new(Some({ let __arg_value = format!("{}", { let __text = match r#loop.lock().unwrap().as_ref() { Some(__value) => format!("&{}", __value), None => "<nil>".to_string() }; __text }); __arg_value }))),
        Arc::new(Mutex::new(Some("&{loop 0x".to_string())))
    ).lock().unwrap().as_ref().unwrap())));
    { let new_val = render(Arc::new(Mutex::new(Some(Box::new(Node { name: Arc::new(Mutex::new(Some("holder".to_string()))), next: Arc::new(Mutex::new(Some(Box::new(tail.clone()) as Box<dyn Any + Send + Sync>))), ..Default::default() }) as Box<dyn Any + Send + Sync>)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *text.lock().unwrap() = __moved_val; };
    println!("{} {} {}", format!("{}", "struct holding a pointer:".to_string()), format!("{}", (*shape(Arc::new(Mutex::new(Some({ let __arg_holder = text.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("{holder 0x".to_string())))).lock().unwrap().as_ref().unwrap())), format!("{}", strings::contains(Arc::new(Mutex::new(Some({ let __arg_holder = text.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("tail".to_string()))))));
    let mut mixed = Arc::new(Mutex::new(Some(vec![Box::new(tail.clone()) as Box<dyn Any + Send + Sync>, Box::new(1i64) as Box<dyn Any + Send + Sync>])));
    { let new_val = Arc::new(Mutex::new(Some(format!("{}", format_any_slice(&mixed))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *text.lock().unwrap() = __moved_val; };
    println!("{} {} {}", format!("{}", "slice holding a pointer:".to_string()), format!("{}", (*shape(Arc::new(Mutex::new(Some({ let __arg_holder = text.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("[0x".to_string())))).lock().unwrap().as_ref().unwrap())), format!("{}", strings::has_suffix(Arc::new(Mutex::new(Some({ let __arg_holder = text.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(" 1]".to_string()))))));
    let mut noNode: Arc<Mutex<Option<Node>>> = Arc::new(Mutex::new(None));
    show(Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(Node { name: Arc::new(Mutex::new(Some("end".to_string()))), next: Arc::new(Mutex::new(Some(Box::new(noNode.clone()) as Box<dyn Any + Send + Sync>))), ..Default::default() }))).clone()) as Box<dyn Any + Send + Sync>))));
    show(Arc::new(Mutex::new(Some(Box::new(p.clone()) as Box<dyn Any + Send + Sync>))));
    println!("{}", format!("{}", Node { name: Arc::new(Mutex::new(Some("typed".to_string()))), ..Default::default() }));
    show(Arc::new(Mutex::new(Some(Box::new(Node { name: Arc::new(Mutex::new(Some("boxed".to_string()))), ..Default::default() }) as Box<dyn Any + Send + Sync>))));
    show(Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(Node { name: Arc::new(Mutex::new(Some("pointer".to_string()))), ..Default::default() }))).clone()) as Box<dyn Any + Send + Sync>))));
    show(Arc::new(Mutex::new(Some(Box::new(Node { name: Arc::new(Mutex::new(Some("full".to_string()))), next: Arc::new(Mutex::new(Some(Box::new(3i64) as Box<dyn Any + Send + Sync>))), ..Default::default() }) as Box<dyn Any + Send + Sync>))));
    println!("{}", format!("{}", Outcome { value: Arc::new(Mutex::new(Some(1))), ..Default::default() }));
    show(Arc::new(Mutex::new(Some(Box::new(Outcome { value: Arc::new(Mutex::new(Some(2))), ..Default::default() }) as Box<dyn Any + Send + Sync>))));
    show(Arc::new(Mutex::new(Some(Box::new(Outcome { value: Arc::new(Mutex::new(Some(3))), err: Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(format!("failed")))))), ..Default::default() }) as Box<dyn Any + Send + Sync>))));
    println!("{}", format!("{}", Report { err: Default::default() }));
    show(Arc::new(Mutex::new(Some(Box::new(Report { err: Default::default() }) as Box<dyn Any + Send + Sync>))));
    show(Arc::new(Mutex::new(Some(Box::new(Report { err: Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(format!("reported")))))), ..Default::default() }) as Box<dyn Any + Send + Sync>))));
    println!("{}", format!("{}", Label { x: Arc::new(Mutex::new(Some(7))), ..Default::default() }));
    show(Arc::new(Mutex::new(Some(Box::new(Label { x: Arc::new(Mutex::new(Some(7))), ..Default::default() }) as Box<dyn Any + Send + Sync>))));
    println!("{}", format!("{}", Node { name: Arc::new(Mutex::new(Some("row".to_string()))), next: Arc::new(Mutex::new(Some(Box::new(Label { x: Arc::new(Mutex::new(Some(7))), ..Default::default() }) as Box<dyn Any + Send + Sync>))), ..Default::default() }));
    show(Arc::new(Mutex::new(Some(Box::new(Node { name: Arc::new(Mutex::new(Some("row".to_string()))), next: Arc::new(Mutex::new(Some(Box::new(Label { x: Arc::new(Mutex::new(Some(7))), ..Default::default() }) as Box<dyn Any + Send + Sync>))), ..Default::default() }) as Box<dyn Any + Send + Sync>))));
    show(Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(Node { name: Arc::new(Mutex::new(Some("row".to_string()))), next: Arc::new(Mutex::new(Some(Box::new(Label { x: Arc::new(Mutex::new(Some(7))), ..Default::default() }) as Box<dyn Any + Send + Sync>))), ..Default::default() }))).clone()) as Box<dyn Any + Send + Sync>))));
    let mut labels = Arc::new(Mutex::new(Some(vec![Box::new(Label { x: Arc::new(Mutex::new(Some(8))), ..Default::default() }) as Box<dyn Any + Send + Sync>, Box::new(Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(2))), y: Arc::new(Mutex::new(Some(3))), ..Default::default() }))).clone()) as Box<dyn Any + Send + Sync>, Box::new(1i64) as Box<dyn Any + Send + Sync>])));
    println!("{} {}", format!("{}", "slice:".to_string()), format!("{}", (*shape(
        Arc::new(Mutex::new(Some(format!("{}", format_any_slice(&labels))))),
        Arc::new(Mutex::new(Some("[&{8 0} 0x".to_string())))
    ).lock().unwrap().as_ref().unwrap())));
    println!("{}", format_any_variadic(&labels));
    println!("{}", format!("{}", Outcome { value: Arc::new(Mutex::new(Some(4))), err: Arc::new(Mutex::new(Some(Arc::new(Missing { line: Arc::new(Mutex::new(Some(9))), ..Default::default() }) as Arc<dyn StdError + Send + Sync>))), ..Default::default() }));
    show(Arc::new(Mutex::new(Some(Box::new(Node { name: Arc::new(Mutex::new(Some("outer".to_string()))), next: Arc::new(Mutex::new(Some(Box::new(Outcome { value: Arc::new(Mutex::new(Some(5))), err: Arc::new(Mutex::new(Some(Arc::new(Missing { line: Arc::new(Mutex::new(Some(9))), ..Default::default() }) as Arc<dyn StdError + Send + Sync>))), ..Default::default() }) as Box<dyn Any + Send + Sync>))), ..Default::default() }) as Box<dyn Any + Send + Sync>))));
    show(Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(Status { code: Arc::new(Mutex::new(Some(404))), ..Default::default() }))).clone()) as Box<dyn Any + Send + Sync>))));
    show(Arc::new(Mutex::new(Some(Box::new(Node { name: Arc::new(Mutex::new(Some("n".to_string()))), next: Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(Status { code: Arc::new(Mutex::new(Some(5))), ..Default::default() }))).clone()) as Box<dyn Any + Send + Sync>))), ..Default::default() }) as Box<dyn Any + Send + Sync>))));
    show(Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(NotFound { name: Arc::new(Mutex::new(Some("y".to_string()))), ..Default::default() }))).clone()) as Box<dyn Any + Send + Sync>))));
    let mut noStatus: Arc<Mutex<Option<Status>>> = Arc::new(Mutex::new(None));
    show(Arc::new(Mutex::new(Some(Box::new(noStatus.clone()) as Box<dyn Any + Send + Sync>))));
    show(Arc::new(Mutex::new(Some(Box::new(Node { name: Arc::new(Mutex::new(Some("n".to_string()))), next: Arc::new(Mutex::new(Some(Box::new(noStatus.clone()) as Box<dyn Any + Send + Sync>))), ..Default::default() }) as Box<dyn Any + Send + Sync>))));
}

impl GoValueClone for Point {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Status {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for NotFound {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Node {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Label {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Missing {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Outcome {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Report {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
