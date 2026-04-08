use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc};

fn main() {
    let mut counts = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::new())));
    { let mut guard = (*counts.borrow().as_ref().unwrap())["go".to_string() as usize].clone().borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    { let mut guard = (*counts.borrow().as_ref().unwrap())["rust".to_string() as usize].clone().borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 2); };
    println!("{} {} {}", (*counts.borrow().as_ref().unwrap()).get(&"go".to_string()).unwrap().borrow().as_ref().unwrap().clone(), (*counts.borrow().as_ref().unwrap()).get(&"rust".to_string()).unwrap().borrow().as_ref().unwrap().clone(), (*counts.borrow().as_ref().unwrap()).len());

    let mut seen = Rc::new(RefCell::new(Some(BTreeMap::<i32, Rc<RefCell<Option<bool>>>>::new())));
    (*seen.borrow_mut().as_mut().unwrap()).insert(10, true.clone());
    (*seen.borrow_mut().as_mut().unwrap()).insert(20, true.clone());
    println!("{} {} {}", (*seen.borrow().as_ref().unwrap()).get(&10).unwrap().borrow().as_ref().unwrap().clone(), (*seen.borrow().as_ref().unwrap()).get(&30).unwrap().borrow().as_ref().unwrap().clone(), (*seen.borrow().as_ref().unwrap()).len());
}