use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct node {
    pub value: Rc<RefCell<Option<i64>>>,
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


fn main() {
    let mut r#box: Rc<RefCell<Option<AnonymousStruct1>>> = Rc::new(RefCell::new(Some(Default::default())));
    let mut first = Rc::new(RefCell::new(Some(node { value: Rc::new(RefCell::new(Some(1))), ..Default::default() })));
    let mut second = Rc::new(RefCell::new(Some(node { value: Rc::new(RefCell::new(Some(2))), ..Default::default() })));

    { let new_val = first.clone(); (*r#box.borrow_mut().as_mut().unwrap()).child = new_val; };
    { let new_val = second.clone(); (*r#box.borrow_mut().as_mut().unwrap()).child = new_val; };

    println!("{}", format!("{}", (*(*(*r#box.borrow().as_ref().unwrap()).child.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())));
}

#[derive(Debug, Clone, Default)]
pub struct AnonymousStruct1 {
    pub child: Rc<RefCell<Option<node>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.child.clone();
        Self {
            child: __go_clone_0_0,
        }
    }
}


impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.child.borrow(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}
