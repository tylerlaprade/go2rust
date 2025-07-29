fn main() {
    println!("{}", "=== Range over slice ===".to_string());
    let mut numbers = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![10, 20, 30, 40, 50])));
    for (i, num) in (*numbers.lock().unwrap().as_ref().unwrap()).iter().enumerate() {
        print!("Index {}: {}\n", (*i.lock().unwrap().as_ref().unwrap()), (*num.lock().unwrap().as_ref().unwrap()));
    }
    println!("{}", "Values only:".to_string());
    for (_, num) in (*numbers.lock().unwrap().as_ref().unwrap()).iter().enumerate() {
        print!("{} ", (*num.lock().unwrap().as_ref().unwrap()));
    }
    println!();
    println!("{}", "Indices only:".to_string());
    for i in 0..(*numbers.lock().unwrap().as_ref().unwrap()).len() {
        print!("{} ", (*i.lock().unwrap().as_ref().unwrap()));
    }
    println!();
    println!("{}", "\n=== Range over array ===".to_string());
    let mut arr = std::sync::Arc::new(std::sync::Mutex::new(Some(["apple".to_string(), "banana".to_string(), "cherry".to_string(), "date".to_string()])));
    for (i, fruit) in (*arr.lock().unwrap().as_ref().unwrap()).iter().enumerate() {
        print!("{}: {}\n", (*i.lock().unwrap().as_ref().unwrap()), (*fruit.lock().unwrap().as_ref().unwrap()));
    }
    println!("{}", "\n=== Range over string ===".to_string());
    let mut text = std::sync::Arc::new(std::sync::Mutex::new(Some("Hello, 世界".to_string())));
    for (i, char) in (*text.lock().unwrap().as_ref().unwrap()).iter().enumerate() {
        print!("Byte {}: %c (Unicode: %U)\n", (*i.lock().unwrap().as_ref().unwrap()), (*char.lock().unwrap().as_ref().unwrap()), (*char.lock().unwrap().as_ref().unwrap()));
    }
    println!("{}", "\n=== Range over map ===".to_string());
    let mut ages = std::sync::Arc::new(std::sync::Mutex::new(Some(std::collections::HashMap::<std::sync::Arc<std::sync::Mutex<Option<String>>>, std::sync::Arc<std::sync::Mutex<Option<i32>>>>::from([("Alice".to_string(), 25), ("Bob".to_string(), 30), ("Charlie".to_string(), 35)]))));
    for (name, age) in &(*ages.lock().unwrap().as_ref().unwrap()) {
        print!("{} is {} years old\n", (*name.lock().unwrap().as_ref().unwrap()), (*age.lock().unwrap().as_ref().unwrap()));
    }
    println!("{}", "Keys only:".to_string());
    for (name, _) in &(*(*ages.lock().unwrap().as_ref().unwrap())) {
        print!("{} ", (*name.lock().unwrap().as_ref().unwrap()));
    }
    println!();
    println!("{}", "\n=== Range over channel ===".to_string());
    let mut ch = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![0; 5])));
    let mut i = std::sync::Arc::new(std::sync::Mutex::new(Some(1)));
    while (*i.lock().unwrap().as_ref().unwrap()) <= 5 {
        
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    close(std::sync::Arc::new(std::sync::Mutex::new(Some((*ch.lock().unwrap().as_ref().unwrap())))));
    for value in 0..(*ch.lock().unwrap().as_ref().unwrap()).len() {
        print!("Received: {}\n", (*value.lock().unwrap().as_ref().unwrap()));
    }
    println!("{}", "\n=== Range with break/continue ===".to_string());
    let mut data = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])));
    println!("{}", "Even numbers only (with continue):".to_string());
    for (_, num) in (*data.lock().unwrap().as_ref().unwrap()).iter().enumerate() {
        if (*num.lock().unwrap().as_ref().unwrap()) % 2 != 0 {
        
    }
        print!("{} ", (*num.lock().unwrap().as_ref().unwrap()));
    }
    println!();
    println!("{}", "Numbers until 6 (with break):".to_string());
    for (_, num) in (*data.lock().unwrap().as_ref().unwrap()).iter().enumerate() {
        if (*num.lock().unwrap().as_ref().unwrap()) > 6 {
        
    }
        print!("{} ", (*num.lock().unwrap().as_ref().unwrap()));
    }
    println!();
    println!("{}", "\n=== Nested range loops ===".to_string());
    let mut matrix = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![, , ])));
    for (i, row) in (*matrix.lock().unwrap().as_ref().unwrap()).iter().enumerate() {
        for (j, val) in (*row.lock().unwrap().as_ref().unwrap()).iter().enumerate() {
        print!("matrix[{}][{}] = {}\n", (*i.lock().unwrap().as_ref().unwrap()), (*j.lock().unwrap().as_ref().unwrap()), (*val.lock().unwrap().as_ref().unwrap()));
    }
    }
    println!("{}", "\n=== Range over empty collections ===".to_string());
    let mut emptySlice: std::sync::Arc<std::sync::Mutex<Option<Vec<i32>>>> = Default::default();
    let mut emptyMap;
    println!("{}", "Empty slice:".to_string());
    for (i, v) in (*emptySlice.lock().unwrap().as_ref().unwrap()).iter().enumerate() {
        print!("This won't print: {}, {}\n", (*i.lock().unwrap().as_ref().unwrap()), (*v.lock().unwrap().as_ref().unwrap()));
    }
    println!("{}", "Empty slice range completed".to_string());
    println!("{}", "Empty map:".to_string());
    for (k, v) in &(*emptyMap.lock().unwrap().as_ref().unwrap()) {
        print!("This won't print: {}, {}\n", (*k.lock().unwrap().as_ref().unwrap()), (*v.lock().unwrap().as_ref().unwrap()));
    }
    println!("{}", "Empty map range completed".to_string());
}