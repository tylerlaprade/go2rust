fn main() {
    eprintln!("{}", "This goes to stderr".to_string());
    let mut s = "hello".to_string();
    eprintln!("{}", s.len());
}