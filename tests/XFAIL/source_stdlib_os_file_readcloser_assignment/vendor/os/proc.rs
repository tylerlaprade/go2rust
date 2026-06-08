use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoPtr,
    GoSliceElemMutRef,
    GoSliceElemPtr,
    GoSliceElemRef,
};

use std::sync::{Arc, Mutex};

pub static Args: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<String>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *Args.lock().unwrap() = Some(vec![]);
}


pub(crate) fn __go_zero_globals() {
    *Args.lock().unwrap() = Some(vec![]);
}


fn __go_init_0() {
    if { let __tmp_x = "darwin".to_string(); let __tmp_y = "windows".to_string(); __tmp_x == __tmp_y } {
                // Initialized in exec_windows.go.
        return;
    }
        // Initialized in exec_windows.go.
    { let new_val = { let __collection_holder = runtime_args().clone(); let __collection_guard = __collection_holder.lock().unwrap(); (*__collection_guard).clone() }; *Args.lock().unwrap() = new_val; };
}

pub fn runtime_args() -> Arc<Mutex<Option<Vec<String>>>> {
    unimplemented!("Go function declaration has no body");
}


pub(crate) fn __go_init_functions() {
    self::__go_init_0();
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
    self::__go_init_0();
}
