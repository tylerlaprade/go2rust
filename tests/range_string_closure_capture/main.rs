use std::cell::{RefCell};
use std::rc::{Rc};

pub fn echo(s: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {
    Rc::new(RefCell::new(Some(s.borrow().as_ref().unwrap().clone())))
}

fn main() {
    let mut names = Rc::new(RefCell::new(Some(vec!["alpha".to_string(), "beta".to_string()])));
    { let __range_holder = names.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for name in __range_values.iter() {
        let name_closure_clone = name.clone(); { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        println!("{}", format!("{}", (*echo(Rc::new(RefCell::new(Some(name_closure_clone.clone())))).borrow().as_ref().unwrap())));
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    } }
}