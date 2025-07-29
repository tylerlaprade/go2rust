mod lib_;
use lib_::*;

fn main() {
    eprintln!("{}", (*get_hello().lock().unwrap().as_ref().unwrap()));
    eprintln!("{}", (*get_world().lock().unwrap().as_ref().unwrap()));
    eprintln!("{}", (*get_magic_number().lock().unwrap().as_ref().unwrap()));
}