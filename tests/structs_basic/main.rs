use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

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


#[derive(Debug, Clone)]
pub struct Address {
    pub street: Rc<RefCell<Option<String>>>,
    pub city: Rc<RefCell<Option<String>>>,
    pub state: Rc<RefCell<Option<String>>>,
}

impl Address {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.street.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.city.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.state.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            street: __go_clone_0_0,
            city: __go_clone_1_0,
            state: __go_clone_2_0,
        }
    }
}


impl Default for Address {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_2_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            street: __go_default_0_0,
            city: __go_default_1_0,
            state: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.street.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.city.borrow().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.state.borrow().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


#[derive(Debug, Clone)]
pub struct Employee {
    pub person: Rc<RefCell<Option<Person>>>,
    pub address: Rc<RefCell<Option<Address>>>,
    pub i_d: Rc<RefCell<Option<i32>>>,
    pub salary: Rc<RefCell<Option<f64>>>,
}

impl Employee {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.person.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.address.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.i_d.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.salary.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            person: __go_clone_0_0,
            address: __go_clone_1_0,
            i_d: __go_clone_2_0,
            salary: __go_clone_3_0,
        }
    }
}


impl Default for Employee {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(Person::default())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(Address::default())));
        let __go_default_2_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_3_0 = Rc::new(RefCell::new(Some(0.0)));
        Self {
            person: __go_default_0_0,
            address: __go_default_1_0,
            i_d: __go_default_2_0,
            salary: __go_default_3_0,
        }
    }
}

impl std::fmt::Display for Employee {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.person.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.address.borrow().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.i_d.borrow().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.salary.borrow().as_ref().unwrap()));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}


impl Employee {
}

fn main() {
        // Basic struct creation
    let mut p1 = Rc::new(RefCell::new(Some(Person { name: Rc::new(RefCell::new(Some("Alice".to_string()))), age: Rc::new(RefCell::new(Some(30))), ..Default::default() })));
    println!("{} {}", format!("{}", "Person 1:".to_string()), format!("{}", (*p1.borrow().as_ref().unwrap())));

        // Struct with field names
    let mut p2 = Rc::new(RefCell::new(Some(Person { name: Rc::new(RefCell::new(Some("Bob".to_string()))), age: Rc::new(RefCell::new(Some(25))), ..Default::default() })));
    println!("{} {}", format!("{}", "Person 2:".to_string()), format!("{}", (*p2.borrow().as_ref().unwrap())));

        // Access and modify fields
    { let new_val = 26; *(*p2.borrow().as_ref().unwrap()).age.borrow_mut() = Some(new_val); };
    println!("{} {}", format!("{}", "Updated Person 2:".to_string()), format!("{}", (*p2.borrow().as_ref().unwrap())));

        // Embedded structs
    let mut emp = Rc::new(RefCell::new(Some(Employee { person: Rc::new(RefCell::new(Some(Person { name: Rc::new(RefCell::new(Some("Charlie".to_string()))), age: Rc::new(RefCell::new(Some(35))), ..Default::default() }))), address: Rc::new(RefCell::new(Some(Address { street: Rc::new(RefCell::new(Some("123 Main St".to_string()))), city: Rc::new(RefCell::new(Some("Anytown".to_string()))), state: Rc::new(RefCell::new(Some("CA".to_string()))), ..Default::default() }))), i_d: Rc::new(RefCell::new(Some(1001))), salary: Rc::new(RefCell::new(Some(75000.0))), ..Default::default() })));

    println!("{} {}", format!("{}", "Employee:".to_string()), format!("{}", (*emp.borrow().as_ref().unwrap())));
    println!("{} {}", format!("{}", "Employee name:".to_string()), format!("{}", (*(*(*emp.borrow().as_ref().unwrap()).person.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone()));
    println!("{} {}", format!("{}", "Employee city:".to_string()), format!("{}", (*(*(*emp.borrow().as_ref().unwrap()).address.borrow().as_ref().unwrap()).city.borrow().as_ref().unwrap()).clone()));
}