use std::cell::{RefCell};
use std::rc::{Rc};

pub fn divmod(a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) {

    return ({
            let __tmp_x = (*a.borrow().as_ref().unwrap());
            let __tmp_y = (*b.borrow().as_ref().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x / __tmp_y)))
        }, {
            let __tmp_x = (*a.borrow().as_ref().unwrap());
            let __tmp_y = (*b.borrow().as_ref().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x % __tmp_y)))
        });
}

pub fn swap(a: Rc<RefCell<Option<String>>>, b: Rc<RefCell<Option<String>>>) -> (Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) {

    return (b.clone(), a.clone());
}

fn main() {
        // Basic multiple returns
    let (mut q, mut r) = divmod(Rc::new(RefCell::new(Some(17))), Rc::new(RefCell::new(Some(5))));
    println!("{} {} {} {}", "Quotient:".to_string(), { let __v = (*q.borrow().as_ref().unwrap()).clone(); __v }, "Remainder:".to_string(), { let __v = (*r.borrow().as_ref().unwrap()).clone(); __v });

        // Multiple assignment
    let (mut x, mut y) = (Rc::new(RefCell::new(Some("hello".to_string()))), Rc::new(RefCell::new(Some("world".to_string()))));
    println!("{} {} {}", "Before swap:".to_string(), { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*y.borrow().as_ref().unwrap()).clone(); __v });

        // Swap using function
    { let (__tmp_0, __tmp_1) = swap(Rc::new(RefCell::new(Some((*x.borrow().as_ref().unwrap()).clone()))), Rc::new(RefCell::new(Some((*y.borrow().as_ref().unwrap()).clone())))); *x.borrow_mut() = __tmp_0.borrow_mut().take(); *y.borrow_mut() = __tmp_1.borrow_mut().take(); };
    println!("{} {} {}", "After swap:".to_string(), { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*y.borrow().as_ref().unwrap()).clone(); __v });

        // Ignoring values
    let (_, mut r2) = divmod(Rc::new(RefCell::new(Some(23))), Rc::new(RefCell::new(Some(7))));
    println!("{} {}", "23 mod 7 =".to_string(), { let __v = (*r2.borrow().as_ref().unwrap()).clone(); __v });
}