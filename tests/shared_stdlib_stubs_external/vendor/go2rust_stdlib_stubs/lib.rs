use std::any::Any;
use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::rc::{Rc};

pub struct GoPtrKey<T>(pub Rc<RefCell<Option<T>>>);

impl<T> Clone for GoPtrKey<T> {
    fn clone(&self) -> Self { GoPtrKey(self.0.clone()) }
}

impl<T> GoPtrKey<T> {
    pub fn new(value: Rc<RefCell<Option<T>>>) -> Self { GoPtrKey(value) }
    pub fn value(&self) -> Rc<RefCell<Option<T>>> { self.0.clone() }
    fn addr(&self) -> usize { Rc::as_ptr(&self.0) as usize }
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

fn go_json_no_error() -> Rc<RefCell<Option<Box<dyn StdError>>>> {
    Rc::new(RefCell::new(None))
}

fn go_json_error(message: String) -> Rc<RefCell<Option<Box<dyn StdError>>>> {
    Rc::new(RefCell::new(Some(Box::<dyn StdError>::from(message))))
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

impl<T> GoJsonInputArg for Rc<RefCell<Option<T>>>
where
    T: GoJsonInputArg + Clone,
{
    fn into_go_json_bytes(self) -> Vec<u8> {
        self.borrow().as_ref().cloned().map(|value| value.into_go_json_bytes()).unwrap_or_default()
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

impl<T> GoJsonDecode for Rc<RefCell<Option<T>>>
where
    T: GoJsonDecode,
{
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        if value.is_null() {
            Ok(Rc::new(RefCell::new(None)))
        } else {
            Ok(Rc::new(RefCell::new(Some(T::go_json_decode(value)?))))
        }
    }
}

impl<T> GoJsonDecodeTarget for Rc<RefCell<Option<T>>>
where
    T: GoJsonDecode,
{
    fn assign_go_json(self, value: &serde_json::Value) -> Result<(), String> {
        if value.is_null() {
            *self.borrow_mut() = None;
        } else {
            *self.borrow_mut() = Some(T::go_json_decode(value)?);
        }
        Ok(())
    }
}



#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Tuple;

impl std::fmt::Display for types_Tuple {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Tuple>")
    }
}


impl types_Tuple {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn len(&self) -> i32 {
        panic!("types_Tuple.len bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Clone)]
pub struct types_Type {
    pub __go_id: usize,
    pub __go_value: Rc<dyn std::any::Any>,
}

impl types_Type {
    pub fn __go_from<T: 'static>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_value: Rc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
}

impl Default for types_Type {
    fn default() -> Self {
        Self { __go_id: 0, __go_value: Rc::new(()) }
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
