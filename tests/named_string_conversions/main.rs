use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct Path(pub Rc<RefCell<Option<String>>>);

impl Display for Path {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().as_ref().unwrap() == other.0.borrow().as_ref().unwrap()
    }
}


pub fn empty_path() -> Rc<RefCell<Option<Path>>> {

    return Rc::new(RefCell::new(Some(Path(Rc::new(RefCell::new(Some("".to_string())))))));
}

pub fn from_string(s: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<Path>>> {

    return Rc::new(RefCell::new(Some(Path(Rc::new(RefCell::new(Some((*s.borrow().as_ref().unwrap()).clone())))))));
}

pub fn from_bytes(b: Rc<RefCell<Option<Vec<u8>>>>) -> Rc<RefCell<Option<Path>>> {

    return Rc::new(RefCell::new(Some(Path(Rc::new(RefCell::new(Some(String::from_utf8((*b.borrow().as_ref().unwrap()).clone()).unwrap())))))));
}

fn main() {
    println!("{}", format!("{}", (*empty_path().borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*from_string(Rc::new(RefCell::new(Some("abc".to_string())))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*from_bytes(Rc::new(RefCell::new(Some(("xy".to_string()).as_bytes().to_vec())))).borrow().as_ref().unwrap())));
}