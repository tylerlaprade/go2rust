use crate::types::*;

use std::cell::{RefCell};
use std::rc::{Rc};

impl Counter {
    /// Methods for Counter type
    pub fn increment(&mut self) {
        { let __target = self.value.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    pub fn add(&mut self, n: Rc<RefCell<Option<i32>>>) {
        { let __target = self.value.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + (*n.borrow().as_ref().unwrap())); };
    }

    pub fn value(&self) -> Rc<RefCell<Option<i32>>> {
        return self.value.clone();
    }
}

impl Point {
    /// Methods for Point type
    pub fn distance(&self, other: Rc<RefCell<Option<Point>>>) -> Rc<RefCell<Option<f64>>> {
        let mut dx = Rc::new(RefCell::new(Some((*self.x.clone().borrow().as_ref().unwrap()) - (*(*other.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap()))));
        let mut dy = Rc::new(RefCell::new(Some((*self.y.clone().borrow().as_ref().unwrap()) - (*(*other.borrow().as_ref().unwrap()).y.borrow().as_ref().unwrap()))));
        return Rc::new(RefCell::new(Some(({ let __bin_dx = (*dx.borrow().as_ref().unwrap()).clone(); __bin_dx * __bin_dx } + { let __bin_dy = (*dy.borrow().as_ref().unwrap()).clone(); __bin_dy * __bin_dy } as f64).sqrt())));
    }

    pub fn r#move(&mut self, dx: Rc<RefCell<Option<f64>>>, dy: Rc<RefCell<Option<f64>>>) {
        { let __target = self.x.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + (*dx.borrow().as_ref().unwrap())); };
        { let __target = self.y.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + (*dy.borrow().as_ref().unwrap())); };
    }
}