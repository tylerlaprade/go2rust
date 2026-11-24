#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

fn main() {
    ::internal_bytealg::__go_init_all();
    ::internal_cpu::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::math::__go_init_all();
    ::math_bits::__go_init_all();
    ::strconv::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let (mut value, mut err) = strconv::unquote(Arc::new(Mutex::new(Some("\"newline\\n\"".to_string()))));
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        println!("{}", format!("{}", "error".to_string()));
        return;
    }
    print!("{}", format!("{}", { let __v = (*value.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    println!("{}", format!("{}", "ok".to_string()));
}