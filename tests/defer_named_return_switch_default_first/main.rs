use std::cell::{RefCell};
use std::rc::{Rc};

pub fn pick(v: Rc<RefCell<Option<i32>>>) -> i32 {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut res: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(0)));

    if (*v.borrow().as_ref().unwrap()) < 0 {
        __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
    }
    { let _switch_val = (*v.borrow().as_ref().unwrap());
    if _switch_val == (0) {
            {
        { let new_val = 0; *res.borrow_mut() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*res.borrow().as_ref().unwrap());
    }
        } else {
            {
        { let new_val = 1; *res.borrow_mut() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*res.borrow().as_ref().unwrap());
    }
        }
    }
}

fn main() {
    println!("{}", format!("{}", pick(Rc::new(RefCell::new(Some(0))))));
    println!("{}", format!("{}", pick(Rc::new(RefCell::new(Some(2))))));
}