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
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*fmt.lock().unwrap().as_ref().unwrap()).sprintf(std::sync::Arc::new(std::sync::Mutex::new(Some("base with num=%v".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some(self.num)))))));
    }
}

fn main() {
    let mut co = std::sync::Arc::new(std::sync::Mutex::new(Some(container { base: base { num: 1 }, str: "some name".to_string() })));
    print!("co={num: {}, str: {}}\n", (*co.lock().unwrap().as_ref().unwrap()).num, (*co.lock().unwrap().as_ref().unwrap()).str);
    println!("{} {}", "also num:".to_string(), (*co.lock().unwrap().as_ref().unwrap()).base::num);
    println!("{} {}", "describe:".to_string(), (*co.lock().unwrap().as_ref().unwrap()).describe());
    
    let mut d = std::sync::Arc::new(std::sync::Mutex::new(Some((*co.lock().unwrap().as_ref().unwrap()))));
    println!("{} {}", "describer:".to_string(), (*d.lock().unwrap().as_ref().unwrap()).describe());
}