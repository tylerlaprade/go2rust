#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::error::Error as StdError;
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

static __GO_OS_ARGS: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<String>>>>> =
    std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(Some(std::env::args().collect::<Vec<String>>()))));

fn go_os_args() -> std::sync::Arc<std::sync::Mutex<Option<Vec<String>>>> {
    __GO_OS_ARGS.clone()
}

fn main() {
    ::internal_bisect::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_godebug::__go_init_all();
    ::internal_godebugs::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math::__go_init_all();
    ::math_bits::__go_init_all();
    ::strconv::__go_init_all();
    ::strings::__go_init_all();
    ::time::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    println!("{}", format!("{}", "=== Testing multiple stdlib imports ===".to_string()));
    println!("{}", format!("{}", "\n--- strings package ---".to_string()));
    let mut upper = strings::to_upper(Arc::new(Mutex::new(Some("hello world".to_string()))));
    println!("{} {}", format!("{}", "Upper:".to_string()), format!("{}", { let __v = (*upper.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    let mut lower = strings::to_lower(Arc::new(Mutex::new(Some("HELLO WORLD".to_string()))));
    println!("{} {}", format!("{}", "Lower:".to_string()), format!("{}", { let __v = (*lower.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    let mut trimmed = strings::trim_space(Arc::new(Mutex::new(Some("  hello  ".to_string()))));
    println!("{} {}", format!("{}", "Trimmed:".to_string()), format!("{}", { let __v = (*trimmed.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    let mut split = strings::split(Arc::new(Mutex::new(Some("a,b,c".to_string()))), Arc::new(Mutex::new(Some(",".to_string()))));
    let mut split = { let __slice_header = GoSliceHeader::go_slice_header(&split); Arc::new(Mutex::new(__slice_header)) };
    print!("Split: {}\n", format_slice(&split));
    println!("{}", format!("{}", "\n--- strconv package ---".to_string()));
    let mut num = Arc::new(Mutex::new(Some(42i64)));
    let mut str = strconv::itoa(Arc::new(Mutex::new(Some({ let __arg_holder = num.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    println!("{} {}", format!("{}", "Number as string:".to_string()), format!("{}", { let __v = (*str.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    let (mut parsed, mut err) = strconv::atoi(Arc::new(Mutex::new(Some("123".to_string()))));
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        println!("{} {}", format!("{}", "Parse error:".to_string()), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    } else {
        println!("{} {}", format!("{}", "Parsed number:".to_string()), format!("{}", parsed));
    }
    let mut floatStr = strconv::format_float(Arc::new(Mutex::new(Some(3.14159))), Arc::new(Mutex::new(Some(('f' as u8) as u8))), Arc::new(Mutex::new(Some(2))), Arc::new(Mutex::new(Some(64))));
    println!("{} {}", format!("{}", "Float as string:".to_string()), format!("{}", { let __v = (*floatStr.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    println!("{}", format!("{}", "\n--- math package ---".to_string()));
    print!("Pi: {:.6}\n", math::PI);
    print!("E: {:.6}\n", math::E);
    print!("Sqrt(16): {:.2}\n", math::sqrt(Arc::new(Mutex::new(Some(16.0)))));
    print!("Pow(2, 8): {:.0}\n", math::pow(Arc::new(Mutex::new(Some(2.0))), Arc::new(Mutex::new(Some(8.0)))));
    print!("Max(10, 20): {:.0}\n", math::max(Arc::new(Mutex::new(Some(10.0))), Arc::new(Mutex::new(Some(20.0)))));
    print!("Min(10, 20): {:.0}\n", math::min(Arc::new(Mutex::new(Some(10.0))), Arc::new(Mutex::new(Some(20.0)))));
    println!("{}", format!("{}", "\n--- time package ---".to_string()));
    let mut localFixed = time::unix(Arc::new(Mutex::new(Some(1700000000 as i64))), Arc::new(Mutex::new(Some(0 as i64))));
    let mut fixed = (*localFixed.lock().unwrap().as_ref().unwrap()).u_t_c();
    println!("{} {}", format!("{}", "Fixed timestamp:".to_string()), format!("{}", (*fixed.lock().unwrap().as_ref().unwrap()).unix()));
    let mut later = (*fixed.lock().unwrap().as_ref().unwrap()).add(Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some(time::HOUR as i64))))))));
    println!("{} {}", format!("{}", "One hour later:".to_string()), format!("{}", (*later.lock().unwrap().as_ref().unwrap()).unix()));
    println!("{}", format!("{}", "\n--- os package ---".to_string()));
    println!("{} {}", format!("{}", "Argument count:".to_string()), format!("{}", ({ let __len = ({ let __len_target = { let __field = go_os_args().clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }); __len })));
    println!("{}", format!("{}", "\n--- Combined usage ---".to_string()));
    let mut timestamp = (*fixed.lock().unwrap().as_ref().unwrap()).unix();
    let mut timestampStr = strconv::format_int(Arc::new(Mutex::new(Some(timestamp))), Arc::new(Mutex::new(Some(10))));
    let mut message = Arc::new(Mutex::new(Some(format!("{}{}", "Timestamp: ".to_string(), { let __v = (*timestampStr.lock().unwrap().as_ref().unwrap()).clone(); __v }))));
    println!("{}", format!("{}", { let __v = (*message.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    let mut result = math::sqrt(Arc::new(Mutex::new(Some({ let __tmp_x = math::pow(Arc::new(Mutex::new(Some(3.0))), Arc::new(Mutex::new(Some(2.0)))); let __tmp_y = math::pow(Arc::new(Mutex::new(Some(4.0))), Arc::new(Mutex::new(Some(2.0)))); __tmp_x + __tmp_y }))));
    let mut resultStr = strconv::format_float(Arc::new(Mutex::new(Some(result))), Arc::new(Mutex::new(Some(('f' as u8) as u8))), Arc::new(Mutex::new(Some(2))), Arc::new(Mutex::new(Some(64))));
    print!("Hypotenuse of 3,4 triangle: {}\n", { let __v = (*resultStr.lock().unwrap().as_ref().unwrap()).clone(); __v });
}