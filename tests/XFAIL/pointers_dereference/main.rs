pub fn zeroval(ival: std::sync::Arc<std::sync::Mutex<Option<i32>>>) {
    { let new_val = 0; *ival.lock().unwrap() = Some(new_val); };
}

pub fn zeroptr(iptr: std::sync::Arc<std::sync::Mutex<Option<std::sync::Arc<std::sync::Mutex<Option<i32>>>>>>) {
    { let new_val = 0; *iptr.lock().unwrap() = Some(new_val); };
}

fn main() {
    let mut i = std::sync::Arc::new(std::sync::Mutex::new(Some(1)));
    println!("{} {}", "initial:".to_string(), (*i.lock().unwrap().as_mut().unwrap()));
    zeroval(std::sync::Arc::new(std::sync::Mutex::new(Some((*i.lock().unwrap().as_mut().unwrap())))));
    println!("{} {}", "zeroval:".to_string(), (*i.lock().unwrap().as_mut().unwrap()));
    zeroptr(std::sync::Arc::new(std::sync::Mutex::new(Some(i.clone()))));
    println!("{} {}", "zeroptr:".to_string(), (*i.lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "pointer:".to_string(), i.clone());
}