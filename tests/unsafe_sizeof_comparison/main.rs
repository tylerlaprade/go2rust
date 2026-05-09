use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut ptr: Rc<RefCell<Option<usize>>> = Rc::new(RefCell::new(Some(Default::default())));
    if (*Rc::new(RefCell::new(Some(std::mem::size_of::<usize>()))).borrow().as_ref().unwrap()) as usize == 8 as usize {
        println!("{}", "wide".to_string());
    } else {
        println!("{}", "narrow".to_string());
    }
    println!("{}", (*Rc::new(RefCell::new(Some((*Rc::new(RefCell::new(Some(std::mem::size_of::<usize>()))).borrow().as_ref().unwrap()) as u32))).borrow().as_ref().unwrap()));
}