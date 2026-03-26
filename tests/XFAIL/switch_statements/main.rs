use std::any::Any;
use std::cell::{RefCell};
use std::rc::{Rc};

pub fn basic_switch(day: Rc<RefCell<Option<i32>>>) {
    match (*day.borrow().as_ref().unwrap()) {
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
    }
}

pub fn switch_with_expression() {
    let mut x = Rc::new(RefCell::new(Some(10)));
    match (*x.borrow().as_ref().unwrap()) * 2 {
        20 => {
            println!("{}", "x * 2 equals 20".to_string());
        }
        30 => {
            println!("{}", "x * 2 equals 30".to_string());
        }
        _ => {
            println!("{}", "x * 2 is something else".to_string());
        }
    }
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
    if let Some(v) = (|| -> Option<Box<dyn Any>> {
        let val = (*value.borrow().as_ref().unwrap());
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(val) = any_val.downcast_ref::<i32>() {
                return Some(Box::new(val.clone()) as Box<dyn Any>);
            }
        }
        None
    })() {
        let v = Rc::new(RefCell::new(Some((*v.downcast_ref::<i32>().unwrap()).clone())));
        print!("Integer: {}\n", (*v.borrow().as_ref().unwrap()));;
    } else if let Some(v) = (|| -> Option<Box<dyn Any>> {
        let val = (*value.borrow().as_ref().unwrap());
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(val) = any_val.downcast_ref::<String>() {
                return Some(Box::new(val.clone()) as Box<dyn Any>);
            }
        }
        None
    })() {
        let v = Rc::new(RefCell::new(Some((*v.downcast_ref::<String>().unwrap()).clone())));
        print!("String: {}\n", (*v.borrow().as_ref().unwrap()));;
    } else if let Some(v) = (|| -> Option<Box<dyn Any>> {
        let val = (*value.borrow().as_ref().unwrap());
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(val) = any_val.downcast_ref::<bool>() {
                return Some(Box::new(val.clone()) as Box<dyn Any>);
            }
        }
        None
    })() {
        let v = Rc::new(RefCell::new(Some((*v.downcast_ref::<bool>().unwrap()).clone())));
        print!("Boolean: {}\n", (*v.borrow().as_ref().unwrap()));;
    } else if let Some(v) = (|| -> Option<Box<dyn Any>> {
        let val = (*value.borrow().as_ref().unwrap());
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(val) = any_val.downcast_ref::<f64>() {
                return Some(Box::new(val.clone()) as Box<dyn Any>);
            }
        }
        None
    })() {
        let v = Rc::new(RefCell::new(Some((*v.downcast_ref::<f64>().unwrap()).clone())));
        print!("Float: {:.2}\n", (*v.borrow().as_ref().unwrap()));;
    } else {
        let v = (*value.borrow().as_ref().unwrap());
        print!("Unknown type: <type>\n");;
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
    type_switch(Rc::new(RefCell::new(Some(42))));
    type_switch(Rc::new(RefCell::new(Some("hello".to_string()))));
    type_switch(true.clone());
    type_switch(Rc::new(RefCell::new(Some(3.14))));
    type_switch(Rc::new(RefCell::new(Some(vec![1, 2, 3]))));
}