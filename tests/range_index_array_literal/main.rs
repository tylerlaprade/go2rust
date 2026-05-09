use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut pairs: Rc<RefCell<Option<Vec<[i32; 2]>>>> = Rc::new(RefCell::new(None));
    for (i, value) in vec![4, 5].iter().copied().enumerate() {
        {(*pairs.borrow_mut()).get_or_insert_with(Vec::new).push([i as i32, value]); pairs.clone()};
    }
    let mut prev = Rc::new(RefCell::new(Some([0, 0])));
    { let __range_holder = pairs.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for pair in __range_values.iter().copied() {
        { let new_val = pair; *prev.borrow_mut() = Some(new_val); };
    } }
    println!("{} {} {} {}", (*pairs.borrow().as_ref().unwrap())[(0) as usize].clone()[(0) as usize].clone(), (*pairs.borrow().as_ref().unwrap())[(1) as usize].clone()[(1) as usize].clone(), (*prev.borrow().as_ref().unwrap())[(0) as usize].clone(), (*prev.borrow().as_ref().unwrap())[(1) as usize].clone());
}