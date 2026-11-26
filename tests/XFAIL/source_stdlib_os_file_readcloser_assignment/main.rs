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


/// A nil interface value in a slot that holds a Box<dyn Any>, such as a
/// []any element: it copies, prints and compares as nil.
#[derive(Clone, Copy, Debug, PartialEq)]
struct GoNilAny;

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
    ::cmp::__go_init_all();
    ::errors::__go_init_all();
    ::internal_abi::__go_init_all();
    ::internal_asan::__go_init_all();
    ::internal_bisect::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_byteorder::__go_init_all();
    ::internal_filepathlite::__go_init_all();
    ::internal_goarch::__go_init_all();
    ::internal_godebug::__go_init_all();
    ::internal_godebugs::__go_init_all();
    ::internal_itoa::__go_init_all();
    ::internal_msan::__go_init_all();
    ::internal_oserror::__go_init_all();
    ::internal_poll::__go_init_all();
    ::internal_race::__go_init_all();
    ::internal_reflectlite::__go_init_all();
    ::internal_runtime_syscall::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::internal_sync::__go_init_all();
    ::internal_syscall_execenv::__go_init_all();
    ::internal_syscall_unix::__go_init_all();
    ::internal_testlog::__go_init_all();
    ::internal_unsafeheader::__go_init_all();
    ::io::__go_init_all();
    ::io_fs::__go_init_all();
    ::iter::__go_init_all();
    ::math_bits::__go_init_all();
    ::os::__go_init_all();
    ::path::__go_init_all();
    ::slices::__go_init_all();
    ::sync::__go_init_all();
    ::sync_atomic::__go_init_all();
    ::syscall::__go_init_all();
    ::time::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let (mut file, mut err) = os::open(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = os::Args.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }))));
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        std::panic::panic_any({ let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => Box::new(GoNilAny) as Box<dyn Any + Send + Sync>, Some(__err) => Box::new(Arc::new(Mutex::new(Some(__err.clone())))) as Box<dyn Any + Send + Sync> } });
    }
    let mut rc: Arc<Mutex<Option<Box<dyn io::r#mod::ReadCloser + Send + Sync>>>> = Arc::new(Mutex::new(None));
    { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(os::types::FilePtr(file.clone())) as Box<dyn io::r#mod::ReadCloser + Send + Sync>))); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *rc.lock().unwrap() = __iface_value; };
    {
        let mut err = (*rc.lock().unwrap().as_mut().unwrap()).close();;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            std::panic::panic_any({ let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => Box::new(GoNilAny) as Box<dyn Any + Send + Sync>, Some(__err) => Box::new(Arc::new(Mutex::new(Some(__err.clone())))) as Box<dyn Any + Send + Sync> } });;
        }
    }
    {
            let __go_print_arg_0 = format!("{}", "closed".to_string());
            eprintln!("{}", __go_print_arg_0)
        };
}