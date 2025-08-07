use std::sync::{Arc, Mutex};

fn main() {
    let mut num: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(100)));
    let mut ptr: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(num.clone())));
    let _ = (*ptr.lock().unwrap().as_mut().unwrap());
}