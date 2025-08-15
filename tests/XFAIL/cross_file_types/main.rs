mod types;
use types::*;

use std::sync::{Arc, Mutex};

fn main() {
        // Create a Person - transpiler needs to know Person struct fields
    let mut p = Person { name: Arc::new(Mutex::new(Some("Alice".to_string()))), age: Arc::new(Mutex::new(Some(30))) };
    print!("Person: {} is {} years old\n", (*p.name.lock().unwrap().as_ref().unwrap()), (*p.age.lock().unwrap().as_ref().unwrap()));

        // Create an Address - transpiler needs to know Address struct fields
    let mut addr = Address { street: Arc::new(Mutex::new(Some("123 Main St".to_string()))), city: Arc::new(Mutex::new(Some("Springfield".to_string()))), zip: Arc::new(Mutex::new(Some("12345".to_string()))) };
    print!("Address: {}, {} {}\n", (*addr.street.lock().unwrap().as_ref().unwrap()), (*addr.city.lock().unwrap().as_ref().unwrap()), (*addr.zip.lock().unwrap().as_ref().unwrap()));

        // Create an Employee - transpiler needs to know nested struct types
    let mut emp = Employee { person: Arc::new(Mutex::new(Some(Person { name: Arc::new(Mutex::new(Some("Bob".to_string()))), age: Arc::new(Mutex::new(Some(25))) }))), address: addr.clone(), i_d: Arc::new(Mutex::new(Some(42))) };
    print!("Employee {}: {} lives at {}\n", (*emp.i_d.lock().unwrap().as_ref().unwrap()), emp.person.name, emp.address.street);

        // Access nested fields - requires knowing the full type hierarchy
    { let new_val = 26; *emp.person.age.lock().unwrap() = Some(new_val); };
    print!("After birthday: {} is now {}\n", emp.person.name, emp.person.age);
}