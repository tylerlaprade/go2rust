#[derive(Debug)]
struct Point {
    x: std::sync::Arc<std::sync::Mutex<Option<int32>>>,
    y: std::sync::Arc<std::sync::Mutex<Option<int32>>>,
}

fn main() {
    let mut p = std::sync::Arc::new(std::sync::Mutex::new(Some(Point { x: std::sync::Arc::new(std::sync::Mutex::new(Some(10))), y: std::sync::Arc::new(std::sync::Mutex::new(Some(20))) })));
    let mut xPtr = ((*int32.lock().unwrap().as_mut().unwrap()))(std::sync::Arc::new(std::sync::Mutex::new(Some((*unsafe.lock().unwrap().as_mut().unwrap()).pointer(std::sync::Arc::new(std::sync::Mutex::new(Some((*p.lock().unwrap().as_mut().unwrap())))))))));
    print!("X via unsafe: {}\n", (*xPtr.lock().unwrap().as_mut().unwrap()));
    let mut yPtr = ((*int32.lock().unwrap().as_mut().unwrap()))(std::sync::Arc::new(std::sync::Mutex::new(Some((*unsafe.lock().unwrap().as_mut().unwrap()).pointer(std::sync::Arc::new(std::sync::Mutex::new(Some(uintptr(std::sync::Arc::new(std::sync::Mutex::new(Some((*unsafe.lock().unwrap().as_mut().unwrap()).pointer(std::sync::Arc::new(std::sync::Mutex::new(Some((*p.lock().unwrap().as_mut().unwrap()))))))))) + (*unsafe.lock().unwrap().as_mut().unwrap()).offsetof(std::sync::Arc::new(std::sync::Mutex::new(Some((*p.lock().unwrap().as_mut().unwrap()).y))))))))))));
    print!("Y via unsafe: {}\n", (*yPtr.lock().unwrap().as_mut().unwrap()));
    print!("Size: {}, Align: {}\n", (*unsafe.lock().unwrap().as_mut().unwrap()).sizeof(std::sync::Arc::new(std::sync::Mutex::new(Some((*p.lock().unwrap().as_mut().unwrap()))))), (*unsafe.lock().unwrap().as_mut().unwrap()).alignof(std::sync::Arc::new(std::sync::Mutex::new(Some((*p.lock().unwrap().as_mut().unwrap()))))));
}