pub fn vals() -> (std::sync::Arc<std::sync::Mutex<Option<i32>>>, std::sync::Arc<std::sync::Mutex<Option<i32>>>) {

    return (std::sync::Arc::new(std::sync::Mutex::new(Some(3))), std::sync::Arc::new(std::sync::Mutex::new(Some(7))));
}

fn main() {
    let (mut a, mut b) = vals();
    println!("{}", (*a.lock().unwrap().as_mut().unwrap()));
    println!("{}", (*b.lock().unwrap().as_mut().unwrap()));
    let (_, mut c) = vals();
    println!("{}", (*c.lock().unwrap().as_mut().unwrap()));
}