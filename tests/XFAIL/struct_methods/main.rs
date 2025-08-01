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
    let mut r = std::sync::Arc::new(std::sync::Mutex::new(Some(rect { width: std::sync::Arc::new(std::sync::Mutex::new(Some(10))), height: std::sync::Arc::new(std::sync::Mutex::new(Some(5))) })));
    println!("{} {}", "area: ".to_string(), (*(*r.lock().unwrap().as_mut().unwrap()).area().lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "perim:".to_string(), (*(*r.lock().unwrap().as_mut().unwrap()).perim().lock().unwrap().as_mut().unwrap()));
    let mut rp = r.clone();
    println!("{} {}", "area: ".to_string(), (*(*rp.lock().unwrap().as_mut().unwrap()).area().lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "perim:".to_string(), (*(*rp.lock().unwrap().as_mut().unwrap()).perim().lock().unwrap().as_mut().unwrap()));
}