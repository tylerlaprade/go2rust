use go2rust_stdlib_stubs::*;

use crate::{GoByteSequence};

use crate::r#mod::*;
use crate::compare_native::*;
use crate::count_native::*;
use crate::equal_generic::*;
use crate::equal_native::*;
use crate::index_arm64::*;
use crate::index_native::*;
use crate::indexbyte_native::*;

use std::sync::{Arc, Mutex};

pub fn last_index_byte_string(s: Arc<Mutex<Option<String>>>, c: Arc<Mutex<Option<u8>>>) -> i32 {
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        if { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; let __tmp_y = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        return { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    -(1)
}