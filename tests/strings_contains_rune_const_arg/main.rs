#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

pub(crate) const SLASH: i32 = ('/' as i32);


fn main() {
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    println!("{}", format!("{}", strings::contains(Arc::new(Mutex::new(Some("a/b".to_string()))), Arc::new(Mutex::new(Some(char::from_u32((SLASH) as u32).unwrap().to_string()))))));
    println!("{}", format!("{}", strings::contains(Arc::new(Mutex::new(Some("axb".to_string()))), Arc::new(Mutex::new(Some(char::from_u32((SLASH) as u32).unwrap().to_string()))))));
}