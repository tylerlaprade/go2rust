use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{debuglog::{dloggerImpl}, mgc::{AnonymousStruct12}};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const DLOG_ENABLED: bool = false;


#[derive(Debug, Clone, Default)]
pub struct dlogPerM {
}

impl dlogPerM {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
    }
}

impl std::fmt::Display for dlogPerM {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}

impl GoJsonDecode for dlogPerM {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub fn put_cached_dlogger(l: Arc<Mutex<Option<dloggerImpl>>>) -> bool {
    false
}

impl GoValueClone for dlogPerM {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
