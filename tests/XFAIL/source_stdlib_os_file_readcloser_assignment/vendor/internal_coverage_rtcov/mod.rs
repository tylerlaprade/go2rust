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
        let __go_clone_0_0 = self.p.clone();
        let __go_clone_1_0 = { let __guard = self.len.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.hash.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.pkg_path.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.pkg_i_d.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.counter_mode.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_6_0 = { let __guard = self.counter_granularity.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            p: __go_clone_0_0,
            len: __go_clone_1_0,
            hash: __go_clone_2_0,
            pkg_path: __go_clone_3_0,
            pkg_i_d: __go_clone_4_0,
            counter_mode: __go_clone_5_0,
            counter_granularity: __go_clone_6_0,
        }
    }
}


impl Default for CovMetaBlob {
    fn default() -> Self {
        Self { p: Arc::new(Mutex::new(None)), len: Arc::new(Mutex::new(Some(0))), hash: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), pkg_path: Arc::new(Mutex::new(Some(String::new()))), pkg_i_d: Arc::new(Mutex::new(Some(0))), counter_mode: Arc::new(Mutex::new(Some(0))), counter_granularity: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for CovMetaBlob {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.p.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", (*self.len.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", format_slice(&self.hash));
        let __go_fmt_3 = format!("{}", (*self.pkg_path.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.pkg_i_d.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", (*self.counter_mode.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_6 = format!("{}", (*self.counter_granularity.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4, __go_fmt_5, __go_fmt_6)
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
        let __go_clone_0_0 = self.list.clone();
        let __go_clone_1_0 = self.pkg_map.clone();
        let __go_clone_2_0 = { let __guard = self.hard_coded_list_needs_updating.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            list: __go_clone_0_0,
            pkg_map: __go_clone_1_0,
            hard_coded_list_needs_updating: __go_clone_2_0,
        }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        Self { list: Arc::new(Mutex::new(None)), pkg_map: Arc::new(Mutex::new(None)), hard_coded_list_needs_updating: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_slice(&self.list));
        let __go_fmt_1 = format!("{}", format_map(&self.pkg_map));
        let __go_fmt_2 = format!("{}", (*self.hard_coded_list_needs_updating.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
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
