pub fn print_any(v: std::sync::Arc<std::sync::Mutex<Option<Box<dyn std::any::Any>>>>) {
    println!("{} {}", "Value:".to_string(), (*v.lock().unwrap().as_mut().unwrap()));
}

fn main() {
    let mut x: std::sync::Arc<std::sync::Mutex<Option<Box<dyn std::any::Any>>>>;
    { let new_val = 42; *x.lock().unwrap() = Some(new_val); };
    println!("{} {}", "x is int:".to_string(), (*x.lock().unwrap().as_mut().unwrap()));
    print_any(x.clone());
    { let new_val = "hello".to_string(); *x.lock().unwrap() = Some(new_val); };
    println!("{} {}", "x is string:".to_string(), (*x.lock().unwrap().as_mut().unwrap()));
    print_any(x.clone());
    { let new_val = 3.14; *x.lock().unwrap() = Some(new_val); };
    println!("{} {}", "x is float:".to_string(), (*x.lock().unwrap().as_mut().unwrap()));
    print_any(x.clone());
    let mut values = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![1, "two".to_string(), 3.0])));
    println!("{} {}", "Mixed values:".to_string(), (*values.lock().unwrap().as_mut().unwrap()));
}