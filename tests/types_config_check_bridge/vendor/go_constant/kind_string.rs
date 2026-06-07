use go2rust_stdlib_stubs::*;

use crate::value::*;

use std::sync::{Arc, Mutex};

pub(crate) const __KIND_NAME: &'static str = "UnknownBoolStringIntFloatComplex";


pub(crate) static _Kind_index: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u8; 7]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *_Kind_index.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *_Kind_index.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0 as u8, 7 as u8, 11 as u8, 17 as u8, 20 as u8, 25 as u8, 32 as u8]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *_Kind_index.lock().unwrap() = Some(std::array::from_fn(|_| 0));
}


pub(crate) fn __go_init_order_0() {
    *_Kind_index.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0 as u8, 7 as u8, 11 as u8, 17 as u8, 20 as u8, 25 as u8, 32 as u8]))).lock().unwrap().as_ref().unwrap()).clone());
}


impl crate::value::Kind {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        if { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::value::Kind(Arc::new(Mutex::new(Some(0 as i32)))); __tmp_x < __tmp_y } || { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::value::Kind(Arc::new(Mutex::new(Some({ let __tmp_x = 7; let __tmp_y = 1; __tmp_x - __tmp_y } as i32)))); __tmp_x >= __tmp_y } {
        return Arc::new(Mutex::new(Some({ let mut __s = String::new(); __s.push_str(&format!("{}", "Kind(".to_string())); __s.push_str(&format!("{}", (*strconv::format_int(Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as i64))), Arc::new(Mutex::new(Some(10)))).lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", ")".to_string())); __s })));
    }
        Arc::new(Mutex::new(Some({ let __s = &(__KIND_NAME); let __low = ({ let __seq = { let __seq_holder = _Kind_index.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*self.0.lock().unwrap().as_ref().unwrap()) as usize].clone() }) as usize; let __high = ({ let __seq = { let __seq_holder = _Kind_index.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[((*self.0.lock().unwrap().as_ref().unwrap()) + 1) as usize].clone() }) as usize; __s[__low..__high].to_string() })))
    }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
