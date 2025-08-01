fn main() {
    let mut c1 = ;
    let mut c2 = ;
    // TODO: Unhandled statement type: GoStmt
    // TODO: Unhandled statement type: GoStmt
    let mut i = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_mut().unwrap()) < 2 {
        // TODO: Unhandled statement type: SelectStmt
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}