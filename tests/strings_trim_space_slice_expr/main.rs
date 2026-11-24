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

    let mut errstr = Arc::new(Mutex::new(Some("no such directory   /tmp/missing\n".to_string())));
    let mut marker = Arc::new(Mutex::new(Some("no such directory".to_string())));
    let mut abspath = strings::trim_space(Arc::new(Mutex::new(Some({ let __s = &((*errstr.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = (strings::index(Arc::new(Mutex::new(Some({ let __arg_holder = errstr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = marker.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) as i64); let __tmp_y = (({ let __len = (*marker.lock().unwrap().as_ref().unwrap()).len(); __len }) as i64); __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))));
    println!("{}", format!("{}", { let __v = (*abspath.lock().unwrap().as_ref().unwrap()).clone(); __v }));
}