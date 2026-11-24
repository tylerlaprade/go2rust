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


static __GO_OS_ARGS: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<String>>>>> =
    std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(Some(std::env::args().collect::<Vec<String>>()))));

fn go_os_args() -> std::sync::Arc<std::sync::Mutex<Option<Vec<String>>>> {
    __GO_OS_ARGS.clone()
}

fn main() {
    ::io::__go_init_all();

    let (mut file, mut err) = os::open({ let __seq = { let __seq_holder = go_os_args().clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() });
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        std::panic::panic_any({ let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => Box::new(Arc::new(Mutex::new(Some(__err.clone())))) as Box<dyn Any + Send + Sync> } });
    }
    let mut rc: Arc<Mutex<Option<Box<dyn io::r#mod::ReadCloser + Send + Sync>>>> = Arc::new(Mutex::new(None));
    { let __iface_handle = Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = file.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn io::r#mod::ReadCloser + Send + Sync>))); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *rc.lock().unwrap() = __iface_value; };
    {
        let mut err = (*rc.lock().unwrap().as_ref().unwrap()).close();;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            std::panic::panic_any({ let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => Box::new(Arc::new(Mutex::new(Some(__err.clone())))) as Box<dyn Any + Send + Sync> } });;
        }
    }
    {
            let __go_print_arg_0 = format!("{}", "closed".to_string());
            eprintln!("{}", __go_print_arg_0)
        };
}