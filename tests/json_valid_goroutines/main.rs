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
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut ready = GoChannel::<AnonymousStruct1>::new();
    let ready_thread = ready.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        ready_thread.close();;;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    ready.recv().unwrap_or_default();
    println!("{} {} {} {} {} {} {} {} {} {} {} {} {} {} {}", format!("{}", json::valid(Arc::new(Mutex::new(Some({ let __arg_value = ({
        let mut __s = String::new();
        __s.push_str(&format!("{}", (*strings::repeat(Arc::new(Mutex::new(Some("[".to_string()))), Arc::new(Mutex::new(Some(10000)))).lock().unwrap().as_ref().unwrap())));
        __s.push_str(&format!("{}", "0".to_string()));
        __s.push_str(&format!("{}", (*strings::repeat(Arc::new(Mutex::new(Some("]".to_string()))), Arc::new(Mutex::new(Some(10000)))).lock().unwrap().as_ref().unwrap())));
        __s
    }).as_bytes().to_go_slice(); __arg_value }))))), format!("{}", json::valid(Arc::new(Mutex::new(Some({ let __arg_value = ({
        let mut __s = String::new();
        __s.push_str(&format!("{}", (*strings::repeat(Arc::new(Mutex::new(Some("[".to_string()))), Arc::new(Mutex::new(Some(10001)))).lock().unwrap().as_ref().unwrap())));
        __s.push_str(&format!("{}", "0".to_string()));
        __s.push_str(&format!("{}", (*strings::repeat(Arc::new(Mutex::new(Some("]".to_string()))), Arc::new(Mutex::new(Some(10001)))).lock().unwrap().as_ref().unwrap())));
        __s
    }).as_bytes().to_go_slice(); __arg_value }))))), format!("{}", json::valid(Arc::new(Mutex::new(Some({ let __arg_value = ({
        let mut __s = String::new();
        __s.push_str(&format!("{}", "\"".to_string()));
        __s.push_str(&format!("{}", (*strings::repeat(Arc::new(Mutex::new(Some("[".to_string()))), Arc::new(Mutex::new(Some(10001)))).lock().unwrap().as_ref().unwrap())));
        __s.push_str(&format!("{}", "\"".to_string()));
        __s
    }).as_bytes().to_go_slice(); __arg_value }))))), format!("{}", json::valid(Arc::new(Mutex::new(Some(("{\"x\":[true,null,\"a\\\\b\"]}".to_string()).as_bytes().to_go_slice()))))), format!("{}", json::valid(Arc::new(Mutex::new(Some((" \n [1,2,3] \t".to_string()).as_bytes().to_go_slice()))))), format!("{}", json::valid(Arc::new(Mutex::new(Some(("\"escaped\\u263a\"".to_string()).as_bytes().to_go_slice()))))), format!("{}", json::valid(Arc::new(Mutex::new(Some(("-1.2e+3".to_string()).as_bytes().to_go_slice()))))), format!("{}", json::valid(Arc::new(Mutex::new(Some(("1e999".to_string()).as_bytes().to_go_slice()))))), format!("{}", json::valid(Arc::new(Mutex::new(Some(("\"\\ud800\"".to_string()).as_bytes().to_go_slice()))))), format!("{}", json::valid(Arc::new(Mutex::new(Some(vec![('"' as u8), 0xff as u8, ('"' as u8)]))))), format!("{}", json::valid(Arc::new(Mutex::new(Some(("{\"x\":".to_string()).as_bytes().to_go_slice()))))), format!("{}", json::valid(Arc::new(Mutex::new(Some(("true false".to_string()).as_bytes().to_go_slice()))))), format!("{}", json::valid(Arc::new(Mutex::new(Some(("01".to_string()).as_bytes().to_go_slice()))))), format!("{}", json::valid(Arc::new(Mutex::new(Some(Vec::<u8>::new()))))), format!("{}", json::valid(Arc::new(Mutex::new(Some(vec![0xff as u8]))))));
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
