use go2rust_stdlib_stubs::*;

use std::sync::{Arc, Mutex};

pub fn has_nonblock_flag(flag: Arc<Mutex<Option<i32>>>) -> bool {
    return { let __tmp_x = { let __tmp_x = { let __v = (*flag.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = syscall::O__N_O_N_B_L_O_C_K; __tmp_x & __tmp_y }; let __tmp_y = 0; __tmp_x != __tmp_y };
}