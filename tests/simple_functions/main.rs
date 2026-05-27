mod lib_;
use lib_::*;

fn main() {
    eprintln!("{}", format!("{}", (*get_hello().borrow().as_ref().unwrap())));
    eprintln!("{}", format!("{}", (*get_world().borrow().as_ref().unwrap())));
    eprintln!("{}", format!("{}", get_magic_number()));
}