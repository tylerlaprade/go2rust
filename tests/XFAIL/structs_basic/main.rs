use std::cell::{RefCell};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
struct Person {
    name: Rc<RefCell<Option<String>>>,
    age: Rc<RefCell<Option<i32>>>,
}

#[derive(Debug, Clone, Default)]
struct Address {
    street: Rc<RefCell<Option<String>>>,
    city: Rc<RefCell<Option<String>>>,
    state: Rc<RefCell<Option<String>>>,
}

#[derive(Debug, Clone, Default)]
struct Employee {
    person: Rc<RefCell<Option<Person>>>,
    address: Rc<RefCell<Option<Address>>>,
    i_d: Rc<RefCell<Option<i32>>>,
    salary: Rc<RefCell<Option<f64>>>,
}

impl Employee {
}

fn main() {
        // Basic struct creation
    let mut p1 = Rc::new(RefCell::new(Some(Person { name: Rc::new(RefCell::new(Some("Alice".to_string()))), age: Rc::new(RefCell::new(Some(30))) })));
    println!("{} {}", "Person 1:".to_string(), (*p1.borrow_mut().as_mut().unwrap()));

        // Struct with field names
    let mut p2 = Rc::new(RefCell::new(Some(Person { name: Rc::new(RefCell::new(Some("Bob".to_string()))), age: Rc::new(RefCell::new(Some(25))) })));
    println!("{} {}", "Person 2:".to_string(), (*p2.borrow_mut().as_mut().unwrap()));

        // Access and modify fields
    { let new_val = 26; *(*p2.borrow_mut().as_mut().unwrap()).age.borrow_mut() = Some(new_val); };
    println!("{} {}", "Updated Person 2:".to_string(), (*p2.borrow_mut().as_mut().unwrap()));

        // Embedded structs
    let mut emp = Rc::new(RefCell::new(Some(Employee { person: Rc::new(RefCell::new(Some(Person { name: Rc::new(RefCell::new(Some("Charlie".to_string()))), age: Rc::new(RefCell::new(Some(35))) }))), address: Rc::new(RefCell::new(Some(Address { street: Rc::new(RefCell::new(Some("123 Main St".to_string()))), city: Rc::new(RefCell::new(Some("Anytown".to_string()))), state: Rc::new(RefCell::new(Some("CA".to_string()))) }))), i_d: Rc::new(RefCell::new(Some(1001))), salary: Rc::new(RefCell::new(Some(75000.0))) })));

    println!("{} {}", "Employee:".to_string(), (*emp.borrow_mut().as_mut().unwrap()));
    println!("{} {}", "Employee name:".to_string(), (*(*(*emp.borrow().as_ref().unwrap()).person.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()));
    println!("{} {}", "Employee city:".to_string(), (*(*(*emp.borrow().as_ref().unwrap()).person.borrow().as_ref().unwrap()).city.borrow().as_ref().unwrap()));
}