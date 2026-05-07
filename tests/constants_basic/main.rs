use std::cell::{RefCell};
use std::rc::{Rc};

pub const PI: f64 = 3.14159;
pub const EULER: f64 = 2.71828;
pub const MAX_USERS: i32 = 100;


pub const NAME: &'static str = "Go2Rust";
pub const VERSION: f64 = 1.0;
pub const DEBUG: bool = true;


pub const SUNDAY: i32 = 0;
pub const MONDAY: i32 = 1;
pub const TUESDAY: i32 = 2;
pub const WEDNESDAY: i32 = 3;
pub const THURSDAY: i32 = 4;
pub const FRIDAY: i32 = 5;
pub const SATURDAY: i32 = 6;


pub const K_B: i64 = 1 << (10 * 1);
pub const M_B: i64 = 1 << (10 * 2);
pub const G_B: i64 = 1 << (10 * 3);
pub const T_B: i64 = 1 << (10 * 4);


pub const RED: i32 = 0;
pub const GREEN: i32 = 1;
pub const BLUE: i32 = 2;


pub const A: i32 = 0 * 2;
pub const B: i32 = 1 * 2;
pub const C: i32 = 2 * 2;
pub const D: i32 = 3 + 10;
pub const E_E: i32 = 4 + 10;
pub const F: i32 = 5 + 10;


fn main() {
        // Basic constants
    println!("{}", "=== Basic constants ===".to_string());
    print!("Pi = {:.5}\n", PI);
    print!("Euler = {:.5}\n", EULER);
    print!("MaxUsers = {}\n", MAX_USERS);

        // Typed constants
    println!("{}", "\n=== Typed constants ===".to_string());
    print!("Name: {}\n", NAME);
    print!("Version: {:.1}\n", VERSION);
    print!("Debug: {}\n", DEBUG);

        // Weekday constants
    println!("{}", "\n=== Weekday constants ===".to_string());
    print!("Sunday = {}\n", SUNDAY);
    print!("Monday = {}\n", MONDAY);
    print!("Wednesday = {}\n", WEDNESDAY);
    print!("Saturday = {}\n", SATURDAY);

        // Size constants
    println!("{}", "\n=== Size constants ===".to_string());
    print!("KB = {} bytes\n", K_B);
    print!("MB = {} bytes\n", M_B);
    print!("GB = {} bytes\n", G_B);
    print!("TB = {} bytes\n", T_B);

        // Color constants
    println!("{}", "\n=== Color constants ===".to_string());
    print!("Red = {}\n", RED);
    print!("Green = {}\n", GREEN);
    print!("Blue = {}\n", BLUE);

        // Complex iota
    println!("{}", "\n=== Complex iota expressions ===".to_string());
    print!("A = {}\n", A);
    print!("B = {}\n", B);
    print!("C = {}\n", C);
    print!("D = {}\n", D);
    print!("EE = {}\n", E_E);
    print!("F = {}\n", F);

        // Local constants
    println!("{}", "\n=== Local constants ===".to_string());
    const localConst: i32 = 42;

    const x: i32 = 10;
const y: i32 = 20;
const z: i32 = x + y;


    print!("localConst = {}\n", localConst);
    print!("x = {}, y = {}, z = {}\n", x, y, z);

        // Untyped constants in expressions
    println!("{}", "\n=== Untyped constants in expressions ===".to_string());
    const untypedInt: i32 = 100;

    const untypedFloat: f64 = 3.14;


    let mut i: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(untypedInt)));
    let mut f: Rc<RefCell<Option<f64>>> = Rc::new(RefCell::new(Some(untypedFloat)));
    let mut mixed = Rc::new(RefCell::new(Some(untypedInt + 3)));

    print!("i = {}\n", { let __v = (*i.borrow().as_ref().unwrap()).clone(); __v });
    print!("f = {:.2}\n", { let __v = (*f.borrow().as_ref().unwrap()).clone(); __v });
    print!("mixed = {}\n", { let __v = (*mixed.borrow().as_ref().unwrap()).clone(); __v });

        // String constants
    println!("{}", "\n=== String constants ===".to_string());
    const greeting: &'static str = "Hello";

    const target: &'static str = "World";

    const message: &'static str = "Hello, World!";


    println!("{}", message);
}