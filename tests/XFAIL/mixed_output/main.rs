use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
        // Mixed output to stdout and stderr
    println!("{}", "=== Mixed Output Test ===".to_string());

        // Standard output
    println!("{}", "This goes to stdout via fmt.Println".to_string());
    print!("This goes to stdout via fmt.Printf: {}\n", 42);

        // Standard error
    (*fmt.borrow_mut().as_mut().unwrap())::fprintln(Rc::new(RefCell::new(Some((*os.borrow_mut().as_mut().unwrap())::stderr))), Rc::new(RefCell::new(Some("This goes to stderr via fmt.Fprintln".to_string()))));
    (*fmt.borrow_mut().as_mut().unwrap())::fprintf(Rc::new(RefCell::new(Some((*os.borrow_mut().as_mut().unwrap())::stderr))), Rc::new(RefCell::new(Some("This goes to stderr via fmt.Fprintf: %s\n".to_string()))), Rc::new(RefCell::new(Some("error message".to_string()))));

        // Built-in println (goes to stderr)
    eprintln!("{}", "This goes to stderr via built-in println".to_string());

        // More mixed output
    println!("{}", "Back to stdout".to_string());
    (*fmt.borrow_mut().as_mut().unwrap())::fprintln(Rc::new(RefCell::new(Some((*os.borrow_mut().as_mut().unwrap())::stderr))), Rc::new(RefCell::new(Some("Back to stderr".to_string()))));

        // Multiple values
    println!("{} {} {} {}", "Multiple".to_string(), "values".to_string(), "to".to_string(), "stdout".to_string());
    (*fmt.borrow_mut().as_mut().unwrap())::fprintln(Rc::new(RefCell::new(Some((*os.borrow_mut().as_mut().unwrap())::stderr))), Rc::new(RefCell::new(Some("Multiple".to_string()))), Rc::new(RefCell::new(Some("values".to_string()))), Rc::new(RefCell::new(Some("to".to_string()))), Rc::new(RefCell::new(Some("stderr".to_string()))));

        // Numbers and strings mixed
    print!("Number: {}, String: {}, Float: {:.2}\n", 123, "hello".to_string(), 3.14);
    (*fmt.borrow_mut().as_mut().unwrap())::fprintf(Rc::new(RefCell::new(Some((*os.borrow_mut().as_mut().unwrap())::stderr))), Rc::new(RefCell::new(Some("Error code: %d, Message: %s\n".to_string()))), Rc::new(RefCell::new(Some(404))), Rc::new(RefCell::new(Some("Not Found".to_string()))));

        // Final messages
    println!("{}", "Program completed successfully".to_string());
    (*fmt.borrow_mut().as_mut().unwrap())::fprintln(Rc::new(RefCell::new(Some((*os.borrow_mut().as_mut().unwrap())::stderr))), Rc::new(RefCell::new(Some("No errors occurred".to_string()))));
}