use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut visit: Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> bool>>>> = Rc::new(RefCell::new(None));
    let visit_closure_clone = visit.clone(); { let __func_lit_target = visit_closure_clone.clone(); let new_val = Box::new(move |i: Rc<RefCell<Option<i32>>>| -> bool {
        if (*i.borrow().as_ref().unwrap()) == 0 {
        return true;
    }
        return { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> bool> = { let mut __f_guard = visit_closure_clone.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> bool> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some((*i.borrow().as_ref().unwrap()) - 1)))) };
    }) as Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> bool>; *__func_lit_target.borrow_mut() = Some(new_val); };

    println!("{}", format!("{}", { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> bool> = { let mut __f_guard = visit.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> bool> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(3)))) }));
}