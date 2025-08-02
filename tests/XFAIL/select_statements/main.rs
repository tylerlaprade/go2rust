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

pub fn basic_select() {
    let mut ch1 = ;
    let mut ch2 = ;
    // TODO: Unhandled statement type: GoStmt
    // TODO: Unhandled statement type: GoStmt
    // TODO: Unhandled statement type: SelectStmt
}

pub fn select_with_timeout() {
    let mut ch = ;
    // TODO: Unhandled statement type: GoStmt
    // TODO: Unhandled statement type: SelectStmt
}

pub fn select_with_default() {
    let mut ch = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![std::sync::Arc::new(std::sync::Mutex::new(Some(0))); 1])));
    // TODO: Unhandled statement type: SelectStmt
    // TODO: Unhandled statement type: SelectStmt
    // TODO: Unhandled statement type: SelectStmt
}

pub fn select_loop() {
    let mut ch1 = ;
    let mut ch2 = ;
    let mut quit = ;
    // TODO: Unhandled statement type: GoStmt
    // TODO: Unhandled statement type: GoStmt
    // TODO: Unhandled statement type: GoStmt
    println!("{}", "Starting select loop:".to_string());
    while true {
        // TODO: Unhandled statement type: SelectStmt
    }
}

pub fn select_with_send() {
    let mut ch1 = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![std::sync::Arc::new(std::sync::Mutex::new(Some(0))); 1])));
    let mut ch2 = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![std::sync::Arc::new(std::sync::Mutex::new(Some(0))); 1])));
    // TODO: Unhandled statement type: SelectStmt
    println!("{} {}", "Reading from ch1:".to_string(), <-(*ch1.lock().unwrap().as_mut().unwrap()));
    // TODO: Unhandled statement type: SelectStmt
}

fn main() {
    println!("{}", "=== Basic select ===".to_string());
    basic_select();
    println!("{}", "\n=== Select with timeout ===".to_string());
    select_with_timeout();
    println!("{}", "\n=== Select with default ===".to_string());
    select_with_default();
    println!("{}", "\n=== Select with send ===".to_string());
    select_with_send();
    println!("{}", "\n=== Select loop ===".to_string());
    select_loop();
    println!("{}", "\n=== All examples completed ===".to_string());
}