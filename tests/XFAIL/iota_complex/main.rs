use std::cell::{RefCell};
use std::rc::{Rc};

const FLAG_READ: i64 = 1 << 0;
const FLAG_WRITE: i64 = 1 << 1;
const FLAG_EXECUTE: i64 = 1 << 2;
const FLAG_DELETE: i64 = 1 << 3;


const K_B: i64 = 1 << (10 * 1);
const M_B: i64 = 1 << (10 * 2);
const G_B: i64 = 1 << (10 * 3);
const T_B: i64 = 1 << (10 * 4);


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
        // Test bit flags
    let mut perms = Rc::new(RefCell::new(Some(FLAG_READ | FLAG_WRITE)));
    print!("Permissions: {} (Read={}, Write={})\n", (*perms.borrow_mut().as_mut().unwrap()), FLAG_READ, FLAG_WRITE);

        // Test size constants
    print!("KB={}, MB={}, GB={}\n", K_B, M_B, G_B);

        // Test multiple iotas
    print!("A={}, B={}, C={}, D={}, E={}, F={}\n", A, B, C, D, E, F);

        // Test reset
    print!("First={}, Second={}, Third={}, Fourth={}\n", FIRST, SECOND, THIRD, FOURTH);
}