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
    let mut slice = Arc::new(Mutex::new(Some(vec![1, 2, 3, 4, 5])));
    println!("{} {}", "Original slice:".to_string(), format_slice(&slice));

    {(*slice.lock().unwrap().as_mut().unwrap()).extend(vec![6, 7]); slice.clone()};
    println!("{} {}", "After append:".to_string(), format_slice(&slice));

    let mut subSlice = Arc::new(Mutex::new(Some((*slice.lock().unwrap().as_mut().unwrap())[1..4].to_vec())));
    println!("{} {}", "Sub-slice [1:4]:".to_string(), format_slice(&subSlice));

    println!("{} {}", "Length:".to_string(), (*slice.lock().unwrap().as_mut().unwrap()).len());
    println!("{} {}", "Capacity:".to_string(), (*slice.lock().unwrap().as_mut().unwrap()).capacity());

    let mut made = Arc::new(Mutex::new(Some(vec![0; 3])));
    (*made.lock().unwrap().as_mut().unwrap())[0] = 10;
    (*made.lock().unwrap().as_mut().unwrap())[1] = 20;
    (*made.lock().unwrap().as_mut().unwrap())[2] = 30;
    println!("{} {}", "Made slice:".to_string(), format_slice(&made));
}