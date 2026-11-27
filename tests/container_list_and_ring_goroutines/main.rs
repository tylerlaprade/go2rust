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
        print!("{}", { let mut __print_text = String::new(); let __print_part_0 = format!("{}", format!("{}", { let __text = match (*e.lock().unwrap().as_ref().unwrap()).value.lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text })); let __print_is_string_0 = { let __print_holder = (*e.lock().unwrap().as_ref().unwrap()).value.clone(); let __print_guard = __print_holder.lock().unwrap(); __print_guard.as_ref().map_or(false, |__value| __value.is::<String>()) }; __print_text.push_str(&__print_part_0); let __print_part_1 = format!("{}", format!("{}", " ".to_string())); let __print_is_string_1 = true; if !__print_is_string_0 && !__print_is_string_1 { __print_text.push(' '); } __print_text.push_str(&__print_part_1); __print_text });
        { let new_val = { let __recv = e.clone(); let __recv_ptr: *const container_list::r#mod::Element = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const container_list::r#mod::Element }; let __result = unsafe { &*__recv_ptr }.next(); __result }.clone(); e = new_val; };
    }
    println!("{} {}", format!("{}", "| len".to_string()), format!("{}", { let __recv = l.clone(); let __recv_ptr: *const container_list::r#mod::List = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const container_list::r#mod::List }; let __result = unsafe { &*__recv_ptr }.len(); __result }));
}

fn main() {
    go_install_panic_exit();
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
    let mut e4 = { let __recv = l.clone(); let __recv_ptr = go_hint_receiver_mut(&__recv); let __result = unsafe { &mut *__recv_ptr }.push_back(Arc::new(Mutex::new(Some(Box::new(4i64) as Box<dyn Any + Send + Sync>)))); __result };
    let mut e1 = { let __recv = l.clone(); let __recv_ptr = go_hint_receiver_mut(&__recv); let __result = unsafe { &mut *__recv_ptr }.push_front(Arc::new(Mutex::new(Some(Box::new(1i64) as Box<dyn Any + Send + Sync>)))); __result };
    { let __recv = l.clone(); let __recv_ptr = go_hint_receiver_mut(&__recv); let __result = unsafe { &mut *__recv_ptr }.insert_before(Arc::new(Mutex::new(Some(Box::new(3i64) as Box<dyn Any + Send + Sync>))), e4.clone()); __result };
    { let __recv = l.clone(); let __recv_ptr = go_hint_receiver_mut(&__recv); let __result = unsafe { &mut *__recv_ptr }.insert_after(Arc::new(Mutex::new(Some(Box::new(2i64) as Box<dyn Any + Send + Sync>))), e1.clone()); __result };
    print_list(l.clone());
    { let __recv = l.clone(); let __recv_ptr: *const container_list::r#mod::List = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const container_list::r#mod::List }; let __result = unsafe { &*__recv_ptr }.move_to_back(e1.clone()); __result };
    { let __method_args = (
    e4.clone(),
    { let __recv = l.clone(); let __recv_ptr: *const container_list::r#mod::List = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const container_list::r#mod::List }; let __result = unsafe { &*__recv_ptr }.front(); __result },); { let __recv = l.clone(); let __recv_ptr: *const container_list::r#mod::List = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const container_list::r#mod::List }; let __result = unsafe { &*__recv_ptr }.move_before(__method_args.0, __method_args.1); __result } };
    print_list(l.clone());
    { let __recv = l.clone(); let __recv_ptr: *mut container_list::r#mod::List = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut container_list::r#mod::List }; let __result = unsafe { &mut *__recv_ptr }.remove(e4.clone()); __result };
    let mut other = container_list::new();
    { let __recv = other.clone(); let __recv_ptr = go_hint_receiver_mut(&__recv); let __result = unsafe { &mut *__recv_ptr }.push_back(Arc::new(Mutex::new(Some(Box::new(9i64) as Box<dyn Any + Send + Sync>)))); __result };
    { let __recv = l.clone(); let __recv_ptr = go_hint_receiver_mut(&__recv); let __result = unsafe { &mut *__recv_ptr }.push_back_list(other.clone()); __result };
    print_list(l.clone());
    let mut e = { let __recv = l.clone(); let __recv_ptr: *const container_list::r#mod::List = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const container_list::r#mod::List }; let __result = unsafe { &*__recv_ptr }.back(); __result };
    while { let __nil_result = (*e.lock().unwrap()).is_some(); __nil_result } {
        print!("{}", { let mut __print_text = String::new(); let __print_part_0 = format!("{}", format!("{}", { let __text = match (*e.lock().unwrap().as_ref().unwrap()).value.lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text })); let __print_is_string_0 = { let __print_holder = (*e.lock().unwrap().as_ref().unwrap()).value.clone(); let __print_guard = __print_holder.lock().unwrap(); __print_guard.as_ref().map_or(false, |__value| __value.is::<String>()) }; __print_text.push_str(&__print_part_0); let __print_part_1 = format!("{}", format!("{}", " ".to_string())); let __print_is_string_1 = true; if !__print_is_string_0 && !__print_is_string_1 { __print_text.push(' '); } __print_text.push_str(&__print_part_1); __print_text });
        { let new_val = { let __recv = e.clone(); let __recv_ptr: *const container_list::r#mod::Element = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const container_list::r#mod::Element }; let __result = unsafe { &*__recv_ptr }.prev(); __result }.clone(); e = new_val; };
    }
    println!();
    let mut r = container_ring::new(Arc::new(Mutex::new(Some(4))));
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = container_ring::__go_nil_recv_ring_len(r.clone()); __tmp_x < __tmp_y } {
        { let new_val = Box::new({ let __bin_i = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }); __bin_i * __bin_i }) as Box<dyn Any + Send + Sync>; *(*r.lock().unwrap().as_ref().unwrap()).value.lock().unwrap() = Some(new_val); };
        { let new_val = { let __recv = r.clone(); let __recv_ptr = go_hint_receiver_mut(&__recv); let __result = unsafe { &mut *__recv_ptr }.next(); __result }.clone(); r = new_val; };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    container_ring::__go_nil_recv_ring_r_do(r.clone(), Arc::new(Mutex::new(Some(Arc::new(move |v: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>| {
        print!("{}", { let mut __print_text = String::new(); let __print_part_0 = format!("{}", format!("{}", { let __text = match v.lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text })); let __print_is_string_0 = { let __print_holder = v.clone(); let __print_guard = __print_holder.lock().unwrap(); __print_guard.as_ref().map_or(false, |__value| __value.is::<String>()) }; __print_text.push_str(&__print_part_0); let __print_part_1 = format!("{}", format!("{}", " ".to_string())); let __print_is_string_1 = true; if !__print_is_string_0 && !__print_is_string_1 { __print_text.push(' '); } __print_text.push_str(&__print_part_1); __print_text });
    }) as Arc<dyn Fn(Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> () + Send + Sync>))));
    println!("{}", format!("{}", container_ring::__go_nil_recv_ring_len(r.clone())));
}