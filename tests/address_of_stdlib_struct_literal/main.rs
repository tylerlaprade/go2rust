use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

pub fn r#use(w: Arc<Mutex<Option<io_Writer>>>) {
    let _ = { let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn make_buffer() -> Arc<Mutex<Option<bytes::buffer::Buffer>>> {
    let (mut stdout, mut stderr) = (Arc::new(Mutex::new(Some(bytes::buffer::Buffer { ..Default::default() }))), Arc::new(Mutex::new(Some(bytes::buffer::Buffer { ..Default::default() }))));
    r#use(Arc::new(Mutex::new(Some({ let __writer = stdout.clone(); io_Writer::__go_from_with_write(__writer.clone(), move |__data| { let mut __guard = __writer.lock().unwrap(); if let Some(__target) = __guard.as_mut() { let _ = __target.write(Arc::new(Mutex::new(Some(__data.to_vec())))); } }) }))));
    r#use(Arc::new(Mutex::new(Some({ let __writer = stderr.clone(); io_Writer::__go_from_with_write(__writer.clone(), move |__data| { let mut __guard = __writer.lock().unwrap(); if let Some(__target) = __guard.as_mut() { let _ = __target.write(Arc::new(Mutex::new(Some(__data.to_vec())))); } }) }))));
    return stdout.clone();
}

fn main() {
    bytes::__go_init_all();
    internal_bytealg::__go_init_all();
    internal_cpu::__go_init_all();
    unicode_utf8::__go_init_all();

    if { let __nil_result = (*make_buffer().lock().unwrap()).is_some(); __nil_result } {
        println!("{}", format!("{}", "buffer".to_string()));
    }
}