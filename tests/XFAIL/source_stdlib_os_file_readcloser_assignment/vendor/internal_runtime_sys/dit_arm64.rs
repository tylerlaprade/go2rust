use go2rust_stdlib_stubs::*;

use crate::consts::*;
use crate::consts_norace::*;
use crate::intrinsics::*;
use crate::nih::*;

pub static DITSupported: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *DITSupported.lock().unwrap() = Some(false);
    *DITSupported.lock().unwrap() = Some((*{ let __field = (*internal_cpu::ARM64.lock().unwrap().as_ref().unwrap()).has_d_i_t.clone(); __field }.lock().unwrap().as_ref().unwrap()));
}


pub(crate) fn __go_zero_globals() {
    *DITSupported.lock().unwrap() = Some(false);
}


pub(crate) fn __go_init_order_0() {
    *DITSupported.lock().unwrap() = Some((*{ let __field = (*internal_cpu::ARM64.lock().unwrap().as_ref().unwrap()).has_d_i_t.clone(); __field }.lock().unwrap().as_ref().unwrap()));
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
