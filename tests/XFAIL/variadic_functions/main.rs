use std::sync::{Arc, Mutex};

pub fn sum(numbers: Arc<Mutex<Option</* TODO: Unhandled type *ast.Ellipsis */ Arc<Mutex<Option<()>>>>>>) -> Arc<Mutex<Option<i32>>> {

    let mut total = Arc::new(Mutex::new(Some(0)));
    for num in &(*numbers.lock().unwrap().as_mut().unwrap()) {
        { let mut guard = total.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + num); };
    }
    return total.clone();
}

pub fn average(numbers: Arc<Mutex<Option</* TODO: Unhandled type *ast.Ellipsis */ Arc<Mutex<Option<()>>>>>>) -> Arc<Mutex<Option<f64>>> {

    if (*numbers.lock().unwrap().as_ref().unwrap()).len() == 0 {
        return Arc::new(Mutex::new(Some(0)));
    }
    let mut total = Arc::new(Mutex::new(Some(0.0)));
    for num in &(*numbers.lock().unwrap().as_mut().unwrap()) {
        { let mut guard = total.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + num); };
    }
    return {
            let __tmp_x = (*total.lock().unwrap().as_mut().unwrap());
            let __tmp_y = Arc::new(Mutex::new(Some((*(*numbers.lock().unwrap().as_ref().unwrap()).len().lock().unwrap().as_ref().unwrap()) as f64)));
            Arc::new(Mutex::new(Some(__tmp_x / __tmp_y)))
        };
}

pub fn print_strings(prefix: Arc<Mutex<Option<String>>>, strings: Arc<Mutex<Option</* TODO: Unhandled type *ast.Ellipsis */ Arc<Mutex<Option<()>>>>>>) {
    print!("{}: ", (*prefix.lock().unwrap().as_mut().unwrap()));
    for (i, str) in (*strings.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        if i > 0 {
        (*fmt.lock().unwrap().as_mut().unwrap()).print(Arc::new(Mutex::new(Some(", ".to_string()))));
    }
        (*fmt.lock().unwrap().as_mut().unwrap()).print(Arc::new(Mutex::new(Some(str))));
    }
    println!();
}

pub fn min(first: Arc<Mutex<Option<i32>>>, rest: Arc<Mutex<Option</* TODO: Unhandled type *ast.Ellipsis */ Arc<Mutex<Option<()>>>>>>) -> Arc<Mutex<Option<i32>>> {

    let mut minimum = Arc::new(Mutex::new(Some((*first.lock().unwrap().as_mut().unwrap()))));
    for num in &(*rest.lock().unwrap().as_mut().unwrap()) {
        if num < (*minimum.lock().unwrap().as_mut().unwrap()) {
        { let new_val = num; *minimum.lock().unwrap() = Some(new_val); };
    }
    }
    return minimum.clone();
}

pub fn concat(separator: Arc<Mutex<Option<String>>>, strings: Arc<Mutex<Option</* TODO: Unhandled type *ast.Ellipsis */ Arc<Mutex<Option<()>>>>>>) -> Arc<Mutex<Option<String>>> {

    if (*strings.lock().unwrap().as_ref().unwrap()).len() == 0 {
        return Arc::new(Mutex::new(Some("".to_string())));
    }
    let mut result = Arc::new(Mutex::new(Some((*strings.lock().unwrap().as_ref().unwrap())[0 as usize].clone())));
    for str in &Arc::new(Mutex::new(Some((*strings.lock().unwrap().as_ref().unwrap())[1 as usize..].to_vec()))) {
        (*result.lock().unwrap().as_mut().unwrap()).push_str(&(*separator.lock().unwrap().as_mut().unwrap()) + str);
    }
    return result.clone();
}

fn main() {
    println!("{} {}", "Sum of no numbers:".to_string(), (*sum().lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Sum of 1, 2, 3:".to_string(), (*sum(Arc::new(Mutex::new(Some(1))), Arc::new(Mutex::new(Some(2))), Arc::new(Mutex::new(Some(3)))).lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Sum of 1, 2, 3, 4, 5:".to_string(), (*sum(Arc::new(Mutex::new(Some(1))), Arc::new(Mutex::new(Some(2))), Arc::new(Mutex::new(Some(3))), Arc::new(Mutex::new(Some(4))), Arc::new(Mutex::new(Some(5)))).lock().unwrap().as_ref().unwrap()));

    let mut numbers = Arc::new(Mutex::new(Some(vec![10, 20, 30, 40])));
    println!("{} {}", "Sum of slice:".to_string(), (*sum(numbers.clone()).lock().unwrap().as_ref().unwrap()));

    println!("{} {}", "Average of 1.5, 2.5, 3.5:".to_string(), (*average(Arc::new(Mutex::new(Some(1.5))), Arc::new(Mutex::new(Some(2.5))), Arc::new(Mutex::new(Some(3.5)))).lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Average of no numbers:".to_string(), (*average().lock().unwrap().as_ref().unwrap()));

    print_strings(Arc::new(Mutex::new(Some("Colors".to_string()))), Arc::new(Mutex::new(Some("red".to_string()))), Arc::new(Mutex::new(Some("green".to_string()))), Arc::new(Mutex::new(Some("blue".to_string()))));
    print_strings(Arc::new(Mutex::new(Some("Animals".to_string()))), Arc::new(Mutex::new(Some("cat".to_string()))), Arc::new(Mutex::new(Some("dog".to_string()))));
    print_strings(Arc::new(Mutex::new(Some("Empty".to_string()))));

    println!("{} {}", "Min of 5, 2, 8, 1, 9:".to_string(), (*min(Arc::new(Mutex::new(Some(5))), Arc::new(Mutex::new(Some(2))), Arc::new(Mutex::new(Some(8))), Arc::new(Mutex::new(Some(1))), Arc::new(Mutex::new(Some(9)))).lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Min of just 42:".to_string(), (*min(Arc::new(Mutex::new(Some(42)))).lock().unwrap().as_ref().unwrap()));

    println!("{} {}", "Concat with comma:".to_string(), (*concat(Arc::new(Mutex::new(Some(", ".to_string()))), Arc::new(Mutex::new(Some("apple".to_string()))), Arc::new(Mutex::new(Some("banana".to_string()))), Arc::new(Mutex::new(Some("cherry".to_string())))).lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Concat with dash:".to_string(), (*concat(Arc::new(Mutex::new(Some(" - ".to_string()))), Arc::new(Mutex::new(Some("one".to_string()))), Arc::new(Mutex::new(Some("two".to_string()))), Arc::new(Mutex::new(Some("three".to_string())))).lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Concat empty:".to_string(), (*concat(Arc::new(Mutex::new(Some(", ".to_string())))).lock().unwrap().as_ref().unwrap()));

    let mut words = Arc::new(Mutex::new(Some(vec!["hello".to_string(), "world".to_string(), "from".to_string(), "go".to_string()])));
    println!("{} {}", "Concat from slice:".to_string(), (*concat(Arc::new(Mutex::new(Some(" ".to_string()))), words.clone()).lock().unwrap().as_ref().unwrap()));
}