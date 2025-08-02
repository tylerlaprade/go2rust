fn format_map<K: std::fmt::Display + std::cmp::Ord + Clone, V>(map: &std::sync::Arc<std::sync::Mutex<Option<std::collections::HashMap<K, std::sync::Arc<std::sync::Mutex<Option<V>>>>>>>) -> String 
where
    V: std::fmt::Display,
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
fn format_slice<T>(slice: &std::sync::Arc<std::sync::Mutex<Option<Vec<T>>>>) -> String 
where
    T: std::fmt::Display,
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
    num: std::sync::Arc<std::sync::Mutex<Option<i32>>>,
}

#[derive(Debug)]
struct container {
    std::sync::_arc<std::sync::_mutex<_option<base>>>: std::sync::Arc<std::sync::Mutex<Option<base>>>,
    str: std::sync::Arc<std::sync::Mutex<Option<String>>>,
}

impl base {
    pub fn describe(&self) -> std::sync::Arc<std::sync::Mutex<Option<String>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some(format!("base with num={}", (*self.num.lock().unwrap().as_mut().unwrap())))));
    }
}

fn main() {
    let mut co = container { base: std::sync::Arc::new(std::sync::Mutex::new(Some(base { num: std::sync::Arc::new(std::sync::Mutex::new(Some(1))) }))), str: std::sync::Arc::new(std::sync::Mutex::new(Some("some name".to_string()))) };
    print!("co={num: {}, str: {}}\n", (*co.lock().unwrap().as_mut().unwrap()).num, (*co.lock().unwrap().as_mut().unwrap()).str);
    println!("{} {}", "also num:".to_string(), (*co.lock().unwrap().as_mut().unwrap()).base::num);
    println!("{} {}", "describe:".to_string(), (*(*co.lock().unwrap().as_mut().unwrap()).describe().lock().unwrap().as_mut().unwrap()));
    
    let mut d: std::sync::Arc<std::sync::Mutex<Option<describer>>> = std::sync::Arc::new(std::sync::Mutex::new(Some((*co.lock().unwrap().as_mut().unwrap()))));
    println!("{} {}", "describer:".to_string(), (*(*d.lock().unwrap().as_mut().unwrap()).describe().lock().unwrap().as_mut().unwrap()));
}