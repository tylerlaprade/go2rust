use std::cell::{RefCell};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Names(pub Rc<RefCell<Option<Vec<String>>>>);


impl Names {
    pub fn swap(&self, i: Rc<RefCell<Option<i32>>>, j: Rc<RefCell<Option<i32>>>) {
        { let __tmp_0 = (*self.0.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); let __tmp_1 = (*self.0.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(); (*self.0.borrow_mut().as_mut().unwrap())[((*i.borrow().as_ref().unwrap())) as usize] = __tmp_0; (*self.0.borrow_mut().as_mut().unwrap())[((*j.borrow().as_ref().unwrap())) as usize] = __tmp_1; };
    }

    pub fn at(&self, i: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some((*self.0.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone())));
    }
}

fn main() {
    let mut names = Rc::new(RefCell::new(Some(Names(Rc::new(RefCell::new(Some(vec!["ada".to_string(), "grace".to_string(), "lin".to_string()])))))));
    (*names.borrow().as_ref().unwrap()).swap(Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(2))));
    println!("{} {}", (*(*names.borrow().as_ref().unwrap()).at(Rc::new(RefCell::new(Some(0)))).borrow().as_ref().unwrap()), (*(*names.borrow().as_ref().unwrap()).at(Rc::new(RefCell::new(Some(2)))).borrow().as_ref().unwrap()));
}