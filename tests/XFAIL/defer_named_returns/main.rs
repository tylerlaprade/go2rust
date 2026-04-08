use std::cell::{RefCell};
use std::rc::{Rc};

pub fn compute() -> Rc<RefCell<Option<i32>>> {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut result: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(0)));

    let result_defer_captured = result.clone(); __defer_stack.push(Box::new(move || {
        (*Rc::new(RefCell::new(Some(Box::new(move || {
        { let mut guard = result_defer_captured.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 10); };
    }) as Box<dyn Fn() -> ()>))).borrow().as_ref().unwrap())();
    }));
    { let new_val = 5; *result.borrow_mut() = Some(new_val); };
    {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return result
    }

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}

pub fn decorate() -> Rc<RefCell<Option<String>>> {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut msg: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(Some(String::new())));

    let msg_defer_captured = msg.clone(); __defer_stack.push(Box::new(move || {
        (*Rc::new(RefCell::new(Some(Box::new(move || {
        { let new_val = format!("{}{}", format!("{}{}", "[".to_string(), (*msg_defer_captured.borrow().as_ref().unwrap())), "]".to_string()); *msg_defer_captured.borrow_mut() = Some(new_val); };
    }) as Box<dyn Fn() -> ()>))).borrow().as_ref().unwrap())();
    }));
    { let new_val = "ok".to_string(); *msg.borrow_mut() = Some(new_val); };
    {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return msg
    }

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}

fn main() {
    println!("{}", (*compute().borrow().as_ref().unwrap()));
    println!("{}", (*decorate().borrow().as_ref().unwrap()));
}