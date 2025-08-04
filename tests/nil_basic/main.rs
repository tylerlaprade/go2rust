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
    let mut p: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(None));
    if (*p.lock().unwrap()).is_none() {
        println!("{}", "p is nil".to_string());
    }

    let mut q: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
    if (*q.lock().unwrap()).is_none() {
        println!("{}", "q is nil".to_string());
    }

    let mut x = Arc::new(Mutex::new(Some(42)));
    { let new_val = (*x.lock().unwrap()).clone(); *p.lock().unwrap() = new_val; };
    if (*p.lock().unwrap()).is_some() {
        println!("{} {}", "p is not nil, value:".to_string(), (*p.lock().unwrap().as_mut().unwrap()));
    }

    *p.lock().unwrap() = None;
    if (*p.lock().unwrap()).is_none() {
        println!("{}", "p is nil again".to_string());
    }
}