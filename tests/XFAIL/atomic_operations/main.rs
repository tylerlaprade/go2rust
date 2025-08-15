use std::sync::{Arc, Mutex};

fn main() {
    let mut counter: Arc<Mutex<Option<i64>>> = Default::default();
    atomic.add_int64(Arc::new(Mutex::new(Some(counter.clone()))), Arc::new(Mutex::new(Some(1))));
    atomic.add_int64(Arc::new(Mutex::new(Some(counter.clone()))), Arc::new(Mutex::new(Some(5))));
    let mut value = atomic.load_int64(Arc::new(Mutex::new(Some(counter.clone()))));
    println!("{} {}", "Atomic counter:".to_string(), (*value.lock().unwrap().as_mut().unwrap()));
}