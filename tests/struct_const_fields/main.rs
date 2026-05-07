use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub const INVALID: i8 = 0;
pub const TYPE: i8 = 1;
pub const FUNC: i8 = 2;
pub const FIELD: i8 = 3;


#[derive(Debug, Clone)]
pub struct Kind(pub Rc<RefCell<Option<i8>>>);

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}


#[derive(Debug, Clone)]
pub struct Version(pub Rc<RefCell<Option<i8>>>);

impl Display for Version {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}


#[derive(Debug, Clone, Default)]
pub struct Symbol {
    pub name: Rc<RefCell<Option<String>>>,
    pub kind: Rc<RefCell<Option<Kind>>>,
    pub version: Rc<RefCell<Option<Version>>>,
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.name.borrow().as_ref().unwrap()), (*self.kind.borrow().as_ref().unwrap()), (*self.version.borrow().as_ref().unwrap()))
    }
}


fn main() {
    let mut sym = Rc::new(RefCell::new(Some(Symbol { name: Rc::new(RefCell::new(Some("Println".to_string()))), kind: Rc::new(RefCell::new(Some(Kind(Rc::new(RefCell::new(Some(FUNC as i8))))))), version: Rc::new(RefCell::new(Some(Version(Rc::new(RefCell::new(Some(1 as i8))))))), ..Default::default() })));
    let mut field = Rc::new(RefCell::new(Some(Symbol { name: Rc::new(RefCell::new(Some("Point.X".to_string()))), kind: Rc::new(RefCell::new(Some(Kind(Rc::new(RefCell::new(Some(FIELD as i8))))))), version: Rc::new(RefCell::new(Some(Version(Rc::new(RefCell::new(Some(0 as i8))))))), ..Default::default() })));

    println!("{} {} {}", (*(*sym.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()), (*(*sym.borrow().as_ref().unwrap()).kind.borrow().as_ref().unwrap()), (*(*sym.borrow().as_ref().unwrap()).version.borrow().as_ref().unwrap()));
    println!("{} {} {}", (*(*field.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()), (*(*field.borrow().as_ref().unwrap()).kind.borrow().as_ref().unwrap()), (*(*field.borrow().as_ref().unwrap()).version.borrow().as_ref().unwrap()));
}