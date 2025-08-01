fn main() {
    let mut p: std::sync::Arc<std::sync::Mutex<Option<i32>>> = std::sync::Arc::new(std::sync::Mutex::new(None));
    if (*p.lock().unwrap()).is_none() {
        println!("{}", "p is nil".to_string());
    }
    let mut q: std::sync::Arc<std::sync::Mutex<Option<String>>> = std::sync::Arc::new(std::sync::Mutex::new(None));
    if (*q.lock().unwrap()).is_none() {
        println!("{}", "q is nil".to_string());
    }
    let mut x = std::sync::Arc::new(std::sync::Mutex::new(Some(42)));
    { let new_val = (*x.lock().unwrap()).clone(); *p.lock().unwrap() = new_val; };
    if (*p.lock().unwrap()).is_some() {
        println!("{} {}", "p is not nil, value:".to_string(), (*p.lock().unwrap().as_mut().unwrap()));
    }
    *p.lock().unwrap() = None;
    if (*p.lock().unwrap()).is_none() {
        println!("{}", "p is nil again".to_string());
    }
}