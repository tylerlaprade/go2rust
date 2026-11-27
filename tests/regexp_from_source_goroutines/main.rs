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
use std::thread;

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
    ::bytes::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math::__go_init_all();
    ::math_bits::__go_init_all();
    ::regexp::__go_init_all();
    ::regexp_syntax::__go_init_all();
    ::sort::__go_init_all();
    ::strconv::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut re = regexp::must_compile(Arc::new(Mutex::new(Some("(\\w+)@(\\w+)\\.com".to_string()))));
    println!("{}", format!("{}", format_nested_slice(&{ let __recv = re.clone(); let __recv_ptr: *const regexp::r#mod::Regexp = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const regexp::r#mod::Regexp }; let __result = unsafe { &*__recv_ptr }.find_all_string_submatch(Arc::new(Mutex::new(Some("a@b.com, c@d.com".to_string()))), Arc::new(Mutex::new(Some(-1)))); __result })));
    println!("{} {}", format!("{}", format_slice(&{ let __recv = re.clone(); let __recv_ptr: *const regexp::r#mod::Regexp = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const regexp::r#mod::Regexp }; let __result = unsafe { &*__recv_ptr }.find_string_index(Arc::new(Mutex::new(Some("x a@b.com".to_string())))); __result })), format!("{}", { let __recv = re.clone(); let __recv_ptr: *const regexp::r#mod::Regexp = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const regexp::r#mod::Regexp }; let __result = unsafe { &*__recv_ptr }.num_subexp(); __result }));
    println!("{}", format!("{}", (*{ let __recv = re.clone(); let __recv_ptr: *const regexp::r#mod::Regexp = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const regexp::r#mod::Regexp }; let __result = unsafe { &*__recv_ptr }.replace_all_string_func(
        Arc::new(Mutex::new(Some("a@b.com".to_string()))),
        Arc::new(Mutex::new(Some(Arc::new(move |__arg0: Arc<Mutex<Option<String>>>| -> Arc<Mutex<Option<String>>> { strings::to_upper(__arg0) }) as Arc<dyn Fn(Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> + Send + Sync>))),
    ); __result }.lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", (*{ let __recv = re.clone(); let __recv_ptr: *const regexp::r#mod::Regexp = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const regexp::r#mod::Regexp }; let __result = unsafe { &*__recv_ptr }.replace_all_string(Arc::new(Mutex::new(Some("a@b.com".to_string()))), Arc::new(Mutex::new(Some("$2 at $1".to_string())))); __result }.lock().unwrap().as_ref().unwrap())));
    let mut named = regexp::must_compile(Arc::new(Mutex::new(Some("(?P<year>\\d{4})-(?P<month>\\d{2})".to_string()))));
    let mut m = { let __recv = named.clone(); let __recv_ptr: *const regexp::r#mod::Regexp = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const regexp::r#mod::Regexp }; let __result = unsafe { &*__recv_ptr }.find_string_submatch(Arc::new(Mutex::new(Some("on 2024-05".to_string())))); __result };
    let mut m = { let __slice_header = GoSliceHeader::go_slice_header(&m); Arc::new(Mutex::new(__slice_header)) };
    { let __range_holder = { let __recv = named.clone(); let __recv_ptr: *const regexp::r#mod::Regexp = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const regexp::r#mod::Regexp }; let __result = unsafe { &*__recv_ptr }.subexp_names(); __result }.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for (i, name) in __range_values.iter().enumerate() {
        if { let __tmp_x = (*name).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        println!("{} {}", format!("{}", name), format!("{}", { let __seq = { let __seq_holder = m.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }));
    }
    } }
    println!("{}", format!("{}", format_slice(&{
        let __recv = regexp::must_compile(Arc::new(Mutex::new(Some("\\s*,\\s*".to_string()))));
        let __result = (*__recv.lock().unwrap().as_ref().unwrap()).split(
            Arc::new(Mutex::new(Some("a , b,c".to_string()))),
            Arc::new(Mutex::new(Some(-1))),
        );
        __result
    })));
    println!("{} {}", format!("{}", (*regexp::quote_meta(Arc::new(Mutex::new(Some("1+1=2?".to_string())))).lock().unwrap().as_ref().unwrap())), format!("{}", {
        let __recv = regexp::must_compile(Arc::new(Mutex::new(Some("^a.c$".to_string()))));
        let __result = (*__recv.lock().unwrap().as_ref().unwrap()).match_string(
            Arc::new(Mutex::new(Some("abc".to_string()))),
        );
        __result
    }));
    let (_, mut err) = regexp::compile(Arc::new(Mutex::new(Some("a(b".to_string()))));
    println!("{}", format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    let (mut ok, _) = regexp::match_string(Arc::new(Mutex::new(Some("^\\d+$".to_string()))), Arc::new(Mutex::new(Some("12345".to_string()))));
    println!("{} {}", format!("{}", ok), format!("{}", (*{ let __recv = re.clone(); let __recv_ptr: *const regexp::r#mod::Regexp = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const regexp::r#mod::Regexp }; let __result = unsafe { &*__recv_ptr }.string(); __result }.lock().unwrap().as_ref().unwrap())));
    let mut done = GoChannel::<bool>::new();
    let done_thread = done.clone(); let re_thread = re.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        println!("{}", format!("{}", { let __recv = re_thread.clone(); let __recv_ptr: *const regexp::r#mod::Regexp = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const regexp::r#mod::Regexp }; let __result = unsafe { &*__recv_ptr }.match_string(Arc::new(Mutex::new(Some("z@y.com".to_string())))); __result }));;
        done_thread.send(true);;;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    done.recv().unwrap_or_default();
}