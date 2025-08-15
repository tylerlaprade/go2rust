use std::sync::{Arc, Mutex};

fn main() {
        // Simple slice range
    let mut numbers = Arc::new(Mutex::new(Some(vec![10, 20, 30])));

        // Index and value
    for (i, num) in (*numbers.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        println!("{} {} {} {}", "Index:".to_string(), i, "Value:".to_string(), num);
    }

        // Value only
    for num in &(*numbers.lock().unwrap().as_mut().unwrap()) {
        println!("{} {}", "Value:".to_string(), num);
    }

        // Index only
    for i in 0..(*numbers.lock().unwrap().as_mut().unwrap()).len() {
        println!("{} {}", "Index:".to_string(), i);
    }
}