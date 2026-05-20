use std::any::Any;
use std::cell::{RefCell};
use std::rc::{Rc};

pub fn classify(v: Rc<RefCell<Option<Box<dyn Any>>>>) -> Rc<RefCell<Option<String>>> {

    {
    let _ts_subject = v.clone();
    let _ts_guard = _ts_subject.borrow();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| __v.as_ref() as &dyn Any);
    if _ts_val.and_then(|__v| __v.downcast_ref::<i32>()).is_some() {
        return Rc::new(RefCell::new(Some("int".to_string())));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<String>()).is_some() {
        return Rc::new(RefCell::new(Some("string".to_string())));;
    } else {
        drop(_ts_guard);
        return Rc::new(RefCell::new(Some("other".to_string())));;
    }
    }
    unreachable!()
}

pub fn nested(v: Rc<RefCell<Option<Box<dyn Any>>>>) -> Rc<RefCell<Option<String>>> {

    {
    let _ts_subject = v.clone();
    let _ts_guard = _ts_subject.borrow();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| __v.as_ref() as &dyn Any);
    if _ts_val.and_then(|__v| __v.downcast_ref::<i32>()).is_some() {
        {
    let _ts_subject = v.clone();
    let _ts_guard = _ts_subject.borrow();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| __v.as_ref() as &dyn Any);
    if _ts_val.and_then(|__v| __v.downcast_ref::<i32>()).is_some() {
        return Rc::new(RefCell::new(Some("nested-int".to_string())));;
    } else {
        drop(_ts_guard);
        return Rc::new(RefCell::new(Some("nested-other".to_string())));;
    }
    }
    unreachable!();
    } else {
        drop(_ts_guard);
        return Rc::new(RefCell::new(Some("other".to_string())));;
    }
    }
    unreachable!()
}

fn main() {
    println!("{}", format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(1) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new("x".to_string()) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(false) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*nested(Rc::new(RefCell::new(Some(Box::new(1) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
}