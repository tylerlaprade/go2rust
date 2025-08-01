const PI: f64 = 3.14159;
const EULER: f64 = 2.71828;
const MAX_USERS: i32 = 100;


const NAME: &'static str = "Go2Rust";
const VERSION: f64 = 1.0;
const DEBUG: bool = true;


const SUNDAY: i32 = 0;
const MONDAY: i32 = 1;
const TUESDAY: i32 = 2;
const WEDNESDAY: i32 = 3;
const THURSDAY: i32 = 4;
const FRIDAY: i32 = 5;
const SATURDAY: i32 = 6;


const K_B: i32 = 1 << (10 * 1);
const M_B: i32 = 2;
const G_B: i32 = 3;
const T_B: i32 = 4;


const RED: i32 = 0;
const GREEN: i32 = 1;
const BLUE: i32 = 2;


const A: i32 = 0 * 2;
const B: i32 = 1;
const C: i32 = 2;
const D: i32 = 3 + 10;
const E_E: i32 = 4;
const F: i32 = 5;


fn main() {
    println!("{}", "=== Basic constants ===".to_string());
    print!("Pi = {:.5}\n", PI);
    print!("Euler = {:.5}\n", EULER);
    print!("MaxUsers = {}\n", MAX_USERS);
    println!("{}", "\n=== Typed constants ===".to_string());
    print!("Name: {}\n", NAME);
    print!("Version: {:.1}\n", VERSION);
    print!("Debug: {}\n", DEBUG);
    println!("{}", "\n=== Weekday constants ===".to_string());
    print!("Sunday = {}\n", SUNDAY);
    print!("Monday = {}\n", MONDAY);
    print!("Wednesday = {}\n", WEDNESDAY);
    print!("Saturday = {}\n", SATURDAY);
    println!("{}", "\n=== Size constants ===".to_string());
    print!("KB = {} bytes\n", K_B);
    print!("MB = {} bytes\n", M_B);
    print!("GB = {} bytes\n", G_B);
    print!("TB = {} bytes\n", T_B);
    println!("{}", "\n=== Color constants ===".to_string());
    print!("Red = {}\n", RED);
    print!("Green = {}\n", GREEN);
    print!("Blue = {}\n", BLUE);
    println!("{}", "\n=== Complex iota expressions ===".to_string());
    print!("A = {}\n", A);
    print!("B = {}\n", B);
    print!("C = {}\n", C);
    print!("D = {}\n", D);
    print!("EE = {}\n", E_E);
    print!("F = {}\n", F);
    println!("{}", "\n=== Local constants ===".to_string());
    const localConst: i32 = 42;

    const x: i32 = 10;
const y: i32 = 20;
const z: i32 = x + y;

    print!("localConst = {}\n", localConst);
    print!("x = {}, y = {}, z = {}\n", x, y, z);
    println!("{}", "\n=== Untyped constants in expressions ===".to_string());
    const untypedInt: i32 = 100;

    const untypedFloat: f64 = 3.14;

    let mut i = std::sync::Arc::new(std::sync::Mutex::new(Some(untypedInt)));
    let mut f = std::sync::Arc::new(std::sync::Mutex::new(Some(untypedFloat)));
    let mut mixed = std::sync::Arc::new(std::sync::Mutex::new(Some(untypedInt + 3)));
    print!("i = {}\n", (*i.lock().unwrap().as_mut().unwrap()));
    print!("f = {:.2}\n", (*f.lock().unwrap().as_mut().unwrap()));
    print!("mixed = {}\n", (*mixed.lock().unwrap().as_mut().unwrap()));
    println!("{}", "\n=== String constants ===".to_string());
    const greeting: &'static str = "Hello";

    const target: &'static str = "World";

    const message: &'static str = greeting + ", " + target + "!";

    println!("{}", message);
}