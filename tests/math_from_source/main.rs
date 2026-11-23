#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::fmt::{Display};
use std::sync::{Arc, Mutex};


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
    ::math::__go_init_all();
    ::math_bits::__go_init_all();

    println!("{} {} {}", format!("{}", { let __tmp_x = math::inf(Arc::new(Mutex::new(Some(1)))); let __tmp_y = 1e+308; __tmp_x > __tmp_y }), format!("{}", math::is_inf(Arc::new(Mutex::new(Some(math::inf(Arc::new(Mutex::new(Some(-1))))))), Arc::new(Mutex::new(Some(-1))))), format!("{}", math::is_na_n(Arc::new(Mutex::new(Some(math::na_n()))))));
    println!("{} {} {} {} {}", { let __text = go_float_string((math::floor(Arc::new(Mutex::new(Some(-2.5))))) as f64, false); __text }, { let __text = go_float_string((math::ceil(Arc::new(Mutex::new(Some(2.1))))) as f64, false); __text }, { let __text = go_float_string((math::trunc(Arc::new(Mutex::new(Some(-2.7))))) as f64, false); __text }, { let __text = go_float_string((math::round(Arc::new(Mutex::new(Some(2.5))))) as f64, false); __text }, { let __text = go_float_string((math::abs(Arc::new(Mutex::new(Some(-3.0))))) as f64, false); __text });
    println!("{} {} {} {} {} {}", { let __text = go_float_string((math::sqrt(Arc::new(Mutex::new(Some(2.0))))) as f64, false); __text }, { let __text = go_float_string((math::pow(Arc::new(Mutex::new(Some(2.0))), Arc::new(Mutex::new(Some(10.0))))) as f64, false); __text }, { let __text = go_float_string((math::exp(Arc::new(Mutex::new(Some(1.0))))) as f64, false); __text }, { let __text = go_float_string((math::log(Arc::new(Mutex::new(Some(math::E))))) as f64, false); __text }, { let __text = go_float_string((math::log2(Arc::new(Mutex::new(Some(8.0))))) as f64, false); __text }, { let __text = go_float_string((math::log10(Arc::new(Mutex::new(Some(1000.0))))) as f64, false); __text });
    println!("{} {} {} {}", { let __text = go_float_string((math::max(Arc::new(Mutex::new(Some(3.0))), Arc::new(Mutex::new(Some(7.0))))) as f64, false); __text }, { let __text = go_float_string((math::min(Arc::new(Mutex::new(Some(3.0))), Arc::new(Mutex::new(Some(7.0))))) as f64, false); __text }, { let __text = go_float_string((math::hypot(Arc::new(Mutex::new(Some(3.0))), Arc::new(Mutex::new(Some(4.0))))) as f64, false); __text }, { let __text = go_float_string((math::r#mod(Arc::new(Mutex::new(Some(7.0))), Arc::new(Mutex::new(Some(3.0))))) as f64, false); __text });
    println!("{} {} {}", format!("{}", math::float64bits(Arc::new(Mutex::new(Some(1.5))))), { let __text = go_float_string((math::float64frombits(Arc::new(Mutex::new(Some(0x3ff8000000000000 as u64))))) as f64, false); __text }, format!("{}", math::signbit(Arc::new(Mutex::new(Some(math::copysign(Arc::new(Mutex::new(Some(0.0))), Arc::new(Mutex::new(Some(-1.0))))))))));
    println!("{} {} {} {} {}", { let __text = go_float_string((math::sin(Arc::new(Mutex::new(Some(0.0))))) as f64, false); __text }, { let __text = go_float_string((math::cos(Arc::new(Mutex::new(Some(0.0))))) as f64, false); __text }, { let __text = go_float_string(({ let __tmp_x = math::atan2(Arc::new(Mutex::new(Some(1.0))), Arc::new(Mutex::new(Some(1.0)))); let __tmp_y = 4.0; __tmp_x * __tmp_y }) as f64, false); __text }, format!("{}", 9223372036854775807i64), format!("{}", { let __tmp_x = (math::MAX_FLOAT64 as f64); let __tmp_y = 1.0; __tmp_x > __tmp_y }));
    let (mut frac, mut exp) = math::frexp(Arc::new(Mutex::new(Some(8.0))));
    println!("{} {} {} {}", { let __text = go_float_string((frac) as f64, false); __text }, format!("{}", exp), { let __text = go_float_string((math::ldexp(Arc::new(Mutex::new(Some(0.5))), Arc::new(Mutex::new(Some(4))))) as f64, false); __text }, { let __text = go_float_string((math::cbrt(Arc::new(Mutex::new(Some(27.0))))) as f64, false); __text });
    let (mut i, mut f) = math::modf(Arc::new(Mutex::new(Some(3.75))));
    println!("{} {}", { let __text = go_float_string((i) as f64, false); __text }, { let __text = go_float_string((f) as f64, false); __text });
}