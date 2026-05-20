use go2rust_stdlib_stubs::*;
use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    example_com_atomicdep_dep::__go_init_all();

    let mut counter = example_com_atomicdep_dep::new_counter();
    println!("{}", format!("{}", (*(*counter.borrow().as_ref().unwrap()).add(Rc::new(RefCell::new(Some(1 as i32)))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*(*counter.borrow().as_ref().unwrap()).add(Rc::new(RefCell::new(Some(-1 as i32)))).borrow().as_ref().unwrap())));
}