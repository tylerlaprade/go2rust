use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_slice, format_slice_values, format_slice_wrapped, go_any_eq, go_lookup_embedded_owner, go_recover, go_register_embedded_owner, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::cond::*;
use crate::hashtriemap::*;
use crate::mutex::*;
use crate::once::*;
use crate::oncefunc::*;
use crate::pool::*;
use crate::poolqueue::*;
use crate::runtime::*;
use crate::rwmutex::*;
use crate::waitgroup::*;

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
        Self { wait: { let __guard = self.wait.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) }, notify: { let __guard = self.notify.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) }, lock: { let __guard = self.lock.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) }, head: { let __guard = self.head.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) }, tail: { let __guard = self.tail.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) } }
    }
}


impl Default for notifyList {
    fn default() -> Self {
        Self { wait: Arc::new(StdMutex::new(Some(0))), notify: Arc::new(StdMutex::new(Some(0))), lock: Arc::new(StdMutex::new(Some(0))), head: Arc::new(StdMutex::new(Some(0))), tail: Arc::new(StdMutex::new(Some(0))) }
    }
}

impl std::fmt::Display for notifyList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.wait.lock().unwrap().as_ref().unwrap()), (*self.notify.lock().unwrap().as_ref().unwrap()), (*self.lock.lock().unwrap().as_ref().unwrap()), (*self.head.lock().unwrap().as_ref().unwrap()), (*self.tail.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for notifyList {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone, Default)]
pub struct AnonymousStruct1 {
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self {  }
    }
}


impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}

impl GoJsonDecode for AnonymousStruct1 {
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
