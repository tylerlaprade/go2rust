use std::any::Any;
use std::fmt::{Display};
use std::sync::{Arc, Mutex};

fn format_slice<T>(slice: &Arc<Mutex<Option<Vec<T>>>>) -> String 
where
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

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

pub fn print_any(v: Arc<Mutex<Option<Box<dyn Any>>>>) {
    println!("{} {}", "Value:".to_string(), format_any(v.lock().unwrap().as_ref().unwrap().as_ref()));
}

fn main() {
        // interface{} can hold any value
    let mut x: Arc<Mutex<Option<Box<dyn Any>>>>;

    { let new_val = Box::new(42) as Box<dyn Any>; *x.lock().unwrap() = Some(new_val); };
    println!("{} {}", "x is int:".to_string(), format_any(x.lock().unwrap().as_ref().unwrap().as_ref()));
    print_any(x.clone());

    { let new_val = Box::new("hello".to_string()) as Box<dyn Any>; *x.lock().unwrap() = Some(new_val); };
    println!("{} {}", "x is string:".to_string(), format_any(x.lock().unwrap().as_ref().unwrap().as_ref()));
    print_any(x.clone());

    { let new_val = Box::new(3.14) as Box<dyn Any>; *x.lock().unwrap() = Some(new_val); };
    println!("{} {}", "x is float:".to_string(), format_any(x.lock().unwrap().as_ref().unwrap().as_ref()));
    print_any(x.clone());

        // interface{} in slice
    let mut values = Arc::new(Mutex::new(Some(vec![Box::new(1) as Box<dyn Any>, Box::new("two".to_string()) as Box<dyn Any>, Box::new(3.0) as Box<dyn Any>])));
    println!("{} {}", "Mixed values:".to_string(), format_slice(&values));
}