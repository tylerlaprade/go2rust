use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let mut timer1 = (*time.lock().unwrap().as_mut().unwrap()).new_timer(Arc::new(Mutex::new(Some(1 * (*(*time.lock().unwrap().as_mut().unwrap())::second.lock().unwrap().as_ref().unwrap())))));

    <-(*timer1.lock().unwrap().as_ref().unwrap()).c;
    println!("{}", "Timer 1 fired".to_string());

    let mut timer2 = (*time.lock().unwrap().as_mut().unwrap()).new_timer(Arc::new(Mutex::new(Some(500 * (*(*time.lock().unwrap().as_mut().unwrap())::millisecond.lock().unwrap().as_ref().unwrap())))));
    let timer2_closure_clone = timer2.clone(); let C_thread = C.clone(); let timer2_thread = timer2.clone(); std::thread::spawn(move || {
        <-(*timer2.lock().unwrap().as_ref().unwrap()).c;;
        println!("{}", "Timer 2 fired".to_string());;;
    });
    let mut stop2 = (*timer2.lock().unwrap().as_mut().unwrap()).stop();
    if (*stop2.lock().unwrap().as_mut().unwrap()) {
        println!("{}", "Timer 2 stopped".to_string());
    }

    std::thread::sleep(std::time::Duration::from_secs(1));
}