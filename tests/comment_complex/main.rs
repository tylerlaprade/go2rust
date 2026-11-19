use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

/// Person represents a person with a name and age
#[derive(Debug, Clone)]
pub struct Person {
    pub name: Rc<RefCell<Option<String>>>,
    pub age: Rc<RefCell<Option<i32>>>,
}

impl Person {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.age.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
            age: __go_clone_1_0,
        }
    }
}


impl Default for Person {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            name: __go_default_0_0,
            age: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.age.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl Person {
    /// Greet prints a greeting message
    pub fn greet(&self) {
                // Check if person is valid
        if false {
                // Handle nil receiver
        println!("{}", format!("{}", "Invalid person".to_string()));
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
    if (*age.borrow().as_ref().unwrap()) < 0 {
                // Return nil for invalid age
        return Rc::new(RefCell::new(None));
    }

        // Return nil for invalid age
        // Create and return the person
    Rc::new(RefCell::new(Some(Person { name: Rc::new(RefCell::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), age: Rc::new(RefCell::new(Some({ let __arg_holder = age.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })))
}

fn main() {
        // Create a new person
    let mut person = new_person(Rc::new(RefCell::new(Some("Alice".to_string()))), Rc::new(RefCell::new(Some(30))));

        // Call the greeting method
    __go_nil_recv_person_greet({ let __recv = person.clone(); __recv });

        // Try with invalid age
    let mut invalid = new_person(Rc::new(RefCell::new(Some("Bob".to_string()))), Rc::new(RefCell::new(Some(-1))));
    if { let __nil_result = (*invalid.borrow()).is_none(); __nil_result } {
                // This should print
        println!("{}", format!("{}", "Failed to create person with invalid age".to_string()));
    }
}

pub fn __go_nil_recv_person_greet(p: Rc<RefCell<Option<Person>>>) {
        // Check if person is valid
    if { let __nil_result = (*p.borrow()).is_none(); __nil_result } {
                // Handle nil receiver
        println!("{}", format!("{}", "Invalid person".to_string()));
        return;
    }

        // Handle nil receiver
        // Print the greeting
    print!("Hello, I'm {} and I'm {} years old\n", (*(*p.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone(), (*(*p.borrow().as_ref().unwrap()).age.borrow().as_ref().unwrap()));
}