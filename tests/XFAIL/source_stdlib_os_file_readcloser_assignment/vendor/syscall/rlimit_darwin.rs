use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::badlinkname_unix::*;
use crate::bpf_bsd::*;
use crate::dirent::*;
use crate::env_unix::*;
use crate::exec_libc2::*;
use crate::exec_unix::*;
use crate::flock_bsd::*;
use crate::forkpipe::*;
use crate::linkname_bsd::*;
use crate::linkname_darwin::*;
use crate::linkname_libc::*;
use crate::linkname_unix::*;
use crate::net::*;
use crate::rlimit::*;
use crate::route_bsd::*;
use crate::route_darwin::*;
use crate::sockcmsg_unix::*;
use crate::sockcmsg_unix_other::*;
use crate::r#mod::*;
use crate::syscall_bsd::*;
use crate::syscall_darwin::*;
use crate::syscall_darwin_arm64::*;
use crate::syscall_unix::*;
use crate::time_nofake::*;
use crate::timestruct::*;
use crate::zerrors_darwin_arm64::*;
use crate::zsyscall_darwin_arm64::*;
use crate::zsysnum_darwin_arm64::*;
use crate::ztypes_darwin_arm64::*;

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