use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut s = Rc::new(RefCell::new(Some({ let mut v = Vec::with_capacity((4) as usize); v.resize((2) as usize, 0); v })));
    (*s.borrow_mut().as_mut().unwrap())[(0) as usize] = ('g' as i32) as u8;
    (*s.borrow_mut().as_mut().unwrap())[(1) as usize] = ('o' as i32) as u8;
    { let new_val = Rc::new(RefCell::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = (3) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); s = new_val; };
    (*s.borrow_mut().as_mut().unwrap())[(2) as usize] = ('2' as i32) as u8;
    let mut t = Rc::new(RefCell::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (1) as usize; let __high = (3) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })));
    println!("{} {} {}", format!("{}", (*s.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", (*s.borrow()).as_ref().map(|__v| __v.capacity()).unwrap_or(0)), format!("{}", (*Rc::new(RefCell::new(Some(String::from_utf8((*s.borrow().as_ref().unwrap()).clone()).unwrap()))).borrow().as_ref().unwrap())));
    println!("{} {} {}", format!("{}", (*t.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", (*t.borrow()).as_ref().map(|__v| __v.capacity()).unwrap_or(0)), format!("{}", (*Rc::new(RefCell::new(Some(String::from_utf8((*t.borrow().as_ref().unwrap()).clone()).unwrap()))).borrow().as_ref().unwrap())));
}