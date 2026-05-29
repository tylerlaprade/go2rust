use std::any::Any;
use std::cell::{RefCell};
use std::rc::{Rc};


fn go_any_clone(value: &dyn Any) -> Box<dyn Any> {
    if let Some(v) = value.downcast_ref::<i32>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<i64>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<i8>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<i16>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<u32>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<u64>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<u8>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<u16>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<usize>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<isize>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<f64>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<f32>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<String>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<&'static str>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<bool>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<char>() { return Box::new(*v) as Box<dyn Any>; }
    panic!("go_any_clone: unsupported dynamic type; add typed lowering instead of cloning Box<dyn Any>")
}

pub fn count(args: Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> i32 {
    (*args.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32
}

pub fn label(value: Rc<RefCell<Option<Box<dyn Any>>>>) -> Rc<RefCell<Option<String>>> {
    {
    let _ts_subject = value.clone();
    let _ts_guard = _ts_subject.borrow();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| __v.as_ref() as &dyn Any);
    if _ts_val.and_then(|__v| __v.downcast_ref::<String>()).is_some() {
        let v = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<String>()).unwrap().clone())));
        drop(_ts_guard);
        return { let __owned = v.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };;
    } else {
        let v = value.clone();
        drop(_ts_guard);
        return Rc::new(RefCell::new(Some("other".to_string())));;
    }
    }
    unreachable!()
}

fn main() {
    let mut value: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new("kept".to_string()) as Box<dyn Any>)));
    println!("{}", format!("{}", count(Rc::new(RefCell::new(Some(vec![{ let __any_holder = value.clone(); let __any_guard = __any_holder.borrow(); go_any_clone(__any_guard.as_ref().expect("nil interface in variadic any argument").as_ref()) }]))))));
    println!("{}", format!("{}", (*label(value.clone()).borrow().as_ref().unwrap())));
}