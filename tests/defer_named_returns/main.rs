use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct counter {
}

impl counter {
    pub fn __go_value_clone(&self) -> Self {
        Self {  }
    }
}

impl std::fmt::Display for counter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


impl counter {
    pub fn method(&self) -> i32 {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut result: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(0)));

        let result_defer_captured = result.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        { let __rhs = 3; let mut guard = result_defer_captured.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
        {
        { let new_val = 4; *result.borrow_mut() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.borrow().as_ref().unwrap());
    }
    }
}

pub fn compute() -> i32 {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut result: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(0)));

    let result_defer_captured = result.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        { let __rhs = 10; let mut guard = result_defer_captured.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
    { let new_val = 5; *result.borrow_mut() = Some(new_val); };
    {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.borrow().as_ref().unwrap());
    }
}

pub fn decorate() -> Rc<RefCell<Option<String>>> {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut msg: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(Some(String::new())));

    let msg_defer_captured = msg.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        { let new_val = format!("{}{}", format!("{}{}", "[".to_string(), (*msg_defer_captured.borrow().as_ref().unwrap())), "]".to_string()); *msg_defer_captured.borrow_mut() = Some(new_val); };
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
    { let new_val = "ok".to_string(); *msg.borrow_mut() = Some(new_val); };
    {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return msg;
    }
}

fn main() {
    println!("{}", format!("{}", compute()));
    println!("{}", format!("{}", (*decorate().borrow().as_ref().unwrap())));
    let mut c: Rc<RefCell<Option<counter>>> = Rc::new(RefCell::new(Some(Default::default())));
    println!("{}", format!("{}", (*c.borrow().as_ref().unwrap()).method()));
}