use std::any::Any;
use std::cell::{RefCell};
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

fn format_any_slice(slice: &Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> String {
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.iter().map(|v| format_any(v.as_ref())).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

pub fn print_any(v: Rc<RefCell<Option<Box<dyn Any>>>>) {
    println!("{} {}", "Value:".to_string(), format_any(v.borrow().as_ref().unwrap().as_ref()));
}

fn main() {
        // interface{} can hold any value
    let mut x: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(None));

    { let new_val = Box::new(42) as Box<dyn Any>; *x.borrow_mut() = Some(new_val); };
    println!("{} {}", "x is int:".to_string(), format_any(x.borrow().as_ref().unwrap().as_ref()));
    print_any(x.clone());

    { let new_val = Box::new("hello".to_string()) as Box<dyn Any>; *x.borrow_mut() = Some(new_val); };
    println!("{} {}", "x is string:".to_string(), format_any(x.borrow().as_ref().unwrap().as_ref()));
    print_any(x.clone());

    { let new_val = Box::new(3.14) as Box<dyn Any>; *x.borrow_mut() = Some(new_val); };
    println!("{} {}", "x is float:".to_string(), format_any(x.borrow().as_ref().unwrap().as_ref()));
    print_any(x.clone());

        // interface{} in slice
    let mut values = Rc::new(RefCell::new(Some(vec![Box::new(1) as Box<dyn Any>, Box::new("two".to_string()) as Box<dyn Any>, Box::new(3.0) as Box<dyn Any>])));
    println!("{} {}", "Mixed values:".to_string(), format_any_slice(&values));
}