use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait Value: std::fmt::Display + Any {
    fn __go_clone_box_value(&self) -> Box<dyn Value>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_value(&self, other: &dyn Value) -> bool;
    fn kind(&self) -> Rc<RefCell<Option<String>>>;
}

impl Clone for Box<dyn Value> {
    fn clone(&self) -> Self {
        Value::__go_clone_box_value(self.as_ref())
    }
}

#[derive(Debug, Clone, Default)]
pub struct boolVal(pub Rc<RefCell<Option<bool>>>);

impl Display for boolVal {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for boolVal {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left == __right
    }
}


impl boolVal {
    pub fn kind(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some("bool".to_string())))
    }
}

impl Value for boolVal {
    fn kind(&self) -> Rc<RefCell<Option<String>>> {
        boolVal::kind(self)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value> {
        Box::new(self.clone()) as Box<dyn Value>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &dyn Value) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<boolVal>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct boolValPtr(pub Rc<RefCell<Option<boolVal>>>);

impl std::fmt::Display for boolValPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Value for boolValPtr {
    fn kind(&self) -> Rc<RefCell<Option<String>>> {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        boolVal::kind(__recv)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value> {
        Box::new(self.clone()) as Box<dyn Value>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &dyn Value) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<boolValPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn make_bool(b: Rc<RefCell<Option<bool>>>) -> Rc<RefCell<Option<Box<dyn Value>>>> {
    Rc::new(RefCell::new(Some(Box::new(boolVal(Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap())))))) as Box<dyn Value>)))
}

fn main() {
    let mut v = make_bool(Rc::new(RefCell::new(Some(true))));
    {
            let __go_print_arg_0 = format!("{}", (*(*v.borrow().as_ref().unwrap()).kind().borrow().as_ref().unwrap()));
            eprintln!("{}", __go_print_arg_0)
        };
}