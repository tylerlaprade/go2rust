fn main() {
    let mut ages = std::collections::HashMap::<String, i32>::new();
    ages.insert("Alice".to_string(), 25);
    ages.insert("Bob".to_string(), 30);
    ages.insert("Charlie".to_string(), 35);
    println!("{} {:?}", "Ages map:".to_string(), ages);
    let mut colors = std::collections::HashMap::<String, String>::from([("red".to_string(), "#FF0000".to_string()), ("green".to_string(), "#00FF00".to_string()), ("blue".to_string(), "#0000FF".to_string())]);
    println!("{} {:?}", "Colors map:".to_string(), colors);
    let (mut age, mut exists) = (ages.get(&"Alice".to_string()).cloned().unwrap_or_default(), ages.contains_key(&"Alice".to_string()));
    if exists {
        println!("{} {}", "Alice's age:".to_string(), age);
    }
    ages.remove(&"Bob".to_string());
    println!("{} {:?}", "After deleting Bob:".to_string(), ages);
    println!("{}", "All colors:".to_string());
    for (name, hex) in &colors {
        println!("{} {} {}", name, "->".to_string(), hex);
    }
}