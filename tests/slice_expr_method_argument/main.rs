use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct sink {
}

impl sink {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
    }
}

impl std::fmt::Display for sink {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


impl sink {
    pub fn count(&self, values: Rc<RefCell<Option<Vec<u8>>>>) -> i64 {
        (*values.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i64
    }
}

fn main() {
    let mut buf: Rc<RefCell<Option<[u8; 128]>>> = Rc::new(RefCell::new(Some(std::array::from_fn::<_, 128, _>(|_| 0))));
    let mut sink = Rc::new(RefCell::new(Some(sink {  })));
    println!("{}", format!("{}", (*sink.borrow().as_ref().unwrap()).count(Rc::new(RefCell::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = (0) as usize; let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))))));
}