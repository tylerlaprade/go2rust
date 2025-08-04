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

#[derive(Debug, Clone)]
struct Celsius(Arc<Mutex<Option<f64>>>);

impl Display for Celsius {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}


#[derive(Debug, Clone)]
struct Fahrenheit(Arc<Mutex<Option<f64>>>);

impl Display for Fahrenheit {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}


type StringAlias = Arc<Mutex<Option<String>>>;


impl Celsius {
    pub fn to_fahrenheit(&self) -> Arc<Mutex<Option<Fahrenheit>>> {
        return Arc::new(Mutex::new(Some(Fahrenheit(Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) * 9.0 / 5.0 + 32.0)))))));
    }
}

impl Fahrenheit {
    pub fn to_celsius(&self) -> Arc<Mutex<Option<Celsius>>> {
        return Arc::new(Mutex::new(Some(Celsius(Arc::new(Mutex::new(Some(((*self.0.lock().unwrap().as_ref().unwrap()) - 32.0) * 5.0 / 9.0)))))));
    }
}

fn main() {
    let mut temp: Arc<Mutex<Option<Celsius>>> = Arc::new(Mutex::new(Some(100)));
    print!("{}°C = {}°F\n", (*temp.lock().unwrap().as_mut().unwrap()), (*(*temp.lock().unwrap().as_mut().unwrap()).to_fahrenheit().lock().unwrap().as_mut().unwrap()));

    let mut f: Arc<Mutex<Option<Fahrenheit>>> = Arc::new(Mutex::new(Some(212)));
    print!("{}°F = {}°C\n", (*f.lock().unwrap().as_mut().unwrap()), (*(*f.lock().unwrap().as_mut().unwrap()).to_celsius().lock().unwrap().as_mut().unwrap()));

    let mut s: StringAlias = Arc::new(Mutex::new(Some("hello".to_string())));
    println!("{}", (*s.lock().unwrap().as_mut().unwrap()));
}