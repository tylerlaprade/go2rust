use std::cell::{RefCell};
use std::rc::{Rc};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Alias;

impl std::fmt::Display for types_Alias {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Alias>")
    }
}


impl types_Alias {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn rhs(&self) -> Rc<RefCell<Option<types_Type>>> {
        Rc::new(RefCell::new(Some::<types_Type>(Default::default())))
    }
}


#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
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


fn main() {
    let mut alias: Rc<RefCell<Option<types_Alias>>> = Rc::new(RefCell::new(None));
    if false {
        println!("{}", format!("{}", (*((*alias.borrow_mut().as_mut().unwrap()).rhs()).borrow().as_ref().unwrap())));
    }
    if false {
        let (mut withRhs, mut ok) = ({
        let __asserted = alias.clone();
        (__asserted.clone(), Rc::new(RefCell::new(Some(true))))
    });
        if (*ok.borrow().as_ref().unwrap()) {
        println!("{}", format!("{}", (*((*withRhs.borrow().as_ref().unwrap()).rhs()).borrow().as_ref().unwrap())));
    }
    }
    println!("{}", "ok".to_string());
}