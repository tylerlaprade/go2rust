use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{malloc::{notInHeap}};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct slice {
    pub array: Arc<Mutex<Option<usize>>>,
    pub len: Arc<Mutex<Option<i32>>>,
    pub cap: Arc<Mutex<Option<i32>>>,
}

impl slice {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.array.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.len.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.cap.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            array: __go_clone_0_0,
            len: __go_clone_1_0,
            cap: __go_clone_2_0,
        }
    }
}


impl Default for slice {
    fn default() -> Self {
        Self { array: Arc::new(Mutex::new(Some(0))), len: Arc::new(Mutex::new(Some(0))), cap: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for slice {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.array.lock().unwrap().as_ref().unwrap()), (*self.len.lock().unwrap().as_ref().unwrap()), (*self.cap.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for slice {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A notInHeapSlice is a slice backed by internal/runtime/sys.NotInHeap memory.
#[derive(Clone)]
pub struct notInHeapSlice {
    pub array: GoPtr<crate::malloc::notInHeap>,
    pub len: Arc<Mutex<Option<i32>>>,
    pub cap: Arc<Mutex<Option<i32>>>,
}

impl notInHeapSlice {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.array.clone();
        let __go_clone_1_0 = { let __guard = self.len.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.cap.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            array: __go_clone_0_0,
            len: __go_clone_1_0,
            cap: __go_clone_2_0,
        }
    }
}


impl Default for notInHeapSlice {
    fn default() -> Self {
        Self { array: GoPtr::nil(), len: Arc::new(Mutex::new(Some(0))), cap: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for notInHeapSlice {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", { if self.array.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } }, (*self.len.lock().unwrap().as_ref().unwrap()), (*self.cap.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for notInHeapSlice {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl GoValueClone for slice {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for notInHeapSlice {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
