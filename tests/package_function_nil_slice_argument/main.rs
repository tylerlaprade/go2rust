use go2rust_stdlib_stubs::*;
use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    example_com_nilslice_dep::__go_init_all();

    println!("{}", format!("{}", example_com_nilslice_dep::count(Rc::new(RefCell::new(None)))));
}