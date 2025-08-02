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
    let mut i = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));

    // TODO: Unhandled statement type: LabeledStmt

    println!("{}", "First loop done".to_string());

    let mut x = std::sync::Arc::new(std::sync::Mutex::new(Some(1)));
    if (*x.lock().unwrap().as_mut().unwrap()) > 0 {
        // TODO: goto not supported
    }
    println!("{}", "This won't print".to_string());

    // TODO: Unhandled statement type: LabeledStmt

    let mut j = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    while (*j.lock().unwrap().as_mut().unwrap()) < 3 {
        let mut k = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    while (*k.lock().unwrap().as_mut().unwrap()) < 3 {
        if (*j.lock().unwrap().as_mut().unwrap()) == 1 && (*k.lock().unwrap().as_mut().unwrap()) == 1 {
        // TODO: goto not supported
    }
        print!("j={}, k={}\n", (*j.lock().unwrap().as_mut().unwrap()), (*k.lock().unwrap().as_mut().unwrap()));
        { let mut guard = k.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    // TODO: Unhandled statement type: LabeledStmt
}