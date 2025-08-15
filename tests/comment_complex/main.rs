use std::sync::{Arc, Mutex};

/// Person represents a person with a name and age
#[derive(Debug)]
struct Person {
    name: Arc<Mutex<Option<String>>>,
    age: Arc<Mutex<Option<i32>>>,
}

impl Person {
    /// Greet prints a greeting message
    pub fn greet(&mut self) {
                // Check if person is valid
if false {
                // Handle nil receiver
println!("{}", "Invalid person".to_string());
        return;
    }
                // Handle nil receiver
        // Print the greeting
print!("Hello, I'm {} and I'm {} years old\n", (*self.name.lock().unwrap().as_ref().unwrap()), (*self.age.lock().unwrap().as_ref().unwrap()));
    }
}

/// NewPerson creates a new Person instance
pub fn new_person(name: Arc<Mutex<Option<String>>>, age: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Person>>> {

        // Validate inputs
if (*age.lock().unwrap().as_mut().unwrap()) < 0 {
                // Return nil for invalid age
return Arc::new(Mutex::new(None));
    }

        // Return nil for invalid age
    // Create and return the person
return Arc::new(Mutex::new(Some(Person { name: name.clone(), age: age.clone() })));
}

fn main() {
        // Create a new person
let mut person = new_person(Arc::new(Mutex::new(Some("Alice".to_string()))), Arc::new(Mutex::new(Some(30))));

        // Call the greeting method
(*person.lock().unwrap().as_mut().unwrap()).greet();

        // Try with invalid age
let mut invalid = new_person(Arc::new(Mutex::new(Some("Bob".to_string()))), Arc::new(Mutex::new(Some(-1))));
    if (*invalid.lock().unwrap()).is_none() {
                // This should print
println!("{}", "Failed to create person with invalid age".to_string());
    }
}