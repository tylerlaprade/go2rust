use go2rust_stdlib_stubs::*;
use std::cell::{RefCell};
use std::rc::{Rc};

pub type Exporter = Rc<RefCell<Option<Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>>>>;


pub fn r#use(e: Exporter) -> Rc<RefCell<Option<i32>>> {

    return example_com_fnconvert_dep::set(e.clone());
}

fn main() {
    println!("{}", (*r#use(Rc::new(RefCell::new(Some(Box::new(move |v: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return {
            let __tmp_x = (*v.borrow().as_ref().unwrap());
            let __tmp_y = 2;
            Rc::new(RefCell::new(Some(__tmp_x * __tmp_y)))
        };
    }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>)))).borrow().as_ref().unwrap()));
}