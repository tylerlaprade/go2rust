use go2rust_stdlib_stubs::*;

use crate::r#mod::*;
use crate::compare_native::*;
use crate::count_native::*;
use crate::equal_generic::*;
use crate::equal_native::*;
use crate::index_native::*;
use crate::indexbyte_native::*;
use crate::lastindexbyte_generic::*;

pub const MAX_BRUTE_FORCE: i32 = 16;


fn __go_init_0() {
        // Optimize cases where the length of the substring is less than 32 bytes
    { let new_val = 32; *MaxLen.lock().unwrap() = Some(new_val); };
}

pub(crate) fn __go_init_functions() {
    self::__go_init_0();
}


pub(crate) fn __go_init_all() {
    self::__go_init_0();
}
