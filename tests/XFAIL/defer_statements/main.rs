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

pub fn defer_example() {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    println!("{}", "Start of function".to_string());

    __defer_stack.push(Box::new(move || {
        println!("{}", "Deferred 1".to_string());
    }));
    __defer_stack.push(Box::new(move || {
        println!("{}", "Deferred 2".to_string());
    }));
    __defer_stack.push(Box::new(move || {
        println!("{}", "Deferred 3".to_string());
    }));

    println!("{}", "Middle of function".to_string());

    __defer_stack.push(Box::new(move || {
        (std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(move || {
        println!("{}", "Anonymous deferred function".to_string());
    }) as Box<dyn Fn() -> () + Send + Sync>))).lock().unwrap().as_ref().unwrap())();
    }));

    println!("{}", "End of function".to_string());

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}

pub fn defer_with_variables() {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut x = std::sync::Arc::new(std::sync::Mutex::new(Some(10)));
    __defer_stack.push(Box::new(move || {
        (std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(move || {
        println!("{} {}", "Deferred x:".to_string(), (*x.lock().unwrap().as_mut().unwrap()));
    }) as Box<dyn Fn() -> () + Send + Sync>))).lock().unwrap().as_ref().unwrap())();
    }));

    { let new_val = 20; *x.lock().unwrap() = Some(new_val); };
    println!("{} {}", "Current x:".to_string(), (*x.lock().unwrap().as_mut().unwrap()));

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}

pub fn defer_in_loop() {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    println!("{}", "Defer in loop:".to_string());
    let mut i = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_mut().unwrap()) < 3 {
        __defer_stack.push(Box::new(move || {
        (std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(move |val: std::sync::Arc<std::sync::Mutex<Option<i32>>>| {
        print!("Deferred loop value: {}\n", (*val.lock().unwrap().as_mut().unwrap()));
    }) as Box<dyn Fn(std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> () + Send + Sync>))).lock().unwrap().as_ref().unwrap())(i.clone());
    }));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    println!("{}", "Loop finished".to_string());

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}

pub fn cleanup() {
    println!("{}", "Cleanup function called".to_string());
}

pub fn resource_example() {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    println!("{}", "Acquiring resource".to_string());
    __defer_stack.push(Box::new(move || {
        cleanup();
    }));

    println!("{}", "Using resource".to_string());

    let mut i = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_mut().unwrap()) < 3 {
        print!("Working... {}\n", (*i.lock().unwrap().as_mut().unwrap()) + 1);
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    println!("{}", "Done with resource".to_string());

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}

fn main() {
    println!("{}", "=== Basic defer example ===".to_string());
    defer_example();

    println!("{}", "\n=== Defer with variables ===".to_string());
    defer_with_variables();

    println!("{}", "\n=== Defer in loop ===".to_string());
    defer_in_loop();

    println!("{}", "\n=== Resource cleanup example ===".to_string());
    resource_example();

    println!("{}", "\n=== Main function ending ===".to_string());
}