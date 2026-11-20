use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    if true {
        let mut hour = Rc::new(RefCell::new(Some("a".to_string())));
        { (*hour.borrow_mut().as_mut().unwrap()).push_str(&"b".to_string()); };
        {
            let __go_print_arg_0 = format!("{}", { let __v = (*hour.borrow().as_ref().unwrap()).clone(); __v });
            eprintln!("{}", __go_print_arg_0)
        };
    }

    let mut hour = Rc::new(RefCell::new(Some(0)));
    { let __rhs = 12; let mut guard = hour.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    {
            let __go_print_arg_0 = format!("{}", { let __v = (*hour.borrow().as_ref().unwrap()).clone(); __v });
            eprintln!("{}", __go_print_arg_0)
        };
}