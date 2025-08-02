fn main() {
    let mut queue = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![std::sync::Arc::new(std::sync::Mutex::new(Some(0))); 2])));
    // TODO: Unhandled statement type: SendStmt
    // TODO: Unhandled statement type: SendStmt
    close(queue.clone());
    for elem in 0..(*queue.lock().unwrap().as_mut().unwrap()).len() {
        println!("{}", elem);
    }
}