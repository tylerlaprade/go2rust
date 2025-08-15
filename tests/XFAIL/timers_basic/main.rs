use std::sync::{Arc, Mutex};

fn main() {
    let mut timer1 = time.new_timer(Arc::new(Mutex::new(Some(1 * (*(*time.lock().unwrap().as_mut().unwrap())::second.lock().unwrap().as_ref().unwrap())))));

    <-timer1.c;
    println!("{}", "Timer 1 fired".to_string());

    let mut timer2 = time.new_timer(Arc::new(Mutex::new(Some(500 * (*(*time.lock().unwrap().as_mut().unwrap())::millisecond.lock().unwrap().as_ref().unwrap())))));
    // TODO: Unhandled statement type: GoStmt
    let mut stop2 = (*timer2.lock().unwrap().as_mut().unwrap()).stop();
    if (*stop2.lock().unwrap().as_mut().unwrap()) {
        println!("{}", "Timer 2 stopped".to_string());
    }

    time.sleep(Arc::new(Mutex::new(Some(1 * (*(*time.lock().unwrap().as_mut().unwrap())::second.lock().unwrap().as_ref().unwrap())))));
}