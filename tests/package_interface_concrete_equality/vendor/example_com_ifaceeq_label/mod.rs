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
        Key::__go_clone_box_key(self.as_ref())
    }
}

#[derive(Clone, Default)]
pub struct Label {
    pub key: Arc<Mutex<Option<Box<dyn Key + Send + Sync>>>>,
}

impl Label {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.key.clone();
        Self {
            key: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.key.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Label {
    pub fn key(&self) -> Arc<Mutex<Option<Box<dyn Key + Send + Sync>>>> {
        { let __field = self.key.clone(); __field }
    }
}

pub fn new(key: Arc<Mutex<Option<Box<dyn Key + Send + Sync>>>>) -> Arc<Mutex<Option<Label>>> {
    Arc::new(Mutex::new(Some(Label { key: key.clone(), ..Default::default() })))
}

impl GoValueClone for Label {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
