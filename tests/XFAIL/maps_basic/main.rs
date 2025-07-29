fn main() {
    let mut ages = std::sync::Arc::new(std::sync::Mutex::new(Some(std::collections::HashMap::<std::sync::Arc<std::sync::Mutex<Option<String>>>, std::sync::Arc<std::sync::Mutex<Option<i32>>>>::new())));
    (*ages.lock().unwrap().as_ref().unwrap()).insert("Alice".to_string(), 25);
    (*ages.lock().unwrap().as_ref().unwrap()).insert("Bob".to_string(), 30);
    (*ages.lock().unwrap().as_ref().unwrap()).insert("Charlie".to_string(), 35);
    println!("{} {:?}", "Ages map:".to_string(), (*ages.lock().unwrap().as_ref().unwrap()));
    let mut colors = std::sync::Arc::new(std::sync::Mutex::new(Some(std::collections::HashMap::<std::sync::Arc<std::sync::Mutex<Option<String>>>, std::sync::Arc<std::sync::Mutex<Option<String>>>>::from([("red".to_string(), "#FF0000".to_string()), ("green".to_string(), "#00FF00".to_string()), ("blue".to_string(), "#0000FF".to_string())]))));
    println!("{} {:?}", "Colors map:".to_string(), (*colors.lock().unwrap().as_ref().unwrap()));
    let (mut (*age.lock().unwrap().as_ref().unwrap()), mut (*exists.lock().unwrap().as_ref().unwrap())) = ((*ages.lock().unwrap().as_ref().unwrap()).get(&"Alice".to_string()).cloned().unwrap_or_default(), (*ages.lock().unwrap().as_ref().unwrap()).contains_key(&"Alice".to_string()));
    if (*exists.lock().unwrap().as_ref().unwrap()) {
        println!("{} {}", "Alice's age:".to_string(), (*age.lock().unwrap().as_ref().unwrap()));
    }
    (*ages.lock().unwrap().as_ref().unwrap()).remove(&"Bob".to_string());
    println!("{} {:?}", "After deleting Bob:".to_string(), (*ages.lock().unwrap().as_ref().unwrap()));
    println!("{}", "All colors:".to_string());
    for (name, hex) in &(*colors.lock().unwrap().as_ref().unwrap()) {
        println!("{} {} {}", (*name.lock().unwrap().as_ref().unwrap()), "->".to_string(), (*hex.lock().unwrap().as_ref().unwrap()));
    }
}