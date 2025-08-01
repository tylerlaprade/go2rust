const FLAG_READ: i32 = 1 << 0;
const FLAG_WRITE: i32 = 1;
const FLAG_EXECUTE: i32 = 2;
const FLAG_DELETE: i32 = 3;


const K_B: i32 = 1 << (10 * 1);
const M_B: i32 = 2;
const G_B: i32 = 3;
const T_B: i32 = 4;


const A: i32 = 0;
const B: i32 = 0 * 10;
const C: i32 = 1;
const D: i32 = 1;
const E: i32 = 2;
const F: i32 = 2;


const FIRST: i32 = 0;
const SECOND: i32 = 1;


const THIRD: i32 = 0;
const FOURTH: i32 = 1;


fn main() {
    let mut perms = std::sync::Arc::new(std::sync::Mutex::new(Some(FLAG_READ | FLAG_WRITE)));
    print!("Permissions: {} (Read={}, Write={})\n", (*perms.lock().unwrap().as_mut().unwrap()), FLAG_READ, FLAG_WRITE);
    print!("KB={}, MB={}, GB={}\n", K_B, M_B, G_B);
    print!("A={}, B={}, C={}, D={}, E={}, F={}\n", A, B, C, D, E, F);
    print!("First={}, Second={}, Third={}, Fourth={}\n", FIRST, SECOND, THIRD, FOURTH);
}