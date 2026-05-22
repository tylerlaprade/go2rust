use go2rust_stdlib_stubs::*;
use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    example_com_package_named_string_conversion_helper::__go_init_all();

    let mut p = Rc::new(RefCell::new(Some(example_com_package_named_string_conversion_helper::Path(Rc::new(RefCell::new(Some((*example_com_package_named_string_conversion_helper::text().borrow().as_ref().unwrap()).clone())))))));
    println!("{}", format!("{}", (*p.borrow().as_ref().unwrap()).0.borrow().as_ref().unwrap().clone() == "".to_string()));
    println!("{}", format!("{}", (*example_com_package_named_string_conversion_helper::object(Rc::new(RefCell::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })))).borrow().as_ref().unwrap())));
}