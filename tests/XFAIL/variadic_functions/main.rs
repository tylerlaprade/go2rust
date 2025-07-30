pub fn sum(numbers: std::sync::Arc<std::sync::Mutex<Option<Unknown>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {

    let mut total = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    for (_, num) in (*numbers.lock().unwrap().as_ref().unwrap()).iter().enumerate() {
        (*total.lock().unwrap().as_ref().unwrap()) += num;
    }
    return std::sync::Arc::new(std::sync::Mutex::new(Some((*total.lock().unwrap().as_ref().unwrap()).clone())));
}

pub fn average(numbers: std::sync::Arc<std::sync::Mutex<Option<Unknown>>>) -> std::sync::Arc<std::sync::Mutex<Option<f64>>> {

    if (*numbers.lock().unwrap().as_ref().unwrap()).len() == 0 {
        return std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    }
    let mut total = std::sync::Arc::new(std::sync::Mutex::new(Some(0.0)));
    for (_, num) in (*numbers.lock().unwrap().as_ref().unwrap()).iter().enumerate() {
        (*total.lock().unwrap().as_ref().unwrap()) += num;
    }
    return std::sync::Arc::new(std::sync::Mutex::new(Some((*total.lock().unwrap().as_ref().unwrap()) / float64(std::sync::Arc::new(std::sync::Mutex::new(Some((*numbers.lock().unwrap().as_ref().unwrap()).len())))))));
}

pub fn print_strings(prefix: std::sync::Arc<std::sync::Mutex<Option<String>>>, strings: std::sync::Arc<std::sync::Mutex<Option<Unknown>>>) {
    print!("{}: ", (*prefix.lock().unwrap().as_ref().unwrap()));
    for (i, str) in (*strings.lock().unwrap().as_ref().unwrap()).iter().enumerate() {
        if i > 0 {
        (*fmt.lock().unwrap().as_ref().unwrap()).print(std::sync::Arc::new(std::sync::Mutex::new(Some(", ".to_string()))));
    }
        (*fmt.lock().unwrap().as_ref().unwrap()).print(std::sync::Arc::new(std::sync::Mutex::new(Some(str))));
    }
    println!();
}

pub fn min(first: std::sync::Arc<std::sync::Mutex<Option<i32>>>, rest: std::sync::Arc<std::sync::Mutex<Option<Unknown>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {

    let mut minimum = std::sync::Arc::new(std::sync::Mutex::new(Some((*first.lock().unwrap().as_ref().unwrap()))));
    for (_, num) in (*rest.lock().unwrap().as_ref().unwrap()).iter().enumerate() {
        if num < (*minimum.lock().unwrap().as_ref().unwrap()) {
        { let new_val = num; *minimum.lock().unwrap() = Some(new_val); };
    }
    }
    return std::sync::Arc::new(std::sync::Mutex::new(Some((*minimum.lock().unwrap().as_ref().unwrap()).clone())));
}

pub fn concat(separator: std::sync::Arc<std::sync::Mutex<Option<String>>>, strings: std::sync::Arc<std::sync::Mutex<Option<Unknown>>>) -> std::sync::Arc<std::sync::Mutex<Option<String>>> {

    if (*strings.lock().unwrap().as_ref().unwrap()).len() == 0 {
        return std::sync::Arc::new(std::sync::Mutex::new(Some("".to_string())));
    }
    let mut result = std::sync::Arc::new(std::sync::Mutex::new(Some((*strings.lock().unwrap().as_ref().unwrap())[0])));
    for (_, str) in (*strings.lock().unwrap().as_ref().unwrap())[1..].to_vec().iter().enumerate() {
        (*result.lock().unwrap().as_mut().unwrap()).push_str(&(*separator.lock().unwrap().as_ref().unwrap()) + str);
    }
    return std::sync::Arc::new(std::sync::Mutex::new(Some((*result.lock().unwrap().as_ref().unwrap()).clone())));
}

fn main() {
    println!("{} {}", "Sum of no numbers:".to_string(), (*sum().lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Sum of 1, 2, 3:".to_string(), (*sum(std::sync::Arc::new(std::sync::Mutex::new(Some(1))), std::sync::Arc::new(std::sync::Mutex::new(Some(2))), std::sync::Arc::new(std::sync::Mutex::new(Some(3)))).lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Sum of 1, 2, 3, 4, 5:".to_string(), (*sum(std::sync::Arc::new(std::sync::Mutex::new(Some(1))), std::sync::Arc::new(std::sync::Mutex::new(Some(2))), std::sync::Arc::new(std::sync::Mutex::new(Some(3))), std::sync::Arc::new(std::sync::Mutex::new(Some(4))), std::sync::Arc::new(std::sync::Mutex::new(Some(5)))).lock().unwrap().as_ref().unwrap()));
    let mut numbers = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![10, 20, 30, 40])));
    println!("{} {}", "Sum of slice:".to_string(), (*sum(std::sync::Arc::new(std::sync::Mutex::new(Some((*numbers.lock().unwrap().as_ref().unwrap()))))).lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Average of 1.5, 2.5, 3.5:".to_string(), (*average(std::sync::Arc::new(std::sync::Mutex::new(Some(1.5))), std::sync::Arc::new(std::sync::Mutex::new(Some(2.5))), std::sync::Arc::new(std::sync::Mutex::new(Some(3.5)))).lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Average of no numbers:".to_string(), (*average().lock().unwrap().as_ref().unwrap()));
    print_strings(std::sync::Arc::new(std::sync::Mutex::new(Some("Colors".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some("red".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some("green".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some("blue".to_string()))));
    print_strings(std::sync::Arc::new(std::sync::Mutex::new(Some("Animals".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some("cat".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some("dog".to_string()))));
    print_strings(std::sync::Arc::new(std::sync::Mutex::new(Some("Empty".to_string()))));
    println!("{} {}", "Min of 5, 2, 8, 1, 9:".to_string(), (*min(std::sync::Arc::new(std::sync::Mutex::new(Some(5))), std::sync::Arc::new(std::sync::Mutex::new(Some(2))), std::sync::Arc::new(std::sync::Mutex::new(Some(8))), std::sync::Arc::new(std::sync::Mutex::new(Some(1))), std::sync::Arc::new(std::sync::Mutex::new(Some(9)))).lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Min of just 42:".to_string(), (*min(std::sync::Arc::new(std::sync::Mutex::new(Some(42)))).lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Concat with comma:".to_string(), (*concat(std::sync::Arc::new(std::sync::Mutex::new(Some(", ".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some("apple".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some("banana".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some("cherry".to_string())))).lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Concat with dash:".to_string(), (*concat(std::sync::Arc::new(std::sync::Mutex::new(Some(" - ".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some("one".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some("two".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some("three".to_string())))).lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Concat empty:".to_string(), (*concat(std::sync::Arc::new(std::sync::Mutex::new(Some(", ".to_string())))).lock().unwrap().as_ref().unwrap()));
    let mut words = std::sync::Arc::new(std::sync::Mutex::new(Some(vec!["hello".to_string(), "world".to_string(), "from".to_string(), "go".to_string()])));
    println!("{} {}", "Concat from slice:".to_string(), (*concat(std::sync::Arc::new(std::sync::Mutex::new(Some(" ".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some((*words.lock().unwrap().as_ref().unwrap()))))).lock().unwrap().as_ref().unwrap()));
}