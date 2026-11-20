use std::any::Any;
use std::cell::{RefCell};
use std::rc::{Rc};

pub fn boxed_int_o_k(v: Rc<RefCell<Option<i64>>>) -> bool {
    let mut boxed = Rc::new(RefCell::new(Some(Box::new((*v.borrow().as_ref().unwrap()).clone()) as Box<dyn Any>)));
    let (_, mut ok) = ({
        let val = boxed.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<i64>() {
                (Rc::new(RefCell::new(Some(typed_val.clone()))), true)
            } else {
                (Rc::new(RefCell::new(Some(0))), false)
            }
        } else {
            (Rc::new(RefCell::new(Some(0))), false)
        }
    });
    ok
}

fn main() {
    println!("{}", format!("{}", boxed_int_o_k(Rc::new(RefCell::new(Some(42))))));
}