use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    '__go_switch_1: loop {
        { let _switch_val = true;
    if _switch_val == (true) {
            if false {
        break '__go_switch_1
    }
            for n in vec![1i64, 2i64, 3i64].iter().copied() {
        if n == 2 {
        break
    }
        println!("{}", format!("{}", n));
    }
            println!("{}", format!("{}", "after".to_string()));
        }
    };
        break;
    }
    println!("{}", format!("{}", "done".to_string()));
}