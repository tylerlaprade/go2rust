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

#[derive(Clone, Default)]
pub struct checkerInputs {
    pub fset: Arc<Mutex<Option<go_token::position::FileSet>>>,
    pub pkg: Arc<Mutex<Option<go_types::package::Package>>>,
    pub info: Arc<Mutex<Option<go_types::api::Info>>>,
}

impl checkerInputs {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.fset.clone();
        let __go_clone_1_0 = self.pkg.clone();
        let __go_clone_2_0 = self.info.clone();
        Self {
            fset: __go_clone_0_0,
            pkg: __go_clone_1_0,
            info: __go_clone_2_0,
        }
    }
}

impl std::fmt::Display for checkerInputs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.fset.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", { let __guard = self.pkg.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_2 = format!("{}", { let __guard = self.info.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


fn main() {
    go_install_panic_exit();
    ::bytes::__go_init_all();
    ::cmp::__go_init_all();
    ::container_heap::__go_init_all();
    ::go_ast::__go_init_all();
    ::go_constant::__go_init_all();
    ::go_token::__go_init_all();
    ::go_types::__go_init_all();
    ::go_version::__go_init_all();
    ::internal_abi::__go_init_all();
    ::internal_bisect::__go_init_all();
    ::internal_buildcfg::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_cpu::__go_init_all();
    ::internal_filepathlite::__go_init_all();
    ::internal_goarch::__go_init_all();
    ::internal_godebug::__go_init_all();
    ::internal_godebugs::__go_init_all();
    ::internal_goexperiment::__go_init_all();
    ::internal_gover::__go_init_all();
    ::internal_goversion::__go_init_all();
    ::internal_race::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::internal_sync::__go_init_all();
    ::internal_types_errors::__go_init_all();
    ::io::__go_init_all();
    ::math::__go_init_all();
    ::math_big::__go_init_all();
    ::math_bits::__go_init_all();
    ::path_filepath::__go_init_all();
    ::slices::__go_init_all();
    ::sort::__go_init_all();
    ::strconv::__go_init_all();
    ::strings::__go_init_all();
    ::sync::__go_init_all();
    ::sync_atomic::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

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
    let mut inputs = Arc::new(Mutex::new(Some(checkerInputs {
        fset: go_token::new_file_set().clone(),
        pkg: go_types::new_package(Arc::new(Mutex::new(Some("example.com/p".to_string()))), Arc::new(Mutex::new(Some("p".to_string())))).clone(),
        info: Arc::new(Mutex::new(Some(go_types::api::Info { ..Default::default() }))).clone(),
        ..Default::default()
    })));
    let mut checker = go_types::new_checker(Arc::new(Mutex::new(None)), { let __field = (*inputs.lock().unwrap().as_ref().unwrap()).fset.clone(); __field }, { let __field = (*inputs.lock().unwrap().as_ref().unwrap()).pkg.clone(); __field }, { let __field = (*inputs.lock().unwrap().as_ref().unwrap()).info.clone(); __field });
    println!("{}", format!("{}", { let __nil_result = (*checker.lock().unwrap()).is_some(); __nil_result }));
}

impl GoValueClone for checkerInputs {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
