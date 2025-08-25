use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
struct User {
    // tags: `json:"name"`
    name: Rc<RefCell<Option<String>>>,
    // tags: `json:"age"`
    age: Rc<RefCell<Option<i32>>>,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.name.borrow().as_ref().unwrap()), (*self.age.borrow().as_ref().unwrap()))
    }
}


fn main() {
    let mut u = Rc::new(RefCell::new(Some(User { name: Rc::new(RefCell::new(Some("Alice".to_string()))), age: Rc::new(RefCell::new(Some(30))) })));
    let (mut data, _) = (*json.borrow_mut().as_mut().unwrap()).marshal(Rc::new(RefCell::new(Some((*u.borrow_mut().as_mut().unwrap())))));
    println!("{}", (*Rc::new(RefCell::new(Some(String::from_utf8((*data.borrow().as_ref().unwrap()).clone()).unwrap()))).borrow().as_ref().unwrap()));
}