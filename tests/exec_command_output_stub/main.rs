#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

fn main() {
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let (mut out, mut err) = {
        let __recv = exec::command("go".to_string(), ("list".to_string(), "-e".to_string(), "-f".to_string(), "{{context.ReleaseTags}}".to_string(), "--".to_string(), "unsafe".to_string()));
        let __result = (*__recv.lock().unwrap().as_mut().unwrap()).output();
        __result
    };
    let mut out = { let __slice_header = GoSliceHeader::go_slice_header(&out); Arc::new(Mutex::new(__slice_header)) };
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        println!("{}", format!("{}", "error".to_string()));
        return;
    }
    println!("{}", format!("{}", strings::has_prefix(Arc::new(Mutex::new(Some({ let __arg_value = String::from_utf8(std::vec::Vec::from((*out.lock().unwrap().as_ref().unwrap()).clone())).unwrap(); __arg_value }))), Arc::new(Mutex::new(Some("[go1.".to_string()))))));
}