mod math;
mod string;
use math::*;
use string::*;

fn main() {
    let mut sum = add(5, 3);
    print!("5 + 3 = {}\n", sum);
    let mut product = multiply(4, 7);
    print!("4 * 7 = {}\n", product);
    let mut repeated = repeat("Go".to_string(), 3);
    print!("Repeat(\"Go\", 3) = {}\n", repeated);
}