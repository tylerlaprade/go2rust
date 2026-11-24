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

    println!("{} {} {} {} {} {} {} {} {} {} {} {} {} {} {}", format!("{}", json::valid(Arc::new(Mutex::new(Some(({
        let mut __s = String::new();
        __s.push_str(&format!("{}", (*strings::repeat(Arc::new(Mutex::new(Some("[".to_string()))), Arc::new(Mutex::new(Some(10000)))).lock().unwrap().as_ref().unwrap())));
        __s.push_str(&format!("{}", "0".to_string()));
        __s.push_str(&format!("{}", (*strings::repeat(Arc::new(Mutex::new(Some("]".to_string()))), Arc::new(Mutex::new(Some(10000)))).lock().unwrap().as_ref().unwrap())));
        __s
    }).as_bytes().to_go_slice()))))), format!("{}", json::valid(Arc::new(Mutex::new(Some(({
        let mut __s = String::new();
        __s.push_str(&format!("{}", (*strings::repeat(Arc::new(Mutex::new(Some("[".to_string()))), Arc::new(Mutex::new(Some(10001)))).lock().unwrap().as_ref().unwrap())));
        __s.push_str(&format!("{}", "0".to_string()));
        __s.push_str(&format!("{}", (*strings::repeat(Arc::new(Mutex::new(Some("]".to_string()))), Arc::new(Mutex::new(Some(10001)))).lock().unwrap().as_ref().unwrap())));
        __s
    }).as_bytes().to_go_slice()))))), format!("{}", json::valid(Arc::new(Mutex::new(Some(({
        let mut __s = String::new();
        __s.push_str(&format!("{}", "\"".to_string()));
        __s.push_str(&format!("{}", (*strings::repeat(Arc::new(Mutex::new(Some("[".to_string()))), Arc::new(Mutex::new(Some(10001)))).lock().unwrap().as_ref().unwrap())));
        __s.push_str(&format!("{}", "\"".to_string()));
        __s
    }).as_bytes().to_go_slice()))))), format!("{}", json::valid(Arc::new(Mutex::new(Some(("{\"x\":[true,null,\"a\\\\b\"]}".to_string()).as_bytes().to_go_slice()))))), format!("{}", json::valid(Arc::new(Mutex::new(Some((" \n [1,2,3] \t".to_string()).as_bytes().to_go_slice()))))), format!("{}", json::valid(Arc::new(Mutex::new(Some(("\"escaped\\u263a\"".to_string()).as_bytes().to_go_slice()))))), format!("{}", json::valid(Arc::new(Mutex::new(Some(("-1.2e+3".to_string()).as_bytes().to_go_slice()))))), format!("{}", json::valid(Arc::new(Mutex::new(Some(("1e999".to_string()).as_bytes().to_go_slice()))))), format!("{}", json::valid(Arc::new(Mutex::new(Some(("\"\\ud800\"".to_string()).as_bytes().to_go_slice()))))), format!("{}", json::valid(Arc::new(Mutex::new(Some(vec![('"' as u8), 0xff as u8, ('"' as u8)]))))), format!("{}", json::valid(Arc::new(Mutex::new(Some(("{\"x\":".to_string()).as_bytes().to_go_slice()))))), format!("{}", json::valid(Arc::new(Mutex::new(Some(("true false".to_string()).as_bytes().to_go_slice()))))), format!("{}", json::valid(Arc::new(Mutex::new(Some(("01".to_string()).as_bytes().to_go_slice()))))), format!("{}", json::valid(Arc::new(Mutex::new(Some(Vec::<u8>::new()))))), format!("{}", json::valid(Arc::new(Mutex::new(Some(vec![0xff as u8]))))));
}