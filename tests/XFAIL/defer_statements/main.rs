pub fn defer_example() {
    println!("{}", "Start of function".to_string());
    // defer println!("{}", "Deferred 1".to_string()) // TODO: defer not yet supported
    // defer println!("{}", "Deferred 2".to_string()) // TODO: defer not yet supported
    // defer println!("{}", "Deferred 3".to_string()) // TODO: defer not yet supported
    println!("{}", "Middle of function".to_string());
    // defer () // TODO: defer not yet supported
    println!("{}", "End of function".to_string());
}

pub fn defer_with_variables() {
    let mut x = std::sync::Arc::new(std::sync::Mutex::new(Some(10)));
    // defer () // TODO: defer not yet supported
    { let new_val = 20; *x.lock().unwrap() = Some(new_val); };
    println!("{} {}", "Current x:".to_string(), (*x.lock().unwrap().as_ref().unwrap()));
}

pub fn defer_in_loop() {
    println!("{}", "Defer in loop:".to_string());
    let mut i = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_ref().unwrap()) < 3 {
        // defer (std::sync::Arc::new(std::sync::Mutex::new(Some((*i.lock().unwrap().as_ref().unwrap()))))) // TODO: defer not yet supported
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    println!("{}", "Loop finished".to_string());
}

pub fn cleanup() {
    println!("{}", "Cleanup function called".to_string());
}

pub fn resource_example() {
    println!("{}", "Acquiring resource".to_string());
    // defer cleanup() // TODO: defer not yet supported
    println!("{}", "Using resource".to_string());
    let mut i = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_ref().unwrap()) < 3 {
        print!("Working... {}\n", (*i.lock().unwrap().as_ref().unwrap()) + 1);
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    println!("{}", "Done with resource".to_string());
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