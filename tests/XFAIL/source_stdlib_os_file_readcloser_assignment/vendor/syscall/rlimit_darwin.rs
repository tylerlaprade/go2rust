use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{syscall_bsd::{sysctl_uint32}, ztypes_darwin_arm64::{Rlimit}};

use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

/// adjustFileLimit adds per-OS limitations on the Rlimit used for RLIMIT_NOFILE. See rlimit.go.
pub fn adjust_file_limit(lim: Arc<Mutex<Option<Rlimit>>>) {
        // On older macOS, setrlimit(RLIMIT_NOFILE, lim) with lim.Cur = infinity fails.
        // Set to the value of kern.maxfilesperproc instead.
    let (mut n, mut err) = sysctl_uint32(Arc::new(Mutex::new(Some("kern.maxfilesperproc".to_string()))));
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return;
    }
    if { let __tmp_x = (*{ let __field = (*lim.lock().unwrap().as_ref().unwrap()).cur.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some(n as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some(n as u64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*lim.lock().unwrap().as_ref().unwrap()).cur.lock().unwrap() = __moved_val; };
    }
}