fn main() {
        // Mixed output to stdout and stderr
    println!("{}", format!("{}", "=== Mixed Output Test ===".to_string()));

        // Standard output
    println!("{}", format!("{}", "This goes to stdout via fmt.Println".to_string()));
    print!("This goes to stdout via fmt.Printf: {}\n", 42);

        // Standard error
    eprintln!("{}", format!("{}", "This goes to stderr via fmt.Fprintln".to_string()));
    eprint!("This goes to stderr via fmt.Fprintf: {}\n", "error message".to_string());

        // Built-in println (goes to stderr)
    eprintln!("{}", format!("{}", "This goes to stderr via built-in println".to_string()));

        // More mixed output
    println!("{}", format!("{}", "Back to stdout".to_string()));
    eprintln!("{}", format!("{}", "Back to stderr".to_string()));

        // Multiple values
    println!("{} {} {} {}", format!("{}", "Multiple".to_string()), format!("{}", "values".to_string()), format!("{}", "to".to_string()), format!("{}", "stdout".to_string()));
    eprintln!("{} {} {} {}", format!("{}", "Multiple".to_string()), format!("{}", "values".to_string()), format!("{}", "to".to_string()), format!("{}", "stderr".to_string()));

        // Numbers and strings mixed
    print!("Number: {}, String: {}, Float: {:.2}\n", 123, "hello".to_string(), 3.14);
    eprint!("Error code: {}, Message: {}\n", 404, "Not Found".to_string());

        // Final messages
    println!("{}", format!("{}", "Program completed successfully".to_string()));
    eprintln!("{}", format!("{}", "No errors occurred".to_string()));
}