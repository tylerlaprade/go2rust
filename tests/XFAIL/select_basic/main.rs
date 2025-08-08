use std::sync::{Arc, Mutex};

fn main() {
    let mut c1 = ;
    let mut c2 = ;

    // TODO: Unhandled statement type: GoStmt
    // TODO: Unhandled statement type: GoStmt

    let mut i = Arc::new(Mutex::new(Some(0)));
    while (*(*i.lock().unwrap().as_mut().unwrap()).lock().unwrap().as_ref().unwrap()) < 2 {
        // TODO: Unhandled statement type: SelectStmt
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}