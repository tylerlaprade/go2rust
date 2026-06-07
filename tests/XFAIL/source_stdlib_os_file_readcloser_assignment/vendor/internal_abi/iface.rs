use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped};

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

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// The first word of every non-empty interface type contains an *ITab.
/// It records the underlying concrete type (Type), the interface type it
/// is implementing (Inter), and some ancillary information.
///
/// allocated in non-garbage-collected memory
#[derive(Clone)]
pub struct ITab {
    pub inter: GoPtr<crate::r#type::InterfaceType>,
    pub r#type: Arc<Mutex<Option<Type>>>,
    pub hash: Arc<Mutex<Option<u32>>>,
    pub fun: Arc<Mutex<Option<[usize; 1]>>>,
}

impl ITab {
    pub fn __go_value_clone(&self) -> Self {
        Self { inter: self.inter.clone(), r#type: self.r#type.clone(), hash: { let __guard = self.hash.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, fun: { let __guard = self.fun.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for ITab {
    fn default() -> Self {
        Self { inter: GoPtr::nil(), r#type: Arc::new(Mutex::new(None)), hash: Arc::new(Mutex::new(Some(0))), fun: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))) }
    }
}

impl std::fmt::Display for ITab {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", { if self.inter.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } }, { let __guard = self.r#type.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.hash.lock().unwrap().as_ref().unwrap()), format_slice(&self.fun))
    }
}

impl GoJsonDecode for ITab {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Hash") {
            out.hash = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Fun") {
            out.fun = <Arc<Mutex<Option<[usize; 1]>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


/// EmptyInterface describes the layout of a "interface{}" or a "any."
/// These are represented differently than non-empty interface, as the first
/// word always points to an abi.Type.
#[derive(Clone)]
pub struct EmptyInterface {
    pub r#type: GoPtr<crate::r#type::Type>,
    pub data: Arc<Mutex<Option<usize>>>,
}

impl EmptyInterface {
    pub fn __go_value_clone(&self) -> Self {
        Self { r#type: self.r#type.clone(), data: { let __guard = self.data.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for EmptyInterface {
    fn default() -> Self {
        Self { r#type: GoPtr::nil(), data: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for EmptyInterface {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", { if self.r#type.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } }, (*self.data.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for EmptyInterface {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl GoValueClone for ITab {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for EmptyInterface {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
