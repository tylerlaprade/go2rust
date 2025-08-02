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
    let mut m = std::sync::Arc::new(std::sync::Mutex::new(Some(std::collections::HashMap::<String, std::sync::Arc<std::sync::Mutex<Option<i32>>>>::new())));
    (*m.lock().unwrap().as_mut().unwrap()).insert("k1".to_string(), std::sync::Arc::new(std::sync::Mutex::new(Some(7))));
    (*m.lock().unwrap().as_mut().unwrap()).insert("k2".to_string(), std::sync::Arc::new(std::sync::Mutex::new(Some(13))));
    println!("{} {}", "map:".to_string(), format_map(&m));

    let mut v1 = std::sync::Arc::new(std::sync::Mutex::new(Some((*(*m.lock().unwrap().as_ref().unwrap()).get(&"k1".to_string()).unwrap().lock().unwrap().as_ref().unwrap()))));
    println!("{} {}", "v1:".to_string(), (*v1.lock().unwrap().as_mut().unwrap()));

    (*m.lock().unwrap().as_mut().unwrap()).remove(&"k2".to_string());
    println!("{} {}", "map:".to_string(), format_map(&m));

    let (_, mut prs) = match (*m.lock().unwrap().as_ref().unwrap()).get(&"k2".to_string()) { Some(v) => (v.clone(), std::sync::Arc::new(std::sync::Mutex::new(Some(true)))), None => (std::sync::Arc::new(std::sync::Mutex::new(Some(0))), std::sync::Arc::new(std::sync::Mutex::new(Some(false)))) };
    println!("{} {}", "prs:".to_string(), (*prs.lock().unwrap().as_mut().unwrap()));
}