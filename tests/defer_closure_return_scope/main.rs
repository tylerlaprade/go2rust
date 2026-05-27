use std::cell::{RefCell};
use std::rc::{Rc};

pub fn run() -> i32 {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut result: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(0)));

    let result_defer_captured = result.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        { let new_val = 7; *result_defer_captured.borrow_mut() = Some(new_val); };
        ()
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
    {
        { let new_val = 3; *result.borrow_mut() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.borrow().as_ref().unwrap());
    }
}

fn main() {
    println!("{}", format!("{}", run()));
}