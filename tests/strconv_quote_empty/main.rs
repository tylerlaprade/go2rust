use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut quoted = Rc::new(RefCell::new(Some(format!("{:?}", "".to_string()))));
    println!("{} {}", format!("{}", (*quoted.borrow().as_ref().unwrap()).len()), format!("{}", (*Rc::new(RefCell::new(Some({ let __s = (*quoted.borrow().as_ref().unwrap()).clone(); __s[(1) as usize..(((*quoted.borrow().as_ref().unwrap()).len() as i32) - (1 as i32)) as usize].to_string() }))).borrow().as_ref().unwrap()).clone() == ""));
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some(format!("{:?}", "go list".to_string())))).borrow().as_ref().unwrap())));
}