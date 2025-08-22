use std::cell::{RefCell};
use std::rc::{Rc};

pub fn divmod(a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) {

    return ({
            let __tmp_x = (*a.borrow_mut().as_mut().unwrap());
            let __tmp_y = (*b.borrow_mut().as_mut().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x / __tmp_y)))
        }, {
            let __tmp_x = (*a.borrow_mut().as_mut().unwrap());
            let __tmp_y = (*b.borrow_mut().as_mut().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x % __tmp_y)))
        });
}

pub fn swap(a: Rc<RefCell<Option<String>>>, b: Rc<RefCell<Option<String>>>) -> (Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) {

    return (b.clone(), a.clone());
}

fn main() {
        // Basic multiple returns
    let (mut q, mut r) = divmod(Rc::new(RefCell::new(Some(17))), Rc::new(RefCell::new(Some(5))));
    println!("{} {} {} {}", "Quotient:".to_string(), (*q.borrow_mut().as_mut().unwrap()), "Remainder:".to_string(), (*r.borrow_mut().as_mut().unwrap()));

        // Multiple assignment
    let (mut x, mut y) = (Rc::new(RefCell::new(Some("hello".to_string()))), Rc::new(RefCell::new(Some("world".to_string()))));
    println!("{} {} {}", "Before swap:".to_string(), (*x.borrow_mut().as_mut().unwrap()), (*y.borrow_mut().as_mut().unwrap()));

        // Swap using function
    (x, y) = swap(x.clone(), y.clone());
    println!("{} {} {}", "After swap:".to_string(), (*x.borrow_mut().as_mut().unwrap()), (*y.borrow_mut().as_mut().unwrap()));

        // Ignoring values
    let (_, mut r2) = divmod(Rc::new(RefCell::new(Some(23))), Rc::new(RefCell::new(Some(7))));
    println!("{} {}", "23 mod 7 =".to_string(), (*r2.borrow_mut().as_mut().unwrap()));
}