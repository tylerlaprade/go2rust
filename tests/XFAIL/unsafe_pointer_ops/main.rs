use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct Point {
    x: Arc<Mutex<Option<i32>>>,
    y: Arc<Mutex<Option<i32>>>,
}

fn main() {
    let mut p = Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(10))), y: Arc::new(Mutex::new(Some(20))) })));

        // Get pointer to X field
    let mut xPtr = (((*int32.lock().unwrap().as_ref().unwrap())).lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some((*p.lock().unwrap().as_mut().unwrap())))));
    print!("X via unsafe: {}\n", (*xPtr.lock().unwrap().as_ref().unwrap()));

        // Get pointer to Y field
    let mut yPtr = (((*int32.lock().unwrap().as_ref().unwrap())).lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some((*(*p.lock().unwrap().as_mut().unwrap()).lock().unwrap().as_ref().unwrap()) as usize))) + (*unsafe.offsetof(Arc::new(Mutex::new(Some(p.y)))).lock().unwrap().as_ref().unwrap())))));
    print!("Y via unsafe: {}\n", (*yPtr.lock().unwrap().as_ref().unwrap()));

        // Size and alignment
    print!("Size: {}, Align: {}\n", (*unsafe.sizeof(Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap()))))).lock().unwrap().as_ref().unwrap()), (*unsafe.alignof(Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap()))))).lock().unwrap().as_ref().unwrap()));
}