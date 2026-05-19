use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut arr: Rc<RefCell<Option<[i32; 3]>>> = Rc::new(RefCell::new(Some(std::array::from_fn(|_| 0))));
    (*arr.borrow_mut().as_mut().unwrap())[(0) as usize] = 10;
    (*arr.borrow_mut().as_mut().unwrap())[(1) as usize] = 20;
    (*arr.borrow_mut().as_mut().unwrap())[(2) as usize] = 30;

    println!("{}", format!("{}", "Array elements:".to_string()));
    let mut i = Rc::new(RefCell::new(Some(0)));
    while ((*i.borrow().as_ref().unwrap()) as i32) < ((*arr.borrow().as_ref().unwrap()).len() as i32) {
        println!("{}", format!("{}", (*arr.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone()));
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

        // Array initialization
    let mut nums = Rc::new(RefCell::new(Some([1, 2, 3, 4])));
    println!("{}", format!("{}", "Initialized array:".to_string()));
    { let __range_holder = nums.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for num in __range_values.iter().copied() {
        println!("{}", format!("{}", num));
    } }
}