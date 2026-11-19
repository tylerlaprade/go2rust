fn main() {
    println!("{} {} {}", format!("{}", 1), format!("{}", !3), format!("{}", 7 & ! 3));
    println!("{}", format!("{}", !0 as u64));
}