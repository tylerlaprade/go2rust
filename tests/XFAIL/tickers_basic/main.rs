use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let mut ticker = (*time.lock().unwrap().as_mut().unwrap())::new_ticker(Arc::new(Mutex::new(Some(250 * (*(*time.lock().unwrap().as_mut().unwrap())::millisecond.lock().unwrap().as_ref().unwrap())))));
    let mut done = ;

    let done_closure_clone = done.clone(); let ticker_closure_clone = ticker.clone(); let C_thread = C.clone(); let done_thread = done.clone(); let ticker_thread = ticker.clone(); std::thread::spawn(move || {
        while true {
        // TODO: Unhandled statement type: SelectStmt
    };;
    });

    std::thread::sleep(std::time::Duration::from_millis(800));
    (*ticker.lock().unwrap().as_mut().unwrap()).stop();
    // TODO: Unhandled statement type: SendStmt
    println!("{}", "Ticker stopped".to_string());
}