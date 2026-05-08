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


pub fn get(e: Arc<Mutex<Option<entry>>>) -> Arc<Mutex<Option<Box<dyn Any>>>> {

    return (*e.lock().unwrap().as_ref().unwrap()).value.clone();
}

fn main() {
    std::thread::spawn(move || {
        ;
    });
    let mut e = Arc::new(Mutex::new(Some(entry { value: Arc::new(Mutex::new(Some(Box::new("ok".to_string()) as Box<dyn Any>))), ..Default::default() })));
    let mut v = get(Arc::new(Mutex::new(Some((*e.lock().unwrap().as_ref().unwrap()).clone()))));
    let (_, mut ok) = ({
        let val = v.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<String>() {
                (Arc::new(Mutex::new(Some(typed_val.clone()))), Arc::new(Mutex::new(Some(true))))
            } else {
                (Arc::new(Mutex::new(Some(String::new()))), Arc::new(Mutex::new(Some(false))))
            }
        } else {
            (Arc::new(Mutex::new(Some(String::new()))), Arc::new(Mutex::new(Some(false))))
        }
    });
    if { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        println!("{}", "ok".to_string());
    }
}