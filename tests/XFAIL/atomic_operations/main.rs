fn main() {
    let mut counter: std::sync::Arc<std::sync::Mutex<Option<i64>>> = Default::default();
    (*atomic.lock().unwrap().as_mut().unwrap()).add_int64(std::sync::Arc::new(std::sync::Mutex::new(Some(counter.clone()))), std::sync::Arc::new(std::sync::Mutex::new(Some(1))));
    (*atomic.lock().unwrap().as_mut().unwrap()).add_int64(std::sync::Arc::new(std::sync::Mutex::new(Some(counter.clone()))), std::sync::Arc::new(std::sync::Mutex::new(Some(5))));
    let mut value = (*atomic.lock().unwrap().as_mut().unwrap()).load_int64(std::sync::Arc::new(std::sync::Mutex::new(Some(counter.clone()))));
    println!("{} {}", "Atomic counter:".to_string(), (*value.lock().unwrap().as_mut().unwrap()));
}