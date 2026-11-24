#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

fn main() {
    ::internal_stringslite::__go_init_all();
    ::math::__go_init_all();
    ::math_bits::__go_init_all();
    ::strconv::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let (mut n, _) = strconv::atoi(Arc::new(Mutex::new(Some("42".to_string()))));
    println!("{} {}", format!("{}", (*Arc::new(Mutex::new(Some({ let __s = "go".to_string(); __s.to_uppercase() }))).lock().unwrap().as_ref().unwrap())), format!("{}", n));
}