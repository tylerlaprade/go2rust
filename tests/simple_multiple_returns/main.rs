use std::sync::{Arc, Mutex};

pub fn divmod(a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>) -> (Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) {

    return ({
            let __tmp_x = (*a.lock().unwrap().as_mut().unwrap());
            let __tmp_y = (*b.lock().unwrap().as_mut().unwrap());
            Arc::new(Mutex::new(Some(__tmp_x / __tmp_y)))
        }, {
            let __tmp_x = (*a.lock().unwrap().as_mut().unwrap());
            let __tmp_y = (*b.lock().unwrap().as_mut().unwrap());
            Arc::new(Mutex::new(Some(__tmp_x % __tmp_y)))
        });
}

pub fn swap(a: Arc<Mutex<Option<String>>>, b: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) {

    return (b.clone(), a.clone());
}

fn main() {
        // Basic multiple returns
    let (mut q, mut r) = divmod(Arc::new(Mutex::new(Some(17))), Arc::new(Mutex::new(Some(5))));
    println!("{} {} {} {}", "Quotient:".to_string(), (*q.lock().unwrap().as_mut().unwrap()), "Remainder:".to_string(), (*r.lock().unwrap().as_mut().unwrap()));

        // Multiple assignment
    let (mut x, mut y) = (Arc::new(Mutex::new(Some("hello".to_string()))), Arc::new(Mutex::new(Some("world".to_string()))));
    println!("{} {} {}", "Before swap:".to_string(), (*x.lock().unwrap().as_mut().unwrap()), (*y.lock().unwrap().as_mut().unwrap()));

        // Swap using function
    (x, y) = swap(x.clone(), y.clone());
    println!("{} {} {}", "After swap:".to_string(), (*x.lock().unwrap().as_mut().unwrap()), (*y.lock().unwrap().as_mut().unwrap()));

        // Ignoring values
    let (_, mut r2) = divmod(Arc::new(Mutex::new(Some(23))), Arc::new(Mutex::new(Some(7))));
    println!("{} {}", "23 mod 7 =".to_string(), (*r2.lock().unwrap().as_mut().unwrap()));
}