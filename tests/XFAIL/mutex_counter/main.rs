use std::cell::{RefCell};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
struct Counter {
    mu: Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>,
    value: Rc<RefCell<Option<i32>>>,
}

impl Counter {
    pub fn increment(&mut self) {
        (*self.mu.clone().borrow().as_mut().unwrap()).lock();
        __defer_stack.push(Box::new(move || {
        (*self.mu.clone().borrow().as_mut().unwrap()).unlock();
    }));
        { let mut guard = self.value.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    pub fn value(&mut self) -> Rc<RefCell<Option<i32>>> {
        (*self.mu.clone().borrow().as_mut().unwrap()).lock();
        __defer_stack.push(Box::new(move || {
        (*self.mu.clone().borrow().as_mut().unwrap()).unlock();
    }));
        return self.value.clone();
    }
}

fn main() {
    let mut counter = Rc::new(RefCell::new(Some(Counter {  })));
    (*counter.borrow_mut().as_mut().unwrap()).increment();
    (*counter.borrow_mut().as_mut().unwrap()).increment();
    println!("{} {}", "Counter value:".to_string(), (*(*counter.borrow_mut().as_mut().unwrap()).value().borrow().as_ref().unwrap()));
}