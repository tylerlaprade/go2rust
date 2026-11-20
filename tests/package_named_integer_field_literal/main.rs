use go2rust_stdlib_stubs::*;
use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    ::example_com_namedfield_dep::__go_init_all();

    let mut cfg = Rc::new(RefCell::new(Some(example_com_namedfield_dep::r#mod::Config { mode: Rc::new(RefCell::new(Some(example_com_namedfield_dep::r#mod::LoadMode(Rc::new(RefCell::new(Some(((example_com_namedfield_dep::NEED_NAME as i64 | example_com_namedfield_dep::NEED_FILES as i64) as i64 | example_com_namedfield_dep::NEED_TYPES as i64) as i64))))))), ..Default::default() })));
    println!("{}", format!("{}", example_com_namedfield_dep::enabled(cfg.clone(), Rc::new(RefCell::new(Some(example_com_namedfield_dep::r#mod::LoadMode(Rc::new(RefCell::new(Some(example_com_namedfield_dep::NEED_FILES as i64))))))))));
}