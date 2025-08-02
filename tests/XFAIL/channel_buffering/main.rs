fn main() {
    let mut messages = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![std::sync::Arc::new(std::sync::Mutex::new(Some(0))); 2])));
    // TODO: Unhandled statement type: SendStmt
    // TODO: Unhandled statement type: SendStmt
    println!("{}", <-(*messages.lock().unwrap().as_mut().unwrap()));
    println!("{}", <-(*messages.lock().unwrap().as_mut().unwrap()));
}