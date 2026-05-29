use std::cell::{RefCell};
use std::rc::{Rc};

/// Assigning a slice expression into a value-typed slice element slot
/// (a[i] = s[lo:hi:cap] where a is [][]byte). The element slot stores a raw Vec;
/// the slice expression lowers to a wrapped handle, so it must be unwrapped to
/// match the raw slot (E0308 otherwise). Mirrors bytes' genSplit/Fields.
fn main() {
    let mut s = Rc::new(RefCell::new(Some(("hello world".to_string()).as_bytes().to_vec())));
    let mut a = Rc::new(RefCell::new(Some(vec![vec![]; (2) as usize])));
    (*a.borrow_mut().as_mut().unwrap())[(0) as usize] = (*Rc::new(RefCell::new(Some({ let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; let _slice = &__seq[(0) as usize..(5) as usize]; let mut _v = Vec::with_capacity((((5) as usize) - ((0) as usize)) as usize); _v.extend_from_slice(_slice); _v }))).borrow().as_ref().unwrap()).clone();
    (*a.borrow_mut().as_mut().unwrap())[(1) as usize] = (*Rc::new(RefCell::new(Some({ let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(6) as usize..(11) as usize].to_vec() }))).borrow().as_ref().unwrap()).clone();
    println!("{} {} {}", format!("{}", (*a.borrow().as_ref().unwrap())[(0) as usize].clone().len()), format!("{}", (*a.borrow().as_ref().unwrap())[(0) as usize].clone().capacity()), format!("{}", (*a.borrow().as_ref().unwrap())[(1) as usize].clone().len()));
}