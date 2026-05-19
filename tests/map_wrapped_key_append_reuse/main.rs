use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Error {
    pub msg: Rc<RefCell<Option<String>>>,
}

impl Error {
    pub fn __go_value_clone(&self) -> Self {
        Self { msg: { let __guard = self.msg.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Error {
    fn default() -> Self {
        Self { msg: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.msg.borrow().as_ref().unwrap()))
    }
}


fn main() {
    let mut stack = Rc::new(RefCell::new(Some(vec!["importer".to_string(), "imported".to_string()])));
    let mut importingPkg = Rc::new(RefCell::new(Some((*stack.borrow().as_ref().unwrap())[(((*stack.borrow().as_ref().unwrap()).len() as i32) - (2 as i32)) as usize].clone())));
    let mut additionalErrors = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Vec<Error>>>>>::new())));

    { let __map_key = (*importingPkg.borrow().as_ref().unwrap()).clone(); let __map_value = { let __slice = { let __map_holder = additionalErrors.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().unwrap().get(&(*importingPkg.borrow().as_ref().unwrap()).clone()).cloned().unwrap_or_else(|| Rc::new(RefCell::new(None))) }; (*__slice.borrow_mut()).get_or_insert_with(Vec::new).push(Error { msg: importingPkg.clone(), ..Default::default() }); __slice.clone() }; (*additionalErrors.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };

    println!("{}", format!("{}", (*additionalErrors.borrow().as_ref().unwrap()).len()));
    println!("{}", format!("{}", (*(*(*additionalErrors.borrow().as_ref().unwrap()).get(&"importer".to_string()).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()).borrow().as_ref().unwrap())[(0) as usize].clone().msg.borrow().as_ref().unwrap())));
}