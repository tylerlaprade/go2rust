#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    ::internal_stringslite::__go_init_all();
    ::math::__go_init_all();
    ::math_bits::__go_init_all();
    ::strconv::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut started = GoChannel::<bool>::new();
    let started_thread = started.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        started_thread.send(true);;;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    started.recv().unwrap_or_default();
    let mut total = Arc::new(Mutex::new(Some(0i64)));
    { let __range_holder = Arc::new(Mutex::new(Some({ let __s = "1, 2,x, 30".to_string(); let __sep = ",".to_string(); __s.split(&__sep).map(|__part| __part.to_string()).collect::<Vec<String>>() }))).clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for mut field in __range_values.iter().cloned() {
        { let new_val = (*Arc::new(Mutex::new(Some({ let __s = field.clone(); __s.trim().to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); field = new_val; };
        let (mut n, mut err) = strconv::atoi(Arc::new(Mutex::new(Some(field.clone()))));
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        println!("{} {} {}", format!("{}", "skip".to_string()), format!("{}", field), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
        continue
    }
        { let __rhs = n; let mut guard = total.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        println!("{} {} {}", format!("{}", field), format!("{}", "->".to_string()), format!("{}", n));
    } }
    for word in &vec!["true".to_string(), "nope".to_string()] {
        let (mut b, mut err) = strconv::parse_bool(Arc::new(Mutex::new(Some((*word).clone()))));
        println!("{} {} {}", format!("{}", word), format!("{}", b), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    }
    println!("{}", format!("{}", { let __v = (*total.lock().unwrap().as_ref().unwrap()).clone(); __v }));
}