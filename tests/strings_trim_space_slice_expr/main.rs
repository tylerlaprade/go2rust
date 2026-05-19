use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut errstr = Rc::new(RefCell::new(Some("no such directory   /tmp/missing\n".to_string())));
    let mut marker = Rc::new(RefCell::new(Some("no such directory".to_string())));
    let mut abspath = Rc::new(RefCell::new(Some({ let __s = (*Rc::new(RefCell::new(Some({ let __s = (*errstr.borrow().as_ref().unwrap()).clone(); __s[(((*Rc::new(RefCell::new(Some({ let __s = (*errstr.borrow().as_ref().unwrap()).clone(); let __substr = (*marker.borrow().as_ref().unwrap()).clone(); __s.find(&__substr).map(|__i| __i as i32).unwrap_or(-1) }))).borrow().as_ref().unwrap()) as i32) + ((*marker.borrow().as_ref().unwrap()).len() as i32)) as usize..].to_string() }))).borrow().as_ref().unwrap()).clone(); __s.trim().to_string() })));
    println!("{}", format!("{}", { let __v = (*abspath.borrow().as_ref().unwrap()).clone(); __v }));
}