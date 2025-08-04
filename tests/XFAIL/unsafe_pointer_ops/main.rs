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

#[derive(Debug)]
struct Point {
    x: Arc<Mutex<Option<int32>>>,
    y: Arc<Mutex<Option<int32>>>,
}

fn main() {
    let mut p = Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(10))), y: Arc::new(Mutex::new(Some(20))) })));

    let mut xPtr = (((*int32.lock().unwrap().as_mut().unwrap())).lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some((*unsafe.lock().unwrap().as_mut().unwrap()).pointer(Arc::new(Mutex::new(Some((*p.lock().unwrap().as_mut().unwrap())))))))));
    print!("X via unsafe: {}\n", (*xPtr.lock().unwrap().as_mut().unwrap()));

    let mut yPtr = (((*int32.lock().unwrap().as_mut().unwrap())).lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some((*unsafe.lock().unwrap().as_mut().unwrap()).pointer(Arc::new(Mutex::new(Some((uintptr.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some((*unsafe.lock().unwrap().as_mut().unwrap()).pointer(Arc::new(Mutex::new(Some((*p.lock().unwrap().as_mut().unwrap()))))))))) + (*unsafe.lock().unwrap().as_mut().unwrap()).offsetof(Arc::new(Mutex::new(Some((*p.lock().unwrap().as_mut().unwrap()).y))))))))))));
    print!("Y via unsafe: {}\n", (*yPtr.lock().unwrap().as_mut().unwrap()));

    print!("Size: {}, Align: {}\n", (*(*unsafe.lock().unwrap().as_mut().unwrap()).sizeof(Arc::new(Mutex::new(Some((*p.lock().unwrap().as_mut().unwrap()))))).lock().unwrap().as_mut().unwrap()), (*(*unsafe.lock().unwrap().as_mut().unwrap()).alignof(Arc::new(Mutex::new(Some((*p.lock().unwrap().as_mut().unwrap()))))).lock().unwrap().as_mut().unwrap()));
}