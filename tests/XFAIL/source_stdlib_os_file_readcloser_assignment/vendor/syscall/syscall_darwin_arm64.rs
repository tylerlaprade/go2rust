use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{syscall_unix::{Errno}, ztypes_darwin_arm64::{Iovec, Msghdr}};

use std::sync::{Arc, Mutex};

impl crate::ztypes_darwin_arm64::Iovec {
    pub fn set_len(&mut self, length: Arc<Mutex<Option<i32>>>) {
        { let new_val = Arc::new(Mutex::new(Some((*length.lock().unwrap().as_ref().unwrap()) as u64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.len.lock().unwrap() = __moved_val; };
    }
}

impl crate::ztypes_darwin_arm64::Msghdr {
    pub fn set_controllen(&mut self, length: Arc<Mutex<Option<i32>>>) {
        { let new_val = Arc::new(Mutex::new(Some((*length.lock().unwrap().as_ref().unwrap()) as u32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.controllen.lock().unwrap() = __moved_val; };
    }
}

/// Implemented in the runtime package (runtime/sys_darwin_64.go)
pub fn syscall_x(r#fn: Arc<Mutex<Option<usize>>>, a1: Arc<Mutex<Option<usize>>>, a2: Arc<Mutex<Option<usize>>>, a3: Arc<Mutex<Option<usize>>>) -> (usize, usize, Arc<Mutex<Option<crate::syscall_unix::Errno>>>) {
    unimplemented!("Go function declaration has no body");
}
