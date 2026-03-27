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

trait Shape: std::fmt::Display {
    fn area(&self) -> Rc<RefCell<Option<f64>>>;
}

#[derive(Debug, Clone, Default)]
struct Rectangle {
    width: Rc<RefCell<Option<f64>>>,
    height: Rc<RefCell<Option<f64>>>,
}

impl std::fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.width.borrow().as_ref().unwrap()), (*self.height.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
struct Circle {
    radius: Rc<RefCell<Option<f64>>>,
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.radius.borrow().as_ref().unwrap()))
    }
}


impl Rectangle {
    pub fn area(&self) -> Rc<RefCell<Option<f64>>> {
        return Rc::new(RefCell::new(Some((*self.width.clone().borrow().as_ref().unwrap()) * (*self.height.clone().borrow().as_ref().unwrap()))));
    }
}

impl Shape for Rectangle {
    fn area(&self) -> Rc<RefCell<Option<f64>>> {
        return Rc::new(RefCell::new(Some((*self.width.clone().borrow().as_ref().unwrap()) * (*self.height.clone().borrow().as_ref().unwrap()))));
    }
}

impl Circle {
    pub fn area(&self) -> Rc<RefCell<Option<f64>>> {
        return Rc::new(RefCell::new(Some(3.14159 * (*self.radius.clone().borrow().as_ref().unwrap()) * (*self.radius.clone().borrow().as_ref().unwrap()))));
    }
}

impl Shape for Circle {
    fn area(&self) -> Rc<RefCell<Option<f64>>> {
        return Rc::new(RefCell::new(Some(3.14159 * (*self.radius.clone().borrow().as_ref().unwrap()) * (*self.radius.clone().borrow().as_ref().unwrap()))));
    }
}

pub fn process_value(value: Rc<RefCell<Option<Box<dyn Any>>>>) {
        // Basic type assertion
    let (mut str, mut ok) = ({
        let val = value.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<String>() {
                (Rc::new(RefCell::new(Some(typed_val.clone()))), Rc::new(RefCell::new(Some(true))))
            } else {
                (Rc::new(RefCell::new(Some(String::new()))), Rc::new(RefCell::new(Some(false))))
            }
        } else {
            (Rc::new(RefCell::new(Some(String::new()))), Rc::new(RefCell::new(Some(false))))
        }
    });
    if (*ok.borrow().as_ref().unwrap()) {
        print!("String value: {} (length: {})\n", (*str.borrow().as_ref().unwrap()), (*str.borrow().as_ref().unwrap()).len());
        return;
    }

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
    });
    if (*ok.borrow().as_ref().unwrap()) {
        print!("Integer value: {} (doubled: {})\n", (*num.borrow().as_ref().unwrap()), (*num.borrow().as_ref().unwrap()) * 2);
        return;
    }

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
    });
    if (*ok.borrow().as_ref().unwrap()) {
        print!("Float value: {:.2} (squared: {:.2})\n", (*f.borrow().as_ref().unwrap()), (*f.borrow().as_ref().unwrap()) * (*f.borrow().as_ref().unwrap()));
        return;
    }

    print!("Unknown type: {} with value: {}\n", go_type_name(&**value.borrow().as_ref().unwrap()), format_any(value.borrow().as_ref().unwrap().as_ref()));
}

pub fn assert_without_check(value: Rc<RefCell<Option<Box<dyn Any>>>>) {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        // This will panic if assertion fails
    __defer_stack.push(Box::new(move || {
        (*Rc::new(RefCell::new(Some(Box::new(move || {
        let mut r = Rc::new(RefCell::new(None::<String>));
    if (*r.borrow()).is_some() {
        print!("Panic recovered: {}\n", format_any(r.borrow().as_ref().unwrap().as_ref()));
    }
    }) as Box<dyn Fn() -> ()>))).borrow().as_ref().unwrap())();
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
    print!("Asserted string: {}\n", (*str.borrow().as_ref().unwrap()));

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}

pub fn describe_shape(s: &dyn Shape) {
    print!("Shape area: {:.2}\n", (*s.area().borrow().as_ref().unwrap()));

        // Type assertion on interface
    let (mut rect, mut ok) = ({
        let val = s.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rectangle>() {
                (Rc::new(RefCell::new(Some(typed_val.clone()))), Rc::new(RefCell::new(Some(true))))
            } else {
                (Rc::new(RefCell::new(Some(Default::default()))), Rc::new(RefCell::new(Some(false))))
            }
        } else {
            (Rc::new(RefCell::new(Some(Default::default()))), Rc::new(RefCell::new(Some(false))))
        }
    });
    if (*ok.borrow().as_ref().unwrap()) {
        print!("  Rectangle: {:.1} x {:.1}\n", (*(*rect.borrow().as_ref().unwrap()).width.borrow().as_ref().unwrap()), (*(*rect.borrow().as_ref().unwrap()).height.borrow().as_ref().unwrap()));
    } else {
        let (mut circle, mut ok) = ({
        let val = s.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Circle>() {
                (Rc::new(RefCell::new(Some(typed_val.clone()))), Rc::new(RefCell::new(Some(true))))
            } else {
                (Rc::new(RefCell::new(Some(Default::default()))), Rc::new(RefCell::new(Some(false))))
            }
        } else {
            (Rc::new(RefCell::new(Some(Default::default()))), Rc::new(RefCell::new(Some(false))))
        }
    });;
        if (*ok.borrow().as_ref().unwrap()) {
            print!("  Circle: radius {:.1}\n", (*(*circle.borrow().as_ref().unwrap()).radius.borrow().as_ref().unwrap()));;
        }
    }
}

fn main() {
        // Test with different types
    let mut values = Rc::new(RefCell::new(Some(vec![Box::new("hello world".to_string()) as Box<dyn Any>, Box::new(42) as Box<dyn Any>, Box::new(3.14159) as Box<dyn Any>, Box::new(true) as Box<dyn Any>, Box::new(Rc::new(RefCell::new(Some(vec![1, 2, 3])))) as Box<dyn Any>])));

    println!("{}", "=== Processing values ===".to_string());
    for val in &(*values.borrow().as_ref().unwrap()) {
        process_value(val.clone());
    }

    println!("{}", "\n=== Assertion without check ===".to_string());
    assert_without_check(Rc::new(RefCell::new(Some(Box::new("valid string".to_string()) as Box<dyn Any>))));
    assert_without_check(Rc::new(RefCell::new(Some(Box::new(123) as Box<dyn Any>))));

    println!("{}", "\n=== Interface type assertions ===".to_string());
    let mut shapes = Rc::new(RefCell::new(Some(vec![Box::new(Rectangle { width: Rc::new(RefCell::new(Some(10.0))), height: Rc::new(RefCell::new(Some(5.0))), ..Default::default() }) as Box<dyn Shape>, Box::new(Circle { radius: Rc::new(RefCell::new(Some(3.0))), ..Default::default() }) as Box<dyn Shape>])));

    for shape in &(*shapes.borrow().as_ref().unwrap()) {
        describe_shape(shape.as_ref());
    }

    println!("{}", "\n=== Type switch alternative ===".to_string());
    for val in &(*values.borrow().as_ref().unwrap()) {
        {
    let _ts_ref = val;
    let _any_val: &dyn Any = _ts_ref.as_ref();
    if _any_val.downcast_ref::<String>().is_some() {
        let v = Rc::new(RefCell::new(Some(_any_val.downcast_ref::<String>().unwrap().clone())));
        print!("String: {}\n", (*v.borrow().as_ref().unwrap()));;
    } else if _any_val.downcast_ref::<i32>().is_some() {
        let v = Rc::new(RefCell::new(Some(_any_val.downcast_ref::<i32>().unwrap().clone())));
        print!("Int: {}\n", (*v.borrow().as_ref().unwrap()));;
    } else if _any_val.downcast_ref::<f64>().is_some() {
        let v = Rc::new(RefCell::new(Some(_any_val.downcast_ref::<f64>().unwrap().clone())));
        print!("Float: {:.2}\n", (*v.borrow().as_ref().unwrap()));;
    } else if _any_val.downcast_ref::<bool>().is_some() {
        let v = Rc::new(RefCell::new(Some(_any_val.downcast_ref::<bool>().unwrap().clone())));
        print!("Bool: {}\n", (*v.borrow().as_ref().unwrap()));;
    } else {
        let v = _any_val;
        print!("Other: {} = {}\n", go_type_name(v), format_any(v.borrow().as_ref().unwrap().as_ref()));;
    }
    }
    }
}