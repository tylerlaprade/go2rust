use std::cell::{RefCell};
use std::rc::{Rc};

pub fn wrap(name: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {
    let mut out: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(Some(Default::default())));
    (*out.borrow_mut().as_mut().unwrap()).push_str(&format!("{}{}", format!("{}{}", "(".to_string(), (*name.borrow().as_ref().unwrap())), ")".to_string()));
    return Rc::new(RefCell::new(Some({ let __builder = out.clone(); let __guard = __builder.borrow(); let __value = (*__guard.as_ref().unwrap()).clone(); drop(__guard); __value })));
}

fn main() {
    println!("{}", format!("{}", (*wrap(Rc::new(RefCell::new(Some("gopher".to_string())))).borrow().as_ref().unwrap())));
}