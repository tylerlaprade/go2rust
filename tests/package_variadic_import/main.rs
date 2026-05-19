use go2rust_stdlib_stubs::*;
use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    example_com_packagevariadic_label::__go_init_all();

    println!("{}", format!("{}", (*example_com_packagevariadic_label::count(Rc::new(RefCell::new(Some("empty".to_string()))), Rc::new(RefCell::new(Some(vec![])))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*example_com_packagevariadic_label::count(Rc::new(RefCell::new(Some("full".to_string()))), Rc::new(RefCell::new(Some(vec!["alpha".to_string(), "beta".to_string()])))).borrow().as_ref().unwrap())));
}