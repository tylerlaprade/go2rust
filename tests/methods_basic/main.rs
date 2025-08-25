use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
struct Counter {
    value: Rc<RefCell<Option<i32>>>,
}

impl std::fmt::Display for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.value.borrow().as_ref().unwrap()))
    }
}


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


impl Counter {
    /// Method with value receiver
    pub fn get_value(&self) -> Rc<RefCell<Option<i32>>> {
        return self.value.clone();
    }

    /// Method with pointer receiver
    pub fn increment(&mut self) {
        { let mut guard = self.value.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    pub fn add(&mut self, n: Rc<RefCell<Option<i32>>>) {
        { let mut guard = self.value.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + (*n.borrow_mut().as_mut().unwrap())); };
    }

    /// Method with return value
    pub fn double(&mut self) -> Rc<RefCell<Option<i32>>> {
        { let mut guard = self.value.borrow_mut(); *guard = Some(guard.as_ref().unwrap() * 2); };
        return self.value.clone();
    }
}

impl Person {
    pub fn greet(&self) {
        print!("Hello, I'm {} and I'm {} years old\n", (*self.name.borrow().as_ref().unwrap()), (*self.age.borrow().as_ref().unwrap()));
    }

    pub fn have_birthday(&mut self) {
        { let mut guard = self.age.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        print!("{} is now {} years old\n", (*self.name.borrow().as_ref().unwrap()), (*self.age.borrow().as_ref().unwrap()));
    }
}

fn main() {
        // Counter methods
    let mut counter = Rc::new(RefCell::new(Some(Counter { value: Rc::new(RefCell::new(Some(0))) })));
    println!("{} {}", "Initial value:".to_string(), (*(*counter.borrow_mut().as_mut().unwrap()).get_value().borrow().as_ref().unwrap()));

    (*counter.borrow_mut().as_mut().unwrap()).increment();
    println!("{} {}", "After increment:".to_string(), (*(*counter.borrow_mut().as_mut().unwrap()).get_value().borrow().as_ref().unwrap()));

    (*counter.borrow_mut().as_mut().unwrap()).add(Rc::new(RefCell::new(Some(5))));
    println!("{} {}", "After adding 5:".to_string(), (*(*counter.borrow_mut().as_mut().unwrap()).get_value().borrow().as_ref().unwrap()));

    let mut doubled = (*counter.borrow_mut().as_mut().unwrap()).double();
    println!("{} {}", "After doubling:".to_string(), (*doubled.borrow_mut().as_mut().unwrap()));

        // Person methods
    let mut person = Rc::new(RefCell::new(Some(Person { name: Rc::new(RefCell::new(Some("Alice".to_string()))), age: Rc::new(RefCell::new(Some(25))) })));
    (*person.borrow_mut().as_mut().unwrap()).greet();
    (*person.borrow_mut().as_mut().unwrap()).have_birthday();
    (*person.borrow_mut().as_mut().unwrap()).greet();
}