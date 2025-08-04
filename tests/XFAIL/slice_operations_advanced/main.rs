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
    let mut s = Arc::new(Mutex::new(Some(vec![0; 3])));
    print!("len={} cap={} {}\n", (*s.lock().unwrap().as_mut().unwrap()).len(), (*s.lock().unwrap().as_mut().unwrap()).capacity(), format_slice(&s));

    {(*s.lock().unwrap().as_mut().unwrap()).extend(vec![1, 2, 3, 4, 5, 6, 7, 8]); s.clone()};
    print!("len={} cap={} {}\n", (*s.lock().unwrap().as_mut().unwrap()).len(), (*s.lock().unwrap().as_mut().unwrap()).capacity(), format_slice(&s));

    let mut s2 = Arc::new(Mutex::new(Some((*s.lock().unwrap().as_mut().unwrap())[2..5].to_vec())));
    print!("s2: len={} cap={} {}\n", (*s2.lock().unwrap().as_mut().unwrap()).len(), (*s2.lock().unwrap().as_mut().unwrap()).capacity(), format_slice(&s2));

    let mut s3 = Arc::new(Mutex::new(Some(vec![0; 3])));
    let mut n = copy(s3.clone(), s.clone());
    print!("Copied {} elements: {}\n", (*n.lock().unwrap().as_mut().unwrap()), format_slice(&s3));

    let mut s4: Arc<Mutex<Option<Vec<i32>>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut s5 = Arc::new(Mutex::new(Some(vec![])));
    print!("s4==nil: {}, s5==nil: {}\n", (*s4.lock().unwrap()).is_none(), (*s5.lock().unwrap()).is_none());
}