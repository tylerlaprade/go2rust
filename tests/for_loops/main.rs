fn main() {
    let mut i = std::sync::Arc::new(std::sync::Mutex::new(Some(1)));
    while (*i.lock().unwrap().as_ref().unwrap()) <= 3 {
        println!("{}", (*i.lock().unwrap().as_ref().unwrap()));
        { let new_val = (*i.lock().unwrap().as_ref().unwrap()) + 1; *i.lock().unwrap() = Some(new_val); };
    }
    let mut j = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    while (*j.lock().unwrap().as_ref().unwrap()) < 3 {
        println!("{}", (*j.lock().unwrap().as_ref().unwrap()));
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    while true {
        println!("{}", "loop".to_string());
        break
    }
}