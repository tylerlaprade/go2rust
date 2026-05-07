use std::cell::{RefCell};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Names(Rc<RefCell<Option<Vec<String>>>>);


impl Names {
    pub fn len(&self) -> Rc<RefCell<Option<i32>>> {
        return Rc::new(RefCell::new(Some((*self.0.borrow().as_ref().unwrap()).len() as i32)));
    }

    pub fn first(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some((*self.0.borrow().as_ref().unwrap())[(0) as usize].clone())));
    }
}

fn main() {
    let mut names = Rc::new(RefCell::new(Some(Names(Rc::new(RefCell::new(Some(vec!["ada".to_string(), "grace".to_string()])))))));
    println!("{} {}", "Len:".to_string(), (*(*names.borrow().as_ref().unwrap()).len().borrow().as_ref().unwrap()));
    println!("{} {}", "First:".to_string(), (*(*names.borrow().as_ref().unwrap()).first().borrow().as_ref().unwrap()));
}