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

pub trait Shape: std::fmt::Display + Any {
    fn __go_clone_box_shape(&self) -> Box<dyn Shape>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_shape(&self, other: &dyn Shape) -> bool;
    fn area(&self) -> Rc<RefCell<Option<f64>>>;
}

impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Self {
        self.__go_clone_box_shape()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    pub width: Rc<RefCell<Option<f64>>>,
    pub height: Rc<RefCell<Option<f64>>>,
}

impl Rectangle {
    pub fn __go_value_clone(&self) -> Self {
        Self { width: { let __guard = self.width.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, height: { let __guard = self.height.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Rectangle {
    fn default() -> Self {
        Self { width: Rc::new(RefCell::new(Some(0.0))), height: Rc::new(RefCell::new(Some(0.0))) }
    }
}

impl std::fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.width.borrow().as_ref().unwrap()), (*self.height.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    pub radius: Rc<RefCell<Option<f64>>>,
}

impl Circle {
    pub fn __go_value_clone(&self) -> Self {
        Self { radius: { let __guard = self.radius.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Circle {
    fn default() -> Self {
        Self { radius: Rc::new(RefCell::new(Some(0.0))) }
    }
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.radius.borrow().as_ref().unwrap()))
    }
}


impl Rectangle {
    pub fn area(&self) -> Rc<RefCell<Option<f64>>> {
        return Rc::new(RefCell::new(Some((*self.width.borrow().as_ref().unwrap()) * (*self.height.borrow().as_ref().unwrap()))));
    }
}

impl Shape for Rectangle {
    fn area(&self) -> Rc<RefCell<Option<f64>>> {
        return Rc::new(RefCell::new(Some((*self.width.borrow().as_ref().unwrap()) * (*self.height.borrow().as_ref().unwrap()))));
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape> {
        Box::new(self.clone()) as Box<dyn Shape>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_shape(&self, other: &dyn Shape) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Rectangle>() {
            self == __other
        } else {
            false
        }
    }
}

impl Circle {
    pub fn area(&self) -> Rc<RefCell<Option<f64>>> {
        return Rc::new(RefCell::new(Some(3.14159 as f64 * (*self.radius.borrow().as_ref().unwrap()) * (*self.radius.borrow().as_ref().unwrap()))));
    }
}

impl Shape for Circle {
    fn area(&self) -> Rc<RefCell<Option<f64>>> {
        return Rc::new(RefCell::new(Some(3.14159 as f64 * (*self.radius.borrow().as_ref().unwrap()) * (*self.radius.borrow().as_ref().unwrap()))));
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape> {
        Box::new(self.clone()) as Box<dyn Shape>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_shape(&self, other: &dyn Shape) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Circle>() {
            self == __other
        } else {
            false
        }
    }
}

pub fn process_value(value: Rc<RefCell<Option<Box<dyn Any>>>>) {
        // Basic type assertion
    {
        let (mut str, mut ok) = ({
        let val = value.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<std::string::String>() {
                (Rc::new(RefCell::new(Some(typed_val.clone()))), Rc::new(RefCell::new(Some(true))))
            } else {
                (Rc::new(RefCell::new(Some(std::string::String::new()))), Rc::new(RefCell::new(Some(false))))
            }
        } else {
            (Rc::new(RefCell::new(Some(std::string::String::new()))), Rc::new(RefCell::new(Some(false))))
        }
    });;
        if (*ok.borrow().as_ref().unwrap()) {
            print!("String value: {} (length: {})\n", { let __v = (*str.borrow().as_ref().unwrap()).clone(); __v }, (*str.borrow().as_ref().unwrap()).len());;
            return;;
        }
    }

    {
        let (mut num, mut ok) = ({
        let val = value.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<i32>() {
                (Rc::new(RefCell::new(Some(typed_val.clone()))), Rc::new(RefCell::new(Some(true))))
            } else {
                (Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(false))))
            }
        } else {
            (Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(false))))
        }
    });;
        if (*ok.borrow().as_ref().unwrap()) {
            print!("Integer value: {} (doubled: {})\n", { let __v = (*num.borrow().as_ref().unwrap()).clone(); __v }, (*num.borrow().as_ref().unwrap()) * 2);;
            return;;
        }
    }

    {
        let (mut f, mut ok) = ({
        let val = value.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<f64>() {
                (Rc::new(RefCell::new(Some(typed_val.clone()))), Rc::new(RefCell::new(Some(true))))
            } else {
                (Rc::new(RefCell::new(Some(0.0))), Rc::new(RefCell::new(Some(false))))
            }
        } else {
            (Rc::new(RefCell::new(Some(0.0))), Rc::new(RefCell::new(Some(false))))
        }
    });;
        if (*ok.borrow().as_ref().unwrap()) {
            print!("Float value: {:.2} (squared: {:.2})\n", { let __v = (*f.borrow().as_ref().unwrap()).clone(); __v }, { let __bin_f = (*f.borrow().as_ref().unwrap()).clone(); __bin_f * __bin_f });;
            return;;
        }
    }

    print!("Unknown type: {} with value: {}\n", go_type_name(&**value.borrow().as_ref().unwrap()), format_any(value.borrow().as_ref().unwrap().as_ref()));
}

pub fn assert_without_check(value: Rc<RefCell<Option<Box<dyn Any>>>>) {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        // This will panic if assertion fails
    __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        {
        let mut r = Rc::new(RefCell::new(None::<Box<dyn Any>>));;
        if (*r.borrow()).is_some() {
            print!("Panic recovered: {}\n", format_any(r.borrow().as_ref().unwrap().as_ref()));;
        }
    }
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));

    let mut str = Rc::new(RefCell::new(Some(({
        let val = value.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<String>().expect("type assertion failed").clone()
        } else {
            panic!("type assertion on nil interface")
        }
    }))));
    print!("Asserted string: {}\n", { let __v = (*str.borrow().as_ref().unwrap()).clone(); __v });

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}

pub fn describe_shape(s: &dyn Shape) {
    print!("Shape area: {:.2}\n", (*s.area().borrow().as_ref().unwrap()));

        // Type assertion on interface
    {
        let (mut rect, mut ok) = ({
        let any_val = s.__go_as_any();
        if let Some(typed_val) = any_val.downcast_ref::<Rectangle>() {
            (Rc::new(RefCell::new(Some(typed_val.clone()))), Rc::new(RefCell::new(Some(true))))
        } else {
            (Rc::new(RefCell::new(Some(Default::default()))), Rc::new(RefCell::new(Some(false))))
        }
    });;
        if (*ok.borrow().as_ref().unwrap()) {
            print!("  Rectangle: {:.1} x {:.1}\n", (*(*rect.borrow().as_ref().unwrap()).width.borrow().as_ref().unwrap()), (*(*rect.borrow().as_ref().unwrap()).height.borrow().as_ref().unwrap()));;
        } else {
        let (mut circle, mut ok) = ({
        let any_val = s.__go_as_any();
        if let Some(typed_val) = any_val.downcast_ref::<Circle>() {
            (Rc::new(RefCell::new(Some(typed_val.clone()))), Rc::new(RefCell::new(Some(true))))
        } else {
            (Rc::new(RefCell::new(Some(Default::default()))), Rc::new(RefCell::new(Some(false))))
        }
    });;
        if (*ok.borrow().as_ref().unwrap()) {
            print!("  Circle: radius {:.1}\n", (*(*circle.borrow().as_ref().unwrap()).radius.borrow().as_ref().unwrap()));;
        }
    }
    }
}

fn main() {
        // Test with different types
    let mut values = Rc::new(RefCell::new(Some(vec![Box::new("hello world".to_string()) as Box<dyn Any>, Box::new(42) as Box<dyn Any>, Box::new(3.14159) as Box<dyn Any>, Box::new(true) as Box<dyn Any>, Box::new(Rc::new(RefCell::new(Some(vec![1, 2, 3])))) as Box<dyn Any>])));

    println!("{}", format!("{}", "=== Processing values ===".to_string()));
    { let __range_holder = values.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for val in __range_values.iter() {
        process_value(val.clone());
    } }

    println!("{}", format!("{}", "\n=== Assertion without check ===".to_string()));
    assert_without_check(Rc::new(RefCell::new(Some(Box::new("valid string".to_string()) as Box<dyn Any>))));
    assert_without_check(Rc::new(RefCell::new(Some(Box::new(123) as Box<dyn Any>))));

    println!("{}", format!("{}", "\n=== Interface type assertions ===".to_string()));
    let mut shapes = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(Box::new(Rectangle { width: Rc::new(RefCell::new(Some(10.0 as f64))), height: Rc::new(RefCell::new(Some(5.0 as f64))), ..Default::default() }) as Box<dyn Shape>))), Rc::new(RefCell::new(Some(Box::new(Circle { radius: Rc::new(RefCell::new(Some(3.0 as f64))), ..Default::default() }) as Box<dyn Shape>)))])));

    { let __range_holder = shapes.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for shape in __range_values.iter() {
        describe_shape(shape.borrow().as_ref().unwrap().as_ref());
    } }

    println!("{}", format!("{}", "\n=== Type switch alternative ===".to_string()));
    { let __range_holder = values.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for val in __range_values.iter() {
        {
    let _ts_ref = val;
    let _ts_is_nil = false;
    let _ts_val: Option<&dyn Any> = Some(_ts_ref.as_ref() as &dyn Any);
    if _ts_val.and_then(|__v| __v.downcast_ref::<String>()).is_some() {
        let v = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<String>()).unwrap().clone())));
        print!("String: {}\n", { let __v = (*v.borrow().as_ref().unwrap()).clone(); __v });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<i32>()).is_some() {
        let v = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<i32>()).unwrap().clone())));
        print!("Int: {}\n", { let __v = (*v.borrow().as_ref().unwrap()).clone(); __v });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<f64>()).is_some() {
        let v = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<f64>()).unwrap().clone())));
        print!("Float: {:.2}\n", { let __v = (*v.borrow().as_ref().unwrap()).clone(); __v });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<bool>()).is_some() {
        let v = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<bool>()).unwrap().clone())));
        print!("Bool: {}\n", { let __v = (*v.borrow().as_ref().unwrap()).clone(); __v });;
    } else {
        let v = _ts_val.unwrap();
        print!("Other: {} = {}\n", go_type_name(v), format_any(v.borrow().as_ref().unwrap().as_ref()));;
    }
    }
    } }
}