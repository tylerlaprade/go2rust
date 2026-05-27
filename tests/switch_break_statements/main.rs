use std::any::Any;
use std::cell::{RefCell};
use std::rc::{Rc};

pub fn regular(v: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<String>>> {
    let mut result = Rc::new(RefCell::new(Some("start".to_string())));
    { let _switch_val = (*v.borrow().as_ref().unwrap());
    if _switch_val == (1) {
            { let new_val = "one".to_string(); *result.borrow_mut() = Some(new_val); };
        } else {
            { let new_val = "other".to_string(); *result.borrow_mut() = Some(new_val); };
        }
    }
    Rc::new(RefCell::new(Some(result.borrow().as_ref().unwrap().clone())))
}

pub fn typed(v: Rc<RefCell<Option<Box<dyn Any>>>>) -> Rc<RefCell<Option<String>>> {
    let mut result = Rc::new(RefCell::new(Some("start".to_string())));
    {
    let _ts_subject = v.clone();
    let _ts_guard = _ts_subject.borrow();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| __v.as_ref() as &dyn Any);
    if _ts_is_nil || _ts_val.and_then(|__v| __v.downcast_ref::<i32>()).is_some() {
        drop(_ts_guard);
        { let new_val = "simple".to_string(); *result.borrow_mut() = Some(new_val); };;
    } else {
        drop(_ts_guard);
        { let new_val = "other".to_string(); *result.borrow_mut() = Some(new_val); };;
    }
    }
    Rc::new(RefCell::new(Some(result.borrow().as_ref().unwrap().clone())))
}

pub fn nested_regular(v: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<String>>> {
    let mut result = Rc::new(RefCell::new(Some("start".to_string())));
    '__go_switch_1: loop {
        { let _switch_val = (*v.borrow().as_ref().unwrap());
    if _switch_val == (1) {
            if true {
        { let new_val = "one".to_string(); *result.borrow_mut() = Some(new_val); };
        break '__go_switch_1
    }
            { let new_val = "bad".to_string(); *result.borrow_mut() = Some(new_val); };
        } else {
            { let new_val = "other".to_string(); *result.borrow_mut() = Some(new_val); };
        }
    };
        break;
    }
    Rc::new(RefCell::new(Some(format!("{}{}", (*result.borrow().as_ref().unwrap()), "-done".to_string()))))
}

pub fn nested_typed(v: Rc<RefCell<Option<Box<dyn Any>>>>) -> Rc<RefCell<Option<String>>> {
    let mut result = Rc::new(RefCell::new(Some("start".to_string())));
    '__go_switch_2: loop {
    {
    let _ts_subject = v.clone();
    let _ts_guard = _ts_subject.borrow();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| __v.as_ref() as &dyn Any);
    if _ts_val.and_then(|__v| __v.downcast_ref::<i32>()).is_some() {
        drop(_ts_guard);
        if true {
        { let new_val = "int".to_string(); *result.borrow_mut() = Some(new_val); };
        break '__go_switch_2
    };
        { let new_val = "bad".to_string(); *result.borrow_mut() = Some(new_val); };;
    } else {
        drop(_ts_guard);
        { let new_val = "other".to_string(); *result.borrow_mut() = Some(new_val); };;
    }
    };
    break;
}
    Rc::new(RefCell::new(Some(format!("{}{}", (*result.borrow().as_ref().unwrap()), "-done".to_string()))))
}

fn main() {
    println!("{}", format!("{}", (*regular(Rc::new(RefCell::new(Some(1)))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*regular(Rc::new(RefCell::new(Some(2)))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*nested_regular(Rc::new(RefCell::new(Some(1)))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*nested_regular(Rc::new(RefCell::new(Some(2)))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*nested_typed(Rc::new(RefCell::new(Some(Box::new(1) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*nested_typed(Rc::new(RefCell::new(Some(Box::new("x".to_string()) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    if false {
        println!("{}", format!("{}", (*typed(Rc::new(RefCell::new(Some(Box::new(1) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    }
}