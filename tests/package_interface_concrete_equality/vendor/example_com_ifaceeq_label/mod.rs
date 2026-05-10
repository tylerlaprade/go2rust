use go2rust_stdlib_stubs::*;

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub trait Key: std::fmt::Display + Any {
    fn __go_clone_box(&self) -> Box<dyn Key + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq(&self, other: &(dyn Key + Send + Sync)) -> bool;
    fn name(&self) -> Arc<Mutex<Option<String>>>;
}

impl Clone for Box<dyn Key + Send + Sync> {
    fn clone(&self) -> Self {
        self.__go_clone_box()
    }
}

#[derive(Clone, Default)]
pub struct Label {
    pub key: Arc<Mutex<Option<Box<dyn Key + Send + Sync>>>>,
}

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.key.lock().unwrap().as_ref().unwrap()))
    }
}


impl Label {
    pub fn key(&self) -> Arc<Mutex<Option<Box<dyn Key + Send + Sync>>>> {
        return self.key.clone();
    }
}

pub fn new(key: &(dyn Key + Send + Sync)) -> Arc<Mutex<Option<Label>>> {

    return Arc::new(Mutex::new(Some(Label { key: Arc::new(Mutex::new(Some(key.__go_clone_box()))), ..Default::default() })));
}