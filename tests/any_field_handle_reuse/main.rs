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

#[derive(Clone, Default)]
pub struct entry {
    pub value: Arc<Mutex<Option<Box<dyn Any>>>>,
}

impl std::fmt::Display for entry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_any(self.value.lock().unwrap().as_ref().unwrap().as_ref()))
    }
}


pub fn assign(e: Arc<Mutex<Option<entry>>>, value: Arc<Mutex<Option<Box<dyn Any>>>>) {
    { let new_val = value.clone(); (*e.lock().unwrap().as_mut().unwrap()).value = new_val; };
}

pub fn each(e: Arc<Mutex<Option<entry>>>, f: Arc<Mutex<Option<Box<dyn Fn(Arc<Mutex<Option<Box<dyn Any>>>>) -> () + Send + Sync>>>>) {
    { let __f_guard = f.lock().unwrap(); let __f = __f_guard.as_ref().unwrap(); (*__f)((*e.lock().unwrap().as_ref().unwrap()).value.clone()) };
}

fn main() {
    std::thread::spawn(move || {
        ;
    });

    let mut value: Arc<Mutex<Option<Box<dyn Any>>>> = Arc::new(Mutex::new(Some(Box::new("ok".to_string()) as Box<dyn Any>)));
    let mut e = Arc::new(Mutex::new(Some(entry { value: Arc::new(Mutex::new(None)) })));
    assign(e.clone(), value.clone());
    each(e.clone(), Arc::new(Mutex::new(Some(Box::new(move |v: Arc<Mutex<Option<Box<dyn Any>>>>| {
        println!("{}", format_any(v.lock().unwrap().as_ref().unwrap().as_ref()));
    }) as Box<dyn Fn(Arc<Mutex<Option<Box<dyn Any>>>>) -> () + Send + Sync>))));
}