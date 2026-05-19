use std::cell::{RefCell};
use std::rc::{Rc};

pub fn classify(n: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<String>>> {

    if (*n.borrow().as_ref().unwrap()) < 0 {
        return Rc::new(RefCell::new(Some("negative".to_string())));
    } else {
        let mut x = Rc::new(RefCell::new(Some(n.borrow().as_ref().unwrap().clone())));;
        if (*x.borrow().as_ref().unwrap()) == 4 {
            return Rc::new(RefCell::new(Some("four".to_string())));;
        } else {
        let mut y = Rc::new(RefCell::new(Some(n.borrow().as_ref().unwrap().clone())));;
        if (*y.borrow().as_ref().unwrap()) == 9 {
            return Rc::new(RefCell::new(Some("nine".to_string())));;
        }
    }
    }
    return Rc::new(RefCell::new(Some("other".to_string())));
}

fn main() {
    println!("{}", format!("{}", (*classify(Rc::new(RefCell::new(Some(4)))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*classify(Rc::new(RefCell::new(Some(9)))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*classify(Rc::new(RefCell::new(Some(5)))).borrow().as_ref().unwrap())));
}