use go2rust_stdlib_stubs::*;
use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    println!("{}", format!("{}", (*example_com_nilslice_dep::count(Rc::new(RefCell::new(None))).borrow().as_ref().unwrap())));
}