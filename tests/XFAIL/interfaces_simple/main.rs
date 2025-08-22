use std::cell::{RefCell};
use std::rc::{Rc};

trait geometry {
    fn area(&self) -> Rc<RefCell<Option<f64>>>;
    fn perim(&self) -> Rc<RefCell<Option<f64>>>;
}

#[derive(Debug, Clone, Default)]
struct rect {
    width: Rc<RefCell<Option<f64>>>,
    height: Rc<RefCell<Option<f64>>>,
}

impl rect {
    pub fn area(&self) -> Rc<RefCell<Option<f64>>> {
        return Rc::new(RefCell::new(Some((*self.width.clone().borrow().as_ref().unwrap()) * (*self.height.clone().borrow().as_ref().unwrap()))));
    }

    pub fn perim(&self) -> Rc<RefCell<Option<f64>>> {
        return Rc::new(RefCell::new(Some(2 * (*self.width.clone().borrow().as_ref().unwrap()) + 2 * (*self.height.clone().borrow().as_ref().unwrap()))));
    }
}

impl geometry for rect {
    fn area(&self) -> Rc<RefCell<Option<f64>>> {
        return Rc::new(RefCell::new(Some((*self.width.clone().borrow().as_ref().unwrap()) * (*self.height.clone().borrow().as_ref().unwrap()))));
    }
    fn perim(&self) -> Rc<RefCell<Option<f64>>> {
        return Rc::new(RefCell::new(Some(2 * (*self.width.clone().borrow().as_ref().unwrap()) + 2 * (*self.height.clone().borrow().as_ref().unwrap()))));
    }
}

pub fn measure(g: Rc<RefCell<Option<Box<dyn geometry>>>>) {
    println!("{}", (*g.borrow_mut().as_mut().unwrap()));
    println!("{}", (*(*g.borrow_mut().as_mut().unwrap()).area().borrow().as_ref().unwrap()));
    println!("{}", (*(*g.borrow_mut().as_mut().unwrap()).perim().borrow().as_ref().unwrap()));
}

fn main() {
    let mut r = Rc::new(RefCell::new(Some(rect { width: Rc::new(RefCell::new(Some(3))), height: Rc::new(RefCell::new(Some(4))) })));
    measure(r.clone());
}