use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct item {
    pub flag: Rc<RefCell<Option<bool>>>,
}

impl item {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.flag.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            flag: __go_clone_0_0,
        }
    }
}


impl Default for item {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(false)));
        Self {
            flag: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.flag.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
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

    pub fn either(&self, y: Rc<RefCell<Option<item>>>) -> bool {
        return (*self.flag.borrow().as_ref().unwrap()) || !(*(*y.borrow().as_ref().unwrap()).flag.borrow().as_ref().unwrap());
    }

    pub fn active(&self) -> bool {
        if (*self.flag.clone().borrow().as_ref().unwrap()) {
        return true;
    }
        false
    }
}

fn main() {
    let mut on = Rc::new(RefCell::new(Some(item { flag: Rc::new(RefCell::new(Some(true))), ..Default::default() })));
    let mut off = Rc::new(RefCell::new(Some(item { flag: Rc::new(RefCell::new(Some(false))) })));
    println!("{}", format!("{}", (*(*on.borrow().as_ref().unwrap()).label().borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*(*off.borrow().as_ref().unwrap()).label().borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*on.borrow().as_ref().unwrap()).either(off.clone())));
    println!("{}", format!("{}", (*off.borrow().as_ref().unwrap()).active()));
}