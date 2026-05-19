use go2rust_stdlib_stubs::*;
use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut v = example_com_package_named_const_argument_helper::new_var();
    example_com_package_named_const_argument_helper::set_kind(v.clone(), Rc::new(RefCell::new(Some(example_com_package_named_const_argument_helper::Kind(Rc::new(RefCell::new(Some(example_com_package_named_const_argument_helper::PACKAGE_VAR as u8))))))));
    println!("{}", format!("{}", "ok".to_string()));
}