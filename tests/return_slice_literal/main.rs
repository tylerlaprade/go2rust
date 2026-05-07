use std::cell::{RefCell};
use std::rc::{Rc};

pub fn single(n: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<Vec<i32>>>> {

    return Rc::new(RefCell::new(Some(vec![(*n.borrow().as_ref().unwrap()).clone()])));
}

pub fn pair(a: Rc<RefCell<Option<String>>>, b: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<Vec<String>>>> {

    return Rc::new(RefCell::new(Some(vec![(*a.borrow().as_ref().unwrap()).clone(), (*b.borrow().as_ref().unwrap()).clone()])));
}

fn main() {
    let mut nums = single(Rc::new(RefCell::new(Some(4))));
    let mut words = pair(Rc::new(RefCell::new(Some("go".to_string()))), Rc::new(RefCell::new(Some("rust".to_string()))));

    println!("{} {}", (*nums.borrow().as_ref().unwrap()).len(), (*nums.borrow().as_ref().unwrap())[(0) as usize].clone());
    println!("{} {} {}", (*words.borrow().as_ref().unwrap()).len(), (*words.borrow().as_ref().unwrap())[(0) as usize].clone(), (*words.borrow().as_ref().unwrap())[(1) as usize].clone());
}