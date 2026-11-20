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
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left == __right
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Path {}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}

impl PartialEq<String> for Path {
    fn eq(&self, other: &String) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialEq<&str> for Path {
    fn eq(&self, other: &&str) -> bool {
        self.0.borrow().as_ref().unwrap().as_str() == *other
    }
}

impl std::ops::Add for Path {
    type Output = Path;
    fn add(self, other: Self) -> Path {
        let __joined = format!("{}{}", self.0.borrow().as_ref().unwrap(), other.0.borrow().as_ref().unwrap());
        Path(Rc::new(RefCell::new(Some(__joined))))
    }
}

impl std::ops::Add<String> for Path {
    type Output = Path;
    fn add(self, other: String) -> Path {
        let __joined = format!("{}{}", self.0.borrow().as_ref().unwrap(), other);
        Path(Rc::new(RefCell::new(Some(__joined))))
    }
}

impl std::ops::Add<&str> for Path {
    type Output = Path;
    fn add(self, other: &str) -> Path {
        let __joined = format!("{}{}", self.0.borrow().as_ref().unwrap(), other);
        Path(Rc::new(RefCell::new(Some(__joined))))
    }
}


pub fn empty_path() -> Rc<RefCell<Option<Path>>> {
    Rc::new(RefCell::new(Some(Path(Rc::new(RefCell::new(Some("".to_string())))))))
}

pub fn from_string(s: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<Path>>> {
    Rc::new(RefCell::new(Some(Path(Rc::new(RefCell::new(Some((*s.borrow().as_ref().unwrap()).clone())))))))
}

pub fn from_bytes(b: Rc<RefCell<Option<Vec<u8>>>>) -> Rc<RefCell<Option<Path>>> {
    Rc::new(RefCell::new(Some(Path(Rc::new(RefCell::new(Some(String::from_utf8((*b.borrow().as_ref().unwrap()).clone()).unwrap())))))))
}

fn main() {
    println!("{}", format!("{}", (*empty_path().borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*from_string(Rc::new(RefCell::new(Some("abc".to_string())))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*from_bytes(Rc::new(RefCell::new(Some(("xy".to_string()).as_bytes().to_vec())))).borrow().as_ref().unwrap())));
}