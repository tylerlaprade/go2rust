use std::any::Any;
use std::cell::{RefCell};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
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
    if val.is::<Record>() { return "main.Record" }
    std::any::type_name_of_val(val)
}

#[derive(Debug, Clone)]
pub struct Record {
    pub name: Rc<RefCell<Option<String>>>,
}

impl Record {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for Record {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}


impl Record {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("{}{}", "record ".to_string(), (*self.name.clone().borrow().as_ref().unwrap())))))
    }
}

impl StdError for Record {}


pub fn run() {
    let mut value = Rc::new(RefCell::new(Some(Record { name: Rc::new(RefCell::new(Some("before".to_string()))), ..Default::default() })));
    let mut direct: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }) as Box<dyn Any>)));
    let mut err: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(Some(Rc::new((*value.borrow().as_ref().unwrap()).__go_value_clone()) as Rc<dyn StdError>)));

    { let new_val = "after".to_string(); *(*value.borrow().as_ref().unwrap()).name.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", (*(*value.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone()), format!("{}", (*({
        let val = direct.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<Record>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "main.Record")).__go_value_clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "main.Record")
        }
    }).name.borrow().as_ref().unwrap()).clone()), format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", err.borrow().as_ref().unwrap())))).borrow().as_ref().unwrap())));

    let mut asserted = Rc::new(RefCell::new(Some(({
        let val = direct.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<Record>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "main.Record")).__go_value_clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "main.Record")
        }
    }))));
    { let new_val = "asserted".to_string(); *(*asserted.borrow().as_ref().unwrap()).name.borrow_mut() = Some(new_val); };
    println!("{} {} {} {}", format!("{}", (*(*value.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone()), format!("{}", (*({
        let val = direct.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<Record>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "main.Record")).__go_value_clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "main.Record")
        }
    }).name.borrow().as_ref().unwrap()).clone()), format!("{}", (*(*asserted.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone()), format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", err.borrow().as_ref().unwrap())))).borrow().as_ref().unwrap())));

    let (mut comma, mut ok) = ({
        let val = direct.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Record>() {
                (Rc::new(RefCell::new(Some(typed_val.__go_value_clone()))), true)
            } else {
                (Rc::new(RefCell::new(Some(Default::default()))), false)
            }
        } else {
            (Rc::new(RefCell::new(Some(Default::default()))), false)
        }
    });
    { let new_val = "comma".to_string(); *(*comma.borrow().as_ref().unwrap()).name.borrow_mut() = Some(new_val); };
    println!("{} {} {} {} {}", format!("{}", (*(*value.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone()), format!("{}", (*({
        let val = direct.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<Record>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "main.Record")).__go_value_clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "main.Record")
        }
    }).name.borrow().as_ref().unwrap()).clone()), format!("{}", (*(*comma.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone()), format!("{}", ok), format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", err.borrow().as_ref().unwrap())))).borrow().as_ref().unwrap())));
}

fn main() {
    run();
}