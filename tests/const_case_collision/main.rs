pub(crate) const FOO_BAR_1: i32 = 5;


pub const FOO_BAR: i32 = FOO_BAR_1;


fn main() {
    println!("{} {}", format!("{}", FOO_BAR_1), format!("{}", FOO_BAR));
}