pub fn safe_divide(a: std::sync::Arc<std::sync::Mutex<Option<f64>>>, b: std::sync::Arc<std::sync::Mutex<Option<f64>>>) -> (std::sync::Arc<std::sync::Mutex<Option<f64>>>, std::sync::Arc<std::sync::Mutex<Option<Box<dyn std::error::Error + Send + Sync>>>>) {
    let mut result: std::sync::Arc<std::sync::Mutex<Option<f64>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(0.0)));
    let mut err: std::sync::Arc<std::sync::Mutex<Option<Box<dyn std::error::Error + Send + Sync>>>> = std::sync::Arc::new(std::sync::Mutex::new(None));

    // defer () // TODO: defer not yet supported
    if (*b.lock().unwrap().as_mut().unwrap()) == 0 {
        panic(std::sync::Arc::new(std::sync::Mutex::new(Some("division by zero".to_string()))));
    }
    { let new_val = (*a.lock().unwrap().as_mut().unwrap()) / (*b.lock().unwrap().as_mut().unwrap()); *result.lock().unwrap() = Some(new_val); };
    return (std::sync::Arc::new(std::sync::Mutex::new(Some((*result.lock().unwrap().as_mut().unwrap()).clone()))), std::sync::Arc::new(std::sync::Mutex::new(None)));
}

pub fn process_slice(slice: std::sync::Arc<std::sync::Mutex<Option<Vec<i32>>>>, index: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> (std::sync::Arc<std::sync::Mutex<Option<i32>>>, std::sync::Arc<std::sync::Mutex<Option<Box<dyn std::error::Error + Send + Sync>>>>) {
    let mut value: std::sync::Arc<std::sync::Mutex<Option<i32>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    let mut err: std::sync::Arc<std::sync::Mutex<Option<Box<dyn std::error::Error + Send + Sync>>>> = std::sync::Arc::new(std::sync::Mutex::new(None));

    // defer () // TODO: defer not yet supported
    { let new_val = (*slice.lock().unwrap().as_mut().unwrap())[(*index.lock().unwrap().as_mut().unwrap())]; *value.lock().unwrap() = Some(new_val); };
    return (std::sync::Arc::new(std::sync::Mutex::new(Some((*value.lock().unwrap().as_mut().unwrap()).clone()))), std::sync::Arc::new(std::sync::Mutex::new(None)));
}

pub fn nested_panic() {
    // defer () // TODO: defer not yet supported
    ();
}

pub fn demonstrate_panic_types() {
    // defer () // TODO: defer not yet supported
    // defer () // TODO: defer not yet supported
    // defer () // TODO: defer not yet supported
    // defer () // TODO: defer not yet supported
}

pub fn chained_defers() {
    // defer () // TODO: defer not yet supported
    // defer () // TODO: defer not yet supported
    // defer () // TODO: defer not yet supported
    // defer () // TODO: defer not yet supported
    println!("{}", "About to return normally".to_string());
}

fn main() {
    println!("{}", "=== Safe divide examples ===".to_string());
    let (mut result, mut err) = safe_divide(std::sync::Arc::new(std::sync::Mutex::new(Some(10))), std::sync::Arc::new(std::sync::Mutex::new(Some(2))));
    if (*err.lock().unwrap()).is_some() {
        print!("Error: {}\n", (*err.lock().unwrap().as_mut().unwrap()));
    } else {
        print!("10 / 2 = {:.2}\n", (*result.lock().unwrap().as_mut().unwrap()));
    }
    (result, err) = safe_divide(std::sync::Arc::new(std::sync::Mutex::new(Some(10))), std::sync::Arc::new(std::sync::Mutex::new(Some(0))));
    if (*err.lock().unwrap()).is_some() {
        print!("Error: {}\n", (*err.lock().unwrap().as_mut().unwrap()));
    } else {
        print!("Result: {:.2}\n", (*result.lock().unwrap().as_mut().unwrap()));
    }
    println!("{}", "\n=== Slice access examples ===".to_string());
    let mut numbers = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![1, 2, 3, 4, 5])));
    let (mut value, mut err) = process_slice(numbers.clone(), std::sync::Arc::new(std::sync::Mutex::new(Some(2))));
    if (*err.lock().unwrap()).is_some() {
        print!("Error: {}\n", (*err.lock().unwrap().as_mut().unwrap()));
    } else {
        print!("numbers[2] = {}\n", (*value.lock().unwrap().as_mut().unwrap()));
    }
    (value, err) = process_slice(numbers.clone(), std::sync::Arc::new(std::sync::Mutex::new(Some(10))));
    if (*err.lock().unwrap()).is_some() {
        print!("Error: {}\n", (*err.lock().unwrap().as_mut().unwrap()));
    } else {
        print!("Value: {}\n", (*value.lock().unwrap().as_mut().unwrap()));
    }
    println!("{}", "\n=== Nested panic example ===".to_string());
    nested_panic();
    println!("{}", "\n=== Different panic types ===".to_string());
    demonstrate_panic_types();
    println!("{}", "\n=== Chained defers with panic ===".to_string());
    chained_defers();
    println!("{}", "\n=== Program completed successfully ===".to_string());
}