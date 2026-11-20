use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

fn format_slice<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[T]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn format_slice_values<T>(slice: &[T]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| v.to_string()).collect();
    format!("[{}]", formatted.join(" "))
}

fn format_slice_wrapped<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[Rc<RefCell<Option<T>>>]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| {
            let inner = v.borrow();
            match inner.as_ref() {
                Some(value) => format!("&{}", value),
                None => "<nil>".to_string(),
            }
        }).collect();
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
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<i32>>>>>() {
        format_slice(v)
    } else {
        panic!("go2rust: cannot print this value held in an interface: its type has no formatting case yet")
    }
}

fn go_any_clone(value: &dyn Any) -> Box<dyn Any> {
    if let Some(v) = value.downcast_ref::<i32>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<i64>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<i8>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<i16>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<u32>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<u64>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<u8>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<u16>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<usize>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<isize>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<f64>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<f32>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<String>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<&'static str>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<bool>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<char>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<i32>>>>>() { return Box::new(v.clone()) as Box<dyn Any>; }

    panic!("go_any_clone: unsupported dynamic type; add typed lowering instead of cloning Box<dyn Any>")
}

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
    if val.is::<Rc<RefCell<Option<Vec<i32>>>>>() { return "[]int" }
    std::any::type_name_of_val(val)
}

pub trait Shape: std::fmt::Display + Any {
    fn __go_clone_box_shape(&self) -> Box<dyn Shape>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_shape(&self, other: &dyn Shape) -> bool;
    fn area(&self) -> f64;
}

impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Self {
        Shape::__go_clone_box_shape(self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    pub width: Rc<RefCell<Option<f64>>>,
    pub height: Rc<RefCell<Option<f64>>>,
}

impl Rectangle {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.width.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.height.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            width: __go_clone_0_0,
            height: __go_clone_0_1,
        }
    }
}


impl Default for Rectangle {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0.0)));
        let __go_default_0_1 = Rc::new(RefCell::new(Some(0.0)));
        Self {
            width: __go_default_0_0,
            height: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.width.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.height.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    pub radius: Rc<RefCell<Option<f64>>>,
}

impl Circle {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.radius.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            radius: __go_clone_0_0,
        }
    }
}


impl Default for Circle {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0.0)));
        Self {
            radius: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.radius.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Rectangle {
    pub fn area(&self) -> f64 {
        return (*self.width.borrow().as_ref().unwrap()) * (*self.height.borrow().as_ref().unwrap());
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        Rectangle::area(self)
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

#[derive(Clone)]
pub struct RectanglePtr(pub Rc<RefCell<Option<Rectangle>>>);

impl std::fmt::Display for RectanglePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Shape for RectanglePtr {
    fn area(&self) -> f64 {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        Rectangle::area(__recv)
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape> {
        Box::new(self.clone()) as Box<dyn Shape>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_shape(&self, other: &dyn Shape) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<RectanglePtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Circle {
    pub fn area(&self) -> f64 {
        3.14159 * (*self.radius.borrow().as_ref().unwrap()) * (*self.radius.borrow().as_ref().unwrap())
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        Circle::area(self)
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

#[derive(Clone)]
pub struct CirclePtr(pub Rc<RefCell<Option<Circle>>>);

impl std::fmt::Display for CirclePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Shape for CirclePtr {
    fn area(&self) -> f64 {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        Circle::area(__recv)
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape> {
        Box::new(self.clone()) as Box<dyn Shape>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_shape(&self, other: &dyn Shape) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<CirclePtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
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
                (Rc::new(RefCell::new(Some(typed_val.clone()))), true)
            } else {
                (Rc::new(RefCell::new(Some(std::string::String::new()))), false)
            }
        } else {
            (Rc::new(RefCell::new(Some(std::string::String::new()))), false)
        }
    });;
        if ok {
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
                (Rc::new(RefCell::new(Some(typed_val.clone()))), true)
            } else {
                (Rc::new(RefCell::new(Some(0))), false)
            }
        } else {
            (Rc::new(RefCell::new(Some(0))), false)
        }
    });;
        if ok {
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
                (Rc::new(RefCell::new(Some(typed_val.clone()))), true)
            } else {
                (Rc::new(RefCell::new(Some(0.0))), false)
            }
        } else {
            (Rc::new(RefCell::new(Some(0.0))), false)
        }
    });;
        if ok {
            print!("Float value: {:.2} (squared: {:.2})\n", { let __v = (*f.borrow().as_ref().unwrap()).clone(); __v }, { let __bin_f = (*f.borrow().as_ref().unwrap()).clone(); __bin_f * __bin_f });;
            return;;
        }
    }

    print!("Unknown type: {} with value: {}\n", __go_type_name(&**value.borrow().as_ref().unwrap()), format_any(value.borrow().as_ref().unwrap().as_ref()));
}

pub fn assert_without_check(value: Rc<RefCell<Option<Box<dyn Any>>>>) {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        // This will panic if assertion fails
    __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        {
        let mut r = Rc::new(RefCell::new(None::<Box<dyn Any>>));;
        if { let __nil_result = (*r.borrow()).is_some(); __nil_result } {
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

pub fn describe_shape(s: Rc<RefCell<Option<Box<dyn Shape>>>>) {
    print!("Shape area: {:.2}\n", (*s.borrow().as_ref().unwrap()).area());

        // Type assertion on interface
    {
        let (mut rect, mut ok) = ({
        let val = s.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Shape>::__go_as_any(any_val.as_ref()).downcast_ref::<Rectangle>() {
            (Rc::new(RefCell::new(Some(typed_val.clone()))), true)
            } else {
                (Rc::new(RefCell::new(Some(Default::default()))), false)
            }
        } else {
            (Rc::new(RefCell::new(Some(Default::default()))), false)
        }
    });;
        if ok {
            print!("  Rectangle: {:.1} x {:.1}\n", (*(*rect.borrow().as_ref().unwrap()).width.borrow().as_ref().unwrap()), (*(*rect.borrow().as_ref().unwrap()).height.borrow().as_ref().unwrap()));;
        } else {
        let (mut circle, mut ok) = ({
        let val = s.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Shape>::__go_as_any(any_val.as_ref()).downcast_ref::<Circle>() {
            (Rc::new(RefCell::new(Some(typed_val.clone()))), true)
            } else {
                (Rc::new(RefCell::new(Some(Default::default()))), false)
            }
        } else {
            (Rc::new(RefCell::new(Some(Default::default()))), false)
        }
    });;
        if ok {
            print!("  Circle: radius {:.1}\n", (*(*circle.borrow().as_ref().unwrap()).radius.borrow().as_ref().unwrap()));;
        }
    }
    }
}

fn main() {
        // Test with different types
    let mut values = Rc::new(RefCell::new(Some(vec![Box::new("hello world".to_string()) as Box<dyn Any>, Box::new(42) as Box<dyn Any>, Box::new(3.14159) as Box<dyn Any>, Box::new(true) as Box<dyn Any>, Box::new(Rc::new(RefCell::new(Some(vec![1, 2, 3]))).clone()) as Box<dyn Any>])));

    println!("{}", format!("{}", "=== Processing values ===".to_string()));
    { let __range_holder = values.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.iter().map(|__e| go_any_clone(__e.as_ref())).collect::<Vec<_>>()).unwrap_or_default(); drop(__range_guard); for val in __range_values.iter() {
        process_value(val.clone());
    } }

    println!("{}", format!("{}", "\n=== Assertion without check ===".to_string()));
    assert_without_check(Rc::new(RefCell::new(Some(Box::new("valid string".to_string()) as Box<dyn Any>))));
    assert_without_check(Rc::new(RefCell::new(Some(Box::new(123) as Box<dyn Any>))));

    println!("{}", format!("{}", "\n=== Interface type assertions ===".to_string()));
    let mut shapes = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(Box::new(Rectangle { width: Rc::new(RefCell::new(Some(10.0))), height: Rc::new(RefCell::new(Some(5.0))), ..Default::default() }) as Box<dyn Shape>))), Rc::new(RefCell::new(Some(Box::new(Circle { radius: Rc::new(RefCell::new(Some(3.0))), ..Default::default() }) as Box<dyn Shape>)))])));

    { let __range_holder = shapes.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for shape in __range_values.iter() {
        describe_shape(shape.clone());
    } }

    println!("{}", format!("{}", "\n=== Type switch alternative ===".to_string()));
    { let __range_holder = values.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.iter().map(|__e| go_any_clone(__e.as_ref())).collect::<Vec<_>>()).unwrap_or_default(); drop(__range_guard); for val in __range_values.iter() {
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
        print!("Other: {} = {}\n", __go_type_name(v), format_any(v.borrow().as_ref().unwrap().as_ref()));;
    }
    }
    } }
}