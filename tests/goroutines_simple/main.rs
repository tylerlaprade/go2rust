#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn f(from: Arc<Mutex<Option<String>>>) {
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x < __tmp_y } {
        println!("{} {} {}", format!("{}", { let __v = (*from.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", ":".to_string()), format!("{}", { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

fn main() {
    ::internal_bisect::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_godebug::__go_init_all();
    ::internal_godebugs::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::math_bits::__go_init_all();
    ::time::__go_init_all();

    f(Arc::new(Mutex::new(Some("direct".to_string()))));
    std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        time::sleep(Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some((50 as i64 * time::MILLISECOND) as i64))))))));;
        f(Arc::new(Mutex::new(Some("goroutine".to_string()))));;;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    let __go_arg_0 = Arc::new(Mutex::new(Some("going".to_string()))); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let __closure = move |msg: Arc<Mutex<Option<String>>>| {
            time::sleep(Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some((10 as i64 * time::MILLISECOND) as i64))))))));;
            println!("{}", format!("{}", { let __v = (*msg.lock().unwrap().as_ref().unwrap()).clone(); __v }));;
        };
        __closure(__go_arg_0);
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    time::sleep(Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some((100 as i64 * time::MILLISECOND) as i64))))))));
    println!("{}", format!("{}", "done".to_string()));
}