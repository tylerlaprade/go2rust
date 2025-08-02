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
    println!("{} {}", "Current x:".to_string(), (*x.lock().unwrap().as_mut().unwrap()));
}

pub fn defer_in_loop() {
    println!("{}", "Defer in loop:".to_string());
    let mut i = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_mut().unwrap()) < 3 {
        // defer (i.clone()) // TODO: defer not yet supported
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
    while (*i.lock().unwrap().as_mut().unwrap()) < 3 {
        print!("Working... {}\n", (*i.lock().unwrap().as_mut().unwrap()) + 1);
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