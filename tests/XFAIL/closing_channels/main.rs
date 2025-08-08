use std::sync::{Arc, Mutex};

fn main() {
    let mut jobs = ;
    let mut done = ;

    // TODO: Unhandled statement type: GoStmt

    let mut j = Arc::new(Mutex::new(Some(1)));
    while (*j.lock().unwrap().as_mut().unwrap()) <= 3 {
        // TODO: Unhandled statement type: SendStmt
        println!("{} {}", "sent job".to_string(), (*j.lock().unwrap().as_mut().unwrap()));
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    (close.lock().unwrap().as_ref().unwrap())(jobs.clone());
    println!("{}", "sent all jobs".to_string());

    <-(*done.lock().unwrap().as_mut().unwrap());
}