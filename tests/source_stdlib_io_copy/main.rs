use go2rust_stdlib_stubs::*;
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

fn main() {
    ::bytes::__go_init_all();
    ::internal_abi::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_cpu::__go_init_all();
    ::internal_race::__go_init_all();
    ::internal_sync::__go_init_all();
    ::io::__go_init_all();
    ::iter::__go_init_all();
    ::math_bits::__go_init_all();
    ::sync::__go_init_all();
    ::sync_atomic::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut src: Arc<Mutex<Option<bytes::buffer::Buffer>>> = Arc::new(Mutex::new(Some(Default::default())));
    (*src.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some("copied text".to_string()))));
    let mut dst: Arc<Mutex<Option<bytes::buffer::Buffer>>> = Arc::new(Mutex::new(Some(Default::default())));
    let (mut n, mut err) = io::copy(Arc::new(Mutex::new(Some(Box::new(bytes::buffer::BufferPtr(dst.clone().clone())) as Box<dyn io::r#mod::Writer + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(bytes::buffer::BufferPtr(src.clone().clone())) as Box<dyn io::r#mod::Reader + Send + Sync>))));
    println!("{} {} {}", format!("{}", n), format!("{}", { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result }), format!("{}", (*(*dst.lock().unwrap().as_ref().unwrap()).string().lock().unwrap().as_ref().unwrap())));
}