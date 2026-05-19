use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut ptr: Rc<RefCell<Option<usize>>> = Rc::new(RefCell::new(Some(0)));
    if (*Rc::new(RefCell::new(Some(std::mem::size_of::<usize>()))).borrow().as_ref().unwrap()) as usize == 8 as usize {
        println!("{}", format!("{}", "wide".to_string()));
    } else {
        println!("{}", format!("{}", "narrow".to_string()));
    }
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some((*Rc::new(RefCell::new(Some(std::mem::size_of::<usize>()))).borrow().as_ref().unwrap()) as u32))).borrow().as_ref().unwrap())));
}