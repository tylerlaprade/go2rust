use std::cell::{RefCell};
use std::error::Error;
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

pub fn safe_divide(a: Rc<RefCell<Option<f64>>>, b: Rc<RefCell<Option<f64>>>) -> (Rc<RefCell<Option<f64>>>, Rc<RefCell<Option<Box<dyn Error + Send + Sync>>>>) {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut result: Rc<RefCell<Option<f64>>> = Rc::new(RefCell::new(Some(Some(0.0)));
    let mut err: Rc<RefCell<Option<Box<dyn Error + Send + Sync>>>> = Rc::new(RefCell::new(Some(None));

    let err_defer_captured = err.clone(); let result_defer_captured = result.clone(); __defer_stack.push(Box::new(move || {
        (Rc::new(RefCell::new(Some(Box::new(move || {
        let mut r = Rc::new(RefCell::new(None))::<String>));
    if (*r.borrow()).is_some() {
        { let new_val = Rc::new(RefCell::new(Some(Some(Box::new(format!("panic occurred: {}", (*r.borrow_mut().as_mut().unwrap()))) as Box<dyn Error + Send + Sync>))); *err_defer_captured.borrow_mut() = Some(new_val); };
        { let new_val = 0; *result_defer_captured.borrow_mut() = Some(new_val); };
    }
    }) as Box<dyn Fn() -> ()>))).borrow().as_ref().unwrap())();
    }));

    if (*b.borrow_mut().as_mut().unwrap()) == 0.0 {
        panic!("division by zero");
    }

    { let new_val = (*a.borrow_mut().as_mut().unwrap()) / (*b.borrow_mut().as_mut().unwrap()); *result.borrow_mut() = Some(new_val); };
    {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (result.clone(), Rc::new(RefCell::new(None)))
    }

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}

pub fn process_slice(slice: Rc<RefCell<Option<Vec<i32>>>>, index: Rc<RefCell<Option<i32>>>) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<Box<dyn Error + Send + Sync>>>>) {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut value: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(Some(0)));
    let mut err: Rc<RefCell<Option<Box<dyn Error + Send + Sync>>>> = Rc::new(RefCell::new(Some(None));

    let err_defer_captured = err.clone(); let value_defer_captured = value.clone(); __defer_stack.push(Box::new(move || {
        (Rc::new(RefCell::new(Some(Box::new(move || {
        let mut r = Rc::new(RefCell::new(None))::<String>));
    if (*r.borrow()).is_some() {
        { let new_val = Rc::new(RefCell::new(Some(Some(Box::new(format!("index out of bounds: {}", (*r.borrow_mut().as_mut().unwrap()))) as Box<dyn Error + Send + Sync>))); *err_defer_captured.borrow_mut() = Some(new_val); };
        { let new_val = -1; *value_defer_captured.borrow_mut() = Some(new_val); };
    }
    }) as Box<dyn Fn() -> ()>))).borrow().as_ref().unwrap())();
    }));

    { let new_val = (*slice.borrow().as_ref().unwrap())[(*index.borrow_mut().as_mut().unwrap()) as usize].clone(); *value.borrow_mut() = Some(new_val); };
    {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (value.clone(), Rc::new(RefCell::new(None)))
    }

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}

pub fn nested_panic() {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    __defer_stack.push(Box::new(move || {
        (Rc::new(RefCell::new(Some(Box::new(move || {
        let mut r = Rc::new(RefCell::new(None))::<String>));
    if (*r.borrow()).is_some() {
        print!("Recovered from nested panic: {}\n", format_any(r.borrow().as_ref().unwrap().as_ref()));
    }
    }) as Box<dyn Fn() -> ()>))).borrow().as_ref().unwrap())();
    }));

    (Rc::new(RefCell::new(Some(Box::new(move || {
        __defer_stack.push(Box::new(move || {
        (Rc::new(RefCell::new(Some(Box::new(move || {
        let mut r = Rc::new(RefCell::new(None))::<String>));
    if (*r.borrow()).is_some() {
        print!("Inner recovery: {}\n", format_any(r.borrow().as_ref().unwrap().as_ref()));
        panic!("re-panicking from inner function");
    }
    }) as Box<dyn Fn() -> ()>))).borrow().as_ref().unwrap())();
    }));
        panic!("original panic");
    }) as Box<dyn Fn() -> ()>))).borrow().as_ref().unwrap())();

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}

pub fn demonstrate_panic_types() {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        // String panic
    __defer_stack.push(Box::new(move || {
        (Rc::new(RefCell::new(Some(Box::new(move || {
        let mut r = Rc::new(RefCell::new(None))::<String>));
    if (*r.borrow()).is_some() {
        print!("Recovered string panic: {}\n", format_any(r.borrow().as_ref().unwrap().as_ref()));
    }
    }) as Box<dyn Fn() -> ()>))).borrow().as_ref().unwrap())();
    }));

    __defer_stack.push(Box::new(move || {
        (Rc::new(RefCell::new(Some(Box::new(move || {
        panic!("string panic message");
    }) as Box<dyn Fn() -> ()>))).borrow().as_ref().unwrap())();
    }));

    __defer_stack.push(Box::new(move || {
        (Rc::new(RefCell::new(Some(Box::new(move || {
        panic!("{}", 42);
    }) as Box<dyn Fn() -> ()>))).borrow().as_ref().unwrap())();
    }));

        // Integer panic
    __defer_stack.push(Box::new(move || {
        (Rc::new(RefCell::new(Some(Box::new(move || {
        panic!("{}", Rc::new(RefCell::new(Some(Some(Box::new(format!("error panic")) as Box<dyn Error + Send + Sync>))));
    }) as Box<dyn Fn() -> ()>))).borrow().as_ref().unwrap())();
    }));

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}

pub fn chained_defers() {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    __defer_stack.push(Box::new(move || {
        (Rc::new(RefCell::new(Some(Box::new(move || {
        let mut r = Rc::new(RefCell::new(None))::<String>));
    if (*r.borrow()).is_some() {
        print!("Final recovery: {}\n", format_any(r.borrow().as_ref().unwrap().as_ref()));
    }
    }) as Box<dyn Fn() -> ()>))).borrow().as_ref().unwrap())();
    }));

    __defer_stack.push(Box::new(move || {
        (Rc::new(RefCell::new(Some(Box::new(move || {
        println!("{}", "Defer 1: This runs".to_string());
    }) as Box<dyn Fn() -> ()>))).borrow().as_ref().unwrap())();
    }));

    __defer_stack.push(Box::new(move || {
        (Rc::new(RefCell::new(Some(Box::new(move || {
        println!("{}", "Defer 2: This also runs".to_string());
        panic!("panic from defer");
    }) as Box<dyn Fn() -> ()>))).borrow().as_ref().unwrap())();
    }));

    __defer_stack.push(Box::new(move || {
        (Rc::new(RefCell::new(Some(Box::new(move || {
        println!("{}", "Defer 3: This runs first".to_string());
    }) as Box<dyn Fn() -> ()>))).borrow().as_ref().unwrap())();
    }));

    println!("{}", "About to return normally".to_string());

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}

fn main() {
    println!("{}", "=== Safe divide examples ===".to_string());

    let (mut result, mut err) = safe_divide(Rc::new(RefCell::new(Some(10))), Rc::new(RefCell::new(Some(2))));
    if (*err.borrow()).is_some() {
        print!("Error: {}\n", (*err.borrow_mut().as_mut().unwrap()));
    } else {
        print!("10 / 2 = {:.2}\n", (*result.borrow_mut().as_mut().unwrap()));
    }

    (result, err) = safe_divide(Rc::new(RefCell::new(Some(10))), Rc::new(RefCell::new(Some(0))));
    if (*err.borrow()).is_some() {
        print!("Error: {}\n", (*err.borrow_mut().as_mut().unwrap()));
    } else {
        print!("Result: {:.2}\n", (*result.borrow_mut().as_mut().unwrap()));
    }

    println!("{}", "\n=== Slice access examples ===".to_string());

    let mut numbers = Rc::new(RefCell::new(Some(vec![1, 2, 3, 4, 5])));

    let (mut value, mut err) = process_slice(numbers.clone(), Rc::new(RefCell::new(Some(2))));
    if (*err.borrow()).is_some() {
        print!("Error: {}\n", (*err.borrow_mut().as_mut().unwrap()));
    } else {
        print!("numbers[2] = {}\n", (*value.borrow_mut().as_mut().unwrap()));
    }

    (value, err) = process_slice(numbers.clone(), Rc::new(RefCell::new(Some(10))));
    if (*err.borrow()).is_some() {
        print!("Error: {}\n", (*err.borrow_mut().as_mut().unwrap()));
    } else {
        print!("Value: {}\n", (*value.borrow_mut().as_mut().unwrap()));
    }

    println!("{}", "\n=== Nested panic example ===".to_string());
    nested_panic();

    println!("{}", "\n=== Different panic types ===".to_string());
    demonstrate_panic_types();

    println!("{}", "\n=== Chained defers with panic ===".to_string());
    chained_defers();

    println!("{}", "\n=== Program completed successfully ===".to_string());
}