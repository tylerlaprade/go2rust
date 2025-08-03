fn format_map<K: std::fmt::Display + std::cmp::Ord + Clone, V>(map: &std::sync::Arc<std::sync::Mutex<Option<std::collections::HashMap<K, std::sync::Arc<std::sync::Mutex<Option<V>>>>>>>) -> String 
where
    V: std::fmt::Display,
{
    let guard = map.lock().unwrap();
    if let Some(ref m) = *guard {
        let mut items: Vec<_> = m.iter().collect();
        items.sort_by_key(|(k, _)| (*k).clone());
        
        let formatted: Vec<String> = items
            .into_iter()
            .map(|(k, v)| {
                let v_guard = v.lock().unwrap();
                if let Some(ref val) = *v_guard {
                    format!("{}:{}", k, val)
                } else {
                    format!("{}:<nil>", k)
                }
            })
            .collect();
        
        format!("map[{}]", formatted.join(" "))
    } else {
        "map[]".to_string()
    }
}
fn format_slice<T>(slice: &std::sync::Arc<std::sync::Mutex<Option<Vec<T>>>>) -> String 
where
    T: std::fmt::Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

pub fn sum(numbers: std::sync::Arc<std::sync::Mutex<Option</* TODO: Unhandled type *ast.Ellipsis */ std::sync::Arc<std::sync::Mutex<Option<()>>>>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {

    let mut total = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    for num in &(*numbers.lock().unwrap().as_mut().unwrap()) {
        { let mut guard = total.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + num); };
    }
    return total.clone();
}

pub fn average(numbers: std::sync::Arc<std::sync::Mutex<Option</* TODO: Unhandled type *ast.Ellipsis */ std::sync::Arc<std::sync::Mutex<Option<()>>>>>>) -> std::sync::Arc<std::sync::Mutex<Option<f64>>> {

    if (*numbers.lock().unwrap().as_mut().unwrap()).len() == 0 {
        return std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    }
    let mut total = std::sync::Arc::new(std::sync::Mutex::new(Some(0.0)));
    for num in &(*numbers.lock().unwrap().as_mut().unwrap()) {
        { let mut guard = total.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + num); };
    }
    return std::sync::Arc::new(std::sync::Mutex::new(Some((*total.lock().unwrap().as_mut().unwrap()) / (float64.lock().unwrap().as_ref().unwrap())(std::sync::Arc::new(std::sync::Mutex::new(Some((*numbers.lock().unwrap().as_mut().unwrap()).len())))))));
}

pub fn print_strings(prefix: std::sync::Arc<std::sync::Mutex<Option<String>>>, strings: std::sync::Arc<std::sync::Mutex<Option</* TODO: Unhandled type *ast.Ellipsis */ std::sync::Arc<std::sync::Mutex<Option<()>>>>>>) {
    print!("{}: ", (*prefix.lock().unwrap().as_mut().unwrap()));
    for (i, str) in (*strings.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        if i > 0 {
        (*fmt.lock().unwrap().as_mut().unwrap()).print(std::sync::Arc::new(std::sync::Mutex::new(Some(", ".to_string()))));
    }
        (*fmt.lock().unwrap().as_mut().unwrap()).print(std::sync::Arc::new(std::sync::Mutex::new(Some(str))));
    }
    println!();
}

pub fn min(first: std::sync::Arc<std::sync::Mutex<Option<i32>>>, rest: std::sync::Arc<std::sync::Mutex<Option</* TODO: Unhandled type *ast.Ellipsis */ std::sync::Arc<std::sync::Mutex<Option<()>>>>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {

    let mut minimum = std::sync::Arc::new(std::sync::Mutex::new(Some((*first.lock().unwrap().as_mut().unwrap()))));
    for num in &(*rest.lock().unwrap().as_mut().unwrap()) {
        if num < (*minimum.lock().unwrap().as_mut().unwrap()) {
        { let new_val = num; *minimum.lock().unwrap() = Some(new_val); };
    }
    }
    return minimum.clone();
}

pub fn concat(separator: std::sync::Arc<std::sync::Mutex<Option<String>>>, strings: std::sync::Arc<std::sync::Mutex<Option</* TODO: Unhandled type *ast.Ellipsis */ std::sync::Arc<std::sync::Mutex<Option<()>>>>>>) -> std::sync::Arc<std::sync::Mutex<Option<String>>> {

    if (*strings.lock().unwrap().as_mut().unwrap()).len() == 0 {
        return std::sync::Arc::new(std::sync::Mutex::new(Some("".to_string())));
    }
    let mut result = std::sync::Arc::new(std::sync::Mutex::new(Some((*strings.lock().unwrap().as_mut().unwrap())[0])));
    for str in &(*strings.lock().unwrap().as_mut().unwrap())[1..].to_vec() {
        (*result.lock().unwrap().as_mut().unwrap()).push_str(&(*separator.lock().unwrap().as_mut().unwrap()) + str);
    }
    return result.clone();
}

fn main() {
    println!("{} {}", "Sum of no numbers:".to_string(), (*sum().lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "Sum of 1, 2, 3:".to_string(), (*sum(std::sync::Arc::new(std::sync::Mutex::new(Some(1))), std::sync::Arc::new(std::sync::Mutex::new(Some(2))), std::sync::Arc::new(std::sync::Mutex::new(Some(3)))).lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "Sum of 1, 2, 3, 4, 5:".to_string(), (*sum(std::sync::Arc::new(std::sync::Mutex::new(Some(1))), std::sync::Arc::new(std::sync::Mutex::new(Some(2))), std::sync::Arc::new(std::sync::Mutex::new(Some(3))), std::sync::Arc::new(std::sync::Mutex::new(Some(4))), std::sync::Arc::new(std::sync::Mutex::new(Some(5)))).lock().unwrap().as_mut().unwrap()));

    let mut numbers = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![10, 20, 30, 40])));
    println!("{} {}", "Sum of slice:".to_string(), (*sum(numbers.clone()).lock().unwrap().as_mut().unwrap()));

    println!("{} {}", "Average of 1.5, 2.5, 3.5:".to_string(), (*average(std::sync::Arc::new(std::sync::Mutex::new(Some(1.5))), std::sync::Arc::new(std::sync::Mutex::new(Some(2.5))), std::sync::Arc::new(std::sync::Mutex::new(Some(3.5)))).lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "Average of no numbers:".to_string(), (*average().lock().unwrap().as_mut().unwrap()));

    print_strings(std::sync::Arc::new(std::sync::Mutex::new(Some("Colors".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some("red".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some("green".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some("blue".to_string()))));
    print_strings(std::sync::Arc::new(std::sync::Mutex::new(Some("Animals".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some("cat".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some("dog".to_string()))));
    print_strings(std::sync::Arc::new(std::sync::Mutex::new(Some("Empty".to_string()))));

    println!("{} {}", "Min of 5, 2, 8, 1, 9:".to_string(), (*min(std::sync::Arc::new(std::sync::Mutex::new(Some(5))), std::sync::Arc::new(std::sync::Mutex::new(Some(2))), std::sync::Arc::new(std::sync::Mutex::new(Some(8))), std::sync::Arc::new(std::sync::Mutex::new(Some(1))), std::sync::Arc::new(std::sync::Mutex::new(Some(9)))).lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "Min of just 42:".to_string(), (*min(std::sync::Arc::new(std::sync::Mutex::new(Some(42)))).lock().unwrap().as_mut().unwrap()));

    println!("{} {}", "Concat with comma:".to_string(), (*concat(std::sync::Arc::new(std::sync::Mutex::new(Some(", ".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some("apple".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some("banana".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some("cherry".to_string())))).lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "Concat with dash:".to_string(), (*concat(std::sync::Arc::new(std::sync::Mutex::new(Some(" - ".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some("one".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some("two".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some("three".to_string())))).lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "Concat empty:".to_string(), (*concat(std::sync::Arc::new(std::sync::Mutex::new(Some(", ".to_string())))).lock().unwrap().as_mut().unwrap()));

    let mut words = std::sync::Arc::new(std::sync::Mutex::new(Some(vec!["hello".to_string(), "world".to_string(), "from".to_string(), "go".to_string()])));
    println!("{} {}", "Concat from slice:".to_string(), (*concat(std::sync::Arc::new(std::sync::Mutex::new(Some(" ".to_string()))), words.clone()).lock().unwrap().as_mut().unwrap()));
}