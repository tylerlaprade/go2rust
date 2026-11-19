use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

pub fn parse() -> (i32, Rc<RefCell<Option<Rc<dyn StdError>>>>) {
    (7, Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("bad".to_string()))))))
}

fn main() {
    let mut values = Rc::new(RefCell::new(Some(vec![0; (1) as usize])));
    let mut errs: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Rc<dyn StdError>>>>>>>> = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(None::<Rc<dyn StdError>>)); (1) as usize])));
    { let (__tmp_0, __tmp_1) = parse(); { let __idx = (0) as usize; (*values.borrow_mut().as_mut().unwrap())[__idx] = __tmp_0; } { let __idx = (0) as usize; (*errs.borrow_mut().as_mut().unwrap())[__idx] = __tmp_1; } };

    println!("{}", format!("{}", (*values.borrow().as_ref().unwrap())[(0) as usize].clone()));
    if { let __nil_result = (*(*errs.borrow().as_ref().unwrap())[(0) as usize].clone().borrow()).is_some(); __nil_result } {
        println!("{}", format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", (*errs.borrow().as_ref().unwrap())[(0) as usize].clone().borrow().as_ref().unwrap())))).borrow().as_ref().unwrap())));
    }
}