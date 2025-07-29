fn main() {
    println!("{}", "=== Mixed Output Test ===".to_string());
    println!("{}", "This goes to stdout via fmt.Println".to_string());
    print!("This goes to stdout via fmt.Printf: {}\n", 42);
    fmt.fprintln(os.stderr, "This goes to stderr via fmt.Fprintln".to_string());
    fmt.fprintf(os.stderr, "This goes to stderr via fmt.Fprintf: %s\n".to_string(), "error message".to_string());
    eprintln!("{}", "This goes to stderr via built-in println".to_string());
    println!("{}", "Back to stdout".to_string());
    fmt.fprintln(os.stderr, "Back to stderr".to_string());
    println!("{} {} {} {}", "Multiple".to_string(), "values".to_string(), "to".to_string(), "stdout".to_string());
    fmt.fprintln(os.stderr, "Multiple".to_string(), "values".to_string(), "to".to_string(), "stderr".to_string());
    print!("Number: {}, String: {}, Float: {:.2}\n", 123, "hello".to_string(), 3.14);
    fmt.fprintf(os.stderr, "Error code: %d, Message: %s\n".to_string(), 404, "Not Found".to_string());
    println!("{}", "Program completed successfully".to_string());
    fmt.fprintln(os.stderr, "No errors occurred".to_string());
}