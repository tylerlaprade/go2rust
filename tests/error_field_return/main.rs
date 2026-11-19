use std::cell::{RefCell};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Clone, Default)]
pub struct parseValue {
    pub err: Rc<RefCell<Option<Rc<dyn StdError>>>>,
}

impl parseValue {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.err.clone();
        Self {
            err: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for parseValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", match self.err.borrow().as_ref() { Some(__v) => format!("{}", __v), None => "<nil>".to_string() });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


pub fn parse(v: Rc<RefCell<Option<parseValue>>>) -> Rc<RefCell<Option<Rc<dyn StdError>>>> {
    { let __err_source = { let __field = (*v.borrow().as_ref().unwrap()).err.clone(); __field }; let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }
}

pub fn parse_pair(v: Rc<RefCell<Option<parseValue>>>) -> (i32, Rc<RefCell<Option<Rc<dyn StdError>>>>) {
    (7, { let __return_value_1 = { let __err_source = { let __field = (*v.borrow().as_ref().unwrap()).err.clone(); __field }; let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; __return_value_1 })
}

fn main() {
    let mut v = Rc::new(RefCell::new(Some(parseValue { err: Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("bad".to_string()))))), ..Default::default() })));
    let mut err = parse(v.clone());
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", err.borrow().as_ref().unwrap())))).borrow().as_ref().unwrap())));

    let (mut n, __tmp_1) = parse_pair(v.clone()); let __moved_tmp_1 = { let mut __guard = __tmp_1.borrow_mut(); __guard.take() }; *err.borrow_mut() = __moved_tmp_1;;
    println!("{}", format!("{}", n));
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", err.borrow().as_ref().unwrap())))).borrow().as_ref().unwrap())));
}