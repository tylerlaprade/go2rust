use go2rust_stdlib_stubs::*;

use std::sync::{Arc, Mutex};

pub fn has_prefix(s: Arc<Mutex<Option<String>>>, prefix: Arc<Mutex<Option<String>>>) -> bool {
    return { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ((*prefix.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x >= __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s[..((*prefix.lock().unwrap().as_ref().unwrap()).len()) as usize].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*prefix.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y };
}