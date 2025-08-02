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

fn main() {
    let mut ages = std::sync::Arc::new(std::sync::Mutex::new(Some(std::collections::HashMap::<String, std::sync::Arc<std::sync::Mutex<Option<i32>>>>::new())));
    (*ages.lock().unwrap().as_mut().unwrap()).insert("Alice".to_string(), std::sync::Arc::new(std::sync::Mutex::new(Some(25))));
    (*ages.lock().unwrap().as_mut().unwrap()).insert("Bob".to_string(), std::sync::Arc::new(std::sync::Mutex::new(Some(30))));
    (*ages.lock().unwrap().as_mut().unwrap()).insert("Charlie".to_string(), std::sync::Arc::new(std::sync::Mutex::new(Some(35))));
    println!("{} {}", "Ages map:".to_string(), format_map(&ages));
    let mut colors = std::sync::Arc::new(std::sync::Mutex::new(Some(std::collections::HashMap::<String, std::sync::Arc<std::sync::Mutex<Option<String>>>>::from([("red".to_string(), std::sync::Arc::new(std::sync::Mutex::new(Some("#FF0000".to_string())))), ("green".to_string(), std::sync::Arc::new(std::sync::Mutex::new(Some("#00FF00".to_string())))), ("blue".to_string(), std::sync::Arc::new(std::sync::Mutex::new(Some("#0000FF".to_string()))))]))));
    println!("{} {}", "Colors map:".to_string(), format_map(&colors));
    let (mut age, mut exists) = match (*ages.lock().unwrap().as_ref().unwrap()).get(&"Alice".to_string()) { Some(v) => (v.clone(), std::sync::Arc::new(std::sync::Mutex::new(Some(true)))), None => (std::sync::Arc::new(std::sync::Mutex::new(Some(0))), std::sync::Arc::new(std::sync::Mutex::new(Some(false)))) };
    if (*exists.lock().unwrap().as_mut().unwrap()) {
        println!("{} {}", "Alice's age:".to_string(), (*age.lock().unwrap().as_mut().unwrap()));
    }
    (*ages.lock().unwrap().as_mut().unwrap()).remove(&"Bob".to_string());
    println!("{} {}", "After deleting Bob:".to_string(), format_map(&ages));
    println!("{}", "All colors:".to_string());
    for (name, hex) in (*colors.lock().unwrap().as_ref().unwrap()).clone() {
        println!("{} {} {}", name, "->".to_string(), (*hex.lock().unwrap().as_mut().unwrap()));
    }
}