use std::any::Any;
use std::sync::{Arc, Mutex};

trait Shape {
    fn area(&self) -> Arc<Mutex<Option<f64>>>;
}

#[derive(Debug)]
struct Rectangle {
    width: Arc<Mutex<Option<f64>>>,
    height: Arc<Mutex<Option<f64>>>,
}

#[derive(Debug)]
struct Circle {
    radius: Arc<Mutex<Option<f64>>>,
}

impl Rectangle {
    pub fn area(&self) -> Arc<Mutex<Option<f64>>> {
        return Arc::new(Mutex::new(Some((*self.width.clone().lock().unwrap().as_mut().unwrap()) * (*self.height.clone().lock().unwrap().as_mut().unwrap()))));
    }
}

impl Shape for Rectangle {
    fn area(&self) -> Arc<Mutex<Option<f64>>> {
        return Arc::new(Mutex::new(Some((*self.width.clone().lock().unwrap().as_mut().unwrap()) * (*self.height.clone().lock().unwrap().as_mut().unwrap()))));
    }
}

impl Circle {
    pub fn area(&self) -> Arc<Mutex<Option<f64>>> {
        return Arc::new(Mutex::new(Some((*(*3.14159.lock().unwrap().as_mut().unwrap()) * (*self.radius.clone().lock().unwrap().as_mut().unwrap()).lock().unwrap().as_mut().unwrap()) * (*self.radius.clone().lock().unwrap().as_mut().unwrap()))));
    }
}

impl Shape for Circle {
    fn area(&self) -> Arc<Mutex<Option<f64>>> {
        return Arc::new(Mutex::new(Some((*(*3.14159.lock().unwrap().as_mut().unwrap()) * (*self.radius.clone().lock().unwrap().as_mut().unwrap()).lock().unwrap().as_mut().unwrap()) * (*self.radius.clone().lock().unwrap().as_mut().unwrap()))));
    }
}

pub fn process_value(value: Arc<Mutex<Option<Box<dyn Any>>>>) {
    let (mut str, mut ok) = ({
        let val = value.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<String>() {
                (Arc::new(Mutex::new(Some(typed_val.clone()))), Arc::new(Mutex::new(Some(true))))
            } else {
                (Arc::new(Mutex::new(Some(String::new()))), Arc::new(Mutex::new(Some(false))))
            }
        } else {
            (Arc::new(Mutex::new(Some(String::new()))), Arc::new(Mutex::new(Some(false))))
        }
    });
    if (*ok.lock().unwrap().as_mut().unwrap()) {
        print!("String value: {} (length: {})\n", (*str.lock().unwrap().as_mut().unwrap()), (*str.lock().unwrap().as_mut().unwrap()).len());
        return;
    }

    let (mut num, mut ok) = ({
        let val = value.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<i32>() {
                (Arc::new(Mutex::new(Some(typed_val.clone()))), Arc::new(Mutex::new(Some(true))))
            } else {
                (Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(false))))
            }
        } else {
            (Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(false))))
        }
    });
    if (*ok.lock().unwrap().as_mut().unwrap()) {
        print!("Integer value: {} (doubled: {})\n", (*num.lock().unwrap().as_mut().unwrap()), (*num.lock().unwrap().as_mut().unwrap()) * 2);
        return;
    }

    let (mut f, mut ok) = ({
        let val = value.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<f64>() {
                (Arc::new(Mutex::new(Some(typed_val.clone()))), Arc::new(Mutex::new(Some(true))))
            } else {
                (Arc::new(Mutex::new(Some(0.0))), Arc::new(Mutex::new(Some(false))))
            }
        } else {
            (Arc::new(Mutex::new(Some(0.0))), Arc::new(Mutex::new(Some(false))))
        }
    });
    if (*ok.lock().unwrap().as_mut().unwrap()) {
        print!("Float value: {:.2} (squared: {:.2})\n", (*f.lock().unwrap().as_mut().unwrap()), (*f.lock().unwrap().as_mut().unwrap()) * (*f.lock().unwrap().as_mut().unwrap()));
        return;
    }

    print!("Unknown type: %T with value: {}\n", (*value.lock().unwrap().as_mut().unwrap()), (*value.lock().unwrap().as_mut().unwrap()));
}

pub fn assert_without_check(value: Arc<Mutex<Option<Box<dyn Any>>>>) {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    __defer_stack.push(Box::new(move || {
        (Arc::new(Mutex::new(Some(Box::new(move || {
        let mut r = recover();
    if (*r.lock().unwrap()).is_some() {
        print!("Panic recovered: {}\n", (*r.lock().unwrap().as_mut().unwrap()));
    }
    }) as Box<dyn Fn() -> () + Send + Sync>))).lock().unwrap().as_ref().unwrap())();
    }));

    let mut str = Arc::new(Mutex::new(Some(({
        let val = value.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<String>().expect("type assertion failed").clone()
        } else {
            panic!("type assertion on nil interface")
        }
    }))));
    print!("Asserted string: {}\n", (*str.lock().unwrap().as_mut().unwrap()));

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}

pub fn describe_shape(s: Arc<Mutex<Option<Box<dyn Shape>>>>) {
    print!("Shape area: {:.2}\n", (*(*s.lock().unwrap().as_mut().unwrap()).area().lock().unwrap().as_ref().unwrap()));

    let (mut rect, mut ok) = ({
        let val = s.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rectangle>() {
                (Arc::new(Mutex::new(Some(typed_val.clone()))), Arc::new(Mutex::new(Some(true))))
            } else {
                (Arc::new(Mutex::new(Some(Default::default()))), Arc::new(Mutex::new(Some(false))))
            }
        } else {
            (Arc::new(Mutex::new(Some(Default::default()))), Arc::new(Mutex::new(Some(false))))
        }
    });
    if (*ok.lock().unwrap().as_mut().unwrap()) {
        print!("  Rectangle: {:.1} x {:.1}\n", (*(*rect.lock().unwrap().as_mut().unwrap()).width.lock().unwrap().as_ref().unwrap()), (*(*rect.lock().unwrap().as_mut().unwrap()).height.lock().unwrap().as_ref().unwrap()));
    } else let (mut circle, mut ok) = ({
        let val = s.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Circle>() {
                (Arc::new(Mutex::new(Some(typed_val.clone()))), Arc::new(Mutex::new(Some(true))))
            } else {
                (Arc::new(Mutex::new(Some(Default::default()))), Arc::new(Mutex::new(Some(false))))
            }
        } else {
            (Arc::new(Mutex::new(Some(Default::default()))), Arc::new(Mutex::new(Some(false))))
        }
    });
    if (*ok.lock().unwrap().as_mut().unwrap()) {
        print!("  Circle: radius {:.1}\n", (*(*circle.lock().unwrap().as_mut().unwrap()).radius.lock().unwrap().as_ref().unwrap()));
    }
}

fn main() {
    let mut values = Arc::new(Mutex::new(Some(vec!["hello world".to_string(), 42, 3.14159, true, Arc::new(Mutex::new(Some(vec![1, 2, 3])))])));

    println!("{}", "=== Processing values ===".to_string());
    for val in &(*values.lock().unwrap().as_mut().unwrap()) {
        process_value(Arc::new(Mutex::new(Some(val))));
    }

    println!("{}", "\n=== Assertion without check ===".to_string());
    assert_without_check(Arc::new(Mutex::new(Some("valid string".to_string()))));
    assert_without_check(Arc::new(Mutex::new(Some(123))));

    println!("{}", "\n=== Interface type assertions ===".to_string());
    let mut shapes = Arc::new(Mutex::new(Some(vec![Rectangle { width: Arc::new(Mutex::new(Some(10))), height: Arc::new(Mutex::new(Some(5))) }, Circle { radius: Arc::new(Mutex::new(Some(3))) }])));

    for shape in &(*shapes.lock().unwrap().as_mut().unwrap()) {
        describe_shape(Arc::new(Mutex::new(Some(shape))));
    }

    println!("{}", "\n=== Type switch alternative ===".to_string());
    for val in &(*values.lock().unwrap().as_mut().unwrap()) {
        // TODO: Unhandled statement type: TypeSwitchStmt
    }
}