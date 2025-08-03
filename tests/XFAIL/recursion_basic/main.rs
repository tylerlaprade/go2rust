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

pub fn factorial(n: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {

    if (*n.lock().unwrap().as_mut().unwrap()) <= 1 {
        return std::sync::Arc::new(std::sync::Mutex::new(Some(1)));
    }
    return std::sync::Arc::new(std::sync::Mutex::new(Some((*n.lock().unwrap().as_mut().unwrap()) * factorial(std::sync::Arc::new(std::sync::Mutex::new(Some((*n.lock().unwrap().as_mut().unwrap()) - 1)))))));
}

pub fn fibonacci(n: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {

    if (*n.lock().unwrap().as_mut().unwrap()) <= 1 {
        return n.clone();
    }
    return std::sync::Arc::new(std::sync::Mutex::new(Some(fibonacci(std::sync::Arc::new(std::sync::Mutex::new(Some((*n.lock().unwrap().as_mut().unwrap()) - 1)))) + fibonacci(std::sync::Arc::new(std::sync::Mutex::new(Some((*n.lock().unwrap().as_mut().unwrap()) - 2)))))));
}

pub fn gcd(a: std::sync::Arc<std::sync::Mutex<Option<i32>>>, b: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {

    if (*b.lock().unwrap().as_mut().unwrap()) == 0 {
        return a.clone();
    }
    return std::sync::Arc::new(std::sync::Mutex::new(Some(gcd(b.clone(), std::sync::Arc::new(std::sync::Mutex::new(Some((*a.lock().unwrap().as_mut().unwrap()) % (*b.lock().unwrap().as_mut().unwrap()))))))));
}

pub fn power(base: std::sync::Arc<std::sync::Mutex<Option<i32>>>, exp: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {

    if (*exp.lock().unwrap().as_mut().unwrap()) == 0 {
        return std::sync::Arc::new(std::sync::Mutex::new(Some(1)));
    }
    if (*exp.lock().unwrap().as_mut().unwrap()) == 1 {
        return base.clone();
    }
    if (*exp.lock().unwrap().as_mut().unwrap()) % 2 == 0 {
        let mut half = power(base.clone(), std::sync::Arc::new(std::sync::Mutex::new(Some((*exp.lock().unwrap().as_mut().unwrap()) / 2))));
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*half.lock().unwrap().as_mut().unwrap()) * (*half.lock().unwrap().as_mut().unwrap()))));
    }
    return std::sync::Arc::new(std::sync::Mutex::new(Some((*base.lock().unwrap().as_mut().unwrap()) * power(base.clone(), std::sync::Arc::new(std::sync::Mutex::new(Some((*exp.lock().unwrap().as_mut().unwrap()) - 1)))))));
}

pub fn sum_array(arr: std::sync::Arc<std::sync::Mutex<Option<Vec<i32>>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {

    if (*arr.lock().unwrap().as_mut().unwrap()).len() == 0 {
        return std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    }
    if (*arr.lock().unwrap().as_mut().unwrap()).len() == 1 {
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*arr.lock().unwrap().as_mut().unwrap())[0])));
    }
    return std::sync::Arc::new(std::sync::Mutex::new(Some((*arr.lock().unwrap().as_mut().unwrap())[0] + sum_array(std::sync::Arc::new(std::sync::Mutex::new(Some((*arr.lock().unwrap().as_mut().unwrap())[1..].to_vec())))))));
}

pub fn reverse_string(s: std::sync::Arc<std::sync::Mutex<Option<String>>>) -> std::sync::Arc<std::sync::Mutex<Option<String>>> {

    if (*s.lock().unwrap().as_mut().unwrap()).len() <= 1 {
        return s.clone();
    }
    return std::sync::Arc::new(std::sync::Mutex::new(Some(reverse_string(std::sync::Arc::new(std::sync::Mutex::new(Some((*s.lock().unwrap().as_mut().unwrap())[1..].to_vec())))) + (string.lock().unwrap().as_ref().unwrap())(std::sync::Arc::new(std::sync::Mutex::new(Some((*s.lock().unwrap().as_mut().unwrap())[0])))))));
}

fn main() {
    println!("{} {}", "Factorial of 5:".to_string(), (*factorial(std::sync::Arc::new(std::sync::Mutex::new(Some(5)))).lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "Factorial of 0:".to_string(), (*factorial(std::sync::Arc::new(std::sync::Mutex::new(Some(0)))).lock().unwrap().as_mut().unwrap()));

    println!("{}", "Fibonacci sequence:".to_string());
    let mut i = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_mut().unwrap()) < 10 {
        print!("fib({}) = {}\n", (*i.lock().unwrap().as_mut().unwrap()), (*fibonacci(i.clone()).lock().unwrap().as_mut().unwrap()));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    println!("{} {}", "GCD of 48 and 18:".to_string(), (*gcd(std::sync::Arc::new(std::sync::Mutex::new(Some(48))), std::sync::Arc::new(std::sync::Mutex::new(Some(18)))).lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "GCD of 17 and 13:".to_string(), (*gcd(std::sync::Arc::new(std::sync::Mutex::new(Some(17))), std::sync::Arc::new(std::sync::Mutex::new(Some(13)))).lock().unwrap().as_mut().unwrap()));

    println!("{} {}", "2^8 =".to_string(), (*power(std::sync::Arc::new(std::sync::Mutex::new(Some(2))), std::sync::Arc::new(std::sync::Mutex::new(Some(8)))).lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "3^4 =".to_string(), (*power(std::sync::Arc::new(std::sync::Mutex::new(Some(3))), std::sync::Arc::new(std::sync::Mutex::new(Some(4)))).lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "5^0 =".to_string(), (*power(std::sync::Arc::new(std::sync::Mutex::new(Some(5))), std::sync::Arc::new(std::sync::Mutex::new(Some(0)))).lock().unwrap().as_mut().unwrap()));

    let mut numbers = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![1, 2, 3, 4, 5])));
    println!("{} {} {} {}", "Sum of".to_string(), format_slice(&numbers), "=".to_string(), (*sum_array(numbers.clone()).lock().unwrap().as_mut().unwrap()));

    let mut original = std::sync::Arc::new(std::sync::Mutex::new(Some("hello".to_string())));
    let mut reversed = reverse_string(original.clone());
    print!("'{}' reversed is '{}'\n", (*original.lock().unwrap().as_mut().unwrap()), (*reversed.lock().unwrap().as_mut().unwrap()));
}