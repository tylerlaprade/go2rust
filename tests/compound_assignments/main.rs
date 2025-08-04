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
    let mut x = Arc::new(Mutex::new(Some(10)));
    { let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 5); };
    print!("x += 5: {}\n", (*x.lock().unwrap().as_mut().unwrap()));

    { let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 3); };
    print!("x -= 3: {}\n", (*x.lock().unwrap().as_mut().unwrap()));

    { let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() * 2); };
    print!("x *= 2: {}\n", (*x.lock().unwrap().as_mut().unwrap()));

    { let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() / 4); };
    print!("x /= 4: {}\n", (*x.lock().unwrap().as_mut().unwrap()));

    { let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() % 5); };
    print!("x %= 5: {}\n", (*x.lock().unwrap().as_mut().unwrap()));

    let mut y = Arc::new(Mutex::new(Some(0b1010)));
    { let mut guard = y.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() & 0b1100); };
    print!("y &= 0b1100: {:b}\n", (*y.lock().unwrap().as_mut().unwrap()));

    { let mut guard = y.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | 0b0011); };
    print!("y |= 0b0011: {:b}\n", (*y.lock().unwrap().as_mut().unwrap()));

    { let mut guard = y.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() ^ 0b0101); };
    print!("y ^= 0b0101: {:b}\n", (*y.lock().unwrap().as_mut().unwrap()));

    { let mut guard = y.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() << 2); };
    print!("y <<= 2: {:b}\n", (*y.lock().unwrap().as_mut().unwrap()));

    { let mut guard = y.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() >> 1); };
    print!("y >>= 1: {:b}\n", (*y.lock().unwrap().as_mut().unwrap()));

    let mut f = Arc::new(Mutex::new(Some(3.14)));
    { let mut guard = f.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 2.86); };
    print!("f += 2.86: {:.2}\n", (*f.lock().unwrap().as_mut().unwrap()));

    { let mut guard = f.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() * 2.0); };
    print!("f *= 2.0: {:.2}\n", (*f.lock().unwrap().as_mut().unwrap()));

    let mut s = Arc::new(Mutex::new(Some("Hello".to_string())));
    (*s.lock().unwrap().as_mut().unwrap()).push_str(&" World".to_string());
    print!("s += \" World\": {}\n", (*s.lock().unwrap().as_mut().unwrap()));
}