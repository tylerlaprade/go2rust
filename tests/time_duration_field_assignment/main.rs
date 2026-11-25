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

    let mut delay = Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some((0 as i64 * time::NANOSECOND) as i64)))))));
    { let new_val = time::r#mod::Duration(Arc::new(Mutex::new(Some((30 as i64 * time::SECOND) as i64)))); *delay.lock().unwrap() = Some(new_val); };
    println!("{}", format!("{}", {
        let __tmp_x = ({ let __v = (*delay.lock().unwrap().as_ref().unwrap()).clone(); __v });
        let __tmp_y = time::r#mod::Duration(Arc::new(Mutex::new(Some((30 as i64 * time::SECOND) as i64))));
        __tmp_x == __tmp_y
    }));
}