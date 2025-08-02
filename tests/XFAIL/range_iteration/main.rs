fn main() {
    let mut nums = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![2, 3, 4])));
    let mut sum = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    for num in &(*nums.lock().unwrap().as_mut().unwrap()) {
        { let mut guard = sum.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + num); };
    }
    println!("{} {}", "sum:".to_string(), (*sum.lock().unwrap().as_mut().unwrap()));
    for (i, num) in (*nums.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        if num == 3 {
        println!("{} {}", "index:".to_string(), i);
    }
    }
    let mut kvs = std::sync::Arc::new(std::sync::Mutex::new(Some(std::collections::HashMap::<String, std::sync::Arc<std::sync::Mutex<Option<String>>>>::from([("a".to_string(), std::sync::Arc::new(std::sync::Mutex::new(Some("apple".to_string())))), ("b".to_string(), std::sync::Arc::new(std::sync::Mutex::new(Some("banana".to_string()))))]))));
    for (k, v) in (*kvs.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        print!("{} -> {}\n", k, v);
    }
}