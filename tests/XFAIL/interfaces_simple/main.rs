use std::sync::{Arc, Mutex};

trait geometry {
    fn area(&self) -> Arc<Mutex<Option<f64>>>;
    fn perim(&self) -> Arc<Mutex<Option<f64>>>;
}

#[derive(Debug)]
struct rect {
    width: Arc<Mutex<Option<f64>>>,
    height: Arc<Mutex<Option<f64>>>,
}

impl rect {
    pub fn area(&self) -> Arc<Mutex<Option<f64>>> {
        return Arc::new(Mutex::new(Some((*self.width.clone().lock().unwrap().as_ref().unwrap()) * (*self.height.clone().lock().unwrap().as_ref().unwrap()))));
    }

    pub fn perim(&self) -> Arc<Mutex<Option<f64>>> {
        return Arc::new(Mutex::new(Some(2 * (*self.width.clone().lock().unwrap().as_ref().unwrap()) + 2 * (*self.height.clone().lock().unwrap().as_ref().unwrap()))));
    }
}

impl geometry for rect {
    fn area(&self) -> Arc<Mutex<Option<f64>>> {
        return Arc::new(Mutex::new(Some((*self.width.clone().lock().unwrap().as_ref().unwrap()) * (*self.height.clone().lock().unwrap().as_ref().unwrap()))));
    }
    fn perim(&self) -> Arc<Mutex<Option<f64>>> {
        return Arc::new(Mutex::new(Some(2 * (*self.width.clone().lock().unwrap().as_ref().unwrap()) + 2 * (*self.height.clone().lock().unwrap().as_ref().unwrap()))));
    }
}

pub fn measure(g: Arc<Mutex<Option<Box<dyn geometry>>>>) {
    println!("{}", (*g.lock().unwrap().as_mut().unwrap()));
    println!("{}", (*g.area().lock().unwrap().as_ref().unwrap()));
    println!("{}", (*g.perim().lock().unwrap().as_ref().unwrap()));
}

fn main() {
    let mut r = rect { width: Arc::new(Mutex::new(Some(3))), height: Arc::new(Mutex::new(Some(4))) };
    measure(r.clone());
}