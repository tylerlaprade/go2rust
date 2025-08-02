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
    let mut x = std::sync::Arc::new(std::sync::Mutex::new(Some(10)));
    { let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 5); };
    print!("x += 5: {}\n", (*x.lock().unwrap().as_mut().unwrap()));
    { let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 3); };
    print!("x -= 3: {}\n", (*x.lock().unwrap().as_mut().unwrap()));
    { let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() * 2); };
    print!("x *= 2: {}\n", (*x.lock().unwrap().as_mut().unwrap()));
    { let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() / 4); };
    print!("x /= 4: {}\n", (*x.lock().unwrap().as_mut().unwrap()));
    { let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() % 5); };
    print!("x %%= 5: {}\n", (*x.lock().unwrap().as_mut().unwrap()));
    let mut y = std::sync::Arc::new(std::sync::Mutex::new(Some(0b1010)));
    y = 0b1100;
    print!("y &= 0b1100: %b\n", (*y.lock().unwrap().as_mut().unwrap()));
    y = 0b0011;
    print!("y |= 0b0011: %b\n", (*y.lock().unwrap().as_mut().unwrap()));
    y = 0b0101;
    print!("y ^= 0b0101: %b\n", (*y.lock().unwrap().as_mut().unwrap()));
    y = 2;
    print!("y <<= 2: %b\n", (*y.lock().unwrap().as_mut().unwrap()));
    y = 1;
    print!("y >>= 1: %b\n", (*y.lock().unwrap().as_mut().unwrap()));
    let mut f = std::sync::Arc::new(std::sync::Mutex::new(Some(3.14)));
    { let mut guard = f.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 2.86); };
    print!("f += 2.86: {:.2}\n", (*f.lock().unwrap().as_mut().unwrap()));
    { let mut guard = f.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() * 2.0); };
    print!("f *= 2.0: {:.2}\n", (*f.lock().unwrap().as_mut().unwrap()));
    let mut s = std::sync::Arc::new(std::sync::Mutex::new(Some("Hello".to_string())));
    (*s.lock().unwrap().as_mut().unwrap()).push_str(&" World".to_string());
    print!("s += \" World\": {}\n", (*s.lock().unwrap().as_mut().unwrap()));
}