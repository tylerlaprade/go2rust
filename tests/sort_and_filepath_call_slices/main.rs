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

fn main() {
    ::internal_bytealg::__go_init_all();
    ::internal_filepathlite::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::path_filepath::__go_init_all();
    ::sort::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut x = Arc::new(Mutex::new(Some(vec![3i64, 1i64, 2i64])));
    println!("{}", format!("{}", sort::ints_are_sorted(x.clone())));
    sort::ints(x.clone());
    println!("{} {}", format!("{}", format_slice(&x)), format!("{}", sort::ints_are_sorted(x.clone())));
    let mut s = Arc::new(Mutex::new(Some(vec!["b".to_string(), "c".to_string(), "a".to_string()])));
    sort::strings(s.clone());
    println!("{} {} {}", format!("{}", format_slice(&s)), format!("{}", sort::strings_are_sorted(s.clone())), format!("{}", sort::search_strings(s.clone(), Arc::new(Mutex::new(Some("b".to_string()))))));
    let mut f = Arc::new(Mutex::new(Some(vec![2.5, (-1.0), 0.0])));
    println!("{}", format!("{}", sort::float64s_are_sorted(f.clone())));
    sort::float64s(f.clone());
    println!("{} {}", format!("{}", format_slice(&f)), format!("{}", sort::float64s_are_sorted(f.clone())));
    println!("{} {} {} {} {}", format!("{}", (*path_filepath::join(Arc::new(Mutex::new(Some(vec!["a".to_string(), "b".to_string(), "../c".to_string()])))).lock().unwrap().as_ref().unwrap())), format!("{}", (*path_filepath::clean(Arc::new(Mutex::new(Some("/x//y/./z/..".to_string())))).lock().unwrap().as_ref().unwrap())), format!("{}", (*path_filepath::base(Arc::new(Mutex::new(Some("/x/y.go".to_string())))).lock().unwrap().as_ref().unwrap())), format!("{}", (*path_filepath::ext(Arc::new(Mutex::new(Some("f.txt".to_string())))).lock().unwrap().as_ref().unwrap())), format!("{}", (*path_filepath::dir(Arc::new(Mutex::new(Some("a/b/c".to_string())))).lock().unwrap().as_ref().unwrap())));
    let (mut rel, mut err) = path_filepath::rel(Arc::new(Mutex::new(Some("/a/b".to_string()))), Arc::new(Mutex::new(Some("/a/c/d".to_string()))));
    println!("{} {}", format!("{}", { let __v = (*rel.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    let (mut ok, _) = path_filepath::r#match(Arc::new(Mutex::new(Some("*.go".to_string()))), Arc::new(Mutex::new(Some("main.go".to_string()))));
    println!("{} {} {}", format!("{}", ok), format!("{}", path_filepath::is_abs(Arc::new(Mutex::new(Some("/x".to_string()))))), format!("{}", format_slice(&path_filepath::split_list(Arc::new(Mutex::new(Some("a:b".to_string())))))));
}