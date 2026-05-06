use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct Person {
    pub name: Rc<RefCell<Option<String>>>,
    pub age: Rc<RefCell<Option<i32>>>,
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.name.borrow().as_ref().unwrap()), (*self.age.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct Address {
    pub street: Rc<RefCell<Option<String>>>,
    pub city: Rc<RefCell<Option<String>>>,
    pub state: Rc<RefCell<Option<String>>>,
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.street.borrow().as_ref().unwrap()), (*self.city.borrow().as_ref().unwrap()), (*self.state.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct Employee {
    pub person: Rc<RefCell<Option<Person>>>,
    pub address: Rc<RefCell<Option<Address>>>,
    pub i_d: Rc<RefCell<Option<i32>>>,
    pub salary: Rc<RefCell<Option<f64>>>,
}


impl Default for Employee {
    fn default() -> Self {
        Self { person: Rc::new(RefCell::new(Some(Person::default()))), address: Rc::new(RefCell::new(Some(Address::default()))), i_d: Default::default(), salary: Default::default() }
    }
}

impl std::fmt::Display for Employee {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.person.borrow().as_ref().unwrap()), (*self.address.borrow().as_ref().unwrap()), (*self.i_d.borrow().as_ref().unwrap()), (*self.salary.borrow().as_ref().unwrap()))
    }
}


impl Employee {
}

fn main() {
        // Basic struct creation
    let mut p1 = Rc::new(RefCell::new(Some(Person { name: Rc::new(RefCell::new(Some("Alice".to_string()))), age: Rc::new(RefCell::new(Some(30))), ..Default::default() })));
    println!("{} {}", "Person 1:".to_string(), (*p1.borrow().as_ref().unwrap()));

        // Struct with field names
    let mut p2 = Rc::new(RefCell::new(Some(Person { name: Rc::new(RefCell::new(Some("Bob".to_string()))), age: Rc::new(RefCell::new(Some(25))), ..Default::default() })));
    println!("{} {}", "Person 2:".to_string(), (*p2.borrow().as_ref().unwrap()));

        // Access and modify fields
    { let new_val = 26; *(*p2.borrow().as_ref().unwrap()).age.borrow_mut() = Some(new_val); };
    println!("{} {}", "Updated Person 2:".to_string(), (*p2.borrow().as_ref().unwrap()));

        // Embedded structs
    let mut emp = Rc::new(RefCell::new(Some(Employee { person: Rc::new(RefCell::new(Some(Person { name: Rc::new(RefCell::new(Some("Charlie".to_string()))), age: Rc::new(RefCell::new(Some(35))), ..Default::default() }))), address: Rc::new(RefCell::new(Some(Address { street: Rc::new(RefCell::new(Some("123 Main St".to_string()))), city: Rc::new(RefCell::new(Some("Anytown".to_string()))), state: Rc::new(RefCell::new(Some("CA".to_string()))), ..Default::default() }))), i_d: Rc::new(RefCell::new(Some(1001))), salary: Rc::new(RefCell::new(Some(75000.0))), ..Default::default() })));

    println!("{} {}", "Employee:".to_string(), (*emp.borrow().as_ref().unwrap()));
    println!("{} {}", "Employee name:".to_string(), (*(*(*emp.borrow().as_ref().unwrap()).person.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()));
    println!("{} {}", "Employee city:".to_string(), (*(*(*emp.borrow().as_ref().unwrap()).address.borrow().as_ref().unwrap()).city.borrow().as_ref().unwrap()));
}