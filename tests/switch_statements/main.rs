use std::any::Any;
use std::cell::{RefCell};
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

pub fn basic_switch(day: Rc<RefCell<Option<i32>>>) {
    { let _switch_val = (*day.borrow().as_ref().unwrap());
    match _switch_val {
        1 => {
            println!("{}", "Monday".to_string());
        }
        2 => {
            println!("{}", "Tuesday".to_string());
        }
        3 => {
            println!("{}", "Wednesday".to_string());
        }
        4 => {
            println!("{}", "Thursday".to_string());
        }
        5 => {
            println!("{}", "Friday".to_string());
        }
        6 | 7 => {
            println!("{}", "Weekend".to_string());
        }
        _ => {
            println!("{}", "Invalid day".to_string());
        }
    } }
}

pub fn switch_with_expression() {
    let mut x = Rc::new(RefCell::new(Some(10)));
    { let _switch_val = (*x.borrow().as_ref().unwrap()) * 2;
    match _switch_val {
        20 => {
            println!("{}", "x * 2 equals 20".to_string());
        }
        30 => {
            println!("{}", "x * 2 equals 30".to_string());
        }
        _ => {
            println!("{}", "x * 2 is something else".to_string());
        }
    } }
}

pub fn switch_without_expression() {
    let mut score = Rc::new(RefCell::new(Some(85)));
    match true {
        true if (*score.borrow().as_ref().unwrap()) >= 90 => {
            println!("{}", "Grade: A".to_string());
        }
        true if (*score.borrow().as_ref().unwrap()) >= 80 => {
            println!("{}", "Grade: B".to_string());
        }
        true if (*score.borrow().as_ref().unwrap()) >= 70 => {
            println!("{}", "Grade: C".to_string());
        }
        true if (*score.borrow().as_ref().unwrap()) >= 60 => {
            println!("{}", "Grade: D".to_string());
        }
        _ => {
            println!("{}", "Grade: F".to_string());
        }
    }
}

pub fn switch_with_fallthrough(num: Rc<RefCell<Option<i32>>>) {
    {
        let _switch_val = (*num.borrow().as_ref().unwrap());
        let mut _fallthrough = false;
        let mut _matched = false;
        if !_matched && (_switch_val == 1) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            println!("{}", "One".to_string());
            _fallthrough = true;
        }
        if !_matched && (_switch_val == 2) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            println!("{}", "Two or after One".to_string());
            _fallthrough = true;
        }
        if !_matched && (_switch_val == 3) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            println!("{}", "Three or after Two or after One".to_string());
        }
        if !_matched || _fallthrough {
            _matched = true;
            _fallthrough = false;
            println!("{}", "Other number".to_string());
        }
    }
}

pub fn type_switch(value: Rc<RefCell<Option<Box<dyn Any>>>>) {
    {
    let _ts_guard = value.borrow();
    let _any_val: &dyn Any = _ts_guard.as_ref().unwrap().as_ref();
    if _any_val.downcast_ref::<i32>().is_some() {
        let v = Rc::new(RefCell::new(Some(_any_val.downcast_ref::<i32>().unwrap().clone())));
        print!("Integer: {}\n", (*v.borrow().as_ref().unwrap()));;
    } else if _any_val.downcast_ref::<String>().is_some() {
        let v = Rc::new(RefCell::new(Some(_any_val.downcast_ref::<String>().unwrap().clone())));
        print!("String: {}\n", (*v.borrow().as_ref().unwrap()));;
    } else if _any_val.downcast_ref::<bool>().is_some() {
        let v = Rc::new(RefCell::new(Some(_any_val.downcast_ref::<bool>().unwrap().clone())));
        print!("Boolean: {}\n", (*v.borrow().as_ref().unwrap()));;
    } else if _any_val.downcast_ref::<f64>().is_some() {
        let v = Rc::new(RefCell::new(Some(_any_val.downcast_ref::<f64>().unwrap().clone())));
        print!("Float: {:.2}\n", (*v.borrow().as_ref().unwrap()));;
    } else {
        let v = _any_val;
        print!("Unknown type: {}\n", go_type_name(v));;
    }
    }
}

fn main() {
    println!("{}", "=== Basic switch ===".to_string());
    basic_switch(Rc::new(RefCell::new(Some(1))));
    basic_switch(Rc::new(RefCell::new(Some(6))));
    basic_switch(Rc::new(RefCell::new(Some(10))));

    println!("{}", "\n=== Switch with expression ===".to_string());
    switch_with_expression();

    println!("{}", "\n=== Switch without expression ===".to_string());
    switch_without_expression();

    println!("{}", "\n=== Switch with fallthrough ===".to_string());
    switch_with_fallthrough(Rc::new(RefCell::new(Some(1))));
    println!("{}", "---".to_string());
    switch_with_fallthrough(Rc::new(RefCell::new(Some(4))));

    println!("{}", "\n=== Type switch ===".to_string());
    type_switch(Rc::new(RefCell::new(Some(Box::new(42) as Box<dyn Any>))));
    type_switch(Rc::new(RefCell::new(Some(Box::new("hello".to_string()) as Box<dyn Any>))));
    type_switch(Rc::new(RefCell::new(Some(Box::new(true) as Box<dyn Any>))));
    type_switch(Rc::new(RefCell::new(Some(Box::new(3.14) as Box<dyn Any>))));
    type_switch(Rc::new(RefCell::new(Some(Box::new(vec![1, 2, 3]) as Box<dyn Any>))));
}