use std::any::Any;
use std::cell::{RefCell};
use std::rc::{Rc};

pub fn sort_pair<S, E: Any + Clone + 'static>(x: Rc<RefCell<Option<Vec<Rc<RefCell<Option<E>>>>>>>, less: Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<E>>>, Rc<RefCell<Option<E>>>) -> bool>>>>) {
    if { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<E>>>, Rc<RefCell<Option<E>>>) -> bool> = { let mut __f_guard = less.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<E>>>, Rc<RefCell<Option<E>>>) -> bool> }; let __f = unsafe { &mut *__f_ptr }; (*__f)((*x.borrow().as_ref().unwrap())[(1) as usize].clone(), (*x.borrow().as_ref().unwrap())[(0) as usize].clone()) } {
        { let __tmp_0 = (*x.borrow().as_ref().unwrap())[(1) as usize].clone(); let __tmp_1 = (*x.borrow().as_ref().unwrap())[(0) as usize].clone(); (*x.borrow_mut().as_mut().unwrap())[(0) as usize] = __tmp_0; (*x.borrow_mut().as_mut().unwrap())[(1) as usize] = __tmp_1; };
    }
}

pub fn less_string(a: Rc<RefCell<Option<String>>>, b: Rc<RefCell<Option<String>>>) -> bool {
    (*a.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap())
}

fn main() {
    let mut values = Rc::new(RefCell::new(Some(vec!["b".to_string(), "a".to_string()])));
    { let __slice_holder_0 = values.clone(); let __slice_arg_0 = { let __slice_guard_0 = __slice_holder_0.borrow(); Rc::new(RefCell::new(__slice_guard_0.as_ref().map(|__v| __v.iter().cloned().map(|__elem| Rc::new(RefCell::new(Some(__elem)))).collect::<Vec<_>>()))) }; sort_pair::<Vec<String>, String>(__slice_arg_0.clone(), Rc::new(RefCell::new(Some(Box::new(move |__arg0: Rc<RefCell<Option<String>>>, __arg1: Rc<RefCell<Option<String>>>| -> bool { less_string(__arg0, __arg1) }) as Box<dyn FnMut(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> bool>)))); let __converted_values_0 = { let __converted_guard_0 = __slice_arg_0.borrow(); __converted_guard_0.as_ref().map(|__v| __v.iter().cloned().map(|__elem| (*__elem.borrow().as_ref().unwrap()).clone()).collect::<Vec<_>>()) }; *__slice_holder_0.borrow_mut() = __converted_values_0; };
    println!("{} {}", format!("{}", (*values.borrow().as_ref().unwrap())[(0) as usize].clone()), format!("{}", (*values.borrow().as_ref().unwrap())[(1) as usize].clone()));
}