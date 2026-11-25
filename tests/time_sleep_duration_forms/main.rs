#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

pub fn pause(d: Arc<Mutex<Option<time::r#mod::Duration>>>) {
    time::sleep(Arc::new(Mutex::new(Some({ let __arg_holder = d.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
}

fn main() {
    ::internal_bisect::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_godebug::__go_init_all();
    ::internal_godebugs::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::math_bits::__go_init_all();
    ::time::__go_init_all();

    time::sleep(Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some(time::MILLISECOND as i64))))))));
    time::sleep(Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some((2 as i64 * time::MILLISECOND) as i64))))))));
    let mut d = Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some((3 as i64 * time::MILLISECOND) as i64)))))));
    time::sleep(Arc::new(Mutex::new(Some({ let __arg_holder = d.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    pause(Arc::new(Mutex::new(Some({ let __arg_holder = d.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let mut n = Arc::new(Mutex::new(Some(2i64)));
    time::sleep(Arc::new(Mutex::new(Some({ let __tmp_x = time::r#mod::Duration(Arc::new(Mutex::new(Some(({ let __v = (*n.lock().unwrap().as_ref().unwrap()); __v }) as i64)))); let __tmp_y = time::r#mod::Duration(Arc::new(Mutex::new(Some(time::MILLISECOND as i64)))); __tmp_x * __tmp_y }))));
    time::sleep(Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some((500 as i64 * time::MICROSECOND) as i64))))))));
    println!("{}", format!("{}", "slept".to_string()));
}