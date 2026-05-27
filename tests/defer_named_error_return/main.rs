use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

pub fn compute() -> (i32, Rc<RefCell<Option<Box<dyn StdError>>>>) {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut result: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(0)));
    let mut err: Rc<RefCell<Option<Box<dyn StdError>>>> = Rc::new(RefCell::new(None));

    let result_defer_captured = result.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        { let mut guard = result_defer_captured.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
    { let __rhs_holder = Rc::new(RefCell::new(Some(Box::<dyn std::error::Error>::from("boom".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.borrow_mut(); guard.take() }; *err.borrow_mut() = new_val; };
    {
        { let new_val = 2; *result.borrow_mut() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return ((*result.borrow().as_ref().unwrap()), err)
    }
}

fn main() {
    let (mut result, mut err) = compute();
    println!("{}", format!("{}", result));
    println!("{}", format!("{}", format!("{}", (*err.borrow().as_ref().unwrap()))));
}