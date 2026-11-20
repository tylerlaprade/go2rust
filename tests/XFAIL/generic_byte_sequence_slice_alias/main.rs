use std::cell::{RefCell};
use std::rc::{Rc};

pub fn tail__slice_byte(s: Rc<RefCell<Option<Vec<u8>>>>) -> Rc<RefCell<Option<Vec<u8>>>> {
    Rc::new(RefCell::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (1) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))
}

fn main() {
    let mut data = Rc::new(RefCell::new(Some(vec![10 as u8, 20 as u8, 30 as u8])));
    let mut view = tail__slice_byte(data.clone());
    { let __idx = (0) as usize; let __rhs = 99 as u8; (*view.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    {
            let __go_print_arg_0 = format!("{}", (*data.borrow().as_ref().unwrap())[(1) as usize].clone());
            let __go_print_arg_1 = format!("{}", (*view.borrow().as_ref().unwrap())[(0) as usize].clone());
            eprintln!("{} {}", __go_print_arg_0, __go_print_arg_1)
        };
}