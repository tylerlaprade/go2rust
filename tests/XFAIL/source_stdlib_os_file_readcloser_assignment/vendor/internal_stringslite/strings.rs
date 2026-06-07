use go2rust_stdlib_stubs::*;

use std::sync::{Arc, Mutex};

pub fn has_prefix(s: Arc<Mutex<Option<String>>>, prefix: Arc<Mutex<Option<String>>>) -> bool {
    return { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ((*prefix.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x >= __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __high = ((*prefix.lock().unwrap().as_ref().unwrap()).len()) as usize; __s[..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*prefix.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y };
}

pub fn index_byte(s: Arc<Mutex<Option<String>>>, c: Arc<Mutex<Option<u8>>>) -> i32 {
    internal_bytealg::index_byte_string(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

pub fn cut_prefix(s: Arc<Mutex<Option<String>>>, prefix: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, bool) {
    let mut after: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut found: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    if !has_prefix(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = prefix.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return ({ let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, false);
    }
    (Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ((*prefix.lock().unwrap().as_ref().unwrap()).len()) as usize; __s[__low..].to_string() }))), true)
}

pub fn clone(s: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some("".to_string())));
    }
    let mut b = Arc::new(Mutex::new(Some(vec![0; ((*s.lock().unwrap().as_ref().unwrap()).len()) as usize])));
    { let _src = (*s.lock().unwrap().as_ref().unwrap()).clone().as_bytes().to_vec(); let _n = std::cmp::min((*b.lock().unwrap().as_ref().unwrap()).len(), _src.len()); for _i in 0.._n { (*b.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
    return Arc::new(Mutex::new(Some({ let __bytes_holder = b.clone(); let __bytes_guard = __bytes_holder.lock().unwrap(); let __bytes = __bytes_guard.as_ref().unwrap(); let __start = (0) as usize; let __len = __bytes.len(); let __end = __start + __len; String::from_utf8(__bytes[__start..__end].to_vec()).unwrap() })));
}