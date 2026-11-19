use std::any::Any;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn zero() -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
    let mut v: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(None));

    v.clone()
}

fn main() {
    std::thread::spawn(move || {
        ;
    });
    if { let __nil_result = (*zero().lock().unwrap()).is_none(); __nil_result } {
        println!("{}", format!("{}", "nil".to_string()));
    }
}