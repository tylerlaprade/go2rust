#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};


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
    ::internal_filepathlite::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::path_filepath::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let (mut entries, mut err) = os::read_dir("data".to_string());
    let mut entries = { let __slice_header = GoSliceHeader::go_slice_header(&entries); Arc::new(Mutex::new(__slice_header)) };
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        println!("{}", format!("{}", "read error".to_string()));
        return;
    }
    { let __range_holder = entries.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for entry in __range_values.iter() {
        let mut joined = path_filepath::join(Arc::new(Mutex::new(Some(vec!["data".to_string(), { let __arg_value = (*entry.name().lock().unwrap().as_ref().unwrap()).clone(); __arg_value }]))));
        println!("{} {} {} {}", format!("{}", "entry".to_string()), format!("{}", { let __v = (*joined.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", (*path_filepath::base(Arc::new(Mutex::new(Some({ let __arg_holder = joined.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap())), format!("{}", entry.is_dir()));
    } }
    let (mut info, __tmp_1) = os::stat("data/nested".to_string()); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1;;
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        println!("{}", format!("{}", "stat error".to_string()));
        return;
    }
    println!("{} {}", format!("{}", "nested".to_string()), format!("{}", (*info.lock().unwrap().as_ref().unwrap()).is_dir()));
}