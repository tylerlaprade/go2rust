use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default)]
struct base {
    num: Arc<Mutex<Option<i32>>>,
}

#[derive(Debug, Clone, Default)]
struct container {
    base: Arc<Mutex<Option<base>>>,
    str: Arc<Mutex<Option<String>>>,
}

impl base {
    pub fn describe(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("base with num={}", (*self.num.lock().unwrap().as_ref().unwrap())))));
    }
}

impl container {
    pub fn describe(&self) -> Arc<Mutex<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.base.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.describe()
    }
}

fn main() {
    let mut co = Arc::new(Mutex::new(Some(container { base: Arc::new(Mutex::new(Some(base { num: Arc::new(Mutex::new(Some(1))) }))), str: Arc::new(Mutex::new(Some("some name".to_string()))) })));

    print!("co={num: {}, str: {}}\n", (*(*(*co.lock().unwrap().as_ref().unwrap()).base.lock().unwrap().as_ref().unwrap()).num.lock().unwrap().as_ref().unwrap()), (*(*co.lock().unwrap().as_ref().unwrap()).str.lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "also num:".to_string(), (*(*co.lock().unwrap().as_mut().unwrap()).base.lock().unwrap().as_mut().unwrap()).base.num);
    println!("{} {}", "describe:".to_string(), (*(*co.lock().unwrap().as_mut().unwrap()).describe().lock().unwrap().as_ref().unwrap()));

    type describer = Arc<Mutex<Option<Unknown>>>;

    let mut d: Arc<Mutex<Option<describer>>> = Arc::new(Mutex::new(Some((*co.lock().unwrap().as_mut().unwrap()))));
    println!("{} {}", "describer:".to_string(), (*(*d.lock().unwrap().as_mut().unwrap()).describe().lock().unwrap().as_ref().unwrap()));
}