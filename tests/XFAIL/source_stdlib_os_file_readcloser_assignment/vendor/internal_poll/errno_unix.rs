use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoPtr,
    GoSliceElemMutRef,
    GoSliceElemPtr,
    GoSliceElemRef,
    format_slice,
    format_slice_values,
    format_slice_wrapped,
    go_recover,
    go_resume_unrecovered_panic,
    go_store_panic_payload,
};

use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

pub(crate) static errEAGAIN: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static errEINVAL: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static errENOENT: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *errEAGAIN.lock().unwrap() = None;
    *errEINVAL.lock().unwrap() = None;
    *errENOENT.lock().unwrap() = None;
    *errEAGAIN.lock().unwrap() = Some(Box::new(syscall::syscall_unix::Errno(Arc::new(Mutex::new(Some(syscall::E_A_G_A_I_N as usize))))) as Box<dyn StdError + Send + Sync>);
    *errEINVAL.lock().unwrap() = Some(Box::new(syscall::syscall_unix::Errno(Arc::new(Mutex::new(Some(syscall::E_I_N_V_A_L as usize))))) as Box<dyn StdError + Send + Sync>);
    *errENOENT.lock().unwrap() = Some(Box::new(syscall::syscall_unix::Errno(Arc::new(Mutex::new(Some(syscall::E_N_O_E_N_T as usize))))) as Box<dyn StdError + Send + Sync>);
}


pub(crate) fn __go_zero_globals() {
    *errEAGAIN.lock().unwrap() = None;
    *errEINVAL.lock().unwrap() = None;
    *errENOENT.lock().unwrap() = None;
}


pub(crate) fn __go_init_order_0() {
    *errEAGAIN.lock().unwrap() = Some(Box::new(syscall::syscall_unix::Errno(Arc::new(Mutex::new(Some(syscall::E_A_G_A_I_N as usize))))) as Box<dyn StdError + Send + Sync>);
}


pub(crate) fn __go_init_order_1() {
    *errEINVAL.lock().unwrap() = Some(Box::new(syscall::syscall_unix::Errno(Arc::new(Mutex::new(Some(syscall::E_I_N_V_A_L as usize))))) as Box<dyn StdError + Send + Sync>);
}


pub(crate) fn __go_init_order_2() {
    *errENOENT.lock().unwrap() = Some(Box::new(syscall::syscall_unix::Errno(Arc::new(Mutex::new(Some(syscall::E_N_O_E_N_T as usize))))) as Box<dyn StdError + Send + Sync>);
}


/// errnoErr returns common boxed Errno values, to prevent
/// allocations at runtime.
pub fn errno_err(e: Arc<Mutex<Option<syscall::syscall_unix::Errno>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    { let _switch_val = (*e.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (syscall::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize))))) {
            return Arc::new(Mutex::new(None));
        } else if _switch_val == (syscall::syscall_unix::Errno(Arc::new(Mutex::new(Some(syscall::E_A_G_A_I_N as usize))))) {
            return errEAGAIN.clone();
        } else if _switch_val == (syscall::syscall_unix::Errno(Arc::new(Mutex::new(Some(syscall::E_I_N_V_A_L as usize))))) {
            return errEINVAL.clone();
        } else if _switch_val == (syscall::syscall_unix::Errno(Arc::new(Mutex::new(Some(syscall::E_N_O_E_N_T as usize))))) {
            return errENOENT.clone();
        }
    }
    return Arc::new(Mutex::new(Some(Box::new((*e.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn StdError + Send + Sync>)));
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
