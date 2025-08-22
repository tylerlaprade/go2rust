mod lib_;
use lib_::*;

fn main() {
    eprintln!("{}", (*get_hello().borrow().as_ref().unwrap()));
    eprintln!("{}", (*get_world().borrow().as_ref().unwrap()));
    eprintln!("{}", (*get_magic_number().borrow().as_ref().unwrap()));
}