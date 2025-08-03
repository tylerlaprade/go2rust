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

pub fn vals() -> (std::sync::Arc<std::sync::Mutex<Option<i32>>>, std::sync::Arc<std::sync::Mutex<Option<i32>>>) {

    return (std::sync::Arc::new(std::sync::Mutex::new(Some(3))), std::sync::Arc::new(std::sync::Mutex::new(Some(7))));
}

fn main() {
    let (mut a, mut b) = (vals.lock().unwrap().as_ref().unwrap())();
    println!("{}", (*a.lock().unwrap().as_mut().unwrap()));
    println!("{}", (*b.lock().unwrap().as_mut().unwrap()));

    let (_, mut c) = (vals.lock().unwrap().as_ref().unwrap())();
    println!("{}", (*c.lock().unwrap().as_mut().unwrap()));
}