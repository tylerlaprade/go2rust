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
struct rect {
    width: std::sync::Arc<std::sync::Mutex<Option<i32>>>,
    height: std::sync::Arc<std::sync::Mutex<Option<i32>>>,
}

impl rect {
    pub fn area(&mut self) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*self.width.clone().lock().unwrap().as_mut().unwrap()) * (*self.height.clone().lock().unwrap().as_mut().unwrap()))));
    }

    pub fn perim(&self) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*2.lock().unwrap().as_mut().unwrap()) * (*self.width.clone().lock().unwrap().as_mut().unwrap()) + (*2.lock().unwrap().as_mut().unwrap()) * (*self.height.clone().lock().unwrap().as_mut().unwrap()))));
    }
}

fn main() {
    let mut r = rect { width: std::sync::Arc::new(std::sync::Mutex::new(Some(10))), height: std::sync::Arc::new(std::sync::Mutex::new(Some(5))) };
    println!("{} {}", "area: ".to_string(), (*(*r.lock().unwrap().as_mut().unwrap()).area().lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "perim:".to_string(), (*(*r.lock().unwrap().as_mut().unwrap()).perim().lock().unwrap().as_mut().unwrap()));

    let mut rp = r.clone();
    println!("{} {}", "area: ".to_string(), (*(*rp.lock().unwrap().as_mut().unwrap()).area().lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "perim:".to_string(), (*(*rp.lock().unwrap().as_mut().unwrap()).perim().lock().unwrap().as_mut().unwrap()));
}