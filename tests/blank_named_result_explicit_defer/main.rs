use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

pub fn load() -> (Rc<RefCell<Option<Vec<i64>>>>, Rc<RefCell<Option<Rc<dyn StdError>>>>) {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let _: Rc<RefCell<Option<Vec<i64>>>> = Rc::new(RefCell::new(None));
    let mut err: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(None));

    let mut err_defer_captured = err.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        if { let __nil_result = (*err_defer_captured.borrow()).is_none(); __nil_result } {
        { let __rhs_holder = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("deferred".to_string()))))).clone(); let new_val = (*__rhs_holder.borrow()).clone(); *err_defer_captured.borrow_mut() = new_val; };
    }
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
    let mut values = Rc::new(RefCell::new(Some(vec![1i64, 2i64])));
    {
        let __return_0 = Rc::new(RefCell::new(Some((*values.borrow().as_ref().unwrap()).clone())));
        *err.borrow_mut() = None;;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (__return_0, err.clone());
    }
}

pub fn pair() -> (Rc<RefCell<Option<Vec<i64>>>>, Rc<RefCell<Option<Rc<dyn StdError>>>>) {
    (Rc::new(RefCell::new(Some(vec![3i64, 4i64, 5i64]))), Rc::new(RefCell::new(None)))
}

pub fn load_tuple() -> (Rc<RefCell<Option<Vec<i64>>>>, Rc<RefCell<Option<Rc<dyn StdError>>>>) {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let _: Rc<RefCell<Option<Vec<i64>>>> = Rc::new(RefCell::new(None));
    let mut err: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(None));

    let mut err_defer_captured = err.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        if { let __nil_result = (*err_defer_captured.borrow()).is_none(); __nil_result } {
        { let __rhs_holder = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("tuple deferred".to_string()))))).clone(); let new_val = (*__rhs_holder.borrow()).clone(); *err_defer_captured.borrow_mut() = new_val; };
    }
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
    {
        let (mut __return_0, mut __return_1) = pair();
        { let __moved_val = { let mut __guard = __return_1.borrow_mut(); __guard.take() }; *err.borrow_mut() = __moved_val; }
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (__return_0, err.clone());
    }
}

fn main() {
    let (mut values, mut err) = load();
    println!("{} {} {}", format!("{}", (*values.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", { let __nil_result = (*err.borrow()).is_some(); __nil_result }), format!("{}", { let __text = match err.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    { let (__tmp_0, __tmp_1) = load_tuple(); let __moved_tmp_0 = { let mut __guard = __tmp_0.borrow_mut(); __guard.take() }; *values.borrow_mut() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.borrow_mut(); __guard.take() }; *err.borrow_mut() = __moved_tmp_1; };
    println!("{} {} {}", format!("{}", (*values.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", { let __nil_result = (*err.borrow()).is_some(); __nil_result }), format!("{}", { let __text = match err.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
}