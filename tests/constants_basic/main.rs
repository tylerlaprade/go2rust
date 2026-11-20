use std::cell::{RefCell};
use std::rc::{Rc};

pub const PI: f64 = 3.14159;
pub const EULER: f64 = 2.71828;
pub const MAX_USERS: i64 = 100;


pub const NAME: &'static str = "Go2Rust";
pub const VERSION: f64 = 1.0;
pub const DEBUG: bool = true;


pub const SUNDAY: i64 = 0;
pub const MONDAY: i64 = 1;
pub const TUESDAY: i64 = 2;
pub const WEDNESDAY: i64 = 3;
pub const THURSDAY: i64 = 4;
pub const FRIDAY: i64 = 5;
pub const SATURDAY: i64 = 6;


pub const K_B: i64 = 1 << (10 * 1);
pub const M_B: i64 = 1 << (10 * 2);
pub const G_B: i64 = 1 << (10 * 3);
pub const T_B: i64 = 1 << (10 * 4);


pub const RED: i64 = 0;
pub const GREEN: i64 = 1;
pub const BLUE: i64 = 2;


pub const A: i64 = 0 * 2;
pub const B: i64 = 1 * 2;
pub const C: i64 = 2 * 2;
pub const D: i64 = 3 + 10;
pub const E_E: i64 = 4 + 10;
pub const F: i64 = 5 + 10;


fn main() {
        // Basic constants
    println!("{}", format!("{}", "=== Basic constants ===".to_string()));
    print!("Pi = {:.5}\n", PI);
    print!("Euler = {:.5}\n", EULER);
    print!("MaxUsers = {}\n", MAX_USERS);

        // Typed constants
    println!("{}", format!("{}", "\n=== Typed constants ===".to_string()));
    print!("Name: {}\n", NAME);
    print!("Version: {:.1}\n", VERSION);
    print!("Debug: {}\n", DEBUG);

        // Weekday constants
    println!("{}", format!("{}", "\n=== Weekday constants ===".to_string()));
    print!("Sunday = {}\n", SUNDAY);
    print!("Monday = {}\n", MONDAY);
    print!("Wednesday = {}\n", WEDNESDAY);
    print!("Saturday = {}\n", SATURDAY);

        // Size constants
    println!("{}", format!("{}", "\n=== Size constants ===".to_string()));
    print!("KB = {} bytes\n", K_B);
    print!("MB = {} bytes\n", M_B);
    print!("GB = {} bytes\n", G_B);
    print!("TB = {} bytes\n", 1099511627776i64);

        // Color constants
    println!("{}", format!("{}", "\n=== Color constants ===".to_string()));
    print!("Red = {}\n", RED);
    print!("Green = {}\n", GREEN);
    print!("Blue = {}\n", BLUE);

        // Complex iota
    println!("{}", format!("{}", "\n=== Complex iota expressions ===".to_string()));
    print!("A = {}\n", A);
    print!("B = {}\n", B);
    print!("C = {}\n", C);
    print!("D = {}\n", D);
    print!("EE = {}\n", E_E);
    print!("F = {}\n", F);

        // Local constants
    println!("{}", format!("{}", "\n=== Local constants ===".to_string()));
    const localConst: i64 = 42;

    const x: i64 = 10;
const y: i64 = 20;
const z: i64 = x + y;


    print!("localConst = {}\n", localConst);
    print!("x = {}, y = {}, z = {}\n", x, y, z);

        // Untyped constants in expressions
    println!("{}", format!("{}", "\n=== Untyped constants in expressions ===".to_string()));
    const untypedInt: i64 = 100;

    const untypedFloat: f64 = 3.14;


    let mut i: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(100)));
    let mut f: Rc<RefCell<Option<f64>>> = Rc::new(RefCell::new(Some(untypedFloat)));
    let mut mixed = Rc::new(RefCell::new(Some(103i64)));

    print!("i = {}\n", { let __v = (*i.borrow().as_ref().unwrap()).clone(); __v });
    print!("f = {:.2}\n", { let __v = (*f.borrow().as_ref().unwrap()).clone(); __v });
    print!("mixed = {}\n", { let __v = (*mixed.borrow().as_ref().unwrap()).clone(); __v });

        // String constants
    println!("{}", format!("{}", "\n=== String constants ===".to_string()));
    const greeting: &'static str = "Hello";

    const target: &'static str = "World";

    const message: &'static str = "Hello, World!";


    println!("{}", format!("{}", message));
}