fn format_map<K: std::fmt::Display + std::cmp::Ord + Clone, V>(map: &std::sync::Arc<std::sync::Mutex<Option<std::collections::HashMap<K, std::sync::Arc<std::sync::Mutex<Option<V>>>>>>>) -> String 
where
    V: std::fmt::Display,
{
    let guard = map.lock().unwrap();
    if let Some(ref m) = *guard {
        let mut items: Vec<_> = m.iter().collect();
        items.sort_by_key(|(k, _)| (*k).clone());
        
        let formatted: Vec<String> = items
            .into_iter()
            .map(|(k, v)| {
                let v_guard = v.lock().unwrap();
                if let Some(ref val) = *v_guard {
                    format!("{}:{}", k, val)
                } else {
                    format!("{}:<nil>", k)
                }
            })
            .collect();
        
        format!("map[{}]", formatted.join(" "))
    } else {
        "map[]".to_string()
    }
}
fn format_slice<T>(slice: &std::sync::Arc<std::sync::Mutex<Option<Vec<T>>>>) -> String 
where
    T: std::fmt::Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

pub fn safe_divide(a: std::sync::Arc<std::sync::Mutex<Option<f64>>>, b: std::sync::Arc<std::sync::Mutex<Option<f64>>>) -> (std::sync::Arc<std::sync::Mutex<Option<f64>>>, std::sync::Arc<std::sync::Mutex<Option<Box<dyn std::error::Error + Send + Sync>>>>) {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut result: std::sync::Arc<std::sync::Mutex<Option<f64>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(0.0)));
    let mut err: std::sync::Arc<std::sync::Mutex<Option<Box<dyn std::error::Error + Send + Sync>>>> = std::sync::Arc::new(std::sync::Mutex::new(None));

    __defer_stack.push(Box::new(move || {
        (std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(move || {
        let mut r = recover();
    if (*r.lock().unwrap()).is_some() {
        { let new_val = std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(format!("panic occurred: {}", (*r.lock().unwrap().as_mut().unwrap()))) as Box<dyn std::error::Error + Send + Sync>))); *err.lock().unwrap() = Some(new_val); };
        { let new_val = 0; *result.lock().unwrap() = Some(new_val); };
    }
    }) as Box<dyn Fn() -> () + Send + Sync>))).lock().unwrap().as_ref().unwrap())();
    }));

    if (*b.lock().unwrap().as_mut().unwrap()) == 0 {
        panic(std::sync::Arc::new(std::sync::Mutex::new(Some("division by zero".to_string()))));
    }

    { let new_val = (*a.lock().unwrap().as_mut().unwrap()) / (*b.lock().unwrap().as_mut().unwrap()); *result.lock().unwrap() = Some(new_val); };
    {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}

pub fn process_slice(slice: std::sync::Arc<std::sync::Mutex<Option<Vec<i32>>>>, index: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> (std::sync::Arc<std::sync::Mutex<Option<i32>>>, std::sync::Arc<std::sync::Mutex<Option<Box<dyn std::error::Error + Send + Sync>>>>) {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut value: std::sync::Arc<std::sync::Mutex<Option<i32>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    let mut err: std::sync::Arc<std::sync::Mutex<Option<Box<dyn std::error::Error + Send + Sync>>>> = std::sync::Arc::new(std::sync::Mutex::new(None));

    __defer_stack.push(Box::new(move || {
        (std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(move || {
        let mut r = recover();
    if (*r.lock().unwrap()).is_some() {
        { let new_val = std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(format!("index out of bounds: {}", (*r.lock().unwrap().as_mut().unwrap()))) as Box<dyn std::error::Error + Send + Sync>))); *err.lock().unwrap() = Some(new_val); };
        { let new_val = -1; *value.lock().unwrap() = Some(new_val); };
    }
    }) as Box<dyn Fn() -> () + Send + Sync>))).lock().unwrap().as_ref().unwrap())();
    }));

    { let new_val = (*slice.lock().unwrap().as_mut().unwrap())[(*index.lock().unwrap().as_mut().unwrap())]; *value.lock().unwrap() = Some(new_val); };
    {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}

pub fn nested_panic() {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    __defer_stack.push(Box::new(move || {
        (std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(move || {
        let mut r = recover();
    if (*r.lock().unwrap()).is_some() {
        print!("Recovered from nested panic: {}\n", (*r.lock().unwrap().as_mut().unwrap()));
    }
    }) as Box<dyn Fn() -> () + Send + Sync>))).lock().unwrap().as_ref().unwrap())();
    }));

    (std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(move || {
        __defer_stack.push(Box::new(move || {
        (std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(move || {
        let mut r = recover();
    if (*r.lock().unwrap()).is_some() {
        print!("Inner recovery: {}\n", (*r.lock().unwrap().as_mut().unwrap()));
        panic(std::sync::Arc::new(std::sync::Mutex::new(Some("re-panicking from inner function".to_string()))));
    }
    }) as Box<dyn Fn() -> () + Send + Sync>))).lock().unwrap().as_ref().unwrap())();
    }));
        panic(std::sync::Arc::new(std::sync::Mutex::new(Some("original panic".to_string()))));
    }) as Box<dyn Fn() -> () + Send + Sync>))).lock().unwrap().as_ref().unwrap())();

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}

pub fn demonstrate_panic_types() {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    __defer_stack.push(Box::new(move || {
        (std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(move || {
        let mut r = recover();
    if (*r.lock().unwrap()).is_some() {
        print!("Recovered string panic: {}\n", (*r.lock().unwrap().as_mut().unwrap()));
    }
    }) as Box<dyn Fn() -> () + Send + Sync>))).lock().unwrap().as_ref().unwrap())();
    }));

    __defer_stack.push(Box::new(move || {
        (std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(move || {
        panic(std::sync::Arc::new(std::sync::Mutex::new(Some("string panic message".to_string()))));
    }) as Box<dyn Fn() -> () + Send + Sync>))).lock().unwrap().as_ref().unwrap())();
    }));

    __defer_stack.push(Box::new(move || {
        (std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(move || {
        panic(std::sync::Arc::new(std::sync::Mutex::new(Some(42))));
    }) as Box<dyn Fn() -> () + Send + Sync>))).lock().unwrap().as_ref().unwrap())();
    }));

    __defer_stack.push(Box::new(move || {
        (std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(move || {
        panic(std::sync::Arc::new(std::sync::Mutex::new(Some(std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(format!("error panic")) as Box<dyn std::error::Error + Send + Sync>)))))));
    }) as Box<dyn Fn() -> () + Send + Sync>))).lock().unwrap().as_ref().unwrap())();
    }));

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}

pub fn chained_defers() {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    __defer_stack.push(Box::new(move || {
        (std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(move || {
        let mut r = recover();
    if (*r.lock().unwrap()).is_some() {
        print!("Final recovery: {}\n", (*r.lock().unwrap().as_mut().unwrap()));
    }
    }) as Box<dyn Fn() -> () + Send + Sync>))).lock().unwrap().as_ref().unwrap())();
    }));

    __defer_stack.push(Box::new(move || {
        (std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(move || {
        println!("{}", "Defer 1: This runs".to_string());
    }) as Box<dyn Fn() -> () + Send + Sync>))).lock().unwrap().as_ref().unwrap())();
    }));

    __defer_stack.push(Box::new(move || {
        (std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(move || {
        println!("{}", "Defer 2: This also runs".to_string());
        panic(std::sync::Arc::new(std::sync::Mutex::new(Some("panic from defer".to_string()))));
    }) as Box<dyn Fn() -> () + Send + Sync>))).lock().unwrap().as_ref().unwrap())();
    }));

    __defer_stack.push(Box::new(move || {
        (std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(move || {
        println!("{}", "Defer 3: This runs first".to_string());
    }) as Box<dyn Fn() -> () + Send + Sync>))).lock().unwrap().as_ref().unwrap())();
    }));

    println!("{}", "About to return normally".to_string());

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
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