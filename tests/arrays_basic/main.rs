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
    let mut arr: Arc<Mutex<Option<[i32; 3]>>> = Arc::new(Mutex::new(Some(Default::default())));
    (*arr.lock().unwrap().as_mut().unwrap())[0] = 10;
    (*arr.lock().unwrap().as_mut().unwrap())[1] = 20;
    (*arr.lock().unwrap().as_mut().unwrap())[2] = 30;

    println!("{}", "Array elements:".to_string());
    let mut i = Arc::new(Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_mut().unwrap()) < (*arr.lock().unwrap().as_mut().unwrap()).len() {
        println!("{}", (*arr.lock().unwrap().as_mut().unwrap())[(*i.lock().unwrap().as_mut().unwrap())]);
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    let mut nums = Arc::new(Mutex::new(Some([1, 2, 3, 4])));
    println!("{}", "Initialized array:".to_string());
    for num in &(*nums.lock().unwrap().as_mut().unwrap()) {
        println!("{}", num);
    }
}