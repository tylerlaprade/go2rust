use go2rust_stdlib_stubs::*;
use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    example_com_selectorzero_model::__go_init_all();

    let mut idx: Rc<RefCell<Option<example_com_selectorzero_model::Index>>> = Rc::new(RefCell::new(Some(example_com_selectorzero_model::Index(Rc::new(RefCell::new(Some(0)))))));
    println!("{}", format!("{}", { let __v = (*idx.borrow().as_ref().unwrap()).clone(); __v }));
}