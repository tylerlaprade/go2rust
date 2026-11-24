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

    let mut cmd = exec::command("go".to_string(), ("list".to_string(), "-export".to_string(), "-f".to_string(), "{{.Export}}".to_string(), "pkg".to_string()));
    println!("{}", format!("{}", (*strings::join({ let __field = (*cmd.lock().unwrap().as_ref().unwrap()).args.clone(); __field }, Arc::new(Mutex::new(Some(" ".to_string())))).lock().unwrap().as_ref().unwrap())));
}