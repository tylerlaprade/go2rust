fn main() {
    let mut p;
    if (*p.lock().unwrap().as_mut().unwrap()).is_none() {
        println!("{}", "p is nil".to_string());
    }
    let mut q = std::sync::Arc::new(std::sync::Mutex::new(Some(None)));
    if (*q.lock().unwrap().as_mut().unwrap()).is_none() {
        println!("{}", "q is nil".to_string());
    }
    let mut x = std::sync::Arc::new(std::sync::Mutex::new(Some(42)));
    { let new_val = x.clone(); *p.lock().unwrap() = Some(new_val); };
    if (*p.lock().unwrap().as_mut().unwrap()).is_some() {
        println!("{} {}", "p is not nil, value:".to_string(), (*p.lock().unwrap().as_mut().unwrap()));
    }
    { let new_val = None; *p.lock().unwrap() = Some(new_val); };
    if (*p.lock().unwrap().as_mut().unwrap()).is_none() {
        println!("{}", "p is nil again".to_string());
    }
}