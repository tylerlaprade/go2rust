use go2rust_stdlib_stubs::*;
use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    example_com_arrayarg_dep::__go_init_all();

    let mut labels = Rc::new(RefCell::new(Some(vec![{ let __v = example_com_arrayarg_dep::of(Rc::new(RefCell::new(Some(4)))); let __owned = (*__v.borrow().as_ref().unwrap()).clone(); __owned }, { let __v = example_com_arrayarg_dep::of(Rc::new(RefCell::new(Some(5)))); let __owned = (*__v.borrow().as_ref().unwrap()).clone(); __owned }])));
    println!("{}", format!("{}", (*example_com_arrayarg_dep::make(Rc::new(RefCell::new(Some([{ let __v = example_com_arrayarg_dep::of(Rc::new(RefCell::new(Some(1)))); let __owned = (*__v.borrow().as_ref().unwrap()).clone(); __owned }, { let __v = example_com_arrayarg_dep::of(Rc::new(RefCell::new(Some(2)))); let __owned = (*__v.borrow().as_ref().unwrap()).clone(); __owned }, { let __v = example_com_arrayarg_dep::of(Rc::new(RefCell::new(Some(3)))); let __owned = (*__v.borrow().as_ref().unwrap()).clone(); __owned }]))), labels.clone()).borrow().as_ref().unwrap())));
}