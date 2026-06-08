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
    format_slice,
    format_slice_values,
    format_slice_wrapped,
    go_any_eq,
    go_recover,
    go_resume_unrecovered_panic,
    go_store_panic_payload,
};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex as StdMutex};

/// Approximation of notifyList in runtime/sema.go. Size and alignment must
/// agree.
#[derive(Debug, Clone)]
pub struct notifyList {
    pub wait: Arc<StdMutex<Option<u32>>>,
    pub notify: Arc<StdMutex<Option<u32>>>,
    pub lock: Arc<StdMutex<Option<usize>>>,
    pub head: Arc<StdMutex<Option<usize>>>,
    pub tail: Arc<StdMutex<Option<usize>>>,
}

impl notifyList {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.wait.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.notify.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.lock.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.head.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.tail.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) };
        Self {
            wait: __go_clone_0_0,
            notify: __go_clone_1_0,
            lock: __go_clone_2_0,
            head: __go_clone_3_0,
            tail: __go_clone_4_0,
        }
    }
}


impl Default for notifyList {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(StdMutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(StdMutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(StdMutex::new(Some(0)));
        let __go_default_3_0 = Arc::new(StdMutex::new(Some(0)));
        let __go_default_4_0 = Arc::new(StdMutex::new(Some(0)));
        Self {
            wait: __go_default_0_0,
            notify: __go_default_1_0,
            lock: __go_default_2_0,
            head: __go_default_3_0,
            tail: __go_default_4_0,
        }
    }
}

impl std::fmt::Display for notifyList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.wait.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.notify.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.lock.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.head.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.tail.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4)
    }
}

impl GoJsonDecode for notifyList {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl GoValueClone for notifyList {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
