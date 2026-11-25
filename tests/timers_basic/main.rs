#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

fn main() {
    ::internal_bisect::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_godebug::__go_init_all();
    ::internal_godebugs::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::math_bits::__go_init_all();
    ::time::__go_init_all();

    let mut timer1 = time::new_timer(Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some((1 as i64 * time::SECOND) as i64))))))));
    { let __channel = (*timer1.lock().unwrap().as_ref().unwrap()).c.clone(); __channel }.recv().unwrap_or_default();
    println!("{}", format!("{}", "Timer 1 fired".to_string()));
    let mut timer2 = time::new_timer(Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some((500 as i64 * time::MILLISECOND) as i64))))))));
    let mut stop2 = { let __recv = timer2.clone(); let __recv_ptr: *const time::sleep::Timer = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const time::sleep::Timer }; let __result = unsafe { &*__recv_ptr }.stop(); __result };
    if stop2 {
        println!("{}", format!("{}", "Timer 2 stopped".to_string()));
    }
    time::sleep(Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some((1 as i64 * time::SECOND) as i64))))))));
}