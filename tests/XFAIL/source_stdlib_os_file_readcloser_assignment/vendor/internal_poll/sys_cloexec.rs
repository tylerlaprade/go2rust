use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{hook_unix::{AcceptFunc, CloseFunc}};

use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

/// Wrapper around the accept system call that marks the returned file
/// descriptor as nonblocking and close-on-exec.
pub fn accept(s: Arc<Mutex<Option<i32>>>) -> (i32, Arc<Mutex<Option<Box<dyn syscall::syscall_unix::Sockaddr + Send + Sync>>>>, Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // See ../syscall/exec_unix.go for description of ForkLock.
        // It is probably okay to hold the lock across syscall.Accept
        // because we have put fd.sysfd into non-blocking mode.
        // However, a call to the File method will put it back into
        // blocking mode. We can't take that risk, so no use of ForkLock here.
    let (mut ns, mut sa, mut err) = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> (i32, Arc<Mutex<Option<Box<dyn syscall::syscall_unix::Sockaddr + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync> = { let mut __f_guard = AcceptFunc.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> (i32, Arc<Mutex<Option<Box<dyn syscall::syscall_unix::Sockaddr + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(s.clone()) };
    if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
        syscall::close_on_exec(Arc::new(Mutex::new(Some(ns))));
    }
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (-(1), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some("accept".to_string()))), err.clone());
    }
    {
        { let __rhs_holder = syscall::set_nonblock(Arc::new(Mutex::new(Some(ns))), Arc::new(Mutex::new(Some(true)))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync> = { let mut __f_guard = CloseFunc.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(ns)))) };;
            return (-(1), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some("setnonblock".to_string()))), err.clone());;
        }
    }
    return (ns, sa.clone(), Arc::new(Mutex::new(Some("".to_string()))), Arc::new(Mutex::new(None)));
}