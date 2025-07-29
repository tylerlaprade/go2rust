fn main() {
    println!("{}", "=== Mixed Output Test ===".to_string());
    println!("{}", "This goes to stdout via fmt.Println".to_string());
    print!("This goes to stdout via fmt.Printf: {}\n", 42);
    (*fmt.lock().unwrap().as_ref().unwrap()).fprintln(std::sync::Arc::new(std::sync::Mutex::new(Some((*os.lock().unwrap().as_ref().unwrap()).stderr))), std::sync::Arc::new(std::sync::Mutex::new(Some("This goes to stderr via fmt.Fprintln".to_string()))));
    (*fmt.lock().unwrap().as_ref().unwrap()).fprintf(std::sync::Arc::new(std::sync::Mutex::new(Some((*os.lock().unwrap().as_ref().unwrap()).stderr))), std::sync::Arc::new(std::sync::Mutex::new(Some("This goes to stderr via fmt.Fprintf: %s\n".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some("error message".to_string()))));
    eprintln!("{}", "This goes to stderr via built-in println".to_string());
    println!("{}", "Back to stdout".to_string());
    (*fmt.lock().unwrap().as_ref().unwrap()).fprintln(std::sync::Arc::new(std::sync::Mutex::new(Some((*os.lock().unwrap().as_ref().unwrap()).stderr))), std::sync::Arc::new(std::sync::Mutex::new(Some("Back to stderr".to_string()))));
    println!("{} {} {} {}", "Multiple".to_string(), "values".to_string(), "to".to_string(), "stdout".to_string());
    (*fmt.lock().unwrap().as_ref().unwrap()).fprintln(std::sync::Arc::new(std::sync::Mutex::new(Some((*os.lock().unwrap().as_ref().unwrap()).stderr))), std::sync::Arc::new(std::sync::Mutex::new(Some("Multiple".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some("values".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some("to".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some("stderr".to_string()))));
    print!("Number: {}, String: {}, Float: {:.2}\n", 123, "hello".to_string(), 3.14);
    (*fmt.lock().unwrap().as_ref().unwrap()).fprintf(std::sync::Arc::new(std::sync::Mutex::new(Some((*os.lock().unwrap().as_ref().unwrap()).stderr))), std::sync::Arc::new(std::sync::Mutex::new(Some("Error code: %d, Message: %s\n".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some(404))), std::sync::Arc::new(std::sync::Mutex::new(Some("Not Found".to_string()))));
    println!("{}", "Program completed successfully".to_string());
    (*fmt.lock().unwrap().as_ref().unwrap()).fprintln(std::sync::Arc::new(std::sync::Mutex::new(Some((*os.lock().unwrap().as_ref().unwrap()).stderr))), std::sync::Arc::new(std::sync::Mutex::new(Some("No errors occurred".to_string()))));
}