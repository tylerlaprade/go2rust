use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{panic::{throw}, runtime2::{g}};

use std::sync::{Arc, Mutex};

pub(crate) const RACEENABLED: bool = false;


pub fn raceproccreate() -> usize {
    throw(Arc::new(Mutex::new(Some("race".to_string()))));
    0
}

pub fn raceprocdestroy(ctx: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("race".to_string()))));
}

pub fn racemapshadow(addr: Arc<Mutex<Option<usize>>>, size: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("race".to_string()))));
}

pub fn racereadpc(addr: Arc<Mutex<Option<usize>>>, callerpc: Arc<Mutex<Option<usize>>>, pc: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("race".to_string()))));
}

pub fn racereadrangepc(addr: Arc<Mutex<Option<usize>>>, sz: Arc<Mutex<Option<usize>>>, callerpc: Arc<Mutex<Option<usize>>>, pc: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("race".to_string()))));
}

pub fn raceacquire(addr: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("race".to_string()))));
}

pub fn raceacquireg(gp: Arc<Mutex<Option<g>>>, addr: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("race".to_string()))));
}

pub fn raceacquirectx(racectx: Arc<Mutex<Option<usize>>>, addr: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("race".to_string()))));
}

pub fn racerelease(addr: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("race".to_string()))));
}

pub fn racereleaseg(gp: Arc<Mutex<Option<g>>>, addr: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("race".to_string()))));
}

pub fn racereleaseacquire(addr: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("race".to_string()))));
}

pub fn racereleaseacquireg(gp: Arc<Mutex<Option<g>>>, addr: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("race".to_string()))));
}

pub fn racereleasemerge(addr: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("race".to_string()))));
}

pub fn racereleasemergeg(gp: GoPtr<crate::runtime2::g>, addr: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("race".to_string()))));
}

pub fn racefingo() {
    throw(Arc::new(Mutex::new(Some("race".to_string()))));
}

pub fn racemalloc(p: Arc<Mutex<Option<usize>>>, sz: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("race".to_string()))));
}

pub fn racefree(p: Arc<Mutex<Option<usize>>>, sz: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("race".to_string()))));
}

pub fn racegostart(pc: Arc<Mutex<Option<usize>>>) -> usize {
    throw(Arc::new(Mutex::new(Some("race".to_string()))));
    0
}

pub fn racectxend(racectx: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("race".to_string()))));
}