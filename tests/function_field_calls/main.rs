use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Clone, Default)]
pub struct Runner {
    pub callback: Rc<RefCell<Option<Box<dyn Fn(Rc<RefCell<Option<String>>>) -> ()>>>>,
}

impl std::fmt::Display for Runner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", "<func>")
    }
}


impl Runner {
    pub fn run(&mut self) {
        { let __f_holder = self.callback.clone(); let __f_guard = __f_holder.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)(Rc::new(RefCell::new(Some("ok".to_string())))) };
    }
}

pub fn print_value(value: Rc<RefCell<Option<String>>>) {
    println!("{}", { let __v = (*value.borrow().as_ref().unwrap()).clone(); __v });
}

fn main() {
    let mut r = Rc::new(RefCell::new(Some(Runner { callback: Rc::new(RefCell::new(Some(Box::new(move |__arg0: Rc<RefCell<Option<String>>>| { print_value(__arg0) }) as Box<dyn Fn(Rc<RefCell<Option<String>>>) -> ()>))), ..Default::default() })));
    (*r.borrow_mut().as_mut().unwrap()).run();
}