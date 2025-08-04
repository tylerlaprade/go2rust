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
struct base {
    num: Arc<Mutex<Option<i32>>>,
}

#[derive(Debug)]
struct container {
    arc<_mutex<_option<base>>>: Arc<Mutex<Option<base>>>,
    str: Arc<Mutex<Option<String>>>,
}

impl base {
    pub fn describe(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("base with num={}", (*self.num.lock().unwrap().as_mut().unwrap())))));
    }
}

fn main() {
    let mut co = container { base: Arc::new(Mutex::new(Some(base { num: Arc::new(Mutex::new(Some(1))) }))), str: Arc::new(Mutex::new(Some("some name".to_string()))) };

    print!("co={num: {}, str: {}}\n", (*co.lock().unwrap().as_mut().unwrap()).num, (*co.lock().unwrap().as_mut().unwrap()).str);
    println!("{} {}", "also num:".to_string(), (*co.lock().unwrap().as_mut().unwrap()).base::num);
    println!("{} {}", "describe:".to_string(), (*(*co.lock().unwrap().as_mut().unwrap()).describe().lock().unwrap().as_mut().unwrap()));

    

    let mut d: Arc<Mutex<Option<describer>>> = Arc::new(Mutex::new(Some((*co.lock().unwrap().as_mut().unwrap()))));
    println!("{} {}", "describer:".to_string(), (*(*d.lock().unwrap().as_mut().unwrap()).describe().lock().unwrap().as_mut().unwrap()));
}