#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use num::Complex;
use std::any::Any;
use std::fmt::{Display};
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
    if __name == "num_complex::Complex<f64>" || __name == "num_complex::Complex<f32>" {
        let __single = __name.ends_with("f32>");
        let (__re, __im) = if __single {
            let __parts = unsafe { &*(__value as *const T as *const [f32; 2]) };
            (__parts[0] as f64, __parts[1] as f64)
        } else {
            let __parts = unsafe { &*(__value as *const T as *const [f64; 2]) };
            (__parts[0], __parts[1])
        };
        let __im_text = go_float_string(__im, __single);
        let __plus = if __im_text.starts_with('-') || __im_text.starts_with('+') { "" } else { "+" };
        return format!("({}{}{}i)", go_float_string(__re, __single), __plus, __im_text);
    }
    __value.to_string()
}

pub(crate) const BIG: i64 = 1 << 62;


fn main() {
    go_install_panic_exit();
    ::math::__go_init_all();
    ::math_bits::__go_init_all();

    println!("{} {} {} {}", format!("{}", (*Arc::new(Mutex::new(Some(9223372036854775807i64))).lock().unwrap().as_ref().unwrap())), format!("{}", (*Arc::new(Mutex::new(Some(4294967295i64))).lock().unwrap().as_ref().unwrap())), format!("{}", (*Arc::new(Mutex::new(Some(BIG as i64))).lock().unwrap().as_ref().unwrap())), format!("{}", (*Arc::new(Mutex::new(Some(255 as u8))).lock().unwrap().as_ref().unwrap())));
    println!("{} {} {} {} {}", format!("{}", false), format!("{}", true), format!("{}", (-1i32)), format!("{}", (-1i64)), format!("{}", (8u16)));
    let mut x = Arc::new(Mutex::new(Some(4294967296i64)));
    println!("{} {}", format!("{}", { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4294967296; __tmp_x == __tmp_y }));
    let mut c = Arc::new(Mutex::new(Some(num::Complex::new(1.5 as f64, -(2) as f64))));
    println!("{} {}", { let __text = go_float_string(({ let __part = (*c.lock().unwrap().as_ref().unwrap()).re; __part }) as f64, false); __text }, { let __text = go_float_string(({ let __part = (*c.lock().unwrap().as_ref().unwrap()).im; __part }) as f64, false); __text });
    print!("{} {} {}\n", format!("{}", { let __part = (*c.lock().unwrap().as_ref().unwrap()).re; __part }), format!("{:.2}", { let __part = (*c.lock().unwrap().as_ref().unwrap()).im; __part }), format!("{}", { let _ = { let __part = (*c.lock().unwrap().as_ref().unwrap()).re; __part }; "float64" }));
    let mut sum = Arc::new(Mutex::new(Some({ let __tmp_x = { let __part = (*c.lock().unwrap().as_ref().unwrap()).re; __part }; let __tmp_y = { let __part = (*c.lock().unwrap().as_ref().unwrap()).im; __part }; __tmp_x + __tmp_y })));
    let mut parts = Arc::new(Mutex::new(Some(vec![{ let __part = (*c.lock().unwrap().as_ref().unwrap()).re; __part }, { let __part = (*c.lock().unwrap().as_ref().unwrap()).im; __part }])));
    println!("{} {} {}", { let __text = go_float_string(({ let __v = (*sum.lock().unwrap().as_ref().unwrap()).clone(); __v }) as f64, false); __text }, format!("{}", format_slice(&parts)), { let __text = go_float_string((math::abs(Arc::new(Mutex::new(Some({ let __part = (*c.lock().unwrap().as_ref().unwrap()).im; __part }))))) as f64, false); __text });
    let mut c64: Arc<Mutex<Option<num::Complex<f32>>>> = Arc::new(Mutex::new(Some(num::Complex::new(3 as f32, 4 as f32))));
    println!("{} {} {}", { let __text = go_float_string(({ let __part = (*c64.lock().unwrap().as_ref().unwrap()).re; __part }) as f64, true); __text }, { let __text = go_float_string(({ let __part = (*c64.lock().unwrap().as_ref().unwrap()).im; __part }) as f64, true); __text }, { let __text = go_float_string((math::hypot(Arc::new(Mutex::new(Some({ let __arg_value = { let __part = (*c64.lock().unwrap().as_ref().unwrap()).re; __part } as f64; __arg_value }))), Arc::new(Mutex::new(Some({ let __arg_value = { let __part = (*c64.lock().unwrap().as_ref().unwrap()).im; __part } as f64; __arg_value }))))) as f64, false); __text });
}