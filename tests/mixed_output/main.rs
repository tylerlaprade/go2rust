fn main() {
        // Mixed output to stdout and stderr
    println!("{}", "=== Mixed Output Test ===".to_string());

        // Standard output
    println!("{}", "This goes to stdout via fmt.Println".to_string());
    print!("This goes to stdout via fmt.Printf: {}\n", 42);

        // Standard error
    eprintln!("{}", "This goes to stderr via fmt.Fprintln".to_string());
    eprint!("This goes to stderr via fmt.Fprintf: {}\n", "error message".to_string());

        // Built-in println (goes to stderr)
    eprintln!("{}", "This goes to stderr via built-in println".to_string());

        // More mixed output
    println!("{}", "Back to stdout".to_string());
    eprintln!("{}", "Back to stderr".to_string());

        // Multiple values
    println!("{} {} {} {}", "Multiple".to_string(), "values".to_string(), "to".to_string(), "stdout".to_string());
    eprintln!("{} {} {} {}", "Multiple".to_string(), "values".to_string(), "to".to_string(), "stderr".to_string());

        // Numbers and strings mixed
    print!("Number: {}, String: {}, Float: {:.2}\n", 123, "hello".to_string(), 3.14);
    eprint!("Error code: {}, Message: {}\n", 404, "Not Found".to_string());

        // Final messages
    println!("{}", "Program completed successfully".to_string());
    eprintln!("{}", "No errors occurred".to_string());
}