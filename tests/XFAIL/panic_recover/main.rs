use std::any::Any;
use std::cell::{RefCell};
use std::error::Error as StdError;
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

pub fn safe_divide(a: Rc<RefCell<Option<f64>>>, b: Rc<RefCell<Option<f64>>>) -> (f64, Rc<RefCell<Option<Box<dyn StdError>>>>) {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut result: Rc<RefCell<Option<f64>>> = Rc::new(RefCell::new(Some(0.0)));
    let mut err: Rc<RefCell<Option<Box<dyn StdError>>>> = Rc::new(RefCell::new(None));

    let err_defer_captured = err.clone(); let result_defer_captured = result.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        {
        let mut r = Rc::new(RefCell::new(None::<Box<dyn Any>>));;
        if (*r.borrow()).is_some() {
            { let __rhs_holder = Rc::new(RefCell::new(Some(Box::<dyn StdError>::from(format!("panic occurred: {}", format_any(r.borrow().as_ref().unwrap().as_ref())))))).clone(); let new_val = { let mut guard = __rhs_holder.borrow_mut(); guard.take() }; *err_defer_captured.borrow_mut() = new_val; };;
            { let new_val = 0.0; *result_defer_captured.borrow_mut() = Some(new_val); };;
        }
    }
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));

    if (*b.borrow().as_ref().unwrap()) == 0.0 {
        panic!("division by zero");
    }

    { let new_val = (*a.borrow().as_ref().unwrap()) / (*b.borrow().as_ref().unwrap()); *result.borrow_mut() = Some(new_val); };
    {
        *err.borrow_mut() = None;;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return ((*result.borrow().as_ref().unwrap()), err);
    }
}

pub fn process_slice(slice: Rc<RefCell<Option<Vec<i32>>>>, index: Rc<RefCell<Option<i32>>>) -> (i32, Rc<RefCell<Option<Box<dyn StdError>>>>) {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut value: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(0)));
    let mut err: Rc<RefCell<Option<Box<dyn StdError>>>> = Rc::new(RefCell::new(None));

    let err_defer_captured = err.clone(); let value_defer_captured = value.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        {
        let mut r = Rc::new(RefCell::new(None::<Box<dyn Any>>));;
        if (*r.borrow()).is_some() {
            { let __rhs_holder = Rc::new(RefCell::new(Some(Box::<dyn StdError>::from(format!("index out of bounds: {}", format_any(r.borrow().as_ref().unwrap().as_ref())))))).clone(); let new_val = { let mut guard = __rhs_holder.borrow_mut(); guard.take() }; *err_defer_captured.borrow_mut() = new_val; };;
            { let new_val = -1; *value_defer_captured.borrow_mut() = Some(new_val); };;
        }
    }
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));

    { let new_val = (*slice.borrow().as_ref().unwrap())[((*index.borrow().as_ref().unwrap())) as usize].clone(); *value.borrow_mut() = Some(new_val); };
    {
        *err.borrow_mut() = None;;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return ((*value.borrow().as_ref().unwrap()), err);
    }
}

pub fn nested_panic() {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        {
        let mut r = Rc::new(RefCell::new(None::<Box<dyn Any>>));;
        if (*r.borrow()).is_some() {
            print!("Recovered from nested panic: {}\n", format_any(r.borrow().as_ref().unwrap().as_ref()));;
        }
    }
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));

    { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();
        __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        {
        let mut r = Rc::new(RefCell::new(None::<Box<dyn Any>>));;
        if (*r.borrow()).is_some() {
            print!("Inner recovery: {}\n", format_any(r.borrow().as_ref().unwrap().as_ref()));;
            panic!("re-panicking from inner function");;
        }
    }
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
        panic!("original panic");
        while let Some(f) = __defer_stack.pop() {
            f();
        }
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}

pub fn demonstrate_panic_types() {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        // String panic
    __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        {
        let mut r = Rc::new(RefCell::new(None::<Box<dyn Any>>));;
        if (*r.borrow()).is_some() {
            print!("Recovered string panic: {}\n", format_any(r.borrow().as_ref().unwrap().as_ref()));;
        }
    }
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));

    __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        panic!("string panic message");
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));

    __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        panic!("{}", 42);
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));

        // Integer panic
    __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        panic!("error panic");
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}

pub fn chained_defers() {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        {
        let mut r = Rc::new(RefCell::new(None::<Box<dyn Any>>));;
        if (*r.borrow()).is_some() {
            print!("Final recovery: {}\n", format_any(r.borrow().as_ref().unwrap().as_ref()));;
        }
    }
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));

    __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        println!("{}", format!("{}", "Defer 1: This runs".to_string()));
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));

    __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        println!("{}", format!("{}", "Defer 2: This also runs".to_string()));
        panic!("panic from defer");
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));

    __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        println!("{}", format!("{}", "Defer 3: This runs first".to_string()));
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));

    println!("{}", format!("{}", "About to return normally".to_string()));

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}

fn main() {
    println!("{}", format!("{}", "=== Safe divide examples ===".to_string()));

    let (mut result, mut err) = safe_divide(Rc::new(RefCell::new(Some(10.0))), Rc::new(RefCell::new(Some(2.0))));
    if (*err.borrow()).is_some() {
        print!("Error: {}\n", format!("{}", (*err.borrow().as_ref().unwrap())));
    } else {
        print!("10 / 2 = {:.2}\n", result);
    }

    { let (__tmp_0, __tmp_1) = safe_divide(Rc::new(RefCell::new(Some(10.0))), Rc::new(RefCell::new(Some(0.0)))); result = __tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.borrow_mut(); __guard.take() }; *err.borrow_mut() = __moved_tmp_1; };
    if (*err.borrow()).is_some() {
        print!("Error: {}\n", format!("{}", (*err.borrow().as_ref().unwrap())));
    } else {
        print!("Result: {:.2}\n", result);
    }

    println!("{}", format!("{}", "\n=== Slice access examples ===".to_string()));

    let mut numbers = Rc::new(RefCell::new(Some(vec![1, 2, 3, 4, 5])));

    let (mut value, __tmp_1) = process_slice(numbers.clone(), Rc::new(RefCell::new(Some(2)))); let __moved_tmp_1 = { let mut __guard = __tmp_1.borrow_mut(); __guard.take() }; *err.borrow_mut() = __moved_tmp_1;;
    if (*err.borrow()).is_some() {
        print!("Error: {}\n", format!("{}", (*err.borrow().as_ref().unwrap())));
    } else {
        print!("numbers[2] = {}\n", value);
    }

    { let (__tmp_0, __tmp_1) = process_slice(numbers.clone(), Rc::new(RefCell::new(Some(10)))); value = __tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.borrow_mut(); __guard.take() }; *err.borrow_mut() = __moved_tmp_1; };
    if (*err.borrow()).is_some() {
        print!("Error: {}\n", format!("{}", (*err.borrow().as_ref().unwrap())));
    } else {
        print!("Value: {}\n", value);
    }

    println!("{}", format!("{}", "\n=== Nested panic example ===".to_string()));
    nested_panic();

    println!("{}", format!("{}", "\n=== Different panic types ===".to_string()));
    demonstrate_panic_types();

    println!("{}", format!("{}", "\n=== Chained defers with panic ===".to_string()));
    chained_defers();

    println!("{}", format!("{}", "\n=== Program completed successfully ===".to_string()));
}