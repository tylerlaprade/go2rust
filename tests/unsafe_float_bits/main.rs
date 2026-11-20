use std::sync::{Arc, Mutex};

pub fn float64bits(f: Arc<Mutex<Option<f64>>>) -> u64 {
    (({ let __v = (*f.lock().unwrap().as_ref().unwrap()).clone(); __v }).to_bits() as u64)
}

pub fn float64frombits(b: Arc<Mutex<Option<u64>>>) -> f64 {
    f64::from_bits(({ let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }) as u64)
}

pub fn float32bits(f: Arc<Mutex<Option<f32>>>) -> u32 {
    (({ let __v = (*f.lock().unwrap().as_ref().unwrap()).clone(); __v }).to_bits() as u32)
}

pub fn signed_of(p: Arc<Mutex<Option<u64>>>) -> i64 {
    (({ let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }) as u64 as i64)
}

pub fn abs(x: Arc<Mutex<Option<f64>>>) -> f64 {
    float64frombits(Arc::new(Mutex::new(Some({ let __tmp_x = float64bits(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = ((1 as u64) << (63 as u64)) as u64; __tmp_x & ! __tmp_y }))))
}

fn main() {
    let mut b = float64bits(Arc::new(Mutex::new(Some(1.5))));
    print!("{} {} {}\n", format!("{}", format!("{:x}", b)), format!("{}", float64frombits(Arc::new(Mutex::new(Some(b))))), format!("{}", format!("{:x}", float32bits(Arc::new(Mutex::new(Some(-2.0)))))));
    let mut u = Arc::new(Mutex::new(Some(((1 as u64) << (63 as u64)) as u64)));
    println!("{} {} {}", format!("{}", signed_of(u.clone())), format!("{}", abs(Arc::new(Mutex::new(Some(-0.25))))), format!("{}", abs(Arc::new(Mutex::new(Some(3.0))))));
}