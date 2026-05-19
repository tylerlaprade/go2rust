use std::cell::{RefCell};
use std::rc::{Rc};

pub fn defer_example() {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    println!("{}", format!("{}", "Start of function".to_string()));

    __defer_stack.push(Box::new(move || {
        println!("{}", format!("{}", "Deferred 1".to_string()));
    }));
    __defer_stack.push(Box::new(move || {
        println!("{}", format!("{}", "Deferred 2".to_string()));
    }));
    __defer_stack.push(Box::new(move || {
        println!("{}", format!("{}", "Deferred 3".to_string()));
    }));

    println!("{}", format!("{}", "Middle of function".to_string()));

    __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        println!("{}", format!("{}", "Anonymous deferred function".to_string()));
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));

    println!("{}", format!("{}", "End of function".to_string()));

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}

pub fn defer_with_variables() {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut x = Rc::new(RefCell::new(Some(10)));
    let x_defer_captured = x.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        println!("{} {}", format!("{}", "Deferred x:".to_string()), format!("{}", { let __v = (*x_defer_captured.borrow().as_ref().unwrap()).clone(); __v }));
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));

    { let new_val = 20; *x.borrow_mut() = Some(new_val); };
    println!("{} {}", format!("{}", "Current x:".to_string()), format!("{}", { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v }));

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}

pub fn defer_in_loop() {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    println!("{}", format!("{}", "Defer in loop:".to_string()));
    let mut i = Rc::new(RefCell::new(Some(0)));
    while (*i.borrow().as_ref().unwrap()) < 3 {
        let __defer_arg_0 = Rc::new(RefCell::new(Some((*i.borrow().as_ref().unwrap()).clone()))); __defer_stack.push(Box::new(move || {
        (move |val: Rc<RefCell<Option<i32>>>| {
        print!("Deferred loop value: {}\n", { let __v = (*val.borrow().as_ref().unwrap()).clone(); __v });;
        })(__defer_arg_0);
    }));
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    println!("{}", format!("{}", "Loop finished".to_string()));

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}

pub fn cleanup() {
    println!("{}", format!("{}", "Cleanup function called".to_string()));
}

pub fn resource_example() {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    println!("{}", format!("{}", "Acquiring resource".to_string()));
    __defer_stack.push(Box::new(move || {
        cleanup();
    }));

    println!("{}", format!("{}", "Using resource".to_string()));

        // Simulate some work
    let mut i = Rc::new(RefCell::new(Some(0)));
    while (*i.borrow().as_ref().unwrap()) < 3 {
        print!("Working... {}\n", (*i.borrow().as_ref().unwrap()) + 1);
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    println!("{}", format!("{}", "Done with resource".to_string()));

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}

fn main() {
    println!("{}", format!("{}", "=== Basic defer example ===".to_string()));
    defer_example();

    println!("{}", format!("{}", "\n=== Defer with variables ===".to_string()));
    defer_with_variables();

    println!("{}", format!("{}", "\n=== Defer in loop ===".to_string()));
    defer_in_loop();

    println!("{}", format!("{}", "\n=== Resource cleanup example ===".to_string()));
    resource_example();

    println!("{}", format!("{}", "\n=== Main function ending ===".to_string()));
}