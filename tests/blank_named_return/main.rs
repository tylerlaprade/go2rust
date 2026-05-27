use std::cell::{RefCell};
use std::rc::{Rc};

pub fn f() -> (i32, Rc<RefCell<Option<String>>>) {
    let _: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(0)));
    let mut s: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(Some(String::new())));

    { let new_val = "ok".to_string(); *s.borrow_mut() = Some(new_val); };
    return (0 as i32, s);
}

fn main() {
    let (_, mut s) = f();
    println!("{}", format!("{}", { let __v = (*s.borrow().as_ref().unwrap()).clone(); __v }));
}