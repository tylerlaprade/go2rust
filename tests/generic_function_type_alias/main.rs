use std::cell::{RefCell};
use std::rc::{Rc};

pub fn words() -> Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> bool>>>>) -> ()>>>> {
    Rc::new(RefCell::new(Some(Box::new(move |r#yield: Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> bool>>>>| {
        if !(*{ let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> bool> = { let mut __f_guard = r#yield.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> bool> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some("go".to_string())))) }.borrow().as_ref().unwrap()) {
        ()
    }
        { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> bool> = { let mut __f_guard = r#yield.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> bool> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some("rust".to_string())))) };
    }) as Box<dyn FnMut(Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> bool>>>>) -> ()>)))
}

pub fn print_until_stop(seq: Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> bool>>>>) -> ()>>>>) {
    { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> bool>>>>) -> ()> = { let mut __f_guard = seq.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> bool>>>>) -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(Box::new(move |word: Rc<RefCell<Option<String>>>| -> bool {
        println!("{} {}", format!("{}", "word:".to_string()), format!("{}", { let __v = (*word.borrow().as_ref().unwrap()).clone(); __v }));
        (*word.borrow().as_ref().unwrap()).clone() != "go"
    }) as Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> bool>)))) };
}

fn main() {
    print_until_stop(words());
}