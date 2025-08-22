use std::any::Any;
use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut x: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new("hello".to_string()) as Box<dyn Any>)));

        // Type assertion with comma-ok
    let (mut s, mut ok) = ({
        let val = x.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<String>() {
                (Rc::new(RefCell::new(Some(typed_val.clone()))), Rc::new(RefCell::new(Some(true))))
            } else {
                (Rc::new(RefCell::new(Some(String::new()))), Rc::new(RefCell::new(Some(false))))
            }
        } else {
            (Rc::new(RefCell::new(Some(String::new()))), Rc::new(RefCell::new(Some(false))))
        }
    });
    if (*ok.borrow_mut().as_mut().unwrap()) {
        println!("{} {}", "x is string:".to_string(), (*s.borrow_mut().as_mut().unwrap()));
    }

        // Type assertion without comma-ok (would panic if wrong)
    let mut str = Rc::new(RefCell::new(Some(({
        let val = x.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<String>().expect("type assertion failed").clone()
        } else {
            panic!("type assertion on nil interface")
        }
    }))));
    println!("{} {}", "Asserted string:".to_string(), (*str.borrow_mut().as_mut().unwrap()));

        // Failed assertion with comma-ok
    let (mut n, mut ok) = ({
        let val = x.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<i32>() {
                (Rc::new(RefCell::new(Some(typed_val.clone()))), Rc::new(RefCell::new(Some(true))))
            } else {
                (Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(false))))
            }
        } else {
            (Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(false))))
        }
    });
    if (*ok.borrow_mut().as_mut().unwrap()) {
        println!("{} {}", "x is int:".to_string(), (*n.borrow_mut().as_mut().unwrap()));
    } else {
        println!("{}", "x is not an int".to_string());
    }
}