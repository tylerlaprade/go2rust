mod types;
use types::*;

use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
        // Create a Person - transpiler needs to know Person struct fields
    let mut p = Rc::new(RefCell::new(Some(Person { name: Rc::new(RefCell::new(Some("Alice".to_string()))), age: Rc::new(RefCell::new(Some(30))) })));
    print!("Person: {} is {} years old\n", (*(*p.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()), (*(*p.borrow().as_ref().unwrap()).age.borrow().as_ref().unwrap()));

        // Create an Address - transpiler needs to know Address struct fields
    let mut addr = Rc::new(RefCell::new(Some(Address { street: Rc::new(RefCell::new(Some("123 Main St".to_string()))), city: Rc::new(RefCell::new(Some("Springfield".to_string()))), zip: Rc::new(RefCell::new(Some("12345".to_string()))) })));
    print!("Address: {}, {} {}\n", (*(*addr.borrow().as_ref().unwrap()).street.borrow().as_ref().unwrap()), (*(*addr.borrow().as_ref().unwrap()).city.borrow().as_ref().unwrap()), (*(*addr.borrow().as_ref().unwrap()).zip.borrow().as_ref().unwrap()));

        // Create an Employee - transpiler needs to know nested struct types
    let mut emp = Rc::new(RefCell::new(Some(Employee { person: Rc::new(RefCell::new(Some(Person { name: Rc::new(RefCell::new(Some("Bob".to_string()))), age: Rc::new(RefCell::new(Some(25))) }))), address: addr.clone(), i_d: Rc::new(RefCell::new(Some(42))) })));
    print!("Employee {}: {} lives at {}\n", (*(*emp.borrow().as_ref().unwrap()).i_d.borrow().as_ref().unwrap()), (*(*(*emp.borrow_mut().as_mut().unwrap()).person.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()), (*(*(*emp.borrow_mut().as_mut().unwrap()).address.borrow().as_ref().unwrap()).street.borrow().as_ref().unwrap()));

        // Access nested fields - requires knowing the full type hierarchy
    { let new_val = 26; *(*(*emp.borrow_mut().as_mut().unwrap()).person.borrow().as_ref().unwrap()).age.borrow_mut() = Some(new_val); };
    print!("After birthday: {} is now {}\n", (*(*(*emp.borrow_mut().as_mut().unwrap()).person.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()), (*(*(*emp.borrow_mut().as_mut().unwrap()).person.borrow().as_ref().unwrap()).age.borrow().as_ref().unwrap()));
}