use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct rect {
    pub width: Rc<RefCell<Option<i64>>>,
    pub height: Rc<RefCell<Option<i64>>>,
}

impl rect {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.width.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.height.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            width: __go_clone_0_0,
            height: __go_clone_0_1,
        }
    }
}


impl Default for rect {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_0_1 = Rc::new(RefCell::new(Some(0)));
        Self {
            width: __go_default_0_0,
            height: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for rect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.width.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.height.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl rect {
    pub fn area(&self) -> i64 {
        return (*self.width.borrow().as_ref().unwrap()) * (*self.height.borrow().as_ref().unwrap());
    }

    pub fn perim(&self) -> i64 {
        2 * (*self.width.borrow().as_ref().unwrap()) + 2 * (*self.height.borrow().as_ref().unwrap())
    }
}

fn main() {
    let mut r = Rc::new(RefCell::new(Some(rect { width: Rc::new(RefCell::new(Some(10))), height: Rc::new(RefCell::new(Some(5))), ..Default::default() })));
    println!("{} {}", format!("{}", "area: ".to_string()), format!("{}", (*r.borrow().as_ref().unwrap()).area()));
    println!("{} {}", format!("{}", "perim:".to_string()), format!("{}", (*r.borrow().as_ref().unwrap()).perim()));

    let mut rp = r.clone();
    println!("{} {}", format!("{}", "area: ".to_string()), format!("{}", (*rp.borrow().as_ref().unwrap()).area()));
    println!("{} {}", format!("{}", "perim:".to_string()), format!("{}", (*rp.borrow().as_ref().unwrap()).perim()));
}