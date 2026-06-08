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
    Arc::new(Mutex::new(Some(syscall::ztypes_darwin_arm64::Iovec { base: {
        let __go_ptr = base.clone();
        match __go_ptr {
            GoPtr::Nil => syscall::GoPtr::nil(),
            GoPtr::Local(__value) => syscall::GoPtr::local(__value.clone()),
            GoPtr::Raw(__addr) => syscall::GoPtr::raw(__addr),
            GoPtr::SliceElem(__value) => syscall::GoPtr::slice_elem(syscall::GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
            GoPtr::ArrayElem(__value) => syscall::GoPtr::array_elem_foreign(std::sync::Arc::new({ let __value = __value.clone(); move || __value.borrow_dyn() }), std::sync::Arc::new({ let __value = __value.clone(); move |__assigned| __value.assign_dyn(__assigned) }), std::sync::Arc::new({ let __value = __value.clone(); move |__callback| __value.with_mut_dyn(__callback) }), std::sync::Arc::new({ let __value = __value.clone(); move || __value.identity_dyn() })),
        }
    }, ..Default::default() })))
}