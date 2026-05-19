use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

/// Person represents a person with name and age
#[derive(Debug, Clone)]
pub struct Person {
    pub name: Rc<RefCell<Option<String>>>,
    pub age: Rc<RefCell<Option<i32>>>,
}

impl Person {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, age: { let __guard = self.age.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Person {
    fn default() -> Self {
        Self { name: Rc::new(RefCell::new(Some(String::new()))), age: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.name.borrow().as_ref().unwrap()), (*self.age.borrow().as_ref().unwrap()))
    }
}


/// Address represents a physical address
#[derive(Debug, Clone)]
pub struct Address {
    pub street: Rc<RefCell<Option<String>>>,
    pub city: Rc<RefCell<Option<String>>>,
    pub zip: Rc<RefCell<Option<String>>>,
}

impl Address {
    pub fn __go_value_clone(&self) -> Self {
        Self { street: { let __guard = self.street.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, city: { let __guard = self.city.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, zip: { let __guard = self.zip.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Address {
    fn default() -> Self {
        Self { street: Rc::new(RefCell::new(Some(String::new()))), city: Rc::new(RefCell::new(Some(String::new()))), zip: Rc::new(RefCell::new(Some(String::new()))) }
    }
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

impl Employee {
    pub fn __go_value_clone(&self) -> Self {
        Self { person: { let __guard = self.person.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, address: { let __guard = self.address.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, i_d: { let __guard = self.i_d.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Employee {
    fn default() -> Self {
        Self { person: Rc::new(RefCell::new(Some(Person::default()))), address: Rc::new(RefCell::new(Some(Address::default()))), i_d: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for Employee {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.person.borrow().as_ref().unwrap()), (*self.address.borrow().as_ref().unwrap()), (*self.i_d.borrow().as_ref().unwrap()))
    }
}
