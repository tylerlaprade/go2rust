use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::error::Error;
use std::any::Any;
use std::cmp::Ord;

fn format_map<K: Display + Ord + Clone, V>(map: &Arc<Mutex<Option<HashMap<K, Arc<Mutex<Option<V>>>>>>>) -> String 
where
    V: Display,
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
fn format_slice<T>(slice: &Arc<Mutex<Option<Vec<T>>>>) -> String 
where
    T: Display,
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
    let mut nums = Arc::new(Mutex::new(Some(vec![2, 3, 4])));
    let mut sum = Arc::new(Mutex::new(Some(0)));
    for num in &(*nums.lock().unwrap().as_mut().unwrap()) {
        { let mut guard = sum.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + num); };
    }
    println!("{} {}", "sum:".to_string(), (*sum.lock().unwrap().as_mut().unwrap()));

    for (i, num) in (*nums.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        if num == 3 {
        println!("{} {}", "index:".to_string(), i);
    }
    }

    let mut kvs = Arc::new(Mutex::new(Some(HashMap::<String, Arc<Mutex<Option<String>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some("apple".to_string())))), ("b".to_string(), Arc::new(Mutex::new(Some("banana".to_string()))))]))));
    for (k, v) in (*kvs.lock().unwrap().as_ref().unwrap()).clone() {
        print!("{} -> {}\n", k, (*v.lock().unwrap().as_mut().unwrap()));
    }
}