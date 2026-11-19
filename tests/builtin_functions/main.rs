use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    {
            let __go_print_arg_0 = format!("{}", "This goes to stderr".to_string());
            eprintln!("{}", __go_print_arg_0)
        };

    let mut s = Rc::new(RefCell::new(Some("hello".to_string())));
    {
            let __go_print_arg_0 = format!("{}", (*s.borrow().as_ref().unwrap()).len());
            eprintln!("{}", __go_print_arg_0)
        };
}