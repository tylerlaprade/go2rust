use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct store {
}

impl std::fmt::Display for store {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


impl store {
    pub fn set(&self, value: Rc<RefCell<Option<Box<dyn Any>>>>) -> Rc<RefCell<Option<Box<dyn Any>>>> {
        return value.clone();
    }
}

fn main() {
    let mut s: Rc<RefCell<Option<store>>> = Rc::new(RefCell::new(Some(Default::default())));
    let (mut seen, _) = ({
        let val = (*s.borrow_mut().as_mut().unwrap()).set(Rc::new(RefCell::new(Some(Box::new(true) as Box<dyn Any>)))).clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<bool>() {
                (Rc::new(RefCell::new(Some(typed_val.clone()))), Rc::new(RefCell::new(Some(true))))
            } else {
                (Rc::new(RefCell::new(Some(false))), Rc::new(RefCell::new(Some(false))))
            }
        } else {
            (Rc::new(RefCell::new(Some(false))), Rc::new(RefCell::new(Some(false))))
        }
    });
    println!("{}", { let __v = (*seen.borrow().as_ref().unwrap()).clone(); __v });
}