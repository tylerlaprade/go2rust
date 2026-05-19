use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct rect {
    pub width: Rc<RefCell<Option<i32>>>,
    pub height: Rc<RefCell<Option<i32>>>,
}

impl rect {
    pub fn __go_value_clone(&self) -> Self {
        Self { width: { let __guard = self.width.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, height: { let __guard = self.height.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for rect {
    fn default() -> Self {
        Self { width: Rc::new(RefCell::new(Some(0))), height: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for rect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.width.borrow().as_ref().unwrap()), (*self.height.borrow().as_ref().unwrap()))
    }
}


impl rect {
    pub fn area(&self) -> Rc<RefCell<Option<i32>>> {
        return Rc::new(RefCell::new(Some((*self.width.borrow().as_ref().unwrap()) * (*self.height.borrow().as_ref().unwrap()))));
    }

    pub fn perim(&self) -> Rc<RefCell<Option<i32>>> {
        return Rc::new(RefCell::new(Some(2 * (*self.width.borrow().as_ref().unwrap()) + 2 * (*self.height.borrow().as_ref().unwrap()))));
    }
}

fn main() {
    let mut r = Rc::new(RefCell::new(Some(rect { width: Rc::new(RefCell::new(Some(10))), height: Rc::new(RefCell::new(Some(5))), ..Default::default() })));
    println!("{} {}", format!("{}", "area: ".to_string()), format!("{}", (*(*r.borrow_mut().as_mut().unwrap()).area().borrow().as_ref().unwrap())));
    println!("{} {}", format!("{}", "perim:".to_string()), format!("{}", (*(*r.borrow().as_ref().unwrap()).perim().borrow().as_ref().unwrap())));

    let mut rp = r.clone();
    println!("{} {}", format!("{}", "area: ".to_string()), format!("{}", (*(*rp.borrow_mut().as_mut().unwrap()).area().borrow().as_ref().unwrap())));
    println!("{} {}", format!("{}", "perim:".to_string()), format!("{}", (*(*rp.borrow().as_ref().unwrap()).perim().borrow().as_ref().unwrap())));
}