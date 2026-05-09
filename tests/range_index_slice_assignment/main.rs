use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut values = Rc::new(RefCell::new(Some(vec![0; (3) as usize])));
    for i in 0..({ let __range_holder = values.clone(); let __range_guard = __range_holder.borrow(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) {
        (*values.borrow_mut().as_mut().unwrap())[(i) as usize] = i as i32;
    }
    println!("{}", (*values.borrow().as_ref().unwrap())[(0) as usize].clone());
    println!("{}", (*values.borrow().as_ref().unwrap())[(1) as usize].clone());
    println!("{}", (*values.borrow().as_ref().unwrap())[(2) as usize].clone());
}