use std::sync::{Arc, Mutex};

const STATE_IDLE: ServerState = 0;
const STATE_CONNECTED: i32 = 1;
const STATE_ERROR: i32 = 2;
const STATE_RETRYING: i32 = 3;


#[derive(Debug, Clone)]
struct ServerState(Arc<Mutex<Option<i32>>>);

impl Display for ServerState {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}


impl ServerState {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some((*(*stateName.lock().unwrap().as_ref().unwrap()).get(&(*self.0.lock().unwrap().as_ref().unwrap())).unwrap().lock().unwrap().as_ref().unwrap()))));
    }
}

fn main() {
    let mut ns = transition(StateIdle.clone());
    println!("{}", (*ns.lock().unwrap().as_mut().unwrap()));

    let mut ns2 = transition(ns.clone());
    println!("{}", (*ns2.lock().unwrap().as_mut().unwrap()));
}

pub fn transition(s: Arc<Mutex<Option<ServerState>>>) -> Arc<Mutex<Option<ServerState>>> {

    match (*s.lock().unwrap().as_mut().unwrap()) {
        STATE_IDLE => {
            return StateConnected.clone();
        }
        STATE_CONNECTED | STATE_RETRYING => {
            return StateIdle.clone();
        }
        STATE_ERROR => {
            return StateError.clone();
        }
        _ => {
            panic(Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some(Box::new(format!("unknown state: {}", (*s.lock().unwrap().as_mut().unwrap()))) as Box<dyn Error + Send + Sync>)))))));
        }
    }
}