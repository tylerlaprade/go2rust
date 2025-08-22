use std::cell::{RefCell};
use std::rc::{Rc};

pub fn vals() -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) {

    return (Rc::new(RefCell::new(Some(3))), Rc::new(RefCell::new(Some(7))));
}

fn main() {
    let (mut a, mut b) = vals();
    println!("{}", (*a.borrow_mut().as_mut().unwrap()));
    println!("{}", (*b.borrow_mut().as_mut().unwrap()));

    let (_, mut c) = vals();
    println!("{}", (*c.borrow_mut().as_mut().unwrap()));
}