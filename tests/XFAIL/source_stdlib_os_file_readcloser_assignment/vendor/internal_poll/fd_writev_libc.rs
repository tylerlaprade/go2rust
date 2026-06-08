use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

///go:linkname writev syscall.writev
pub fn writev(fd: Arc<Mutex<Option<i32>>>, iovecs: Arc<Mutex<Option<Vec<syscall::ztypes_darwin_arm64::Iovec>>>>) -> (usize, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    unimplemented!("Go function declaration has no body");
}
