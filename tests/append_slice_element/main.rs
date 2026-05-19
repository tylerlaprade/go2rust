use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut chunks = Rc::new(RefCell::new(Some(Vec::<Vec<String>>::new())));
    let mut patterns = Rc::new(RefCell::new(Some(vec!["a".to_string(), "bb".to_string(), "ccc".to_string()])));
    let mut start = Rc::new(RefCell::new(Some(0)));

    { let new_val = { let __append_target = chunks.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push((*Rc::new(RefCell::new(Some({ let __seq = { let __seq_holder = patterns.clone(); let __seq_guard = __seq_holder.borrow(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[((*start.borrow().as_ref().unwrap())) as usize..(2) as usize].to_vec() }))).borrow().as_ref().unwrap()).clone()); __append_target.clone() }; chunks = new_val; };
    { let new_val = { let __append_target = chunks.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push((*Rc::new(RefCell::new(Some({ let __seq = { let __seq_holder = patterns.clone(); let __seq_guard = __seq_holder.borrow(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(2) as usize..].to_vec() }))).borrow().as_ref().unwrap()).clone()); __append_target.clone() }; chunks = new_val; };

    println!("{} {} {} {} {} {}", format!("{}", (*chunks.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", (*chunks.borrow().as_ref().unwrap())[(0) as usize].clone().len()), format!("{}", (*chunks.borrow().as_ref().unwrap())[(0) as usize].clone()[(0) as usize].clone()), format!("{}", (*chunks.borrow().as_ref().unwrap())[(0) as usize].clone()[(1) as usize].clone()), format!("{}", (*chunks.borrow().as_ref().unwrap())[(1) as usize].clone().len()), format!("{}", (*chunks.borrow().as_ref().unwrap())[(1) as usize].clone()[(0) as usize].clone()));
}