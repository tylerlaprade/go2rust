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


impl Counter {
    pub fn increment(&mut self) {
        { let __target = self.value.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    pub fn value(&self) -> i32 {
        return (*self.value.borrow().as_ref().unwrap());
    }
}

pub fn new_counter() -> Rc<RefCell<Option<Counter>>> {
    Rc::new(RefCell::new(Some(Counter { value: Rc::new(RefCell::new(Some(0 as i32))), ..Default::default() })))
}

fn main() {
    let mut counter = new_counter();
    (*counter.borrow_mut().as_mut().unwrap()).increment();
    (*counter.borrow_mut().as_mut().unwrap()).increment();
    println!("{} {}", format!("{}", "Counter value:".to_string()), format!("{}", (*counter.borrow().as_ref().unwrap()).value()));
}