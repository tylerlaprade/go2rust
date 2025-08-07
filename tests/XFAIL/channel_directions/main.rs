use std::sync::{Arc, Mutex};

pub fn ping(pings: Arc<Mutex<Option</* TODO: Unhandled type *ast.ChanType */ Arc<Mutex<Option<()>>>>>>, msg: Arc<Mutex<Option<String>>>) {
    // TODO: Unhandled statement type: SendStmt
}

pub fn pong(pings: Arc<Mutex<Option</* TODO: Unhandled type *ast.ChanType */ Arc<Mutex<Option<()>>>>>>, pongs: Arc<Mutex<Option</* TODO: Unhandled type *ast.ChanType */ Arc<Mutex<Option<()>>>>>>) {
    let mut msg = Arc::new(Mutex::new(Some(<-(*pings.lock().unwrap().as_mut().unwrap()))));
    // TODO: Unhandled statement type: SendStmt
}

fn main() {
    let mut pings = ;
    let mut pongs = ;
    ping(pings.clone(), Arc::new(Mutex::new(Some("passed message".to_string()))));
    pong(pings.clone(), pongs.clone());
    println!("{}", <-(*pongs.lock().unwrap().as_mut().unwrap()));
}