use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
        // Basic nil comparisons
    let mut p: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(None));
    if (*p.borrow()).is_none() {
        println!("{}", "p is nil".to_string());
    }

        // Assign nil
    let mut q: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));
    if (*q.borrow()).is_none() {
        println!("{}", "q is nil".to_string());
    }

        // Non-nil pointer
    let mut x = Rc::new(RefCell::new(Some(42)));
    { let new_val = x.clone().clone(); p = new_val; };
    if (*p.borrow()).is_some() {
        println!("{} {}", "p is not nil, value:".to_string(), { let __v = (*p.borrow().as_ref().unwrap()).clone(); __v });
    }

        // Set back to nil
    *p.borrow_mut() = None;
    if (*p.borrow()).is_none() {
        println!("{}", "p is nil again".to_string());
    }
}