use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mut messages = ;

    let messages_thread = messages.clone(); std::thread::spawn(move || {
        // TODO: Unhandled statement type: SendStmt;;
    });

    let mut msg = Arc::new(Mutex::new(Some(<-(*messages.lock().unwrap().as_mut().unwrap()))));
    println!("{}", (*msg.lock().unwrap().as_mut().unwrap()));
}