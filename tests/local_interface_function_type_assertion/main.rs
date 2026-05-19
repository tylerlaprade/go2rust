use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait hasValue: std::fmt::Display + Any {
    fn __go_clone_box(&self) -> Box<dyn hasValue>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq(&self, other: &dyn hasValue) -> bool;
    fn value(&self) -> Rc<RefCell<Option<i32>>>;
}

impl Clone for Box<dyn hasValue> {
    fn clone(&self) -> Self {
        self.__go_clone_box()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct r#box {
    pub n: Rc<RefCell<Option<i32>>>,
}

impl r#box {
    pub fn __go_value_clone(&self) -> Self {
        Self { n: { let __guard = self.n.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for r#box {
    fn default() -> Self {
        Self { n: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for r#box {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.n.borrow().as_ref().unwrap()))
    }
}


impl r#box {
    pub fn value(&self) -> Rc<RefCell<Option<i32>>> {
        return self.n.clone();
    }
}

impl hasValue for r#box {
    fn value(&self) -> Rc<RefCell<Option<i32>>> {
        return self.n.clone();
    }
    fn __go_clone_box(&self) -> Box<dyn hasValue> {
        Box::new(self.clone()) as Box<dyn hasValue>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq(&self, other: &dyn hasValue) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<r#box>() {
            self == __other
        } else {
            false
        }
    }
}

pub fn probe(v: Rc<RefCell<Option<Box<dyn Any>>>>) {
    

    let (mut h, mut ok) = ({
        let val = v.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<r#box>() {
                (Rc::new(RefCell::new(Some(Box::new(typed_val.clone()) as Box<dyn hasValue>))), Rc::new(RefCell::new(Some(true))))
            } else {
                (Rc::new(RefCell::new(None::<Box<dyn hasValue>>)), Rc::new(RefCell::new(Some(false))))
            }
        } else {
            (Rc::new(RefCell::new(None::<Box<dyn hasValue>>)), Rc::new(RefCell::new(Some(false))))
        }
    });
    if (*ok.borrow().as_ref().unwrap()) {
        println!("{}", format!("{}", (*(*h.borrow().as_ref().unwrap()).value().borrow().as_ref().unwrap())));
    } else {
        println!("{}", format!("{}", "no".to_string()));
    }
}

fn main() {
    probe(Rc::new(RefCell::new(Some(Box::new(r#box { n: Rc::new(RefCell::new(Some(7))), ..Default::default() }) as Box<dyn Any>))));
    probe(Rc::new(RefCell::new(Some(Box::new("x".to_string()) as Box<dyn Any>))));
}