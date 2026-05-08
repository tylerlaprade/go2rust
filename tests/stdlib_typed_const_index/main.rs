use std::cell::{RefCell};
use std::rc::{Rc};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Basic;

impl std::fmt::Display for types_Basic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Basic>")
    }
}


impl types_Basic {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


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


impl From<types_Basic> for types_Type {
    fn from(_value: types_Basic) -> Self {
        Self::default()
    }
}


pub mod types {
    use super::*;
    pub fn typ() -> Rc<RefCell<Option<Vec<Rc<RefCell<Option<types_Basic>>>>>>> {
        Rc::new(RefCell::new(Some::<Vec<Rc<RefCell<Option<types_Basic>>>>>(Default::default())))
    }
}


pub fn is_invalid(t: Rc<RefCell<Option<types_Type>>>) -> Rc<RefCell<Option<bool>>> {

    return {
            let __tmp_x = (*t.borrow().as_ref().unwrap()).clone();
            let __tmp_y = { let __arg = (*types::typ().borrow().as_ref().unwrap())[0usize].clone(); let __converted = { let __arg_guard = __arg.borrow(); let __converted: types_Type = (*__arg_guard.as_ref().unwrap()).clone().into(); __converted }; __converted };
            Rc::new(RefCell::new(Some(__tmp_x == __tmp_y)))
        };
}

fn main() {
    if false {
        println!("{}", (*is_invalid({ let __arg = (*types::typ().borrow().as_ref().unwrap())[0usize].clone(); let __converted = { let __arg_guard = __arg.borrow(); (*__arg_guard.as_ref().unwrap()).clone().into() }; Rc::new(RefCell::new(Some(__converted))) }).borrow().as_ref().unwrap()));
    }
    println!("{}", "ok".to_string());
}