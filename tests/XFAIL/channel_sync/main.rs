use std::sync::{Arc, Mutex};

pub fn worker(done: Arc<Mutex<Option</* TODO: Unhandled type *ast.ChanType */ Arc<Mutex<Option<()>>>>>>) {
    fmt.print(Arc::new(Mutex::new(Some("working...".to_string()))));
    time.sleep(Arc::new(Mutex::new(Some(500 * (*(*time.lock().unwrap().as_mut().unwrap())::millisecond.lock().unwrap().as_ref().unwrap())))));
    println!("{}", "done".to_string());

    // TODO: Unhandled statement type: SendStmt
}

fn main() {
    let mut done = ;
    // TODO: Unhandled statement type: GoStmt

    <-(*done.lock().unwrap().as_mut().unwrap());
}