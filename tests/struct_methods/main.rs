use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default)]
struct rect {
    width: Arc<Mutex<Option<i32>>>,
    height: Arc<Mutex<Option<i32>>>,
}

impl rect {
    pub fn area(&mut self) -> Arc<Mutex<Option<i32>>> {
        return Arc::new(Mutex::new(Some((*self.width.clone().lock().unwrap().as_ref().unwrap()) * (*self.height.clone().lock().unwrap().as_ref().unwrap()))));
    }

    pub fn perim(&self) -> Arc<Mutex<Option<i32>>> {
        return Arc::new(Mutex::new(Some(2 * (*self.width.clone().lock().unwrap().as_ref().unwrap()) + 2 * (*self.height.clone().lock().unwrap().as_ref().unwrap()))));
    }
}

fn main() {
    let mut r = Arc::new(Mutex::new(Some(rect { width: Arc::new(Mutex::new(Some(10))), height: Arc::new(Mutex::new(Some(5))) })));
    println!("{} {}", "area: ".to_string(), (*(*r.lock().unwrap().as_mut().unwrap()).area().lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "perim:".to_string(), (*(*r.lock().unwrap().as_mut().unwrap()).perim().lock().unwrap().as_ref().unwrap()));

    let mut rp = r.clone();
    println!("{} {}", "area: ".to_string(), (*(*rp.lock().unwrap().as_mut().unwrap()).area().lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "perim:".to_string(), (*(*rp.lock().unwrap().as_mut().unwrap()).perim().lock().unwrap().as_ref().unwrap()));
}