use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;


fn format_any(value: &(dyn Any + Send + Sync)) -> String {
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
    fn __go_clone_box_reader(&self) -> Box<dyn Reader + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_reader(&self, other: &(dyn Reader + Send + Sync)) -> bool;
    fn read(&self) -> i32;
}

impl Clone for Box<dyn Reader + Send + Sync> {
    fn clone(&self) -> Self {
        Reader::__go_clone_box_reader(self.as_ref())
    }
}

#[derive(Clone, Default)]
pub struct holder {
    pub reader: Arc<Mutex<Option<Box<dyn Reader + Send + Sync>>>>,
    pub value: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>,
}

impl holder {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.reader.clone();
        let __go_clone_1_0 = self.value.clone();
        Self {
            reader: __go_clone_0_0,
            value: __go_clone_1_0,
        }
    }
}

impl std::fmt::Display for holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.reader.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", match self.value.lock().unwrap().as_ref() { Some(__v) => format_any(__v.as_ref()), None => "<nil>".to_string() });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
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
    println!("{}", format!("{}", { let __nil_target = (*global.lock().unwrap().as_ref().unwrap()).value.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result }));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
