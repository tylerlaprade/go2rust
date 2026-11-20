use std::cell::{RefCell};
use std::rc::{Rc};

pub fn count(values: Rc<RefCell<Option<Vec<u8>>>>) -> i32 {
    (*values.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32
}

fn main() {
    let mut buf: Rc<RefCell<Option<[u8; 128]>>> = Rc::new(RefCell::new(Some(std::array::from_fn::<_, 128, _>(|_| 0))));
    println!("{}", format!("{}", count(Rc::new(RefCell::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = (0) as usize; let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))))));
}