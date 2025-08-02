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
    let mut arr: std::sync::Arc<std::sync::Mutex<Option<[i32; 3]>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(Default::default())));
    (*arr.lock().unwrap().as_mut().unwrap())[0] = 10;
    (*arr.lock().unwrap().as_mut().unwrap())[1] = 20;
    (*arr.lock().unwrap().as_mut().unwrap())[2] = 30;
    println!("{}", "Array elements:".to_string());
    let mut i = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_mut().unwrap()) < (*arr.lock().unwrap().as_mut().unwrap()).len() {
        println!("{}", (*arr.lock().unwrap().as_mut().unwrap())[(*i.lock().unwrap().as_mut().unwrap())]);
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    let mut nums = std::sync::Arc::new(std::sync::Mutex::new(Some([1, 2, 3, 4])));
    println!("{}", "Initialized array:".to_string());
    for num in &(*nums.lock().unwrap().as_mut().unwrap()) {
        println!("{}", num);
    }
}