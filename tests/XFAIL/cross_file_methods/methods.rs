use std::cell::{RefCell};
use std::rc::{Rc};

impl Counter {
    /// Methods for Counter type
    pub fn increment(&mut self) {
        { let mut guard = self.value.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    pub fn add(&mut self, n: Rc<RefCell<Option<i32>>>) {
        { let mut guard = self.value.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + (*n.borrow_mut().as_mut().unwrap())); };
    }

    pub fn value(&self) -> Rc<RefCell<Option<i32>>> {
        return self.value.clone();
    }
}

impl Point {
    /// Methods for Point type
    pub fn distance(&self, other: Rc<RefCell<Option<Point>>>) -> Rc<RefCell<Option<f64>>> {
        let mut dx = Rc::new(RefCell::new(Some((*self.x.clone().borrow().as_ref().unwrap()) - (*(*(*other.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()))));
        let mut dy = Rc::new(RefCell::new(Some((*self.y.clone().borrow().as_ref().unwrap()) - (*(*(*other.borrow().as_ref().unwrap()).y.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()))));
        return Rc::new(RefCell::new(Some((*math.borrow_mut().as_mut().unwrap()).sqrt(Rc::new(RefCell::new(Some((*dx.borrow_mut().as_mut().unwrap()) * (*dx.borrow_mut().as_mut().unwrap()) + (*dy.borrow_mut().as_mut().unwrap()) * (*dy.borrow_mut().as_mut().unwrap()))))))));
    }

    pub fn r#move(&mut self, dx: Rc<RefCell<Option<f64>>>, dy: Rc<RefCell<Option<f64>>>) {
        { let mut guard = self.x.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + (*dx.borrow_mut().as_mut().unwrap())); };
        { let mut guard = self.y.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + (*dy.borrow_mut().as_mut().unwrap())); };
    }
}