fn main() {
    println!("{}", format!("{}", "Hello from fmt.Println".to_string()));
    println!("{} {}", format!("{}", "This call ".to_string()), format!("{}", "has two inputs".to_string()));
}