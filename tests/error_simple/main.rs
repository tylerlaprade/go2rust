pub fn divide(a: std::sync::Arc<std::sync::Mutex<Option<i32>>>, b: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> (std::sync::Arc<std::sync::Mutex<Option<i32>>>, std::sync::Arc<std::sync::Mutex<Option<Box<dyn std::error::Error + Send + Sync>>>>) {

    if (*b.lock().unwrap().as_mut().unwrap()) == 0 {
        return (std::sync::Arc::new(std::sync::Mutex::new(Some(0))), std::sync::Arc::new(std::sync::Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("division by zero".to_string())))));
    }
    return (std::sync::Arc::new(std::sync::Mutex::new(Some((*a.lock().unwrap().as_mut().unwrap()) / (*b.lock().unwrap().as_mut().unwrap())))), std::sync::Arc::new(std::sync::Mutex::new(None)));
}

fn main() {
    let (mut result, mut err) = divide(std::sync::Arc::new(std::sync::Mutex::new(Some(10))), std::sync::Arc::new(std::sync::Mutex::new(Some(2))));
    if (*err.lock().unwrap()).is_some() {
        println!("{} {}", "Error:".to_string(), (*err.lock().unwrap().as_mut().unwrap()));
    } else {
        println!("{} {}", "Result:".to_string(), (*result.lock().unwrap().as_mut().unwrap()));
    }
    (result, err) = divide(std::sync::Arc::new(std::sync::Mutex::new(Some(10))), std::sync::Arc::new(std::sync::Mutex::new(Some(0))));
    if (*err.lock().unwrap()).is_some() {
        println!("{} {}", "Error:".to_string(), (*err.lock().unwrap().as_mut().unwrap()));
    } else {
        println!("{} {}", "Result:".to_string(), (*result.lock().unwrap().as_mut().unwrap()));
    }
}