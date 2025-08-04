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

pub fn say_hello(name: Arc<Mutex<Option<String>>>) {
    let mut i = Arc::new(Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_mut().unwrap()) < 3 {
        print!("Hello {}! ({})\n", (*name.lock().unwrap().as_mut().unwrap()), (*i.lock().unwrap().as_mut().unwrap()) + 1);
        (*time.lock().unwrap().as_mut().unwrap()).sleep(Arc::new(Mutex::new(Some(100 * (*time.lock().unwrap().as_mut().unwrap()).millisecond))));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

pub fn counter(start: Arc<Mutex<Option<i32>>>) {
    let mut i = Arc::new(Mutex::new(Some((*start.lock().unwrap().as_mut().unwrap()))));
    while (*i.lock().unwrap().as_mut().unwrap()) < (*start.lock().unwrap().as_mut().unwrap()) + 5 {
        print!("Count: {}\n", (*i.lock().unwrap().as_mut().unwrap()));
        (*time.lock().unwrap().as_mut().unwrap()).sleep(Arc::new(Mutex::new(Some(50 * (*time.lock().unwrap().as_mut().unwrap()).millisecond))));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

fn main() {
    println!("{}", "Starting goroutines...".to_string());

    // TODO: Unhandled statement type: GoStmt
    // TODO: Unhandled statement type: GoStmt
    // TODO: Unhandled statement type: GoStmt

    // TODO: Unhandled statement type: GoStmt

    (*time.lock().unwrap().as_mut().unwrap()).sleep(Arc::new(Mutex::new(Some(1 * (*time.lock().unwrap().as_mut().unwrap()).second))));
    println!("{}", "Main function ending".to_string());
}