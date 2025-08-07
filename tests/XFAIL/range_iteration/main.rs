use std::collections::HashMap;
use std::sync::{Arc, Mutex};

fn main() {
    let mut nums = Arc::new(Mutex::new(Some(vec![2, 3, 4])));
    let mut sum = Arc::new(Mutex::new(Some(0)));
    for num in &(*nums.lock().unwrap().as_mut().unwrap()) {
        { let mut guard = sum.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + num); };
    }
    println!("{} {}", "sum:".to_string(), (*sum.lock().unwrap().as_mut().unwrap()));

    for (i, num) in (*nums.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        if num == 3 {
        println!("{} {}", "index:".to_string(), i);
    }
    }

    let mut kvs = Arc::new(Mutex::new(Some(HashMap::<String, Arc<Mutex<Option<String>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some("apple".to_string())))), ("b".to_string(), Arc::new(Mutex::new(Some("banana".to_string()))))]))));
    for (k, v) in (*kvs.lock().unwrap().as_ref().unwrap()).clone() {
        print!("{} -> {}\n", k, (*v.lock().unwrap().as_mut().unwrap()));
    }
}