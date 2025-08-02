fn main() {
    let mut m = std::sync::Arc::new(std::sync::Mutex::new(Some(std::collections::HashMap::<String, std::sync::Arc<std::sync::Mutex<Option<i32>>>>::new())));
    (*m.lock().unwrap().as_mut().unwrap()).insert("k1".to_string(), std::sync::Arc::new(std::sync::Mutex::new(Some(7))));
    (*m.lock().unwrap().as_mut().unwrap()).insert("k2".to_string(), std::sync::Arc::new(std::sync::Mutex::new(Some(13))));
    println!("{} {:?}", "map:".to_string(), (*m.lock().unwrap().as_mut().unwrap()));
    let mut v1 = std::sync::Arc::new(std::sync::Mutex::new(Some((*(*m.lock().unwrap().as_ref().unwrap()).get(&"k1".to_string()).unwrap().lock().unwrap().as_ref().unwrap()))));
    println!("{} {}", "v1:".to_string(), (*v1.lock().unwrap().as_mut().unwrap()));
    (*m.lock().unwrap().as_mut().unwrap()).remove(&"k2".to_string());
    println!("{} {:?}", "map:".to_string(), (*m.lock().unwrap().as_mut().unwrap()));
    let (_, mut prs) = match (*m.lock().unwrap().as_ref().unwrap()).get(&"k2".to_string()) { Some(v) => (v.clone(), std::sync::Arc::new(std::sync::Mutex::new(Some(true)))), None => (std::sync::Arc::new(std::sync::Mutex::new(Some(0))), std::sync::Arc::new(std::sync::Mutex::new(Some(false)))) };
    println!("{} {}", "prs:".to_string(), (*prs.lock().unwrap().as_mut().unwrap()));
}