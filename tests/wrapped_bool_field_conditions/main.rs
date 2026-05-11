use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct item {
    pub flag: Rc<RefCell<Option<bool>>>,
}

impl item {
    pub fn __go_value_clone(&self) -> Self {
        Self { flag: { let __guard = self.flag.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}

impl std::fmt::Display for item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.flag.borrow().as_ref().unwrap()))
    }
}


impl item {
    pub fn label(&self) -> Rc<RefCell<Option<String>>> {
        if (*self.flag.clone().borrow().as_ref().unwrap()) {
            return Rc::new(RefCell::new(Some("on".to_string())));
        } else {
            return Rc::new(RefCell::new(Some("off".to_string())));
        }
    }

    pub fn either(&self, y: Rc<RefCell<Option<item>>>) -> Rc<RefCell<Option<bool>>> {
        return Rc::new(RefCell::new(Some((*self.flag.clone().borrow().as_ref().unwrap()) || !(*(*y.borrow().as_ref().unwrap()).flag.borrow().as_ref().unwrap()))));
    }

    pub fn active(&self) -> Rc<RefCell<Option<bool>>> {
        if (*self.flag.clone().borrow().as_ref().unwrap()) {
        return Rc::new(RefCell::new(Some(true)));
    }
        return Rc::new(RefCell::new(Some(false)));
    }
}

fn main() {
    let mut on = Rc::new(RefCell::new(Some(item { flag: Rc::new(RefCell::new(Some(true))), ..Default::default() })));
    let mut off = Rc::new(RefCell::new(Some(item { flag: Rc::new(RefCell::new(Some(false))) })));
    println!("{}", (*(*on.borrow_mut().as_mut().unwrap()).label().borrow().as_ref().unwrap()));
    println!("{}", (*(*off.borrow_mut().as_mut().unwrap()).label().borrow().as_ref().unwrap()));
    println!("{}", (*(*on.borrow_mut().as_mut().unwrap()).either(off.clone()).borrow().as_ref().unwrap()));
    println!("{}", (*(*off.borrow_mut().as_mut().unwrap()).active().borrow().as_ref().unwrap()));
}