use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
        // Basic fallthrough
    let mut x = Rc::new(RefCell::new(Some(2)));
    {
        let _switch_val = (*x.borrow().as_ref().unwrap());
        let mut _fallthrough = false;
        let mut _matched = false;
        if !_matched && (_switch_val == 1) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            println!("{}", format!("{}", "One".to_string()));
        }
        if !_matched && (_switch_val == 2) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            println!("{}", format!("{}", "Two".to_string()));
            _fallthrough = true;
        }
        if !_matched && (_switch_val == 3) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            println!("{}", format!("{}", "Three (via fallthrough)".to_string()));
        }
        if !_matched && (_switch_val == 4) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            println!("{}", format!("{}", "Four".to_string()));
        }
        if !_matched || _fallthrough {
            _matched = true;
            _fallthrough = false;
            println!("{}", format!("{}", "Other".to_string()));
        }
    }

    println!("{}", format!("{}", "---".to_string()));

        // Multiple fallthrough
    let mut grade = Rc::new(RefCell::new(Some(('B' as i32))));
    {
        let _switch_val = (*grade.borrow().as_ref().unwrap());
        let mut _fallthrough = false;
        let mut _matched = false;
        if !_matched && (_switch_val == ('A' as i32)) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            println!("{}", format!("{}", "Excellent!".to_string()));
            _fallthrough = true;
        }
        if !_matched && (_switch_val == ('B' as i32)) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            println!("{}", format!("{}", "Good job!".to_string()));
            _fallthrough = true;
        }
        if !_matched && (_switch_val == ('C' as i32)) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            println!("{}", format!("{}", "Passed".to_string()));
        }
        if !_matched && (_switch_val == ('D' as i32)) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            println!("{}", format!("{}", "Barely passed".to_string()));
        }
        if !_matched && (_switch_val == ('F' as i32)) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            println!("{}", format!("{}", "Failed".to_string()));
        }
    }

    println!("{}", format!("{}", "---".to_string()));

        // Fallthrough with conditions
    let mut n = Rc::new(RefCell::new(Some(15)));
    {
        let mut _fallthrough = false;
        let mut _matched = false;
        if !_matched && ((*n.borrow().as_ref().unwrap()) % 15 == 0) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            println!("{}", format!("{}", "FizzBuzz".to_string()));
            _fallthrough = true;
        }
        if !_matched && ((*n.borrow().as_ref().unwrap()) % 3 == 0) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            println!("{}", format!("{}", "Fizz".to_string()));
        }
        if !_matched && ((*n.borrow().as_ref().unwrap()) % 5 == 0) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            println!("{}", format!("{}", "Buzz".to_string()));
        }
        if !_matched || _fallthrough {
            _matched = true;
            _fallthrough = false;
            println!("{}", format!("{}", { let __v = (*n.borrow().as_ref().unwrap()).clone(); __v }));
        }
    }
}