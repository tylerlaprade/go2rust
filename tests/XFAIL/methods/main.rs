pub fn increment() {
    c::value += 1;
}

pub fn value() -> i32 {
    return c::value;
}

pub fn new_counter() -> Unknown {
    return ;
}

fn main() {
    let mut counter = new_counter();
    counter::increment();
    counter::increment();
    println!("{} {}", "Counter value:".to_string(), counter::value());
}