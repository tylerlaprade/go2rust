use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_lookup_embedded_owner, go_register_embedded_owner};

use crate::r#mod::*;
use crate::abi_arm64::*;
use crate::compiletype::*;
use crate::escape::*;
use crate::funcpc::*;
use crate::map_noswiss::*;
use crate::map_select_swiss::*;
use crate::map_swiss::*;
use crate::rangefuncconsts::*;
use crate::runtime::*;
use crate::stack::*;
use crate::switch::*;
use crate::symtab::*;
use crate::r#type::*;

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// EmptyInterface describes the layout of a "interface{}" or a "any."
/// These are represented differently than non-empty interface, as the first
/// word always points to an abi.Type.
#[derive(Clone)]
pub struct EmptyInterface {
    pub r#type: Arc<Mutex<Option<Type>>>,
    pub data: Arc<Mutex<Option<usize>>>,
}

impl EmptyInterface {
    pub fn __go_value_clone(&self) -> Self {
        Self { r#type: self.r#type.clone(), data: { let __guard = self.data.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for EmptyInterface {
    fn default() -> Self {
        Self { r#type: Arc::new(Mutex::new(None)), data: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for EmptyInterface {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", { let __guard = self.r#type.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.data.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for EmptyInterface {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl GoValueClone for EmptyInterface {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
