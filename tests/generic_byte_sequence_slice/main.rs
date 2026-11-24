use std::cell::{RefCell};
use std::rc::{Rc};

pub fn tail__string(mut s: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {
    { let new_val = Rc::new(RefCell::new(Some({ let __s = &((*s.borrow().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.borrow_mut(); __guard.take() }; *s.borrow_mut() = __moved_val; };
    { let __owned = s.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) }
}

pub fn tail__slice_byte(mut s: Rc<RefCell<Option<Vec<u8>>>>) -> Rc<RefCell<Option<Vec<u8>>>> {
    { let new_val = Rc::new(RefCell::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (1) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); s = new_val; };
    s.clone()
}

pub fn section__string(s: Rc<RefCell<Option<String>>>, low: Rc<RefCell<Option<i32>>>, high: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<String>>> {
    Rc::new(RefCell::new(Some({ let __s = &((*s.borrow().as_ref().unwrap()).clone()); let __low = ((*low.borrow().as_ref().unwrap())) as usize; let __high = ((*high.borrow().as_ref().unwrap())) as usize; __s[__low..__high].to_string() })))
}

pub fn section__slice_byte(s: Rc<RefCell<Option<Vec<u8>>>>, low: Rc<RefCell<Option<i32>>>, high: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<Vec<u8>>>> {
    Rc::new(RefCell::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ((*low.borrow().as_ref().unwrap())) as usize; let __high = ((*high.borrow().as_ref().unwrap())) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))
}

pub fn as_text__string(s: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {
    Rc::new(RefCell::new(Some((*Rc::new(RefCell::new(Some({ let __s = &((*s.borrow().as_ref().unwrap()).clone()); let __high = (2) as usize; __s[..__high].to_string() }))).borrow().as_ref().unwrap()).to_string())))
}

pub fn as_text__slice_byte(s: Rc<RefCell<Option<Vec<u8>>>>) -> Rc<RefCell<Option<String>>> {
    Rc::new(RefCell::new(Some(String::from_utf8((*Rc::new(RefCell::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = (2) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).borrow().as_ref().unwrap()).clone()).unwrap())))
}

fn main() {
    {
            let __go_print_arg_0 = format!("{}", (*tail__string(Rc::new(RefCell::new(Some("abc".to_string())))).borrow().as_ref().unwrap()));
            eprintln!("{}", __go_print_arg_0)
        };
    {
            let __go_print_arg_0 = format!("{}", (*tail__slice_byte(Rc::new(RefCell::new(Some(vec![1 as u8, 255 as u8, 128 as u8])))).borrow().as_ref().unwrap())[(0) as usize].clone());
            eprintln!("{}", __go_print_arg_0)
        };
    {
            let __go_print_arg_0 = format!("{}", (*section__string(Rc::new(RefCell::new(Some("abcd".to_string()))), Rc::new(RefCell::new(Some(1))), Rc::new(RefCell::new(Some(3)))).borrow().as_ref().unwrap()));
            eprintln!("{}", __go_print_arg_0)
        };
    {
            let __go_print_arg_0 = format!("{}", (*section__slice_byte(Rc::new(RefCell::new(Some(vec![1 as u8, 255 as u8, 128 as u8]))), Rc::new(RefCell::new(Some(1))), Rc::new(RefCell::new(Some(3)))).borrow().as_ref().unwrap())[(1) as usize].clone());
            eprintln!("{}", __go_print_arg_0)
        };
    {
            let __go_print_arg_0 = format!("{}", (*as_text__string(Rc::new(RefCell::new(Some("hello".to_string())))).borrow().as_ref().unwrap()));
            let __go_print_arg_1 = format!("{}", (*as_text__slice_byte(Rc::new(RefCell::new(Some(vec![103 as u8, 111 as u8, 33 as u8])))).borrow().as_ref().unwrap()));
            eprintln!("{} {}", __go_print_arg_0, __go_print_arg_1)
        };
}