use std::collections::HashMap;
use std::sync::{Arc, Mutex};

fn main() {
        // Range over slice
    println!("{}", "=== Range over slice ===".to_string());
    let mut numbers = Arc::new(Mutex::new(Some(vec![10, 20, 30, 40, 50])));

        // With index and value
    for (i, num) in (*numbers.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        print!("Index {}: {}\n", i, num);
    }

        // Only value
    println!("{}", "Values only:".to_string());
    for num in &(*numbers.lock().unwrap().as_mut().unwrap()) {
        print!("{} ", num);
    }
    println!();

        // Only index
    println!("{}", "Indices only:".to_string());
    for i in 0..(*numbers.lock().unwrap().as_mut().unwrap()).len() {
        print!("{} ", i);
    }
    println!();

        // Range over array
    println!("{}", "\n=== Range over array ===".to_string());
    let mut arr = Arc::new(Mutex::new(Some(["apple".to_string(), "banana".to_string(), "cherry".to_string(), "date".to_string()])));
    for (i, fruit) in (*arr.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        print!("{}: {}\n", i, fruit);
    }

        // Range over string
    println!("{}", "\n=== Range over string ===".to_string());
    let mut text = Arc::new(Mutex::new(Some("Hello, 世界".to_string())));
    for (i, char) in (*(*text.lock().unwrap().as_mut().unwrap()).lock().unwrap().as_ref().unwrap()).chars().enumerate() {
        print!("Byte {}: {} (Unicode: {:?})\n", i, char, char);
    }

        // Range over map
    println!("{}", "\n=== Range over map ===".to_string());
    let mut ages = Arc::new(Mutex::new(Some(HashMap::<String, Arc<Mutex<Option<i32>>>>::from([("Alice".to_string(), Arc::new(Mutex::new(Some(25)))), ("Bob".to_string(), Arc::new(Mutex::new(Some(30)))), ("Charlie".to_string(), Arc::new(Mutex::new(Some(35))))]))));

    for (name, age) in (*ages.lock().unwrap().as_ref().unwrap()).clone() {
        print!("{} is {} years old\n", name, (*age.lock().unwrap().as_mut().unwrap()));
    }

        // Only keys
    println!("{}", "Keys only:".to_string());
    for (name, _) in (*ages.lock().unwrap().as_ref().unwrap()).clone() {
        print!("{} ", name);
    }
    println!();

        // Range over channel
    println!("{}", "\n=== Range over channel ===".to_string());
    let mut ch = ;

        // Send some values
    let mut i = Arc::new(Mutex::new(Some(1)));
    while (*i.lock().unwrap().as_mut().unwrap()) <= 5 {
        // TODO: Unhandled statement type: SendStmt
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    (close.lock().unwrap().as_ref().unwrap())(ch.clone());

        // Range over closed channel
    for value in 0..(*ch.lock().unwrap().as_mut().unwrap()).len() {
        print!("Received: {}\n", value);
    }

        // Range with break and continue
    println!("{}", "\n=== Range with break/continue ===".to_string());
    let mut data = Arc::new(Mutex::new(Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])));

    println!("{}", "Even numbers only (with continue):".to_string());
    for num in &(*data.lock().unwrap().as_mut().unwrap()) {
        if num % 2 != 0 {
        continue
    }
        print!("{} ", num);
    }
    println!();

    println!("{}", "Numbers until 6 (with break):".to_string());
    for num in &(*data.lock().unwrap().as_mut().unwrap()) {
        if num > 6 {
        break
    }
        print!("{} ", num);
    }
    println!();

        // Nested range loops
    println!("{}", "\n=== Nested range loops ===".to_string());
    let mut matrix = Arc::new(Mutex::new(Some(vec![, , ])));

    for (i, row) in (*matrix.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
        print!("matrix[{}][{}] = {}\n", i, j, val);
    }
    }

        // Range over empty collections
    println!("{}", "\n=== Range over empty collections ===".to_string());
    let mut emptySlice: Arc<Mutex<Option<Vec<i32>>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut emptyMap: Arc<Mutex<Option<HashMap<String, i32>>>>;

    println!("{}", "Empty slice:".to_string());
    for (i, v) in (*emptySlice.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        print!("This won't print: {}, {}\n", i, v);
    }
    println!("{}", "Empty slice range completed".to_string());

    println!("{}", "Empty map:".to_string());
    for (k, v) in (*emptyMap.lock().unwrap().as_ref().unwrap()).clone() {
        print!("This won't print: {}, {}\n", k, (*v.lock().unwrap().as_mut().unwrap()));
    }
    println!("{}", "Empty map range completed".to_string());
}