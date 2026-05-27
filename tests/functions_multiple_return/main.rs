pub fn vals() -> (i32, i32) {

    return (3 as i32, 7 as i32);
}

fn main() {
    let (mut a, mut b) = vals();
    println!("{}", format!("{}", a));
    println!("{}", format!("{}", b));

    let (_, mut c) = vals();
    println!("{}", format!("{}", c));
}