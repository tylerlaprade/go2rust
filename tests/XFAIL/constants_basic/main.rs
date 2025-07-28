fn main() {
    println!("{}", "=== Basic constants ===".to_string());
    print!("Pi = %.5f\n", Pi);
    print!("Euler = %.5f\n", Euler);
    print!("MaxUsers = {}\n", MaxUsers);
    println!("{}", "\n=== Typed constants ===".to_string());
    print!("Name: {}\n", Name);
    print!("Version: %.1f\n", Version);
    print!("Debug: %t\n", Debug);
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
    print!("f = %.2f\n", f);
    print!("mixed = {}\n", mixed);
    println!("{}", "\n=== String constants ===".to_string());
    
    
    
    println!("{}", message);
}