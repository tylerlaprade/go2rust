use std::any::Any;
use std::cell::{RefCell};
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

trait Shape {
    fn area(&self) -> Rc<RefCell<Option<f64>>>;
}

#[derive(Debug, Clone, Default)]
struct Rectangle {
    width: Rc<RefCell<Option<f64>>>,
    height: Rc<RefCell<Option<f64>>>,
}

#[derive(Debug, Clone, Default)]
struct Circle {
    radius: Rc<RefCell<Option<f64>>>,
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
    if (*ok.borrow_mut().as_mut().unwrap()) {
        print!("String value: {} (length: {})\n", (*str.borrow_mut().as_mut().unwrap()), (*str.borrow().as_ref().unwrap()).len());
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
    if (*ok.borrow_mut().as_mut().unwrap()) {
        print!("Integer value: {} (doubled: {})\n", (*num.borrow_mut().as_mut().unwrap()), (*num.borrow_mut().as_mut().unwrap()) * 2);
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
    if (*ok.borrow_mut().as_mut().unwrap()) {
        print!("Float value: {:.2} (squared: {:.2})\n", (*f.borrow_mut().as_mut().unwrap()), (*f.borrow_mut().as_mut().unwrap()) * (*f.borrow_mut().as_mut().unwrap()));
        return;
    }

    print!("Unknown type: %T with value: {}\n", format_any(value.borrow().as_ref().unwrap().as_ref()), format_any(value.borrow().as_ref().unwrap().as_ref()));
}

pub fn assert_without_check(value: Rc<RefCell<Option<Box<dyn Any>>>>) {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        // This will panic if assertion fails
    __defer_stack.push(Box::new(move || {
        (Rc::new(RefCell::new(Some(Box::new(move || {
        let mut r = Arc::new(Mutex::new(None::<String>));
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
    print!("Asserted string: {}\n", (*str.borrow_mut().as_mut().unwrap()));

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}

pub fn describe_shape(s: Rc<RefCell<Option<Box<dyn Shape>>>>) {
    print!("Shape area: {:.2}\n", (*(*s.borrow_mut().as_mut().unwrap()).area().borrow().as_ref().unwrap()));

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
    if (*ok.borrow_mut().as_mut().unwrap()) {
        print!("  Rectangle: {:.1} x {:.1}\n", (*(*rect.borrow().as_ref().unwrap()).width.borrow().as_ref().unwrap()), (*(*rect.borrow().as_ref().unwrap()).height.borrow().as_ref().unwrap()));
    } else let (mut circle, mut ok) = ({
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
    });
    if (*ok.borrow_mut().as_mut().unwrap()) {
        print!("  Circle: radius {:.1}\n", (*(*circle.borrow().as_ref().unwrap()).radius.borrow().as_ref().unwrap()));
    }
}

fn main() {
        // Test with different types
    let mut values = Rc::new(RefCell::new(Some(vec![Box::new("hello world".to_string()) as Box<dyn Any>, Box::new(42) as Box<dyn Any>, Box::new(3.14159) as Box<dyn Any>, Box::new(true) as Box<dyn Any>, Box::new(Rc::new(RefCell::new(Some(vec![1, 2, 3])))) as Box<dyn Any>])));

    println!("{}", "=== Processing values ===".to_string());
    for val in &(*values.borrow_mut().as_mut().unwrap()) {
        process_value(Rc::new(RefCell::new(Some(*val))));
    }

    println!("{}", "\n=== Assertion without check ===".to_string());
    assert_without_check(Rc::new(RefCell::new(Some("valid string".to_string()))));
    assert_without_check(Rc::new(RefCell::new(Some(123))));

    println!("{}", "\n=== Interface type assertions ===".to_string());
    let mut shapes = Rc::new(RefCell::new(Some(vec![Rectangle { width: Rc::new(RefCell::new(Some(10))), height: Rc::new(RefCell::new(Some(5))) }, Circle { radius: Rc::new(RefCell::new(Some(3))) }])));

    for shape in &(*shapes.borrow_mut().as_mut().unwrap()) {
        describe_shape(Rc::new(RefCell::new(Some(*shape))));
    }

    println!("{}", "\n=== Type switch alternative ===".to_string());
    for val in &(*values.borrow_mut().as_mut().unwrap()) {
        // TODO: Unhandled statement type: TypeSwitchStmt
    }
}