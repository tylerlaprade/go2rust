use go2rust_stdlib_stubs::*;
use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    ::unicode_utf8::__go_init_all();

    {
            let __go_print_arg_0 = format!("{}", unicode_utf8::rune_count_in_string(Rc::new(RefCell::new(Some("h\u{e9}".to_string())))));
            eprintln!("{}", __go_print_arg_0)
        };
}