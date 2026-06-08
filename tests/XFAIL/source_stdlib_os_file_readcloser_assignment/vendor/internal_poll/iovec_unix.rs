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

use std::sync::{Arc, Mutex};

pub fn new_iovec_with_base(base: GoPtr<u8>) -> Arc<Mutex<Option<syscall::ztypes_darwin_arm64::Iovec>>> {
    Arc::new(Mutex::new(Some(syscall::ztypes_darwin_arm64::Iovec { base: { let __go_ptr = base.clone(); match __go_ptr { GoPtr::Nil => syscall::GoPtr::nil(), GoPtr::Local(__value) => syscall::GoPtr::local(__value.clone()), GoPtr::Raw(__addr) => syscall::GoPtr::raw(__addr), GoPtr::SliceElem(__value) => syscall::GoPtr::slice_elem(syscall::GoSliceElemPtr::new(__value.slice_handle(), __value.index())), GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } }, ..Default::default() })))
}