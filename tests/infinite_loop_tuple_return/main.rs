use std::cell::{RefCell};
use std::rc::{Rc};

pub fn choose(ok: Rc<RefCell<Option<bool>>>) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<String>>>) {

    loop {
        if (*ok.borrow().as_ref().unwrap()) {
        return (Rc::new(RefCell::new(Some(1 as i32))), Rc::new(RefCell::new(Some("ok".to_string()))));
    }
        return (Rc::new(RefCell::new(Some(0 as i32))), Rc::new(RefCell::new(Some("no".to_string()))));
    }
}

fn main() {
    let (mut n, mut label) = choose(Rc::new(RefCell::new(Some(true))));
    println!("{} {}", format!("{}", { let __v = (*n.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*label.borrow().as_ref().unwrap()).clone(); __v }));
}