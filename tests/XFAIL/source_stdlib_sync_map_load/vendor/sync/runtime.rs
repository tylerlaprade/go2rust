use go2rust_stdlib_stubs::*;

use crate::{GoAtomicPointer, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_slice, format_slice_values, format_slice_wrapped, go_any_eq};

use crate::cond::*;
use crate::hashtriemap::*;
use crate::mutex::*;
use crate::once::*;
use crate::oncefunc::*;
use crate::pool::*;
use crate::poolqueue::*;
use crate::runtime2::*;
use crate::rwmutex::*;
use crate::waitgroup::*;

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex as StdMutex};

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


/// Ensure that sync and runtime agree on size of notifyList.
pub fn runtime_notify_list_check(size: Arc<StdMutex<Option<usize>>>) {
    let _ = size;
}


fn __go_init_0() {
    let mut n: Arc<StdMutex<Option<notifyList>>> = Arc::new(StdMutex::new(Some(Default::default())));
    runtime_notify_list_check(Arc::new(StdMutex::new(Some(std::mem::size_of::<notifyList>()))));
}

pub(crate) fn __go_init_functions() {
    self::__go_init_0();
}


pub(crate) fn __go_init_all() {
    self::__go_init_0();
}
