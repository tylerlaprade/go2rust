pub fn factorial(n: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {

    if (*n.lock().unwrap().as_mut().unwrap()) <= 1 {
        return std::sync::Arc::new(std::sync::Mutex::new(Some(1)));
    }
    return std::sync::Arc::new(std::sync::Mutex::new(Some((*n.lock().unwrap().as_mut().unwrap()) * factorial(std::sync::Arc::new(std::sync::Mutex::new(Some((*n.lock().unwrap().as_mut().unwrap()) - 1)))))));
}

pub fn fibonacci(n: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {

    if (*n.lock().unwrap().as_mut().unwrap()) <= 1 {
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*n.lock().unwrap().as_mut().unwrap()).clone())));
    }
    return std::sync::Arc::new(std::sync::Mutex::new(Some(fibonacci(std::sync::Arc::new(std::sync::Mutex::new(Some((*n.lock().unwrap().as_mut().unwrap()) - 1)))) + fibonacci(std::sync::Arc::new(std::sync::Mutex::new(Some((*n.lock().unwrap().as_mut().unwrap()) - 2)))))));
}

pub fn gcd(a: std::sync::Arc<std::sync::Mutex<Option<i32>>>, b: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {

    if (*b.lock().unwrap().as_mut().unwrap()) == 0 {
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*a.lock().unwrap().as_mut().unwrap()).clone())));
    }
    return std::sync::Arc::new(std::sync::Mutex::new(Some(gcd(std::sync::Arc::new(std::sync::Mutex::new(Some((*b.lock().unwrap().as_mut().unwrap())))), std::sync::Arc::new(std::sync::Mutex::new(Some((*a.lock().unwrap().as_mut().unwrap()) % (*b.lock().unwrap().as_mut().unwrap()))))))));
}

pub fn power(base: std::sync::Arc<std::sync::Mutex<Option<i32>>>, exp: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {

    if (*exp.lock().unwrap().as_mut().unwrap()) == 0 {
        return std::sync::Arc::new(std::sync::Mutex::new(Some(1)));
    }
    if (*exp.lock().unwrap().as_mut().unwrap()) == 1 {
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*base.lock().unwrap().as_mut().unwrap()).clone())));
    }
    if (*exp.lock().unwrap().as_mut().unwrap()) % 2 == 0 {
        let mut half = power(std::sync::Arc::new(std::sync::Mutex::new(Some((*base.lock().unwrap().as_mut().unwrap())))), std::sync::Arc::new(std::sync::Mutex::new(Some((*exp.lock().unwrap().as_mut().unwrap()) / 2))));
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*half.lock().unwrap().as_mut().unwrap()) * (*half.lock().unwrap().as_mut().unwrap()))));
    }
    return std::sync::Arc::new(std::sync::Mutex::new(Some((*base.lock().unwrap().as_mut().unwrap()) * power(std::sync::Arc::new(std::sync::Mutex::new(Some((*base.lock().unwrap().as_mut().unwrap())))), std::sync::Arc::new(std::sync::Mutex::new(Some((*exp.lock().unwrap().as_mut().unwrap()) - 1)))))));
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
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*s.lock().unwrap().as_mut().unwrap()).clone())));
    }
    return std::sync::Arc::new(std::sync::Mutex::new(Some(reverse_string(std::sync::Arc::new(std::sync::Mutex::new(Some((*s.lock().unwrap().as_mut().unwrap())[1..].to_vec())))) + string(std::sync::Arc::new(std::sync::Mutex::new(Some((*s.lock().unwrap().as_mut().unwrap())[0])))))));
}

fn main() {
    println!("{} {}", "Factorial of 5:".to_string(), (*factorial(std::sync::Arc::new(std::sync::Mutex::new(Some(5)))).lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "Factorial of 0:".to_string(), (*factorial(std::sync::Arc::new(std::sync::Mutex::new(Some(0)))).lock().unwrap().as_mut().unwrap()));
    println!("{}", "Fibonacci sequence:".to_string());
    let mut i = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_mut().unwrap()) < 10 {
        print!("fib({}) = {}\n", (*i.lock().unwrap().as_mut().unwrap()), fibonacci(std::sync::Arc::new(std::sync::Mutex::new(Some((*i.lock().unwrap().as_mut().unwrap()))))));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    println!("{} {}", "GCD of 48 and 18:".to_string(), (*gcd(std::sync::Arc::new(std::sync::Mutex::new(Some(48))), std::sync::Arc::new(std::sync::Mutex::new(Some(18)))).lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "GCD of 17 and 13:".to_string(), (*gcd(std::sync::Arc::new(std::sync::Mutex::new(Some(17))), std::sync::Arc::new(std::sync::Mutex::new(Some(13)))).lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "2^8 =".to_string(), (*power(std::sync::Arc::new(std::sync::Mutex::new(Some(2))), std::sync::Arc::new(std::sync::Mutex::new(Some(8)))).lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "3^4 =".to_string(), (*power(std::sync::Arc::new(std::sync::Mutex::new(Some(3))), std::sync::Arc::new(std::sync::Mutex::new(Some(4)))).lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "5^0 =".to_string(), (*power(std::sync::Arc::new(std::sync::Mutex::new(Some(5))), std::sync::Arc::new(std::sync::Mutex::new(Some(0)))).lock().unwrap().as_mut().unwrap()));
    let mut numbers = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![1, 2, 3, 4, 5])));
    println!("{} {} {} {}", "Sum of".to_string(), (*numbers.lock().unwrap().as_mut().unwrap()), "=".to_string(), (*sum_array(std::sync::Arc::new(std::sync::Mutex::new(Some((*numbers.lock().unwrap().as_mut().unwrap()))))).lock().unwrap().as_mut().unwrap()));
    let mut original = std::sync::Arc::new(std::sync::Mutex::new(Some("hello".to_string())));
    let mut reversed = reverse_string(std::sync::Arc::new(std::sync::Mutex::new(Some((*original.lock().unwrap().as_mut().unwrap())))));
    print!("'{}' reversed is '{}'\n", (*original.lock().unwrap().as_mut().unwrap()), (*reversed.lock().unwrap().as_mut().unwrap()));
}