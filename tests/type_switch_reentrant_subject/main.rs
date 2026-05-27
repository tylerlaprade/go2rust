use std::any::Any;
use std::cell::{RefCell};
use std::rc::{Rc};

pub fn is_string(v: Rc<RefCell<Option<Box<dyn Any>>>>) -> bool {
    {
    let _ts_subject = v.clone();
    let _ts_guard = _ts_subject.borrow();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| __v.as_ref() as &dyn Any);
    if _ts_val.and_then(|__v| __v.downcast_ref::<String>()).is_some() {
        drop(_ts_guard);
        return true;;
    } else {
        drop(_ts_guard);
        return false;;
    }
    }
    unreachable!()
}

pub fn classify(v: Rc<RefCell<Option<Box<dyn Any>>>>) -> Rc<RefCell<Option<String>>> {
    {
    let _ts_subject = v.clone();
    let _ts_guard = _ts_subject.borrow();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| __v.as_ref() as &dyn Any);
    if _ts_val.and_then(|__v| __v.downcast_ref::<String>()).is_some() {
        drop(_ts_guard);
        if is_string(v.clone()) {
        return Rc::new(RefCell::new(Some("string".to_string())));
    };
    } else {
        drop(_ts_guard);
        return Rc::new(RefCell::new(Some("other".to_string())));;
    }
    }
    Rc::new(RefCell::new(Some("other".to_string())))
}

fn main() {
    println!("{}", format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new("hello".to_string()) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(42) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
}