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

use crate::{runtime2::{guintptr, m}};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// A coro represents extra concurrency without extra parallelism,
/// as would be needed for a coroutine implementation.
/// The coro does not represent a specific coroutine, only the ability
/// to do coroutine-style control transfers.
/// It can be thought of as like a special channel that always has
/// a goroutine blocked on it. If another goroutine calls coroswitch(c),
/// the caller becomes the goroutine blocked in c, and the goroutine
/// formerly blocked in c starts running.
/// These switches continue until a call to coroexit(c),
/// which ends the use of the coro by releasing the blocked
/// goroutine in c and exiting the current goroutine.
///
/// Coros are heap allocated and garbage collected, so that user code
/// can hold a pointer to a coro without causing potential dangling
/// pointer errors.
#[derive(Clone)]
pub struct coro {
    pub gp: Arc<Mutex<Option<guintptr>>>,
    pub f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<coro>>>) -> () + Send + Sync>>>>,
    pub mp: Arc<Mutex<Option<m>>>,
    pub locked_ext: Arc<Mutex<Option<u32>>>,
    pub locked_int: Arc<Mutex<Option<u32>>>,
}

impl coro {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.gp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.f.clone();
        let __go_clone_2_0 = self.mp.clone();
        let __go_clone_3_0 = { let __guard = self.locked_ext.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.locked_int.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            gp: __go_clone_0_0,
            f: __go_clone_1_0,
            mp: __go_clone_2_0,
            locked_ext: __go_clone_3_0,
            locked_int: __go_clone_4_0,
        }
    }
}


impl Default for coro {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        let __go_default_2_0 = Arc::new(Mutex::new(None));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            gp: __go_default_0_0,
            f: __go_default_1_0,
            mp: __go_default_2_0,
            locked_ext: __go_default_3_0,
            locked_int: __go_default_4_0,
        }
    }
}

impl std::fmt::Display for coro {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.gp.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", "<func>");
        let __go_fmt_2 = format!("{}", { let __guard = self.mp.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_3 = format!("{}", (*self.locked_ext.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.locked_int.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4)
    }
}


impl GoValueClone for coro {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
