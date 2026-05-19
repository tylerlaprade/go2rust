use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut result: Rc<RefCell<Option<Vec<i32>>>> = Rc::new(RefCell::new(None));
    let result_closure_clone = result.clone(); let mut set = Rc::new(RefCell::new(Some(Box::new(move |values: Rc<RefCell<Option<Vec<i32>>>>| {
        { let new_val = values.borrow().as_ref().unwrap().clone(); *result_closure_clone.borrow_mut() = Some(new_val); };
    }) as Box<dyn FnMut(Rc<RefCell<Option<Vec<i32>>>>) -> ()>)));

    { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<Vec<i32>>>>) -> ()> = { let mut __f_guard = set.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<Vec<i32>>>>) -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(vec![4, 5])))) };
    println!("{}", format!("{}", (*result.borrow().as_ref().unwrap())[(0) as usize].clone()));
    println!("{}", format!("{}", (*result.borrow().as_ref().unwrap()).len()));
}