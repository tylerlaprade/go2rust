use go2rust_stdlib_stubs::*;
use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    example_com_namedfield_dep::__go_init_all();

    let mut cfg = Rc::new(RefCell::new(Some(example_com_namedfield_dep::Config { mode: Rc::new(RefCell::new(Some(example_com_namedfield_dep::LoadMode(Rc::new(RefCell::new(Some(example_com_namedfield_dep::NEED_NAME as i32 | example_com_namedfield_dep::NEED_FILES as i32 as i32)))) | example_com_namedfield_dep::LoadMode(Rc::new(RefCell::new(Some(example_com_namedfield_dep::NEED_TYPES as i32))))))), ..Default::default() })));
    println!("{}", format!("{}", example_com_namedfield_dep::enabled(cfg.clone(), Rc::new(RefCell::new(Some(example_com_namedfield_dep::NEED_FILES))))));
}