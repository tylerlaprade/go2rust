fn main() {
    let mut s = vec![0; 3];
    print!("len={} cap={} {}\n", (*s.lock().unwrap().as_mut().unwrap()).len(), (*s.lock().unwrap().as_mut().unwrap()).capacity(), (*s.lock().unwrap().as_mut().unwrap()));
    { let new_val = {(*s.lock().unwrap().as_mut().unwrap()).extend(vec![1, 2, 3, 4, 5, 6, 7, 8]); (*s.lock().unwrap().as_mut().unwrap())}; *s.lock().unwrap() = Some(new_val); };
    print!("len={} cap={} {}\n", (*s.lock().unwrap().as_mut().unwrap()).len(), (*s.lock().unwrap().as_mut().unwrap()).capacity(), (*s.lock().unwrap().as_mut().unwrap()));
    let mut s2 = std::sync::Arc::new(std::sync::Mutex::new(Some((*s.lock().unwrap().as_mut().unwrap())[2..5].to_vec())));
    print!("s2: len={} cap={} {}\n", (*s2.lock().unwrap().as_mut().unwrap()).len(), (*s2.lock().unwrap().as_mut().unwrap()).capacity(), (*s2.lock().unwrap().as_mut().unwrap()));
    let mut s3 = vec![0; 3];
    let mut n = copy(std::sync::Arc::new(std::sync::Mutex::new(Some((*s3.lock().unwrap().as_mut().unwrap())))), std::sync::Arc::new(std::sync::Mutex::new(Some((*s.lock().unwrap().as_mut().unwrap())))));
    print!("Copied {} elements: {}\n", (*n.lock().unwrap().as_mut().unwrap()), (*s3.lock().unwrap().as_mut().unwrap()));
    let mut s4: std::sync::Arc<std::sync::Mutex<Option<Vec<i32>>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(Default::default())));
    let mut s5 = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![])));
    print!("s4==nil: {}, s5==nil: {}\n", (*s4.lock().unwrap()).is_none(), (*s5.lock().unwrap()).is_none());
}