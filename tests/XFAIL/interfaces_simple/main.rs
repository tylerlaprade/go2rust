

#[derive(Debug)]
struct rect {
    width: std::sync::Arc<std::sync::Mutex<Option<f64>>>,
    height: std::sync::Arc<std::sync::Mutex<Option<f64>>>,
}

impl rect {
    pub fn area(&self) -> std::sync::Arc<std::sync::Mutex<Option<f64>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some(self.width * self.height)));
    }

    pub fn perim(&self) -> std::sync::Arc<std::sync::Mutex<Option<f64>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some(2 * self.width + 2 * self.height)));
    }
}

pub fn measure(g: std::sync::Arc<std::sync::Mutex<Option<geometry>>>) {
    println!("{}", (*g.lock().unwrap().as_ref().unwrap()));
    println!("{}", (*g.lock().unwrap().as_ref().unwrap()).area());
    println!("{}", (*g.lock().unwrap().as_ref().unwrap()).perim());
}

fn main() {
    let mut r = std::sync::Arc::new(std::sync::Mutex::new(Some(rect { width: 3, height: 4 })));
    measure(std::sync::Arc::new(std::sync::Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap())))));
}