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
    ::flag::__go_init_all();
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

    let mut fs = flag::new_flag_set(Arc::new(Mutex::new(Some("demo".to_string()))), Arc::new(Mutex::new(Some(flag::r#mod::ErrorHandling(Arc::new(Mutex::new(Some(flag::CONTINUE_ON_ERROR as i64))))))));
    let mut n = { let __recv = fs.clone(); let __recv_ptr: *mut flag::r#mod::FlagSet = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut flag::r#mod::FlagSet }; let __result = unsafe { &mut *__recv_ptr }.int(Arc::new(Mutex::new(Some("n".to_string()))), Arc::new(Mutex::new(Some(1))), Arc::new(Mutex::new(Some("count".to_string())))); __result };
    let mut name = { let __recv = fs.clone(); let __recv_ptr: *mut flag::r#mod::FlagSet = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut flag::r#mod::FlagSet }; let __result = unsafe { &mut *__recv_ptr }.string(Arc::new(Mutex::new(Some("name".to_string()))), Arc::new(Mutex::new(Some("x".to_string()))), Arc::new(Mutex::new(Some("name".to_string())))); __result };
    let mut verbose = { let __recv = fs.clone(); let __recv_ptr: *mut flag::r#mod::FlagSet = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut flag::r#mod::FlagSet }; let __result = unsafe { &mut *__recv_ptr }.bool(Arc::new(Mutex::new(Some("v".to_string()))), Arc::new(Mutex::new(Some(false))), Arc::new(Mutex::new(Some("verbose".to_string())))); __result };
    let mut d = { let __recv = fs.clone(); let __recv_ptr: *mut flag::r#mod::FlagSet = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut flag::r#mod::FlagSet }; let __result = unsafe { &mut *__recv_ptr }.duration(Arc::new(Mutex::new(Some("d".to_string()))), Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some(time::SECOND as i64))))))), Arc::new(Mutex::new(Some("delay".to_string())))); __result };
    let mut limit: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    { let __recv = fs.clone(); let __recv_ptr: *mut flag::r#mod::FlagSet = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut flag::r#mod::FlagSet }; let __result = unsafe { &mut *__recv_ptr }.int_var(limit.clone(), Arc::new(Mutex::new(Some("limit".to_string()))), Arc::new(Mutex::new(Some(10))), Arc::new(Mutex::new(Some("limit".to_string())))); __result };
    let mut err = { let __recv = fs.clone(); let __recv_ptr: *mut flag::r#mod::FlagSet = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut flag::r#mod::FlagSet }; let __result = unsafe { &mut *__recv_ptr }.parse(Arc::new(Mutex::new(Some(vec!["-n".to_string(), "5".to_string(), "-name".to_string(), "gopher".to_string(), "-v".to_string(), "-d".to_string(), "250ms".to_string(), "-limit=3".to_string(), "rest".to_string(), "args".to_string()])))); __result };
    { let __go_print_arg_1 = { let __recv = fs.clone(); let __recv_ptr: *const flag::r#mod::FlagSet = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const flag::r#mod::FlagSet }; let __result = unsafe { &*__recv_ptr }.args(); __result }; let __go_print_arg_2 = Arc::new(Mutex::new(Some({ let __recv = fs.clone(); let __recv_ptr: *const flag::r#mod::FlagSet = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const flag::r#mod::FlagSet }; let __result = unsafe { &*__recv_ptr }.n_arg(); __result }))); let __go_print_value = println!("{} {} {} {} {} {} {} {}", format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*name.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*verbose.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*limit.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", format_slice(&__go_print_arg_1)), format!("{}", { let __v = (*__go_print_arg_2.lock().unwrap().as_ref().unwrap()).clone(); __v })); __go_print_value };
    { let __recv = fs.clone(); let __recv_ptr: *const flag::r#mod::FlagSet = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const flag::r#mod::FlagSet }; let __result = unsafe { &*__recv_ptr }.visit_all(
        Arc::new(Mutex::new(Some(Arc::new(move |f: Arc<Mutex<Option<flag::r#mod::Flag>>>| {
        println!("{} {} {}", format!("{}", (*{ let __field = (*f.lock().unwrap().as_ref().unwrap()).name.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", (*(*(*f.lock().unwrap().as_ref().unwrap()).value.lock().unwrap().as_ref().unwrap()).string().lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __field = (*f.lock().unwrap().as_ref().unwrap()).def_value.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()));
    }) as Arc<dyn Fn(Arc<Mutex<Option<flag::r#mod::Flag>>>) -> () + Send + Sync>))),
    ); __result };
}