use std::any::Any;
use std::cell::{RefCell};
use std::rc::{Rc};

pub fn classify(value: Rc<RefCell<Option<Box<dyn Any>>>>) -> Rc<RefCell<Option<String>>> {

    let mut i = Rc::new(RefCell::new(Some(0)));
    while (*i.borrow().as_ref().unwrap()) < 2 {
        {
    let _ts_subject = value.clone();
    let _ts_guard = _ts_subject.borrow();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| __v.as_ref() as &dyn Any);
    if _ts_val.and_then(|__v| __v.downcast_ref::<i32>()).is_some() {
        let v = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<i32>()).unwrap().clone())));
        drop(_ts_guard);
        if (*i.borrow().as_ref().unwrap()) == 0 {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue
    };
        return Rc::new(RefCell::new(Some(format!("int:{}", { let __v = (*v.borrow().as_ref().unwrap()).clone(); __v }))));;
    } else {
        let v = value.clone();
        drop(_ts_guard);
        return Rc::new(RefCell::new(Some("other".to_string())));;
    }
    }
    unreachable!();
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return Rc::new(RefCell::new(Some("none".to_string())));
}

fn main() {
    println!("{}", format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(3) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
}