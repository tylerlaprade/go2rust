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
    ::internal_stringslite::__go_init_all();
    ::math::__go_init_all();
    ::math_bits::__go_init_all();
    ::strconv::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut str = Arc::new(Mutex::new(Some("42".to_string())));
    let (mut num, mut err) = strconv::atoi(Arc::new(Mutex::new(Some({ let __arg_holder = str.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        println!("{} {}", format!("{}", "Error:".to_string()), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
        return;
    }
    println!("{} {}", format!("{}", "Parsed number:".to_string()), format!("{}", num));
}