#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

pub fn inside(path: Arc<Mutex<Option<String>>>, dir: Arc<Mutex<Option<String>>>) -> bool {
    strings::has_prefix(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_value = format!("{}{}", { let __v = (*dir.lock().unwrap().as_ref().unwrap()).clone(); __v }, (*Arc::new(Mutex::new(Some(char::from_u32((example_com_stringconst_dep::SEPARATOR) as u32).unwrap().to_string()))).lock().unwrap().as_ref().unwrap())); __arg_value }))))
}

fn main() {
    ::example_com_stringconst_dep::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    println!("{}", format!("{}", inside(Arc::new(Mutex::new(Some("root/file.go".to_string()))), Arc::new(Mutex::new(Some("root".to_string()))))));
}