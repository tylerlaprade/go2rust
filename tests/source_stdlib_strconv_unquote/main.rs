#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

fn main() {
    ::internal_bytealg::__go_init_all();
    ::internal_cpu::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::strconv::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let (mut value, mut err) = strconv::unquote(Rc::new(RefCell::new(Some("\"newline\\n\"".to_string()))));
    if { let __nil_result = (*err.borrow()).is_some(); __nil_result } {
        println!("{}", format!("{}", "error".to_string()));
        return;
    }
    print!("{}", format!("{}", { let __v = (*value.borrow().as_ref().unwrap()).clone(); __v }));
    println!("{}", format!("{}", "ok".to_string()));
}