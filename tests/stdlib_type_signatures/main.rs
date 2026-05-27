use std::cell::{RefCell};
use std::rc::{Rc};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ast_File;

impl std::fmt::Display for ast_File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_File>")
    }
}


impl ast_File {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


pub fn label(file: Rc<RefCell<Option<ast_File>>>) -> Rc<RefCell<Option<String>>> {
    Rc::new(RefCell::new(Some("ok".to_string())))
}

fn main() {
    println!("{}", format!("{}", (*label(Rc::new(RefCell::new(None))).borrow().as_ref().unwrap())));
}