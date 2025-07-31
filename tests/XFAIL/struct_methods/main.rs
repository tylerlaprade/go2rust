#[derive(Debug)]
struct rect {
    width: std::sync::Arc<std::sync::Mutex<Option<i32>>>,
    height: std::sync::Arc<std::sync::Mutex<Option<i32>>>,
}

impl rect {
    pub fn area(&mut self) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some(self.width * self.height)));
    }

    pub fn perim(&self) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some(2 * self.width + 2 * self.height)));
    }
}

fn main() {
    let mut r = std::sync::Arc::new(std::sync::Mutex::new(Some(rect { width: 10, height: 5 })));
    println!("{} {}", "area: ".to_string(), (*r.lock().unwrap().as_ref().unwrap()).area());
    println!("{} {}", "perim:".to_string(), (*r.lock().unwrap().as_ref().unwrap()).perim());
    let mut rp = r.clone();
    println!("{} {}", "area: ".to_string(), (*rp.lock().unwrap().as_ref().unwrap()).area());
    println!("{} {}", "perim:".to_string(), (*rp.lock().unwrap().as_ref().unwrap()).perim());
}