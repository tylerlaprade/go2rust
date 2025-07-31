fn main() {
    let mut ticker = (*time.lock().unwrap().as_ref().unwrap()).new_ticker(std::sync::Arc::new(std::sync::Mutex::new(Some(500 * (*time.lock().unwrap().as_ref().unwrap()).millisecond))));
    let mut done = ;
    // TODO: Unhandled statement type: GoStmt
    (*time.lock().unwrap().as_ref().unwrap()).sleep(std::sync::Arc::new(std::sync::Mutex::new(Some(1600 * (*time.lock().unwrap().as_ref().unwrap()).millisecond))));
    (*ticker.lock().unwrap().as_ref().unwrap()).stop();
    // TODO: Unhandled statement type: SendStmt
    println!("{}", "Ticker stopped".to_string());
}