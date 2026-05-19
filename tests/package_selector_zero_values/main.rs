use go2rust_stdlib_stubs::*;
use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut idx: Rc<RefCell<Option<example_com_selectorzero_model::Index>>> = Rc::new(RefCell::new(Some(example_com_selectorzero_model::Index(Rc::new(RefCell::new(Some(0)))))));
    println!("{}", format!("{}", { let __v = (*idx.borrow().as_ref().unwrap()).clone(); __v }));
}