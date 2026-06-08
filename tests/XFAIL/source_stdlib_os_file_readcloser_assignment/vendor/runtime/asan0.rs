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

pub(crate) const ASANENABLED: bool = false;


pub fn asanread(addr: Arc<Mutex<Option<usize>>>, sz: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("asan".to_string()))));
}

pub fn asanwrite(addr: Arc<Mutex<Option<usize>>>, sz: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("asan".to_string()))));
}

pub fn asanunpoison(addr: Arc<Mutex<Option<usize>>>, sz: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("asan".to_string()))));
}

pub fn asanpoison(addr: Arc<Mutex<Option<usize>>>, sz: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("asan".to_string()))));
}