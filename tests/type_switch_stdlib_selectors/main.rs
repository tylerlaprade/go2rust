use std::cell::{RefCell};
use std::rc::{Rc};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Basic;

impl std::fmt::Display for types_Basic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Basic>")
    }
}


impl types_Basic {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Named;

impl std::fmt::Display for types_Named {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Named>")
    }
}


impl types_Named {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Type;

impl std::fmt::Display for types_Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Type>")
    }
}


impl types_Type {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


pub fn classify(t: Rc<RefCell<Option<types_Type>>>) -> Rc<RefCell<Option<String>>> {

    {
    let _ts_subject = t.clone();
    let _ts_guard = _ts_subject.borrow();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val = _ts_guard.as_ref();
    if _ts_is_nil || _ts_val.and_then(|__v| __v.downcast_ref::<types_Basic>()).is_some() {
        let x = t.clone();
        return Rc::new(RefCell::new(Some("nil-or-basic".to_string())));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<types_Named>()).is_some() {
        let x = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<types_Named>()).unwrap().clone())));
        let _ = (*x.borrow().as_ref().unwrap());;
        return Rc::new(RefCell::new(Some("named".to_string())));;
    } else {
        let x = t.clone();
        return Rc::new(RefCell::new(Some("other".to_string())));;
    }
    }
    unreachable!()
}

fn main() {
    if false {
        let _ = classify(Rc::new(RefCell::new(None)));
    }
    println!("{}", "ok".to_string());
}