use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

pub fn build_cleanup(empty: Rc<RefCell<Option<bool>>>) -> (Rc<RefCell<Option<Box<dyn FnMut() -> ()>>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut cleanup: Rc<RefCell<Option<Box<dyn FnMut() -> ()>>>> = Rc::new(RefCell::new(None));
    let mut err: Rc<RefCell<Option<Box<dyn StdError>>>> = Rc::new(RefCell::new(None));

    if (*empty.borrow().as_ref().unwrap()) {
        {
        { let new_val = Box::new(move || {
        println!("{}", "empty".to_string());
    }) as Box<dyn FnMut() -> ()>; *cleanup.borrow_mut() = Some(new_val); };
        *err.borrow_mut() = None;;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (cleanup, err)
    }
    }
    let cleanup_defer_captured = cleanup.clone(); let err_defer_captured = err.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        { let new_val = Box::new(move || {
        println!("{}", "cleanup".to_string());
    }) as Box<dyn FnMut() -> ()>; *cleanup_defer_captured.borrow_mut() = Some(new_val); };
        if (*err_defer_captured.borrow()).is_some() {
        { let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = cleanup_defer_captured.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
        *cleanup_defer_captured.borrow_mut() = None;
    }
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
    {
        *cleanup.borrow_mut() = None;
        *err.borrow_mut() = None;;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (cleanup, err)
    }
}

fn main() {
    let (mut emptyCleanup, mut emptyErr) = build_cleanup(Rc::new(RefCell::new(Some(true))));
    println!("{}", (*emptyErr.borrow()).is_none());
    { let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = emptyCleanup.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };

    let (mut cleanup, mut err) = build_cleanup(Rc::new(RefCell::new(Some(false))));
    println!("{}", (*err.borrow()).is_none());
    { let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = cleanup.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
}