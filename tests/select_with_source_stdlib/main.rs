#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;


fn go_install_panic_exit() {
    std::panic::set_hook(Box::new(|__info| {
        let __text = go_panic_payload_text(__info.payload());
        {
            use std::io::Write;
            std::io::stdout().flush().ok();
        }
        eprintln!("panic: {}", __text);
        std::process::exit(2);
    }));
}

fn go_panic_payload_text(__payload: &dyn Any) -> String {
    if let Some(__text) = __payload.downcast_ref::<String>() {
        return go_runtime_error_text(__text.clone());
    }
    if let Some(__text) = __payload.downcast_ref::<&'static str>() {
        return go_runtime_error_text(__text.to_string());
    }
    if let Some(__inner) = __payload.downcast_ref::<Box<dyn Any + Send + Sync>>() {
        let __text = go_panic_payload_text(&**__inner);
        return __text;
    }
    if let Some(__inner) = __payload.downcast_ref::<Box<dyn Any + Send>>() {
        return go_panic_payload_text(&**__inner);
    }
    if let Some(__inner) = __payload.downcast_ref::<Box<dyn Any>>() {
        return go_panic_payload_text(&**__inner);
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
    if let Some(__value) = __payload.downcast_ref::<f64>() {
        // Go prints a float in a panic as +1.500000e+000.
        if __value.is_nan() {
            return String::from("NaN");
        }
        if __value.is_infinite() {
            return String::from(if *__value > 0.0 { "+Inf" } else { "-Inf" });
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

fn main() {
    go_install_panic_exit();
    ::internal_bisect::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_godebug::__go_init_all();
    ::internal_godebugs::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::strings::__go_init_all();
    ::time::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut words = GoChannel::<String>::new();
    let mut done = GoChannel::<AnonymousStruct1>::new();
    let done_thread = done.clone(); let words_thread = words.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        { let __range_holder = strings::fields(Arc::new(Mutex::new(Some("select works with strings".to_string())))).clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for w in __range_values.iter() {
        words_thread.send((*w).clone());
    } };
        done_thread.close();;;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    let mut got: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
    loop {
        {
    let __go_select_channel_0 = words.clone();
    let __go_select_channel_1 = done.clone();
    let __go_select_channel_2 = time::after(Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some((5 as i64 * time::SECOND) as i64)))))))).clone();
    loop {
        if let Some(__select_recv) = __go_select_channel_0.try_recv_select() {
            let __select_ok = __select_recv.is_some();
            let w = match __select_recv { Some(__v) => __v, None => Default::default() };
            let mut w = Arc::new(Mutex::new(Some(w)));
            { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = got.clone(); let __append_value = (*strings::to_upper(Arc::new(Mutex::new(Some({ let __arg_holder = w.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *got.lock().unwrap() = __slice_header; };
            continue;
            break;
        }
        if let Some(_) = __go_select_channel_1.try_recv_select() {
            break;
        }
        if let Some(_) = __go_select_channel_2.try_recv_select() {
            println!("{}", format!("{}", "timeout".to_string()));
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    }
        break
    }
    println!("{}", format!("{}", (*strings::join(got.clone(), Arc::new(Mutex::new(Some(" ".to_string())))).lock().unwrap().as_ref().unwrap())));
    let mut ticks = Arc::new(Mutex::new(Some(0i64)));
    let mut timer = time::after(Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some((20 as i64 * time::MILLISECOND) as i64))))))));
    let mut stop = Arc::new(Mutex::new(Some(false)));
    while !{ let __v = (*stop.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        {
    let __go_select_channel_0 = timer.clone();
    loop {
        if let Some(_) = __go_select_channel_0.try_recv_select() {
            { let new_val = true; *stop.lock().unwrap() = Some(new_val); };
            break;
        }
        { let mut guard = ticks.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        time::sleep(Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some(time::MILLISECOND as i64))))))));
        break;
    }
    }
    }
    println!("{}", format!("{}", { let __tmp_x = { let __v = (*ticks.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y }));
}

#[derive(Debug, Clone, Default)]
pub struct AnonymousStruct1 {
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
    }
}


impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}
