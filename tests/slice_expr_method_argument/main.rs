use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct sink {
}

impl sink {
    pub fn __go_value_clone(&self) -> Self {
        Self {  }
    }
}

impl std::fmt::Display for sink {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


impl sink {
    pub fn count(&self, values: Rc<RefCell<Option<Vec<u8>>>>) -> i32 {
        (*values.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32
    }
}

fn main() {
    let mut buf: Rc<RefCell<Option<[u8; 128]>>> = Rc::new(RefCell::new(Some(std::array::from_fn(|_| 0))));
    let mut sink = Rc::new(RefCell::new(Some(sink {  })));
    println!("{}", format!("{}", (*sink.borrow().as_ref().unwrap()).count(Rc::new(RefCell::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.borrow(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[..(0) as usize].to_vec() }))))));
}