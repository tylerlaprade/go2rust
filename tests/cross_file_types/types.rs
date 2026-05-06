use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

/// Person represents a person with name and age
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


/// Address represents a physical address
#[derive(Debug, Clone, Default)]
pub struct Address {
    pub street: Rc<RefCell<Option<String>>>,
    pub city: Rc<RefCell<Option<String>>>,
    pub zip: Rc<RefCell<Option<String>>>,
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.street.borrow().as_ref().unwrap()), (*self.city.borrow().as_ref().unwrap()), (*self.zip.borrow().as_ref().unwrap()))
    }
}


/// Employee combines Person and Address
#[derive(Debug, Clone)]
pub struct Employee {
    pub person: Rc<RefCell<Option<Person>>>,
    pub address: Rc<RefCell<Option<Address>>>,
    pub i_d: Rc<RefCell<Option<i32>>>,
}


impl Default for Employee {
    fn default() -> Self {
        Self { person: Rc::new(RefCell::new(Some(Person::default()))), address: Rc::new(RefCell::new(Some(Address::default()))), i_d: Default::default() }
    }
}

impl std::fmt::Display for Employee {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.person.borrow().as_ref().unwrap()), (*self.address.borrow().as_ref().unwrap()), (*self.i_d.borrow().as_ref().unwrap()))
    }
}
