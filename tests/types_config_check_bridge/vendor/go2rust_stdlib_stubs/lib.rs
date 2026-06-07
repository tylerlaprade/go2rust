use std::any::Any;
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

pub fn go_any_clone(value: &(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync> {
    if let Some(v) = value.downcast_ref::<i32>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<i64>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<i8>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<i16>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<u32>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<u64>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<u8>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<u16>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<usize>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<isize>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<f64>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<f32>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<String>() { return Box::new(v.clone()) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<&'static str>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<bool>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<char>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }

    panic!("go_any_clone: unsupported dynamic type; add typed lowering instead of cloning Box<dyn Any>")
}

pub trait GoValueClone {
    fn go_value_clone(&self) -> Self;
}

macro_rules! impl_go_value_clone_copy {
    ($($t:ty),* $(,)?) => {
        $(impl GoValueClone for $t {
            fn go_value_clone(&self) -> Self { *self }
        })*
    };
}

impl_go_value_clone_copy!(bool, char, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, &'static str);

impl GoValueClone for String {
    fn go_value_clone(&self) -> Self { self.clone() }
}

impl GoValueClone for Box<dyn Any + Send + Sync> {
    fn go_value_clone(&self) -> Self { go_any_clone(self.as_ref()) }
}

pub trait GoComparable {
    fn go_eq(&self, other: &Self) -> bool;
    fn go_hash(&self, seed: usize) -> usize;
}

fn go_hash_value<T: std::hash::Hash>(value: &T, seed: usize) -> usize {
    let mut __hasher = std::collections::hash_map::DefaultHasher::new();
    std::hash::Hash::hash(&seed, &mut __hasher);
    std::hash::Hash::hash(value, &mut __hasher);
    std::hash::Hasher::finish(&__hasher) as usize
}

macro_rules! impl_go_comparable_hash {
    ($($t:ty),* $(,)?) => {
        $(impl GoComparable for $t {
            fn go_eq(&self, other: &Self) -> bool { self == other }
            fn go_hash(&self, seed: usize) -> usize { go_hash_value(self, seed) }
        })*
    };
}

impl_go_comparable_hash!(bool, char, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, String, &'static str);

impl GoComparable for f32 {
    fn go_eq(&self, other: &Self) -> bool { self == other }
    fn go_hash(&self, seed: usize) -> usize { go_hash_value(&self.to_bits(), seed) }
}

impl GoComparable for f64 {
    fn go_eq(&self, other: &Self) -> bool { self == other }
    fn go_hash(&self, seed: usize) -> usize { go_hash_value(&self.to_bits(), seed) }
}

fn go_any_comparable_eq(left: &(dyn Any + Send + Sync), right: &(dyn Any + Send + Sync)) -> bool {
    if left.type_id() != right.type_id() {
        return false;
    }
    if let Some(v) = left.downcast_ref::<i32>() { return right.downcast_ref::<i32>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<i64>() { return right.downcast_ref::<i64>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<i8>() { return right.downcast_ref::<i8>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<i16>() { return right.downcast_ref::<i16>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<u32>() { return right.downcast_ref::<u32>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<u64>() { return right.downcast_ref::<u64>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<u8>() { return right.downcast_ref::<u8>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<u16>() { return right.downcast_ref::<u16>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<usize>() { return right.downcast_ref::<usize>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<isize>() { return right.downcast_ref::<isize>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<f64>() { return right.downcast_ref::<f64>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<f32>() { return right.downcast_ref::<f32>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<String>() { return right.downcast_ref::<String>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<&str>() { return right.downcast_ref::<&str>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<bool>() { return right.downcast_ref::<bool>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<char>() { return right.downcast_ref::<char>().map_or(false, |r| v == r); }
    panic!("interface comparison with uncomparable dynamic type")
}

fn go_any_comparable_hash(value: &(dyn Any + Send + Sync), seed: usize) -> usize {
    if let Some(v) = value.downcast_ref::<i32>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<i64>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<i8>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<i16>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<u32>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<u64>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<u8>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<u16>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<usize>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<isize>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<f64>() { return go_hash_value(&(value.type_id(), v.to_bits()), seed); }
    if let Some(v) = value.downcast_ref::<f32>() { return go_hash_value(&(value.type_id(), v.to_bits()), seed); }
    if let Some(v) = value.downcast_ref::<String>() { return go_hash_value(&(value.type_id(), v), seed); }
    if let Some(v) = value.downcast_ref::<&str>() { return go_hash_value(&(value.type_id(), v), seed); }
    if let Some(v) = value.downcast_ref::<bool>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<char>() { return go_hash_value(&(value.type_id(), *v), seed); }
    panic!("interface hash with uncomparable dynamic type")
}

impl GoComparable for Box<dyn Any + Send + Sync> {
    fn go_eq(&self, other: &Self) -> bool { go_any_comparable_eq(self.as_ref(), other.as_ref()) }
    fn go_hash(&self, seed: usize) -> usize { go_any_comparable_hash(self.as_ref(), seed) }
}

#[derive(Clone, Copy)]
pub struct GoAnyTypeMetadata {
    pub kind: &'static str,
    pub comparable: bool,
    pub elem_kind: Option<&'static str>,
    pub elem_comparable: bool,
}

pub struct GoAnyMetadataBox {
    pub value: Box<dyn Any + Send + Sync>,
    pub metadata: GoAnyTypeMetadata,
}

fn go_any_type_metadata_registry() -> &'static std::sync::Mutex<std::collections::HashMap<std::any::TypeId, GoAnyTypeMetadata>> {
    static REGISTRY: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<std::any::TypeId, GoAnyTypeMetadata>>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

fn go_any_value_metadata_registry() -> &'static std::sync::Mutex<std::collections::HashMap<usize, GoAnyTypeMetadata>> {
    static REGISTRY: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<usize, GoAnyTypeMetadata>>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

fn go_any_value_metadata_key(value: &(dyn Any + Send + Sync)) -> usize {
    value as *const (dyn Any + Send + Sync) as *const () as usize
}

pub fn go_register_any_type<T: Any + Send + Sync + 'static>(kind: &'static str, comparable: bool) {
    go_any_type_metadata_registry().lock().unwrap().insert(std::any::TypeId::of::<T>(), GoAnyTypeMetadata { kind, comparable, elem_kind: None, elem_comparable: false });
}

pub fn go_register_any_type_with_elem<T: Any + Send + Sync + 'static>(kind: &'static str, comparable: bool, elem_kind: &'static str, elem_comparable: bool) {
    go_any_type_metadata_registry().lock().unwrap().insert(std::any::TypeId::of::<T>(), GoAnyTypeMetadata { kind, comparable, elem_kind: Some(elem_kind), elem_comparable });
}

pub fn go_box_any_with_metadata<T: Any + Send + Sync + 'static>(value: T, kind: &'static str, comparable: bool) -> Box<dyn Any + Send + Sync> {
    let metadata = GoAnyTypeMetadata { kind, comparable, elem_kind: None, elem_comparable: false };
    Box::new(GoAnyMetadataBox { value: Box::new(value) as Box<dyn Any + Send + Sync>, metadata }) as Box<dyn Any + Send + Sync>
}

pub fn go_register_any_value_metadata(value: &(dyn Any + Send + Sync), kind: &'static str, comparable: bool) {
    go_any_value_metadata_registry().lock().unwrap().insert(go_any_value_metadata_key(value), GoAnyTypeMetadata { kind, comparable, elem_kind: None, elem_comparable: false });
}

pub fn go_any_type_metadata(value: &(dyn Any + Send + Sync)) -> Option<GoAnyTypeMetadata> {
    if let Some(__boxed) = value.downcast_ref::<GoAnyMetadataBox>() {
        return Some(__boxed.metadata);
    }
    go_any_value_metadata_registry().lock().unwrap().get(&go_any_value_metadata_key(value)).copied()
        .or_else(|| go_any_type_metadata_registry().lock().unwrap().get(&value.type_id()).copied())
}


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

impl GoJsonInputArg for bytes_Buffer {
    fn into_go_json_bytes(self) -> Vec<u8> {
        self.__go_bytes()
    }
}



#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct bisect_Matcher;

impl std::fmt::Display for bisect_Matcher {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<bisect_Matcher>")
    }
}


impl bisect_Matcher {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn stack<T0>(&self, _arg0: T0) -> bool {
        panic!("bisect_Matcher.stack bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone)]
pub struct bytes_Buffer {
    pub __go_data: std::sync::Arc<std::sync::Mutex<Vec<u8>>>,
}

impl Default for bytes_Buffer {
    fn default() -> Self {
        Self { __go_data: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())) }
    }
}

impl std::fmt::Display for bytes_Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.__go_string())
    }
}

impl bytes_Buffer {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }

    pub fn __go_from_string(value: String) -> Self {
        Self { __go_data: std::sync::Arc::new(std::sync::Mutex::new(value.into_bytes())) }
    }

    pub fn __go_write_bytes(&self, data: &[u8]) {
        self.__go_data.lock().unwrap().extend_from_slice(data);
    }

    pub fn __go_bytes(&self) -> Vec<u8> {
        self.__go_data.lock().unwrap().clone()
    }

    pub fn __go_string(&self) -> String {
        String::from_utf8_lossy(&self.__go_data.lock().unwrap()).into_owned()
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some::<String>(self.__go_string())))
    }

    pub fn bytes(&self) -> Arc<Mutex<Option<Vec<u8>>>> {
        Arc::new(Mutex::new(Some::<Vec<u8>>(self.__go_bytes())))
    }

    pub fn len(&self) -> i32 {
        self.__go_data.lock().unwrap().len() as i32
    }

    pub fn reset(&self) {
        self.__go_data.lock().unwrap().clear();
    }

    pub fn available(&self) -> i32 {
        self.len()
    }

    pub fn available_buffer(&self) -> Arc<Mutex<Option<Vec<u8>>>> {
        Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::new())))
    }

    pub fn cap(&self) -> i32 {
        self.len()
    }

    pub fn grow<T0>(&self, _arg0: T0) {
    }

    pub fn next<T0>(&self, _arg0: T0) -> Arc<Mutex<Option<Vec<u8>>>> {
        Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::new())))
    }

    pub fn read<T0>(&self, _arg0: T0) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        (0 as i32, Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }

    pub fn read_byte(&self) -> (u8, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        (0 as u8, Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }

    pub fn read_bytes<T0>(&self, _arg0: T0) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        (Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::new()))), Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }

    pub fn read_from<T0>(&self, _arg0: T0) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        (0 as i64, Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }

    pub fn read_rune(&self) -> (i32, i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        (0 as i32, 0 as i32, Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }

    pub fn read_string<T0>(&self, _arg0: T0) -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        (Arc::new(Mutex::new(Some::<String>(String::new()))), Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }

    pub fn truncate<T0>(&self, _arg0: T0) {
        self.reset();
    }

    pub fn unread_byte(&self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))
    }

    pub fn unread_rune(&self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))
    }

    pub fn write<T0: 'static>(&self, arg0: T0) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let bytes = if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Vec<u8>>() {
            v.clone()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Arc<Mutex<Option<Vec<u8>>>>>() {
            v.lock().unwrap().as_ref().cloned().unwrap_or_default()
        } else {
            Vec::new()
        };
        let n = bytes.len() as i32;
        self.__go_write_bytes(&bytes);
        (n, Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }

    pub fn write_string<T0: 'static>(&self, arg0: T0) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let value = if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<String>() {
            v.clone()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<&str>() {
            (*v).to_string()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Arc<Mutex<Option<String>>>>() {
            v.lock().unwrap().as_ref().cloned().unwrap_or_default()
        } else {
            String::new()
        };
        let bytes = value.into_bytes();
        let n = bytes.len() as i32;
        self.__go_write_bytes(&bytes);
        (n, Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }

    pub fn write_byte<T0: 'static>(&self, arg0: T0) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let value = if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<u8>() {
            *v
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<i32>() {
            *v as u8
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Arc<Mutex<Option<u8>>>>() {
            v.lock().unwrap().as_ref().copied().unwrap_or_default()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Arc<Mutex<Option<i32>>>>() {
            v.lock().unwrap().as_ref().copied().unwrap_or_default() as u8
        } else {
            0
        };
        self.__go_write_bytes(&[value]);
        Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))
    }

    pub fn write_rune<T0: 'static>(&self, arg0: T0) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let value = if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<char>() {
            *v
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<i32>() {
            char::from_u32(*v as u32).unwrap_or('\0')
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Arc<Mutex<Option<i32>>>>() {
            char::from_u32(v.lock().unwrap().as_ref().copied().unwrap_or_default() as u32).unwrap_or('\0')
        } else {
            '\0'
        };
        let mut encoded = [0u8; 4];
        let bytes = value.encode_utf8(&mut encoded).as_bytes().to_vec();
        let n = bytes.len() as i32;
        self.__go_write_bytes(&bytes);
        (n, Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }

    pub fn write_to<T0>(&self, _arg0: T0) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        (self.__go_data.lock().unwrap().len() as i64, Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct bytes_Reader;

impl std::fmt::Display for bytes_Reader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<bytes_Reader>")
    }
}


impl bytes_Reader {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Clone)]
pub struct constraint_Expr {
    pub __go_id: usize,
    pub __go_value: Arc<dyn std::any::Any + Send + Sync>,
}

impl constraint_Expr {
    pub fn __go_from<T: 'static + Send + Sync>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_value: Arc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
}

impl Default for constraint_Expr {
    fn default() -> Self {
        Self { __go_id: 0, __go_value: Arc::new(()) }
    }
}

impl std::fmt::Debug for constraint_Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<constraint_Expr>")
    }
}

impl std::fmt::Display for constraint_Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<constraint_Expr>")
    }
}

impl PartialEq for constraint_Expr {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for constraint_Expr {}

impl PartialOrd for constraint_Expr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for constraint_Expr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct errors_errorString;

impl std::fmt::Display for errors_errorString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<errors_errorString>")
    }
}

impl std::error::Error for errors_errorString {}


impl errors_errorString {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        panic!("errors_errorString.error bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct errors_joinError;

impl std::fmt::Display for errors_joinError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<errors_joinError>")
    }
}

impl std::error::Error for errors_joinError {}


impl errors_joinError {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        panic!("errors_joinError.error bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Clone)]
pub struct fmt_ScanState {
    pub __go_id: usize,
    pub __go_value: Arc<dyn std::any::Any + Send + Sync>,
}

impl fmt_ScanState {
    pub fn __go_from<T: 'static + Send + Sync>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_value: Arc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
    pub fn read<T0>(&self, _arg0: T0) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        panic!("fmt_ScanState.read bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
    pub fn read_rune(&self) -> (i32, i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        panic!("fmt_ScanState.read_rune bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
    pub fn skip_space(&self) {
        panic!("fmt_ScanState.skip_space bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
    pub fn token<T0, T1>(&self, _arg0: T0, _arg1: T1) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        panic!("fmt_ScanState.token bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
    pub fn unread_rune(&self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        panic!("fmt_ScanState.unread_rune bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
    pub fn width(&self) -> (i32, bool) {
        panic!("fmt_ScanState.width bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}

impl Default for fmt_ScanState {
    fn default() -> Self {
        Self { __go_id: 0, __go_value: Arc::new(()) }
    }
}

impl std::fmt::Debug for fmt_ScanState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<fmt_ScanState>")
    }
}

impl std::fmt::Display for fmt_ScanState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<fmt_ScanState>")
    }
}

impl PartialEq for fmt_ScanState {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for fmt_ScanState {}

impl PartialOrd for fmt_ScanState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for fmt_ScanState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
    }
}


#[derive(Clone)]
pub struct fmt_State {
    pub __go_id: usize,
    pub __go_value: Arc<dyn std::any::Any + Send + Sync>,
}

impl fmt_State {
    pub fn __go_from<T: 'static + Send + Sync>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_value: Arc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
    pub fn flag<T0>(&self, _arg0: T0) -> bool {
        panic!("fmt_State.flag bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
    pub fn precision(&self) -> (i32, bool) {
        panic!("fmt_State.precision bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
    pub fn width(&self) -> (i32, bool) {
        panic!("fmt_State.width bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
    pub fn write<T0>(&self, _arg0: T0) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        panic!("fmt_State.write bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}

impl Default for fmt_State {
    fn default() -> Self {
        Self { __go_id: 0, __go_value: Arc::new(()) }
    }
}

impl std::fmt::Debug for fmt_State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<fmt_State>")
    }
}

impl std::fmt::Display for fmt_State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<fmt_State>")
    }
}

impl PartialEq for fmt_State {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for fmt_State {}

impl PartialOrd for fmt_State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for fmt_State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct fmt_wrapError;

impl std::fmt::Display for fmt_wrapError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<fmt_wrapError>")
    }
}

impl std::error::Error for fmt_wrapError {}


impl fmt_wrapError {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        panic!("fmt_wrapError.error bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct fmt_wrapErrors;

impl std::fmt::Display for fmt_wrapErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<fmt_wrapErrors>")
    }
}

impl std::error::Error for fmt_wrapErrors {}


impl fmt_wrapErrors {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        panic!("fmt_wrapErrors.error bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct fs_FileInfo {
    pub name: String,
    pub is_dir: bool,
    pub size: i64,
}

impl std::fmt::Display for fs_FileInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<fs_FileInfo>")
    }
}


impl fs_FileInfo {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some::<String>(self.name.clone())))
    }
    pub fn size(&self) -> i64 {
        self.size
    }
    pub fn is_dir(&self) -> bool {
        self.is_dir
    }
}


#[derive(Clone)]
pub struct io_ByteScanner {
    pub __go_id: usize,
    pub __go_value: Arc<dyn std::any::Any + Send + Sync>,
}

impl io_ByteScanner {
    pub fn __go_from<T: 'static + Send + Sync>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_value: Arc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
    pub fn read_byte(&self) -> (u8, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        panic!("io_ByteScanner.read_byte bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
    pub fn unread_byte(&self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        panic!("io_ByteScanner.unread_byte bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}

impl Default for io_ByteScanner {
    fn default() -> Self {
        Self { __go_id: 0, __go_value: Arc::new(()) }
    }
}

impl std::fmt::Debug for io_ByteScanner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<io_ByteScanner>")
    }
}

impl std::fmt::Display for io_ByteScanner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<io_ByteScanner>")
    }
}

impl PartialEq for io_ByteScanner {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for io_ByteScanner {}

impl PartialOrd for io_ByteScanner {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for io_ByteScanner {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
    }
}


#[derive(Clone)]
pub struct io_Reader {
    pub __go_id: usize,
    pub __go_value: Arc<dyn std::any::Any + Send + Sync>,
}

impl io_Reader {
    pub fn __go_from<T: 'static + Send + Sync>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_value: Arc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
}

impl Default for io_Reader {
    fn default() -> Self {
        Self { __go_id: 0, __go_value: Arc::new(()) }
    }
}

impl std::fmt::Debug for io_Reader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<io_Reader>")
    }
}

impl std::fmt::Display for io_Reader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<io_Reader>")
    }
}

impl PartialEq for io_Reader {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for io_Reader {}

impl PartialOrd for io_Reader {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for io_Reader {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
    }
}


#[derive(Clone)]
pub struct io_Writer {
    pub __go_id: usize,
    pub __go_value: Arc<dyn std::any::Any + Send + Sync>,
}

impl io_Writer {
    pub fn __go_from<T: 'static + Send + Sync>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_value: Arc::new(value) }
    }

    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }

    pub fn __go_write_bytes(&self, data: &[u8]) {
        if let Some(buffer) = self.downcast_ref::<bytes_Buffer>() {
            buffer.__go_write_bytes(data);
        }
        if let Some(file) = self.downcast_ref::<os_File>() {
            file.__go_write_bytes(data);
        }
        if let Some(builder) = self.downcast_ref::<Arc<Mutex<Option<String>>>>() {
            let mut guard = builder.lock().unwrap();
            guard.get_or_insert_with(String::new).push_str(&String::from_utf8_lossy(data));
        }
    }

    pub fn write<T0: 'static>(&self, arg0: T0) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let bytes = if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Vec<u8>>() {
            v.clone()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Arc<Mutex<Option<Vec<u8>>>>>() {
            v.lock().unwrap().as_ref().cloned().unwrap_or_default()
        } else {
            Vec::new()
        };
        let n = bytes.len() as i32;
        self.__go_write_bytes(&bytes);
        (n, Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }
}

impl Default for io_Writer {
    fn default() -> Self {
        Self { __go_id: 0, __go_value: Arc::new(()) }
    }
}

impl std::fmt::Debug for io_Writer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<io_Writer>")
    }
}

impl std::fmt::Display for io_Writer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<io_Writer>")
    }
}

impl PartialEq for io_Writer {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for io_Writer {}

impl PartialOrd for io_Writer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for io_Writer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
    }
}


#[derive(Debug, Clone)]
pub struct os_File {
    pub __go_data: std::sync::Arc<std::sync::Mutex<Vec<u8>>>,
    pub __go_closed: std::sync::Arc<std::sync::atomic::AtomicBool>,
    pub __go_wait_for_close: bool,
}

impl Default for os_File {
    fn default() -> Self {
        Self {
            __go_data: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
            __go_closed: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            __go_wait_for_close: false,
        }
    }
}

impl std::fmt::Display for os_File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<os_File>")
    }
}

impl PartialEq for os_File {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__go_data, &other.__go_data)
    }
}

impl Eq for os_File {}

impl os_File {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }

    pub fn __go_write_bytes(&self, data: &[u8]) {
        self.__go_data.lock().unwrap().extend_from_slice(data);
    }

    pub fn __go_read_all(&self) -> Vec<u8> {
        self.__go_data.lock().unwrap().clone()
    }

    pub fn __go_read_all_for_copy(&self) -> Vec<u8> {
        while self.__go_wait_for_close && !self.__go_closed.load(std::sync::atomic::Ordering::SeqCst) {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        self.__go_read_all()
    }

    pub fn close(&self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        self.__go_closed.store(true, std::sync::atomic::Ordering::SeqCst);
        Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))
    }

    pub fn write<T0: 'static>(&self, arg0: T0) -> (Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let bytes = if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Vec<u8>>() {
            v.clone()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Arc<Mutex<Option<Vec<u8>>>>>() {
            v.lock().unwrap().as_ref().cloned().unwrap_or_default()
        } else {
            Vec::new()
        };
        let n = bytes.len() as i32;
        self.__go_write_bytes(&bytes);
        (Arc::new(Mutex::new(Some::<i32>(n))), Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }

    pub fn write_string<T0: 'static>(&self, arg0: T0) -> (Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let value = if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<String>() {
            v.clone()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<&str>() {
            (*v).to_string()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Arc<Mutex<Option<String>>>>() {
            v.lock().unwrap().as_ref().cloned().unwrap_or_default()
        } else {
            String::new()
        };
        let bytes = value.into_bytes();
        let n = bytes.len() as i32;
        self.__go_write_bytes(&bytes);
        (Arc::new(Mutex::new(Some::<i32>(n))), Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }

    pub fn read<T0>(&self, _arg0: T0) -> (Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        (Arc::new(Mutex::new(Some::<i32>(0))), Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }

    pub fn read_at<T0: 'static, T1: 'static>(&self, arg0: T0, arg1: T1) -> (Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let offset = if let Some(v) = (&arg1 as &dyn std::any::Any).downcast_ref::<i64>() {
            *v
        } else if let Some(v) = (&arg1 as &dyn std::any::Any).downcast_ref::<Arc<Mutex<Option<i64>>>>() {
            v.lock().unwrap().as_ref().copied().unwrap_or_default()
        } else {
            0
        };
        let data = self.__go_read_all();
        let mut n = 0i32;
        if offset >= 0 {
            let start = offset as usize;
            if start < data.len() {
                if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Arc<Mutex<Option<Vec<u8>>>>>() {
                    let mut guard = v.lock().unwrap();
                    if let Some(target) = guard.as_mut() {
                        let count = std::cmp::min(target.len(), data.len() - start);
                        target[..count].copy_from_slice(&data[start..start + count]);
                        n = count as i32;
                    }
                }
            }
        }
        (Arc::new(Mutex::new(Some::<i32>(n))), Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct rand_Rand;

impl std::fmt::Display for rand_Rand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<rand_Rand>")
    }
}


impl rand_Rand {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn uint32(&self) -> u32 {
        panic!("rand_Rand.uint32 bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Clone)]
pub struct rand_Source {
    pub __go_id: usize,
    pub __go_value: Arc<dyn std::any::Any + Send + Sync>,
}

impl rand_Source {
    pub fn __go_from<T: 'static + Send + Sync>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_value: Arc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
}

impl Default for rand_Source {
    fn default() -> Self {
        Self { __go_id: 0, __go_value: Arc::new(()) }
    }
}

impl std::fmt::Debug for rand_Source {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<rand_Source>")
    }
}

impl std::fmt::Display for rand_Source {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<rand_Source>")
    }
}

impl PartialEq for rand_Source {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for rand_Source {}

impl PartialOrd for rand_Source {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for rand_Source {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
    }
}


#[derive(Clone)]
pub struct reflect_Type {
    pub __go_id: usize,
    pub __go_value: Arc<dyn std::any::Any + Send + Sync>,
}

impl reflect_Type {
    pub fn __go_from<T: 'static + Send + Sync>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_value: Arc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
}

impl Default for reflect_Type {
    fn default() -> Self {
        Self { __go_id: 0, __go_value: Arc::new(()) }
    }
}

impl std::fmt::Debug for reflect_Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<reflect_Type>")
    }
}

impl std::fmt::Display for reflect_Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<reflect_Type>")
    }
}

impl PartialEq for reflect_Type {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for reflect_Type {}

impl PartialOrd for reflect_Type {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for reflect_Type {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct reflect_Value;

impl std::fmt::Display for reflect_Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<reflect_Value>")
    }
}


impl reflect_Value {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn elem(&self) -> Arc<Mutex<Option<reflect_Value>>> {
        panic!("reflect_Value.elem bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
    pub fn field<T0>(&self, _arg0: T0) -> Arc<Mutex<Option<reflect_Value>>> {
        panic!("reflect_Value.field bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
    pub fn r#type(&self) -> Arc<Mutex<Option<reflect_Type>>> {
        panic!("reflect_Value.r#type bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct reflectlite_Value;

impl std::fmt::Display for reflectlite_Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<reflectlite_Value>")
    }
}


impl reflectlite_Value {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn len(&self) -> i32 {
        panic!("reflectlite_Value.len bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct runtime_PanicNilError;

impl std::fmt::Display for runtime_PanicNilError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<runtime_PanicNilError>")
    }
}

impl std::error::Error for runtime_PanicNilError {}


impl runtime_PanicNilError {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        panic!("runtime_PanicNilError.error bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct runtime_TypeAssertionError;

impl std::fmt::Display for runtime_TypeAssertionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<runtime_TypeAssertionError>")
    }
}

impl std::error::Error for runtime_TypeAssertionError {}


impl runtime_TypeAssertionError {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        panic!("runtime_TypeAssertionError.error bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct runtime_boundsError;

impl std::fmt::Display for runtime_boundsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<runtime_boundsError>")
    }
}

impl std::error::Error for runtime_boundsError {}


impl runtime_boundsError {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        panic!("runtime_boundsError.error bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct runtime_errorAddressString;

impl std::fmt::Display for runtime_errorAddressString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<runtime_errorAddressString>")
    }
}

impl std::error::Error for runtime_errorAddressString {}


impl runtime_errorAddressString {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        panic!("runtime_errorAddressString.error bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct runtime_errorString;

impl std::fmt::Display for runtime_errorString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<runtime_errorString>")
    }
}

impl std::error::Error for runtime_errorString {}


impl runtime_errorString {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        panic!("runtime_errorString.error bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct runtime_plainError;

impl std::fmt::Display for runtime_plainError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<runtime_plainError>")
    }
}

impl std::error::Error for runtime_plainError {}


impl runtime_plainError {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        panic!("runtime_plainError.error bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


impl From<bytes_Reader> for io_ByteScanner {
    fn from(_value: bytes_Reader) -> Self {
        Self::__go_from(_value)
    }
}


pub mod bisect {
    use super::*;
    pub fn new<T0>(_arg0: T0) -> (Arc<Mutex<Option<bisect_Matcher>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        panic!("new bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


pub mod byteorder {
    use super::*;
    pub fn b_e_put_uint32<T0, T1>(_arg0: T0, _arg1: T1) {
        panic!("b_e_put_uint32 bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn b_e_uint32<T0>(_arg0: T0) -> u32 {
        panic!("b_e_uint32 bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn b_e_uint64<T0>(_arg0: T0) -> u64 {
        panic!("b_e_uint64 bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


pub mod bytes {
    use super::*;
    pub fn has_prefix<T0, T1>(_arg0: T0, _arg1: T1) -> bool {
        panic!("has_prefix bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn last_index_byte<T0, T1>(_arg0: T0, _arg1: T1) -> i32 {
        panic!("last_index_byte bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn new_buffer_string<T0>(_arg0: T0) -> Arc<Mutex<Option<bytes_Buffer>>> {
        panic!("new_buffer_string bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn new_reader<T0>(_arg0: T0) -> Arc<Mutex<Option<bytes_Reader>>> {
        panic!("new_reader bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn trim_right<T0, T1>(_arg0: T0, _arg1: T1) -> Arc<Mutex<Option<Vec<u8>>>> {
        panic!("trim_right bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


pub mod constraint {
    use super::*;
    pub fn go_version<T0>(_arg0: T0) -> Arc<Mutex<Option<String>>> {
        panic!("go_version bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn parse<T0>(_arg0: T0) -> (Arc<Mutex<Option<constraint_Expr>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        panic!("parse bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


pub mod fmt {
    use super::*;
    pub fn fprint<T0, T1>(_arg0: T0, _arg1: T1) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        panic!("fprint bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


pub mod fs {
    use super::*;
    pub fn SkipAll() -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))
    }

    pub fn SkipDir() -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))
    }
}


pub mod goarch {
    use super::*;
    pub const PTR_SIZE: i32 = 8;
}


pub mod io {
    use super::*;
    pub const SEEK_CURRENT: i32 = 1;
    pub const SEEK_END: i32 = 2;
    pub const SEEK_START: i32 = 0;

    pub fn EOF() -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))
    }

    pub fn ErrShortWrite() -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))
    }

    pub fn read_all<T0>(_arg0: T0) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        panic!("read_all bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn write_string<T0, T1>(_arg0: T0, _arg1: T1) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        panic!("write_string bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


pub mod os {
    use super::*;
    use std::path::Path;

    pub trait GoStringArg {
        fn into_go_string(self) -> String;
    }

    impl GoStringArg for String {
        fn into_go_string(self) -> String {
            self
        }
    }

    impl<'a> GoStringArg for &'a str {
        fn into_go_string(self) -> String {
            self.to_string()
        }
    }

    impl<'a> GoStringArg for &'a String {
        fn into_go_string(self) -> String {
            self.clone()
        }
    }

    impl GoStringArg for Arc<Mutex<Option<String>>> {
        fn into_go_string(self) -> String {
            self.lock().unwrap().as_ref().cloned().unwrap_or_default()
        }
    }

    type GoError = Arc<Mutex<Option<Box<dyn std::error::Error + Send + Sync>>>>;

    fn no_error() -> GoError {
        Arc::new(Mutex::new(None))
    }

    fn io_error(err: std::io::Error) -> GoError {
        Arc::new(Mutex::new(Some(Box::new(err))))
    }

    pub const PATH_LIST_SEPARATOR: i32 = 58;
    pub const PATH_SEPARATOR: i32 = 47;

    pub fn getenv<T0: 'static>(_arg0: T0) -> Arc<Mutex<Option<String>>> {
        let key = if let Some(v) = (&_arg0 as &dyn Any).downcast_ref::<String>() {
            v.clone()
        } else if let Some(v) = (&_arg0 as &dyn Any).downcast_ref::<Arc<Mutex<Option<String>>>>() {
            v.lock().unwrap().as_ref().cloned().unwrap_or_default()
        } else {
            panic!("os.Getenv bridge: expected string argument")
        };
        Arc::new(Mutex::new(Some::<String>(std::env::var(key).unwrap_or_default())))
    }

    pub fn lstat<T0: GoStringArg>(_arg0: T0) -> (Arc<Mutex<Option<fs_FileInfo>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let path = _arg0.into_go_string();
        match std::fs::symlink_metadata(&path) {
            Ok(metadata) => {
                let name = Path::new(&path).file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_else(|| path.clone());
                (Arc::new(Mutex::new(Some::<fs_FileInfo>(fs_FileInfo { name, is_dir: metadata.is_dir(), size: metadata.len() as i64 }))), no_error())
            }
            Err(err) => (Arc::new(Mutex::new(Some::<fs_FileInfo>(fs_FileInfo::default()))), io_error(err)),
        }
    }

    pub fn read_file<T0: GoStringArg>(_arg0: T0) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let path = _arg0.into_go_string();
        match std::fs::read(&path) {
            Ok(data) => (Arc::new(Mutex::new(Some::<Vec<u8>>(data))), no_error()),
            Err(err) => (Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::new()))), io_error(err)),
        }
    }
}


pub mod rand {
    use super::*;
    pub fn new<T0>(_arg0: T0) -> Arc<Mutex<Option<rand_Rand>>> {
        panic!("new bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn new_source<T0>(_arg0: T0) -> Arc<Mutex<Option<rand_Source>>> {
        panic!("new_source bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


pub mod reflectlite {
    use super::*;
    pub fn swapper<T0>(_arg0: T0) -> Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> () + Send + Sync>>>> {
        panic!("swapper bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn value_of<T0>(_arg0: T0) -> Arc<Mutex<Option<reflectlite_Value>>> {
        panic!("value_of bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


pub mod runtime {
    use super::*;
    pub fn caller<T0>(_arg0: T0) -> (usize, Arc<Mutex<Option<String>>>, i32, bool) {
        panic!("caller bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn g_o_m_a_x_p_r_o_c_s<T0>(_arg0: T0) -> i32 {
        std::thread::available_parallelism().map(|n| n.get() as i32).unwrap_or(1).max(1)
    }
}
