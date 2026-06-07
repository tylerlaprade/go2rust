use go2rust_stdlib_stubs::*;

use std::any::Any;
use std::cmp::Ord;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

fn format_map<K: Display + Ord + Clone, V>(map: &Arc<Mutex<Option<BTreeMap<K, Arc<Mutex<Option<V>>>>>>>) -> String
where
    V: Display,
{
    let guard = map.lock().unwrap();
    if let Some(ref m) = *guard {
        let mut items: Vec<_> = m.iter().collect();
        items.sort_by_key(|(k, _)| (*k).clone());

        let formatted: Vec<String> = items
            .into_iter()
            .map(|(k, v)| {
                let v_guard = v.lock().unwrap();
                if let Some(ref val) = *v_guard {
                    format!("{}:{}", k, val)
                } else {
                    format!("{}:<nil>", k)
                }
            })
            .collect();

        format!("map[{}]", formatted.join(" "))
    } else {
        "map[]".to_string()
    }
}
fn format_slice<T, C>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[T]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn format_slice_values<T>(slice: &[T]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| v.to_string()).collect();
    format!("[{}]", formatted.join(" "))
}

fn format_slice_wrapped<T, C>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[Arc<Mutex<Option<T>>>]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| {
            let inner = v.lock().unwrap();
            match inner.as_ref() {
                Some(value) => format!("&{}", value),
                None => "<nil>".to_string(),
            }
        }).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

/// CovMetaBlob is a container for holding the meta-data symbol (an
/// RODATA variable) for an instrumented Go package. Here "p" points to
/// the symbol itself, "len" is the length of the sym in bytes, and
/// "hash" is an md5sum for the sym computed by the compiler. When
/// the init function for a coverage-instrumented package executes, it
/// will make a call into the runtime which will create a covMetaBlob
/// object for the package and chain it onto a global list.
#[derive(Debug, Clone)]
pub struct CovMetaBlob {
    pub p: Arc<Mutex<Option<u8>>>,
    pub len: Arc<Mutex<Option<u32>>>,
    pub hash: Arc<Mutex<Option<[u8; 16]>>>,
    pub pkg_path: Arc<Mutex<Option<String>>>,
    pub pkg_i_d: Arc<Mutex<Option<i32>>>,
    pub counter_mode: Arc<Mutex<Option<u8>>>,
    pub counter_granularity: Arc<Mutex<Option<u8>>>,
}

impl CovMetaBlob {
    pub fn __go_value_clone(&self) -> Self {
        Self { p: self.p.clone(), len: { let __guard = self.len.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, hash: { let __guard = self.hash.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pkg_path: { let __guard = self.pkg_path.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pkg_i_d: { let __guard = self.pkg_i_d.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, counter_mode: { let __guard = self.counter_mode.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, counter_granularity: { let __guard = self.counter_granularity.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for CovMetaBlob {
    fn default() -> Self {
        Self { p: Arc::new(Mutex::new(None)), len: Arc::new(Mutex::new(Some(0))), hash: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), pkg_path: Arc::new(Mutex::new(Some(String::new()))), pkg_i_d: Arc::new(Mutex::new(Some(0))), counter_mode: Arc::new(Mutex::new(Some(0))), counter_granularity: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for CovMetaBlob {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {}}}", { let __guard = self.p.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.len.lock().unwrap().as_ref().unwrap()), format_slice(&self.hash), (*self.pkg_path.lock().unwrap().as_ref().unwrap()), (*self.pkg_i_d.lock().unwrap().as_ref().unwrap()), (*self.counter_mode.lock().unwrap().as_ref().unwrap()), (*self.counter_granularity.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for CovMetaBlob {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("P") {
            out.p = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Len") {
            out.len = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Hash") {
            out.hash = <Arc<Mutex<Option<[u8; 16]>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("PkgPath") {
            out.pkg_path = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("PkgID") {
            out.pkg_i_d = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("CounterMode") {
            out.counter_mode = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("CounterGranularity") {
            out.counter_granularity = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


pub static Meta: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<AnonymousStruct1>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *Meta.lock().unwrap() = Some(Default::default());
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct1 {
    pub list: Arc<Mutex<Option<Vec<CovMetaBlob>>>>,
    pub pkg_map: Arc<Mutex<Option<BTreeMap<i32, Arc<Mutex<Option<i32>>>>>>>,
    pub hard_coded_list_needs_updating: Arc<Mutex<Option<bool>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self { list: self.list.clone(), pkg_map: self.pkg_map.clone(), hard_coded_list_needs_updating: { let __guard = self.hard_coded_list_needs_updating.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        Self { list: Arc::new(Mutex::new(None)), pkg_map: Arc::new(Mutex::new(None)), hard_coded_list_needs_updating: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", format_slice(&self.list), format_map(&self.pkg_map), (*self.hard_coded_list_needs_updating.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct1 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub type Meta = AnonymousStruct1;


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for CovMetaBlob {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
