use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{panic::{throw}, r#extern::{G_O_A_R_C_H, G_O_O_S}, tagptr::{taggedPointer}};

use std::sync::{Arc, Mutex};

pub(crate) const ADDR_BITS: i32 = 48;
pub(crate) const TAG_BITS: i32 = 64 - ADDR_BITS + 3;
pub(crate) const AIX_ADDR_BITS: i32 = 57;
pub(crate) const AIX_TAG_BITS: i32 = 64 - AIX_ADDR_BITS + 3;
pub(crate) const RISCV64_ADDR_BITS: i32 = 56;
pub(crate) const RISCV64_TAG_BITS: i32 = 64 - RISCV64_ADDR_BITS + 3;


pub(crate) const TAGGED_POINTER_BITS: i32 = (internal_goos::IS_AIX * AIX_TAG_BITS) + (internal_goarch::IS_RISCV64 * RISCV64_TAG_BITS) + ((1 - internal_goos::IS_AIX) * (1 - internal_goarch::IS_RISCV64) * TAG_BITS);


impl crate::tagptr::taggedPointer {
    /// Pointer returns the pointer from a taggedPointer.
    pub fn pointer(&self) -> Arc<Mutex<Option<usize>>> {
        if { let __tmp_x = "arm64".to_string(); let __tmp_y = "amd64".to_string(); __tmp_x == __tmp_y } {
                // amd64 systems can place the stack above the VA hole, so we need to sign extend
                // val before unpacking.
        return Arc::new(Mutex::new(Some(({ let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = TAG_BITS; __tmp_x >> __tmp_y }; let __tmp_y = 3; __tmp_x << __tmp_y }) as usize)));
    }
                // amd64 systems can place the stack above the VA hole, so we need to sign extend
                // val before unpacking.
        if { let __tmp_x = "darwin".to_string(); let __tmp_y = "aix".to_string(); __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some(((((((*self.0.lock().unwrap().as_ref().unwrap()) >> AIX_TAG_BITS) << 3i32)) | (0xa << 56i32))) as usize)));
    }
        if { let __tmp_x = "arm64".to_string(); let __tmp_y = "riscv64".to_string(); __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some(((((*self.0.lock().unwrap().as_ref().unwrap()) >> RISCV64_TAG_BITS) << 3i32)) as usize)));
    }
        Arc::new(Mutex::new(Some(((((*self.0.lock().unwrap().as_ref().unwrap()) >> TAG_BITS) << 3i32)) as usize)))
    }

    /// Tag returns the tag from a taggedPointer.
    pub fn tag(&self) -> usize {
        (*Arc::new(Mutex::new(Some((((*self.0.lock().unwrap().as_ref().unwrap()) & (({ let __tmp_x = 1; let __tmp_y = TAGGED_POINTER_BITS; __tmp_x << __tmp_y } - 1)))) as usize))).lock().unwrap().as_ref().unwrap())
    }
}

/// taggedPointerPack created a taggedPointer from a pointer and a tag.
/// Tag bits that don't fit in the result are discarded.
pub fn tagged_pointer_pack(ptr: Arc<Mutex<Option<usize>>>, tag: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<crate::tagptr::taggedPointer>>> {
    if { let __tmp_x = "darwin".to_string(); let __tmp_y = "aix".to_string(); __tmp_x == __tmp_y } {
        if { let __tmp_x = "arm64".to_string(); let __tmp_y = "ppc64".to_string(); __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("check this code for aix on non-ppc64".to_string()))));
    }
        return Arc::new(Mutex::new(Some(crate::tagptr::taggedPointer(Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some((*ptr.lock().unwrap().as_ref().unwrap()) as usize as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = 64; let __tmp_y = AIX_ADDR_BITS; __tmp_x - __tmp_y }); __tmp_x << __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*tag.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (((1 as usize) << (AIX_TAG_BITS as usize)) - (1 as usize)) as usize; __tmp_x & __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x | __tmp_y } as u64)))))));
    }
    if { let __tmp_x = "arm64".to_string(); let __tmp_y = "riscv64".to_string(); __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some(crate::tagptr::taggedPointer(Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some((*ptr.lock().unwrap().as_ref().unwrap()) as usize as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = 64; let __tmp_y = RISCV64_ADDR_BITS; __tmp_x - __tmp_y }); __tmp_x << __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*tag.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (((1 as usize) << (RISCV64_TAG_BITS as usize)) - (1 as usize)) as usize; __tmp_x & __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x | __tmp_y } as u64)))))));
    }
    Arc::new(Mutex::new(Some(crate::tagptr::taggedPointer(Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some((*ptr.lock().unwrap().as_ref().unwrap()) as usize as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = 64; let __tmp_y = ADDR_BITS; __tmp_x - __tmp_y }); __tmp_x << __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*tag.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (((1 as usize) << (TAG_BITS as usize)) - (1 as usize)) as usize; __tmp_x & __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x | __tmp_y } as u64)))))))
}