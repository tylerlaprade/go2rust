use std::cell::{RefCell};
use std::rc::{Rc};

pub fn vals() -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) {

    return (Rc::new(RefCell::new(Some(3))), Rc::new(RefCell::new(Some(7))));
}

fn main() {
    let (mut a, mut b) = vals();
    println!("{}", format!("{}", { let __v = (*a.borrow().as_ref().unwrap()).clone(); __v }));
    println!("{}", format!("{}", { let __v = (*b.borrow().as_ref().unwrap()).clone(); __v }));

    let (_, mut c) = vals();
    println!("{}", format!("{}", { let __v = (*c.borrow().as_ref().unwrap()).clone(); __v }));
}