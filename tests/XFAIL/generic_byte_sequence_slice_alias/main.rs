use std::cell::{RefCell};
use std::rc::{Rc};


pub trait GoByteSequence: Clone {
    fn go_len(&self) -> usize;
    fn go_byte(&self, index: usize) -> u8;
    fn go_slice(&self, start: usize, end: Option<usize>) -> Self;

    fn go_to_string(&self) -> String;
}

impl GoByteSequence for String {
    fn go_len(&self) -> usize {
        self.len()
    }

    fn go_byte(&self, index: usize) -> u8 {
        self.as_bytes()[index]
    }

    fn go_slice(&self, start: usize, end: Option<usize>) -> Self {
        let end = end.unwrap_or_else(|| self.len());
        self[start..end].to_string()
    }

    fn go_to_string(&self) -> String {
        self.clone()
    }
}

impl GoByteSequence for Vec<u8> {
    fn go_len(&self) -> usize {
        self.len()
    }

    fn go_byte(&self, index: usize) -> u8 {
        self[index]
    }

    fn go_slice(&self, start: usize, end: Option<usize>) -> Self {
        let end = end.unwrap_or_else(|| self.len());
        self[start..end].to_vec()
    }

    fn go_to_string(&self) -> String {
        String::from_utf8(self.clone()).unwrap()
    }
}

pub fn tail<S: GoByteSequence + Clone + 'static>(s: Rc<RefCell<Option<S>>>) -> Rc<RefCell<Option<S>>> {
    Rc::new(RefCell::new(Some({ let __seq = s.clone(); let __low = (1) as usize; let __high = None; let __slice = __seq.borrow().as_ref().unwrap().go_slice(__low, __high); __slice })))
}

fn main() {
    let mut data = Rc::new(RefCell::new(Some(vec![10 as u8, 20 as u8, 30 as u8])));
    let mut view = tail::<Vec<u8>>(Rc::new(RefCell::new(Some({ let __arg_holder = data.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    { let __idx = (0) as usize; let __rhs = 99 as u8; (*view.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    {
            let __go_print_arg_0 = format!("{}", (*data.borrow().as_ref().unwrap())[(1) as usize].clone());
            let __go_print_arg_1 = format!("{}", (*view.borrow().as_ref().unwrap())[(0) as usize].clone());
            eprintln!("{} {}", __go_print_arg_0, __go_print_arg_1)
        };
}