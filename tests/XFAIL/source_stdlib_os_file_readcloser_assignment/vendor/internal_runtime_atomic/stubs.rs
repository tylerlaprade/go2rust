use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef};

use crate::atomic_arm64::*;
use crate::types::*;
use crate::types_64bit::*;
use crate::unaligned::*;

use std::sync::{Arc, Mutex};

///go:noescape
pub fn cas(ptr: GoPtr<u32>, old: Arc<Mutex<Option<u32>>>, new: Arc<Mutex<Option<u32>>>) -> bool {
    unimplemented!("Go function declaration has no body");
}


/// NO go:noescape annotation; see atomic_pointer.go.
pub fn casp1(ptr: GoPtr<usize>, old: Arc<Mutex<Option<usize>>>, new: Arc<Mutex<Option<usize>>>) -> bool {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn casint32(ptr: Arc<Mutex<Option<i32>>>, old: Arc<Mutex<Option<i32>>>, new: Arc<Mutex<Option<i32>>>) -> bool {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn casint64(ptr: Arc<Mutex<Option<i64>>>, old: Arc<Mutex<Option<i64>>>, new: Arc<Mutex<Option<i64>>>) -> bool {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn casuintptr(ptr: GoPtr<usize>, old: Arc<Mutex<Option<usize>>>, new: Arc<Mutex<Option<usize>>>) -> bool {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn storeint32(ptr: Arc<Mutex<Option<i32>>>, new: Arc<Mutex<Option<i32>>>) {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn storeint64(ptr: Arc<Mutex<Option<i64>>>, new: Arc<Mutex<Option<i64>>>) {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn storeuintptr(ptr: Arc<Mutex<Option<usize>>>, new: Arc<Mutex<Option<usize>>>) {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn loaduintptr(ptr: GoPtr<usize>) -> usize {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn loaduint(ptr: Arc<Mutex<Option<u64>>>) -> u64 {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn loadint32(ptr: Arc<Mutex<Option<i32>>>) -> i32 {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn loadint64(ptr: Arc<Mutex<Option<i64>>>) -> i64 {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn xaddint32(ptr: Arc<Mutex<Option<i32>>>, delta: Arc<Mutex<Option<i32>>>) -> i32 {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn xaddint64(ptr: Arc<Mutex<Option<i64>>>, delta: Arc<Mutex<Option<i64>>>) -> i64 {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn xchgint32(ptr: Arc<Mutex<Option<i32>>>, new: Arc<Mutex<Option<i32>>>) -> i32 {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn xchgint64(ptr: Arc<Mutex<Option<i64>>>, new: Arc<Mutex<Option<i64>>>) -> i64 {
    unimplemented!("Go function declaration has no body");
}
