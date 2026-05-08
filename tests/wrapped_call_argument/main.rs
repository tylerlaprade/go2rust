use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct Box_ {
    pub value: Rc<RefCell<Option<i32>>>,
}

impl std::fmt::Display for Box_ {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.value.borrow().as_ref().unwrap()))
    }
}


impl Box_ {
    pub fn inner(&self) -> Rc<RefCell<Option<Box_>>> {
        return Rc::new(RefCell::new(Some(self.clone())));
    }

    pub fn r#use(&self, other: Rc<RefCell<Option<Box_>>>) -> Rc<RefCell<Option<i32>>> {
        return Rc::new(RefCell::new(Some((*(*other.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap()).clone())));
    }
}

fn main() {
    let mut r#box = Rc::new(RefCell::new(Some(Box_ { value: Rc::new(RefCell::new(Some(7))), ..Default::default() })));
    let mut holder = Rc::new(RefCell::new(Some(Box_ { value: Rc::new(RefCell::new(Some(0))) })));
    println!("{}", (*(*holder.borrow_mut().as_mut().unwrap()).r#use((*r#box.borrow_mut().as_mut().unwrap()).inner()).borrow().as_ref().unwrap()));
}