use std::cell::{RefCell};
use std::rc::{Rc};

pub fn count(values: Rc<RefCell<Option<Vec<u8>>>>) -> i32 {

    return (*values.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32;
}

fn main() {
    let mut buf: Rc<RefCell<Option<[u8; 128]>>> = Rc::new(RefCell::new(Some(std::array::from_fn(|_| 0))));
    println!("{}", format!("{}", count(Rc::new(RefCell::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.borrow(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[..(0) as usize].to_vec() }))))));
}