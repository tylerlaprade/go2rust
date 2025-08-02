pub fn ping(pings: std::sync::Arc<std::sync::Mutex<Option<Unknown>>>, msg: std::sync::Arc<std::sync::Mutex<Option<String>>>) {
    // TODO: Unhandled statement type: SendStmt
}

pub fn pong(pings: std::sync::Arc<std::sync::Mutex<Option<Unknown>>>, pongs: std::sync::Arc<std::sync::Mutex<Option<Unknown>>>) {
    let mut msg = std::sync::Arc::new(std::sync::Mutex::new(Some(<-(*pings.lock().unwrap().as_mut().unwrap()))));
    // TODO: Unhandled statement type: SendStmt
}

fn main() {
    let mut pings = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![std::sync::Arc::new(std::sync::Mutex::new(Some(0))); 1])));
    let mut pongs = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![std::sync::Arc::new(std::sync::Mutex::new(Some(0))); 1])));
    ping(pings.clone(), std::sync::Arc::new(std::sync::Mutex::new(Some("passed message".to_string()))));
    pong(pings.clone(), pongs.clone());
    println!("{}", <-(*pongs.lock().unwrap().as_mut().unwrap()));
}