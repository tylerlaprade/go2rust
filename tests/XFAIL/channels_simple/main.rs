use std::sync::{Arc, Mutex};

fn main() {
    let mut messages = ;

    // TODO: Unhandled statement type: GoStmt

    let mut msg = Arc::new(Mutex::new(Some(<-(*messages.lock().unwrap().as_mut().unwrap()))));
    println!("{}", (*msg.lock().unwrap().as_mut().unwrap()));
}