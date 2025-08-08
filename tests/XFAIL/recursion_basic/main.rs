use std::fmt::{Display};
use std::sync::{Arc, Mutex};

fn format_slice<T>(slice: &Arc<Mutex<Option<Vec<T>>>>) -> String 
where
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

pub fn factorial(n: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> {

    if (*n.lock().unwrap().as_mut().unwrap()) <= 1 {
        return Arc::new(Mutex::new(Some(1)));
    }
    return {
            let __tmp_x = (*n.lock().unwrap().as_mut().unwrap());
            let __tmp_y = factorial(Arc::new(Mutex::new(Some((*n.lock().unwrap().as_mut().unwrap()) - 1))));
            Arc::new(Mutex::new(Some(__tmp_x * __tmp_y)))
        };
}

pub fn fibonacci(n: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> {

    if (*n.lock().unwrap().as_mut().unwrap()) <= 1 {
        return n.clone();
    }
    return Arc::new(Mutex::new(Some((*fibonacci(Arc::new(Mutex::new(Some((*n.lock().unwrap().as_mut().unwrap()) - 1)))).lock().unwrap().as_ref().unwrap()) + (*fibonacci(Arc::new(Mutex::new(Some((*n.lock().unwrap().as_mut().unwrap()) - 2)))).lock().unwrap().as_ref().unwrap()))));
}

pub fn gcd(a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> {

    if (*b.lock().unwrap().as_mut().unwrap()) == 0 {
        return a.clone();
    }
    return Arc::new(Mutex::new(Some(gcd(b.clone(), Arc::new(Mutex::new(Some((*a.lock().unwrap().as_mut().unwrap()) % (*b.lock().unwrap().as_mut().unwrap()))))))));
}

pub fn power(base: Arc<Mutex<Option<i32>>>, exp: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> {

    if (*exp.lock().unwrap().as_mut().unwrap()) == 0 {
        return Arc::new(Mutex::new(Some(1)));
    }
    if (*exp.lock().unwrap().as_mut().unwrap()) == 1 {
        return base.clone();
    }
    if (*exp.lock().unwrap().as_mut().unwrap()) % 2 == 0 {
        let mut half = power(base.clone(), Arc::new(Mutex::new(Some((*exp.lock().unwrap().as_mut().unwrap()) / 2))));
        return {
            let __tmp_x = (*half.lock().unwrap().as_mut().unwrap());
            let __tmp_y = (*half.lock().unwrap().as_mut().unwrap());
            Arc::new(Mutex::new(Some(__tmp_x * __tmp_y)))
        };
    }
    return {
            let __tmp_x = (*base.lock().unwrap().as_mut().unwrap());
            let __tmp_y = power(base.clone(), Arc::new(Mutex::new(Some((*exp.lock().unwrap().as_mut().unwrap()) - 1))));
            Arc::new(Mutex::new(Some(__tmp_x * __tmp_y)))
        };
}

pub fn sum_array(arr: Arc<Mutex<Option<Vec<i32>>>>) -> Arc<Mutex<Option<i32>>> {

    if (*arr.lock().unwrap().as_ref().unwrap()).len() == 0 {
        return Arc::new(Mutex::new(Some(0)));
    }
    if (*arr.lock().unwrap().as_ref().unwrap()).len() == 1 {
        return Arc::new(Mutex::new(Some((*arr.lock().unwrap().as_ref().unwrap())[0 as usize].clone())));
    }
    return Arc::new(Mutex::new(Some((*(*arr.lock().unwrap().as_ref().unwrap())[0 as usize].clone().lock().unwrap().as_ref().unwrap()) + (*sum_array(Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some((*arr.lock().unwrap().as_ref().unwrap())[1 as usize..].to_vec()))))))).lock().unwrap().as_ref().unwrap()))));
}

pub fn reverse_string(s: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {

    if (*s.lock().unwrap().as_ref().unwrap()).len() <= 1 {
        return s.clone();
    }
    return Arc::new(Mutex::new(Some((*reverse_string(Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap())[1 as usize..].to_vec()))))))).lock().unwrap().as_ref().unwrap()) + Arc::new(Mutex::new(Some((*(*(*s.lock().unwrap().as_mut().unwrap()).lock().unwrap().as_ref().unwrap()).as_bytes()[0 as usize].lock().unwrap().as_ref().unwrap()).to_string()))))));
}

fn main() {
    println!("{} {}", "Factorial of 5:".to_string(), (*factorial(Arc::new(Mutex::new(Some(5)))).lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Factorial of 0:".to_string(), (*factorial(Arc::new(Mutex::new(Some(0)))).lock().unwrap().as_ref().unwrap()));

    println!("{}", "Fibonacci sequence:".to_string());
    let mut i = Arc::new(Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_mut().unwrap()) < 10 {
        print!("fib({}) = {}\n", (*i.lock().unwrap().as_mut().unwrap()), (*fibonacci(i.clone()).lock().unwrap().as_ref().unwrap()));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    println!("{} {}", "GCD of 48 and 18:".to_string(), (*gcd(Arc::new(Mutex::new(Some(48))), Arc::new(Mutex::new(Some(18)))).lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "GCD of 17 and 13:".to_string(), (*gcd(Arc::new(Mutex::new(Some(17))), Arc::new(Mutex::new(Some(13)))).lock().unwrap().as_ref().unwrap()));

    println!("{} {}", "2^8 =".to_string(), (*power(Arc::new(Mutex::new(Some(2))), Arc::new(Mutex::new(Some(8)))).lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "3^4 =".to_string(), (*power(Arc::new(Mutex::new(Some(3))), Arc::new(Mutex::new(Some(4)))).lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "5^0 =".to_string(), (*power(Arc::new(Mutex::new(Some(5))), Arc::new(Mutex::new(Some(0)))).lock().unwrap().as_ref().unwrap()));

    let mut numbers = Arc::new(Mutex::new(Some(vec![1, 2, 3, 4, 5])));
    println!("{} {} {} {}", "Sum of".to_string(), format_slice(&numbers), "=".to_string(), (*sum_array(numbers.clone()).lock().unwrap().as_ref().unwrap()));

    let mut original = Arc::new(Mutex::new(Some("hello".to_string())));
    let mut reversed = reverse_string(original.clone());
    print!("'{}' reversed is '{}'\n", (*original.lock().unwrap().as_mut().unwrap()), (*reversed.lock().unwrap().as_mut().unwrap()));
}