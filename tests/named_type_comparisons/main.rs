use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub const INVALID: i8 = 0;
pub const STRING: i8 = 1;
pub const BOOL: i8 = 2;


#[derive(Debug, Clone, Default)]
pub struct Kind(pub Rc<RefCell<Option<i8>>>);

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for Kind {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().as_ref().unwrap() == other.0.borrow().as_ref().unwrap()
    }
}


pub fn different(a: Rc<RefCell<Option<Kind>>>, b: Rc<RefCell<Option<Kind>>>) -> Rc<RefCell<Option<bool>>> {

    return {
            let __tmp_x = (*a.borrow().as_ref().unwrap()).clone();
            let __tmp_y = (*b.borrow().as_ref().unwrap()).clone();
            Rc::new(RefCell::new(Some(__tmp_x != __tmp_y)))
        };
}

pub fn same(a: Rc<RefCell<Option<Kind>>>, b: Rc<RefCell<Option<Kind>>>) -> Rc<RefCell<Option<bool>>> {

    return {
            let __tmp_x = (*a.borrow().as_ref().unwrap()).clone();
            let __tmp_y = (*b.borrow().as_ref().unwrap()).clone();
            Rc::new(RefCell::new(Some(__tmp_x == __tmp_y)))
        };
}

fn main() {
    println!("{}", (*different(Rc::new(RefCell::new(Some(Kind(Rc::new(RefCell::new(Some(INVALID as i8))))))), Rc::new(RefCell::new(Some(Kind(Rc::new(RefCell::new(Some(STRING as i8)))))))).borrow().as_ref().unwrap()));
    println!("{}", (*same(Rc::new(RefCell::new(Some(Kind(Rc::new(RefCell::new(Some(BOOL as i8))))))), Rc::new(RefCell::new(Some(Kind(Rc::new(RefCell::new(Some(BOOL as i8)))))))).borrow().as_ref().unwrap()));
}