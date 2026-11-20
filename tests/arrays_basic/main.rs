use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut arr: Rc<RefCell<Option<[i64; 3]>>> = Rc::new(RefCell::new(Some(std::array::from_fn::<_, 3, _>(|_| 0))));
    { let __idx = (0) as usize; let __rhs = 10; (*arr.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    { let __idx = (1) as usize; let __rhs = 20; (*arr.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    { let __idx = (2) as usize; let __rhs = 30; (*arr.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };

    println!("{}", format!("{}", "Array elements:".to_string()));
    let mut i = Rc::new(RefCell::new(Some(0i64)));
    while ((*i.borrow().as_ref().unwrap()) as i64) < ((*arr.borrow().as_ref().unwrap()).len() as i64) {
        println!("{}", format!("{}", (*arr.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone()));
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

        // Array initialization
    let mut nums = Rc::new(RefCell::new(Some([1i64, 2i64, 3i64, 4i64])));
    println!("{}", format!("{}", "Initialized array:".to_string()));
    { let __range_holder = nums.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for num in __range_values.iter().copied() {
        println!("{}", format!("{}", num));
    } }
}