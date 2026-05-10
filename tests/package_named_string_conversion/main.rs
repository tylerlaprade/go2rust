use go2rust_stdlib_stubs::*;
use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut p = Rc::new(RefCell::new(Some(example_com_package_named_string_conversion_helper::Path(Rc::new(RefCell::new(Some((*example_com_package_named_string_conversion_helper::text().borrow().as_ref().unwrap()).clone())))))));
    println!("{}", (*p.borrow().as_ref().unwrap()).0.borrow().as_ref().unwrap().clone() == "".to_string());
    println!("{}", (*example_com_package_named_string_conversion_helper::object(Rc::new(RefCell::new(Some((*p.borrow().as_ref().unwrap()).clone())))).borrow().as_ref().unwrap()));
}