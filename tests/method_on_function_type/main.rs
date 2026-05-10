use std::cell::{RefCell};
use std::rc::{Rc};

pub type formatter = Rc<RefCell<Option<Box<dyn Fn(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>>>>>;


pub trait formatterMethods {
    fn format(&self, path: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>;
}

impl formatterMethods for formatter {
    fn format(&self, path: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {
        return { let __f_guard = self.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)(path.clone()) };
    }
}

fn main() {
    let mut f = Rc::new(RefCell::new(Some(Box::new(move |path: Rc<RefCell<Option<String>>>| -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some(format!("{}{}", "pkg:".to_string(), (*path.borrow().as_ref().unwrap())))));
    }) as Box<dyn Fn(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>>)));
    println!("{}", (*f.format(Rc::new(RefCell::new(Some("fmt".to_string())))).borrow().as_ref().unwrap()));
}