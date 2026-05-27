use std::any::Any;
use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

fn main() {
    let mut f: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(Box::new(move || -> (Rc<RefCell<Option<String>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        (Rc::new(RefCell::new(Some("ok".to_string()))), Rc::new(RefCell::new(None)))
    }) as Box<dyn FnMut() -> (Rc<RefCell<Option<String>>>, Rc<RefCell<Option<Box<dyn StdError>>>>)>) as Box<dyn Any>)));

    let (mut s, mut err) = ({
        let val = f.clone();
        let mut guard = val.borrow_mut();
        if let Some(ref mut any_val) = *guard {
            let __f = any_val.downcast_mut::<Box<dyn FnMut() -> (Rc<RefCell<Option<String>>>, Rc<RefCell<Option<Box<dyn StdError>>>>)>>().expect("type assertion failed");
            (*__f)()
        } else {
            panic!("type assertion on nil interface")
        }
    });
    println!("{} {}", format!("{}", { let __v = (*s.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", (*err.borrow()).is_none()));
}