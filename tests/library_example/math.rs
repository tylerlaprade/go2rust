pub fn add(a: std::sync::Arc<std::sync::Mutex<Option<i32>>>, b: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {

    return std::sync::Arc::new(std::sync::Mutex::new(Some((*a.lock().unwrap().as_ref().unwrap()) + (*b.lock().unwrap().as_ref().unwrap()))));
}

pub fn multiply(a: std::sync::Arc<std::sync::Mutex<Option<i32>>>, b: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {

    return std::sync::Arc::new(std::sync::Mutex::new(Some((*a.lock().unwrap().as_ref().unwrap()) * (*b.lock().unwrap().as_ref().unwrap()))));
}