fn main() {
    println!("{} {}", "Program name:".to_string(), (*os.lock().unwrap().as_mut().unwrap()).args[0]);
    println!("{} {}", "Arguments:".to_string(), (*os.lock().unwrap().as_mut().unwrap()).args[1..].to_vec());
    println!("{} {}", "Total args:".to_string(), (*os.lock().unwrap().as_mut().unwrap()).args.len());
}