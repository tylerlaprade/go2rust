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

    let mut requests = GoChannel::<i64>::new_buffered(5 as usize);
    let mut i = Arc::new(Mutex::new(Some(1i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 5; __tmp_x <= __tmp_y } {
        requests.send((*i.lock().unwrap().as_ref().unwrap()));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    requests.close();
    let mut limiter = time::tick(Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some((100 as i64 * time::MILLISECOND) as i64))))))));
    for req in requests.clone() {
        limiter.recv().unwrap_or_default();
        println!("{} {}", format!("{}", "regular request".to_string()), format!("{}", req));
    }
    let mut burstyLimiter = GoChannel::<time::r#mod::Time>::new_buffered(3 as usize);
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x < __tmp_y } {
        burstyLimiter.send((*time::unix(Arc::new(Mutex::new(Some(0 as i64))), Arc::new(Mutex::new(Some(0 as i64)))).lock().unwrap().as_ref().unwrap()).clone());
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    let burstyLimiter_thread = burstyLimiter.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        for t in time::tick(Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some((100 as i64 * time::MILLISECOND) as i64)))))))).clone() {
        burstyLimiter_thread.send(t.clone());
    };;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    let mut burstyRequests = GoChannel::<i64>::new_buffered(5 as usize);
    let mut i = Arc::new(Mutex::new(Some(1i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 5; __tmp_x <= __tmp_y } {
        burstyRequests.send((*i.lock().unwrap().as_ref().unwrap()));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    burstyRequests.close();
    for req in burstyRequests.clone() {
        burstyLimiter.recv().unwrap_or_default();
        println!("{} {}", format!("{}", "bursty request".to_string()), format!("{}", req));
    }
}