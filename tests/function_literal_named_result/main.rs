use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut build = Rc::new(RefCell::new(Some(Box::new(move || -> (Rc<RefCell<Option<String>>>) {
    let mut value: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(Some(String::new())));

        { let new_val = "named result".to_string(); *value.borrow_mut() = Some(new_val); };
        return value.clone();
    }) as Box<dyn Fn() -> (Rc<RefCell<Option<String>>>)>)));
    println!("{}", (*{ let __f_guard = build.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)() }.borrow().as_ref().unwrap()));
}