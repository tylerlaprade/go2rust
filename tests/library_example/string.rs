pub fn repeat(s: std::sync::Arc<std::sync::Mutex<Option<String>>>, n: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<String>>> {

    let mut result = std::sync::Arc::new(std::sync::Mutex::new(Some("".to_string())));
    let mut i = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_mut().unwrap()) < (*n.lock().unwrap().as_mut().unwrap()) {
        (*result.lock().unwrap().as_mut().unwrap()).push_str(&(*s.lock().unwrap().as_mut().unwrap()));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return std::sync::Arc::new(std::sync::Mutex::new(Some((*result.lock().unwrap().as_mut().unwrap()).clone())));
}