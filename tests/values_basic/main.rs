fn main() {
    println!("{}", format!("{}", format!("{}{}", "go".to_string(), "lang".to_string())));
    println!("{} {}", format!("{}", "1+1 =".to_string()), format!("{}", 1 + 1));
    println!("{} {}", format!("{}", "7.0/3.0 =".to_string()), format!("{}", 7.0 / 3.0));
    println!("{}", format!("{}", true && false));
    println!("{}", format!("{}", true || false));
    println!("{}", format!("{}", !true));
}