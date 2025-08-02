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

#[derive(Debug)]
struct User {
    i_d: std::sync::Arc<std::sync::Mutex<Option<i32>>>,
    name: std::sync::Arc<std::sync::Mutex<Option<String>>>,
    email: std::sync::Arc<std::sync::Mutex<Option<String>>>,
    is_active: std::sync::Arc<std::sync::Mutex<Option<bool>>>,
    internal: std::sync::Arc<std::sync::Mutex<Option<String>>>,
}

fn main() {
    let mut u = User { i_d: std::sync::Arc::new(std::sync::Mutex::new(Some(1))), name: std::sync::Arc::new(std::sync::Mutex::new(Some("Alice".to_string()))), email: std::sync::Arc::new(std::sync::Mutex::new(Some("alice@example.com".to_string()))) };
    let mut t = (*reflect.lock().unwrap().as_mut().unwrap()).type_of(std::sync::Arc::new(std::sync::Mutex::new(Some((*u.lock().unwrap().as_mut().unwrap())))));

    let mut i = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_mut().unwrap()) < (*t.lock().unwrap().as_mut().unwrap()).num_field() {
        let mut field = (*t.lock().unwrap().as_mut().unwrap()).field(std::sync::Arc::new(std::sync::Mutex::new(Some((*i.lock().unwrap().as_mut().unwrap())))));
        print!("{}: json=%q db=%q\n", (*field.lock().unwrap().as_mut().unwrap()).name, (*(*field.lock().unwrap().as_mut().unwrap()).tag.get(std::sync::Arc::new(std::sync::Mutex::new(Some("json".to_string())))).lock().unwrap().as_mut().unwrap()), (*(*field.lock().unwrap().as_mut().unwrap()).tag.get(std::sync::Arc::new(std::sync::Mutex::new(Some("db".to_string())))).lock().unwrap().as_mut().unwrap()));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}