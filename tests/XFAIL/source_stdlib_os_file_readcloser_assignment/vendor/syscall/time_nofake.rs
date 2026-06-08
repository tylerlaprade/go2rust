use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use std::any::Any;
use std::sync::{Arc, Mutex};

pub(crate) const FAKETIME: bool = false;


pub fn faketime_write(fd: Arc<Mutex<Option<i32>>>, p: Arc<Mutex<Option<Vec<u8>>>>) -> i32 {
        // This should never be called since faketime is false.
    std::panic::panic_any(Box::new("not implemented".to_string()) as Box<dyn Any + Send + Sync>);
}