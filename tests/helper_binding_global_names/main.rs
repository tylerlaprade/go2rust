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
    V: Display + Clone,
{
    format_map_nil(map, "<nil>")
}

// format_map_nil prints a value the map holds no value for as __nil_text:
// [] for a nil slice, as Go prints one.
fn format_map_nil<K: Display + Ord + Clone, V>(map: &Arc<Mutex<Option<BTreeMap<K, Arc<Mutex<Option<V>>>>>>>, __nil_text: &str) -> String
where
    V: Display + Clone,
{
    // The entries are copied out first, in key order, so a value's String
    // method that reads this map does not wait on the lock printing holds.
    let items: Option<Vec<(K, Arc<Mutex<Option<V>>>)>> = { let guard = map.lock().unwrap(); guard.as_ref().map(|m| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect()) };
    if let Some(items) = items {
        let formatted: Vec<String> = items
            .iter()
            .map(|(k, v)| {
                // The value is copied out too, so its String method may
                // read its own entry.
                let value = { let v_guard = v.lock().unwrap(); v_guard.clone() };
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
fn format_nested_slice<T, C, Inner>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[Inner]>,
    Inner: AsRef<[T]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
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
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<point>>>>() {
        go_format_any_pointer(nested, Arc::as_ptr(v) as *const (), || v.try_lock().map(|__guard| __guard.is_none()).unwrap_or(false), || match (*v.lock().unwrap()).as_ref() { Some(__pointee) => format!("&{}", __pointee), None => "<nil>".to_string() })
    } else if let Some(v) = value.downcast_ref::<GoNilAny>() {
        "<nil>".to_string()
    } else if let Some(v) = value.downcast_ref::<point>() {
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

fn go_any_eq(left: &Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>, right: &Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> bool {
    if Arc::ptr_eq(left, right) {
        let guard = left.lock().unwrap();
        return match guard.as_ref() {
            None => true,
            Some(value) => go_any_values_eq(value.as_ref(), value.as_ref()),
        };
    }
    let left_guard = left.lock().unwrap();
    let right_guard = right.lock().unwrap();
    match (left_guard.as_ref(), right_guard.as_ref()) {
        (None, None) => true,
        (None, Some(_)) | (Some(_), None) => false,
        (Some(left), Some(right)) => go_any_values_eq(left.as_ref(), right.as_ref()),
    }
}

fn go_any_values_eq(left: &(dyn Any + Send + Sync), right: &(dyn Any + Send + Sync)) -> bool {
    if left.type_id() != right.type_id() {
        return false;
    }
    if let Some(v) = left.downcast_ref::<i32>() { return right.downcast_ref::<i32>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<i64>() { return right.downcast_ref::<i64>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<i8>() { return right.downcast_ref::<i8>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<i16>() { return right.downcast_ref::<i16>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<u32>() { return right.downcast_ref::<u32>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<u64>() { return right.downcast_ref::<u64>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<u8>() { return right.downcast_ref::<u8>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<u16>() { return right.downcast_ref::<u16>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<usize>() { return right.downcast_ref::<usize>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<isize>() { return right.downcast_ref::<isize>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<f64>() { return right.downcast_ref::<f64>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<f32>() { return right.downcast_ref::<f32>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<String>() { return right.downcast_ref::<String>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<&str>() { return right.downcast_ref::<&str>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<bool>() { return right.downcast_ref::<bool>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<char>() { return right.downcast_ref::<char>().map_or(false, |r| v == r); }
    panic!("interface comparison with uncomparable dynamic type")
}

/// A nil interface value in a slot that holds a Box<dyn Any>, such as a
/// []any element: it copies, prints and compares as nil.
#[derive(Clone, Copy, Debug, PartialEq)]
struct GoNilAny;

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
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<point>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<GoNilAny>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<point>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
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

fn __go_type_name(val: &dyn Any) -> &'static str {
    if val.is::<i64>() { return "int" }
    if val.is::<i32>() { return "int32" }
    if val.is::<char>() { return "int32" }
    if val.is::<i8>() { return "int8" }
    if val.is::<i16>() { return "int16" }
    if val.is::<u64>() { return "uint" }
    if val.is::<u32>() { return "uint32" }
    if val.is::<u8>() { return "uint8" }
    if val.is::<u16>() { return "uint16" }
    if val.is::<f64>() { return "float64" }
    if val.is::<f32>() { return "float32" }
    if val.is::<bool>() { return "bool" }
    if val.is::<String>() { return "string" }
    if val.is::<Vec<i64>>() { return "[]int" }
    if val.is::<Vec<i32>>() { return "[]int32" }
    if val.is::<Vec<f64>>() { return "[]float64" }
    if val.is::<Vec<String>>() { return "[]string" }
    if val.is::<Vec<bool>>() { return "[]bool" }
    if val.is::<Arc<Mutex<Option<point>>>>() { return "*main.point" }
    if val.is::<GoNilAny>() { return "<nil>" }
    if val.is::<point>() { return "main.point" }
    std::any::type_name_of_val(val)
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

// An error that wraps others: fmt.Errorf with %w, or errors.Join. many is
// true when it wraps more than one, and then errors.Unwrap does not see it.
#[derive(Debug)]
struct GoWrapError {
    msg: String,
    wrapped: Vec<Arc<dyn StdError + Send + Sync>>,
    many: bool,
}

impl std::fmt::Display for GoWrapError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl StdError for GoWrapError {}

fn go_error_wrap(msg: String, wrapped: Vec<Option<Arc<dyn StdError + Send + Sync>>>) -> Arc<dyn StdError + Send + Sync> {
    let many = wrapped.len() > 1;
    Arc::new(GoWrapError { msg, wrapped: wrapped.into_iter().flatten().collect(), many }) as Arc<dyn StdError + Send + Sync>
}

fn go_errors_join(errs: Vec<Option<Arc<dyn StdError + Send + Sync>>>) -> Option<Arc<dyn StdError + Send + Sync>> {
    let errs: Vec<Arc<dyn StdError + Send + Sync>> = errs.into_iter().flatten().collect();
    if errs.is_empty() {
        return None;
    }
    let msg = errs.iter().map(|e| e.to_string()).collect::<Vec<_>>().join("\n");
    Some(Arc::new(GoWrapError { msg, wrapped: errs, many: true }) as Arc<dyn StdError + Send + Sync>)
}

// err and every error it wraps, depth first, in the order errors.Is and
// errors.As visit them.
fn go_error_chain(err: Option<Arc<dyn StdError + Send + Sync>>) -> Vec<Arc<dyn StdError + Send + Sync>> {
    let mut out = Vec::new();
    let mut stack: Vec<Arc<dyn StdError + Send + Sync>> = err.into_iter().collect();
    while let Some(e) = stack.pop() {
        if let Some(w) = e.downcast_ref::<GoWrapError>() {
            stack.extend(w.wrapped.iter().rev().cloned());
        }
        if let Some(__v) = e.downcast_ref::<strconv::atoi::NumError>() { if let Some(__c) = (*__v.unwrap().lock().unwrap()).clone() { stack.push(__c); } }
        out.push(e);
    }
    out
}

fn go_errors_unwrap(err: Option<Arc<dyn StdError + Send + Sync>>) -> Option<Arc<dyn StdError + Send + Sync>> {
    let err = err?;
    if let Some(w) = err.downcast_ref::<GoWrapError>() {
        return if w.many { None } else { w.wrapped.first().cloned() };
    }
    let e = err;
    let mut stack: Vec<Arc<dyn StdError + Send + Sync>> = Vec::new();
        if let Some(__v) = e.downcast_ref::<strconv::atoi::NumError>() { if let Some(__c) = (*__v.unwrap().lock().unwrap()).clone() { stack.push(__c); } }
    stack.pop()
}

// Two errors are the same error when they point to the same value, as ==
// on two errors is written.
fn go_errors_is(err: Option<Arc<dyn StdError + Send + Sync>>, target: Option<Arc<dyn StdError + Send + Sync>>) -> bool {
    match (err, target) {
        (None, None) => true,
        (Some(err), Some(target)) => go_error_chain(Some(err)).iter().any(|e| std::ptr::addr_eq(&**e, &*target)),
        _ => false,
    }
}

// Whether value has a String or an Error method. fmt prints such a value as
// the one text the method returns.
fn go_format_is_text(__value: &(dyn Any + Send + Sync)) -> bool {
    false
}

// The basic value a newtype such as "type Level int" holds, boxed.
fn go_format_named_basic_value(__value: &(dyn Any + Send + Sync)) -> Option<Box<dyn Any + Send + Sync>> {
    None
}

// value as %+v prints it, when it is a struct.
fn go_format_fields_value(__value: &(dyn Any + Send + Sync)) -> Option<String> {
    if let Some(__v) = __value.downcast_ref::<point>() {
        return Some(format!("{:#}", __v));
    }
    None
}

// The elements of value, boxed, when it is an array of basic values.
fn go_format_elements(__value: &(dyn Any + Send + Sync)) -> Option<Vec<Box<dyn Any + Send + Sync>>> {
    None
}

// A nil interface value passed to a format known only at run time. The
// arguments are boxes, which cannot be nil themselves.
#[allow(dead_code)]
struct GoFormatNil;

#[allow(unused_imports)]
mod __go_runtime_format {
    use super::{format_any, __go_type_name, go_format_is_text, go_format_named_basic_value, go_format_fields_value, go_format_elements, GoFormatNil, GoNilAny};
    use std::any::Any;
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::{Arc, Mutex};
    #[allow(unused_imports)]
    use super::{GoSlice, Vec};

// A verb with its flags, as fmt reads it from a format.
#[derive(Default, Clone, Copy)]
struct GoFormatSpec {
    plus: bool,
    minus: bool,
    sharp: bool,
    space: bool,
    zero: bool,
    width: Option<usize>,
    precision: Option<usize>,
}

// Pads text to the width of spec. A number passes the length of its sign so
// that zero padding goes between the sign and the digits.
fn go_format_pad(spec: &GoFormatSpec, text: String, sign_len: usize, zero_allowed: bool) -> String {
    let width = match spec.width {
        Some(width) => width,
        None => return text,
    };
    let len = text.chars().count();
    if len >= width {
        return text;
    }
    let fill = width - len;
    if spec.minus {
        return text + &" ".repeat(fill);
    }
    if spec.zero && zero_allowed {
        let (sign, digits) = text.split_at(sign_len);
        return format!("{}{}{}", sign, "0".repeat(fill), digits);
    }
    " ".repeat(fill) + &text
}

fn go_format_sign(spec: &GoFormatSpec, negative: bool) -> &'static str {
    if negative {
        "-"
    } else if spec.plus {
        "+"
    } else if spec.space {
        " "
    } else {
        ""
    }
}

// Whether strconv.IsPrint prints c as it is: a letter, mark, number,
// punctuation, symbol or the ASCII space. Rust's standard library has no
// table of Unicode categories, so this knows the control characters, the
// other spaces, the format characters in common use (the soft hyphen, the
// zero-width and direction marks, the byte order mark) and the private-use
// areas. It does not know which code points are unassigned.
fn go_format_is_print(c: char) -> bool {
    if c == ' ' {
        return true;
    }
    if c.is_control() || c.is_whitespace() {
        return false;
    }
    !matches!(c as u32, 0xad | 0x600..=0x605 | 0x61c | 0x6dd | 0x70f | 0x8e2 | 0x180e | 0x200b..=0x200f | 0x202a..=0x202e | 0x2060..=0x206f | 0xfeff | 0xfff9..=0xfffb | 0xe000..=0xf8ff | 0xe0001 | 0xe0020..=0xe007f | 0xf0000..=0xffffd | 0x100000..=0x10fffd)
}

// Writes text the way strconv.Quote writes the inside of a quoted string,
// with quote as the delimiter. ascii is the + flag: every character outside
// ASCII is written as an escape.
fn go_format_escape(out: &mut String, text: &str, quote: char, ascii: bool) {
    for c in text.chars() {
        match c {
            '\u{7}' => out.push_str("\\a"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{b}' => out.push_str("\\v"),
            '\\' => out.push_str("\\\\"),
            c if c == quote => {
                out.push('\\');
                out.push(c);
            }
            c if (c as u32) < 0x20 || c == '\u{7f}' => out.push_str(&format!("\\x{:02x}", c as u32)),
            c if !go_format_is_print(c) || (ascii && !c.is_ascii()) => {
                if (c as u32) < 0x10000 {
                    out.push_str(&format!("\\u{:04x}", c as u32));
                } else {
                    out.push_str(&format!("\\U{:08x}", c as u32));
                }
            }
            c => out.push(c),
        }
    }
}

fn go_format_quote(text: &str, quote: char, ascii: bool) -> String {
    let mut out = String::new();
    out.push(quote);
    go_format_escape(&mut out, text, quote, ascii);
    out.push(quote);
    out
}

fn go_format_integer(spec: &GoFormatSpec, verb: char, value: i128) -> Option<String> {
    let negative = value < 0;
    let magnitude = value.unsigned_abs();
    let mut digits = match verb {
        'd' | 'v' => magnitude.to_string(),
        'b' => format!("{:b}", magnitude),
        'o' | 'O' => format!("{:o}", magnitude),
        'x' => format!("{:x}", magnitude),
        'X' => format!("{:X}", magnitude),
        'c' => {
            let c = u32::try_from(value).ok().and_then(char::from_u32).unwrap_or('\u{fffd}');
            return Some(go_format_pad(spec, c.to_string(), 0, true));
        }
        'q' => {
            let c = u32::try_from(value).ok().and_then(char::from_u32).unwrap_or('\u{fffd}');
            return Some(go_format_pad(spec, go_format_quote(&c.to_string(), '\'', spec.plus), 0, true));
        }
        'U' => {
            // A precision above four asks for more digits.
            let digits = spec.precision.filter(|precision| *precision > 4).unwrap_or(4);
            let mut text = format!("U+{:0digits$X}", value as u64, digits = digits);
            if spec.sharp {
                if let Some(c) = u32::try_from(value).ok().and_then(char::from_u32).filter(|c| go_format_is_print(*c)) {
                    text.push_str(&format!(" '{}'", c));
                }
            }
            return Some(go_format_pad(spec, text, 0, false));
        }
        _ => return None,
    };
    let sign = go_format_sign(spec, negative);
    // A precision and zero padding both ask for a least number of digits.
    // The sign is counted in the width of zero padding; 0x, 0b and 0o are
    // not, they come on top.
    let least_digits = match spec.precision {
        // Zero with a precision of zero prints nothing but the padding.
        Some(0) if magnitude == 0 => return Some(go_format_pad(&GoFormatSpec { zero: false, ..*spec }, String::new(), 0, false)),
        Some(precision) => precision,
        None if spec.zero && !spec.minus => spec.width.unwrap_or(0).saturating_sub(sign.len()),
        None => 0,
    };
    if digits.len() < least_digits {
        digits = "0".repeat(least_digits - digits.len()) + &digits;
    }
    // The 0 that # puts before an octal number is only added to one that
    // does not start with a 0 already. %#O has it as well, after its 0o.
    let octal_zero = if spec.sharp && matches!(verb, 'o' | 'O') && !digits.starts_with('0') { "0" } else { "" };
    let prefix = match verb {
        'b' if spec.sharp => "0b",
        'O' => "0o",
        'x' if spec.sharp => "0x",
        'X' if spec.sharp => "0X",
        _ => "",
    };
    Some(go_format_pad(spec, format!("{}{}{}{}", sign, prefix, octal_zero, digits), 0, false))
}

// The digits of a float and the power of ten of the first one, from Rust's
// {:e} form, which rounds as strconv does.
fn go_format_float_digits(exponent_form: &str) -> (String, i32) {
    let (mantissa, exponent) = exponent_form.split_once('e').unwrap();
    let digits: String = mantissa.chars().filter(|c| c.is_ascii_digit()).collect();
    (digits, exponent.parse().unwrap())
}

fn go_format_float_exponent(digits: &str, exponent: i32, upper: bool) -> String {
    let mut out = String::new();
    out.push_str(&digits[..1]);
    if digits.len() > 1 {
        out.push('.');
        out.push_str(&digits[1..]);
    }
    out.push(if upper { 'E' } else { 'e' });
    out.push(if exponent < 0 { '-' } else { '+' });
    out.push_str(&format!("{:02}", exponent.abs()));
    out
}

// Formats the magnitude of a float as strconv.FormatFloat does for 'e', 'f'
// and 'g'. shortest is the float printed with as few digits as identify it.
fn go_format_float_magnitude(verb: char, precision: Option<usize>, shortest: &str, with_precision: impl Fn(usize) -> String, fixed: impl Fn(usize) -> String) -> String {
    let upper = verb.is_ascii_uppercase();
    match verb.to_ascii_lowercase() {
        'e' => {
            let (digits, exponent) = go_format_float_digits(&with_precision(precision.unwrap_or(6)));
            go_format_float_exponent(&digits, exponent, upper)
        }
        'f' => fixed(precision.unwrap_or(6)),
        _ => {
            let (digits, exponent, exponent_limit) = match precision {
                None => {
                    let (digits, exponent) = go_format_float_digits(shortest);
                    (digits, exponent, 6)
                }
                Some(precision) => {
                    let precision = precision.max(1);
                    let (digits, exponent) = go_format_float_digits(&with_precision(precision - 1));
                    let digits = digits.trim_end_matches('0');
                    let digits = if digits.is_empty() { "0".to_string() } else { digits.to_string() };
                    let mut limit = precision as i32;
                    if limit > digits.len() as i32 && digits.len() as i32 >= exponent + 1 {
                        limit = digits.len() as i32;
                    }
                    (digits, exponent, limit)
                }
            };
            if exponent < -4 || exponent >= exponent_limit {
                return go_format_float_exponent(&digits, exponent, upper);
            }
            if exponent < 0 {
                return format!("0.{}{}", "0".repeat((-exponent - 1) as usize), digits);
            }
            let whole = (exponent + 1) as usize;
            if digits.len() <= whole {
                return format!("{}{}", digits, "0".repeat(whole - digits.len()));
            }
            format!("{}.{}", &digits[..whole], &digits[whole..])
        }
    }
}

fn go_format_float(spec: &GoFormatSpec, verb: char, value: f64, single: bool) -> Option<String> {
    // fmt also writes a float in binary (%b) and in hex (%x). Neither is
    // done here.
    if matches!(verb, 'b' | 'x' | 'X') {
        panic!("go2rust: %{} of a float is not supported in a format that is only known at run time", verb);
    }
    if !matches!(verb, 'v' | 'e' | 'E' | 'f' | 'F' | 'g' | 'G') {
        return None;
    }
    if spec.sharp {
        panic!("go2rust: the # flag on %{} of a float is not supported in a format that is only known at run time", verb);
    }
    if value.is_nan() {
        let sign = if spec.plus { "+" } else if spec.space { " " } else { "" };
        return Some(go_format_pad(spec, format!("{}NaN", sign), 0, false));
    }
    let negative = value.is_sign_negative();
    if value.is_infinite() {
        // +Inf keeps its sign, unless the space flag asks for a space there.
        let sign = if negative { "-" } else if spec.space && !spec.plus { " " } else { "+" };
        return Some(go_format_pad(spec, format!("{}Inf", sign), 0, false));
    }
    let magnitude = value.abs();
    // A float32 keeps the digits of the float32, not of the float64 it was
    // widened to.
    let narrow = magnitude as f32;
    let shortest = if single { format!("{:e}", narrow) } else { format!("{:e}", magnitude) };
    let with_precision = |precision: usize| if single { format!("{:.*e}", precision, narrow) } else { format!("{:.*e}", precision, magnitude) };
    let fixed = |precision: usize| if single { format!("{:.*}", precision, narrow) } else { format!("{:.*}", precision, magnitude) };
    let verb = if verb == 'v' { 'g' } else { verb };
    let digits = go_format_float_magnitude(verb, spec.precision, &shortest, with_precision, fixed);
    let sign = go_format_sign(spec, negative);
    Some(go_format_pad(spec, format!("{}{}", sign, digits), sign.len(), true))
}

// %x and %X of a string or a []byte: two digits for each byte. A precision
// counts bytes here, not characters.
fn go_format_hex(spec: &GoFormatSpec, verb: char, bytes: &[u8]) -> String {
    let bytes = match spec.precision {
        Some(precision) if precision < bytes.len() => &bytes[..precision],
        _ => bytes,
    };
    let prefix = if !spec.sharp { "" } else if verb == 'x' { "0x" } else { "0X" };
    let digits: Vec<String> = bytes.iter().map(|b| if verb == 'x' { format!("{:02x}", b) } else { format!("{:02X}", b) }).collect();
    let text = if spec.space {
        digits.iter().map(|d| format!("{}{}", prefix, d)).collect::<Vec<_>>().join(" ")
    } else if digits.is_empty() {
        String::new()
    } else {
        format!("{}{}", prefix, digits.concat())
    };
    go_format_pad(spec, text, 0, true)
}

fn go_format_string(spec: &GoFormatSpec, verb: char, text: &str) -> Option<String> {
    if matches!(verb, 'x' | 'X') {
        return Some(go_format_hex(spec, verb, text.as_bytes()));
    }
    let truncated: String = match spec.precision {
        Some(precision) => text.chars().take(precision).collect(),
        None => text.to_string(),
    };
    let formatted = match verb {
        's' | 'v' => truncated,
        'q' => {
            // strconv.CanBackquote: no backquote, no ASCII control character
            // but tab, and no byte order mark.
            let raw = truncated.chars().all(|c| if c.is_ascii() { (c >= ' ' || c == '\t') && c != '\u{60}' && c != '\u{7f}' } else { c != '\u{feff}' });
            if spec.sharp && raw {
                format!("\u{60}{}\u{60}", truncated)
            } else {
                go_format_quote(&truncated, '"', spec.plus)
            }
        }
        _ => return None,
    };
    Some(go_format_pad(spec, formatted, 0, true))
}

// A []byte prints as text for the verbs that print a string.
fn go_format_bytes(spec: &GoFormatSpec, verb: char, bytes: &[u8]) -> Option<String> {
    if matches!(verb, 'x' | 'X') {
        return Some(go_format_hex(spec, verb, bytes));
    }
    if !matches!(verb, 's' | 'q') {
        return None;
    }
    // A precision counts characters, and a byte that is not UTF-8 counts as
    // one. It is applied first: what is cut off has no say in how the rest
    // is written.
    let mut kept = bytes.len();
    if let Some(precision) = spec.precision {
        let (mut count, mut end) = (0, 0);
        'chunks: for chunk in bytes.utf8_chunks() {
            for c in chunk.valid().chars() {
                if count == precision {
                    break 'chunks;
                }
                count += 1;
                end += c.len_utf8();
            }
            for _ in chunk.invalid() {
                if count == precision {
                    break 'chunks;
                }
                count += 1;
                end += 1;
            }
        }
        kept = end;
    }
    let bytes = &bytes[..kept];
    let spec = &GoFormatSpec { precision: None, ..*spec };
    match std::str::from_utf8(bytes) {
        Ok(text) => go_format_string(spec, verb, text),
        // %q writes a byte that is not UTF-8 as an escape.
        Err(_) if verb == 'q' => {
            let mut quoted = String::from("\"");
            for chunk in bytes.utf8_chunks() {
                go_format_escape(&mut quoted, chunk.valid(), '"', spec.plus);
                for byte in chunk.invalid() {
                    quoted.push_str(&format!("\\x{:02x}", byte));
                }
            }
            quoted.push('"');
            Some(go_format_pad(spec, quoted, 0, true))
        }
        // %s would have to write the byte as it is, and the result is a Rust
        // String, which cannot hold it.
        Err(_) => panic!("go2rust: %s of a []byte that is not UTF-8 is not supported in a format that is only known at run time"),
    }
}

// Formats one argument for one verb. None means the verb does not apply to
// the argument's type, which fmt reports in the output.
fn go_format_argument(spec: &GoFormatSpec, verb: char, value: &(dyn Any + Send + Sync)) -> Option<String> {
    if verb == 'T' {
        return Some(go_format_pad(spec, __go_type_name(value).to_string(), 0, true));
    }
    if verb == 'p' {
        panic!("go2rust: %p is not supported in a format that is only known at run time");
    }
    // %+v and %#v ask for field names and Go syntax. A number or a bool has
    // neither, so it prints as %v prints it, with two exceptions for %#v: an
    // unsigned integer is written in hex and a string in quotes.
    let (plus_v, sharp_v) = (verb == 'v' && spec.plus, verb == 'v' && spec.sharp);
    let basic = GoFormatSpec { plus: spec.plus && !plus_v, sharp: spec.sharp && !sharp_v, ..*spec };
    macro_rules! signed {
        ($($t:ty),*) => {
            $(if let Some(v) = value.downcast_ref::<$t>() {
                return go_format_integer(&basic, verb, *v as i128);
            })*
        };
    }
    signed!(i8, i16, i32, i64, isize);
    // A rune taken from a range over a string can be a Rust char; it is a
    // Go rune, an int32.
    if let Some(v) = value.downcast_ref::<char>() {
        return go_format_integer(&basic, verb, *v as i128);
    }
    macro_rules! unsigned {
        ($($t:ty),*) => {
            $(if let Some(v) = value.downcast_ref::<$t>() {
                if sharp_v {
                    return go_format_integer(&GoFormatSpec { sharp: true, ..basic }, 'x', *v as i128);
                }
                return go_format_integer(&basic, verb, *v as i128);
            })*
        };
    }
    unsigned!(u8, u16, u32, u64, usize);
    if let Some(v) = value.downcast_ref::<f64>() {
        return go_format_float(&basic, verb, *v, false);
    }
    if let Some(v) = value.downcast_ref::<f32>() {
        return go_format_float(&basic, verb, *v as f64, true);
    }
    if let Some(v) = value.downcast_ref::<bool>() {
        return if matches!(verb, 't' | 'v') { Some(go_format_pad(&basic, v.to_string(), 0, true)) } else { None };
    }
    let string_verb = if sharp_v { 'q' } else { verb };
    if let Some(v) = value.downcast_ref::<String>() {
        return go_format_string(&basic, string_verb, v);
    }
    if let Some(v) = value.downcast_ref::<&str>() {
        return go_format_string(&basic, string_verb, v);
    }
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Vec<u8>>>>>() {
        let guard = v.lock().unwrap();
        let bytes: &[u8] = guard.as_deref().unwrap_or(&[]);
        if let Some(text) = go_format_bytes(spec, verb, bytes) {
            return Some(text);
        }
        // Any other verb is applied to each byte, between brackets. A verb
        // that does not fit a number is reported for each byte. %#v writes
        // the slice as Go source, which is not done here.
        if !sharp_v {
            let items: Vec<String> = bytes.iter().map(|byte| go_format_integer(&basic, verb, *byte as i128).unwrap_or_else(|| format!("%!{}(uint8={})", verb, go_format_integer(&basic, 'd', *byte as i128).unwrap_or_default()))).collect();
            return Some(format!("[{}]", items.join(" ")));
        }
    }
    // A slice of strings, numbers or bools: the verb, with its width and
    // flags, is applied to each element, between brackets, as fmt does
    // (%q of []string is ["a" "b"]). An element the verb does not fit is
    // reported in its place. A plain %v is left to Println's form below.
    let plain_v = verb == 'v' && spec.width.is_none() && spec.precision.is_none() && !spec.plus && !spec.sharp && !spec.space;
    macro_rules! basic_slice {
        ($($t:ty),*) => {
            $(if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Vec<$t>>>>>() {
                if !plain_v && !sharp_v {
                    let guard = v.lock().unwrap();
                    let items: Vec<String> = guard.as_ref().map(|values| values.iter().map(|item| {
                        let item: &(dyn Any + Send + Sync) = item;
                        go_format_argument(spec, verb, item).unwrap_or_else(|| format!("%!{}({})", verb, go_format_typed_value(spec, item)))
                    }).collect()).unwrap_or_default();
                    return Some(format!("[{}]", items.join(" ")));
                }
            })*
        };
    }
    basic_slice!(String, i64, i32, i16, i8, u64, u32, u16, f64, f32, bool);
    // An array of them the same way. An array of bytes is the text or the
    // hex of those bytes for %s, %q, %x and %X, as a []byte is.
    if !plain_v && !sharp_v {
        if let Some(items) = go_format_elements(value) {
            let bytes: Option<Vec<u8>> = items.iter().map(|item| item.downcast_ref::<u8>().copied()).collect();
            if let Some(bytes) = bytes.filter(|bytes| !bytes.is_empty()) {
                if let Some(text) = go_format_bytes(spec, verb, &bytes) {
                    return Some(text);
                }
            }
            let items: Vec<String> = items.iter().map(|item| {
                let item: &(dyn Any + Send + Sync) = item.as_ref();
                go_format_argument(spec, verb, item).unwrap_or_else(|| format!("%!{}({})", verb, go_format_typed_value(spec, item)))
            }).collect();
            return Some(format!("[{}]", items.join(" ")));
        }
    }
    // A value with a String or an Error method is the text that method
    // returns, for the verbs that print a string.
    if go_format_is_text(value) && !sharp_v && matches!(verb, 'v' | 's' | 'q' | 'x' | 'X') {
        return go_format_string(&basic, verb, &format_any(value));
    }
    if let Some(inner) = go_format_named_basic_value(value) {
        return go_format_argument(spec, verb, inner.as_ref());
    }
    // Any other value prints as Println prints it, for a plain %v. fmt
    // applies a width, a precision, a flag or another verb to each field or
    // element, which is not done here. nil is one text.
    if plus_v && spec.width.is_none() && spec.precision.is_none() && !spec.space {
        if let Some(text) = go_format_fields_value(value) {
            return Some(text);
        }
    }
    if verb == 'v' && !plus_v && !sharp_v {
        let text = format_any(value);
        if text == "<nil>" {
            return Some(go_format_pad(spec, text, 0, true));
        }
        if spec.width.is_none() && spec.precision.is_none() && !spec.space {
            return Some(text);
        }
    }
    panic!("go2rust: %{}{}{} of a {} is not supported in a format that is only known at run time", if spec.plus { "+" } else { "" }, if spec.sharp { "#" } else { "" }, verb, __go_type_name(value));
}

// The number after % or after the dot: digits, or * for the next argument.
// Ok(None) is no number. Err is a * whose argument is not a usable integer.
fn go_format_number(chars: &mut std::iter::Peekable<std::str::Chars>, args: &[Box<dyn Any + Send + Sync>], next: &mut usize) -> Result<Option<i64>, ()> {
    if chars.peek() == Some(&'*') {
        chars.next();
        let arg: &(dyn Any + Send + Sync) = match args.get(*next) {
            Some(arg) => arg.as_ref(),
            None => return Err(()),
        };
        *next += 1;
        // fmt takes any integer type here.
        let mut value: Option<i128> = None;
        macro_rules! integer {
            ($($t:ty),*) => {
                $(if let Some(v) = arg.downcast_ref::<$t>() {
                    value = Some(*v as i128);
                })*
            };
        }
        integer!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
        return match value {
            Some(value) if value.abs() <= 1_000_000 => Ok(Some(value as i64)),
            _ => Err(()),
        };
    }
    let mut value: Option<i64> = None;
    while let Some(digit) = chars.peek().and_then(|c| c.to_digit(10)) {
        value = Some(value.unwrap_or(0).saturating_mul(10).saturating_add(digit as i64));
        chars.next();
    }
    Ok(value.filter(|value| *value <= 1_000_000))
}

// Whether value is of a type that go_format_argument formats itself.
fn go_format_is_basic(value: &(dyn Any + Send + Sync)) -> bool {
    macro_rules! is_one_of {
        ($($t:ty),*) => { false $(|| value.is::<$t>())* };
    }
    is_one_of!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, bool, String, &str, Arc<Mutex<Option<Vec<u8>>>>)
}

// A nil interface value: the nil literal, or a nil interface value boxed as
// the program boxes one.
fn go_format_is_nil(value: &(dyn Any + Send + Sync)) -> bool {
    value.is::<GoFormatNil>() || value.is::<GoNilAny>()
}

// An argument as fmt names it when it does not fit the format: type=value.
// The value is written as %v writes it, with the width, the precision and
// the flags of the verb that did not fit.
fn go_format_typed_value(spec: &GoFormatSpec, value: &(dyn Any + Send + Sync)) -> String {
    if go_format_is_nil(value) {
        return "<nil>".to_string();
    }
    let text = go_format_argument(spec, go_format_plain_verb(value), value).unwrap_or_default();
    format!("{}={}", __go_type_name(value), text)
}

// The verb that writes value as %v does and leaves the flags as they are.
// %v itself would read + and # as %+v and %#v.
fn go_format_plain_verb(value: &(dyn Any + Send + Sync)) -> char {
    macro_rules! is_one_of {
        ($($t:ty),*) => { false $(|| value.is::<$t>())* };
    }
    if is_one_of!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize) {
        'd'
    } else if is_one_of!(f32, f64) {
        'g'
    } else if value.is::<bool>() {
        't'
    } else if is_one_of!(String, &str) {
        's'
    } else {
        'v'
    }
}

// fmt.Sprintf for a format that is only known when the program runs. A
// format written as a literal is expanded when the program is translated and
// does not come here. wraps is true for fmt.Errorf, where %w prints as %v.
pub(super) fn go_sprintf(format: &str, args: &[Box<dyn Any + Send + Sync>], wraps: bool) -> String {
    let mut out = String::new();
    let mut next = 0;
    let mut chars = format.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '%' {
            out.push(c);
            continue;
        }
        let mut spec = GoFormatSpec::default();
        loop {
            match chars.peek() {
                Some('+') => spec.plus = true,
                Some('-') => spec.minus = true,
                Some('#') => spec.sharp = true,
                Some(' ') => spec.space = true,
                Some('0') => spec.zero = true,
                _ => break,
            }
            chars.next();
        }
        if chars.peek() == Some(&'[') {
            panic!("go2rust: an argument index such as %[1]d is not supported in a format that is only known at run time");
        }
        match go_format_number(&mut chars, args, &mut next) {
            // A negative width from an argument means left alignment.
            Ok(Some(width)) if width < 0 => {
                spec.minus = true;
                spec.width = Some((-width) as usize);
            }
            Ok(width) => spec.width = width.map(|width| width as usize),
            Err(()) => out.push_str("%!(BADWIDTH)"),
        }
        if chars.peek() == Some(&'.') {
            chars.next();
            match go_format_number(&mut chars, args, &mut next) {
                Ok(Some(precision)) if precision < 0 => out.push_str("%!(BADPREC)"),
                Ok(precision) => spec.precision = Some(precision.unwrap_or(0) as usize),
                Err(()) => out.push_str("%!(BADPREC)"),
            }
        }
        if chars.peek() == Some(&'[') {
            panic!("go2rust: an argument index such as %[1]d is not supported in a format that is only known at run time");
        }
        let verb = match chars.next() {
            Some(verb) => verb,
            None => {
                out.push_str("%!(NOVERB)");
                break;
            }
        };
        if verb == '%' {
            out.push('%');
            continue;
        }
        let arg = match args.get(next) {
            Some(arg) => arg,
            None => {
                out.push_str(&format!("%!{}(MISSING)", verb));
                continue;
            }
        };
        next += 1;
        // In fmt.Errorf %w prints an error as %v does. A number, a string, a
        // bool or a []byte is not an error, and fmt reports the verb: once
        // for a scalar, and for each byte of a []byte.
        // A nil interface value prints <nil> for %v and %T, and fmt reports
        // any other verb.
        if go_format_is_nil(arg.as_ref()) {
            if matches!(verb, 'v' | 'T') {
                out.push_str(&go_format_pad(&spec, "<nil>".to_string(), 0, true));
            } else {
                out.push_str(&format!("%!{}(<nil>)", verb));
            }
            continue;
        }
        let verb = if verb == 'w' && wraps && !go_format_is_basic(arg.as_ref()) { 'v' } else { verb };
        match go_format_argument(&spec, verb, arg.as_ref()) {
            Some(text) => out.push_str(&text),
            None => out.push_str(&format!("%!{}({})", verb, go_format_typed_value(&spec, arg.as_ref()))),
        }
    }
    if next < args.len() {
        let extra: Vec<String> = args[next..].iter().map(|arg| go_format_typed_value(&GoFormatSpec::default(), arg.as_ref())).collect();
        out.push_str(&format!("%!(EXTRA {})", extra.join(", ")));
    }
    out
}
}
use __go_runtime_format::go_sprintf;

#[derive(Debug, Clone)]
pub struct point {
    pub x: Arc<Mutex<Option<i64>>>,
    pub y: Arc<Mutex<Option<i64>>>,
}

impl point {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.y.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
            y: __go_clone_0_1,
        }
    }
}


impl Default for point {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_0_1 = Arc::new(Mutex::new(Some(0)));
        Self {
            x: __go_default_0_0,
            y: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_plus = f.alternate();
        let __go_fmt_0 = if __go_plus { format!("{:#}", (*self.x.lock().unwrap().as_ref().unwrap())) } else { format!("{}", (*self.x.lock().unwrap().as_ref().unwrap())) };
        let __go_fmt_1 = if __go_plus { format!("{:#}", (*self.y.lock().unwrap().as_ref().unwrap())) } else { format!("{}", (*self.y.lock().unwrap().as_ref().unwrap())) };
        if __go_plus {
            return write!(f, "{{X:{} Y:{}}}", __go_fmt_0, __go_fmt_1);
        }
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub(crate) static a_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static absolute_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static accept_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static access_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static actual_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static addr_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static address_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static after_mod_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static any_val_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static arg_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static arg0_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static arg1_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static args_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static array_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static as_error_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static ascii_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static assign_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static at_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static b_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static b0_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static b1_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static b2_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static base_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static base0_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static basic_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static before_query_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static bit_len_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static bit_size_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static body_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static bool_error_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static borrow_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static boxed_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static bytes_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static c_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static c0_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static c1_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static c2_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static c3_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static call_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static callback_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static cancel_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static cancel_cancelled_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static cancel_done_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static cancel_err_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static cancelled_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static candidate_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static candidate_text_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static candidates_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static capture_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static cell_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static channel_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static char_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static chars_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static child_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static chunk_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static clean_path_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static cleaned_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static clone_value_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static closed_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static cmd_env_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static code_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static command_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static component_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static context_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static count_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static ctx_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static current_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static cvar_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static cw_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static d_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static data_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static day_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static days_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static delta_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static depth_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static digit_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static digits_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static dir_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static dirfd_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static disk_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static doe_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static done_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static doy_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static drain_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static drained_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static duration_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static e_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static elem_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static elem_guard_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static elem_kind_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static embedded_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static embedded_ref_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static end_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static entries_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static entry_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static env_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static eof_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static era_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static err_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static errno_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static errs_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static escape_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static escaped_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static exp_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static exp_text_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static expand_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static exponent_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static exponent_form_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static exponent_limit_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static extra_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static f_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static fd_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static field_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static field_value_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static file_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static file_path_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static file_type_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static fill_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static fire_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static first_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static fixed_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static flag_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static flags_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static fmtc_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static forget_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static format_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static formatted_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static frac_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static fraction_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static from_relative_path_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static g_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static generation_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static getter_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static getter_guard_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static go_format_integer_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static go_index_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static guard_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static h_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static handle_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static has_more_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static has_separator_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static have_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static have_digits_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static held_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static hex_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static hh_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static hi_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static high_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static host_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static hour_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static identity_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static in_string_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static index_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static inner_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static inner_guard_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static input_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static inputc_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static io_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static is_dir_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static is_nil_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static item_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static items_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static j_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static kept_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static key_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static label_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static late_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static least_digits_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static left_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static left_guard_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static r#let: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static limit_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static list_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static lo_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static lock_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static locked_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static r#loop: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static low_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static lower_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static m_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static magnitude_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static maj_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static mantissa_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static many_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static map_key_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static marker_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static matched_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static matches_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static message_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static metadata_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static method_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static minus_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static minute_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static mod_index_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static mode_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static month_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static mp_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static msg_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static n_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static name_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static nanos_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static narrow_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static native_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static negative_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static nested_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static new_val_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static newlines_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static next_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static next_byte_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static nextc_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static nl_is_end_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static nl_is_space_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static now_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static object_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static octal_zero_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static offset_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static ok_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static ok_verbs_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static old_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static op_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static ops_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static options_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static other_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static out_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static outer_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static outer_index_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static output_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static owner_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static p_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static pad2_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static pad3_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static parent_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static path_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static path_error_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static pattern_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static payload_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static pending_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static period_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static perm_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static pipe_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static plain_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static plain_v_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static plus_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static plus_v_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static poisoned_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static pos_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static precision_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static prefix_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static previous_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static ptr_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static quote_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static quoted_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static r_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static raw_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static raw_query_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static read_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static read_all_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static ready_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static registry_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static relative_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static repl_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static rest_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static result_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static right_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static right_guard_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static rows_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static rs_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static rx_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static s_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static s0_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static s1_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static saw_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static scale_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static scheme_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static second_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static seconds_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static secs_of_day_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static seed_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static seen_point_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static sender_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static setter_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static setter_guard_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static shared_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static sharp_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static sharp_v_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static shift_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static shortest_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static sign_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static sign_len_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static simple_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static single_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static skip_space_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static slash_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static slice_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static slot_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static source_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static space_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static spec_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static src_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static stack_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static start_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static state_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static state_ref_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static status_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static std_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static stderr_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static stdin_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static stdout_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static step_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static stopped_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static stored_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static string_verb_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static suffix_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static syntax_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static t_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static table_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static target_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static target_code_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static targets_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static temp1_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static temp2_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static text_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static text_ref_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static thread_channel_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static thread_stopped_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static timeout_cancelled_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static timeout_done_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static timeout_err_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static timer_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static tok_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static trailing_space_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static truncated_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static tx_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static typ_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static typed_val_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static underscores_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static unread_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static unsigned_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static upper_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static r#use: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static utf8_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static v_guard_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static val_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static value_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static value_start_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static values_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static verb_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static w_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static wait_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static want_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static was_active_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static was_stopped_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static when_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static whole_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static wid_present_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static width_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static with_mut_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static with_precision_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static word_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static wrapped_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static wraps_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static write_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static y_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static year_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static yoe_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static z_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static zero_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static zero_allowed_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *a_1.lock().unwrap() = Some(0);
    *absolute_1.lock().unwrap() = Some(0);
    *accept_1.lock().unwrap() = Some(0);
    *access_1.lock().unwrap() = Some(0);
    *actual_1.lock().unwrap() = Some(0);
    *addr_1.lock().unwrap() = Some(0);
    *address_1.lock().unwrap() = Some(0);
    *after_mod_1.lock().unwrap() = Some(0);
    *any_val_1.lock().unwrap() = Some(0);
    *arg_1.lock().unwrap() = Some(0);
    *arg0_1.lock().unwrap() = Some(0);
    *arg1_1.lock().unwrap() = Some(0);
    *args_1.lock().unwrap() = Some(0);
    *array_1.lock().unwrap() = Some(0);
    *as_error_1.lock().unwrap() = Some(0);
    *ascii_1.lock().unwrap() = Some(0);
    *assign_1.lock().unwrap() = Some(0);
    *at_1.lock().unwrap() = Some(0);
    *b_1.lock().unwrap() = Some(0);
    *b0_1.lock().unwrap() = Some(0);
    *b1_1.lock().unwrap() = Some(0);
    *b2_1.lock().unwrap() = Some(0);
    *base_1.lock().unwrap() = Some(0);
    *base0_1.lock().unwrap() = Some(0);
    *basic_1.lock().unwrap() = Some(0);
    *before_query_1.lock().unwrap() = Some(0);
    *bit_len_1.lock().unwrap() = Some(0);
    *bit_size_1.lock().unwrap() = Some(0);
    *body_1.lock().unwrap() = Some(0);
    *bool_error_1.lock().unwrap() = Some(0);
    *borrow_1.lock().unwrap() = Some(0);
    *boxed_1.lock().unwrap() = Some(0);
    *bytes_1.lock().unwrap() = Some(0);
    *c_1.lock().unwrap() = Some(0);
    *c0_1.lock().unwrap() = Some(0);
    *c1_1.lock().unwrap() = Some(0);
    *c2_1.lock().unwrap() = Some(0);
    *c3_1.lock().unwrap() = Some(0);
    *call_1.lock().unwrap() = Some(0);
    *callback_1.lock().unwrap() = Some(0);
    *cancel_1.lock().unwrap() = Some(0);
    *cancel_cancelled_1.lock().unwrap() = Some(0);
    *cancel_done_1.lock().unwrap() = Some(0);
    *cancel_err_1.lock().unwrap() = Some(0);
    *cancelled_1.lock().unwrap() = Some(0);
    *candidate_1.lock().unwrap() = Some(0);
    *candidate_text_1.lock().unwrap() = Some(0);
    *candidates_1.lock().unwrap() = Some(0);
    *capture_1.lock().unwrap() = Some(0);
    *cell_1.lock().unwrap() = Some(0);
    *channel_1.lock().unwrap() = Some(0);
    *char_1.lock().unwrap() = Some(0);
    *chars_1.lock().unwrap() = Some(0);
    *child_1.lock().unwrap() = Some(0);
    *chunk_1.lock().unwrap() = Some(0);
    *clean_path_1.lock().unwrap() = Some(0);
    *cleaned_1.lock().unwrap() = Some(0);
    *clone_value_1.lock().unwrap() = Some(0);
    *closed_1.lock().unwrap() = Some(0);
    *cmd_env_1.lock().unwrap() = Some(0);
    *code_1.lock().unwrap() = Some(0);
    *command_1.lock().unwrap() = Some(0);
    *component_1.lock().unwrap() = Some(0);
    *context_1.lock().unwrap() = Some(0);
    *count_1.lock().unwrap() = Some(0);
    *ctx_1.lock().unwrap() = Some(0);
    *current_1.lock().unwrap() = Some(0);
    *cvar_1.lock().unwrap() = Some(0);
    *cw_1.lock().unwrap() = Some(0);
    *d_1.lock().unwrap() = Some(0);
    *data_1.lock().unwrap() = Some(0);
    *day_1.lock().unwrap() = Some(0);
    *days_1.lock().unwrap() = Some(0);
    *delta_1.lock().unwrap() = Some(0);
    *depth_1.lock().unwrap() = Some(0);
    *digit_1.lock().unwrap() = Some(0);
    *digits_1.lock().unwrap() = Some(0);
    *dir_1.lock().unwrap() = Some(0);
    *dirfd_1.lock().unwrap() = Some(0);
    *disk_1.lock().unwrap() = Some(0);
    *doe_1.lock().unwrap() = Some(0);
    *done_1.lock().unwrap() = Some(0);
    *doy_1.lock().unwrap() = Some(0);
    *drain_1.lock().unwrap() = Some(0);
    *drained_1.lock().unwrap() = Some(0);
    *duration_1.lock().unwrap() = Some(0);
    *e_1.lock().unwrap() = Some(0);
    *elem_1.lock().unwrap() = Some(0);
    *elem_guard_1.lock().unwrap() = Some(0);
    *elem_kind_1.lock().unwrap() = Some(0);
    *embedded_1.lock().unwrap() = Some(0);
    *embedded_ref_1.lock().unwrap() = Some(0);
    *end_1.lock().unwrap() = Some(0);
    *entries_1.lock().unwrap() = Some(0);
    *entry_1.lock().unwrap() = Some(0);
    *env_1.lock().unwrap() = Some(0);
    *eof_1.lock().unwrap() = Some(0);
    *era_1.lock().unwrap() = Some(0);
    *err_1.lock().unwrap() = Some(0);
    *errno_1.lock().unwrap() = Some(0);
    *errs_1.lock().unwrap() = Some(0);
    *escape_1.lock().unwrap() = Some(0);
    *escaped_1.lock().unwrap() = Some(0);
    *exp_1.lock().unwrap() = Some(0);
    *exp_text_1.lock().unwrap() = Some(0);
    *expand_1.lock().unwrap() = Some(0);
    *exponent_1.lock().unwrap() = Some(0);
    *exponent_form_1.lock().unwrap() = Some(0);
    *exponent_limit_1.lock().unwrap() = Some(0);
    *extra_1.lock().unwrap() = Some(0);
    *f_1.lock().unwrap() = Some(0);
    *fd_1.lock().unwrap() = Some(0);
    *field_1.lock().unwrap() = Some(0);
    *field_value_1.lock().unwrap() = Some(0);
    *file_1.lock().unwrap() = Some(0);
    *file_path_1.lock().unwrap() = Some(0);
    *file_type_1.lock().unwrap() = Some(0);
    *fill_1.lock().unwrap() = Some(0);
    *fire_1.lock().unwrap() = Some(0);
    *first_1.lock().unwrap() = Some(0);
    *fixed_1.lock().unwrap() = Some(0);
    *flag_1.lock().unwrap() = Some(0);
    *flags_1.lock().unwrap() = Some(0);
    *fmtc_1.lock().unwrap() = Some(0);
    *forget_1.lock().unwrap() = Some(0);
    *format_1.lock().unwrap() = Some(0);
    *formatted_1.lock().unwrap() = Some(0);
    *frac_1.lock().unwrap() = Some(0);
    *fraction_1.lock().unwrap() = Some(0);
    *from_relative_path_1.lock().unwrap() = Some(0);
    *g_1.lock().unwrap() = Some(0);
    *generation_1.lock().unwrap() = Some(0);
    *getter_1.lock().unwrap() = Some(0);
    *getter_guard_1.lock().unwrap() = Some(0);
    *go_format_integer_1.lock().unwrap() = Some(0);
    *go_index_1.lock().unwrap() = Some(0);
    *guard_1.lock().unwrap() = Some(0);
    *h_1.lock().unwrap() = Some(0);
    *handle_1.lock().unwrap() = Some(0);
    *has_more_1.lock().unwrap() = Some(0);
    *has_separator_1.lock().unwrap() = Some(0);
    *have_1.lock().unwrap() = Some(0);
    *have_digits_1.lock().unwrap() = Some(0);
    *held_1.lock().unwrap() = Some(0);
    *hex_1.lock().unwrap() = Some(0);
    *hh_1.lock().unwrap() = Some(0);
    *hi_1.lock().unwrap() = Some(0);
    *high_1.lock().unwrap() = Some(0);
    *host_1.lock().unwrap() = Some(0);
    *hour_1.lock().unwrap() = Some(0);
    *identity_1.lock().unwrap() = Some(0);
    *in_string_1.lock().unwrap() = Some(0);
    *index_1.lock().unwrap() = Some(0);
    *inner_1.lock().unwrap() = Some(0);
    *inner_guard_1.lock().unwrap() = Some(0);
    *input_1.lock().unwrap() = Some(0);
    *inputc_1.lock().unwrap() = Some(0);
    *io_1.lock().unwrap() = Some(0);
    *is_dir_1.lock().unwrap() = Some(0);
    *is_nil_1.lock().unwrap() = Some(0);
    *item_1.lock().unwrap() = Some(0);
    *items_1.lock().unwrap() = Some(0);
    *j_1.lock().unwrap() = Some(0);
    *kept_1.lock().unwrap() = Some(0);
    *key_1.lock().unwrap() = Some(0);
    *label_1.lock().unwrap() = Some(0);
    *late_1.lock().unwrap() = Some(0);
    *least_digits_1.lock().unwrap() = Some(0);
    *left_1.lock().unwrap() = Some(0);
    *left_guard_1.lock().unwrap() = Some(0);
    *r#let.lock().unwrap() = Some(0);
    *limit_1.lock().unwrap() = Some(0);
    *list_1.lock().unwrap() = Some(0);
    *lo_1.lock().unwrap() = Some(0);
    *lock_1.lock().unwrap() = Some(0);
    *locked_1.lock().unwrap() = Some(0);
    *r#loop.lock().unwrap() = Some(0);
    *low_1.lock().unwrap() = Some(0);
    *lower_1.lock().unwrap() = Some(0);
    *m_1.lock().unwrap() = Some(0);
    *magnitude_1.lock().unwrap() = Some(0);
    *maj_1.lock().unwrap() = Some(0);
    *mantissa_1.lock().unwrap() = Some(0);
    *many_1.lock().unwrap() = Some(0);
    *map_key_1.lock().unwrap() = Some(0);
    *marker_1.lock().unwrap() = Some(0);
    *matched_1.lock().unwrap() = Some(0);
    *matches_1.lock().unwrap() = Some(0);
    *message_1.lock().unwrap() = Some(0);
    *metadata_1.lock().unwrap() = Some(0);
    *method_1.lock().unwrap() = Some(0);
    *minus_1.lock().unwrap() = Some(0);
    *minute_1.lock().unwrap() = Some(0);
    *mod_index_1.lock().unwrap() = Some(0);
    *mode_1.lock().unwrap() = Some(0);
    *month_1.lock().unwrap() = Some(0);
    *mp_1.lock().unwrap() = Some(0);
    *msg_1.lock().unwrap() = Some(0);
    *n_1.lock().unwrap() = Some(0);
    *name_1.lock().unwrap() = Some(0);
    *nanos_1.lock().unwrap() = Some(0);
    *narrow_1.lock().unwrap() = Some(0);
    *native_1.lock().unwrap() = Some(0);
    *negative_1.lock().unwrap() = Some(0);
    *nested_1.lock().unwrap() = Some(0);
    *new_val_1.lock().unwrap() = Some(0);
    *newlines_1.lock().unwrap() = Some(0);
    *next_1.lock().unwrap() = Some(0);
    *next_byte_1.lock().unwrap() = Some(0);
    *nextc_1.lock().unwrap() = Some(0);
    *nl_is_end_1.lock().unwrap() = Some(0);
    *nl_is_space_1.lock().unwrap() = Some(0);
    *now_1.lock().unwrap() = Some(0);
    *object_1.lock().unwrap() = Some(0);
    *octal_zero_1.lock().unwrap() = Some(0);
    *offset_1.lock().unwrap() = Some(0);
    *ok_1.lock().unwrap() = Some(0);
    *ok_verbs_1.lock().unwrap() = Some(0);
    *old_1.lock().unwrap() = Some(0);
    *op_1.lock().unwrap() = Some(0);
    *ops_1.lock().unwrap() = Some(0);
    *options_1.lock().unwrap() = Some(0);
    *other_1.lock().unwrap() = Some(0);
    *out_1.lock().unwrap() = Some(0);
    *outer_1.lock().unwrap() = Some(0);
    *outer_index_1.lock().unwrap() = Some(0);
    *output_1.lock().unwrap() = Some(0);
    *owner_1.lock().unwrap() = Some(0);
    *p_1.lock().unwrap() = Some(0);
    *pad2_1.lock().unwrap() = Some(0);
    *pad3_1.lock().unwrap() = Some(0);
    *parent_1.lock().unwrap() = Some(0);
    *path_1.lock().unwrap() = Some(0);
    *path_error_1.lock().unwrap() = Some(0);
    *pattern_1.lock().unwrap() = Some(0);
    *payload_1.lock().unwrap() = Some(0);
    *pending_1.lock().unwrap() = Some(0);
    *period_1.lock().unwrap() = Some(0);
    *perm_1.lock().unwrap() = Some(0);
    *pipe_1.lock().unwrap() = Some(0);
    *plain_1.lock().unwrap() = Some(0);
    *plain_v_1.lock().unwrap() = Some(0);
    *plus_1.lock().unwrap() = Some(0);
    *plus_v_1.lock().unwrap() = Some(0);
    *poisoned_1.lock().unwrap() = Some(0);
    *pos_1.lock().unwrap() = Some(0);
    *precision_1.lock().unwrap() = Some(0);
    *prefix_1.lock().unwrap() = Some(0);
    *previous_1.lock().unwrap() = Some(0);
    *ptr_1.lock().unwrap() = Some(0);
    *quote_1.lock().unwrap() = Some(0);
    *quoted_1.lock().unwrap() = Some(0);
    *r_1.lock().unwrap() = Some(0);
    *raw_1.lock().unwrap() = Some(0);
    *raw_query_1.lock().unwrap() = Some(0);
    *read_1.lock().unwrap() = Some(0);
    *read_all_1.lock().unwrap() = Some(0);
    *ready_1.lock().unwrap() = Some(0);
    *registry_1.lock().unwrap() = Some(0);
    *relative_1.lock().unwrap() = Some(0);
    *repl_1.lock().unwrap() = Some(0);
    *rest_1.lock().unwrap() = Some(0);
    *result_1.lock().unwrap() = Some(0);
    *right_1.lock().unwrap() = Some(0);
    *right_guard_1.lock().unwrap() = Some(0);
    *rows_1.lock().unwrap() = Some(0);
    *rs_1.lock().unwrap() = Some(0);
    *rx_1.lock().unwrap() = Some(0);
    *s_1.lock().unwrap() = Some(0);
    *s0_1.lock().unwrap() = Some(0);
    *s1_1.lock().unwrap() = Some(0);
    *saw_1.lock().unwrap() = Some(0);
    *scale_1.lock().unwrap() = Some(0);
    *scheme_1.lock().unwrap() = Some(0);
    *second_1.lock().unwrap() = Some(0);
    *seconds_1.lock().unwrap() = Some(0);
    *secs_of_day_1.lock().unwrap() = Some(0);
    *seed_1.lock().unwrap() = Some(0);
    *seen_point_1.lock().unwrap() = Some(0);
    *sender_1.lock().unwrap() = Some(0);
    *setter_1.lock().unwrap() = Some(0);
    *setter_guard_1.lock().unwrap() = Some(0);
    *shared_1.lock().unwrap() = Some(0);
    *sharp_1.lock().unwrap() = Some(0);
    *sharp_v_1.lock().unwrap() = Some(0);
    *shift_1.lock().unwrap() = Some(0);
    *shortest_1.lock().unwrap() = Some(0);
    *sign_1.lock().unwrap() = Some(0);
    *sign_len_1.lock().unwrap() = Some(0);
    *simple_1.lock().unwrap() = Some(0);
    *single_1.lock().unwrap() = Some(0);
    *skip_space_1.lock().unwrap() = Some(0);
    *slash_1.lock().unwrap() = Some(0);
    *slice_1.lock().unwrap() = Some(0);
    *slot_1.lock().unwrap() = Some(0);
    *source_1.lock().unwrap() = Some(0);
    *space_1.lock().unwrap() = Some(0);
    *spec_1.lock().unwrap() = Some(0);
    *src_1.lock().unwrap() = Some(0);
    *stack_1.lock().unwrap() = Some(0);
    *start_1.lock().unwrap() = Some(0);
    *state_1.lock().unwrap() = Some(0);
    *state_ref_1.lock().unwrap() = Some(0);
    *status_1.lock().unwrap() = Some(0);
    *std_1.lock().unwrap() = Some(0);
    *stderr_1.lock().unwrap() = Some(0);
    *stdin_1.lock().unwrap() = Some(0);
    *stdout_1.lock().unwrap() = Some(0);
    *step_1.lock().unwrap() = Some(0);
    *stopped_1.lock().unwrap() = Some(0);
    *stored_1.lock().unwrap() = Some(0);
    *string_verb_1.lock().unwrap() = Some(0);
    *suffix_1.lock().unwrap() = Some(0);
    *syntax_1.lock().unwrap() = Some(0);
    *t_1.lock().unwrap() = Some(0);
    *table_1.lock().unwrap() = Some(0);
    *target_1.lock().unwrap() = Some(0);
    *target_code_1.lock().unwrap() = Some(0);
    *targets_1.lock().unwrap() = Some(0);
    *temp1_1.lock().unwrap() = Some(0);
    *temp2_1.lock().unwrap() = Some(0);
    *text_1.lock().unwrap() = Some(0);
    *text_ref_1.lock().unwrap() = Some(0);
    *thread_channel_1.lock().unwrap() = Some(0);
    *thread_stopped_1.lock().unwrap() = Some(0);
    *timeout_cancelled_1.lock().unwrap() = Some(0);
    *timeout_done_1.lock().unwrap() = Some(0);
    *timeout_err_1.lock().unwrap() = Some(0);
    *timer_1.lock().unwrap() = Some(0);
    *tok_1.lock().unwrap() = Some(0);
    *trailing_space_1.lock().unwrap() = Some(0);
    *truncated_1.lock().unwrap() = Some(0);
    *tx_1.lock().unwrap() = Some(0);
    *typ_1.lock().unwrap() = Some(0);
    *typed_val_1.lock().unwrap() = Some(0);
    *underscores_1.lock().unwrap() = Some(0);
    *unread_1.lock().unwrap() = Some(0);
    *unsigned_1.lock().unwrap() = Some(0);
    *upper_1.lock().unwrap() = Some(0);
    *r#use.lock().unwrap() = Some(0);
    *utf8_1.lock().unwrap() = Some(0);
    *v_guard_1.lock().unwrap() = Some(0);
    *val_1.lock().unwrap() = Some(0);
    *value_1.lock().unwrap() = Some(0);
    *value_start_1.lock().unwrap() = Some(0);
    *values_1.lock().unwrap() = Some(0);
    *verb_1.lock().unwrap() = Some(0);
    *w_1.lock().unwrap() = Some(0);
    *wait_1.lock().unwrap() = Some(0);
    *want_1.lock().unwrap() = Some(0);
    *was_active_1.lock().unwrap() = Some(0);
    *was_stopped_1.lock().unwrap() = Some(0);
    *when_1.lock().unwrap() = Some(0);
    *whole_1.lock().unwrap() = Some(0);
    *wid_present_1.lock().unwrap() = Some(0);
    *width_1.lock().unwrap() = Some(0);
    *with_mut_1.lock().unwrap() = Some(0);
    *with_precision_1.lock().unwrap() = Some(0);
    *word_1.lock().unwrap() = Some(0);
    *wrapped_1.lock().unwrap() = Some(0);
    *wraps_1.lock().unwrap() = Some(0);
    *write_1.lock().unwrap() = Some(0);
    *y_1.lock().unwrap() = Some(0);
    *year_1.lock().unwrap() = Some(0);
    *yoe_1.lock().unwrap() = Some(0);
    *z_1.lock().unwrap() = Some(0);
    *zero_1.lock().unwrap() = Some(0);
    *zero_allowed_1.lock().unwrap() = Some(0);
}


impl point {
    pub fn show(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(format!("{}{}{}", format!("{}", (*self.x.lock().unwrap().as_ref().unwrap())), format!("{}", ":".to_string()), format!("{}", (*self.y.lock().unwrap().as_ref().unwrap()))))))
    }
}

pub fn sum() -> i64 {
    return {
            let __go_binary_0 = (*a_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_1 = (*absolute_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_2 = __go_binary_0 + __go_binary_1;
            let __go_binary_3 = (*accept_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_4 = __go_binary_2 + __go_binary_3;
            let __go_binary_5 = (*access_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_6 = __go_binary_4 + __go_binary_5;
            let __go_binary_7 = (*actual_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_8 = __go_binary_6 + __go_binary_7;
            let __go_binary_9 = (*addr_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_10 = __go_binary_8 + __go_binary_9;
            let __go_binary_11 = (*address_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_12 = __go_binary_10 + __go_binary_11;
            let __go_binary_13 = (*after_mod_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_14 = __go_binary_12 + __go_binary_13;
            let __go_binary_15 = (*any_val_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_16 = __go_binary_14 + __go_binary_15;
            let __go_binary_17 = (*arg_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_18 = __go_binary_16 + __go_binary_17;
            let __go_binary_19 = (*arg0_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_20 = __go_binary_18 + __go_binary_19;
            let __go_binary_21 = (*arg1_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_22 = __go_binary_20 + __go_binary_21;
            let __go_binary_23 = (*args_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_24 = __go_binary_22 + __go_binary_23;
            let __go_binary_25 = (*array_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_26 = __go_binary_24 + __go_binary_25;
            let __go_binary_27 = (*as_error_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_28 = __go_binary_26 + __go_binary_27;
            let __go_binary_29 = (*ascii_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_30 = __go_binary_28 + __go_binary_29;
            let __go_binary_31 = (*assign_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_32 = __go_binary_30 + __go_binary_31;
            let __go_binary_33 = (*at_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_34 = __go_binary_32 + __go_binary_33;
            let __go_binary_35 = (*b_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_36 = __go_binary_34 + __go_binary_35;
            let __go_binary_37 = (*b0_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_38 = __go_binary_36 + __go_binary_37;
            let __go_binary_39 = (*b1_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_40 = __go_binary_38 + __go_binary_39;
            let __go_binary_41 = (*b2_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_42 = __go_binary_40 + __go_binary_41;
            let __go_binary_43 = (*base_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_44 = __go_binary_42 + __go_binary_43;
            let __go_binary_45 = (*base0_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_46 = __go_binary_44 + __go_binary_45;
            let __go_binary_47 = (*basic_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_48 = __go_binary_46 + __go_binary_47;
            let __go_binary_49 = (*before_query_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_50 = __go_binary_48 + __go_binary_49;
            let __go_binary_51 = (*bit_len_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_52 = __go_binary_50 + __go_binary_51;
            let __go_binary_53 = (*bit_size_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_54 = __go_binary_52 + __go_binary_53;
            let __go_binary_55 = (*body_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_56 = __go_binary_54 + __go_binary_55;
            let __go_binary_57 = (*bool_error_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_58 = __go_binary_56 + __go_binary_57;
            let __go_binary_59 = (*borrow_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_60 = __go_binary_58 + __go_binary_59;
            let __go_binary_61 = (*boxed_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_62 = __go_binary_60 + __go_binary_61;
            let __go_binary_63 = (*bytes_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_64 = __go_binary_62 + __go_binary_63;
            let __go_binary_65 = (*c_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_66 = __go_binary_64 + __go_binary_65;
            let __go_binary_67 = (*c0_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_68 = __go_binary_66 + __go_binary_67;
            let __go_binary_69 = (*c1_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_70 = __go_binary_68 + __go_binary_69;
            let __go_binary_71 = (*c2_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_72 = __go_binary_70 + __go_binary_71;
            let __go_binary_73 = (*c3_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_74 = __go_binary_72 + __go_binary_73;
            let __go_binary_75 = (*call_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_76 = __go_binary_74 + __go_binary_75;
            let __go_binary_77 = (*callback_1.lock().unwrap().as_ref().unwrap());
            let __go_binary_78 = __go_binary_76 + __go_binary_77;
            __go_binary_78
        };
}

fn main() {
    go_install_panic_exit();
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math::__go_init_all();
    ::math_bits::__go_init_all();
    ::sort::__go_init_all();
    ::strconv::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    __go_init_all();
    { let new_val = 3; *value_1.lock().unwrap() = Some(new_val); };
    { let new_val = 4; *slice_1.lock().unwrap() = Some(new_val); };
    { let __go_print_arg_1 = sum(); let __go_print_value = println!("{} {} {}", format!("{}", { let __v = (*value_1.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*slice_1.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", __go_print_arg_1)); __go_print_value };
    let mut x: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(1i64) as Box<dyn Any + Send + Sync>)));
    let mut y_local: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new("s".to_string()) as Box<dyn Any + Send + Sync>)));
    println!("{} {} {} {}", format!("{}", { let __text = match x.lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }), format!("{}", { let __text = match y_local.lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }), format!("{}", go_any_eq(&x, &y_local)), format!("{}", format_any_slice(&Arc::new(Mutex::new(Some(vec![Box::new(1i64) as Box<dyn Any + Send + Sync>, Box::new("a".to_string()) as Box<dyn Any + Send + Sync>, Box::new(2.5) as Box<dyn Any + Send + Sync>, Box::new(GoNilAny) as Box<dyn Any + Send + Sync>]))))));
    let mut m_local = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Vec<i64>>>>>::from([("k".to_string(), Arc::new(Mutex::new(Some(vec![1i64, 2i64])))), ("j".to_string(), Arc::new(Mutex::new(Some(vec![3i64]))))]))));
    println!("{} {}", format!("{}", format_map_nil(&m_local, "[]")), format!("{}", ({ let __len = (*m_local.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })));
    let mut nested_local = Arc::new(Mutex::new(Some(vec![vec!["a".to_string()], vec!["b".to_string(), "c".to_string()]])));
    println!("{} {}", format!("{}", format_nested_slice(&nested_local)), format!("{}", format_nested_slice(&Arc::new(Mutex::new(Some([[1i64, 2i64], [3i64, 4i64]]))))));
    let mut p_local = Arc::new(Mutex::new(Some(point { x: Arc::new(Mutex::new(Some(1))), y: Arc::new(Mutex::new(Some(2))), ..Default::default() })));
    { let __go_print_arg_2 = { let __recv = p_local.clone(); let __recv_ptr: *const point = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const point }; let __result = unsafe { &*__recv_ptr }.show(); __result }; let __go_print_value = println!("{} {}", format!("{}", { let __v = (*p_local.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }), format!("{}", { let __v = (*__go_print_arg_2.lock().unwrap().as_ref().unwrap()).clone(); __v })); __go_print_value };
    print!("{}", go_sprintf(&"%v %+v %q %x %5.2f %T\n".to_string(), &[{ let __go_format_arg = Box::new(point { x: Arc::new(Mutex::new(Some(3))), y: Arc::new(Mutex::new(Some(4))), ..Default::default() }) as Box<dyn Any + Send + Sync>; __go_format_arg }, { let __go_format_arg = Box::new(point { x: Arc::new(Mutex::new(Some(5))), y: Arc::new(Mutex::new(Some(6))), ..Default::default() }) as Box<dyn Any + Send + Sync>; __go_format_arg }, { let __go_format_arg = Box::new("q".to_string()) as Box<dyn Any + Send + Sync>; __go_format_arg }, { let __go_format_arg = Box::new(255i64) as Box<dyn Any + Send + Sync>; __go_format_arg }, { let __go_format_arg = Box::new(3.14159) as Box<dyn Any + Send + Sync>; __go_format_arg }, { let __go_format_arg = Box::new(p_local.clone()) as Box<dyn Any + Send + Sync>; __go_format_arg }], false));
    let mut err_local = Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("boom".to_string())))));
    let mut wrapped_local = Arc::new(Mutex::new(Some(go_error_wrap(format!("wrap: {}", match { let __err_handle = { let __err_source = err_local.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }; let __err_value = (*__err_handle.lock().unwrap()).clone(); __err_value } { Some(__wrapped) => format!("{}", __wrapped), None => "%!w(<nil>)".to_string() }), vec![{ let __err_handle = { let __err_source = err_local.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }; let __err_value = (*__err_handle.lock().unwrap()).clone(); __err_value }]))));
    println!("{} {} {} {}", format!("{}", { let __text = match err_local.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __text = match wrapped_local.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", (*Arc::new(Mutex::new(Some(go_errors_is({ let __err_handle = { let __err_source = wrapped_local.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }; let __err_value = (*__err_handle.lock().unwrap()).clone(); __err_value }, { let __err_handle = { let __err_source = err_local.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }; let __err_value = (*__err_handle.lock().unwrap()).clone(); __err_value })))).lock().unwrap().as_ref().unwrap())), format!("{}", { let __left = Arc::new(Mutex::new(go_errors_unwrap({ let __err_handle = { let __err_source = wrapped_local.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }; let __err_value = (*__err_handle.lock().unwrap()).clone(); __err_value }))).clone(); let __right = err_local.clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__l), Some(__r)) => std::ptr::addr_eq(&**__l, &**__r), (None, None) => true, _ => false } }; __eq }));
    let mut words = strings::fields(Arc::new(Mutex::new(Some(" b a c ".to_string()))));
    let mut words = { let __slice_header = GoSliceHeader::go_slice_header(&words); Arc::new(Mutex::new(__slice_header)) };
    sort::strings(words.clone());
    let (mut n_local, mut convErr) = strconv::atoi(Arc::new(Mutex::new(Some("42".to_string()))));
    { let __go_print_arg_3 = strings::join(words.clone(), Arc::new(Mutex::new(Some("-".to_string())))); let __go_print_arg_4 = strconv::quote(Arc::new(Mutex::new(Some("hi".to_string())))); let __go_print_value = println!("{} {} {} {} {}", format!("{}", format_slice(&words)), format!("{}", { let __v = (*__go_print_arg_3.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", n_local), format!("{}", { let __text = match convErr.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __v = (*__go_print_arg_4.lock().unwrap().as_ref().unwrap()).clone(); __v })); __go_print_value };
    println!("{} {}", format!("{}", (*Arc::new(Mutex::new(Some(format!("{}", { let mut __print_text = String::new(); let __print_part_0 = format!("{}", format!("{}", { let __text = match x.lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text })); let __print_is_string_0 = { let __print_holder = x.clone(); let __print_guard = __print_holder.lock().unwrap(); __print_guard.as_ref().map_or(false, |__value| __value.is::<String>()) }; __print_text.push_str(&__print_part_0); let __print_part_1 = format!("{}", format!("{}", 2)); let __print_is_string_1 = false; if !__print_is_string_0 && !__print_is_string_1 { __print_text.push(' '); } __print_text.push_str(&__print_part_1); __print_text })))).lock().unwrap().as_ref().unwrap())), format!("{}", (*Arc::new(Mutex::new(Some(format!("{}", go_sprintf(&"%08.3f|%-4d|%x".to_string(), &[{ let __go_format_arg = Box::new(3.5) as Box<dyn Any + Send + Sync>; __go_format_arg }, { let __go_format_arg = Box::new(7i64) as Box<dyn Any + Send + Sync>; __go_format_arg }, { let __go_format_arg = Box::new("hi".to_string()) as Box<dyn Any + Send + Sync>; __go_format_arg }], false))))).lock().unwrap().as_ref().unwrap())));
}

pub(crate) fn __go_init_all() {
    static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();
    __GO_INIT_ONCE.call_once(|| {
        self::__go_init_globals();
    });
}


impl GoValueClone for point {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
