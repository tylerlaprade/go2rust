use go2rust_stdlib_stubs::*;

use crate::{r#mod::{MaxLen}};

pub const MAX_BRUTE_FORCE: i32 = 64;


fn __go_init_0() {
    if (*{ let __field = (*internal_cpu::X86.lock().unwrap().as_ref().unwrap()).has_a_v_x2.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        { let new_val = 63; *MaxLen.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = 31; *MaxLen.lock().unwrap() = Some(new_val); };
    }
}

pub(crate) fn __go_init_functions() {
    self::__go_init_0();
}


pub(crate) fn __go_init_all() {
    self::__go_init_0();
}
