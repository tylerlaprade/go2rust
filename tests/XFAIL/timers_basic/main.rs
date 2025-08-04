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
    let mut timer1 = (*time.lock().unwrap().as_mut().unwrap()).new_timer(Arc::new(Mutex::new(Some(2 * (*time.lock().unwrap().as_mut().unwrap())::second))));

    <-(*timer1.lock().unwrap().as_mut().unwrap()).c;
    println!("{}", "Timer 1 fired".to_string());

    let mut timer2 = (*time.lock().unwrap().as_mut().unwrap()).new_timer(Arc::new(Mutex::new(Some((*time.lock().unwrap().as_mut().unwrap())::second))));
    // TODO: Unhandled statement type: GoStmt
    let mut stop2 = (*timer2.lock().unwrap().as_mut().unwrap()).stop();
    if (*stop2.lock().unwrap().as_mut().unwrap()) {
        println!("{}", "Timer 2 stopped".to_string());
    }

    (*time.lock().unwrap().as_mut().unwrap()).sleep(Arc::new(Mutex::new(Some(2 * (*time.lock().unwrap().as_mut().unwrap())::second))));
}