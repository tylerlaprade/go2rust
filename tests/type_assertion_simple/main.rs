use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display};
use std::rc::{Rc};


fn __go_type_name(val: &dyn Any) -> &'static str {
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

pub trait hasName: std::fmt::Display + Any {
    fn __go_clone_box_has_name(&self) -> Box<dyn hasName>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_has_name(&self, other: &dyn hasName) -> bool;
    fn name(&self) -> Rc<RefCell<Option<String>>>;
}

impl Clone for Box<dyn hasName> {
    fn clone(&self) -> Self {
        hasName::__go_clone_box_has_name(self.as_ref())
    }
}

pub trait hasNameAndString: hasName + std::fmt::Display + Any {
    fn __go_clone_box_has_name_and_string(&self) -> Box<dyn hasNameAndString>;
    fn __go_eq_has_name_and_string(&self, other: &dyn hasNameAndString) -> bool;
    fn string(&self) -> Rc<RefCell<Option<String>>>;
}

impl Clone for Box<dyn hasNameAndString> {
    fn clone(&self) -> Self {
        hasNameAndString::__go_clone_box_has_name_and_string(self.as_ref())
    }
}

impl hasName for Box<dyn hasNameAndString> {
    fn __go_clone_box_has_name(&self) -> Box<dyn hasName> {
        Box::new((*self).clone()) as Box<dyn hasName>
    }
    fn __go_as_any(&self) -> &dyn Any {
        (**self).__go_as_any()
    }
    fn __go_eq_has_name(&self, other: &dyn hasName) -> bool {
        (**self).__go_eq_has_name(other)
    }
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        (**self).name()
    }
}

pub fn asserted_anonymous_interface(v: Rc<RefCell<Option<Box<dyn hasNameAndString>>>>) -> bool {
    let (_, mut ok) = ({
        let __asserted = v.clone();
        (__asserted.clone(), true)
    });
    ok
}

fn main() {
    let mut x: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new("hello".to_string()) as Box<dyn Any>)));

        // Type assertion with comma-ok
    {
        let (mut s, mut ok) = ({
        let val = x.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<std::string::String>() {
                (Rc::new(RefCell::new(Some(typed_val.clone()))), true)
            } else {
                (Rc::new(RefCell::new(Some(std::string::String::new()))), false)
            }
        } else {
            (Rc::new(RefCell::new(Some(std::string::String::new()))), false)
        }
    });;
        if ok {
            println!("{} {}", format!("{}", "x is string:".to_string()), format!("{}", { let __v = (*s.borrow().as_ref().unwrap()).clone(); __v }));;
        }
    }

        // Type assertion without comma-ok (would panic if wrong)
    let mut str = Rc::new(RefCell::new(Some(({
        let val = x.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<String>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "string")).clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "string")
        }
    }))));
    println!("{} {}", format!("{}", "Asserted string:".to_string()), format!("{}", { let __v = (*str.borrow().as_ref().unwrap()).clone(); __v }));

        // Failed assertion with comma-ok
    {
        let (mut n, mut ok) = ({
        let val = x.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<i32>() {
                (Rc::new(RefCell::new(Some(typed_val.clone()))), true)
            } else {
                (Rc::new(RefCell::new(Some(0))), false)
            }
        } else {
            (Rc::new(RefCell::new(Some(0))), false)
        }
    });;
        if ok {
            println!("{} {}", format!("{}", "x is int:".to_string()), format!("{}", { let __v = (*n.borrow().as_ref().unwrap()).clone(); __v }));;
        } else {
            println!("{}", format!("{}", "x is not an int".to_string()));;
        }
    }
}