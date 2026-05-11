use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;


fn format_any(value: &dyn Any) -> String {
    if let Some(v) = value.downcast_ref::<i32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<String>() {
        v.clone()
    } else if let Some(v) = value.downcast_ref::<&str>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<bool>() {
        v.to_string()
    } else {
        "<unknown>".to_string()
    }
}

pub trait Reader: std::fmt::Display + Any {
    fn __go_clone_box(&self) -> Box<dyn Reader + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq(&self, other: &(dyn Reader + Send + Sync)) -> bool;
    fn read(&self) -> Arc<Mutex<Option<i32>>>;
}

impl Clone for Box<dyn Reader + Send + Sync> {
    fn clone(&self) -> Self {
        self.__go_clone_box()
    }
}

#[derive(Clone, Default)]
pub struct holder {
    pub reader: Arc<Mutex<Option<Box<dyn Reader + Send + Sync>>>>,
    pub value: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>,
}

impl holder {
    pub fn __go_value_clone(&self) -> Self {
        Self { reader: self.reader.clone(), value: self.value.clone() }
    }
}

impl std::fmt::Display for holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.reader.lock().unwrap().as_ref().unwrap()), format_any(self.value.lock().unwrap().as_ref().unwrap().as_ref()))
    }
}


pub(crate) static global: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<holder>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *global.lock().unwrap() = Some(Default::default());
}


fn main() {
    __go_init_all();
    std::thread::spawn(move || {
        ;
    });
    println!("{}", (*(*global.lock().unwrap().as_ref().unwrap()).value.lock().unwrap()).is_none());
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
