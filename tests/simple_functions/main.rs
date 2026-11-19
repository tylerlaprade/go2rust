mod lib_;
use lib_::*;

fn main() {
    {
            let __go_print_arg_0 = format!("{}", (*get_hello().borrow().as_ref().unwrap()));
            eprintln!("{}", __go_print_arg_0)
        };
    {
            let __go_print_arg_0 = format!("{}", (*get_world().borrow().as_ref().unwrap()));
            eprintln!("{}", __go_print_arg_0)
        };
    {
            let __go_print_arg_0 = format!("{}", get_magic_number());
            eprintln!("{}", __go_print_arg_0)
        };
}