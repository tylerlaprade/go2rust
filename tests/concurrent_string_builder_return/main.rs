#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

pub fn build(done: GoChannel<bool>) -> Arc<Mutex<Option<String>>> {
    let mut output: Arc<Mutex<Option<strings::builder::Builder>>> = Arc::new(Mutex::new(Some(Default::default())));
    (*output.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some("ready".to_string()))));
    done.send(true);
    return (*output.lock().unwrap().as_ref().unwrap()).string();
}

fn main() {
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut done = GoChannel::<bool>::new_buffered(1 as usize);
    println!("{}", format!("{}", (*build(done.clone()).lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", done.recv().unwrap_or_default()));
}