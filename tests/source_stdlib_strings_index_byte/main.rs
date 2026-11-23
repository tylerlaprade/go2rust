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
    ::strings::__go_init_all();

    println!("{}", format!("{}", strings::index_byte(Arc::new(Mutex::new(Some("a\u{e9}zz".to_string()))), Arc::new(Mutex::new(Some(('z' as u8) as u8))))));
    println!("{}", format!("{}", strings::index_byte(Arc::new(Mutex::new(Some("a\u{e9}zz".to_string()))), Arc::new(Mutex::new(Some(0xa9 as u8))))));
    println!("{}", format!("{}", strings::index_byte(Arc::new(Mutex::new(Some("a\u{e9}zz".to_string()))), Arc::new(Mutex::new(Some(0 as u8))))));
    println!("{}", format!("{}", strings::index_byte(Arc::new(Mutex::new(Some("".to_string()))), Arc::new(Mutex::new(Some(('a' as u8) as u8))))));
    println!("{}", format!("{}", strings::index_byte(Arc::new(Mutex::new(Some("\u{0}a".to_string()))), Arc::new(Mutex::new(Some(0 as u8))))));
}