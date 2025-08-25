use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
struct rect {
    width: Rc<RefCell<Option<i32>>>,
    height: Rc<RefCell<Option<i32>>>,
}

impl std::fmt::Display for rect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.width.borrow().as_ref().unwrap()), (*self.height.borrow().as_ref().unwrap()))
    }
}


impl rect {
    pub fn area(&mut self) -> Rc<RefCell<Option<i32>>> {
        return Rc::new(RefCell::new(Some((*self.width.clone().borrow().as_ref().unwrap()) * (*self.height.clone().borrow().as_ref().unwrap()))));
    }

    pub fn perim(&self) -> Rc<RefCell<Option<i32>>> {
        return Rc::new(RefCell::new(Some(2 * (*self.width.clone().borrow().as_ref().unwrap()) + 2 * (*self.height.clone().borrow().as_ref().unwrap()))));
    }
}

fn main() {
    let mut r = Rc::new(RefCell::new(Some(rect { width: Rc::new(RefCell::new(Some(10))), height: Rc::new(RefCell::new(Some(5))) })));
    println!("{} {}", "area: ".to_string(), (*(*r.borrow_mut().as_mut().unwrap()).area().borrow().as_ref().unwrap()));
    println!("{} {}", "perim:".to_string(), (*(*r.borrow_mut().as_mut().unwrap()).perim().borrow().as_ref().unwrap()));

    let mut rp = r.clone();
    println!("{} {}", "area: ".to_string(), (*(*rp.borrow_mut().as_mut().unwrap()).area().borrow().as_ref().unwrap()));
    println!("{} {}", "perim:".to_string(), (*(*rp.borrow_mut().as_mut().unwrap()).perim().borrow().as_ref().unwrap()));
}