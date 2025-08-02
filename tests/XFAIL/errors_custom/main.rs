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
struct argError {
    arg: std::sync::Arc<std::sync::Mutex<Option<i32>>>,
    prob: std::sync::Arc<std::sync::Mutex<Option<String>>>,
}

impl argError {
    pub fn error(&mut self) -> std::sync::Arc<std::sync::Mutex<Option<String>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some(format!("{} - {}", (*self.arg.lock().unwrap().as_mut().unwrap()), (*self.prob.lock().unwrap().as_mut().unwrap())))));
    }
}

impl std::error::Error for argError {}

impl std::fmt::Display for argError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_mut().unwrap()))
    }
}

pub fn f1(arg: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> (std::sync::Arc<std::sync::Mutex<Option<i32>>>, std::sync::Arc<std::sync::Mutex<Option<Box<dyn std::error::Error + Send + Sync>>>>) {

    if (*arg.lock().unwrap().as_mut().unwrap()) == 42 {
        return (std::sync::Arc::new(std::sync::Mutex::new(Some(-1))), std::sync::Arc::new(std::sync::Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("can't work with 42".to_string())))));
    }
    return (std::sync::Arc::new(std::sync::Mutex::new(Some((*arg.lock().unwrap().as_mut().unwrap()) + 3))), std::sync::Arc::new(std::sync::Mutex::new(None)));
}

pub fn f2(arg: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> (std::sync::Arc<std::sync::Mutex<Option<i32>>>, std::sync::Arc<std::sync::Mutex<Option<Box<dyn std::error::Error + Send + Sync>>>>) {

    if (*arg.lock().unwrap().as_mut().unwrap()) == 42 {
        return (std::sync::Arc::new(std::sync::Mutex::new(Some(-1))), std::sync::Arc::new(std::sync::Mutex::new(Some(std::sync::Arc::new(std::sync::Mutex::new(Some(argError { ,  })))))));
    }
    return (std::sync::Arc::new(std::sync::Mutex::new(Some((*arg.lock().unwrap().as_mut().unwrap()) + 3))), std::sync::Arc::new(std::sync::Mutex::new(None)));
}

fn main() {
    for i in &std::sync::Arc::new(std::sync::Mutex::new(Some(vec![7, 42]))) {
        let (mut r, mut e) = f1(std::sync::Arc::new(std::sync::Mutex::new(Some(i))));
    if (*e.lock().unwrap()).is_some() {
        println!("{} {}", "f1 failed:".to_string(), (*e.lock().unwrap().as_mut().unwrap()));
    } else {
        println!("{} {}", "f1 worked:".to_string(), (*r.lock().unwrap().as_mut().unwrap()));
    }
    }
    for i in &std::sync::Arc::new(std::sync::Mutex::new(Some(vec![7, 42]))) {
        let (mut r, mut e) = f2(std::sync::Arc::new(std::sync::Mutex::new(Some(i))));
    if (*e.lock().unwrap()).is_some() {
        println!("{} {}", "f2 failed:".to_string(), (*e.lock().unwrap().as_mut().unwrap()));
    } else {
        println!("{} {}", "f2 worked:".to_string(), (*r.lock().unwrap().as_mut().unwrap()));
    }
    }

    let (_, mut e) = f2(std::sync::Arc::new(std::sync::Mutex::new(Some(42))));
    let (mut ae, mut ok) = match (*e.lock().unwrap().as_mut().unwrap()).downcast_ref::<std::sync::Arc<std::sync::Mutex<Option<argError>>>>() { Some(v) => (v.clone(), true), None => (Default::default(), false) };
    if (*ok.lock().unwrap().as_mut().unwrap()) {
        println!("{}", (*ae.lock().unwrap().as_mut().unwrap()).arg);
        println!("{}", (*ae.lock().unwrap().as_mut().unwrap()).prob);
    }
}