use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{fd_unix::{FD}};

use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

impl crate::fd_unix::FD {
    /// OpenDir returns a pointer to a DIR structure suitable for
    /// ReadDir. In case of an error, the name of the failed
    /// syscall is returned along with a syscall.Errno.
    pub fn open_dir(&mut self) -> (usize, Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
                // fdopendir(3) takes control of the file descriptor,
                // so use a dup.
        let (mut fd2, mut call, mut err) = self.dup();
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (0, { let __owned = call.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, err.clone());
    }
        let mut dir: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
        loop {
        { let (__tmp_0, __tmp_1) = fdopendir(Arc::new(Mutex::new(Some(fd2)))); *dir.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        if { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_I_N_T_R as usize)).unwrap_or(false); !__matched } {
        break
    }
    }
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        syscall::close(Arc::new(Mutex::new(Some(fd2))));
        return (0, Arc::new(Mutex::new(Some("fdopendir".to_string()))), err.clone());
    }
        return ({ let __v = (*dir.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some("".to_string()))), Arc::new(Mutex::new(None)));
    }
}

/// Implemented in syscall/syscall_darwin.go.
///
///go:linkname fdopendir syscall.fdopendir
pub fn fdopendir(fd: Arc<Mutex<Option<i32>>>) -> (usize, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    unimplemented!("Go function declaration has no body");
}
