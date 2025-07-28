fn main() {
    println!("{}", "Testing multiple imports".to_string());
    let mut upper = "hello".to_string().to_uppercase();
    println!("{} {}", "Upper:".to_string(), upper);
    let mut num = 42;
    let mut str = num.to_string();
    println!("{} {}", "Number as string:".to_string(), str);
    let mut lower = "WORLD".to_string().to_lowercase();
    println!("{} {}", "Lower:".to_string(), lower);
}