mod types;
use types::*;

use std::sync::{Arc, Mutex};

fn main() {
    let mut p = Person { name: Arc::new(Mutex::new(Some("Alice".to_string()))), age: Arc::new(Mutex::new(Some(30))) };
    print!("Person: {} is {} years old\n", (*(*p.lock().unwrap().as_mut().unwrap()).name.lock().unwrap().as_ref().unwrap()), (*(*p.lock().unwrap().as_mut().unwrap()).age.lock().unwrap().as_ref().unwrap()));

    let mut addr = Address { street: Arc::new(Mutex::new(Some("123 Main St".to_string()))), city: Arc::new(Mutex::new(Some("Springfield".to_string()))), zip: Arc::new(Mutex::new(Some("12345".to_string()))) };
    print!("Address: {}, {} {}\n", (*(*addr.lock().unwrap().as_mut().unwrap()).street.lock().unwrap().as_ref().unwrap()), (*(*addr.lock().unwrap().as_mut().unwrap()).city.lock().unwrap().as_ref().unwrap()), (*(*addr.lock().unwrap().as_mut().unwrap()).zip.lock().unwrap().as_ref().unwrap()));

    let mut emp = Employee { person: Arc::new(Mutex::new(Some(Person { name: Arc::new(Mutex::new(Some("Bob".to_string()))), age: Arc::new(Mutex::new(Some(25))) }))), address: Arc::new(Mutex::new(Some((*addr.lock().unwrap().as_mut().unwrap())))), i_d: Arc::new(Mutex::new(Some(42))) };
    print!("Employee {}: {} lives at {}\n", (*(*emp.lock().unwrap().as_mut().unwrap()).i_d.lock().unwrap().as_ref().unwrap()), (*emp.lock().unwrap().as_mut().unwrap()).person.name, (*emp.lock().unwrap().as_mut().unwrap()).address.street);

    { let new_val = 26; *(*emp.lock().unwrap().as_mut().unwrap()).person.age.lock().unwrap() = Some(new_val); };
    print!("After birthday: {} is now {}\n", (*emp.lock().unwrap().as_mut().unwrap()).person.name, (*emp.lock().unwrap().as_mut().unwrap()).person.age);
}