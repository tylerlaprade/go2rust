use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct base {
    num: Arc<Mutex<Option<i32>>>,
}

#[derive(Debug)]
struct container {
    base: Arc<Mutex<Option<base>>>,
    str: Arc<Mutex<Option<String>>>,
}

impl base {
    pub fn describe(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("base with num={}", (*self.num.lock().unwrap().as_ref().unwrap())))));
    }
}

fn main() {
    let mut co = container { base: Arc::new(Mutex::new(Some(base { num: Arc::new(Mutex::new(Some(1))) }))), str: Arc::new(Mutex::new(Some("some name".to_string()))) };

    print!("co={num: {}, str: {}}\n", (*(*co.lock().unwrap().as_mut().unwrap()).base.num.lock().unwrap().as_ref().unwrap()), (*(*co.lock().unwrap().as_mut().unwrap()).str.lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "also num:".to_string(), (*co.lock().unwrap().as_mut().unwrap()).base.num);
    println!("{} {}", "describe:".to_string(), (*(*co.lock().unwrap().as_mut().unwrap()).describe().lock().unwrap().as_ref().unwrap()));

    

    let mut d: Arc<Mutex<Option<describer>>> = Arc::new(Mutex::new(Some((*co.lock().unwrap().as_mut().unwrap()))));
    println!("{} {}", "describer:".to_string(), (*(*d.lock().unwrap().as_mut().unwrap()).describe().lock().unwrap().as_ref().unwrap()));
}