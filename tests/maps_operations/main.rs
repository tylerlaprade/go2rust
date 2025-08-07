use std::cmp::Ord;
use std::collections::HashMap;
use std::fmt::{Display};
use std::sync::{Arc, Mutex};

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

fn main() {
    let mut m = Arc::new(Mutex::new(Some(HashMap::<String, Arc<Mutex<Option<i32>>>>::new())));
    (*m.lock().unwrap().as_mut().unwrap()).insert("k1".to_string(), Arc::new(Mutex::new(Some(7))));
    (*m.lock().unwrap().as_mut().unwrap()).insert("k2".to_string(), Arc::new(Mutex::new(Some(13))));
    println!("{} {}", "map:".to_string(), format_map(&m));

    let mut v1 = Arc::new(Mutex::new(Some((*(*m.lock().unwrap().as_ref().unwrap()).get(&"k1".to_string()).unwrap().lock().unwrap().as_ref().unwrap()))));
    println!("{} {}", "v1:".to_string(), (*v1.lock().unwrap().as_mut().unwrap()));

    (*m.lock().unwrap().as_mut().unwrap()).remove(&"k2".to_string());
    println!("{} {}", "map:".to_string(), format_map(&m));

    let (_, mut prs) = match (*m.lock().unwrap().as_ref().unwrap()).get(&"k2".to_string()) { Some(v) => (v.clone(), Arc::new(Mutex::new(Some(true)))), None => (Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(false)))) };
    println!("{} {}", "prs:".to_string(), (*prs.lock().unwrap().as_mut().unwrap()));
}