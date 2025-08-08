use std::sync::{Arc, Mutex};

impl Counter {
    /// Methods for Counter type
    pub fn increment(&mut self) {
        { let mut guard = self.value.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    pub fn add(&mut self, n: Arc<Mutex<Option<i32>>>) {
        { let mut guard = self.value.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + (*n.lock().unwrap().as_mut().unwrap())); };
    }

    pub fn value(&self) -> Arc<Mutex<Option<i32>>> {
        return self.value.clone();
    }
}

impl Point {
    /// Methods for Point type
    pub fn distance(&self, other: Arc<Mutex<Option<Point>>>) -> Arc<Mutex<Option<f64>>> {
        let mut dx = Arc::new(Mutex::new(Some((*self.x.clone().lock().unwrap().as_mut().unwrap()) - (*(*other.lock().unwrap().as_mut().unwrap()).x.lock().unwrap().as_mut().unwrap()))));
        let mut dy = Arc::new(Mutex::new(Some((*self.y.clone().lock().unwrap().as_mut().unwrap()) - (*(*other.lock().unwrap().as_mut().unwrap()).y.lock().unwrap().as_mut().unwrap()))));
        return Arc::new(Mutex::new(Some((*math.lock().unwrap().as_mut().unwrap()).sqrt(Arc::new(Mutex::new(Some((*dx.lock().unwrap().as_mut().unwrap()) * (*dx.lock().unwrap().as_mut().unwrap()) + (*dy.lock().unwrap().as_mut().unwrap()) * (*dy.lock().unwrap().as_mut().unwrap()))))))));
    }

    pub fn move(&mut self, dx: Arc<Mutex<Option<f64>>>, dy: Arc<Mutex<Option<f64>>>) {
        { let mut guard = self.x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + (*dx.lock().unwrap().as_mut().unwrap())); };
        { let mut guard = self.y.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + (*dy.lock().unwrap().as_mut().unwrap())); };
    }
}