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

use crate::{poolqueue::{AnonymousStruct1}};

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex as StdMutex};

/// noCopy may be added to structs which must not be copied
/// after the first use.
///
/// See https://golang.org/issues/8005#issuecomment-190753527
/// for details.
///
/// Note that it must not be embedded, due to the Lock and Unlock methods.
#[derive(Debug, Clone, Default)]
pub struct noCopy {
}

impl noCopy {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
    }
}

impl std::fmt::Display for noCopy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}

impl GoJsonDecode for noCopy {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl noCopy {
    /// Lock is a no-op used by -copylocks checker from `go vet`.
    pub fn lock(&self) {
    }

    pub fn unlock(&self) {
    }
}

impl Locker for noCopy {
    fn lock(&mut self) {
        noCopy::lock(self)
    }
    fn unlock(&mut self) {
        noCopy::unlock(self)
    }
    fn __go_clone_box_locker(&self) -> Box<dyn Locker + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Locker + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_locker(&self, other: &(dyn Locker + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<noCopy>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct noCopyPtr(pub Arc<StdMutex<Option<noCopy>>>);

impl std::fmt::Display for noCopyPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Locker for noCopyPtr {
    fn lock(&mut self) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        noCopy::lock(__recv)
    }
    fn unlock(&mut self) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        noCopy::unlock(__recv)
    }
    fn __go_clone_box_locker(&self) -> Box<dyn Locker + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Locker + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_locker(&self, other: &(dyn Locker + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<noCopyPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl GoValueClone for noCopy {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
