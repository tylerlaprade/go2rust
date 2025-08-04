use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::error::Error;
use std::any::Any;
use std::cmp::Ord;

fn format_map<K: Display + Ord + Clone, V>(map: &Arc<Mutex<Option<HashMap<K, Arc<Mutex<Option<V>>>>>>>) -> String 
where
    V: Display,
{
    let guard = map.lock().unwrap();
    if let Some(ref m) = *guard {
        let mut items: Vec<_> = m.iter().collect();
        items.sort_by_key(|(k, _)| (*k).clone());
        
        let formatted: Vec<String> = items
            .into_iter()
            .map(|(k, v)| {
                let v_guard = v.lock().unwrap();
                if let Some(ref val) = *v_guard {
                    format!("{}:{}", k, val)
                } else {
                    format!("{}:<nil>", k)
                }
            })
            .collect();
        
        format!("map[{}]", formatted.join(" "))
    } else {
        "map[]".to_string()
    }
}
fn format_slice<T>(slice: &Arc<Mutex<Option<Vec<T>>>>) -> String 
where
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

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