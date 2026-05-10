pub(crate) const DEBUG: bool = false;


fn main() {
    if DEBUG {
        println!("{}", "debug".to_string());
    } else {
        println!("{}", "release".to_string());
    }
}