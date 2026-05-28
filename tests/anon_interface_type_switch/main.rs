use std::any::Any;
use std::cell::{RefCell};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

/// GAP: a type switch/assertion whose case is a NON-NAMED anonymous interface
/// with methods (interface{ Unwrap() error }). goTypesTypeToRust falls through
/// to Box<dyn Any> and goTypeToRustBase emits the literal "Unknown" placeholder
/// (E0277 trait-bound + E0282 inference). These are soft fallbacks that should
/// lower to a real structural trait check. Root cause: go/types.go.
#[derive(Debug, Clone)]
pub struct wrapped {
    pub msg: Rc<RefCell<Option<String>>>,
}

impl wrapped {
    pub fn __go_value_clone(&self) -> Self {
        Self { msg: { let __guard = self.msg.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for wrapped {
    fn default() -> Self {
        Self { msg: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for wrapped {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}


impl wrapped {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        return self.msg.clone();
    }

    pub fn unwrap(&self) -> Rc<RefCell<Option<Box<dyn StdError>>>> {
        Rc::new(RefCell::new(None))
    }
}

impl StdError for wrapped {}


pub fn describe(err: Rc<RefCell<Option<Box<dyn StdError>>>>) -> Rc<RefCell<Option<String>>> {
    {
    let _ts_subject = err.clone();
    let _ts_guard = _ts_subject.borrow();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val = _ts_guard.as_ref();
    if _ts_val.and_then(|__v| __v.downcast_ref::<wrapped>()).is_some() {
        drop(_ts_guard);
        return Rc::new(RefCell::new(Some("unwrappable".to_string())));;
    } else {
        drop(_ts_guard);
        return Rc::new(RefCell::new(Some("plain".to_string())));;
    }
    }
    unreachable!()
}

fn main() {
    println!("{}", format!("{}", (*describe(Rc::new(RefCell::new(Some(Box::new(wrapped { msg: Rc::new(RefCell::new(Some("x".to_string()))), ..Default::default() }) as Box<dyn StdError>)))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*describe(Rc::new(RefCell::new(Some(Box::<dyn StdError>::from(format!("plain")))))).borrow().as_ref().unwrap())));
}