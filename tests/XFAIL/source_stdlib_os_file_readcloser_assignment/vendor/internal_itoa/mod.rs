use go2rust_stdlib_stubs::*;

use std::sync::{Arc, Mutex};

pub(crate) const HEX: &'static str = "0123456789abcdef";


/// Itoa converts val to a decimal string.
pub fn itoa(val: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<String>>> {
    if { let __tmp_x = { let __v = (*val.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        return Arc::new(Mutex::new(Some(format!("{}{}", "-".to_string(), (*uitoa(Arc::new(Mutex::new(Some(-((*val.lock().unwrap().as_ref().unwrap())) as u64)))).lock().unwrap().as_ref().unwrap())))));
    }
    uitoa(Arc::new(Mutex::new(Some((*val.lock().unwrap().as_ref().unwrap()) as u64))))
}

/// Uitoa converts val to a decimal string.
pub fn uitoa(mut val: Arc<Mutex<Option<u64>>>) -> Arc<Mutex<Option<String>>> {
    if { let __tmp_x = { let __v = (*val.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some("0".to_string())));
    }
    let mut buf: Arc<Mutex<Option<[u8; 20]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = 20; let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*val.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10 as u64; __tmp_x >= __tmp_y } {
        let mut q = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*val.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10 as u64; __tmp_x / __tmp_y })));
        (*buf.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __tmp_x = ('0' as u64); let __tmp_y = { let __v = (*val.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = { let __tmp_x = { let __v = (*q.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10 as u64; __tmp_x * __tmp_y }; __tmp_x - __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()).clone();
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        { let new_val = q.lock().unwrap().as_ref().unwrap().clone(); *val.lock().unwrap() = Some(new_val); };
    }

        // val < 10
    (*buf.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = (*Arc::new(Mutex::new(Some(({ let __tmp_x = ('0' as u64); let __tmp_y = { let __v = (*val.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()).clone();
    return Arc::new(Mutex::new(Some(String::from_utf8((*Arc::new(Mutex::new(Some({
        let __seq_holder = buf.clone();
        let __seq_guard = __seq_holder.lock().unwrap();
        let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0);
        let mut __seq = (*__seq_guard.as_ref().unwrap()).clone();
        drop(__seq_guard);
        let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __high = __seq.len();
        let __max = __source_cap;
        let _slice = &__seq[__low..__high];
        let mut _v = Vec::with_capacity((__max - __low) as usize);
        _v.extend_from_slice(_slice);
        _v
    }))).lock().unwrap().as_ref().unwrap()).clone()).unwrap())));
}