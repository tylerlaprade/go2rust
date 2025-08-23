use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mut jobs = ;
    let mut done = ;

    let done_closure_clone = done.clone(); let jobs_closure_clone = jobs.clone(); let done_thread = done.clone(); let jobs_thread = jobs.clone(); std::thread::spawn(move || {
        while true {
        let (mut j, mut more) = <-(*jobs_thread.lock().unwrap().as_mut().unwrap());
        if (*more.lock().unwrap().as_mut().unwrap()) {
        println!("{} {}", "received job".to_string(), (*j.lock().unwrap().as_mut().unwrap()));
    } else {
        println!("{}", "received all jobs".to_string());
        // TODO: Unhandled statement type: SendStmt
        return;
    }
    };;
    });

    let mut j = Arc::new(Mutex::new(Some(1)));
    while (*j.lock().unwrap().as_mut().unwrap()) <= 3 {
        // TODO: Unhandled statement type: SendStmt
        println!("{} {}", "sent job".to_string(), (*j.lock().unwrap().as_mut().unwrap()));
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    (*close.lock().unwrap().as_ref().unwrap())(jobs.clone());
    println!("{}", "sent all jobs".to_string());

    <-(*done.lock().unwrap().as_mut().unwrap());
}