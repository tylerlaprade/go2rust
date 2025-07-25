fn increment() {
    
}

fn value() -> i32 {
    return c::value;
}

fn new_counter() -> Unknown {
    return ;
}

fn main() {
    
    counter::increment();
    counter::increment();
    println!("{} {}", "Counter value:", counter::value());
}