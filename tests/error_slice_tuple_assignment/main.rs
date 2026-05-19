use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

pub fn parse() -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {

    return (Rc::new(RefCell::new(Some(7))), Rc::new(RefCell::new(Some(Box::<dyn std::error::Error>::from("bad".to_string())))));
}

fn main() {
    let mut values = Rc::new(RefCell::new(Some(vec![0; (1) as usize])));
    let mut errs: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Box<dyn StdError>>>>>>>> = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(None::<Box<dyn StdError>>)); (1) as usize])));
    { let (__tmp_0, __tmp_1) = parse(); (*values.borrow_mut().as_mut().unwrap())[(0) as usize] = __tmp_0.borrow_mut().take().unwrap_or_default(); (*errs.borrow_mut().as_mut().unwrap())[(0) as usize] = __tmp_1; };

    println!("{}", format!("{}", (*values.borrow().as_ref().unwrap())[(0) as usize].clone()));
    if (*(*errs.borrow().as_ref().unwrap())[(0) as usize].clone().borrow()).is_some() {
        println!("{}", format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", (*errs.borrow().as_ref().unwrap())[(0) as usize].clone().borrow().as_ref().unwrap())))).borrow().as_ref().unwrap())));
    }
}