use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

/// Person represents a person with name and age
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


/// Address represents a physical address
#[derive(Debug, Clone, Default)]
struct Address {
    street: Rc<RefCell<Option<String>>>,
    city: Rc<RefCell<Option<String>>>,
    zip: Rc<RefCell<Option<String>>>,
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.street.borrow().as_ref().unwrap()), (*self.city.borrow().as_ref().unwrap()), (*self.zip.borrow().as_ref().unwrap()))
    }
}


/// Employee combines Person and Address
#[derive(Debug, Clone, Default)]
struct Employee {
    person: Rc<RefCell<Option<Person>>>,
    address: Rc<RefCell<Option<Address>>>,
    i_d: Rc<RefCell<Option<i32>>>,
}

impl std::fmt::Display for Employee {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.person.borrow().as_ref().unwrap()), (*self.address.borrow().as_ref().unwrap()), (*self.i_d.borrow().as_ref().unwrap()))
    }
}
