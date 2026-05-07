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
    if _switch_val == (1) {
            println!("{}", "Monday".to_string());
        } else if _switch_val == (2) {
            println!("{}", "Tuesday".to_string());
        } else if _switch_val == (3) {
            println!("{}", "Wednesday".to_string());
        } else if _switch_val == (4) {
            println!("{}", "Thursday".to_string());
        } else if _switch_val == (5) {
            println!("{}", "Friday".to_string());
        } else if _switch_val == (6) || _switch_val == (7) {
            println!("{}", "Weekend".to_string());
        } else {
            println!("{}", "Invalid day".to_string());
        }
    }
}

pub fn switch_with_expression() {
    let mut x = Rc::new(RefCell::new(Some(10)));
    { let _switch_val = (*x.borrow().as_ref().unwrap()) * 2;
    if _switch_val == (20) {
            println!("{}", "x * 2 equals 20".to_string());
        } else if _switch_val == (30) {
            println!("{}", "x * 2 equals 30".to_string());
        } else {
            println!("{}", "x * 2 is something else".to_string());
        }
    }
}

pub fn switch_without_expression() {
    let mut score = Rc::new(RefCell::new(Some(85)));
    if (*score.borrow().as_ref().unwrap()) >= 90 {
            println!("{}", "Grade: A".to_string());
        } else if (*score.borrow().as_ref().unwrap()) >= 80 {
            println!("{}", "Grade: B".to_string());
        } else if (*score.borrow().as_ref().unwrap()) >= 70 {
            println!("{}", "Grade: C".to_string());
        } else if (*score.borrow().as_ref().unwrap()) >= 60 {
            println!("{}", "Grade: D".to_string());
        } else {
            println!("{}", "Grade: F".to_string());
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
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| __v.as_ref() as &dyn Any);
    if _ts_val.and_then(|__v| __v.downcast_ref::<i32>()).is_some() {
        let v = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<i32>()).unwrap().clone())));
        print!("Integer: {}\n", { let __v = (*v.borrow().as_ref().unwrap()).clone(); __v });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<String>()).is_some() {
        let v = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<String>()).unwrap().clone())));
        print!("String: {}\n", { let __v = (*v.borrow().as_ref().unwrap()).clone(); __v });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<bool>()).is_some() {
        let v = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<bool>()).unwrap().clone())));
        print!("Boolean: {}\n", { let __v = (*v.borrow().as_ref().unwrap()).clone(); __v });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<f64>()).is_some() {
        let v = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<f64>()).unwrap().clone())));
        print!("Float: {:.2}\n", { let __v = (*v.borrow().as_ref().unwrap()).clone(); __v });;
    } else {
        let v = value.clone();
        print!("Unknown type: {}\n", go_type_name(&**v.borrow().as_ref().unwrap()));;
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