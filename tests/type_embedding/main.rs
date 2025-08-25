use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

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


#[derive(Debug, Clone, Default)]
struct Employee {
    person: Rc<RefCell<Option<Person>>>,
    i_d: Rc<RefCell<Option<i32>>>,
}

impl std::fmt::Display for Employee {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.i_d.borrow().as_ref().unwrap()))
    }
}


impl Employee {
}

fn main() {
    let mut e = Rc::new(RefCell::new(Some(Employee { person: Rc::new(RefCell::new(Some(Person { name: Rc::new(RefCell::new(Some("John".to_string()))), age: Rc::new(RefCell::new(Some(30))) }))), i_d: Rc::new(RefCell::new(Some(123))) })));
    println!("{}", (*(*(*e.borrow().as_ref().unwrap()).person.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()));
    println!("{}", (*(*e.borrow().as_ref().unwrap()).i_d.borrow().as_ref().unwrap()));
}