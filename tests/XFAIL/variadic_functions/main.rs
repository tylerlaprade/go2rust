pub fn sum(numbers: Unknown) -> i32 {
    let mut total = 0;
    for (_, num) in numbers.iter().enumerate() {
        total.push_str(&num);
    }
    return total;
}

pub fn average(numbers: Unknown) -> f64 {
    
    let mut total = 0.0;
    for (_, num) in numbers.iter().enumerate() {
        total.push_str(&num);
    }
    return total / float64(numbers.len());
}

pub fn print_strings(prefix: String, strings: Unknown) {
    print!("{}: ", prefix);
    for (i, str) in strings.iter().enumerate() {
        
        fmt.print(str);
    }
    println!();
}

pub fn min(first: i32, rest: Unknown) -> i32 {
    let mut minimum = first;
    for (_, num) in rest.iter().enumerate() {
        
    }
    return minimum;
}

pub fn concat(separator: String, strings: Unknown) -> String {
    
    let mut result = strings[0];
    for (_, str) in strings[1..].to_vec().iter().enumerate() {
        result.push_str(&separator + str);
    }
    return result;
}

fn main() {
    println!("{} {}", "Sum of no numbers:".to_string(), sum());
    println!("{} {}", "Sum of 1, 2, 3:".to_string(), sum(1, 2, 3));
    println!("{} {}", "Sum of 1, 2, 3, 4, 5:".to_string(), sum(1, 2, 3, 4, 5));
    let mut numbers = vec![10, 20, 30, 40];
    println!("{} {}", "Sum of slice:".to_string(), sum(numbers));
    println!("{} {}", "Average of 1.5, 2.5, 3.5:".to_string(), average(1.5, 2.5, 3.5));
    println!("{} {}", "Average of no numbers:".to_string(), average());
    print_strings("Colors".to_string(), "red".to_string(), "green".to_string(), "blue".to_string());
    print_strings("Animals".to_string(), "cat".to_string(), "dog".to_string());
    print_strings("Empty".to_string());
    println!("{} {}", "Min of 5, 2, 8, 1, 9:".to_string(), min(5, 2, 8, 1, 9));
    println!("{} {}", "Min of just 42:".to_string(), min(42));
    println!("{} {}", "Concat with comma:".to_string(), concat(", ".to_string(), "apple".to_string(), "banana".to_string(), "cherry".to_string()));
    println!("{} {}", "Concat with dash:".to_string(), concat(" - ".to_string(), "one".to_string(), "two".to_string(), "three".to_string()));
    println!("{} {}", "Concat empty:".to_string(), concat(", ".to_string()));
    let mut words = vec!["hello".to_string(), "world".to_string(), "from".to_string(), "go".to_string()];
    println!("{} {}", "Concat from slice:".to_string(), concat(" ".to_string(), words));
}