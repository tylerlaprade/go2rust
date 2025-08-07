use std::sync::{Arc, Mutex};

pub fn vals() -> (Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) {

    return (Arc::new(Mutex::new(Some(3))), Arc::new(Mutex::new(Some(7))));
}

fn main() {
    let (mut a, mut b) = vals();
    println!("{}", (*a.lock().unwrap().as_mut().unwrap()));
    println!("{}", (*b.lock().unwrap().as_mut().unwrap()));

    let (_, mut c) = vals();
    println!("{}", (*c.lock().unwrap().as_mut().unwrap()));
}