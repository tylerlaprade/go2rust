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

pub fn tail<S: GoByteSequence + Clone + 'static>(mut s: Rc<RefCell<Option<S>>>) -> Rc<RefCell<Option<S>>> {
    { let new_val = Rc::new(RefCell::new(Some({ let __seq = s.clone(); let __low = (1) as usize; let __high = None; let __slice = __seq.borrow().as_ref().unwrap().go_slice(__low, __high); __slice }))); let __moved_val = { let mut __guard = new_val.borrow_mut(); __guard.take() }; *s.borrow_mut() = __moved_val; };
    s.clone()
}

pub fn section<S: GoByteSequence + Clone + 'static>(s: Rc<RefCell<Option<S>>>, low: Rc<RefCell<Option<i32>>>, high: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<S>>> {
    Rc::new(RefCell::new(Some({ let __seq = s.clone(); let __low = ((*low.borrow().as_ref().unwrap())) as usize; let __high = Some(((*high.borrow().as_ref().unwrap())) as usize); let __slice = __seq.borrow().as_ref().unwrap().go_slice(__low, __high); __slice })))
}

pub fn as_text<S: GoByteSequence + Clone + 'static>(s: Rc<RefCell<Option<S>>>) -> Rc<RefCell<Option<String>>> {
    Rc::new(RefCell::new(Some((*Rc::new(RefCell::new(Some({ let __seq = s.clone(); let __low = 0; let __high = Some((2) as usize); let __slice = __seq.borrow().as_ref().unwrap().go_slice(__low, __high); __slice }))).borrow().as_ref().unwrap()).go_to_string())))
}

fn main() {
    {
            let __go_print_arg_0 = format!("{}", (*tail::<String>(Rc::new(RefCell::new(Some("abc".to_string())))).borrow().as_ref().unwrap()));
            eprintln!("{}", __go_print_arg_0)
        };
    {
            let __go_print_arg_0 = format!("{}", (*tail::<Vec<u8>>(Rc::new(RefCell::new(Some(vec![1 as u8, 255 as u8, 128 as u8])))).borrow().as_ref().unwrap())[(0) as usize].clone());
            eprintln!("{}", __go_print_arg_0)
        };
    {
            let __go_print_arg_0 = format!("{}", (*section::<String>(Rc::new(RefCell::new(Some("abcd".to_string()))), Rc::new(RefCell::new(Some(1))), Rc::new(RefCell::new(Some(3)))).borrow().as_ref().unwrap()));
            eprintln!("{}", __go_print_arg_0)
        };
    {
            let __go_print_arg_0 = format!("{}", (*section::<Vec<u8>>(Rc::new(RefCell::new(Some(vec![1 as u8, 255 as u8, 128 as u8]))), Rc::new(RefCell::new(Some(1))), Rc::new(RefCell::new(Some(3)))).borrow().as_ref().unwrap())[(1) as usize].clone());
            eprintln!("{}", __go_print_arg_0)
        };
    {
            let __go_print_arg_0 = format!("{}", (*as_text::<String>(Rc::new(RefCell::new(Some("hello".to_string())))).borrow().as_ref().unwrap()));
            let __go_print_arg_1 = format!("{}", (*as_text::<Vec<u8>>(Rc::new(RefCell::new(Some(vec![103 as u8, 111 as u8, 33 as u8])))).borrow().as_ref().unwrap()));
            eprintln!("{} {}", __go_print_arg_0, __go_print_arg_1)
        };
}