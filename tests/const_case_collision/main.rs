pub(crate) const FOO_BAR_1: i64 = 5;


pub const FOO_BAR: i64 = FOO_BAR_1;


fn main() {
    println!("{} {}", format!("{}", FOO_BAR_1), format!("{}", FOO_BAR));
}