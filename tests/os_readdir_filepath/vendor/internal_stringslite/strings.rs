use go2rust_stdlib_stubs::*;

use std::sync::{Arc, Mutex};

pub fn index_byte(s: Arc<Mutex<Option<String>>>, c: Arc<Mutex<Option<u8>>>) -> i32 {
    internal_bytealg::index_byte_string(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}