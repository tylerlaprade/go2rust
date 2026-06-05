use go2rust_stdlib_stubs::*;

use crate::{GoByteSequence};

use crate::r#mod::*;
use crate::compare_native::*;
use crate::count_native::*;
use crate::equal_generic::*;
use crate::equal_native::*;
use crate::index_native::*;
use crate::indexbyte_native::*;
use crate::lastindexbyte_generic::*;

use std::sync::{Arc, Mutex};

pub const MAX_BRUTE_FORCE: i32 = 16;


fn __go_init_0() {
        // Optimize cases where the length of the substring is less than 32 bytes
    { let new_val = 32; *MaxLen.lock().unwrap() = Some(new_val); };
}

/// Cutover reports the number of failures of IndexByte we should tolerate
/// before switching over to Index.
/// n is the number of bytes processed so far.
/// See the bytes.Index implementation for details.
pub fn cutover(n: Arc<Mutex<Option<i32>>>) -> i32 {
        // 1 error per 16 characters, plus a few slop to start.
    return { let __tmp_x = 4; let __tmp_y = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x >> __tmp_y }; __tmp_x + __tmp_y };
}

pub(crate) fn __go_init_functions() {
    self::__go_init_0();
}


pub(crate) fn __go_init_all() {
    self::__go_init_0();
}
