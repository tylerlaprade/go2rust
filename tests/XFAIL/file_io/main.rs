use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let (mut file, mut err) = (*os.borrow_mut().as_mut().unwrap()).create(Rc::new(RefCell::new(Some("test.txt".to_string()))));
    if (*err.borrow()).is_some() {
        println!("{} {}", "Error:".to_string(), (*err.borrow_mut().as_mut().unwrap()));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return
    }
    }
    __defer_stack.push(Box::new(move || {
        (*file.borrow_mut().as_mut().unwrap()).close();
    }));

    (*file.borrow_mut().as_mut().unwrap()).write_string(Rc::new(RefCell::new(Some("Hello, World!".to_string()))));
    println!("{}", "File written successfully".to_string());

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}