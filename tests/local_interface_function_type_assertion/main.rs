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

impl hasValue for r#box {
    fn value(&self) -> i64 {
        r#box::value(self)
    }
    fn __go_clone_box_has_value(&self) -> Box<dyn hasValue> {
        Box::new(self.clone()) as Box<dyn hasValue>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_has_value(&self, other: &dyn hasValue) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<r#box>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct boxPtr(pub Rc<RefCell<Option<r#box>>>);

impl std::fmt::Display for boxPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl hasValue for boxPtr {
    fn value(&self) -> i64 {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        r#box::value(__recv)
    }
    fn __go_clone_box_has_value(&self) -> Box<dyn hasValue> {
        Box::new(self.clone()) as Box<dyn hasValue>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_has_value(&self, other: &dyn hasValue) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<boxPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
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