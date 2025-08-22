use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
        // Even simpler: mutation through aliasing
    let mut x = Rc::new(RefCell::new(Some(10)));
    let mut y = x.clone();
    let mut z = Rc::new(RefCell::new(Some((*y.borrow_mut().as_mut().unwrap()))));

    { let new_val = 20; *y.borrow_mut() = Some(new_val); };
    { let new_val = 30; *z.borrow_mut() = Some(new_val); };

    println!("{} {}", "x =".to_string(), (*x.borrow_mut().as_mut().unwrap()));
}