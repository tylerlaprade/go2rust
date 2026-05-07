use std::cell::{RefCell};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Names(pub Rc<RefCell<Option<Vec<String>>>>);


impl Names {
    pub fn len(&self) -> Rc<RefCell<Option<i32>>> {
        return Rc::new(RefCell::new(Some((*self.0.borrow().as_ref().unwrap()).len() as i32)));
    }

    pub fn first(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some((*self.0.borrow().as_ref().unwrap())[(0) as usize].clone())));
    }

    pub fn join(&self) -> Rc<RefCell<Option<String>>> {
        let mut out = Rc::new(RefCell::new(Some("".to_string())));
        { let __range_guard = self.0.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for (i, name) in __range_values.iter().enumerate() {
        if i > 0 {
        (*out.borrow_mut().as_mut().unwrap()).push_str(&",".to_string());
    }
        (*out.borrow_mut().as_mut().unwrap()).push_str(&name);
    } }
        return out.clone();
    }
}

fn main() {
    let mut names = Rc::new(RefCell::new(Some(Names(Rc::new(RefCell::new(Some(vec!["ada".to_string(), "grace".to_string()])))))));
    println!("{} {}", "Len:".to_string(), (*(*names.borrow().as_ref().unwrap()).len().borrow().as_ref().unwrap()));
    println!("{} {}", "First:".to_string(), (*(*names.borrow().as_ref().unwrap()).first().borrow().as_ref().unwrap()));
    println!("{} {}", "Join:".to_string(), (*(*names.borrow().as_ref().unwrap()).join().borrow().as_ref().unwrap()));
}