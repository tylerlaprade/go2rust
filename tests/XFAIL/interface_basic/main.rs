pub fn print_any(v: Box<dyn std::any::Any>) {
    println!("{} {}", "Value:".to_string(), v);
}

fn main() {
    let mut x;
    x = 42;
    println!("{} {}", "x is int:".to_string(), x);
    print_any(x);
    x = "hello".to_string();
    println!("{} {}", "x is string:".to_string(), x);
    print_any(x);
    x = 3.14;
    println!("{} {}", "x is float:".to_string(), x);
    print_any(x);
    let mut values = vec![1, "two".to_string(), 3.0];
    println!("{} {}", "Mixed values:".to_string(), values);
}