use std::cell::{RefCell};
use std::rc::{Rc};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Object;

impl std::fmt::Display for types_Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Object>")
    }
}


impl types_Object {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Scope;

impl std::fmt::Display for types_Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Scope>")
    }
}


impl types_Scope {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn lookup<T0>(&self, _arg0: T0) -> Rc<RefCell<Option<types_Object>>> {
        Rc::new(RefCell::new(Some::<types_Object>(Default::default())))
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_TypeName;

impl std::fmt::Display for types_TypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_TypeName>")
    }
}


impl types_TypeName {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


pub mod types {
    use super::*;
    pub fn Universe() -> Rc<RefCell<Option<types_Scope>>> {
        Rc::new(RefCell::new(Some::<types_Scope>(Default::default())))
    }
}


fn main() {
    if false {
        let (mut name, mut ok) = ({
        let val = (*types::Universe().borrow_mut().as_mut().unwrap()).lookup("int".to_string()).clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<types_TypeName>() {
                (Rc::new(RefCell::new(Some(typed_val.clone()))), Rc::new(RefCell::new(Some(true))))
            } else {
                (Rc::new(RefCell::new(None::<types_TypeName>)), Rc::new(RefCell::new(Some(false))))
            }
        } else {
            (Rc::new(RefCell::new(None::<types_TypeName>)), Rc::new(RefCell::new(Some(false))))
        }
    });
    if (*ok.borrow().as_ref().unwrap()) {
        let _ = (*name.borrow().as_ref().unwrap());
    }
    }

    println!("{}", "ok".to_string());
}