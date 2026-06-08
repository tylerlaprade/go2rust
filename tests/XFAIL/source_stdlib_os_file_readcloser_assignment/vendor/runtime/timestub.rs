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

use crate::{sys_darwin::{walltime}, time_nofake::{nanotime}};

use std::sync::{Arc, Mutex};

/// time_now should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - gitee.com/quant1x/gox
///   - github.com/phuslu/log
///   - github.com/sethvargo/go-limiter
///   - github.com/ulule/limiter/v3
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname time_now time.now
pub fn time_now() -> (i64, i32, i64) {
    let mut sec: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut nsec: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut mono: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));

    { let (__tmp_0, __tmp_1) = walltime(); *sec.lock().unwrap() = Some(__tmp_0); *nsec.lock().unwrap() = Some(__tmp_1); };
    return ({ let __v = (*sec.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*nsec.lock().unwrap().as_ref().unwrap()).clone(); __v }, nanotime());
}