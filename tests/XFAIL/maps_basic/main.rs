fn main() {
    let mut ages = std::sync::Arc::new(std::sync::Mutex::new(Some(std::collections::HashMap::<String, std::sync::Arc<std::sync::Mutex<Option<i32>>>>::new())));
    (*ages.lock().unwrap().as_mut().unwrap()).insert("Alice".to_string(), std::sync::Arc::new(std::sync::Mutex::new(Some(25))));
    (*ages.lock().unwrap().as_mut().unwrap()).insert("Bob".to_string(), std::sync::Arc::new(std::sync::Mutex::new(Some(30))));
    (*ages.lock().unwrap().as_mut().unwrap()).insert("Charlie".to_string(), std::sync::Arc::new(std::sync::Mutex::new(Some(35))));
    println!("{} {:?}", "Ages map:".to_string(), (*ages.lock().unwrap().as_mut().unwrap()));
    let mut colors = std::sync::Arc::new(std::sync::Mutex::new(Some(std::collections::HashMap::<String, std::sync::Arc<std::sync::Mutex<Option<String>>>>::from([("red".to_string(), std::sync::Arc::new(std::sync::Mutex::new(Some("#FF0000".to_string())))), ("green".to_string(), std::sync::Arc::new(std::sync::Mutex::new(Some("#00FF00".to_string())))), ("blue".to_string(), std::sync::Arc::new(std::sync::Mutex::new(Some("#0000FF".to_string()))))]))));
    println!("{} {:?}", "Colors map:".to_string(), (*colors.lock().unwrap().as_mut().unwrap()));
    let (mut age, mut exists) = match (*ages.lock().unwrap().as_ref().unwrap()).get(&"Alice".to_string()) { Some(v) => (v.clone(), std::sync::Arc::new(std::sync::Mutex::new(Some(true)))), None => (std::sync::Arc::new(std::sync::Mutex::new(Some(0))), std::sync::Arc::new(std::sync::Mutex::new(Some(false)))) };
    if (*exists.lock().unwrap().as_mut().unwrap()) {
        println!("{} {}", "Alice's age:".to_string(), (*age.lock().unwrap().as_mut().unwrap()));
    }
    (*ages.lock().unwrap().as_mut().unwrap()).remove(&"Bob".to_string());
    println!("{} {:?}", "After deleting Bob:".to_string(), (*ages.lock().unwrap().as_mut().unwrap()));
    println!("{}", "All colors:".to_string());
    for (name, hex) in (*colors.lock().unwrap().as_ref().unwrap()).clone() {
        println!("{} {} {}", name, "->".to_string(), (*hex.lock().unwrap().as_mut().unwrap()));
    }
}