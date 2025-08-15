use std::error::Error;
use std::sync::{Arc, Mutex};


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

pub fn safe_divide(a: Arc<Mutex<Option<f64>>>, b: Arc<Mutex<Option<f64>>>) -> (Arc<Mutex<Option<f64>>>, Arc<Mutex<Option<Box<dyn Error + Send + Sync>>>>) {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut result: Arc<Mutex<Option<f64>>> = Arc::new(Mutex::new(Some(0.0)));
    let mut err: Arc<Mutex<Option<Box<dyn Error + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let err_defer_captured = err.clone(); let result_defer_captured = result.clone(); __defer_stack.push(Box::new(move || {
        (Arc::new(Mutex::new(Some(Box::new(move || {
        let mut r = Arc::new(Mutex::new(None::<String>));
    if (*r.lock().unwrap()).is_some() {
        { let new_val = Arc::new(Mutex::new(Some(Box::new(format!("panic occurred: {}", (*r.lock().unwrap().as_mut().unwrap()))) as Box<dyn Error + Send + Sync>))); *err.lock().unwrap() = Some(new_val); };
        { let new_val = 0; *result.lock().unwrap() = Some(new_val); };
    }
    }) as Box<dyn Fn() -> () + Send + Sync>))).lock().unwrap().as_ref().unwrap())();
    }));

    if (*b.lock().unwrap().as_mut().unwrap()) == 0.0 {
        panic!("division by zero");
    }

    { let new_val = (*a.lock().unwrap().as_mut().unwrap()) / (*b.lock().unwrap().as_mut().unwrap()); *result.lock().unwrap() = Some(new_val); };
    {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (result.clone(), Arc::new(Mutex::new(None)))
    }

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}

pub fn process_slice(slice: Arc<Mutex<Option<Vec<i32>>>>, index: Arc<Mutex<Option<i32>>>) -> (Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<Box<dyn Error + Send + Sync>>>>) {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut value: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn Error + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let err_defer_captured = err.clone(); let value_defer_captured = value.clone(); __defer_stack.push(Box::new(move || {
        (Arc::new(Mutex::new(Some(Box::new(move || {
        let mut r = Arc::new(Mutex::new(None::<String>));
    if (*r.lock().unwrap()).is_some() {
        { let new_val = Arc::new(Mutex::new(Some(Box::new(format!("index out of bounds: {}", (*r.lock().unwrap().as_mut().unwrap()))) as Box<dyn Error + Send + Sync>))); *err.lock().unwrap() = Some(new_val); };
        { let new_val = -1; *value.lock().unwrap() = Some(new_val); };
    }
    }) as Box<dyn Fn() -> () + Send + Sync>))).lock().unwrap().as_ref().unwrap())();
    }));

    { let new_val = (*slice.lock().unwrap().as_ref().unwrap())[(*index.lock().unwrap().as_mut().unwrap()) as usize].clone(); *value.lock().unwrap() = Some(new_val); };
    {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (value.clone(), Arc::new(Mutex::new(None)))
    }

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}

pub fn nested_panic() {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    __defer_stack.push(Box::new(move || {
        (Arc::new(Mutex::new(Some(Box::new(move || {
        let mut r = Arc::new(Mutex::new(None::<String>));
    if (*r.lock().unwrap()).is_some() {
        print!("Recovered from nested panic: {}\n", format_any(r.lock().unwrap().as_ref().unwrap().as_ref()));
    }
    }) as Box<dyn Fn() -> () + Send + Sync>))).lock().unwrap().as_ref().unwrap())();
    }));

    (Arc::new(Mutex::new(Some(Box::new(move || {
        __defer_stack.push(Box::new(move || {
        (Arc::new(Mutex::new(Some(Box::new(move || {
        let mut r = Arc::new(Mutex::new(None::<String>));
    if (*r.lock().unwrap()).is_some() {
        print!("Inner recovery: {}\n", format_any(r.lock().unwrap().as_ref().unwrap().as_ref()));
        panic!("re-panicking from inner function");
    }
    }) as Box<dyn Fn() -> () + Send + Sync>))).lock().unwrap().as_ref().unwrap())();
    }));
        panic!("original panic");
    }) as Box<dyn Fn() -> () + Send + Sync>))).lock().unwrap().as_ref().unwrap())();

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}

pub fn demonstrate_panic_types() {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        // String panic
    __defer_stack.push(Box::new(move || {
        (Arc::new(Mutex::new(Some(Box::new(move || {
        let mut r = Arc::new(Mutex::new(None::<String>));
    if (*r.lock().unwrap()).is_some() {
        print!("Recovered string panic: {}\n", format_any(r.lock().unwrap().as_ref().unwrap().as_ref()));
    }
    }) as Box<dyn Fn() -> () + Send + Sync>))).lock().unwrap().as_ref().unwrap())();
    }));

    __defer_stack.push(Box::new(move || {
        (Arc::new(Mutex::new(Some(Box::new(move || {
        panic!("string panic message");
    }) as Box<dyn Fn() -> () + Send + Sync>))).lock().unwrap().as_ref().unwrap())();
    }));

    __defer_stack.push(Box::new(move || {
        (Arc::new(Mutex::new(Some(Box::new(move || {
        panic!("{}", 42);
    }) as Box<dyn Fn() -> () + Send + Sync>))).lock().unwrap().as_ref().unwrap())();
    }));

        // Integer panic
    __defer_stack.push(Box::new(move || {
        (Arc::new(Mutex::new(Some(Box::new(move || {
        panic!("{}", Arc::new(Mutex::new(Some(Box::new(format!("error panic")) as Box<dyn Error + Send + Sync>))));
    }) as Box<dyn Fn() -> () + Send + Sync>))).lock().unwrap().as_ref().unwrap())();
    }));

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}

pub fn chained_defers() {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    __defer_stack.push(Box::new(move || {
        (Arc::new(Mutex::new(Some(Box::new(move || {
        let mut r = Arc::new(Mutex::new(None::<String>));
    if (*r.lock().unwrap()).is_some() {
        print!("Final recovery: {}\n", format_any(r.lock().unwrap().as_ref().unwrap().as_ref()));
    }
    }) as Box<dyn Fn() -> () + Send + Sync>))).lock().unwrap().as_ref().unwrap())();
    }));

    __defer_stack.push(Box::new(move || {
        (Arc::new(Mutex::new(Some(Box::new(move || {
        println!("{}", "Defer 1: This runs".to_string());
    }) as Box<dyn Fn() -> () + Send + Sync>))).lock().unwrap().as_ref().unwrap())();
    }));

    __defer_stack.push(Box::new(move || {
        (Arc::new(Mutex::new(Some(Box::new(move || {
        println!("{}", "Defer 2: This also runs".to_string());
        panic!("panic from defer");
    }) as Box<dyn Fn() -> () + Send + Sync>))).lock().unwrap().as_ref().unwrap())();
    }));

    __defer_stack.push(Box::new(move || {
        (Arc::new(Mutex::new(Some(Box::new(move || {
        println!("{}", "Defer 3: This runs first".to_string());
    }) as Box<dyn Fn() -> () + Send + Sync>))).lock().unwrap().as_ref().unwrap())();
    }));

    println!("{}", "About to return normally".to_string());

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}

fn main() {
    println!("{}", "=== Safe divide examples ===".to_string());

    let (mut result, mut err) = safe_divide(Arc::new(Mutex::new(Some(10))), Arc::new(Mutex::new(Some(2))));
    if (*err.lock().unwrap()).is_some() {
        print!("Error: {}\n", (*err.lock().unwrap().as_mut().unwrap()));
    } else {
        print!("10 / 2 = {:.2}\n", (*result.lock().unwrap().as_mut().unwrap()));
    }

    (result, err) = safe_divide(Arc::new(Mutex::new(Some(10))), Arc::new(Mutex::new(Some(0))));
    if (*err.lock().unwrap()).is_some() {
        print!("Error: {}\n", (*err.lock().unwrap().as_mut().unwrap()));
    } else {
        print!("Result: {:.2}\n", (*result.lock().unwrap().as_mut().unwrap()));
    }

    println!("{}", "\n=== Slice access examples ===".to_string());

    let mut numbers = Arc::new(Mutex::new(Some(vec![1, 2, 3, 4, 5])));

    let (mut value, mut err) = process_slice(numbers.clone(), Arc::new(Mutex::new(Some(2))));
    if (*err.lock().unwrap()).is_some() {
        print!("Error: {}\n", (*err.lock().unwrap().as_mut().unwrap()));
    } else {
        print!("numbers[2] = {}\n", (*value.lock().unwrap().as_mut().unwrap()));
    }

    (value, err) = process_slice(numbers.clone(), Arc::new(Mutex::new(Some(10))));
    if (*err.lock().unwrap()).is_some() {
        print!("Error: {}\n", (*err.lock().unwrap().as_mut().unwrap()));
    } else {
        print!("Value: {}\n", (*value.lock().unwrap().as_mut().unwrap()));
    }

    println!("{}", "\n=== Nested panic example ===".to_string());
    nested_panic();

    println!("{}", "\n=== Different panic types ===".to_string());
    demonstrate_panic_types();

    println!("{}", "\n=== Chained defers with panic ===".to_string());
    chained_defers();

    println!("{}", "\n=== Program completed successfully ===".to_string());
}