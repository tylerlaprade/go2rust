#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut ready = GoChannel::<AnonymousStruct1>::new();
    let ready_thread = ready.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        ready_thread.close();;;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    ready.recv().unwrap_or_default();
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

#[derive(Debug, Clone, Default)]
pub struct AnonymousStruct1 {
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
    }
}


impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}
