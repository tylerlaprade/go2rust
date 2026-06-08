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

use crate::{exec::{Signal}};

use std::sync::{Arc, Mutex};

pub static Interrupt: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn Signal + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static Kill: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn Signal + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *Interrupt.lock().unwrap() = None;
    *Kill.lock().unwrap() = None;
    *Interrupt.lock().unwrap() = Some(Box::new(syscall::syscall_unix::Signal(Arc::new(Mutex::new(Some(syscall::S_I_G_I_N_T as i32))))) as Box<dyn Signal + Send + Sync>);
    *Kill.lock().unwrap() = Some(Box::new(syscall::syscall_unix::Signal(Arc::new(Mutex::new(Some(syscall::S_I_G_K_I_L_L as i32))))) as Box<dyn Signal + Send + Sync>);
}


pub(crate) fn __go_zero_globals() {
    *Interrupt.lock().unwrap() = None;
    *Kill.lock().unwrap() = None;
}


pub(crate) fn __go_init_order_8() {
    *Interrupt.lock().unwrap() = Some(Box::new(syscall::syscall_unix::Signal(Arc::new(Mutex::new(Some(syscall::S_I_G_I_N_T as i32))))) as Box<dyn Signal + Send + Sync>);
}


pub(crate) fn __go_init_order_9() {
    *Kill.lock().unwrap() = Some(Box::new(syscall::syscall_unix::Signal(Arc::new(Mutex::new(Some(syscall::S_I_G_K_I_L_L as i32))))) as Box<dyn Signal + Send + Sync>);
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
