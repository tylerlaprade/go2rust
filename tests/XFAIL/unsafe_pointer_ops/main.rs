use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct Point {
    x: Arc<Mutex<Option<i32>>>,
    y: Arc<Mutex<Option<i32>>>,
}

fn main() {
    let mut p = Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(10))), y: Arc::new(Mutex::new(Some(20))) })));

    let mut xPtr = (((*int32.lock().unwrap().as_ref().unwrap())).lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some((*p.lock().unwrap().as_mut().unwrap())))));
    print!("X via unsafe: {}\n", (*xPtr.lock().unwrap().as_ref().unwrap()));

    let mut yPtr = (((*int32.lock().unwrap().as_ref().unwrap())).lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some((*(*p.lock().unwrap().as_mut().unwrap()).lock().unwrap().as_ref().unwrap()) as usize))) + (*unsafe.lock().unwrap().as_mut().unwrap()).offsetof(Arc::new(Mutex::new(Some((*p.lock().unwrap().as_mut().unwrap()).y))))))));
    print!("Y via unsafe: {}\n", (*yPtr.lock().unwrap().as_ref().unwrap()));

    print!("Size: {}, Align: {}\n", (*(*unsafe.lock().unwrap().as_mut().unwrap()).sizeof(Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap()))))).lock().unwrap().as_ref().unwrap()), (*(*unsafe.lock().unwrap().as_mut().unwrap()).alignof(Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap()))))).lock().unwrap().as_ref().unwrap()));
}