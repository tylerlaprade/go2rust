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
struct User {
    i_d: Arc<Mutex<Option<i32>>>,
    name: Arc<Mutex<Option<String>>>,
    email: Arc<Mutex<Option<String>>>,
    is_active: Arc<Mutex<Option<bool>>>,
    internal: Arc<Mutex<Option<String>>>,
}

fn main() {
    let mut u = User { i_d: Arc::new(Mutex::new(Some(1))), name: Arc::new(Mutex::new(Some("Alice".to_string()))), email: Arc::new(Mutex::new(Some("alice@example.com".to_string()))) };
    let mut t = (*reflect.lock().unwrap().as_mut().unwrap()).type_of(Arc::new(Mutex::new(Some((*u.lock().unwrap().as_mut().unwrap())))));

    let mut i = Arc::new(Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_mut().unwrap()) < (*t.lock().unwrap().as_mut().unwrap()).num_field() {
        let mut field = (*t.lock().unwrap().as_mut().unwrap()).field(Arc::new(Mutex::new(Some((*i.lock().unwrap().as_mut().unwrap())))));
        print!("{}: json=%q db=%q\n", (*field.lock().unwrap().as_mut().unwrap()).name, (*(*field.lock().unwrap().as_mut().unwrap()).tag.get(Arc::new(Mutex::new(Some("json".to_string())))).lock().unwrap().as_mut().unwrap()), (*(*field.lock().unwrap().as_mut().unwrap()).tag.get(Arc::new(Mutex::new(Some("db".to_string())))).lock().unwrap().as_mut().unwrap()));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}