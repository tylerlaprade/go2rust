#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

pub fn clean(name: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    strings::to_upper(strings::trim_prefix(strings::to_lower(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))), Arc::new(Mutex::new(Some("r#".to_string())))))
}

pub fn clean_suffix(name: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    strings::to_lower(strings::trim_suffix(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(".RS".to_string())))))
}

fn main() {
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    println!("{}", format!("{}", (*clean(Arc::new(Mutex::new(Some("R#Go2Rust".to_string())))).lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", (*clean_suffix(Arc::new(Mutex::new(Some("Go2Rust.RS".to_string())))).lock().unwrap().as_ref().unwrap())));
}