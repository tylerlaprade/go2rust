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

use crate::{fd_unix::{FD}};

use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

pub static TestHookDidSendFile: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<crate::fd_unix::FD>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>, Arc<Mutex<Option<bool>>>) -> () + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *TestHookDidSendFile.lock().unwrap() = Some(Box::new(move |dstFD: Arc<Mutex<Option<FD>>>, src: Arc<Mutex<Option<i32>>>, written: Arc<Mutex<Option<i64>>>, err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>, handled: Arc<Mutex<Option<bool>>>| {
    }) as Box<dyn FnMut(Arc<Mutex<Option<FD>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>, Arc<Mutex<Option<bool>>>) -> () + Send + Sync>);
}


pub(crate) fn __go_zero_globals() {
}


pub(crate) fn __go_init_order_11() {
    *TestHookDidSendFile.lock().unwrap() = Some(Box::new(move |dstFD: Arc<Mutex<Option<FD>>>, src: Arc<Mutex<Option<i32>>>, written: Arc<Mutex<Option<i64>>>, err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>, handled: Arc<Mutex<Option<bool>>>| {
    }) as Box<dyn FnMut(Arc<Mutex<Option<FD>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>, Arc<Mutex<Option<bool>>>) -> () + Send + Sync>);
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
