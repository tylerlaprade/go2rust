use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct parsed {
    pub kind: Rc<RefCell<Option<String>>>,
    pub rest: Rc<RefCell<Option<String>>>,
}

impl std::fmt::Display for parsed {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.kind.borrow().as_ref().unwrap()), (*self.rest.borrow().as_ref().unwrap()))
    }
}


pub fn split(x: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<parsed>>> {

    let mut p: Rc<RefCell<Option<parsed>>> = Rc::new(RefCell::new(Some(Default::default())));
    { let __tmp_0 = Rc::new(RefCell::new(Some({ let __s = (*x.borrow().as_ref().unwrap()).clone(); __s[..(1) as usize].to_string() }))); let __tmp_1 = Rc::new(RefCell::new(Some({ let __s = (*x.borrow().as_ref().unwrap()).clone(); __s[(1) as usize..].to_string() }))); *(*p.borrow().as_ref().unwrap()).kind.borrow_mut() = __tmp_0.borrow_mut().take(); *x.borrow_mut() = __tmp_1.borrow_mut().take(); };
    { let new_val = x.borrow().as_ref().unwrap().clone(); *(*p.borrow().as_ref().unwrap()).rest.borrow_mut() = Some(new_val); };
    return p.clone();
}

fn main() {
    let mut p = split(Rc::new(RefCell::new(Some("abc".to_string()))));
    println!("{}", (*(*p.borrow().as_ref().unwrap()).kind.borrow().as_ref().unwrap()));
    println!("{}", (*(*p.borrow().as_ref().unwrap()).rest.borrow().as_ref().unwrap()));
}