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
    } else if let Some(v) = value.downcast_ref::<i8>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i16>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<isize>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u8>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u16>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<usize>() {
        v.to_string()
    } else {
        panic!("go2rust: cannot print this value held in an interface: its type has no formatting case yet")
    }
}

// Prints a value held inside the one being printed. Nothing this program
// puts in an interface prints differently there.
fn format_any_nested(value: &dyn Any) -> String {
    format_any(value)
}

#[derive(Clone, Default)]
pub struct Cell__any {
    pub value: Rc<RefCell<Option<Box<dyn Any>>>>,
}

impl Cell__any {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.value.clone();
        Self {
            value: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for Cell__any {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", match self.value.borrow().as_ref() { Some(__v) => format_any_nested(__v.as_ref()), None => "<nil>".to_string() });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Cell__any {
    pub fn store(&mut self, value: Rc<RefCell<Option<Box<dyn Any>>>>) {
        { let new_val = value.clone(); self.value = new_val; };
    }

    pub fn load(&self) -> Rc<RefCell<Option<Box<dyn Any>>>> {
        self.value.clone()
    }
}

pub fn r#use(c: Rc<RefCell<Option<Cell__any>>>) {
    (*c.borrow_mut().as_mut().unwrap()).store(Rc::new(RefCell::new(Some(Box::new("value".to_string()) as Box<dyn Any>))));
    println!("{}", format!("{}", format_any(((*c.borrow().as_ref().unwrap()).load()).borrow().as_ref().unwrap().as_ref())));
}

fn main() {
}