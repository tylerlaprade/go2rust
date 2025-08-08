use std::sync::{Arc, Mutex};

fn main() {
    let mut i = Arc::new(Mutex::new(Some(0)));

    // TODO: Unhandled statement type: LabeledStmt

    println!("{}", "First loop done".to_string());

    let mut x = Arc::new(Mutex::new(Some(1)));
    if (*x.lock().unwrap().as_mut().unwrap()) > 0 {
        // TODO: goto not supported
    }
    println!("{}", "This won't print".to_string());

    // TODO: Unhandled statement type: LabeledStmt

    let mut j = Arc::new(Mutex::new(Some(0)));
    while (*j.lock().unwrap().as_mut().unwrap()) < 3 {
        let mut k = Arc::new(Mutex::new(Some(0)));
    while (*k.lock().unwrap().as_mut().unwrap()) < 3 {
        if (*j.lock().unwrap().as_mut().unwrap()) == 1 && (*k.lock().unwrap().as_mut().unwrap()) == 1 {
        // TODO: goto not supported
    }
        print!("j={}, k={}\n", (*j.lock().unwrap().as_mut().unwrap()), (*k.lock().unwrap().as_mut().unwrap()));
        { let mut guard = k.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    // TODO: Unhandled statement type: LabeledStmt
}