use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

fn main() {
    bytes::__go_init_all();
    internal_bytealg::__go_init_all();
    internal_cpu::__go_init_all();
    unicode_utf8::__go_init_all();

    let mut buf: Arc<Mutex<Option<bytes::buffer::Buffer>>> = Arc::new(Mutex::new(Some(Default::default())));
    println!("{}", format!("{}", { let __tmp_x = (*(*buf.lock().unwrap().as_ref().unwrap()).string().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y }));
    (*buf.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some("go".to_string()))));
    (*buf.lock().unwrap().as_mut().unwrap()).write_byte(Arc::new(Mutex::new(Some(('2' as i32) as u8))));
    (*buf.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some("rust".to_string()))));
    println!("{} {}", format!("{}", (*buf.lock().unwrap().as_ref().unwrap()).len()), format!("{}", (*(*buf.lock().unwrap().as_ref().unwrap()).string().lock().unwrap().as_ref().unwrap())));
}