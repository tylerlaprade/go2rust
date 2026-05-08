use std::any::Any;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn zero() -> Arc<Mutex<Option<Box<dyn Any>>>> {
    let mut v: Arc<Mutex<Option<Box<dyn Any>>>> = Arc::new(Mutex::new(None));

    return v;
}

fn main() {
    std::thread::spawn(move || {
        ;
    });
    if (*zero().lock().unwrap()).is_none() {
        println!("{}", "nil".to_string());
    }
}