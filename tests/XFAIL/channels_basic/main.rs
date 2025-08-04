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

pub fn sender(ch: Arc<Mutex<Option</* TODO: Unhandled type *ast.ChanType */ Arc<Mutex<Option<()>>>>>>) {
    let mut i = Arc::new(Mutex::new(Some(1)));
    while (*i.lock().unwrap().as_mut().unwrap()) <= 5 {
        print!("Sending: {}\n", (*i.lock().unwrap().as_mut().unwrap()));
        // TODO: Unhandled statement type: SendStmt
        (*time.lock().unwrap().as_mut().unwrap()).sleep(Arc::new(Mutex::new(Some(100 * (*time.lock().unwrap().as_mut().unwrap()).millisecond))));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    (close.lock().unwrap().as_ref().unwrap())(ch.clone());
}

pub fn receiver(ch: Arc<Mutex<Option</* TODO: Unhandled type *ast.ChanType */ Arc<Mutex<Option<()>>>>>>) {
    while true {
        let (mut value, mut ok) = <-(*ch.lock().unwrap().as_mut().unwrap());
        if !(*ok.lock().unwrap().as_mut().unwrap()) {
        println!("{}", "Channel closed".to_string());
        break
    }
        print!("Received: {}\n", (*value.lock().unwrap().as_mut().unwrap()));
    }
}

fn main() {
    let mut ch = ;

    // TODO: Unhandled statement type: GoStmt
    // TODO: Unhandled statement type: GoStmt

    (*time.lock().unwrap().as_mut().unwrap()).sleep(Arc::new(Mutex::new(Some(1 * (*time.lock().unwrap().as_mut().unwrap()).second))));

    let mut buffered = ;
    // TODO: Unhandled statement type: SendStmt
    // TODO: Unhandled statement type: SendStmt
    // TODO: Unhandled statement type: SendStmt

    println!("{}", "Buffered channel contents:".to_string());
    let mut i = Arc::new(Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_mut().unwrap()) < 3 {
        let mut msg = Arc::new(Mutex::new(Some(<-(*buffered.lock().unwrap().as_mut().unwrap()))));
        println!("{} {}", "Got:".to_string(), (*msg.lock().unwrap().as_mut().unwrap()));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    let mut numbers = ;
    // TODO: Unhandled statement type: GoStmt

    println!("{}", "Range over channel:".to_string());
    for num in 0..(*numbers.lock().unwrap().as_mut().unwrap()).len() {
        println!("{} {}", "Number:".to_string(), num);
    }
}