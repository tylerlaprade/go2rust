use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Pair {
    pub left: Rc<RefCell<Option<String>>>,
    pub right: Rc<RefCell<Option<i32>>>,
}

impl Pair {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.left.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.right.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            left: __go_clone_0_0,
            right: __go_clone_1_0,
        }
    }
}


impl Default for Pair {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            left: __go_default_0_0,
            right: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.left.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.right.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


fn main() {
    let mut p = Rc::new(RefCell::new(Some(Pair { left: Rc::new(RefCell::new(Some("go".to_string()))), right: Rc::new(RefCell::new(Some(2))), ..Default::default() })));
    println!("{} {}", format!("{}", (*(*p.borrow().as_ref().unwrap()).left.borrow().as_ref().unwrap()).clone()), format!("{}", (*(*p.borrow().as_ref().unwrap()).right.borrow().as_ref().unwrap())));
}