use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut buf = Rc::new(RefCell::new(Some(vec![0; 5])));
    let mut n = { let _src = ("hello".to_string()).as_bytes().to_vec(); let _n = std::cmp::min(((*buf.borrow().as_ref().unwrap())).len(), _src.len()); for _i in 0.._n { (*buf.borrow_mut().as_mut().unwrap())[_i] = _src[_i].clone(); } Rc::new(RefCell::new(Some(_n as i32))) };
    println!("{} {}", (*n.borrow().as_ref().unwrap()), (*Rc::new(RefCell::new(Some(String::from_utf8((*buf.borrow().as_ref().unwrap()).clone()).unwrap()))).borrow().as_ref().unwrap()));

    let mut buf2 = Rc::new(RefCell::new(Some(vec![0; 3])));
    let mut n2 = { let _src = ("transpile".to_string()).as_bytes().to_vec(); let _n = std::cmp::min(((*buf2.borrow().as_ref().unwrap())).len(), _src.len()); for _i in 0.._n { (*buf2.borrow_mut().as_mut().unwrap())[_i] = _src[_i].clone(); } Rc::new(RefCell::new(Some(_n as i32))) };
    println!("{} {}", (*n2.borrow().as_ref().unwrap()), (*Rc::new(RefCell::new(Some(String::from_utf8((*buf2.borrow().as_ref().unwrap()).clone()).unwrap()))).borrow().as_ref().unwrap()));
}