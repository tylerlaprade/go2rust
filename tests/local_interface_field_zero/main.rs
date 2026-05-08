use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait Reader: std::fmt::Display {
    fn read(&self) -> Rc<RefCell<Option<i32>>>;
}

#[derive(Clone, Default)]
pub struct holder {
    pub reader: Rc<RefCell<Option<Box<dyn Reader>>>>,
    pub count: Rc<RefCell<Option<i32>>>,
}

impl std::fmt::Display for holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.reader.borrow().as_ref().unwrap()), (*self.count.borrow().as_ref().unwrap()))
    }
}


pub fn zero_holder() -> Rc<RefCell<Option<holder>>> {

    return Rc::new(RefCell::new(Some(holder { reader: Rc::new(RefCell::new(None)), count: Rc::new(RefCell::new(Some(0))) })));
}

fn main() {
    let mut holder = zero_holder();
    println!("{}", (*(*holder.borrow().as_ref().unwrap()).count.borrow().as_ref().unwrap()));
}