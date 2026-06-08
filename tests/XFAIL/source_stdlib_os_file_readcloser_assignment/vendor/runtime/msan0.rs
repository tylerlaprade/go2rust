use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoPtr,
    GoSliceElemMutRef,
    GoSliceElemPtr,
    GoSliceElemRef,
    format_any,
    format_map,
    format_nested_pointer_slice,
    format_nested_pointer_slice_wrapped,
    format_nested_slice,
    format_nested_slice_wrapped,
    format_slice,
    format_slice_values,
    format_slice_wrapped,
    format_slice_wrapped_values,
    go_any_clone,
    go_const_str_eq,
    go_recover,
    go_resume_unrecovered_panic,
    go_store_panic_payload,
};

use crate::{panic::{throw}};

use std::sync::{Arc, Mutex};

pub(crate) const MSANENABLED: bool = false;


pub fn msanread(addr: Arc<Mutex<Option<usize>>>, sz: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("msan".to_string()))));
}

pub fn msanwrite(addr: Arc<Mutex<Option<usize>>>, sz: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("msan".to_string()))));
}

pub fn msanmalloc(addr: Arc<Mutex<Option<usize>>>, sz: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("msan".to_string()))));
}

pub fn msanfree(addr: Arc<Mutex<Option<usize>>>, sz: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("msan".to_string()))));
}