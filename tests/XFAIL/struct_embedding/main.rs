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