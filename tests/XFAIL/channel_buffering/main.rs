fn main() {
    let mut messages = vec![0; 2];
    // TODO: Unhandled statement type: SendStmt
    // TODO: Unhandled statement type: SendStmt
    println!("{}", <-(*messages.lock().unwrap().as_ref().unwrap()));
    println!("{}", <-(*messages.lock().unwrap().as_ref().unwrap()));
}