fn main() {
    let mut queue = vec![0; 2];
    // TODO: Unhandled statement type: SendStmt
    // TODO: Unhandled statement type: SendStmt
    close(std::sync::Arc::new(std::sync::Mutex::new(Some((*queue.lock().unwrap().as_mut().unwrap())))));
    for elem in 0..(*queue.lock().unwrap().as_mut().unwrap()).len() {
        println!("{}", elem);
    }
}