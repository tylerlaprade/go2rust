fn main() {
    let mut timer1 = (*time.lock().unwrap().as_mut().unwrap()).new_timer(std::sync::Arc::new(std::sync::Mutex::new(Some(2 * (*time.lock().unwrap().as_mut().unwrap()).second))));
    <-(*timer1.lock().unwrap().as_mut().unwrap()).c;
    println!("{}", "Timer 1 fired".to_string());
    let mut timer2 = (*time.lock().unwrap().as_mut().unwrap()).new_timer(std::sync::Arc::new(std::sync::Mutex::new(Some((*time.lock().unwrap().as_mut().unwrap()).second))));
    // TODO: Unhandled statement type: GoStmt
    let mut stop2 = (*timer2.lock().unwrap().as_mut().unwrap()).stop();
    if (*stop2.lock().unwrap().as_mut().unwrap()) {
        println!("{}", "Timer 2 stopped".to_string());
    }
    (*time.lock().unwrap().as_mut().unwrap()).sleep(std::sync::Arc::new(std::sync::Mutex::new(Some(2 * (*time.lock().unwrap().as_mut().unwrap()).second))));
}