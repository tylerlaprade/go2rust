use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

pub fn r#use(w: Arc<Mutex<Option<Box<dyn io::r#mod::Writer + Send + Sync>>>>) {
    let _ = { let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn make_buffer() -> Arc<Mutex<Option<bytes::buffer::Buffer>>> {
    let (mut stdout, mut stderr) = (Arc::new(Mutex::new(Some(bytes::buffer::Buffer { ..Default::default() }))), Arc::new(Mutex::new(Some(bytes::buffer::Buffer { ..Default::default() }))));
    r#use(Arc::new(Mutex::new(Some(Box::new(bytes::buffer::BufferPtr(stdout.clone())) as Box<dyn io::r#mod::Writer + Send + Sync>))));
    r#use(Arc::new(Mutex::new(Some(Box::new(bytes::buffer::BufferPtr(stderr.clone())) as Box<dyn io::r#mod::Writer + Send + Sync>))));
    return stdout.clone();
}

fn main() {
    ::bytes::__go_init_all();
    ::internal_abi::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_cpu::__go_init_all();
    ::internal_race::__go_init_all();
    ::internal_sync::__go_init_all();
    ::io::__go_init_all();
    ::sync::__go_init_all();
    ::sync_atomic::__go_init_all();
    ::unicode_utf8::__go_init_all();

    if { let __nil_result = (*make_buffer().lock().unwrap()).is_some(); __nil_result } {
        println!("{}", format!("{}", "buffer".to_string()));
    }
}