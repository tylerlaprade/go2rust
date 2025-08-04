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
struct argError {
    arg: Arc<Mutex<Option<i32>>>,
    prob: Arc<Mutex<Option<String>>>,
}

impl argError {
    pub fn error(&mut self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("{} - {}", (*self.arg.lock().unwrap().as_mut().unwrap()), (*self.prob.lock().unwrap().as_mut().unwrap())))));
    }
}

impl Error for argError {}

impl Display for argError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_mut().unwrap()))
    }
}

pub fn f1(arg: Arc<Mutex<Option<i32>>>) -> (Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<Box<dyn Error + Send + Sync>>>>) {

    if (*arg.lock().unwrap().as_mut().unwrap()) == 42 {
        return (Arc::new(Mutex::new(Some(-1))), Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("can't work with 42".to_string())))));
    }
    return (Arc::new(Mutex::new(Some((*arg.lock().unwrap().as_mut().unwrap()) + 3))), Arc::new(Mutex::new(None)));
}

pub fn f2(arg: Arc<Mutex<Option<i32>>>) -> (Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<Box<dyn Error + Send + Sync>>>>) {

    if (*arg.lock().unwrap().as_mut().unwrap()) == 42 {
        return (Arc::new(Mutex::new(Some(-1))), Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some(argError { ,  })))))));
    }
    return (Arc::new(Mutex::new(Some((*arg.lock().unwrap().as_mut().unwrap()) + 3))), Arc::new(Mutex::new(None)));
}

fn main() {
    for i in &Arc::new(Mutex::new(Some(vec![7, 42]))) {
        let (mut r, mut e) = f1(Arc::new(Mutex::new(Some(i))));
    if (*e.lock().unwrap()).is_some() {
        println!("{} {}", "f1 failed:".to_string(), (*e.lock().unwrap().as_mut().unwrap()));
    } else {
        println!("{} {}", "f1 worked:".to_string(), (*r.lock().unwrap().as_mut().unwrap()));
    }
    }
    for i in &Arc::new(Mutex::new(Some(vec![7, 42]))) {
        let (mut r, mut e) = f2(Arc::new(Mutex::new(Some(i))));
    if (*e.lock().unwrap()).is_some() {
        println!("{} {}", "f2 failed:".to_string(), (*e.lock().unwrap().as_mut().unwrap()));
    } else {
        println!("{} {}", "f2 worked:".to_string(), (*r.lock().unwrap().as_mut().unwrap()));
    }
    }

    let (_, mut e) = f2(Arc::new(Mutex::new(Some(42))));
    let (mut ae, mut ok) = match (*e.lock().unwrap().as_mut().unwrap()).downcast_ref::<Arc<Mutex<Option<argError>>>>() { Some(v) => (v.clone(), true), None => (Default::default(), false) };
    if (*ok.lock().unwrap().as_mut().unwrap()) {
        println!("{}", (*ae.lock().unwrap().as_mut().unwrap()).arg);
        println!("{}", (*ae.lock().unwrap().as_mut().unwrap()).prob);
    }
}