use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct Loader {
}

impl Loader {
    pub fn __go_value_clone(&self) -> Self {
        Self {  }
    }
}

impl std::fmt::Display for Loader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


impl Loader {
    pub fn load(&self, patterns: Rc<RefCell<Option<Vec<String>>>>) -> Rc<RefCell<Option<i32>>> {
        return Rc::new(RefCell::new(Some((*patterns.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));
    }
}

fn main() {
    let mut loader = Rc::new(RefCell::new(Some(Loader {  })));
    println!("{}", format!("{}", (*(*loader.borrow().as_ref().unwrap()).load(Rc::new(RefCell::new(Some(vec![".".to_string()])))).borrow().as_ref().unwrap())));
}