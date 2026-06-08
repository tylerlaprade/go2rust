use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{mgc::{AnonymousStruct12}};

use std::fmt::{Display, Formatter};

pub(crate) const OS_RELAX_MIN_N_S: i32 = 0;


/// winlibcall is not implemented on non-Windows systems,
/// but it is used in non-OS-specific parts of the runtime.
/// Define it as an empty struct to avoid wasting stack space.
#[derive(Debug, Clone, Default)]
pub struct winlibcall {
}

impl winlibcall {
    pub fn __go_value_clone(&self) -> Self {
        Self {  }
    }
}

impl std::fmt::Display for winlibcall {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}

impl GoJsonDecode for winlibcall {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static haveHighResSleep: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *haveHighResSleep.lock().unwrap() = Some(false);
    *haveHighResSleep.lock().unwrap() = Some(true);
}


pub(crate) fn __go_zero_globals() {
    *haveHighResSleep.lock().unwrap() = Some(false);
}


pub(crate) fn __go_init_order_41() {
    *haveHighResSleep.lock().unwrap() = Some(true);
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for winlibcall {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
