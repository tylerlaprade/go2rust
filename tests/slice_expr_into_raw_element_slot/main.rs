use std::cell::{RefCell};
use std::rc::{Rc};

/// Assigning a slice expression into a value-typed slice element slot
/// (a[i] = s[lo:hi:cap] where a is [][]byte). The element slot stores a raw Vec;
/// the slice expression lowers to a wrapped handle, so it must be unwrapped to
/// match the raw slot (E0308 otherwise). Mirrors bytes' genSplit/Fields.
fn main() {
    let mut s = Rc::new(RefCell::new(Some(("hello world".to_string()).as_bytes().to_vec())));
    let mut a = Rc::new(RefCell::new(Some(vec![vec![]; (2) as usize])));
    { let __idx = (0) as usize; let __rhs = (*Rc::new(RefCell::new(Some({ let mut __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; let __low = (0) as usize; let __high = (5) as usize; let __max = (5) as usize; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).borrow().as_ref().unwrap()).clone(); (*a.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    { let __idx = (1) as usize; let __rhs = (*Rc::new(RefCell::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (6) as usize; let __high = (11) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).borrow().as_ref().unwrap()).clone(); (*a.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {} {}", format!("{}", (*a.borrow().as_ref().unwrap())[(0) as usize].clone().len()), format!("{}", (*a.borrow().as_ref().unwrap())[(0) as usize].clone().capacity()), format!("{}", (*a.borrow().as_ref().unwrap())[(1) as usize].clone().len()));
}