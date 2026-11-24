#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

pub fn wrap(name: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    let mut out: Arc<Mutex<Option<strings::builder::Builder>>> = Arc::new(Mutex::new(Some(Default::default())));
    (*out.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some({
        let mut __s = String::new();
        __s.push_str(&format!("{}", "(".to_string()));
        __s.push_str(&format!("{}", { let __v = (*name.lock().unwrap().as_ref().unwrap()).clone(); __v }));
        __s.push_str(&format!("{}", ")".to_string()));
        __s
    }))));
    return (*out.lock().unwrap().as_ref().unwrap()).string();
}

fn main() {
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    println!("{}", format!("{}", (*wrap(Arc::new(Mutex::new(Some("gopher".to_string())))).lock().unwrap().as_ref().unwrap())));
}