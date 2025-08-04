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

pub fn zeroval(ival: Arc<Mutex<Option<i32>>>) {
    { let new_val = 0; *ival.lock().unwrap() = Some(new_val); };
}

pub fn zeroptr(iptr: Arc<Mutex<Option<i32>>>) {
    { let new_val = 0; *iptr.lock().unwrap() = Some(new_val); };
}

fn main() {
    let mut i = Arc::new(Mutex::new(Some(1)));
    println!("{} {}", "initial:".to_string(), (*i.lock().unwrap().as_mut().unwrap()));

    zeroval(i.clone());
    println!("{} {}", "zeroval:".to_string(), (*i.lock().unwrap().as_mut().unwrap()));

    zeroptr(Arc::new(Mutex::new(Some(i.clone()))));
    println!("{} {}", "zeroptr:".to_string(), (*i.lock().unwrap().as_mut().unwrap()));

    println!("{} {}", "pointer:".to_string(), i.clone());
}