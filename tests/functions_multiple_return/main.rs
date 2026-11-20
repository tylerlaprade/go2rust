pub fn vals() -> (i64, i64) {
    (3, 7)
}

fn main() {
    let (mut a, mut b) = vals();
    println!("{}", format!("{}", a));
    println!("{}", format!("{}", b));

    let (_, mut c) = vals();
    println!("{}", format!("{}", c));
}