use std::cell::{RefCell};
use std::rc::{Rc};

pub fn divmod(a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>) -> (i32, i32) {

    return ((*a.borrow().as_ref().unwrap()) / (*b.borrow().as_ref().unwrap()), (*a.borrow().as_ref().unwrap()) % (*b.borrow().as_ref().unwrap()));
}

pub fn swap(a: Rc<RefCell<Option<String>>>, b: Rc<RefCell<Option<String>>>) -> (Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) {

    return (Rc::new(RefCell::new(Some(b.borrow().as_ref().unwrap().clone()))), Rc::new(RefCell::new(Some(a.borrow().as_ref().unwrap().clone()))));
}

fn main() {
        // Basic multiple returns
    let (mut q, mut r) = divmod(Rc::new(RefCell::new(Some(17))), Rc::new(RefCell::new(Some(5))));
    println!("{} {} {} {}", format!("{}", "Quotient:".to_string()), format!("{}", q), format!("{}", "Remainder:".to_string()), format!("{}", r));

        // Multiple assignment
    let (mut x, mut y) = (Rc::new(RefCell::new(Some("hello".to_string()))), Rc::new(RefCell::new(Some("world".to_string()))));
    println!("{} {} {}", format!("{}", "Before swap:".to_string()), format!("{}", { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*y.borrow().as_ref().unwrap()).clone(); __v }));

        // Swap using function
    { let (__tmp_0, __tmp_1) = swap(Rc::new(RefCell::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.borrow_mut(); __guard.take() }; *x.borrow_mut() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.borrow_mut(); __guard.take() }; *y.borrow_mut() = __moved_tmp_1; };
    println!("{} {} {}", format!("{}", "After swap:".to_string()), format!("{}", { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*y.borrow().as_ref().unwrap()).clone(); __v }));

        // Ignoring values
    let (_, mut r2) = divmod(Rc::new(RefCell::new(Some(23))), Rc::new(RefCell::new(Some(7))));
    println!("{} {}", format!("{}", "23 mod 7 =".to_string()), format!("{}", r2));
}