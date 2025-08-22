use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut arr: Rc<RefCell<Option<[i32; 3]>>> = Rc::new(RefCell::new(Some(Default::default())));
    (*arr.borrow_mut().as_mut().unwrap())[0] = 10;
    (*arr.borrow_mut().as_mut().unwrap())[1] = 20;
    (*arr.borrow_mut().as_mut().unwrap())[2] = 30;

    println!("{}", "Array elements:".to_string());
    let mut i = Rc::new(RefCell::new(Some(0)));
    while (*i.borrow_mut().as_mut().unwrap()) < (*arr.borrow().as_ref().unwrap()).len() {
        println!("{}", (*arr.borrow().as_ref().unwrap())[(*i.borrow_mut().as_mut().unwrap()) as usize].clone());
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

        // Array initialization
    let mut nums = Rc::new(RefCell::new(Some([1, 2, 3, 4])));
    println!("{}", "Initialized array:".to_string());
    for num in &(*nums.borrow_mut().as_mut().unwrap()) {
        println!("{}", num);
    }
}