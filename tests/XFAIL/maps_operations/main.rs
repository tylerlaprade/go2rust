fn main() {
    let mut m = std::collections::HashMap::<std::sync::Arc<std::sync::Mutex<Option<String>>>, std::sync::Arc<std::sync::Mutex<Option<i32>>>>::new();
    (*m.lock().unwrap().as_mut().unwrap())["k1".to_string()] = 7;
    (*m.lock().unwrap().as_mut().unwrap())["k2".to_string()] = 13;
    println!("{} {}", "map:".to_string(), (*m.lock().unwrap().as_ref().unwrap()));
    let mut v1 = std::sync::Arc::new(std::sync::Mutex::new(Some((*m.lock().unwrap().as_ref().unwrap())["k1".to_string()])));
    println!("{} {}", "v1:".to_string(), (*v1.lock().unwrap().as_ref().unwrap()));
    (*m.lock().unwrap().as_ref().unwrap()).remove(&"k2".to_string());
    println!("{} {}", "map:".to_string(), (*m.lock().unwrap().as_ref().unwrap()));
    let (_, mut prs) = (*m.lock().unwrap().as_ref().unwrap())["k2".to_string()];
    println!("{} {}", "prs:".to_string(), (*prs.lock().unwrap().as_ref().unwrap()));
}