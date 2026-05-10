use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut split = Rc::new(RefCell::new(Some({ let __s = "  file.go:12".to_string(); let __sep = ":".to_string(); __s.split(&__sep).map(|__part| __part.to_string()).collect::<Vec<String>>() })));
    let mut filename = Rc::new(RefCell::new(Some({ let __s = (*split.borrow().as_ref().unwrap())[(0) as usize].clone(); __s.trim().to_string() })));
    println!("{}", { let __v = (*filename.borrow().as_ref().unwrap()).clone(); __v });
}