use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoByteSequence,
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

use std::sync::{Arc, Mutex};

pub fn goroot_zone_source(goroot: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, bool) {
    if { let __tmp_x = (*goroot.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(Some("".to_string()))), false);
    }
    return (Arc::new(Mutex::new(Some(format!("{}{}", { let __v = (*goroot.lock().unwrap().as_ref().unwrap()).clone(); __v }, "/lib/time/zoneinfo.zip".to_string())))), true);
}