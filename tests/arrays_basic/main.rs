fn main() {
    let mut arr: [i32; 3] = Default::default();
    arr[0] = 10;
    arr[1] = 20;
    arr[2] = 30;
    println!("{}", "Array elements:".to_string());
    let mut i = 0;
    while i < arr.len() {
        println!("{}", arr[i]);
        i += 1;
    }
    let mut nums = [1, 2, 3, 4];
    println!("{}", "Initialized array:".to_string());
    for (_, num) in nums.iter().enumerate() {
        println!("{}", num);
    }
}