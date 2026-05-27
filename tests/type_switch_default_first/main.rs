use std::any::Any;
use std::cell::{RefCell};
use std::rc::{Rc};

pub fn classify(v: Rc<RefCell<Option<Box<dyn Any>>>>) -> Rc<RefCell<Option<String>>> {
    let mut result = Rc::new(RefCell::new(Some("".to_string())));
    {
    let _ts_subject = v.clone();
    let _ts_guard = _ts_subject.borrow();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| __v.as_ref() as &dyn Any);
    if _ts_val.and_then(|__v| __v.downcast_ref::<i32>()).is_some() {
        drop(_ts_guard);
        { let new_val = "int".to_string(); *result.borrow_mut() = Some(new_val); };;
    } else {
        drop(_ts_guard);
        { let new_val = "other".to_string(); *result.borrow_mut() = Some(new_val); };;
    }
    }
    return Rc::new(RefCell::new(Some(result.borrow().as_ref().unwrap().clone())));
}

fn main() {
    println!("{}", format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(7) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new("x".to_string()) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
}