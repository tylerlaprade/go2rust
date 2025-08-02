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

pub fn sender(ch: std::sync::Arc<std::sync::Mutex<Option<Unknown>>>) {
    let mut i = std::sync::Arc::new(std::sync::Mutex::new(Some(1)));
    while (*i.lock().unwrap().as_mut().unwrap()) <= 5 {
        print!("Sending: {}\n", (*i.lock().unwrap().as_mut().unwrap()));
        // TODO: Unhandled statement type: SendStmt
        (*time.lock().unwrap().as_mut().unwrap()).sleep(std::sync::Arc::new(std::sync::Mutex::new(Some(100 * (*time.lock().unwrap().as_mut().unwrap()).millisecond))));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    close(ch.clone());
}

pub fn receiver(ch: std::sync::Arc<std::sync::Mutex<Option<Unknown>>>) {
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
    (*time.lock().unwrap().as_mut().unwrap()).sleep(std::sync::Arc::new(std::sync::Mutex::new(Some(1 * (*time.lock().unwrap().as_mut().unwrap()).second))));
    let mut buffered = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![std::sync::Arc::new(std::sync::Mutex::new(Some(0))); 3])));
    // TODO: Unhandled statement type: SendStmt
    // TODO: Unhandled statement type: SendStmt
    // TODO: Unhandled statement type: SendStmt
    println!("{}", "Buffered channel contents:".to_string());
    let mut i = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_mut().unwrap()) < 3 {
        let mut msg = std::sync::Arc::new(std::sync::Mutex::new(Some(<-(*buffered.lock().unwrap().as_mut().unwrap()))));
        println!("{} {}", "Got:".to_string(), (*msg.lock().unwrap().as_mut().unwrap()));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    let mut numbers = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![std::sync::Arc::new(std::sync::Mutex::new(Some(0))); 5])));
    // TODO: Unhandled statement type: GoStmt
    println!("{}", "Range over channel:".to_string());
    for num in 0..(*numbers.lock().unwrap().as_mut().unwrap()).len() {
        println!("{} {}", "Number:".to_string(), num);
    }
}