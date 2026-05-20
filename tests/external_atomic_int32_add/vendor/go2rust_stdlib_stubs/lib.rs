use std::any::Any;
use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::rc::{Rc};

#[derive(Clone)]
pub struct GoPtrKey<T>(pub Rc<RefCell<Option<T>>>);

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

impl GoJsonDecode for f64 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_f64().ok_or_else(|| go_json_expected(value, "number"))
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



#[derive(Debug, Clone)]
pub struct atomic_Int32 {
    __go_value: std::sync::Arc<std::sync::atomic::AtomicI32>,
}

impl Default for atomic_Int32 {
    fn default() -> Self {
        Self { __go_value: std::sync::Arc::new(std::sync::atomic::AtomicI32::new(0)) }
    }
}

impl std::fmt::Display for atomic_Int32 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<atomic_Int32>")
    }
}

fn __go_atomic_i32_arg<T: 'static>(arg: &T) -> i32 {
    let any = arg as &dyn std::any::Any;
    if let Some(v) = any.downcast_ref::<i32>() {
        *v
    } else if let Some(v) = any.downcast_ref::<i64>() {
        *v as i32
    } else if let Some(v) = any.downcast_ref::<u32>() {
        *v as i32
    } else if let Some(v) = any.downcast_ref::<u64>() {
        *v as i32
    } else if let Some(v) = any.downcast_ref::<usize>() {
        *v as i32
    } else if let Some(v) = any.downcast_ref::<isize>() {
        *v as i32
    } else if let Some(v) = any.downcast_ref::<std::sync::Arc<std::sync::Mutex<Option<i32>>>>() {
        v.lock().unwrap().as_ref().copied().unwrap_or_default()
    } else if let Some(v) = any.downcast_ref::<std::rc::Rc<std::cell::RefCell<Option<i32>>>>() {
        v.borrow().as_ref().copied().unwrap_or_default()
    } else {
        0
    }
}

impl atomic_Int32 {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }

    pub fn add<T0: 'static>(&self, arg0: T0) -> Rc<RefCell<Option<i32>>> {
        let delta = __go_atomic_i32_arg(&arg0);
        let previous = self.__go_value.fetch_add(delta, std::sync::atomic::Ordering::SeqCst);
        Rc::new(RefCell::new(Some::<i32>(previous.wrapping_add(delta))))
    }

    pub fn load(&self) -> Rc<RefCell<Option<i32>>> {
        Rc::new(RefCell::new(Some::<i32>(self.__go_value.load(std::sync::atomic::Ordering::SeqCst))))
    }

    pub fn store<T0: 'static>(&self, arg0: T0) {
        self.__go_value.store(__go_atomic_i32_arg(&arg0), std::sync::atomic::Ordering::SeqCst);
    }

    pub fn swap<T0: 'static>(&self, arg0: T0) -> Rc<RefCell<Option<i32>>> {
        Rc::new(RefCell::new(Some::<i32>(self.__go_value.swap(__go_atomic_i32_arg(&arg0), std::sync::atomic::Ordering::SeqCst))))
    }

    pub fn compare_and_swap<T0: 'static, T1: 'static>(&self, old: T0, new: T1) -> Rc<RefCell<Option<bool>>> {
        let old = __go_atomic_i32_arg(&old);
        let new = __go_atomic_i32_arg(&new);
        Rc::new(RefCell::new(Some::<bool>(self.__go_value.compare_exchange(old, new, std::sync::atomic::Ordering::SeqCst, std::sync::atomic::Ordering::SeqCst).is_ok())))
    }
}
