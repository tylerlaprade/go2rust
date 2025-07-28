mod lib_;
use lib_::*;

fn main() {
    eprintln!("{}", get_hello());
    eprintln!("{}", get_world());
    eprintln!("{}", get_magic_number());
}