#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
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

pub fn channel(label: Arc<Mutex<Option<String>>>, ch: GoChannel<i64>) -> GoChannel<i64> {
    println!("{}", format!("{}", { let __v = (*label.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    ch.clone()
}

pub fn value() -> i64 {
    println!("{}", format!("{}", "value".to_string()));
    9
}

fn main() {
    go_install_panic_exit();
    ::internal_bisect::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_godebug::__go_init_all();
    ::internal_godebugs::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::math_bits::__go_init_all();
    ::time::__go_init_all();

    let mut ready = GoChannel::<i64>::new_buffered(1 as usize);
    ready.send(4);
    let mut full = GoChannel::<i64>::new_buffered(1 as usize);
    full.send(1);
    {
    let __go_select_channel_0 = channel(Arc::new(Mutex::new(Some("receive".to_string()))), ready.clone()).clone();
    let __go_select_channel_1 = channel(Arc::new(Mutex::new(Some("send".to_string()))), full.clone()).clone();
    let mut __go_select_value_1 = Some(value());
    loop {
        if let Some(__select_recv) = __go_select_channel_0.try_recv_select() {
            let __select_ok = __select_recv.is_some();
            let n = match __select_recv { Some(__v) => __v, None => Default::default() };
            let mut n = Arc::new(Mutex::new(Some(n)));
            println!("{}", format!("{}", { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }));
            break;
        }
        if __go_select_channel_1.try_send_select(&mut __go_select_value_1) {
            println!("{}", format!("{}", "unexpected send".to_string()));
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    }
    {
    let __go_select_channel_0 = channel(Arc::new(Mutex::new(Some("full".to_string()))), full.clone()).clone();
    let mut __go_select_value_0 = Some(value());
    loop {
        if __go_select_channel_0.try_send_select(&mut __go_select_value_0) {
            println!("{}", format!("{}", "unexpected send".to_string()));
            break;
        }
        println!("{}", format!("{}", "default".to_string()));
        break;
    }
    }
    let mut target = GoChannel::<i64>::new_buffered(1 as usize);
    {
    let __go_select_channel_0 = channel(Arc::new(Mutex::new(Some("target".to_string()))), target.clone()).clone();
    let mut __go_select_value_0 = Some(value());
    loop {
        if __go_select_channel_0.try_send_select(&mut __go_select_value_0) {
            println!("{}", format!("{}", target.recv().unwrap_or_default()));
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    }
    let mut stringsCh = GoChannel::<String>::new_buffered(1 as usize);
    stringsCh.send("old".to_string());
    let stringsCh_thread = stringsCh.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        time::sleep(Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some((3 as i64 * time::MILLISECOND) as i64))))))));;
        stringsCh_thread.recv().unwrap_or_default();;;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    {
    let __go_select_channel_0 = stringsCh.clone();
    let mut __go_select_value_0 = Some("new".to_string());
    loop {
        if __go_select_channel_0.try_send_select(&mut __go_select_value_0) {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    }
    println!("{}", format!("{}", stringsCh.recv().unwrap_or_default()));
}