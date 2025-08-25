use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

/// Person represents a person with a name and age
#[derive(Debug, Clone, Default)]
struct Person {
    name: Rc<RefCell<Option<String>>>,
    age: Rc<RefCell<Option<i32>>>,
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.name.borrow().as_ref().unwrap()), (*self.age.borrow().as_ref().unwrap()))
    }
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
        print!("Hello, I'm {} and I'm {} years old\n", (*self.name.borrow().as_ref().unwrap()), (*self.age.borrow().as_ref().unwrap()));
    }
}

/// NewPerson creates a new Person instance
pub fn new_person(name: Rc<RefCell<Option<String>>>, age: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<Person>>> {

        // Validate inputs
    if (*age.borrow_mut().as_mut().unwrap()) < 0 {
                // Return nil for invalid age
        return Rc::new(RefCell::new(None));
    }

        // Return nil for invalid age
        // Create and return the person
    return Rc::new(RefCell::new(Some(Person { name: name.clone(), age: age.clone() })));
}

fn main() {
        // Create a new person
    let mut person = new_person(Rc::new(RefCell::new(Some("Alice".to_string()))), Rc::new(RefCell::new(Some(30))));

        // Call the greeting method
    (*person.borrow_mut().as_mut().unwrap()).greet();

        // Try with invalid age
    let mut invalid = new_person(Rc::new(RefCell::new(Some("Bob".to_string()))), Rc::new(RefCell::new(Some(-1))));
    if (*invalid.borrow()).is_none() {
                // This should print
        println!("{}", "Failed to create person with invalid age".to_string());
    }
}