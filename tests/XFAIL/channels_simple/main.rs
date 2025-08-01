fn main() {
    let mut messages = ;
    // TODO: Unhandled statement type: GoStmt
    let mut msg = std::sync::Arc::new(std::sync::Mutex::new(Some(<-(*messages.lock().unwrap().as_mut().unwrap()))));
    println!("{}", (*msg.lock().unwrap().as_mut().unwrap()));
}