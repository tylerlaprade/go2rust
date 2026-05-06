use std::cell::{RefCell};
use std::rc::{Rc};


fn go_rand_state() -> &'static std::sync::Mutex<u64> {
    static STATE: std::sync::OnceLock<std::sync::Mutex<u64>> = std::sync::OnceLock::new();
    STATE.get_or_init(|| std::sync::Mutex::new(1))
}

fn go_rand_seed(seed: i64) {
    *go_rand_state().lock().unwrap() = seed as u64;
}

fn go_rand_next_u64() -> u64 {
    let mut state = go_rand_state().lock().unwrap();
    *state = state
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    *state
}

fn go_rand_intn(n: i32) -> i32 {
    if n <= 0 {
        panic!("invalid argument to Intn");
    }
    (go_rand_next_u64() % n as u64) as i32
}

fn go_rand_float64() -> f64 {
    ((go_rand_next_u64() >> 11) as f64) / ((1u64 << 53) as f64)
}

fn main() {
    go_rand_seed(1 as i64);
    let mut n = Rc::new(RefCell::new(Some(go_rand_intn(100 as i32))));
    let mut f = Rc::new(RefCell::new(Some(go_rand_float64())));
    println!("{} {}", "Random int in range:".to_string(), (*n.borrow().as_ref().unwrap()) >= 0 && (*n.borrow().as_ref().unwrap()) < 100);
    println!("{} {}", "Random float in range:".to_string(), (*f.borrow().as_ref().unwrap()) >= 0.0 && (*f.borrow().as_ref().unwrap()) < 1.0);
}