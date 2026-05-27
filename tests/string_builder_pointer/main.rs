use std::cell::{RefCell};
use std::rc::{Rc};

pub fn append_parts(out: Rc<RefCell<Option<String>>>, suffix: Rc<RefCell<Option<String>>>) {
    (*out.borrow_mut().as_mut().unwrap()).push_str("go");
    (*out.borrow_mut().as_mut().unwrap()).push((('2' as i32)) as u8 as char);
    (*out.borrow_mut().as_mut().unwrap()).push('r');
    (*out.borrow_mut().as_mut().unwrap()).push_str(&(*suffix.borrow().as_ref().unwrap()).clone());
}

pub fn read(out: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {
    Rc::new(RefCell::new(Some({ let __builder = out.clone(); let __guard = __builder.borrow(); let __value = (*__guard.as_ref().unwrap()).clone(); drop(__guard); __value })))
}

pub fn size(out: Rc<RefCell<Option<String>>>) -> i32 {
    Rc::new(RefCell::new(Some((*out.borrow().as_ref().unwrap()).len() as i32)))
}

fn main() {
    let mut builder: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(Some(Default::default())));
    append_parts(builder.clone(), Rc::new(RefCell::new(Some("ust".to_string()))));
    println!("{} {}", format!("{}", (*read(builder.clone()).borrow().as_ref().unwrap())), format!("{}", size(builder.clone())));
}