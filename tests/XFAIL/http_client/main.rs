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
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let (mut resp, mut err) = (*http.lock().unwrap().as_mut().unwrap()).get(std::sync::Arc::new(std::sync::Mutex::new(Some("https://httpbin.org/json".to_string()))));
    if (*err.lock().unwrap()).is_some() {
        println!("{} {}", "Error:".to_string(), (*err.lock().unwrap().as_mut().unwrap()));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return
    }
    __defer_stack.push(Box::new(move || {
        (*resp.lock().unwrap().as_mut().unwrap()).body.close();
    }));

    let (mut body, _) = (*io.lock().unwrap().as_mut().unwrap()).read_all(std::sync::Arc::new(std::sync::Mutex::new(Some((*resp.lock().unwrap().as_mut().unwrap()).body))));
    println!("{} {}", "Response:".to_string(), (string.lock().unwrap().as_ref().unwrap())(body.clone())[..100].to_vec());

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}