#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

pub fn classify(value: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    { let _switch_val = { let __v = strings::to_lower(Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned };
    if _switch_val == ("go".to_string()) || _switch_val == ("rust".to_string()) {
            return Arc::new(Mutex::new(Some("systems".to_string())));
        } else if _switch_val == ("python".to_string()) {
            return Arc::new(Mutex::new(Some("scripting".to_string())));
        } else {
            return Arc::new(Mutex::new(Some("other".to_string())));
        }
    }
}

fn main() {
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    println!("{}", format!("{}", (*classify(Arc::new(Mutex::new(Some("Go".to_string())))).lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", (*classify(Arc::new(Mutex::new(Some("python".to_string())))).lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", (*classify(Arc::new(Mutex::new(Some("zig".to_string())))).lock().unwrap().as_ref().unwrap())));
}