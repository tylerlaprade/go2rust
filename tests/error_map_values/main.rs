use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct customError {
    pub msg: Rc<RefCell<Option<String>>>,
}

impl customError {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.msg.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            msg: __go_clone_0_0,
        }
    }
}


impl Default for customError {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            msg: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for customError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}


impl customError {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        return self.msg.clone();
    }
}

impl StdError for customError {}


pub fn accept(err: Rc<RefCell<Option<Box<dyn StdError>>>>) {
    if { let __nil_result = (*err.borrow()).is_none(); __nil_result } {
        println!("{}", format!("{}", "nil".to_string()));
        return;
    }
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", err.borrow().as_ref().unwrap())))).borrow().as_ref().unwrap())));
}

fn main() {
    let mut errs = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Box<dyn StdError>>>>>::from([("one".to_string(), Rc::new(RefCell::new(Some(Box::<dyn std::error::Error>::from("one".to_string()))))), ("nil".to_string(), Rc::new(RefCell::new(None::<Box<dyn StdError>>)))]))));
    { let __map_key = "two".to_string(); let __map_value = Rc::new(RefCell::new(Some(Box::new(customError { msg: Rc::new(RefCell::new(Some("two".to_string()))), ..Default::default() }) as Box<dyn StdError>))); (*errs.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    { let __map_key = "nil".to_string(); let __map_value = Rc::new(RefCell::new(Some(Box::<dyn std::error::Error>::from("three".to_string())))); (*errs.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };

    {
        let mut err = { let __map_holder = errs.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"one".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) };;
        if { let __nil_result = (*err.borrow()).is_some(); __nil_result } {
            accept(err.clone());;
        }
    }
    accept({ let __map_holder = errs.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"two".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) });
    accept({ let __map_holder = errs.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"nil".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) });
}