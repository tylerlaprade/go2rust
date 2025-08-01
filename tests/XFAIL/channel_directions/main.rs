pub fn ping(pings: std::sync::Arc<std::sync::Mutex<Option<Unknown>>>, msg: std::sync::Arc<std::sync::Mutex<Option<String>>>) {
    // TODO: Unhandled statement type: SendStmt
}

pub fn pong(pings: std::sync::Arc<std::sync::Mutex<Option<Unknown>>>, pongs: std::sync::Arc<std::sync::Mutex<Option<Unknown>>>) {
    let mut msg = std::sync::Arc::new(std::sync::Mutex::new(Some(<-(*pings.lock().unwrap().as_mut().unwrap()))));
    // TODO: Unhandled statement type: SendStmt
}

fn main() {
    let mut pings = vec![0; 1];
    let mut pongs = vec![0; 1];
    ping(std::sync::Arc::new(std::sync::Mutex::new(Some((*pings.lock().unwrap().as_mut().unwrap())))), std::sync::Arc::new(std::sync::Mutex::new(Some("passed message".to_string()))));
    pong(std::sync::Arc::new(std::sync::Mutex::new(Some((*pings.lock().unwrap().as_mut().unwrap())))), std::sync::Arc::new(std::sync::Mutex::new(Some((*pongs.lock().unwrap().as_mut().unwrap())))));
    println!("{}", <-(*pongs.lock().unwrap().as_mut().unwrap()));
}