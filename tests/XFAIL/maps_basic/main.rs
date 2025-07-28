fn main() {
    let mut ages = std::collections::HashMap::<String, i32>::new();
    ages["Alice".to_string()] = 25;
    ages["Bob".to_string()] = 30;
    ages["Charlie".to_string()] = 35;
    println!("{} {}", "Ages map:".to_string(), ages);
    let mut colors = std::collections::HashMap::<String, String>::from([("red".to_string(), "#FF0000".to_string()), ("green".to_string(), "#00FF00".to_string()), ("blue".to_string(), "#0000FF".to_string())]);
    println!("{} {}", "Colors map:".to_string(), colors);
    let mut age, let mut exists = ages["Alice".to_string()];
    
    ages.remove(&"Bob".to_string());
    println!("{} {}", "After deleting Bob:".to_string(), ages);
    println!("{}", "All colors:".to_string());
    for (name, hex) in colors.iter().enumerate() {
        println!("{} {} {}", name, "->".to_string(), hex);
    }
}