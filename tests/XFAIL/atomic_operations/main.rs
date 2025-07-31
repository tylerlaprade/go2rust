fn main() {
    let mut counter = Default::default();
    (*atomic.lock().unwrap().as_ref().unwrap()).add_int64(std::sync::Arc::new(std::sync::Mutex::new(Some(counter.clone()))), std::sync::Arc::new(std::sync::Mutex::new(Some(1))));
    (*atomic.lock().unwrap().as_ref().unwrap()).add_int64(std::sync::Arc::new(std::sync::Mutex::new(Some(counter.clone()))), std::sync::Arc::new(std::sync::Mutex::new(Some(5))));
    let mut value = (*atomic.lock().unwrap().as_ref().unwrap()).load_int64(std::sync::Arc::new(std::sync::Mutex::new(Some(counter.clone()))));
    println!("{} {}", "Atomic counter:".to_string(), (*value.lock().unwrap().as_ref().unwrap()));
}