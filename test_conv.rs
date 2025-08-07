use std::sync::{Arc, Mutex};

fn main() {
    let mut i: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(42)));
    let mut f: Arc<Mutex<Option<f64>>> = Arc::new(Mutex::new(Some((*(*i.lock().unwrap().as_mut().unwrap()).lock().unwrap().as_ref().unwrap()) as f64)));
    let _ = (*f.lock().unwrap().as_mut().unwrap());
}