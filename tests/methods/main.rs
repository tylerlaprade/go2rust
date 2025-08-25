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


impl Counter {
    pub fn increment(&mut self) {
        { let mut guard = self.value.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    pub fn value(&mut self) -> Rc<RefCell<Option<i32>>> {
        return self.value.clone();
    }
}

pub fn new_counter() -> Rc<RefCell<Option<Counter>>> {

    return Rc::new(RefCell::new(Some(Counter { value: Rc::new(RefCell::new(Some(0))) })));
}

fn main() {
    let mut counter = new_counter();
    (*counter.borrow_mut().as_mut().unwrap()).increment();
    (*counter.borrow_mut().as_mut().unwrap()).increment();
    println!("{} {}", "Counter value:".to_string(), (*(*counter.borrow_mut().as_mut().unwrap()).value().borrow().as_ref().unwrap()));
}