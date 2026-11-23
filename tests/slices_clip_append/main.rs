#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::fmt::{Display};
use std::sync::{Arc, Mutex};

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

fn go_float_string(value: f64, single: bool) -> String {
    if value.is_nan() {
        return "NaN".to_string();
    }
    if value.is_infinite() {
        return if value < 0.0 { "-Inf".to_string() } else { "+Inf".to_string() };
    }
    let sign = if value.is_sign_negative() { "-" } else { "" };
    let shortest = if single { format!("{:e}", value.abs() as f32) } else { format!("{:e}", value.abs()) };
    let (mantissa, exponent) = shortest.split_once('e').unwrap();
    let digits: String = mantissa.chars().filter(|c| c.is_ascii_digit()).collect();
    let exponent: i32 = exponent.parse().unwrap();
    if exponent < -4 || exponent >= 6 {
        let mut text = String::from(sign);
        text.push_str(&digits[..1]);
        if digits.len() > 1 {
            text.push('.');
            text.push_str(&digits[1..]);
        }
        text.push('e');
        text.push(if exponent < 0 { '-' } else { '+' });
        text.push_str(&format!("{:02}", exponent.abs()));
        return text;
    }
    if exponent < 0 {
        return format!("{}0.{}{}", sign, "0".repeat((-exponent - 1) as usize), digits);
    }
    let whole = (exponent + 1) as usize;
    if digits.len() <= whole {
        return format!("{}{}{}", sign, digits, "0".repeat(whole - digits.len()));
    }
    format!("{}{}.{}", sign, &digits[..whole], &digits[whole..])
}

// A value inside a slice or a map prints as fmt prints it: a float through
// go_float_string, anything else through its Display. The element type may
// borrow, so it is recognized by name rather than by TypeId.
fn go_display_value<T: Display>(value: &T) -> String {
    let name = std::any::type_name::<T>();
    if name == "f64" {
        // T is f64.
        return go_float_string(unsafe { *(value as *const T as *const f64) }, false);
    }
    if name == "f32" {
        // T is f32.
        return go_float_string(unsafe { *(value as *const T as *const f32) } as f64, true);
    }
    value.to_string()
}

fn main() {
    ::cmp::__go_init_all();
    ::iter::__go_init_all();
    ::math_bits::__go_init_all();
    ::slices::__go_init_all();

    let mut env = Arc::new(Mutex::new(Some({ let mut v: Vec<String> = (0..(2) as usize).map(|_| "".to_string()).collect(); v.truncate((1) as usize); v })));
    { let __idx = (0) as usize; let __rhs = "A=1".to_string(); (*env.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    let mut combined = { let __append_target = { let __result = slices::clip::<Vec<String>, String>({ let __slice_holder = env.clone(); { let __slice_guard = __slice_holder.lock().unwrap(); Arc::new(Mutex::new(__slice_guard.as_ref().map(|__v| __v.iter().cloned().map(|__elem| Arc::new(Mutex::new(Some(__elem)))).collect::<Vec<_>>()))) } }); let __result_guard = __result.lock().unwrap(); Arc::new(Mutex::new(__result_guard.as_ref().map(|__v| __v.iter().cloned().map(|__elem| (*__elem.lock().unwrap().as_ref().unwrap()).clone()).collect::<Vec<_>>()))) }.clone(); let __append_value = "PWD=/tmp".to_string(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) };
    { let __idx = (0) as usize; let __rhs = "B=2".to_string(); (*combined.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    println!("{}", format!("{}", format_slice(&env)));
    println!("{}", format!("{}", format_slice(&combined)));
}