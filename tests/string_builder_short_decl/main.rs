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
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut builder = strings::builder::Builder { ..Default::default() };
    builder.write_string(Arc::new(Mutex::new(Some("a".to_string()))));
    builder.write_byte(Arc::new(Mutex::new(Some(('b' as u8) as u8))));
    builder.write_rune(Arc::new(Mutex::new(Some(('c' as i32)))));
    let mut result = builder.string();
    println!("{} {}", format!("{}", { let __v = (*result.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", builder.len()));
}