use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const GO122_INTERFACE_SWITCH_CACHE: bool = true;


#[derive(Debug, Clone)]
pub struct InterfaceSwitchCache {
    pub mask: Arc<Mutex<Option<usize>>>,
    pub entries: Arc<Mutex<Option<[InterfaceSwitchCacheEntry; 1]>>>,
}

impl InterfaceSwitchCache {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.mask.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.entries.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            mask: __go_clone_0_0,
            entries: __go_clone_1_0,
        }
    }
}


impl Default for InterfaceSwitchCache {
    fn default() -> Self {
        Self { mask: Arc::new(Mutex::new(Some(0))), entries: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default())))) }
    }
}

impl std::fmt::Display for InterfaceSwitchCache {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.mask.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice(&self.entries));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for InterfaceSwitchCache {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Mask") {
            out.mask = <Arc<Mutex<Option<usize>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct InterfaceSwitchCacheEntry {
    pub typ: Arc<Mutex<Option<usize>>>,
    pub case: Arc<Mutex<Option<i32>>>,
    pub itab: Arc<Mutex<Option<usize>>>,
}

impl InterfaceSwitchCacheEntry {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.typ.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.case.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.itab.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            typ: __go_clone_0_0,
            case: __go_clone_1_0,
            itab: __go_clone_2_0,
        }
    }
}


impl Default for InterfaceSwitchCacheEntry {
    fn default() -> Self {
        Self { typ: Arc::new(Mutex::new(Some(0))), case: Arc::new(Mutex::new(Some(0))), itab: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for InterfaceSwitchCacheEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.typ.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.case.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.itab.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}

impl GoJsonDecode for InterfaceSwitchCacheEntry {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Typ") {
            out.typ = <Arc<Mutex<Option<usize>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Case") {
            out.case = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Itab") {
            out.itab = <Arc<Mutex<Option<usize>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct TypeAssertCache {
    pub mask: Arc<Mutex<Option<usize>>>,
    pub entries: Arc<Mutex<Option<[TypeAssertCacheEntry; 1]>>>,
}

impl TypeAssertCache {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.mask.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.entries.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            mask: __go_clone_0_0,
            entries: __go_clone_1_0,
        }
    }
}


impl Default for TypeAssertCache {
    fn default() -> Self {
        Self { mask: Arc::new(Mutex::new(Some(0))), entries: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default())))) }
    }
}

impl std::fmt::Display for TypeAssertCache {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.mask.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice(&self.entries));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for TypeAssertCache {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Mask") {
            out.mask = <Arc<Mutex<Option<usize>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct TypeAssertCacheEntry {
    pub typ: Arc<Mutex<Option<usize>>>,
    pub itab: Arc<Mutex<Option<usize>>>,
}

impl TypeAssertCacheEntry {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.typ.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.itab.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            typ: __go_clone_0_0,
            itab: __go_clone_1_0,
        }
    }
}


impl Default for TypeAssertCacheEntry {
    fn default() -> Self {
        Self { typ: Arc::new(Mutex::new(Some(0))), itab: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for TypeAssertCacheEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.typ.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.itab.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for TypeAssertCacheEntry {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Typ") {
            out.typ = <Arc<Mutex<Option<usize>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Itab") {
            out.itab = <Arc<Mutex<Option<usize>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


impl GoValueClone for InterfaceSwitchCache {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for InterfaceSwitchCacheEntry {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for TypeAssertCache {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for TypeAssertCacheEntry {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
