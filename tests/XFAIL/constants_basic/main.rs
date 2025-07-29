const PI: f64 = 3.14159;
const EULER: f64 = 2.71828;
const MAX_USERS: i32 = 100;


const NAME: String = "Go2Rust";
const VERSION: f64 = 1.0;
const DEBUG: bool = true;


const SUNDAY: i32 = 0;
const MONDAY: i32 = 1;
const TUESDAY: i32 = 2;
const WEDNESDAY: i32 = 3;
const THURSDAY: i32 = 4;
const FRIDAY: i32 = 5;
const SATURDAY: i32 = 6;


const K_B: i32 = 1 << (10 * 2);
const M_B: i32 = 3;
const G_B: i32 = 4;
const T_B: i32 = 5;


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
    print!("Pi = %.5f\n", Pi);
    print!("Euler = %.5f\n", Euler);
    print!("MaxUsers = {}\n", MaxUsers);
    println!("{}", "\n=== Typed constants ===".to_string());
    print!("Name: {}\n", Name);
    print!("Version: {:.1}\n", Version);
    print!("Debug: {}\n", Debug);
    println!("{}", "\n=== Weekday constants ===".to_string());
    print!("Sunday = {}\n", Sunday);
    print!("Monday = {}\n", Monday);
    print!("Wednesday = {}\n", Wednesday);
    print!("Saturday = {}\n", Saturday);
    println!("{}", "\n=== Size constants ===".to_string());
    print!("KB = {} bytes\n", KB);
    print!("MB = {} bytes\n", MB);
    print!("GB = {} bytes\n", GB);
    print!("TB = {} bytes\n", TB);
    println!("{}", "\n=== Color constants ===".to_string());
    print!("Red = {}\n", Red);
    print!("Green = {}\n", Green);
    print!("Blue = {}\n", Blue);
    println!("{}", "\n=== Complex iota expressions ===".to_string());
    print!("A = {}\n", A);
    print!("B = {}\n", B);
    print!("C = {}\n", C);
    print!("D = {}\n", D);
    print!("EE = {}\n", EE);
    print!("F = {}\n", F);
    println!("{}", "\n=== Local constants ===".to_string());
    
    
    print!("localConst = {}\n", localConst);
    print!("x = {}, y = {}, z = {}\n", x, y, z);
    println!("{}", "\n=== Untyped constants in expressions ===".to_string());
    
    
    let mut i = untypedInt;
    let mut f = untypedFloat;
    let mut mixed = untypedInt + 3;
    print!("i = {}\n", i);
    print!("f = {:.2}\n", f);
    print!("mixed = {}\n", mixed);
    println!("{}", "\n=== String constants ===".to_string());
    
    
    
    println!("{}", message);
}