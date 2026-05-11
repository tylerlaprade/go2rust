use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
struct AnonymousStruct1 {
    name: Rc<RefCell<Option<String>>>,
    count: Rc<RefCell<Option<i32>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, count: { let __guard = self.count.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.name.borrow().as_ref().unwrap()), (*self.count.borrow().as_ref().unwrap()))
    }
}


fn main() {
    let mut item = Rc::new(RefCell::new(Some(AnonymousStruct1 { name: Rc::new(RefCell::new(Some("go".to_string()))), count: Rc::new(RefCell::new(Some(2))) })));
    println!("{} {}", (*(*item.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone(), (*(*item.borrow().as_ref().unwrap()).count.borrow().as_ref().unwrap()));
}