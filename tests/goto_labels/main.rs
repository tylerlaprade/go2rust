use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut i = Rc::new(RefCell::new(Some(0)));

    'r#loop: loop {
        if (*i.borrow().as_ref().unwrap()) < 5 {
        print!("i = {}\n", { let __v = (*i.borrow().as_ref().unwrap()).clone(); __v });
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        continue 'r#loop
    }
        break 'r#loop;
    }

    'skip: {
        println!("{}", "First loop done".to_string());
                // Goto to skip code
        let mut x = Rc::new(RefCell::new(Some(1)));
        if (*x.borrow().as_ref().unwrap()) > 0 {
        break 'skip
    }
        println!("{}", "This won't print".to_string());
    }
    println!("{}", "Skipped to here".to_string());

    'done: {
                // More complex goto pattern
        let mut j = Rc::new(RefCell::new(Some(0)));
    while (*j.borrow().as_ref().unwrap()) < 3 {
        let mut k = Rc::new(RefCell::new(Some(0)));
    while (*k.borrow().as_ref().unwrap()) < 3 {
        if (*j.borrow().as_ref().unwrap()) == 1 && (*k.borrow().as_ref().unwrap()) == 1 {
        break 'done
    }
        print!("j={}, k={}\n", { let __v = (*j.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*k.borrow().as_ref().unwrap()).clone(); __v });
        { let mut guard = k.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
    println!("{}", "All done".to_string());
}