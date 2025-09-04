use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
        // This test verifies that external package imports are detected
    println!("{}", "Testing external package imports".to_string());

        // This would normally use testify
    if github_com_stretchr_testify_assert::equal(Rc::new(RefCell::new(Some(None))), Rc::new(RefCell::new(Some(1))), Rc::new(RefCell::new(Some(1)))) {
        println!("{}", "Assert would work here".to_string());
    }
}