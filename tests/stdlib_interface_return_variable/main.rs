use std::cell::{RefCell};
use std::rc::{Rc};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Chan;

impl std::fmt::Display for types_Chan {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Chan>")
    }
}


impl types_Chan {
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


impl From<types_Chan> for types_Type {
    fn from(_value: types_Chan) -> Self {
        Self::default()
    }
}


pub fn as_type(t: Rc<RefCell<Option<types_Type>>>) -> Rc<RefCell<Option<types_Type>>> {

    let (mut ch, mut ok) = ({
        let val = t.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<types_Chan>() {
                (Rc::new(RefCell::new(Some(typed_val.clone()))), Rc::new(RefCell::new(Some(true))))
            } else {
                (Rc::new(RefCell::new(Some(Default::default()))), Rc::new(RefCell::new(Some(false))))
            }
        } else {
            (Rc::new(RefCell::new(Some(Default::default()))), Rc::new(RefCell::new(Some(false))))
        }
    });
    if (*ok.borrow().as_ref().unwrap()) {
        return Rc::new(RefCell::new(Some((*ch.borrow().as_ref().unwrap()).clone().into())));
    }
    return t.clone();
}

fn main() {
    if false {
        println!("{}", format!("{}", (*(as_type(Rc::new(RefCell::new(None)))).borrow().as_ref().unwrap())));
    }
    println!("{}", "ok".to_string());
}