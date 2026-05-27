use std::cell::{RefCell};
use std::rc::{Rc};

pub const FLAG_READ: i32 = 1 << 0;
pub const FLAG_WRITE: i32 = 1 << 1;
pub const FLAG_EXECUTE: i32 = 1 << 2;
pub const FLAG_DELETE: i32 = 1 << 3;


pub const K_B: i64 = 1 << (10 * 1);
pub const M_B: i64 = 1 << (10 * 2);
pub const G_B: i64 = 1 << (10 * 3);
pub const T_B: i64 = 1 << (10 * 4);


pub const A: i32 = 0;
pub const B: i32 = 0 * 10;
pub const C: i32 = 1;
pub const D: i32 = 1 * 10;
pub const E: i32 = 2;
pub const F: i32 = 2 * 10;


pub const FIRST: i32 = 0;
pub const SECOND: i32 = 1;


pub const THIRD: i32 = 0;
pub const FOURTH: i32 = 1;


fn main() {
        // Test bit flags
    let mut perms = Rc::new(RefCell::new(Some(FLAG_READ | FLAG_WRITE)));
    print!("Permissions: {} (Read={}, Write={})\n", { let __v = (*perms.borrow().as_ref().unwrap()).clone(); __v }, FLAG_READ, FLAG_WRITE);

        // Test size constants
    print!("KB={}, MB={}, GB={}\n", K_B, M_B, G_B);

        // Test multiple iotas
    print!("A={}, B={}, C={}, D={}, E={}, F={}\n", A, B, C, D, E, F);

        // Test reset
    print!("First={}, Second={}, Third={}, Fourth={}\n", FIRST, SECOND, THIRD, FOURTH);
}