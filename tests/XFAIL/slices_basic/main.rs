fn main() {
    let mut slice = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![1, 2, 3, 4, 5])));
    println!("{} {}", "Original slice:".to_string(), (*slice.lock().unwrap().as_ref().unwrap()));
    { let new_val = {(*slice.lock().unwrap().as_ref().unwrap()).extend(vec![6, 7]); (*slice.lock().unwrap().as_ref().unwrap())}; *slice.lock().unwrap() = Some(new_val); };
    println!("{} {}", "After append:".to_string(), (*slice.lock().unwrap().as_ref().unwrap()));
    let mut subSlice = std::sync::Arc::new(std::sync::Mutex::new(Some((*slice.lock().unwrap().as_ref().unwrap())[1..4].to_vec())));
    println!("{} {}", "Sub-slice [1:4]:".to_string(), (*subSlice.lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Length:".to_string(), (*slice.lock().unwrap().as_ref().unwrap()).len());
    println!("{} {}", "Capacity:".to_string(), (*slice.lock().unwrap().as_ref().unwrap()).capacity());
    let mut made = vec![0; 3];
    (*made.lock().unwrap().as_mut().unwrap())[0] = 10;
    (*made.lock().unwrap().as_mut().unwrap())[1] = 20;
    (*made.lock().unwrap().as_mut().unwrap())[2] = 30;
    println!("{} {}", "Made slice:".to_string(), (*made.lock().unwrap().as_ref().unwrap()));
}