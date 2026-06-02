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

#[derive(Default)]
pub struct Cell<T: Any + 'static> {
    pub value: Rc<RefCell<Option<T>>>,
}

impl<T: Any + 'static> Cell<T> {
    pub fn __go_value_clone(&self) -> Self {
        Self { value: self.value.clone() }
    }
}

impl<T: Any + 'static> Clone for Cell<T> {
    fn clone(&self) -> Self {
        self.__go_value_clone()
    }
}

impl<T: Any + 'static> std::fmt::Display for Cell<T> where T: std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.value.borrow().as_ref().unwrap()))
    }
}


impl<T: Any + Clone + 'static> Cell<T> {
    pub fn store(&mut self, value: Rc<RefCell<Option<T>>>) {
        { let new_val = value.borrow().as_ref().unwrap().clone(); *self.value.borrow_mut() = Some(new_val); };
    }

    pub fn load(&self) -> Rc<RefCell<Option<T>>> {
        self.value.clone()
    }
}

pub fn r#use(c: Rc<RefCell<Option<Cell<Box<dyn Any>>>>>) {
    (*c.borrow_mut().as_mut().unwrap()).store(Rc::new(RefCell::new(Some(Box::new("value".to_string()) as Box<dyn Any>))));
    println!("{}", format!("{}", format_any(((*c.borrow().as_ref().unwrap()).load()).borrow().as_ref().unwrap().as_ref())));
}

fn main() {
}