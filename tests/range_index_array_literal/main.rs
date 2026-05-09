use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut pairs: Rc<RefCell<Option<Vec<[i32; 2]>>>> = Rc::new(RefCell::new(None));
    for (i, value) in vec![4, 5].iter().copied().enumerate() {
        {(*pairs.borrow_mut()).get_or_insert_with(Vec::new).push([i as i32, value]); pairs.clone()};
    }
    println!("{} {}", (*pairs.borrow().as_ref().unwrap())[(0) as usize].clone()[(0) as usize].clone(), (*pairs.borrow().as_ref().unwrap())[(1) as usize].clone()[(1) as usize].clone());
}