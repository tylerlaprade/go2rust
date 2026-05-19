use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

pub fn collect(names: Rc<RefCell<Option<Vec<String>>>>) -> Rc<RefCell<Option<Vec<Rc<RefCell<Option<Box<dyn StdError>>>>>>>> {

    let mut n = Rc::new(RefCell::new(Some((*names.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));let mut errs: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Box<dyn StdError>>>>>>>> = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(None::<Box<dyn StdError>>)); ((*n.borrow().as_ref().unwrap())) as usize])));
    { let __range_holder = names.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for (i, name) in __range_values.iter().enumerate() {
        let errs_closure_clone = errs.clone(); let i_closure_clone = i.clone(); let name_closure_clone = name.clone(); { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        (*errs_closure_clone.borrow_mut().as_mut().unwrap())[(i_closure_clone) as usize] = Rc::new(RefCell::new(Some(Box::<dyn std::error::Error>::from(name_closure_clone.clone()))));
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    } }
    return errs.clone();
}

fn main() {
    let mut errs = collect(Rc::new(RefCell::new(Some(vec!["alpha".to_string(), "beta".to_string()]))));
    println!("{} {}", format!("{}", (*errs.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", (*errs.borrow().as_ref().unwrap())[(1) as usize].clone().borrow().as_ref().unwrap())))).borrow().as_ref().unwrap())));
}