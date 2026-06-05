use go2rust_stdlib_stubs::*;

use crate::path::*;
use crate::path_nonwindows::*;

use std::sync::{Arc, Mutex};

pub const SEPARATOR: i32 = ('/' as i32);
pub const LIST_SEPARATOR: i32 = (':' as i32);


pub fn is_path_separator(c: Arc<Mutex<Option<u8>>>) -> bool {
    return { let __tmp_x = SEPARATOR as u8; let __tmp_y = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y };
}

/// volumeNameLen returns length of the leading volume name on Windows.
/// It returns 0 elsewhere.
pub fn volume_name_len_1(path: Arc<Mutex<Option<String>>>) -> i32 {
    0
}