const STATE_IDLE: ServerState = 0;
const STATE_CONNECTED: i32 = 1;
const STATE_ERROR: i32 = 2;
const STATE_RETRYING: i32 = 3;




impl ServerState {
    pub fn string(&self) -> std::sync::Arc<std::sync::Mutex<Option<String>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*stateName.lock().unwrap().as_mut().unwrap())[self])));
    }
}

fn main() {
    let mut ns = transition(std::sync::Arc::new(std::sync::Mutex::new(Some(STATE_IDLE))));
    println!("{}", (*ns.lock().unwrap().as_mut().unwrap()));
    let mut ns2 = transition(std::sync::Arc::new(std::sync::Mutex::new(Some((*ns.lock().unwrap().as_mut().unwrap())))));
    println!("{}", (*ns2.lock().unwrap().as_mut().unwrap()));
}

pub fn transition(s: std::sync::Arc<std::sync::Mutex<Option<ServerState>>>) -> std::sync::Arc<std::sync::Mutex<Option<ServerState>>> {

    match (*s.lock().unwrap().as_mut().unwrap()) {
        STATE_IDLE => {
            return std::sync::Arc::new(std::sync::Mutex::new(Some((*StateConnected.lock().unwrap().as_mut().unwrap()).clone())));
        }
        STATE_CONNECTED | STATE_RETRYING => {
            return std::sync::Arc::new(std::sync::Mutex::new(Some((*StateIdle.lock().unwrap().as_mut().unwrap()).clone())));
        }
        STATE_ERROR => {
            return std::sync::Arc::new(std::sync::Mutex::new(Some((*StateError.lock().unwrap().as_mut().unwrap()).clone())));
        }
        _ => {
            panic(std::sync::Arc::new(std::sync::Mutex::new(Some(std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(format!("unknown state: {}", (*s.lock().unwrap().as_mut().unwrap()))) as Box<dyn std::error::Error + Send + Sync>)))))));
        }
    }
}