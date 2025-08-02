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

fn main() {
    println!("{}", "=== Mixed Output Test ===".to_string());

    println!("{}", "This goes to stdout via fmt.Println".to_string());
    print!("This goes to stdout via fmt.Printf: {}\n", 42);

    (*fmt.lock().unwrap().as_mut().unwrap()).fprintln(std::sync::Arc::new(std::sync::Mutex::new(Some((*os.lock().unwrap().as_mut().unwrap()).stderr))), std::sync::Arc::new(std::sync::Mutex::new(Some("This goes to stderr via fmt.Fprintln".to_string()))));
    (*fmt.lock().unwrap().as_mut().unwrap()).fprintf(std::sync::Arc::new(std::sync::Mutex::new(Some((*os.lock().unwrap().as_mut().unwrap()).stderr))), std::sync::Arc::new(std::sync::Mutex::new(Some("This goes to stderr via fmt.Fprintf: %s\n".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some("error message".to_string()))));

    eprintln!("{}", "This goes to stderr via built-in println".to_string());

    println!("{}", "Back to stdout".to_string());
    (*fmt.lock().unwrap().as_mut().unwrap()).fprintln(std::sync::Arc::new(std::sync::Mutex::new(Some((*os.lock().unwrap().as_mut().unwrap()).stderr))), std::sync::Arc::new(std::sync::Mutex::new(Some("Back to stderr".to_string()))));

    println!("{} {} {} {}", "Multiple".to_string(), "values".to_string(), "to".to_string(), "stdout".to_string());
    (*fmt.lock().unwrap().as_mut().unwrap()).fprintln(std::sync::Arc::new(std::sync::Mutex::new(Some((*os.lock().unwrap().as_mut().unwrap()).stderr))), std::sync::Arc::new(std::sync::Mutex::new(Some("Multiple".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some("values".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some("to".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some("stderr".to_string()))));

    print!("Number: {}, String: {}, Float: {:.2}\n", 123, "hello".to_string(), 3.14);
    (*fmt.lock().unwrap().as_mut().unwrap()).fprintf(std::sync::Arc::new(std::sync::Mutex::new(Some((*os.lock().unwrap().as_mut().unwrap()).stderr))), std::sync::Arc::new(std::sync::Mutex::new(Some("Error code: %d, Message: %s\n".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some(404))), std::sync::Arc::new(std::sync::Mutex::new(Some("Not Found".to_string()))));

    println!("{}", "Program completed successfully".to_string());
    (*fmt.lock().unwrap().as_mut().unwrap()).fprintln(std::sync::Arc::new(std::sync::Mutex::new(Some((*os.lock().unwrap().as_mut().unwrap()).stderr))), std::sync::Arc::new(std::sync::Mutex::new(Some("No errors occurred".to_string()))));
}