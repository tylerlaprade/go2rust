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

    let mut split = strings::split(Arc::new(Mutex::new(Some("  file.go:12".to_string()))), Arc::new(Mutex::new(Some(":".to_string()))));
    let mut split = { let __slice_header = GoSliceHeader::go_slice_header(&split); Arc::new(Mutex::new(__slice_header)) };
    let mut filename = strings::trim_space(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = split.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }))));
    println!("{}", format!("{}", { let __v = (*filename.lock().unwrap().as_ref().unwrap()).clone(); __v }));
}