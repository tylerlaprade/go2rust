#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::fmt::{Display};
use std::sync::{Arc, Mutex};
use std::thread;

#[allow(unused_macros)]
macro_rules! print {
    ($($__arg:tt)*) => {{
        std::print!($($__arg)*);
        std::io::Write::flush(&mut std::io::stdout()).ok();
    }};
}

fn format_any(value: &(dyn Any + Send + Sync)) -> String {
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
    } else {
        panic!("go2rust: cannot print this value held in an interface: its type has no formatting case yet")
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

pub fn print_list(l: Arc<Mutex<Option<container_list::r#mod::List>>>) {
    let mut e = { let __recv = l.clone(); let __recv_ptr: *const container_list::r#mod::List = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const container_list::r#mod::List }; let __result = unsafe { &*__recv_ptr }.front(); __result };
    while { let __nil_result = (*e.lock().unwrap()).is_some(); __nil_result } {
        print!("{}{}", format!("{}", { let __text = match (*e.lock().unwrap().as_ref().unwrap()).value.lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }), format!("{}", " ".to_string()));
        { let new_val = { let __recv = e.clone(); let __recv_ptr: *const container_list::r#mod::Element = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const container_list::r#mod::Element }; let __result = unsafe { &*__recv_ptr }.next(); __result }.clone(); e = new_val; };
    }
    println!("{} {}", format!("{}", "| len".to_string()), format!("{}", { let __recv = l.clone(); let __recv_ptr: *const container_list::r#mod::List = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const container_list::r#mod::List }; let __result = unsafe { &*__recv_ptr }.len(); __result }));
}

fn main() {
    ::container_list::__go_init_all();
    ::container_ring::__go_init_all();

    let mut done = GoChannel::<bool>::new();
    let done_thread = done.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        done_thread.send(true);;;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    done.recv().unwrap_or_default();
    let mut l = container_list::new();
    let mut e4 = { let __recv = l.clone(); let __recv_ptr: *mut container_list::r#mod::List = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut container_list::r#mod::List }; let __result = unsafe { &mut *__recv_ptr }.push_back(Arc::new(Mutex::new(Some(Box::new(4i64) as Box<dyn Any + Send + Sync>)))); __result };
    let mut e1 = { let __recv = l.clone(); let __recv_ptr: *mut container_list::r#mod::List = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut container_list::r#mod::List }; let __result = unsafe { &mut *__recv_ptr }.push_front(Arc::new(Mutex::new(Some(Box::new(1i64) as Box<dyn Any + Send + Sync>)))); __result };
    { let __recv = l.clone(); let __recv_ptr: *mut container_list::r#mod::List = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut container_list::r#mod::List }; let __result = unsafe { &mut *__recv_ptr }.insert_before(Arc::new(Mutex::new(Some(Box::new(3i64) as Box<dyn Any + Send + Sync>))), e4.clone()); __result };
    { let __recv = l.clone(); let __recv_ptr: *mut container_list::r#mod::List = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut container_list::r#mod::List }; let __result = unsafe { &mut *__recv_ptr }.insert_after(Arc::new(Mutex::new(Some(Box::new(2i64) as Box<dyn Any + Send + Sync>))), e1.clone()); __result };
    print_list(l.clone());
    { let __recv = l.clone(); let __recv_ptr: *const container_list::r#mod::List = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const container_list::r#mod::List }; let __result = unsafe { &*__recv_ptr }.move_to_back(e1.clone()); __result };
    { let __method_args = (
    e4.clone(),
    { let __recv = l.clone(); let __recv_ptr: *const container_list::r#mod::List = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const container_list::r#mod::List }; let __result = unsafe { &*__recv_ptr }.front(); __result },); { let __recv = l.clone(); let __recv_ptr: *const container_list::r#mod::List = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const container_list::r#mod::List }; let __result = unsafe { &*__recv_ptr }.move_before(__method_args.0, __method_args.1); __result } };
    print_list(l.clone());
    { let __recv = l.clone(); let __recv_ptr: *mut container_list::r#mod::List = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut container_list::r#mod::List }; let __result = unsafe { &mut *__recv_ptr }.remove(e4.clone()); __result };
    let mut other = container_list::new();
    { let __recv = other.clone(); let __recv_ptr: *mut container_list::r#mod::List = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut container_list::r#mod::List }; let __result = unsafe { &mut *__recv_ptr }.push_back(Arc::new(Mutex::new(Some(Box::new(9i64) as Box<dyn Any + Send + Sync>)))); __result };
    { let __recv = l.clone(); let __recv_ptr: *mut container_list::r#mod::List = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut container_list::r#mod::List }; let __result = unsafe { &mut *__recv_ptr }.push_back_list(other.clone()); __result };
    print_list(l.clone());
    let mut e = { let __recv = l.clone(); let __recv_ptr: *const container_list::r#mod::List = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const container_list::r#mod::List }; let __result = unsafe { &*__recv_ptr }.back(); __result };
    while { let __nil_result = (*e.lock().unwrap()).is_some(); __nil_result } {
        print!("{}{}", format!("{}", { let __text = match (*e.lock().unwrap().as_ref().unwrap()).value.lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }), format!("{}", " ".to_string()));
        { let new_val = { let __recv = e.clone(); let __recv_ptr: *const container_list::r#mod::Element = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const container_list::r#mod::Element }; let __result = unsafe { &*__recv_ptr }.prev(); __result }.clone(); e = new_val; };
    }
    println!();
    let mut r = container_ring::new(Arc::new(Mutex::new(Some(4))));
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = container_ring::__go_nil_recv_ring_len(r.clone()); __tmp_x < __tmp_y } {
        { let new_val = Box::new({ let __bin_i = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }); __bin_i * __bin_i }) as Box<dyn Any + Send + Sync>; *(*r.lock().unwrap().as_ref().unwrap()).value.lock().unwrap() = Some(new_val); };
        { let new_val = { let __recv = r.clone(); let __recv_ptr: *mut container_ring::r#mod::Ring = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut container_ring::r#mod::Ring }; let __result = unsafe { &mut *__recv_ptr }.next(); __result }.clone(); r = new_val; };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    container_ring::__go_nil_recv_ring_r_do(r.clone(), Arc::new(Mutex::new(Some(Arc::new(move |v: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>| {
        print!("{}{}", format!("{}", { let __text = match v.lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }), format!("{}", " ".to_string()));
    }) as Arc<dyn Fn(Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> () + Send + Sync>))));
    println!("{}", format!("{}", container_ring::__go_nil_recv_ring_len(r.clone())));
}