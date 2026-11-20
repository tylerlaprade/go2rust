use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

pub fn load() -> (Rc<RefCell<Option<Vec<i64>>>>, Rc<RefCell<Option<Rc<dyn StdError>>>>) {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let _: Rc<RefCell<Option<Vec<i64>>>> = Rc::new(RefCell::new(None));
    let mut err: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(None));

    __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
    {
        let __return_0 = Rc::new(RefCell::new(None));
        { let __rhs_holder = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("missing".to_string()))))).clone(); let new_val = (*__rhs_holder.borrow()).clone(); *err.borrow_mut() = new_val; };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (__return_0, err.clone());
    }
}

fn main() {
    let (_, mut err) = load();
    println!("{}", format!("{}", { let __nil_result = (*err.borrow()).is_some(); __nil_result }));
}