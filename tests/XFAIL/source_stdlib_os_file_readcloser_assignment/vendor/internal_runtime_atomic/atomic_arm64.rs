use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef};

use crate::stubs::*;
use crate::types::*;
use crate::types_64bit::*;
use crate::unaligned::*;

use std::sync::{Arc, Mutex};

pub(crate) const OFFSET_A_R_M64_HAS_A_T_O_M_I_C_S: usize = std::mem::offset_of!(internal_cpu::ARM64, has_a_t_o_m_i_c_s);


///go:noescape
pub fn xadd(ptr: GoPtr<u32>, delta: Arc<Mutex<Option<i32>>>) -> u32 {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn xadd64(ptr: Arc<Mutex<Option<u64>>>, delta: Arc<Mutex<Option<i64>>>) -> u64 {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn xadduintptr(ptr: Arc<Mutex<Option<usize>>>, delta: Arc<Mutex<Option<usize>>>) -> usize {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn xchg8(ptr: GoPtr<u8>, new: Arc<Mutex<Option<u8>>>) -> u8 {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn xchg(ptr: Arc<Mutex<Option<u32>>>, new: Arc<Mutex<Option<u32>>>) -> u32 {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn xchg64(ptr: Arc<Mutex<Option<u64>>>, new: Arc<Mutex<Option<u64>>>) -> u64 {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn xchguintptr(ptr: Arc<Mutex<Option<usize>>>, new: Arc<Mutex<Option<usize>>>) -> usize {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn load(ptr: GoPtr<u32>) -> u32 {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn load8(ptr: GoPtr<u8>) -> u8 {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn load64(ptr: Arc<Mutex<Option<u64>>>) -> u64 {
    unimplemented!("Go function declaration has no body");
}


/// NO go:noescape annotation; *ptr escapes if result escapes (#31525)
pub fn loadp(ptr: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<usize>>> {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn load_acq(addr: Arc<Mutex<Option<u32>>>) -> u32 {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn load_acq64(ptr: Arc<Mutex<Option<u64>>>) -> u64 {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn load_acquintptr(ptr: Arc<Mutex<Option<usize>>>) -> usize {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn or8(ptr: GoPtr<u8>, val: Arc<Mutex<Option<u8>>>) {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn and8(ptr: GoPtr<u8>, val: Arc<Mutex<Option<u8>>>) {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn and(ptr: Arc<Mutex<Option<u32>>>, val: Arc<Mutex<Option<u32>>>) {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn or(ptr: Arc<Mutex<Option<u32>>>, val: Arc<Mutex<Option<u32>>>) {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn cas64(ptr: Arc<Mutex<Option<u64>>>, old: Arc<Mutex<Option<u64>>>, new: Arc<Mutex<Option<u64>>>) -> bool {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn cas_rel(ptr: Arc<Mutex<Option<u32>>>, old: Arc<Mutex<Option<u32>>>, new: Arc<Mutex<Option<u32>>>) -> bool {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn store(ptr: Arc<Mutex<Option<u32>>>, val: Arc<Mutex<Option<u32>>>) {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn store8(ptr: Arc<Mutex<Option<u8>>>, val: Arc<Mutex<Option<u8>>>) {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn store64(ptr: Arc<Mutex<Option<u64>>>, val: Arc<Mutex<Option<u64>>>) {
    unimplemented!("Go function declaration has no body");
}


/// NO go:noescape annotation; see atomic_pointer.go.
pub fn storep_no_w_b(ptr: Arc<Mutex<Option<usize>>>, val: Arc<Mutex<Option<usize>>>) {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn store_rel(ptr: Arc<Mutex<Option<u32>>>, val: Arc<Mutex<Option<u32>>>) {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn store_rel64(ptr: Arc<Mutex<Option<u64>>>, val: Arc<Mutex<Option<u64>>>) {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn store_reluintptr(ptr: Arc<Mutex<Option<usize>>>, val: Arc<Mutex<Option<usize>>>) {
    unimplemented!("Go function declaration has no body");
}
