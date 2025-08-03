fn format_map<K: std::fmt::Display + std::cmp::Ord + Clone, V>(map: &std::sync::Arc<std::sync::Mutex<Option<std::collections::HashMap<K, std::sync::Arc<std::sync::Mutex<Option<V>>>>>>>) -> String 
where
    V: std::fmt::Display,
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
fn format_slice<T>(slice: &std::sync::Arc<std::sync::Mutex<Option<Vec<T>>>>) -> String 
where
    T: std::fmt::Display,
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


// TODO: Unhandled type declaration: Ident
type ServerState = std::sync::Arc<std::sync::Mutex<Option<()>>>

impl ServerState {
    pub fn string(&self) -> std::sync::Arc<std::sync::Mutex<Option<String>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*(*stateName.lock().unwrap().as_ref().unwrap()).get(&self).unwrap().lock().unwrap().as_ref().unwrap()))));
    }
}

fn main() {
    let mut ns = (transition.lock().unwrap().as_ref().unwrap())(StateIdle.clone());
    println!("{}", (*ns.lock().unwrap().as_mut().unwrap()));

    let mut ns2 = (transition.lock().unwrap().as_ref().unwrap())(ns.clone());
    println!("{}", (*ns2.lock().unwrap().as_mut().unwrap()));
}

pub fn transition(s: std::sync::Arc<std::sync::Mutex<Option<ServerState>>>) -> std::sync::Arc<std::sync::Mutex<Option<ServerState>>> {

    match (*s.lock().unwrap().as_mut().unwrap()) {
        STATE_IDLE => {
            return std::sync::Arc::new(std::sync::Mutex::new(Some(STATE_CONNECTED)));
        }
        STATE_CONNECTED | STATE_RETRYING => {
            return std::sync::Arc::new(std::sync::Mutex::new(Some(STATE_IDLE)));
        }
        STATE_ERROR => {
            return std::sync::Arc::new(std::sync::Mutex::new(Some(STATE_ERROR)));
        }
        _ => {
            panic(std::sync::Arc::new(std::sync::Mutex::new(Some(std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(format!("unknown state: {}", (*s.lock().unwrap().as_mut().unwrap()))) as Box<dyn std::error::Error + Send + Sync>)))))));
        }
    }
}