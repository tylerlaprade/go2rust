fn main() {
    let mut add = std::sync::Arc::new(std::sync::Mutex::new(Some()));
    println!("{} {}", "add(3, 4) =".to_string(), (*add(std::sync::Arc::new(std::sync::Mutex::new(Some(3))), std::sync::Arc::new(std::sync::Mutex::new(Some(4)))).lock().unwrap().as_mut().unwrap()));
    let mut x = std::sync::Arc::new(std::sync::Mutex::new(Some(10)));
    let mut increment = std::sync::Arc::new(std::sync::Mutex::new(Some()));
    println!("{} {}", "increment() =".to_string(), (*increment().lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "increment() =".to_string(), (*increment().lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "x =".to_string(), (*x.lock().unwrap().as_mut().unwrap()));
    let mut makeMultiplier = std::sync::Arc::new(std::sync::Mutex::new(Some()));
    let mut double = make_multiplier(std::sync::Arc::new(std::sync::Mutex::new(Some(2))));
    let mut triple = make_multiplier(std::sync::Arc::new(std::sync::Mutex::new(Some(3))));
    println!("{} {}", "double(5) =".to_string(), (*double(std::sync::Arc::new(std::sync::Mutex::new(Some(5)))).lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "triple(5) =".to_string(), (*triple(std::sync::Arc::new(std::sync::Mutex::new(Some(5)))).lock().unwrap().as_mut().unwrap()));
    let mut result = (std::sync::Arc::new(std::sync::Mutex::new(Some(4))), std::sync::Arc::new(std::sync::Mutex::new(Some(5))));
    println!("{} {}", "IIFE result =".to_string(), (*result.lock().unwrap().as_mut().unwrap()));
    let mut operations = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![, , ])));
    for (i, op) in (*operations.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        print!("operations[{}](10, 5) = {}\n", i, op(std::sync::Arc::new(std::sync::Mutex::new(Some(10))), std::sync::Arc::new(std::sync::Mutex::new(Some(5)))));
    }
}