use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

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
use crate::rlimit_darwin::*;
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

pub(crate) static origRlimitNofile: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<sync_atomic::r#type::Pointer<crate::ztypes_darwin_arm64::Rlimit>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *origRlimitNofile.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_zero_globals() {
    *origRlimitNofile.lock().unwrap() = Some(Default::default());
}


/// Some systems set an artificially low soft limit on open file count, for compatibility
/// with code that uses select and its hard-coded maximum file descriptor
/// (limited by the size of fd_set).
///
/// Go does not use select, so it should not be subject to these limits.
/// On some systems the limit is 256, which is very easy to run into,
/// even in simple programs like gofmt when they parallelize walking
/// a file tree.
///
/// After a long discussion on go.dev/issue/46279, we decided the
/// best approach was for Go to raise the limit unconditionally for itself,
/// and then leave old software to set the limit back as needed.
/// Code that really wants Go to leave the limit alone can set the hard limit,
/// which Go of course has no choice but to respect.
fn __go_init_0() {
    let mut lim: Arc<Mutex<Option<Rlimit>>> = Arc::new(Mutex::new(Some(Default::default())));
    {
        let mut err = getrlimit(Arc::new(Mutex::new(Some(8))), lim.clone());;
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } && { let __tmp_x = (*{ let __field = (*lim.lock().unwrap().as_ref().unwrap()).max.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u64; __tmp_x > __tmp_y } && { let __tmp_x = (*{ let __field = (*lim.lock().unwrap().as_ref().unwrap()).cur.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = (*{ let __field = (*lim.lock().unwrap().as_ref().unwrap()).max.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as u64; __tmp_x - __tmp_y }; __tmp_x < __tmp_y } {
            (*origRlimitNofile.lock().unwrap().as_mut().unwrap()).store(sync_atomic::GoPtr::local(lim.clone()));;
            let mut nlim = { let __owned = lim.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };;
            { let new_val = { let __tmp_x = (*{ let __field = (*nlim.lock().unwrap().as_ref().unwrap()).max.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as u64; __tmp_x - __tmp_y }; *(*nlim.lock().unwrap().as_ref().unwrap()).cur.lock().unwrap() = Some(new_val); };;
            adjust_file_limit(nlim.clone());;
            setrlimit_1(Arc::new(Mutex::new(Some(8))), GoPtr::local(nlim.clone()));;
        }
    }
}

pub(crate) fn __go_init_functions() {
    self::__go_init_0();
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
    self::__go_init_0();
}
