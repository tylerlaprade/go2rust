use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Type;

impl std::fmt::Display for types_Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Type>")
    }
}


impl types_Type {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct localType {
}

impl localType {
    pub fn __go_value_clone(&self) -> Self {
        Self {  }
    }
}

impl std::fmt::Display for localType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().borrow().as_ref().unwrap()))
    }
}


impl localType {
    pub fn underlying(&self) -> Rc<RefCell<Option<types_Type>>> {
        return Rc::new(RefCell::new(Some(types_Type::default())));
    }

    pub fn string(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some("local".to_string())));
    }
}

pub fn make_type() -> Rc<RefCell<Option<types_Type>>> {

    return Rc::new(RefCell::new(Some(types_Type::default())));
}

fn main() {
    let mut typesList = Rc::new(RefCell::new(Some(vec![types_Type::default(), { let __v = make_type(); let __owned = (*__v.borrow().as_ref().unwrap()).clone(); __owned }])));
    println!("{}", (*typesList.borrow().as_ref().unwrap()).len());
}