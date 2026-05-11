use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct rect {
    pub width: Rc<RefCell<Option<i32>>>,
    pub height: Rc<RefCell<Option<i32>>>,
}

impl rect {
    pub fn __go_value_clone(&self) -> Self {
        Self { width: { let __guard = self.width.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, height: { let __guard = self.height.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}

impl std::fmt::Display for rect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.width.borrow().as_ref().unwrap()), (*self.height.borrow().as_ref().unwrap()))
    }
}


impl rect {
    pub fn area(&self) -> Rc<RefCell<Option<i32>>> {
        return Rc::new(RefCell::new(Some((*self.width.clone().borrow().as_ref().unwrap()) * (*self.height.clone().borrow().as_ref().unwrap()))));
    }

    pub fn perim(&self) -> Rc<RefCell<Option<i32>>> {
        return Rc::new(RefCell::new(Some(2 * (*self.width.clone().borrow().as_ref().unwrap()) + 2 * (*self.height.clone().borrow().as_ref().unwrap()))));
    }
}

fn main() {
    let mut r = Rc::new(RefCell::new(Some(rect { width: Rc::new(RefCell::new(Some(10))), height: Rc::new(RefCell::new(Some(5))), ..Default::default() })));
    println!("{} {}", "area: ".to_string(), (*(*r.borrow_mut().as_mut().unwrap()).area().borrow().as_ref().unwrap()));
    println!("{} {}", "perim:".to_string(), (*(*r.borrow().as_ref().unwrap()).perim().borrow().as_ref().unwrap()));

    let mut rp = r.clone();
    println!("{} {}", "area: ".to_string(), (*(*rp.borrow_mut().as_mut().unwrap()).area().borrow().as_ref().unwrap()));
    println!("{} {}", "perim:".to_string(), (*(*rp.borrow().as_ref().unwrap()).perim().borrow().as_ref().unwrap()));
}