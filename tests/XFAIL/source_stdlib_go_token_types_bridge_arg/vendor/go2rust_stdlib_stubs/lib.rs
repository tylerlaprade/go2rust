use std::any::Any;
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

pub struct GoPtrKey<T>(pub Arc<Mutex<Option<T>>>);

impl<T> Clone for GoPtrKey<T> {
    fn clone(&self) -> Self { GoPtrKey(self.0.clone()) }
}

impl<T> GoPtrKey<T> {
    pub fn new(value: Arc<Mutex<Option<T>>>) -> Self { GoPtrKey(value) }
    pub fn value(&self) -> Arc<Mutex<Option<T>>> { self.0.clone() }
    fn addr(&self) -> usize { Arc::as_ptr(&self.0) as usize }
}

impl<T> PartialEq for GoPtrKey<T> {
    fn eq(&self, other: &Self) -> bool { self.addr() == other.addr() }
}
impl<T> Eq for GoPtrKey<T> {}
impl<T> PartialOrd for GoPtrKey<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl<T> Ord for GoPtrKey<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.addr().cmp(&other.addr()) }
}
impl<T> std::fmt::Debug for GoPtrKey<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0x{:x}", self.addr()) }
}
impl<T> std::fmt::Display for GoPtrKey<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0x{:x}", self.addr()) }
}

fn __go_next_external_interface_id() -> usize {
    static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

pub use serde_json;

pub trait GoJsonInputArg {
    fn into_go_json_bytes(self) -> Vec<u8>;
}

pub trait GoJsonDecode: Sized {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String>;
}

pub trait GoJsonDecodeTarget {
    fn assign_go_json(self, value: &serde_json::Value) -> Result<(), String>;
}

fn go_json_no_error() -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    Arc::new(Mutex::new(None))
}

fn go_json_error(message: String) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(message))))
}

pub fn go_json_expected(value: &serde_json::Value, want: &str) -> String {
    format!("expected {}, got {}", want, value)
}

impl GoJsonInputArg for Vec<u8> {
    fn into_go_json_bytes(self) -> Vec<u8> {
        self
    }
}

impl GoJsonInputArg for String {
    fn into_go_json_bytes(self) -> Vec<u8> {
        self.into_bytes()
    }
}

impl<'a> GoJsonInputArg for &'a str {
    fn into_go_json_bytes(self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl<T> GoJsonInputArg for Arc<Mutex<Option<T>>>
where
    T: GoJsonInputArg + Clone,
{
    fn into_go_json_bytes(self) -> Vec<u8> {
        self.lock().unwrap().as_ref().cloned().map(|value| value.into_go_json_bytes()).unwrap_or_default()
    }
}

impl GoJsonDecode for String {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_str().map(|value| value.to_string()).ok_or_else(|| go_json_expected(value, "string"))
    }
}

impl GoJsonDecode for bool {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_bool().ok_or_else(|| go_json_expected(value, "bool"))
    }
}

impl GoJsonDecode for i32 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_i64().map(|value| value as i32).ok_or_else(|| go_json_expected(value, "integer"))
    }
}

impl GoJsonDecode for i16 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_i64().map(|value| value as i16).ok_or_else(|| go_json_expected(value, "integer"))
    }
}

impl GoJsonDecode for i8 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_i64().map(|value| value as i8).ok_or_else(|| go_json_expected(value, "integer"))
    }
}

impl GoJsonDecode for i64 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_i64().ok_or_else(|| go_json_expected(value, "integer"))
    }
}

impl GoJsonDecode for u8 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_u64().map(|value| value as u8).ok_or_else(|| go_json_expected(value, "integer"))
    }
}

impl GoJsonDecode for u16 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_u64().map(|value| value as u16).ok_or_else(|| go_json_expected(value, "integer"))
    }
}

impl GoJsonDecode for u32 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_u64().map(|value| value as u32).ok_or_else(|| go_json_expected(value, "integer"))
    }
}

impl GoJsonDecode for u64 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_u64().ok_or_else(|| go_json_expected(value, "integer"))
    }
}

impl GoJsonDecode for usize {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_u64().map(|value| value as usize).ok_or_else(|| go_json_expected(value, "integer"))
    }
}

impl GoJsonDecode for f64 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_f64().ok_or_else(|| go_json_expected(value, "number"))
    }
}

impl GoJsonDecode for f32 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_f64().map(|value| value as f32).ok_or_else(|| go_json_expected(value, "number"))
    }
}

impl<T> GoJsonDecode for Vec<T>
where
    T: GoJsonDecode,
{
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let array = value.as_array().ok_or_else(|| go_json_expected(value, "array"))?;
        array.iter().map(T::go_json_decode).collect()
    }
}

impl<T, const N: usize> GoJsonDecode for [T; N]
where
    T: GoJsonDecode + Default,
{
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let array = value.as_array().ok_or_else(|| go_json_expected(value, "array"))?;
        if array.len() != N {
            return Err(format!("expected array of length {}, got {}", N, array.len()));
        }
        let mut out = std::array::from_fn(|_| T::default());
        for (index, item) in array.iter().enumerate() {
            out[index] = T::go_json_decode(item)?;
        }
        Ok(out)
    }
}

impl<V> GoJsonDecode for BTreeMap<String, V>
where
    V: GoJsonDecode,
{
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = BTreeMap::new();
        for (key, value) in object {
            out.insert(key.clone(), V::go_json_decode(value)?);
        }
        Ok(out)
    }
}

impl<T> GoJsonDecode for Arc<Mutex<Option<T>>>
where
    T: GoJsonDecode,
{
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        if value.is_null() {
            Ok(Arc::new(Mutex::new(None)))
        } else {
            Ok(Arc::new(Mutex::new(Some(T::go_json_decode(value)?))))
        }
    }
}

impl<T> GoJsonDecodeTarget for Arc<Mutex<Option<T>>>
where
    T: GoJsonDecode,
{
    fn assign_go_json(self, value: &serde_json::Value) -> Result<(), String> {
        if value.is_null() {
            *self.lock().unwrap() = None;
        } else {
            *self.lock().unwrap() = Some(T::go_json_decode(value)?);
        }
        Ok(())
    }
}



#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct token_Pos(pub i32);

impl PartialEq<i32> for token_Pos {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<token_Pos> for i32 {
    fn eq(&self, other: &token_Pos) -> bool {
        *self == other.0
    }
}

impl std::ops::BitAnd for token_Pos {
    type Output = token_Pos;
    fn bitand(self, other: Self) -> token_Pos {
        token_Pos(self.0 & other.0)
    }
}

impl std::ops::BitOr for token_Pos {
    type Output = token_Pos;
    fn bitor(self, other: Self) -> token_Pos {
        token_Pos(self.0 | other.0)
    }
}

impl std::fmt::Display for token_Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<token_Pos>")
    }
}


impl token_Pos {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Package;

impl std::fmt::Display for types_Package {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Package>")
    }
}


impl types_Package {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Clone)]
pub struct types_Type {
    pub __go_id: usize,
    pub __go_value: Arc<dyn std::any::Any + Send + Sync>,
}

impl types_Type {
    pub fn __go_from<T: 'static + Send + Sync>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_value: Arc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
}

impl Default for types_Type {
    fn default() -> Self {
        Self { __go_id: 0, __go_value: Arc::new(()) }
    }
}

impl std::fmt::Debug for types_Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Type>")
    }
}

impl std::fmt::Display for types_Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Type>")
    }
}

impl PartialEq for types_Type {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for types_Type {}

impl PartialOrd for types_Type {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for types_Type {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
    }
}


pub trait GoTypesTokenPosArg {
    fn __go_into_token_pos_arg(self) -> token_Pos;
}

impl GoTypesTokenPosArg for token_Pos {
    fn __go_into_token_pos_arg(self) -> token_Pos { self }
}

impl GoTypesTokenPosArg for i32 {
    fn __go_into_token_pos_arg(self) -> token_Pos { token_Pos(self) }
}

impl GoTypesTokenPosArg for Arc<Mutex<Option<token_Pos>>> {
    fn __go_into_token_pos_arg(self) -> token_Pos {
        self.lock().unwrap().as_ref().copied().unwrap_or_default()
    }
}

pub trait GoTypesPackageArg {
    fn __go_into_types_package_arg(self) -> Arc<Mutex<Option<types_Package>>>;
}

impl GoTypesPackageArg for () {
    fn __go_into_types_package_arg(self) -> Arc<Mutex<Option<types_Package>>> {
        Arc::new(Mutex::new(None::<types_Package>))
    }
}

impl GoTypesPackageArg for Arc<Mutex<Option<types_Package>>> {
    fn __go_into_types_package_arg(self) -> Arc<Mutex<Option<types_Package>>> { self }
}

pub trait GoTypesOptionalTypeArg {
    fn __go_into_optional_types_type_arg(self) -> Arc<Mutex<Option<types_Type>>>;
}

impl GoTypesOptionalTypeArg for () {
    fn __go_into_optional_types_type_arg(self) -> Arc<Mutex<Option<types_Type>>> {
        Arc::new(Mutex::new(None::<types_Type>))
    }
}

impl GoTypesOptionalTypeArg for Arc<Mutex<Option<types_Type>>> {
    fn __go_into_optional_types_type_arg(self) -> Arc<Mutex<Option<types_Type>>> { self }
}

pub trait GoTypesStringArg {
    fn __go_into_string_arg(self) -> String;
}

impl GoTypesStringArg for String {
    fn __go_into_string_arg(self) -> String { self }
}

impl<'a> GoTypesStringArg for &'a str {
    fn __go_into_string_arg(self) -> String { self.to_string() }
}

impl<'a> GoTypesStringArg for &'a String {
    fn __go_into_string_arg(self) -> String { self.clone() }
}

impl GoTypesStringArg for Arc<Mutex<Option<String>>> {
    fn __go_into_string_arg(self) -> String {
        self.lock().unwrap().as_ref().cloned().unwrap_or_default()
    }
}

pub trait GoTypesTypeNameArg {
    fn __go_into_type_name_arg(self) -> Arc<Mutex<Option<types_TypeName>>>;
}

impl GoTypesTypeNameArg for () {
    fn __go_into_type_name_arg(self) -> Arc<Mutex<Option<types_TypeName>>> {
        Arc::new(Mutex::new(None::<types_TypeName>))
    }
}

impl GoTypesTypeNameArg for Arc<Mutex<Option<types_TypeName>>> {
    fn __go_into_type_name_arg(self) -> Arc<Mutex<Option<types_TypeName>>> { self }
}

#[derive(Debug, Clone, Default)]
pub struct types_TypeName {
    pub __go_pos: token_Pos,
    pub __go_pkg: Arc<Mutex<Option<types_Package>>>,
    pub __go_name: String,
    pub __go_type: Arc<Mutex<Option<types_Type>>>,
}

impl std::fmt::Display for types_TypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.__go_string())
    }
}

impl types_TypeName {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    fn __go_string(&self) -> String {
        let type_guard = self.__go_type.lock().unwrap();
        if let Some(typ) = type_guard.as_ref() {
            if typ.downcast_ref::<types_TypeParam>().is_some() {
                return format!("type parameter {} <nil>", self.__go_name);
            }
        }
        format!("type {}", self.__go_name)
    }
}


#[derive(Debug, Clone, Default)]
pub struct types_TypeParam {
    pub __go_obj: Arc<Mutex<Option<types_TypeName>>>,
    pub __go_constraint: Arc<Mutex<Option<types_Type>>>,
    pub __go_index: i32,
}

impl std::fmt::Display for types_TypeParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let obj_guard = self.__go_obj.lock().unwrap();
        if let Some(obj) = obj_guard.as_ref() {
            write!(f, "{}", obj.__go_name)
        } else {
            write!(f, "<types_TypeParam>")
        }
    }
}

impl types_TypeParam {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


pub mod types {
    use super::*;
    pub fn new_type_name<T0: GoTypesTokenPosArg, T1: GoTypesPackageArg, T2: GoTypesStringArg, T3: GoTypesOptionalTypeArg>(_arg0: T0, _arg1: T1, _arg2: T2, _arg3: T3) -> Arc<Mutex<Option<types_TypeName>>> {
        let value = types_TypeName { __go_pos: _arg0.__go_into_token_pos_arg(), __go_pkg: _arg1.__go_into_types_package_arg(), __go_name: _arg2.__go_into_string_arg(), __go_type: _arg3.__go_into_optional_types_type_arg() };
        Arc::new(Mutex::new(Some::<types_TypeName>(value)))
    }

    pub fn new_type_param<T0: GoTypesTypeNameArg, T1: GoTypesOptionalTypeArg>(_arg0: T0, _arg1: T1) -> Arc<Mutex<Option<types_TypeParam>>> {
        let obj = _arg0.__go_into_type_name_arg();
        let param = types_TypeParam { __go_obj: obj.clone(), __go_constraint: _arg1.__go_into_optional_types_type_arg(), __go_index: -1 };
        {
            let mut obj_guard = obj.lock().unwrap();
            if let Some(obj_value) = obj_guard.as_mut() {
                obj_value.__go_type = Arc::new(Mutex::new(Some::<types_Type>(types_Type::__go_from(param.clone()))));
            } else {
                panic!("types.NewTypeParam bridge: nil TypeName object")
            }
        }
        Arc::new(Mutex::new(Some::<types_TypeParam>(param)))
    }
}
