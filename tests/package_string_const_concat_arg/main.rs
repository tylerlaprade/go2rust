use go2rust_stdlib_stubs::*;
use std::cell::{RefCell};
use std::rc::{Rc};

pub fn inside(path: Rc<RefCell<Option<String>>>, dir: Rc<RefCell<Option<String>>>) -> bool {

    return (*Rc::new(RefCell::new(Some({ let __s = (*path.borrow().as_ref().unwrap()).clone(); let __arg = format!("{}{}", (*dir.borrow().as_ref().unwrap()), (*Rc::new(RefCell::new(Some(char::from_u32((example_com_stringconst_dep::SEPARATOR) as u32).unwrap().to_string()))).borrow().as_ref().unwrap())); __s.starts_with(&__arg) }))).borrow().as_ref().unwrap());
}

fn main() {
    example_com_stringconst_dep::__go_init_all();

    println!("{}", format!("{}", inside(Rc::new(RefCell::new(Some("root/file.go".to_string()))), Rc::new(RefCell::new(Some("root".to_string()))))));
}