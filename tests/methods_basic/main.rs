use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Counter {
    pub value: Rc<RefCell<Option<i32>>>,
}

impl Counter {
    pub fn __go_value_clone(&self) -> Self {
        Self { value: { let __guard = self.value.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Counter {
    fn default() -> Self {
        Self { value: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.value.borrow().as_ref().unwrap()))
    }
}


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


impl Counter {
    /// Method with value receiver
    pub fn get_value(&self) -> i32 {
        return (*self.value.borrow().as_ref().unwrap());
    }

    /// Method with pointer receiver
    pub fn increment(&mut self) {
        { let __target = self.value.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    pub fn add(&mut self, n: Rc<RefCell<Option<i32>>>) {
        { let __target = self.value.clone(); let __rhs = (*n.borrow().as_ref().unwrap()); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }

    /// Method with return value
    pub fn double(&mut self) -> i32 {
        { let __target = self.value.clone(); let __rhs = 2; let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() * __rhs); };
        return (*self.value.borrow().as_ref().unwrap());
    }
}

impl Person {
    pub fn greet(&self) {
        print!("Hello, I'm {} and I'm {} years old\n", (*self.name.borrow().as_ref().unwrap()), (*self.age.borrow().as_ref().unwrap()));
    }

    pub fn have_birthday(&mut self) {
        { let __target = self.age.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        print!("{} is now {} years old\n", (*self.name.borrow().as_ref().unwrap()), (*self.age.borrow().as_ref().unwrap()));
    }
}

fn main() {
        // Counter methods
    let mut counter = Rc::new(RefCell::new(Some(Counter { value: Rc::new(RefCell::new(Some(0 as i32))), ..Default::default() })));
    println!("{} {}", format!("{}", "Initial value:".to_string()), format!("{}", (*counter.borrow().as_ref().unwrap()).get_value()));

    (*counter.borrow_mut().as_mut().unwrap()).increment();
    println!("{} {}", format!("{}", "After increment:".to_string()), format!("{}", (*counter.borrow().as_ref().unwrap()).get_value()));

    (*counter.borrow_mut().as_mut().unwrap()).add(Rc::new(RefCell::new(Some(5))));
    println!("{} {}", format!("{}", "After adding 5:".to_string()), format!("{}", (*counter.borrow().as_ref().unwrap()).get_value()));

    let mut doubled = (*counter.borrow_mut().as_mut().unwrap()).double();
    println!("{} {}", format!("{}", "After doubling:".to_string()), format!("{}", doubled));

        // Person methods
    let mut person = Rc::new(RefCell::new(Some(Person { name: Rc::new(RefCell::new(Some("Alice".to_string()))), age: Rc::new(RefCell::new(Some(25 as i32))), ..Default::default() })));
    (*person.borrow().as_ref().unwrap()).greet();
    (*person.borrow_mut().as_mut().unwrap()).have_birthday();
    (*person.borrow().as_ref().unwrap()).greet();
}