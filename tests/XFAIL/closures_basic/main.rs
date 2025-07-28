pub fn make_counter() -> Unknown {
    let mut count = 0;
    return ;
}

pub fn make_adder(x: i32) -> Unknown {
    return ;
}

pub fn apply_operation(nums: Vec<i32>, op: Unknown) -> Vec<i32> {
    let mut result = vec![0; nums.len()];
    for (i, num) in nums.iter().enumerate() {
        result[i] = op(num);
    }
    return result;
}

fn main() {
    let mut counter = make_counter();
    println!("{} {}", "Counter 1:".to_string(), counter());
    println!("{} {}", "Counter 2:".to_string(), counter());
    println!("{} {}", "Counter 3:".to_string(), counter());
    let mut counter2 = make_counter();
    println!("{} {}", "Counter2 1:".to_string(), counter2());
    println!("{} {}", "Counter 4:".to_string(), counter());
    let mut add5 = make_adder(5);
    let mut add10 = make_adder(10);
    println!("{} {}", "5 + 3 =".to_string(), add5(3));
    println!("{} {}", "10 + 7 =".to_string(), add10(7));
    let mut numbers = vec![1, 2, 3, 4, 5];
    let mut squared = apply_operation(numbers, );
    println!("{} {}", "Squared:".to_string(), squared);
    let mut doubled = apply_operation(numbers, );
    println!("{} {}", "Doubled:".to_string(), doubled);
    let mut multiplier = 3;
    let mut tripled = apply_operation(numbers, );
    println!("{} {}", "Tripled:".to_string(), tripled);
    let mut result = (10, 20);
    println!("{} {}", "Immediate result:".to_string(), result);
}