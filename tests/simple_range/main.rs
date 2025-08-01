fn main() {
    let mut numbers = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![10, 20, 30])));
    for (i, num) in (*numbers.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        println!("{} {} {} {}", "Index:".to_string(), i, "Value:".to_string(), num);
    }
    for num in &(*numbers.lock().unwrap().as_mut().unwrap()) {
        println!("{} {}", "Value:".to_string(), num);
    }
    for i in 0..(*numbers.lock().unwrap().as_mut().unwrap()).len() {
        println!("{} {}", "Index:".to_string(), i);
    }
}