use std::sync::{Arc, Mutex};

fn main() {
        // Mixed output to stdout and stderr
    println!("{}", "=== Mixed Output Test ===".to_string());

        // Standard output
    println!("{}", "This goes to stdout via fmt.Println".to_string());
    print!("This goes to stdout via fmt.Printf: {}\n", 42);

        // Standard error
    fmt.fprintln(Arc::new(Mutex::new(Some((*os.lock().unwrap().as_mut().unwrap())::stderr))), Arc::new(Mutex::new(Some("This goes to stderr via fmt.Fprintln".to_string()))));
    fmt.fprintf(Arc::new(Mutex::new(Some((*os.lock().unwrap().as_mut().unwrap())::stderr))), Arc::new(Mutex::new(Some("This goes to stderr via fmt.Fprintf: %s\n".to_string()))), Arc::new(Mutex::new(Some("error message".to_string()))));

        // Built-in println (goes to stderr)
    eprintln!("{}", "This goes to stderr via built-in println".to_string());

        // More mixed output
    println!("{}", "Back to stdout".to_string());
    fmt.fprintln(Arc::new(Mutex::new(Some((*os.lock().unwrap().as_mut().unwrap())::stderr))), Arc::new(Mutex::new(Some("Back to stderr".to_string()))));

        // Multiple values
    println!("{} {} {} {}", "Multiple".to_string(), "values".to_string(), "to".to_string(), "stdout".to_string());
    fmt.fprintln(Arc::new(Mutex::new(Some((*os.lock().unwrap().as_mut().unwrap())::stderr))), Arc::new(Mutex::new(Some("Multiple".to_string()))), Arc::new(Mutex::new(Some("values".to_string()))), Arc::new(Mutex::new(Some("to".to_string()))), Arc::new(Mutex::new(Some("stderr".to_string()))));

        // Numbers and strings mixed
    print!("Number: {}, String: {}, Float: {:.2}\n", 123, "hello".to_string(), 3.14);
    fmt.fprintf(Arc::new(Mutex::new(Some((*os.lock().unwrap().as_mut().unwrap())::stderr))), Arc::new(Mutex::new(Some("Error code: %d, Message: %s\n".to_string()))), Arc::new(Mutex::new(Some(404))), Arc::new(Mutex::new(Some("Not Found".to_string()))));

        // Final messages
    println!("{}", "Program completed successfully".to_string());
    fmt.fprintln(Arc::new(Mutex::new(Some((*os.lock().unwrap().as_mut().unwrap())::stderr))), Arc::new(Mutex::new(Some("No errors occurred".to_string()))));
}