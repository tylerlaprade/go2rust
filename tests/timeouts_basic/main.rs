#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    ::internal_bisect::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_godebug::__go_init_all();
    ::internal_godebugs::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::math_bits::__go_init_all();
    ::time::__go_init_all();

    let mut c1 = GoChannel::<String>::new_buffered(1 as usize);
    let c1_thread = c1.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        time::sleep(Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some((1 as i64 * time::SECOND) as i64))))))));;
        c1_thread.send("result 1".to_string());;;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    let mut timeout1 = time::after(Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some((500 as i64 * time::MILLISECOND) as i64))))))));
    {
    let __go_select_channel_0 = c1.clone();
    let __go_select_channel_1 = timeout1.clone();
    loop {
        if let Some(__select_recv) = __go_select_channel_0.try_recv_select() {
            let __select_ok = __select_recv.is_some();
            let res = match __select_recv { Some(__v) => __v, None => Default::default() };
            let mut res = Arc::new(Mutex::new(Some(res)));
            println!("{}", format!("{}", { let __v = (*res.lock().unwrap().as_ref().unwrap()).clone(); __v }));
            break;
        }
        if let Some(_) = __go_select_channel_1.try_recv_select() {
            println!("{}", format!("{}", "timeout 1".to_string()));
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    }
    let mut c2 = GoChannel::<String>::new_buffered(1 as usize);
    let c2_thread = c2.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        time::sleep(Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some((1 as i64 * time::SECOND) as i64))))))));;
        c2_thread.send("result 2".to_string());;;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    let mut timeout2 = time::after(Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some((1500 as i64 * time::MILLISECOND) as i64))))))));
    {
    let __go_select_channel_0 = c2.clone();
    let __go_select_channel_1 = timeout2.clone();
    loop {
        if let Some(__select_recv) = __go_select_channel_0.try_recv_select() {
            let __select_ok = __select_recv.is_some();
            let res = match __select_recv { Some(__v) => __v, None => Default::default() };
            let mut res = Arc::new(Mutex::new(Some(res)));
            println!("{}", format!("{}", { let __v = (*res.lock().unwrap().as_ref().unwrap()).clone(); __v }));
            break;
        }
        if let Some(_) = __go_select_channel_1.try_recv_select() {
            println!("{}", format!("{}", "timeout 2".to_string()));
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    }
}