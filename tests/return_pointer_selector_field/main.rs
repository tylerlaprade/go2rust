use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct node {
    pub value: Rc<RefCell<Option<i32>>>,
}

impl node {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.value.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            value: __go_clone_0_0,
        }
    }
}


impl Default for node {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            value: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.value.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone, Default)]
pub struct holder {
    pub child: Rc<RefCell<Option<node>>>,
}

impl holder {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.child.clone();
        Self {
            child: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.child.borrow(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


pub fn get_child(h: Rc<RefCell<Option<holder>>>) -> Rc<RefCell<Option<node>>> {
    (*h.borrow().as_ref().unwrap()).child.clone()
}

fn main() {
    if false {
        let mut h = Rc::new(RefCell::new(Some(holder { child: Default::default() })));
        println!("{}", format!("{}", format!("&{}", (*get_child(h.clone()).borrow().as_ref().unwrap()))));
    }
    println!("{}", format!("{}", "ok".to_string()));
}