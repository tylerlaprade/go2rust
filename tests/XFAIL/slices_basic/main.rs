fn main() {
    let mut slice = vec![1, 2, 3, 4, 5];
    println!("{} {}", "Original slice:".to_string(), slice);
    slice = {slice.extend(vec![6, 7]); slice};
    println!("{} {}", "After append:".to_string(), slice);
    let mut subSlice = slice[1..4].to_vec();
    println!("{} {}", "Sub-slice [1:4]:".to_string(), subSlice);
    println!("{} {}", "Length:".to_string(), slice.len());
    println!("{} {}", "Capacity:".to_string(), slice.capacity());
    let mut made = vec![0; 3];
    made[0] = 10;
    made[1] = 20;
    made[2] = 30;
    println!("{} {}", "Made slice:".to_string(), made);
}