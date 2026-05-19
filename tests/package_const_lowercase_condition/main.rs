pub(crate) const DEBUG: bool = false;


fn main() {
    if DEBUG {
        println!("{}", format!("{}", "debug".to_string()));
    } else {
        println!("{}", format!("{}", "release".to_string()));
    }
}