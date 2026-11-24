#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

fn main() {
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::math::__go_init_all();
    ::math_bits::__go_init_all();
    ::strconv::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut quoted = strconv::quote(Arc::new(Mutex::new(Some("".to_string()))));
    println!("{} {}", format!("{}", ({ let __len = (*quoted.lock().unwrap().as_ref().unwrap()).len(); __len })), format!("{}", { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*quoted.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; let __high = ({ let __tmp_x = (({ let __len = (*quoted.lock().unwrap().as_ref().unwrap()).len(); __len }) as i64); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; __s[__low..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y }));
    println!("{}", format!("{}", (*strconv::quote(Arc::new(Mutex::new(Some("go list".to_string())))).lock().unwrap().as_ref().unwrap())));
}