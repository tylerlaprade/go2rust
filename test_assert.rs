use std::any::Any;
use std::sync::{Arc, Mutex};

fn main() {
    let mut x: Arc<Mutex<Option<Box<dyn Any>>>> = Arc::new(Mutex::new(Some(Box::new("hello".to_string()) as Box<dyn Any>)));
    let mut s = Arc::new(Mutex::new(Some(({
        let val = x;
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
    }))));
    println!("{}", (*s.lock().unwrap().as_mut().unwrap()));
}