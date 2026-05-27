use std::any::Any;
use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};


fn go_type_name(val: &dyn Any) -> &'static str {
    if val.is::<i32>() { return "int" }
    if val.is::<i64>() { return "int64" }
    if val.is::<i8>() { return "int8" }
    if val.is::<i16>() { return "int16" }
    if val.is::<u32>() { return "uint" }
    if val.is::<u64>() { return "uint64" }
    if val.is::<u8>() { return "uint8" }
    if val.is::<u16>() { return "uint16" }
    if val.is::<f64>() { return "float64" }
    if val.is::<f32>() { return "float32" }
    if val.is::<bool>() { return "bool" }
    if val.is::<String>() { return "string" }
    if val.is::<Vec<i32>>() { return "[]int" }
    if val.is::<Vec<i64>>() { return "[]int64" }
    if val.is::<Vec<f64>>() { return "[]float64" }
    if val.is::<Vec<String>>() { return "[]string" }
    if val.is::<Vec<bool>>() { return "[]bool" }
    std::any::type_name_of_val(val)
}

pub fn label() -> Rc<RefCell<Option<String>>> {
    Rc::new(RefCell::new(Some("x".to_string())))
}

fn main() {
    let mut err = Rc::new(RefCell::new(Some(Box::<dyn StdError>::from(format!("bad type {}", go_type_name(&(*label().borrow().as_ref().unwrap())))))));
    println!("{}", format!("{}", format!("{}", (*err.borrow().as_ref().unwrap()))));
    let mut value = label();
    println!("{}", format!("{}", format!("{}", (*(Rc::new(RefCell::new(Some(Box::<dyn StdError>::from(format!("stored type {}", go_type_name(value.borrow().as_ref().unwrap()))))))).borrow().as_ref().unwrap()))));
}