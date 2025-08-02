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







impl Celsius {
    pub fn to_fahrenheit(&self) -> std::sync::Arc<std::sync::Mutex<Option<Fahrenheit>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some(fahrenheit(std::sync::Arc::new(std::sync::Mutex::new(Some(self * 9 / 5 + 32)))))));
    }
}

impl Fahrenheit {
    pub fn to_celsius(&self) -> std::sync::Arc<std::sync::Mutex<Option<Celsius>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some(celsius(std::sync::Arc::new(std::sync::Mutex::new(Some((self - 32) * 5 / 9)))))));
    }
}

fn main() {
    let mut temp: std::sync::Arc<std::sync::Mutex<Option<Celsius>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(100)));
    print!("{}°C = {}°F\n", (*temp.lock().unwrap().as_mut().unwrap()), (*(*temp.lock().unwrap().as_mut().unwrap()).to_fahrenheit().lock().unwrap().as_mut().unwrap()));

    let mut f: std::sync::Arc<std::sync::Mutex<Option<Fahrenheit>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(212)));
    print!("{}°F = {}°C\n", (*f.lock().unwrap().as_mut().unwrap()), (*(*f.lock().unwrap().as_mut().unwrap()).to_celsius().lock().unwrap().as_mut().unwrap()));

    let mut s: std::sync::Arc<std::sync::Mutex<Option<StringAlias>>> = std::sync::Arc::new(std::sync::Mutex::new(Some("hello".to_string())));
    println!("{}", (*s.lock().unwrap().as_mut().unwrap()));
}