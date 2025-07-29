fn main() {
    let mut arr: std::sync::Arc<std::sync::Mutex<Option<[i32; 3]>>> = Default::default();
    (*arr.lock().unwrap().as_mut().unwrap())[0] = 10;
    (*arr.lock().unwrap().as_mut().unwrap())[1] = 20;
    (*arr.lock().unwrap().as_mut().unwrap())[2] = 30;
    println!("{}", "Array elements:".to_string());
    let mut i = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_ref().unwrap()) < (*arr.lock().unwrap().as_ref().unwrap()).len() {
        println!("{}", (*arr.lock().unwrap().as_ref().unwrap())[(*i.lock().unwrap().as_ref().unwrap())]);
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    let mut nums = std::sync::Arc::new(std::sync::Mutex::new(Some([1, 2, 3, 4])));
    println!("{}", "Initialized array:".to_string());
    for (_, num) in (*nums.lock().unwrap().as_ref().unwrap()).iter().enumerate() {
        println!("{}", (*num.lock().unwrap().as_ref().unwrap()));
    }
}