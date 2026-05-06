use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
struct AnonymousStruct1 {
    name: Rc<RefCell<Option<String>>>,
    count: Rc<RefCell<Option<i32>>>,
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.name.borrow().as_ref().unwrap()), (*self.count.borrow().as_ref().unwrap()))
    }
}


fn main() {
    let mut item = Rc::new(RefCell::new(Some(AnonymousStruct1 { name: Rc::new(RefCell::new(Some("go".to_string()))), count: Rc::new(RefCell::new(Some(2))) })));
    println!("{} {}", (*(*item.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()), (*(*item.borrow().as_ref().unwrap()).count.borrow().as_ref().unwrap()));
}