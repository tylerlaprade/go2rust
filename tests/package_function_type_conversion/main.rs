use go2rust_stdlib_stubs::*;
use std::cell::{RefCell};
use std::rc::{Rc};

pub type Exporter = Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> i32>>>>;


pub fn r#use(e: Exporter) -> i32 {

    return example_com_fnconvert_dep::set(e.clone());
}

fn main() {
    example_com_fnconvert_dep::__go_init_all();

    println!("{}", format!("{}", r#use(Rc::new(RefCell::new(Some(Box::new(move |v: Rc<RefCell<Option<i32>>>| -> i32 {
        return (*v.borrow().as_ref().unwrap()) * 2;
    }) as Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> i32>))))));
}