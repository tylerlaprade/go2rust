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

#[allow(unused_macros)]
macro_rules! print {
    ($($__arg:tt)*) => {{
        std::print!($($__arg)*);
        std::io::Write::flush(&mut std::io::stdout()).ok();
    }};
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

pub fn basic_select() {
    let mut ch1 = GoChannel::<String>::new();
    let mut ch2 = GoChannel::<String>::new();
    let ch1_thread = ch1.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        time::sleep(Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some((100 as i64 * time::MILLISECOND) as i64))))))));;
        ch1_thread.send("from ch1".to_string());;;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    let ch2_thread = ch2.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        time::sleep(Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some((200 as i64 * time::MILLISECOND) as i64))))))));;
        ch2_thread.send("from ch2".to_string());;;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    {
    let __go_select_channel_0 = ch1.clone();
    let __go_select_channel_1 = ch2.clone();
    loop {
        if let Some(__select_recv) = __go_select_channel_0.try_recv_select() {
            let __select_ok = __select_recv.is_some();
            let msg1 = match __select_recv { Some(__v) => __v, None => Default::default() };
            let mut msg1 = Arc::new(Mutex::new(Some(msg1)));
            println!("{} {}", format!("{}", "Received:".to_string()), format!("{}", { let __v = (*msg1.lock().unwrap().as_ref().unwrap()).clone(); __v }));
            break;
        }
        if let Some(__select_recv) = __go_select_channel_1.try_recv_select() {
            let __select_ok = __select_recv.is_some();
            let msg2 = match __select_recv { Some(__v) => __v, None => Default::default() };
            let mut msg2 = Arc::new(Mutex::new(Some(msg2)));
            println!("{} {}", format!("{}", "Received:".to_string()), format!("{}", { let __v = (*msg2.lock().unwrap().as_ref().unwrap()).clone(); __v }));
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    }
}

pub fn select_with_timeout() {
    let mut ch = GoChannel::<String>::new();
    let mut timeout = time::after(Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some((100 as i64 * time::MILLISECOND) as i64))))))));
    let ch_thread = ch.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        time::sleep(Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some((300 as i64 * time::MILLISECOND) as i64))))))));;
        ch_thread.send("delayed message".to_string());;;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    {
    let __go_select_channel_0 = ch.clone();
    let __go_select_channel_1 = timeout.clone();
    loop {
        if let Some(__select_recv) = __go_select_channel_0.try_recv_select() {
            let __select_ok = __select_recv.is_some();
            let msg = match __select_recv { Some(__v) => __v, None => Default::default() };
            let mut msg = Arc::new(Mutex::new(Some(msg)));
            println!("{} {}", format!("{}", "Got message:".to_string()), format!("{}", { let __v = (*msg.lock().unwrap().as_ref().unwrap()).clone(); __v }));
            break;
        }
        if let Some(_) = __go_select_channel_1.try_recv_select() {
            println!("{}", format!("{}", "Timeout occurred".to_string()));
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    }
}

pub fn select_with_default() {
    let mut ch = GoChannel::<String>::new_buffered(1 as usize);
    {
    let __go_select_channel_0 = ch.clone();
    let mut __go_select_value_0 = Some("hello".to_string());
    loop {
        if __go_select_channel_0.try_send_select(&mut __go_select_value_0) {
            println!("{}", format!("{}", "Sent message".to_string()));
            break;
        }
        println!("{}", format!("{}", "Channel full, couldn't send".to_string()));
        break;
    }
    }
    {
    let __go_select_channel_0 = ch.clone();
    loop {
        if let Some(__select_recv) = __go_select_channel_0.try_recv_select() {
            let __select_ok = __select_recv.is_some();
            let msg = match __select_recv { Some(__v) => __v, None => Default::default() };
            let mut msg = Arc::new(Mutex::new(Some(msg)));
            println!("{} {}", format!("{}", "Received:".to_string()), format!("{}", { let __v = (*msg.lock().unwrap().as_ref().unwrap()).clone(); __v }));
            break;
        }
        println!("{}", format!("{}", "No message available".to_string()));
        break;
    }
    }
    {
    let __go_select_channel_0 = ch.clone();
    loop {
        if let Some(__select_recv) = __go_select_channel_0.try_recv_select() {
            let __select_ok = __select_recv.is_some();
            let msg = match __select_recv { Some(__v) => __v, None => Default::default() };
            let mut msg = Arc::new(Mutex::new(Some(msg)));
            println!("{} {}", format!("{}", "Received:".to_string()), format!("{}", { let __v = (*msg.lock().unwrap().as_ref().unwrap()).clone(); __v }));
            break;
        }
        println!("{}", format!("{}", "No message available".to_string()));
        break;
    }
    }
}

pub fn select_loop() {
    let mut ch1 = GoChannel::<i64>::new();
    let mut ch2 = GoChannel::<i64>::new();
    let mut quit = GoChannel::<bool>::new();
    let ch1_thread = ch1.clone(); let ch2_thread = ch2.clone(); let quit_thread = quit.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        time::sleep(Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some((50 as i64 * time::MILLISECOND) as i64))))))));;
        ch1_thread.send(0);;
        time::sleep(Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some((50 as i64 * time::MILLISECOND) as i64))))))));;
        ch2_thread.send(10);;
        time::sleep(Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some((50 as i64 * time::MILLISECOND) as i64))))))));;
        ch1_thread.send(1);;
        time::sleep(Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some((50 as i64 * time::MILLISECOND) as i64))))))));;
        ch2_thread.send(11);;
        time::sleep(Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some((50 as i64 * time::MILLISECOND) as i64))))))));;
        ch1_thread.send(2);;
        time::sleep(Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some((50 as i64 * time::MILLISECOND) as i64))))))));;
        ch2_thread.send(12);;
        time::sleep(Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some((50 as i64 * time::MILLISECOND) as i64))))))));;
        quit_thread.send(true);;;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    println!("{}", format!("{}", "Starting select loop:".to_string()));
    loop {
        {
    let __go_select_channel_0 = ch1.clone();
    let __go_select_channel_1 = ch2.clone();
    let __go_select_channel_2 = quit.clone();
    loop {
        if let Some(__select_recv) = __go_select_channel_0.try_recv_select() {
            let __select_ok = __select_recv.is_some();
            let val1 = match __select_recv { Some(__v) => __v, None => Default::default() };
            let mut val1 = Arc::new(Mutex::new(Some(val1)));
            print!("From ch1: {}\n", { let __v = (*val1.lock().unwrap().as_ref().unwrap()).clone(); __v });
            break;
        }
        if let Some(__select_recv) = __go_select_channel_1.try_recv_select() {
            let __select_ok = __select_recv.is_some();
            let val2 = match __select_recv { Some(__v) => __v, None => Default::default() };
            let mut val2 = Arc::new(Mutex::new(Some(val2)));
            print!("From ch2: {}\n", { let __v = (*val2.lock().unwrap().as_ref().unwrap()).clone(); __v });
            break;
        }
        if let Some(_) = __go_select_channel_2.try_recv_select() {
            println!("{}", format!("{}", "Quit signal received".to_string()));
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    }
    }
}

pub fn select_with_send() {
    let mut ch1 = GoChannel::<String>::new_buffered(1 as usize);
    let mut ch2 = GoChannel::<String>::new_buffered(1 as usize);
    ch2.send("preloaded ch2".to_string());
    {
    let __go_select_channel_0 = ch1.clone();
    let mut __go_select_value_0 = Some("message to ch1".to_string());
    let __go_select_channel_1 = ch2.clone();
    let mut __go_select_value_1 = Some("message to ch2".to_string());
    loop {
        if __go_select_channel_0.try_send_select(&mut __go_select_value_0) {
            println!("{}", format!("{}", "Sent to ch1".to_string()));
            break;
        }
        if __go_select_channel_1.try_send_select(&mut __go_select_value_1) {
            println!("{}", format!("{}", "Sent to ch2".to_string()));
            break;
        }
        println!("{}", format!("{}", "Both channels busy".to_string()));
        break;
    }
    }
    println!("{} {}", format!("{}", "Reading from ch1:".to_string()), format!("{}", ch1.recv().unwrap_or_default()));
    {
    let __go_select_channel_0 = ch2.clone();
    loop {
        if let Some(__select_recv) = __go_select_channel_0.try_recv_select() {
            let __select_ok = __select_recv.is_some();
            let msg = match __select_recv { Some(__v) => __v, None => Default::default() };
            let mut msg = Arc::new(Mutex::new(Some(msg)));
            println!("{} {}", format!("{}", "Reading from ch2:".to_string()), format!("{}", { let __v = (*msg.lock().unwrap().as_ref().unwrap()).clone(); __v }));
            break;
        }
        println!("{}", format!("{}", "ch2 is empty".to_string()));
        break;
    }
    }
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

    println!("{}", format!("{}", "=== Basic select ===".to_string()));
    basic_select();
    println!("{}", format!("{}", "\n=== Select with timeout ===".to_string()));
    select_with_timeout();
    println!("{}", format!("{}", "\n=== Select with default ===".to_string()));
    select_with_default();
    println!("{}", format!("{}", "\n=== Select with send ===".to_string()));
    select_with_send();
    println!("{}", format!("{}", "\n=== Select loop ===".to_string()));
    select_loop();
    println!("{}", format!("{}", "\n=== All examples completed ===".to_string()));
}