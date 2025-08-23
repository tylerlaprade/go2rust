use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let mut c1 = ;
    let mut c2 = ;

    let c1_closure_clone = c1.clone(); let c1_thread = c1.clone(); std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(500));;
        // TODO: Unhandled statement type: SendStmt;;
    });
    let c2_closure_clone = c2.clone(); let c2_thread = c2.clone(); std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(1));;
        // TODO: Unhandled statement type: SendStmt;;
    });

    let mut i = Arc::new(Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_mut().unwrap()) < 2 {
        // TODO: Unhandled statement type: SelectStmt
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}