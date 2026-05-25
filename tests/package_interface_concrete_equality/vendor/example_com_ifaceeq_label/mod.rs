use go2rust_stdlib_stubs::*;

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub trait Key: std::fmt::Display + Any {
    fn __go_clone_box_key(&self) -> Box<dyn Key + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_key(&self, other: &(dyn Key + Send + Sync)) -> bool;
    fn name(&self) -> Arc<Mutex<Option<String>>>;
}

impl Clone for Box<dyn Key + Send + Sync> {
    fn clone(&self) -> Self {
        self.__go_clone_box_key()
    }
}

#[derive(Clone, Default)]
pub struct Label {
    pub key: Arc<Mutex<Option<Box<dyn Key + Send + Sync>>>>,
}

impl Label {
    pub fn __go_value_clone(&self) -> Self {
        Self { key: self.key.clone() }
    }
}

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.key.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Label {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl Label {
    pub fn key(&self) -> Arc<Mutex<Option<Box<dyn Key + Send + Sync>>>> {
        return self.key.clone();
    }
}

pub fn new(key: Arc<Mutex<Option<Box<dyn Key + Send + Sync>>>>) -> Arc<Mutex<Option<Label>>> {

    return Arc::new(Mutex::new(Some(Label { key: key.clone(), ..Default::default() })));
}