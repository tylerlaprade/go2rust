use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait hasValue: std::fmt::Display + Any {
    fn __go_clone_box_has_value(&self) -> Box<dyn hasValue>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_has_value(&self, other: &dyn hasValue) -> bool;
    fn value(&self) -> i64;
}

impl Clone for Box<dyn hasValue> {
    fn clone(&self) -> Self {
        hasValue::__go_clone_box_has_value(self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct r#box {
    pub n: Rc<RefCell<Option<i64>>>,
}

impl r#box {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.n.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            n: __go_clone_0_0,
        }
    }
}


impl Default for r#box {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            n: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for r#box {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.n.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl r#box {
    pub fn value(&self) -> i64 {
        return (*self.n.borrow().as_ref().unwrap());
    }
}

pub fn probe(v: Rc<RefCell<Option<Box<dyn Any>>>>) {
    

    let (mut h, mut ok) = ({
        let val = v.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<r#box>() {
                (Rc::new(RefCell::new(Some(Box::new(typed_val.clone()) as Box<dyn hasValue>))), true)
            } else {
                (Rc::new(RefCell::new(None::<Box<dyn hasValue>>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<Box<dyn hasValue>>)), false)
        }
    });
    if ok {
        println!("{}", format!("{}", (*h.borrow().as_ref().unwrap()).value()));
    } else {
        println!("{}", format!("{}", "no".to_string()));
    }
}

fn main() {
    probe(Rc::new(RefCell::new(Some(Box::new(r#box { n: Rc::new(RefCell::new(Some(7))), ..Default::default() }) as Box<dyn Any>))));
    probe(Rc::new(RefCell::new(Some(Box::new("x".to_string()) as Box<dyn Any>))));
}