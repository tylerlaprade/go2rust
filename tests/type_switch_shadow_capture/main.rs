use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};


fn format_any(value: &dyn Any) -> String {
    if let Some(v) = value.downcast_ref::<i32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<String>() {
        v.clone()
    } else if let Some(v) = value.downcast_ref::<&str>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<bool>() {
        v.to_string()
    } else {
        "<unknown>".to_string()
    }
}

#[derive(Debug, Clone, Default)]
pub struct node {
    pub value: Rc<RefCell<Option<i32>>>,
}

impl std::fmt::Display for node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.value.borrow().as_ref().unwrap()))
    }
}


#[derive(Clone, Default)]
pub struct holder {
    pub value: Rc<RefCell<Option<Box<dyn Any>>>>,
}

impl std::fmt::Display for holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_any(self.value.borrow().as_ref().unwrap().as_ref()))
    }
}


impl node {
    pub fn value(&self) -> Rc<RefCell<Option<i32>>> {
        return self.value.clone();
    }
}

impl holder {
    pub fn current(&self) -> Rc<RefCell<Option<Box<dyn Any>>>> {
        return self.value.clone();
    }
}

fn main() {
    let mut T = holder { value: Rc::new(RefCell::new(Some(Box::new(node { value: Rc::new(RefCell::new(Some(7))), ..Default::default() }) as Box<dyn Any>))), ..Default::default() }.current();
    let T_closure_clone = T.clone(); let mut visit = Rc::new(RefCell::new(Some(Box::new(move || -> Rc<RefCell<Option<i32>>> {
        {
    let _ts_subject = T.clone();
    let _ts_guard = _ts_subject.borrow();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| __v.as_ref() as &dyn Any);
    if _ts_val.and_then(|__v| __v.downcast_ref::<node>()).is_some() {
        let T = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<node>()).unwrap().clone())));
        return (*T.borrow().as_ref().unwrap()).value();;
    } else {
        let T = T.clone();
        return Rc::new(RefCell::new(Some(0)));;
    }
    }
    }) as Box<dyn Fn() -> Rc<RefCell<Option<i32>>>>)));
    println!("{}", (*{ let __f_guard = visit.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)() }.borrow().as_ref().unwrap()));
}