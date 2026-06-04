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



#[derive(Debug, Clone, Default)]
pub struct ast_ArrayType {
    pub elt: Arc<Mutex<Option<ast_Expr>>>,
    pub len: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_ArrayType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_ArrayType>")
    }
}


impl ast_ArrayType {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_AssignStmt {
    pub lhs: Arc<Mutex<Option<Vec<ast_Expr>>>>,
    pub rhs: Arc<Mutex<Option<Vec<ast_Expr>>>>,
    pub tok: Arc<Mutex<Option<token_Token>>>,
    pub tok_pos: Arc<Mutex<Option<token_Pos>>>,
}

impl std::fmt::Display for ast_AssignStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_AssignStmt>")
    }
}


impl ast_AssignStmt {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn pos(&self) -> Arc<Mutex<Option<token_Pos>>> {
        panic!("ast_AssignStmt.pos bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ast_BadDecl;

impl std::fmt::Display for ast_BadDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_BadDecl>")
    }
}


impl ast_BadDecl {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ast_BadExpr;

impl std::fmt::Display for ast_BadExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_BadExpr>")
    }
}


impl ast_BadExpr {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ast_BadStmt;

impl std::fmt::Display for ast_BadStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_BadStmt>")
    }
}


impl ast_BadStmt {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_BasicLit {
    pub kind: Arc<Mutex<Option<token_Token>>>,
    pub value: Arc<Mutex<Option<String>>>,
    pub value_pos: Arc<Mutex<Option<token_Pos>>>,
}

impl std::fmt::Display for ast_BasicLit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_BasicLit>")
    }
}


impl ast_BasicLit {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_BinaryExpr {
    pub op: Arc<Mutex<Option<token_Token>>>,
    pub op_pos: Arc<Mutex<Option<token_Pos>>>,
    pub x: Arc<Mutex<Option<ast_Expr>>>,
    pub y: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_BinaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_BinaryExpr>")
    }
}


impl ast_BinaryExpr {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_BlockStmt {
    pub lbrace: Arc<Mutex<Option<token_Pos>>>,
    pub list: Arc<Mutex<Option<Vec<ast_Stmt>>>>,
    pub rbrace: Arc<Mutex<Option<token_Pos>>>,
}

impl std::fmt::Display for ast_BlockStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_BlockStmt>")
    }
}


impl ast_BlockStmt {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn end(&self) -> Arc<Mutex<Option<token_Pos>>> {
        panic!("ast_BlockStmt.end bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
    pub fn pos(&self) -> Arc<Mutex<Option<token_Pos>>> {
        panic!("ast_BlockStmt.pos bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_BranchStmt {
    pub label: Arc<Mutex<Option<ast_Ident>>>,
    pub tok: Arc<Mutex<Option<token_Token>>>,
}

impl std::fmt::Display for ast_BranchStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_BranchStmt>")
    }
}


impl ast_BranchStmt {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_CallExpr {
    pub args: Arc<Mutex<Option<Vec<ast_Expr>>>>,
    pub ellipsis: Arc<Mutex<Option<token_Pos>>>,
    pub fun: Arc<Mutex<Option<ast_Expr>>>,
    pub lparen: Arc<Mutex<Option<token_Pos>>>,
    pub rparen: Arc<Mutex<Option<token_Pos>>>,
}

impl std::fmt::Display for ast_CallExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_CallExpr>")
    }
}


impl ast_CallExpr {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn pos(&self) -> Arc<Mutex<Option<token_Pos>>> {
        panic!("ast_CallExpr.pos bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_CaseClause {
    pub body: Arc<Mutex<Option<Vec<ast_Stmt>>>>,
    pub colon: Arc<Mutex<Option<token_Pos>>>,
    pub list: Arc<Mutex<Option<Vec<ast_Expr>>>>,
}

impl std::fmt::Display for ast_CaseClause {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_CaseClause>")
    }
}


impl ast_CaseClause {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ast_ChanDir(pub i32);

impl PartialEq<i32> for ast_ChanDir {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<ast_ChanDir> for i32 {
    fn eq(&self, other: &ast_ChanDir) -> bool {
        *self == other.0
    }
}

impl std::ops::BitAnd for ast_ChanDir {
    type Output = ast_ChanDir;
    fn bitand(self, other: Self) -> ast_ChanDir {
        ast_ChanDir(self.0 & other.0)
    }
}

impl std::ops::BitOr for ast_ChanDir {
    type Output = ast_ChanDir;
    fn bitor(self, other: Self) -> ast_ChanDir {
        ast_ChanDir(self.0 | other.0)
    }
}

impl std::fmt::Display for ast_ChanDir {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_ChanDir>")
    }
}


impl ast_ChanDir {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_ChanType {
    pub dir: Arc<Mutex<Option<ast_ChanDir>>>,
    pub value: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_ChanType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_ChanType>")
    }
}


impl ast_ChanType {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_CommClause {
    pub body: Arc<Mutex<Option<Vec<ast_Stmt>>>>,
    pub comm: Arc<Mutex<Option<ast_Stmt>>>,
}

impl std::fmt::Display for ast_CommClause {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_CommClause>")
    }
}


impl ast_CommClause {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_CompositeLit {
    pub elts: Arc<Mutex<Option<Vec<ast_Expr>>>>,
    pub r#type: Arc<Mutex<Option<ast_Expr>>>,
    pub rbrace: Arc<Mutex<Option<token_Pos>>>,
}

impl std::fmt::Display for ast_CompositeLit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_CompositeLit>")
    }
}


impl ast_CompositeLit {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Clone)]
pub struct ast_Decl {
    pub __go_id: usize,
    pub __go_pos: i32,
    pub __go_value: Arc<dyn std::any::Any + Send + Sync>,
}

impl ast_Decl {
    pub fn __go_from<T: 'static + Send + Sync>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_pos: 0, __go_value: Arc::new(value) }
    }
    pub fn __go_from_with_pos<T: 'static + Send + Sync>(value: T, pos: i32) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_pos: pos, __go_value: Arc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
}

impl Default for ast_Decl {
    fn default() -> Self {
        Self { __go_id: 0, __go_pos: 0, __go_value: Arc::new(()) }
    }
}

impl std::fmt::Debug for ast_Decl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_Decl>")
    }
}

impl std::fmt::Display for ast_Decl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_Decl>")
    }
}

impl PartialEq for ast_Decl {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for ast_Decl {}

impl PartialOrd for ast_Decl {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ast_Decl {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_DeclStmt {
    pub decl: Arc<Mutex<Option<ast_Decl>>>,
}

impl std::fmt::Display for ast_DeclStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_DeclStmt>")
    }
}


impl ast_DeclStmt {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_DeferStmt {
    pub call: Arc<Mutex<Option<ast_CallExpr>>>,
}

impl std::fmt::Display for ast_DeferStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_DeferStmt>")
    }
}


impl ast_DeferStmt {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_Ellipsis {
    pub elt: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_Ellipsis {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_Ellipsis>")
    }
}


impl ast_Ellipsis {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ast_EmptyStmt;

impl std::fmt::Display for ast_EmptyStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_EmptyStmt>")
    }
}


impl ast_EmptyStmt {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Clone)]
pub struct ast_Expr {
    pub __go_id: usize,
    pub __go_pos: i32,
    pub __go_value: Arc<dyn std::any::Any + Send + Sync>,
}

impl ast_Expr {
    pub fn __go_from<T: 'static + Send + Sync>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_pos: 0, __go_value: Arc::new(value) }
    }
    pub fn __go_from_with_pos<T: 'static + Send + Sync>(value: T, pos: i32) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_pos: pos, __go_value: Arc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
    pub fn end(&self) -> Arc<Mutex<Option<token_Pos>>> {
        panic!("ast_Expr.end bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
    pub fn pos(&self) -> Arc<Mutex<Option<token_Pos>>> {
        Arc::new(Mutex::new(Some(token_Pos(self.__go_pos))))
    }
}

impl Default for ast_Expr {
    fn default() -> Self {
        Self { __go_id: 0, __go_pos: 0, __go_value: Arc::new(()) }
    }
}

impl std::fmt::Debug for ast_Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_Expr>")
    }
}

impl std::fmt::Display for ast_Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_Expr>")
    }
}

impl PartialEq for ast_Expr {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for ast_Expr {}

impl PartialOrd for ast_Expr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ast_Expr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_ExprStmt {
    pub x: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_ExprStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_ExprStmt>")
    }
}


impl ast_ExprStmt {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_Field {
    pub names: Arc<Mutex<Option<Vec<Arc<Mutex<Option<ast_Ident>>>>>>>,
    pub r#type: Arc<Mutex<Option<ast_Expr>>>,
    pub tag: Arc<Mutex<Option<ast_BasicLit>>>,
}

impl std::fmt::Display for ast_Field {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_Field>")
    }
}


impl ast_Field {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn pos(&self) -> Arc<Mutex<Option<token_Pos>>> {
        panic!("ast_Field.pos bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_FieldList {
    pub list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<ast_Field>>>>>>>,
}

impl std::fmt::Display for ast_FieldList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_FieldList>")
    }
}


impl ast_FieldList {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn num_fields(&self) -> i32 {
        panic!("ast_FieldList.num_fields bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
    pub fn pos(&self) -> Arc<Mutex<Option<token_Pos>>> {
        panic!("ast_FieldList.pos bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_File {
    pub decls: Arc<Mutex<Option<Vec<ast_Decl>>>>,
    pub go_version: Arc<Mutex<Option<String>>>,
    pub name: Arc<Mutex<Option<ast_Ident>>>,
    pub package: Arc<Mutex<Option<token_Pos>>>,
}

impl std::fmt::Display for ast_File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_File>")
    }
}


impl ast_File {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn end(&self) -> Arc<Mutex<Option<token_Pos>>> {
        panic!("ast_File.end bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
    pub fn pos(&self) -> Arc<Mutex<Option<token_Pos>>> {
        panic!("ast_File.pos bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_ForStmt {
    pub body: Arc<Mutex<Option<ast_BlockStmt>>>,
    pub cond: Arc<Mutex<Option<ast_Expr>>>,
    pub init: Arc<Mutex<Option<ast_Stmt>>>,
    pub post: Arc<Mutex<Option<ast_Stmt>>>,
}

impl std::fmt::Display for ast_ForStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_ForStmt>")
    }
}


impl ast_ForStmt {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_FuncDecl {
    pub body: Arc<Mutex<Option<ast_BlockStmt>>>,
    pub name: Arc<Mutex<Option<ast_Ident>>>,
    pub r#type: Arc<Mutex<Option<ast_FuncType>>>,
    pub recv: Arc<Mutex<Option<ast_FieldList>>>,
}

impl std::fmt::Display for ast_FuncDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_FuncDecl>")
    }
}


impl ast_FuncDecl {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn end(&self) -> Arc<Mutex<Option<token_Pos>>> {
        panic!("ast_FuncDecl.end bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
    pub fn pos(&self) -> Arc<Mutex<Option<token_Pos>>> {
        panic!("ast_FuncDecl.pos bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_FuncLit {
    pub body: Arc<Mutex<Option<ast_BlockStmt>>>,
    pub r#type: Arc<Mutex<Option<ast_FuncType>>>,
}

impl std::fmt::Display for ast_FuncLit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_FuncLit>")
    }
}


impl ast_FuncLit {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn pos(&self) -> Arc<Mutex<Option<token_Pos>>> {
        panic!("ast_FuncLit.pos bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_FuncType {
    pub params: Arc<Mutex<Option<ast_FieldList>>>,
    pub results: Arc<Mutex<Option<ast_FieldList>>>,
    pub type_params: Arc<Mutex<Option<ast_FieldList>>>,
}

impl std::fmt::Display for ast_FuncType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_FuncType>")
    }
}


impl ast_FuncType {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn end(&self) -> Arc<Mutex<Option<token_Pos>>> {
        panic!("ast_FuncType.end bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
    pub fn pos(&self) -> Arc<Mutex<Option<token_Pos>>> {
        panic!("ast_FuncType.pos bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_GenDecl {
    pub specs: Arc<Mutex<Option<Vec<ast_Spec>>>>,
    pub tok: Arc<Mutex<Option<token_Token>>>,
}

impl std::fmt::Display for ast_GenDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_GenDecl>")
    }
}


impl ast_GenDecl {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn pos(&self) -> Arc<Mutex<Option<token_Pos>>> {
        panic!("ast_GenDecl.pos bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_GoStmt {
    pub call: Arc<Mutex<Option<ast_CallExpr>>>,
}

impl std::fmt::Display for ast_GoStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_GoStmt>")
    }
}


impl ast_GoStmt {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_Ident {
    pub name: Arc<Mutex<Option<String>>>,
    pub name_pos: Arc<Mutex<Option<token_Pos>>>,
}

impl std::fmt::Display for ast_Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_Ident>")
    }
}


impl ast_Ident {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn pos(&self) -> Arc<Mutex<Option<token_Pos>>> {
        panic!("ast_Ident.pos bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_IfStmt {
    pub body: Arc<Mutex<Option<ast_BlockStmt>>>,
    pub cond: Arc<Mutex<Option<ast_Expr>>>,
    pub init: Arc<Mutex<Option<ast_Stmt>>>,
    pub r#else: Arc<Mutex<Option<ast_Stmt>>>,
}

impl std::fmt::Display for ast_IfStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_IfStmt>")
    }
}


impl ast_IfStmt {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_ImportSpec {
    pub name: Arc<Mutex<Option<ast_Ident>>>,
    pub path: Arc<Mutex<Option<ast_BasicLit>>>,
}

impl std::fmt::Display for ast_ImportSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_ImportSpec>")
    }
}


impl ast_ImportSpec {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn pos(&self) -> Arc<Mutex<Option<token_Pos>>> {
        panic!("ast_ImportSpec.pos bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_IncDecStmt {
    pub tok: Arc<Mutex<Option<token_Token>>>,
    pub tok_pos: Arc<Mutex<Option<token_Pos>>>,
    pub x: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_IncDecStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_IncDecStmt>")
    }
}


impl ast_IncDecStmt {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_IndexExpr {
    pub index: Arc<Mutex<Option<ast_Expr>>>,
    pub lbrack: Arc<Mutex<Option<token_Pos>>>,
    pub rbrack: Arc<Mutex<Option<token_Pos>>>,
    pub x: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_IndexExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_IndexExpr>")
    }
}


impl ast_IndexExpr {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_IndexListExpr {
    pub indices: Arc<Mutex<Option<Vec<ast_Expr>>>>,
    pub lbrack: Arc<Mutex<Option<token_Pos>>>,
    pub rbrack: Arc<Mutex<Option<token_Pos>>>,
    pub x: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_IndexListExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_IndexListExpr>")
    }
}


impl ast_IndexListExpr {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_InterfaceType {
    pub methods: Arc<Mutex<Option<ast_FieldList>>>,
}

impl std::fmt::Display for ast_InterfaceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_InterfaceType>")
    }
}


impl ast_InterfaceType {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn pos(&self) -> Arc<Mutex<Option<token_Pos>>> {
        panic!("ast_InterfaceType.pos bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_KeyValueExpr {
    pub key: Arc<Mutex<Option<ast_Expr>>>,
    pub value: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_KeyValueExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_KeyValueExpr>")
    }
}


impl ast_KeyValueExpr {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_LabeledStmt {
    pub label: Arc<Mutex<Option<ast_Ident>>>,
    pub stmt: Arc<Mutex<Option<ast_Stmt>>>,
}

impl std::fmt::Display for ast_LabeledStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_LabeledStmt>")
    }
}


impl ast_LabeledStmt {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_MapType {
    pub key: Arc<Mutex<Option<ast_Expr>>>,
    pub value: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_MapType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_MapType>")
    }
}


impl ast_MapType {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Clone)]
pub struct ast_Node {
    pub __go_id: usize,
    pub __go_pos: i32,
    pub __go_value: Arc<dyn std::any::Any + Send + Sync>,
}

impl ast_Node {
    pub fn __go_from<T: 'static + Send + Sync>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_pos: 0, __go_value: Arc::new(value) }
    }
    pub fn __go_from_with_pos<T: 'static + Send + Sync>(value: T, pos: i32) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_pos: pos, __go_value: Arc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
    pub fn end(&self) -> Arc<Mutex<Option<token_Pos>>> {
        panic!("ast_Node.end bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
    pub fn pos(&self) -> Arc<Mutex<Option<token_Pos>>> {
        Arc::new(Mutex::new(Some(token_Pos(self.__go_pos))))
    }
}

impl Default for ast_Node {
    fn default() -> Self {
        Self { __go_id: 0, __go_pos: 0, __go_value: Arc::new(()) }
    }
}

impl std::fmt::Debug for ast_Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_Node>")
    }
}

impl std::fmt::Display for ast_Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_Node>")
    }
}

impl PartialEq for ast_Node {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for ast_Node {}

impl PartialOrd for ast_Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ast_Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_ParenExpr {
    pub x: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_ParenExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_ParenExpr>")
    }
}


impl ast_ParenExpr {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_RangeStmt {
    pub body: Arc<Mutex<Option<ast_BlockStmt>>>,
    pub key: Arc<Mutex<Option<ast_Expr>>>,
    pub tok: Arc<Mutex<Option<token_Token>>>,
    pub tok_pos: Arc<Mutex<Option<token_Pos>>>,
    pub value: Arc<Mutex<Option<ast_Expr>>>,
    pub x: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_RangeStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_RangeStmt>")
    }
}


impl ast_RangeStmt {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_ReturnStmt {
    pub results: Arc<Mutex<Option<Vec<ast_Expr>>>>,
}

impl std::fmt::Display for ast_ReturnStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_ReturnStmt>")
    }
}


impl ast_ReturnStmt {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_SelectStmt {
    pub body: Arc<Mutex<Option<ast_BlockStmt>>>,
}

impl std::fmt::Display for ast_SelectStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_SelectStmt>")
    }
}


impl ast_SelectStmt {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_SelectorExpr {
    pub sel: Arc<Mutex<Option<ast_Ident>>>,
    pub x: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_SelectorExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_SelectorExpr>")
    }
}


impl ast_SelectorExpr {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn pos(&self) -> Arc<Mutex<Option<token_Pos>>> {
        panic!("ast_SelectorExpr.pos bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_SendStmt {
    pub arrow: Arc<Mutex<Option<token_Pos>>>,
    pub chan: Arc<Mutex<Option<ast_Expr>>>,
    pub value: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_SendStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_SendStmt>")
    }
}


impl ast_SendStmt {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_SliceExpr {
    pub high: Arc<Mutex<Option<ast_Expr>>>,
    pub low: Arc<Mutex<Option<ast_Expr>>>,
    pub max: Arc<Mutex<Option<ast_Expr>>>,
    pub rbrack: Arc<Mutex<Option<token_Pos>>>,
    pub slice3: Arc<Mutex<Option<bool>>>,
    pub x: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_SliceExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_SliceExpr>")
    }
}


impl ast_SliceExpr {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Clone)]
pub struct ast_Spec {
    pub __go_id: usize,
    pub __go_pos: i32,
    pub __go_value: Arc<dyn std::any::Any + Send + Sync>,
}

impl ast_Spec {
    pub fn __go_from<T: 'static + Send + Sync>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_pos: 0, __go_value: Arc::new(value) }
    }
    pub fn __go_from_with_pos<T: 'static + Send + Sync>(value: T, pos: i32) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_pos: pos, __go_value: Arc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
}

impl Default for ast_Spec {
    fn default() -> Self {
        Self { __go_id: 0, __go_pos: 0, __go_value: Arc::new(()) }
    }
}

impl std::fmt::Debug for ast_Spec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_Spec>")
    }
}

impl std::fmt::Display for ast_Spec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_Spec>")
    }
}

impl PartialEq for ast_Spec {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for ast_Spec {}

impl PartialOrd for ast_Spec {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ast_Spec {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_StarExpr {
    pub x: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_StarExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_StarExpr>")
    }
}


impl ast_StarExpr {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Clone)]
pub struct ast_Stmt {
    pub __go_id: usize,
    pub __go_pos: i32,
    pub __go_value: Arc<dyn std::any::Any + Send + Sync>,
}

impl ast_Stmt {
    pub fn __go_from<T: 'static + Send + Sync>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_pos: 0, __go_value: Arc::new(value) }
    }
    pub fn __go_from_with_pos<T: 'static + Send + Sync>(value: T, pos: i32) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_pos: pos, __go_value: Arc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
    pub fn pos(&self) -> Arc<Mutex<Option<token_Pos>>> {
        Arc::new(Mutex::new(Some(token_Pos(self.__go_pos))))
    }
}

impl Default for ast_Stmt {
    fn default() -> Self {
        Self { __go_id: 0, __go_pos: 0, __go_value: Arc::new(()) }
    }
}

impl std::fmt::Debug for ast_Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_Stmt>")
    }
}

impl std::fmt::Display for ast_Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_Stmt>")
    }
}

impl PartialEq for ast_Stmt {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for ast_Stmt {}

impl PartialOrd for ast_Stmt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ast_Stmt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_StructType {
    pub fields: Arc<Mutex<Option<ast_FieldList>>>,
}

impl std::fmt::Display for ast_StructType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_StructType>")
    }
}


impl ast_StructType {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_SwitchStmt {
    pub body: Arc<Mutex<Option<ast_BlockStmt>>>,
    pub init: Arc<Mutex<Option<ast_Stmt>>>,
    pub tag: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_SwitchStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_SwitchStmt>")
    }
}


impl ast_SwitchStmt {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_TypeAssertExpr {
    pub r#type: Arc<Mutex<Option<ast_Expr>>>,
    pub x: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_TypeAssertExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_TypeAssertExpr>")
    }
}


impl ast_TypeAssertExpr {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_TypeSpec {
    pub assign: Arc<Mutex<Option<token_Pos>>>,
    pub name: Arc<Mutex<Option<ast_Ident>>>,
    pub r#type: Arc<Mutex<Option<ast_Expr>>>,
    pub type_params: Arc<Mutex<Option<ast_FieldList>>>,
}

impl std::fmt::Display for ast_TypeSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_TypeSpec>")
    }
}


impl ast_TypeSpec {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_TypeSwitchStmt {
    pub assign: Arc<Mutex<Option<ast_Stmt>>>,
    pub body: Arc<Mutex<Option<ast_BlockStmt>>>,
    pub init: Arc<Mutex<Option<ast_Stmt>>>,
}

impl std::fmt::Display for ast_TypeSwitchStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_TypeSwitchStmt>")
    }
}


impl ast_TypeSwitchStmt {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_UnaryExpr {
    pub op: Arc<Mutex<Option<token_Token>>>,
    pub x: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_UnaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_UnaryExpr>")
    }
}


impl ast_UnaryExpr {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_ValueSpec {
    pub names: Arc<Mutex<Option<Vec<Arc<Mutex<Option<ast_Ident>>>>>>>,
    pub r#type: Arc<Mutex<Option<ast_Expr>>>,
    pub values: Arc<Mutex<Option<Vec<ast_Expr>>>>,
}

impl std::fmt::Display for ast_ValueSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_ValueSpec>")
    }
}


impl ast_ValueSpec {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn end(&self) -> Arc<Mutex<Option<token_Pos>>> {
        panic!("ast_ValueSpec.end bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
    pub fn pos(&self) -> Arc<Mutex<Option<token_Pos>>> {
        panic!("ast_ValueSpec.pos bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct atomic_Uint32;

impl std::fmt::Display for atomic_Uint32 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<atomic_Uint32>")
    }
}


impl atomic_Uint32 {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn add<T0>(&self, _arg0: T0) -> u32 {
        panic!("atomic_Uint32.add bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone, Default)]
pub struct buildcfg_ExperimentFlags {
    pub alias_type_params: Arc<Mutex<Option<bool>>>,
    pub range_func: Arc<Mutex<Option<bool>>>,
}

impl std::fmt::Display for buildcfg_ExperimentFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<buildcfg_ExperimentFlags>")
    }
}


impl buildcfg_ExperimentFlags {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
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
pub struct constant_Kind(pub i32);

impl PartialEq<i32> for constant_Kind {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<constant_Kind> for i32 {
    fn eq(&self, other: &constant_Kind) -> bool {
        *self == other.0
    }
}

impl std::ops::BitAnd for constant_Kind {
    type Output = constant_Kind;
    fn bitand(self, other: Self) -> constant_Kind {
        constant_Kind(self.0 & other.0)
    }
}

impl std::ops::BitOr for constant_Kind {
    type Output = constant_Kind;
    fn bitor(self, other: Self) -> constant_Kind {
        constant_Kind(self.0 | other.0)
    }
}

impl std::fmt::Display for constant_Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<constant_Kind>")
    }
}


impl constant_Kind {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Clone)]
pub struct constant_Value {
    pub __go_id: usize,
    pub __go_value: Arc<dyn std::any::Any + Send + Sync>,
}

impl constant_Value {
    pub fn __go_from<T: 'static + Send + Sync>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_value: Arc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
    pub fn kind(&self) -> Arc<Mutex<Option<constant_Kind>>> {
        panic!("constant_Value.kind bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        panic!("constant_Value.string bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}

impl Default for constant_Value {
    fn default() -> Self {
        Self { __go_id: 0, __go_value: Arc::new(()) }
    }
}

impl std::fmt::Debug for constant_Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<constant_Value>")
    }
}

impl std::fmt::Display for constant_Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<constant_Value>")
    }
}

impl PartialEq for constant_Value {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for constant_Value {}

impl PartialOrd for constant_Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for constant_Value {
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


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct godebug_Setting;

impl std::fmt::Display for godebug_Setting {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<godebug_Setting>")
    }
}


impl godebug_Setting {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn inc_non_default(&self) {
        panic!("godebug_Setting.inc_non_default bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
    pub fn value(&self) -> Arc<Mutex<Option<String>>> {
        panic!("godebug_Setting.value bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
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


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct strconv_NumError;

impl std::fmt::Display for strconv_NumError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<strconv_NumError>")
    }
}

impl std::error::Error for strconv_NumError {}


impl strconv_NumError {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        panic!("strconv_NumError.error bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct token_File;

impl std::fmt::Display for token_File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<token_File>")
    }
}


impl token_File {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn base(&self) -> i32 {
        panic!("token_File.base bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        panic!("token_File.name bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
    pub fn size(&self) -> i32 {
        panic!("token_File.size bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct token_FileSet;

impl std::fmt::Display for token_FileSet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<token_FileSet>")
    }
}


impl token_FileSet {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn file<T0>(&self, _arg0: T0) -> Arc<Mutex<Option<token_File>>> {
        panic!("token_FileSet.file bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
    pub fn position<T0>(&self, _arg0: T0) -> Arc<Mutex<Option<token_Position>>> {
        panic!("token_FileSet.position bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
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
    pub fn is_valid(&self) -> Arc<Mutex<Option<bool>>> {
        Arc::new(Mutex::new(Some(self.0 != 0)))
    }
}


#[derive(Debug, Clone, Default)]
pub struct token_Position {
    pub filename: Arc<Mutex<Option<String>>>,
    pub line: Arc<Mutex<Option<i32>>>,
}

impl std::fmt::Display for token_Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<token_Position>")
    }
}


impl token_Position {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        panic!("token_Position.string bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct token_Token(pub i32);

impl PartialEq<i32> for token_Token {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<token_Token> for i32 {
    fn eq(&self, other: &token_Token) -> bool {
        *self == other.0
    }
}

impl std::ops::BitAnd for token_Token {
    type Output = token_Token;
    fn bitand(self, other: Self) -> token_Token {
        token_Token(self.0 & other.0)
    }
}

impl std::ops::BitOr for token_Token {
    type Output = token_Token;
    fn bitor(self, other: Self) -> token_Token {
        token_Token(self.0 | other.0)
    }
}

impl std::fmt::Display for token_Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", token_string_value(*self))
    }
}

fn token_string_value(tok: token_Token) -> &'static str {
    match tok.0 {
        4 => "IDENT",
        5 => "INT",
        6 => "FLOAT",
        7 => "IMAG",
        8 => "CHAR",
        9 => "STRING",
        12 => "+",
        13 => "-",
        14 => "*",
        15 => "/",
        16 => "%",
        17 => "&",
        18 => "|",
        19 => "^",
        20 => "<<",
        21 => ">>",
        22 => "&^",
        23 => "+=",
        24 => "-=",
        25 => "*=",
        26 => "/=",
        27 => "%=",
        28 => "&=",
        29 => "|=",
        30 => "^=",
        31 => "<<=",
        32 => ">>=",
        33 => "&^=",
        34 => "&&",
        35 => "||",
        36 => "<-",
        37 => "++",
        38 => "--",
        39 => "==",
        40 => "<",
        41 => ">",
        42 => "=",
        43 => "!",
        44 => "!=",
        45 => "<=",
        46 => ">=",
        47 => ":=",
        48 => "...",
        61 => "break",
        62 => "case",
        63 => "chan",
        64 => "const",
        65 => "continue",
        66 => "default",
        67 => "defer",
        68 => "else",
        69 => "fallthrough",
        70 => "for",
        71 => "func",
        72 => "go",
        73 => "goto",
        74 => "if",
        75 => "import",
        76 => "interface",
        77 => "map",
        78 => "package",
        79 => "range",
        80 => "return",
        81 => "select",
        82 => "struct",
        83 => "switch",
        84 => "type",
        85 => "var",
        88 => "~",
        _ => "ILLEGAL",
    }
}

impl token_Token {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some::<String>(token_string_value(*self).to_string())))
    }
}


impl From<ast_BasicLit> for ast_Expr {
    fn from(_value: ast_BasicLit) -> Self {
        Self::__go_from(_value)
    }
}


impl From<ast_BinaryExpr> for ast_Expr {
    fn from(_value: ast_BinaryExpr) -> Self {
        Self::__go_from(_value)
    }
}


impl From<ast_CallExpr> for ast_Expr {
    fn from(_value: ast_CallExpr) -> Self {
        Self::__go_from(_value)
    }
}


impl From<ast_FuncType> for ast_Expr {
    fn from(_value: ast_FuncType) -> Self {
        Self::__go_from(_value)
    }
}


impl From<ast_Ident> for ast_Expr {
    fn from(_value: ast_Ident) -> Self {
        Self::__go_from(_value)
    }
}


impl From<ast_IndexExpr> for ast_Expr {
    fn from(_value: ast_IndexExpr) -> Self {
        Self::__go_from(_value)
    }
}


impl From<ast_IndexListExpr> for ast_Expr {
    fn from(_value: ast_IndexListExpr) -> Self {
        Self::__go_from(_value)
    }
}


impl From<ast_InterfaceType> for ast_Expr {
    fn from(_value: ast_InterfaceType) -> Self {
        Self::__go_from(_value)
    }
}


impl From<ast_ParenExpr> for ast_Expr {
    fn from(_value: ast_ParenExpr) -> Self {
        Self::__go_from(_value)
    }
}


impl From<ast_SelectorExpr> for ast_Expr {
    fn from(_value: ast_SelectorExpr) -> Self {
        Self::__go_from(_value)
    }
}


impl From<ast_SliceExpr> for ast_Expr {
    fn from(_value: ast_SliceExpr) -> Self {
        Self::__go_from(_value)
    }
}


impl From<ast_TypeAssertExpr> for ast_Expr {
    fn from(_value: ast_TypeAssertExpr) -> Self {
        Self::__go_from(_value)
    }
}


impl From<ast_UnaryExpr> for ast_Expr {
    fn from(_value: ast_UnaryExpr) -> Self {
        Self::__go_from(_value)
    }
}


impl From<ast_AssignStmt> for ast_Node {
    fn from(_value: ast_AssignStmt) -> Self {
        Self::__go_from(_value)
    }
}


impl From<ast_BlockStmt> for ast_Node {
    fn from(_value: ast_BlockStmt) -> Self {
        Self::__go_from(_value)
    }
}


impl From<ast_CallExpr> for ast_Node {
    fn from(_value: ast_CallExpr) -> Self {
        Self::__go_from(_value)
    }
}


impl From<ast_CaseClause> for ast_Node {
    fn from(_value: ast_CaseClause) -> Self {
        Self::__go_from(_value)
    }
}


impl From<ast_CompositeLit> for ast_Node {
    fn from(_value: ast_CompositeLit) -> Self {
        Self::__go_from(_value)
    }
}


impl From<ast_Expr> for ast_Node {
    fn from(_value: ast_Expr) -> Self {
        Self { __go_id: _value.__go_id, __go_pos: _value.__go_pos, __go_value: _value.__go_value.clone() }
    }
}


impl From<ast_Field> for ast_Node {
    fn from(_value: ast_Field) -> Self {
        Self::__go_from(_value)
    }
}


impl From<ast_File> for ast_Node {
    fn from(_value: ast_File) -> Self {
        Self::__go_from(_value)
    }
}


impl From<ast_ForStmt> for ast_Node {
    fn from(_value: ast_ForStmt) -> Self {
        Self::__go_from(_value)
    }
}


impl From<ast_FuncLit> for ast_Node {
    fn from(_value: ast_FuncLit) -> Self {
        Self::__go_from(_value)
    }
}


impl From<ast_FuncType> for ast_Node {
    fn from(_value: ast_FuncType) -> Self {
        Self::__go_from(_value)
    }
}


impl From<ast_IfStmt> for ast_Node {
    fn from(_value: ast_IfStmt) -> Self {
        Self::__go_from(_value)
    }
}


impl From<ast_ImportSpec> for ast_Node {
    fn from(_value: ast_ImportSpec) -> Self {
        Self::__go_from(_value)
    }
}


impl From<ast_IncDecStmt> for ast_Node {
    fn from(_value: ast_IncDecStmt) -> Self {
        Self::__go_from(_value)
    }
}


impl From<ast_RangeStmt> for ast_Node {
    fn from(_value: ast_RangeStmt) -> Self {
        Self::__go_from(_value)
    }
}


impl From<ast_SendStmt> for ast_Node {
    fn from(_value: ast_SendStmt) -> Self {
        Self::__go_from(_value)
    }
}


impl From<ast_SliceExpr> for ast_Node {
    fn from(_value: ast_SliceExpr) -> Self {
        Self::__go_from(_value)
    }
}


impl From<ast_Stmt> for ast_Node {
    fn from(_value: ast_Stmt) -> Self {
        Self { __go_id: _value.__go_id, __go_pos: _value.__go_pos, __go_value: _value.__go_value.clone() }
    }
}


impl From<ast_SwitchStmt> for ast_Node {
    fn from(_value: ast_SwitchStmt) -> Self {
        Self::__go_from(_value)
    }
}


impl From<ast_TypeSpec> for ast_Node {
    fn from(_value: ast_TypeSpec) -> Self {
        Self::__go_from(_value)
    }
}


impl From<ast_TypeSwitchStmt> for ast_Node {
    fn from(_value: ast_TypeSwitchStmt) -> Self {
        Self::__go_from(_value)
    }
}


impl From<ast_BlockStmt> for ast_Stmt {
    fn from(_value: ast_BlockStmt) -> Self {
        Self::__go_from(_value)
    }
}


impl From<ast_ReturnStmt> for ast_Stmt {
    fn from(_value: ast_ReturnStmt) -> Self {
        Self::__go_from(_value)
    }
}


pub mod ast {
    use super::*;

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

    pub const R_E_C_V: ast_ChanDir = ast_ChanDir(2);
    pub const S_E_N_D: ast_ChanDir = ast_ChanDir(1);

    pub fn new_ident<T0: GoStringArg>(_arg0: T0) -> Arc<Mutex<Option<ast_Ident>>> {
        Arc::new(Mutex::new(Some::<ast_Ident>(ast_Ident { name: Arc::new(Mutex::new(Some::<String>(_arg0.into_go_string()))), ..Default::default() })))
    }

    pub fn unparen<T0>(_arg0: T0) -> Arc<Mutex<Option<ast_Expr>>> {
        panic!("unparen bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


pub mod atomic {
    use super::*;
    pub fn add_int32<T0, T1>(_arg0: T0, _arg1: T1) -> i32 {
        panic!("add_int32 bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn load_int32<T0>(_arg0: T0) -> i32 {
        panic!("load_int32 bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


pub mod buildcfg {
    use super::*;
    pub fn Experiment() -> Arc<Mutex<Option<buildcfg_ExperimentFlags>>> {
        Arc::new(Mutex::new(Some::<buildcfg_ExperimentFlags>(Default::default())))
    }
}


pub mod bytes {
    use super::*;
    pub fn new_buffer_string<T0>(_arg0: T0) -> Arc<Mutex<Option<bytes_Buffer>>> {
        panic!("new_buffer_string bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


pub mod cmp {
    use super::*;
    pub fn compare<T>(_arg0: T, _arg1: T) -> i32 {
        panic!("compare bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


pub mod constant {
    use super::*;
    pub const BOOL: constant_Kind = constant_Kind(1);
    pub const COMPLEX: constant_Kind = constant_Kind(5);
    pub const FLOAT: constant_Kind = constant_Kind(4);
    pub const INT: constant_Kind = constant_Kind(3);
    pub const STRING: constant_Kind = constant_Kind(2);
    pub const UNKNOWN: constant_Kind = constant_Kind(0);

    pub fn binary_op<T0, T1, T2>(_arg0: T0, _arg1: T1, _arg2: T2) -> Arc<Mutex<Option<constant_Value>>> {
        panic!("binary_op bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn bit_len<T0>(_arg0: T0) -> i32 {
        panic!("bit_len bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn bool_val<T0>(_arg0: T0) -> bool {
        panic!("bool_val bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn compare<T0, T1, T2>(_arg0: T0, _arg1: T1, _arg2: T2) -> bool {
        panic!("compare bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn float32_val<T0>(_arg0: T0) -> (f32, bool) {
        panic!("float32_val bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn float64_val<T0>(_arg0: T0) -> (f64, bool) {
        panic!("float64_val bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn imag<T0>(_arg0: T0) -> Arc<Mutex<Option<constant_Value>>> {
        panic!("imag bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn int64_val<T0>(_arg0: T0) -> (i64, bool) {
        panic!("int64_val bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn make_bool<T0>(_arg0: T0) -> Arc<Mutex<Option<constant_Value>>> {
        panic!("make_bool bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn make_float64<T0>(_arg0: T0) -> Arc<Mutex<Option<constant_Value>>> {
        panic!("make_float64 bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn make_from_literal<T0, T1, T2>(_arg0: T0, _arg1: T1, _arg2: T2) -> Arc<Mutex<Option<constant_Value>>> {
        panic!("make_from_literal bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn make_imag<T0>(_arg0: T0) -> Arc<Mutex<Option<constant_Value>>> {
        panic!("make_imag bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn make_int64<T0>(_arg0: T0) -> Arc<Mutex<Option<constant_Value>>> {
        panic!("make_int64 bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn make_string<T0>(_arg0: T0) -> Arc<Mutex<Option<constant_Value>>> {
        panic!("make_string bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn make_unknown() -> Arc<Mutex<Option<constant_Value>>> {
        panic!("make_unknown bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn real<T0>(_arg0: T0) -> Arc<Mutex<Option<constant_Value>>> {
        panic!("real bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn shift<T0, T1, T2>(_arg0: T0, _arg1: T1, _arg2: T2) -> Arc<Mutex<Option<constant_Value>>> {
        panic!("shift bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn sign<T0>(_arg0: T0) -> i32 {
        panic!("sign bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn string_val<T0>(_arg0: T0) -> Arc<Mutex<Option<String>>> {
        panic!("string_val bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn to_complex<T0>(_arg0: T0) -> Arc<Mutex<Option<constant_Value>>> {
        panic!("to_complex bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn to_float<T0>(_arg0: T0) -> Arc<Mutex<Option<constant_Value>>> {
        panic!("to_float bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn to_int<T0>(_arg0: T0) -> Arc<Mutex<Option<constant_Value>>> {
        panic!("to_int bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn uint64_val<T0>(_arg0: T0) -> (u64, bool) {
        panic!("uint64_val bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn unary_op<T0, T1, T2>(_arg0: T0, _arg1: T1, _arg2: T2) -> Arc<Mutex<Option<constant_Value>>> {
        panic!("unary_op bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


pub mod filepath {
    use super::*;
    use std::path::{Path, PathBuf};

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

    pub trait GoPathJoinArgs {
        fn into_path_parts(self) -> Vec<String>;
    }

    impl<T0: GoStringArg> GoPathJoinArgs for (T0,) {
        fn into_path_parts(self) -> Vec<String> {
            vec![self.0.into_go_string()]
        }
    }

    impl<T0: GoStringArg, T1: GoStringArg> GoPathJoinArgs for (T0, T1) {
        fn into_path_parts(self) -> Vec<String> {
            vec![self.0.into_go_string(), self.1.into_go_string()]
        }
    }

    impl<T0: GoStringArg, T1: GoStringArg, T2: GoStringArg> GoPathJoinArgs for (T0, T1, T2) {
        fn into_path_parts(self) -> Vec<String> {
            vec![self.0.into_go_string(), self.1.into_go_string(), self.2.into_go_string()]
        }
    }

    type GoError = Arc<Mutex<Option<Box<dyn std::error::Error + Send + Sync>>>>;

    fn no_error() -> GoError {
        Arc::new(Mutex::new(None))
    }

    fn io_error(err: std::io::Error) -> GoError {
        Arc::new(Mutex::new(Some(Box::new(err))))
    }

    fn normalize_path(path: PathBuf) -> String {
        path.components().collect::<PathBuf>().to_string_lossy().into_owned()
    }

    pub fn base<T0: GoStringArg>(_arg0: T0) -> Arc<Mutex<Option<String>>> {
        let path = _arg0.into_go_string();
        Arc::new(Mutex::new(Some::<String>(Path::new(&path).file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or(path))))
    }
}


pub mod fmt {
    use super::*;
    pub fn fprint<T0, T1>(_arg0: T0, _arg1: T1) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        panic!("fprint bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


pub mod godebug {
    use super::*;
    pub fn new<T0>(_arg0: T0) -> Arc<Mutex<Option<godebug_Setting>>> {
        panic!("new bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


pub mod goversion {
    use super::*;
    pub const VERSION: i32 = 24;
}


pub mod heap {
    use super::*;
    pub fn fix<T0, T1>(_arg0: T0, _arg1: T1) {
        panic!("fix bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn init<T0>(_arg0: T0) {
        panic!("init bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn pop<T0>(_arg0: T0) -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
        panic!("pop bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


pub mod math {
    use super::*;
    pub fn is_inf<T0, T1>(_arg0: T0, _arg1: T1) -> bool {
        panic!("is_inf bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


pub mod runtime {
    use super::*;
    pub fn caller<T0>(_arg0: T0) -> (usize, Arc<Mutex<Option<String>>>, i32, bool) {
        panic!("caller bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


pub mod slices {
    use super::*;
    pub fn equal_func<S1, S2, E1, E2>(_arg0: Arc<Mutex<Option<Vec<Arc<Mutex<Option<E1>>>>>>>, _arg1: Arc<Mutex<Option<Vec<Arc<Mutex<Option<E2>>>>>>>, _arg2: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<E1>>>, Arc<Mutex<Option<E2>>>) -> bool + Send + Sync>>>>) -> bool {
        panic!("equal_func bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn index<S, E>(_arg0: Arc<Mutex<Option<Vec<Arc<Mutex<Option<E>>>>>>>, _arg1: Arc<Mutex<Option<E>>>) -> i32 {
        panic!("index bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn is_sorted_func<S, E>(_arg0: Arc<Mutex<Option<Vec<Arc<Mutex<Option<E>>>>>>>, _arg1: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<E>>>, Arc<Mutex<Option<E>>>) -> i32 + Send + Sync>>>>) -> bool {
        panic!("is_sorted_func bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


pub mod strconv {
    use super::*;

    pub trait GoStrconvStringArg {
        fn into_go_strconv_string(self) -> String;
    }

    impl GoStrconvStringArg for String {
        fn into_go_strconv_string(self) -> String {
            self
        }
    }

    impl<'a> GoStrconvStringArg for &'a str {
        fn into_go_strconv_string(self) -> String {
            self.to_string()
        }
    }

    impl<'a> GoStrconvStringArg for &'a String {
        fn into_go_strconv_string(self) -> String {
            self.clone()
        }
    }

    impl GoStrconvStringArg for Arc<Mutex<Option<String>>> {
        fn into_go_strconv_string(self) -> String {
            self.lock().unwrap().as_ref().cloned().unwrap_or_default()
        }
    }

    fn strconv_error(message: String) -> Box<dyn std::error::Error + Send + Sync> {
        Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, message))
    }

    fn strconv_hex_digit(ch: char) -> Option<u32> {
        ch.to_digit(16)
    }

    fn strconv_read_hex<I: Iterator<Item = char>>(chars: &mut I, count: usize) -> Result<char, Box<dyn std::error::Error + Send + Sync>> {
        let mut value = 0u32;
        for _ in 0..count {
            let ch = chars.next().ok_or_else(|| strconv_error("invalid quoted string".to_string()))?;
            let digit = strconv_hex_digit(ch).ok_or_else(|| strconv_error("invalid quoted string".to_string()))?;
            value = (value << 4) | digit;
        }
        char::from_u32(value).ok_or_else(|| strconv_error("invalid quoted string".to_string()))
    }

    fn strconv_unquote_text(input: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut chars = input.chars();
        let quote = chars.next().ok_or_else(|| strconv_error("invalid quoted string".to_string()))?;
        if quote != '"' && quote != '\'' && quote != char::from(96) {
            return Err(strconv_error("invalid quoted string".to_string()));
        }
        let inner = input.strip_prefix(quote).and_then(|s| s.strip_suffix(quote)).ok_or_else(|| strconv_error("invalid quoted string".to_string()))?;
        if quote == char::from(96) {
            return Ok(inner.to_string());
        }
        let mut out = String::new();
        let mut chars = inner.chars();
        while let Some(ch) = chars.next() {
            if ch != '\\' {
                out.push(ch);
                continue;
            }
            let esc = chars.next().ok_or_else(|| strconv_error("invalid quoted string".to_string()))?;
            match esc {
                'a' => out.push('\x07'),
                'b' => out.push('\x08'),
                'f' => out.push('\x0c'),
                'n' => out.push('\n'),
                'r' => out.push('\r'),
                't' => out.push('\t'),
                'v' => out.push('\x0b'),
                '\\' => out.push('\\'),
                '"' => out.push('"'),
                '\'' => out.push('\''),
                'x' => out.push(strconv_read_hex(&mut chars, 2)?),
                'u' => out.push(strconv_read_hex(&mut chars, 4)?),
                'U' => out.push(strconv_read_hex(&mut chars, 8)?),
                '0'..='7' => {
                    let mut value = esc.to_digit(8).unwrap();
                    for _ in 0..2 {
                        let Some(next) = chars.clone().next() else { break };
                        let Some(digit) = next.to_digit(8) else { break };
                        chars.next();
                        value = (value << 3) | digit;
                    }
                    out.push(char::from_u32(value).ok_or_else(|| strconv_error("invalid quoted string".to_string()))?);
                }
                _ => return Err(strconv_error("invalid quoted string".to_string())),
            }
        }
        Ok(out)
    }

    pub fn unquote<T0: GoStrconvStringArg>(_arg0: T0) -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        match strconv_unquote_text(&_arg0.into_go_strconv_string()) {
            Ok(value) => (Arc::new(Mutex::new(Some::<String>(value))), Arc::new(Mutex::new(None::<Box<dyn std::error::Error + Send + Sync>>))),
            Err(err) => (Arc::new(Mutex::new(None::<String>)), Arc::new(Mutex::new(Some::<Box<dyn std::error::Error + Send + Sync>>(err)))),
        }
    }
}


pub mod strings {
    use super::*;
    pub fn contains_rune<T0, T1>(_arg0: T0, _arg1: T1) -> bool {
        panic!("contains_rune bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn last_index_any<T0, T1>(_arg0: T0, _arg1: T1) -> i32 {
        panic!("last_index_any bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


pub mod token {
    use super::*;

    pub const A_D_D: token_Token = token_Token(12);
    pub const A_D_D__A_S_S_I_G_N: token_Token = token_Token(23);
    pub const A_N_D: token_Token = token_Token(17);
    pub const A_N_D__A_S_S_I_G_N: token_Token = token_Token(28);
    pub const A_N_D__N_O_T: token_Token = token_Token(22);
    pub const A_N_D__N_O_T__A_S_S_I_G_N: token_Token = token_Token(33);
    pub const A_R_R_O_W: token_Token = token_Token(36);
    pub const A_S_S_I_G_N: token_Token = token_Token(42);
    pub const B_R_E_A_K: token_Token = token_Token(61);
    pub const C_A_S_E: token_Token = token_Token(62);
    pub const C_H_A_N: token_Token = token_Token(63);
    pub const C_H_A_R: token_Token = token_Token(8);
    pub const C_O_L_O_N: token_Token = token_Token(58);
    pub const C_O_M_M_A: token_Token = token_Token(52);
    pub const C_O_M_M_E_N_T: token_Token = token_Token(2);
    pub const C_O_N_S_T: token_Token = token_Token(64);
    pub const C_O_N_T_I_N_U_E: token_Token = token_Token(65);
    pub const D_E_C: token_Token = token_Token(38);
    pub const D_E_F_A_U_L_T: token_Token = token_Token(66);
    pub const D_E_F_E_R: token_Token = token_Token(67);
    pub const D_E_F_I_N_E: token_Token = token_Token(47);
    pub const E_L_L_I_P_S_I_S: token_Token = token_Token(48);
    pub const E_L_S_E: token_Token = token_Token(68);
    pub const E_O_F: token_Token = token_Token(1);
    pub const E_Q_L: token_Token = token_Token(39);
    pub const F_A_L_L_T_H_R_O_U_G_H: token_Token = token_Token(69);
    pub const F_L_O_A_T: token_Token = token_Token(6);
    pub const F_O_R: token_Token = token_Token(70);
    pub const F_U_N_C: token_Token = token_Token(71);
    pub const G_E_Q: token_Token = token_Token(46);
    pub const G_O: token_Token = token_Token(72);
    pub const G_O_T_O: token_Token = token_Token(73);
    pub const G_T_R: token_Token = token_Token(41);
    pub const I_D_E_N_T: token_Token = token_Token(4);
    pub const I_F: token_Token = token_Token(74);
    pub const I_L_L_E_G_A_L: token_Token = token_Token(0);
    pub const I_M_A_G: token_Token = token_Token(7);
    pub const I_M_P_O_R_T: token_Token = token_Token(75);
    pub const I_N_C: token_Token = token_Token(37);
    pub const I_N_T: token_Token = token_Token(5);
    pub const I_N_T_E_R_F_A_C_E: token_Token = token_Token(76);
    pub const L_A_N_D: token_Token = token_Token(34);
    pub const L_B_R_A_C_E: token_Token = token_Token(51);
    pub const L_B_R_A_C_K: token_Token = token_Token(50);
    pub const L_E_Q: token_Token = token_Token(45);
    pub const L_O_R: token_Token = token_Token(35);
    pub const L_P_A_R_E_N: token_Token = token_Token(49);
    pub const L_S_S: token_Token = token_Token(40);
    pub const M_A_P: token_Token = token_Token(77);
    pub const M_U_L: token_Token = token_Token(14);
    pub const M_U_L__A_S_S_I_G_N: token_Token = token_Token(25);
    pub const NO_POS: token_Pos = token_Pos(0);
    pub const N_E_Q: token_Token = token_Token(44);
    pub const N_O_T: token_Token = token_Token(43);
    pub const O_R: token_Token = token_Token(18);
    pub const O_R__A_S_S_I_G_N: token_Token = token_Token(29);
    pub const P_A_C_K_A_G_E: token_Token = token_Token(78);
    pub const P_E_R_I_O_D: token_Token = token_Token(53);
    pub const Q_U_O: token_Token = token_Token(15);
    pub const Q_U_O__A_S_S_I_G_N: token_Token = token_Token(26);
    pub const R_A_N_G_E: token_Token = token_Token(79);
    pub const R_B_R_A_C_E: token_Token = token_Token(56);
    pub const R_B_R_A_C_K: token_Token = token_Token(55);
    pub const R_E_M: token_Token = token_Token(16);
    pub const R_E_M__A_S_S_I_G_N: token_Token = token_Token(27);
    pub const R_E_T_U_R_N: token_Token = token_Token(80);
    pub const R_P_A_R_E_N: token_Token = token_Token(54);
    pub const S_E_L_E_C_T: token_Token = token_Token(81);
    pub const S_E_M_I_C_O_L_O_N: token_Token = token_Token(57);
    pub const S_H_L: token_Token = token_Token(20);
    pub const S_H_L__A_S_S_I_G_N: token_Token = token_Token(31);
    pub const S_H_R: token_Token = token_Token(21);
    pub const S_H_R__A_S_S_I_G_N: token_Token = token_Token(32);
    pub const S_T_R_I_N_G: token_Token = token_Token(9);
    pub const S_T_R_U_C_T: token_Token = token_Token(82);
    pub const S_U_B: token_Token = token_Token(13);
    pub const S_U_B__A_S_S_I_G_N: token_Token = token_Token(24);
    pub const S_W_I_T_C_H: token_Token = token_Token(83);
    pub const T_I_L_D_E: token_Token = token_Token(88);
    pub const T_Y_P_E: token_Token = token_Token(84);
    pub const V_A_R: token_Token = token_Token(85);
    pub const X_O_R: token_Token = token_Token(19);
    pub const X_O_R__A_S_S_I_G_N: token_Token = token_Token(30);

    pub fn is_exported<T0>(_arg0: T0) -> bool {
        panic!("is_exported bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


pub mod unicode {
    use super::*;
    pub const MAX_RUNE: i32 = 1114111;
    pub const REPLACEMENT_CHAR: i32 = 65533;

    pub fn is_digit<T0>(_arg0: T0) -> bool {
        panic!("is_digit bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn is_graphic<T0>(_arg0: T0) -> bool {
        panic!("is_graphic bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn is_letter<T0>(_arg0: T0) -> bool {
        panic!("is_letter bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn is_space<T0>(_arg0: T0) -> bool {
        panic!("is_space bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn is_upper<T0>(_arg0: T0) -> bool {
        panic!("is_upper bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


pub mod utf8 {
    use super::*;
    pub fn decode_rune_in_string<T0>(_arg0: T0) -> (i32, i32) {
        panic!("decode_rune_in_string bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn encode_rune<T0, T1>(_arg0: T0, _arg1: T1) -> i32 {
        panic!("encode_rune bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


pub mod version {
    use super::*;
    pub fn compare<T0, T1>(_arg0: T0, _arg1: T1) -> i32 {
        panic!("compare bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn lang<T0>(_arg0: T0) -> Arc<Mutex<Option<String>>> {
        panic!("lang bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}
