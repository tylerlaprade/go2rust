pub fn factorial(n: i32) -> i32 {

    if n <= 1 {
        return 1;
    }
    return n * factorial(n - 1);
}

pub fn fibonacci(n: i32) -> i32 {

    if n <= 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

pub fn gcd(a: i32, b: i32) -> i32 {

    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}

pub fn power(base: i32, exp: i32) -> i32 {

    if exp == 0 {
        return 1;
    }
    if exp == 1 {
        return base;
    }
    if exp % 2 == 0 {
        let mut half = power(base, exp / 2);
        return half * half;
    }
    return base * power(base, exp - 1);
}

pub fn sum_array(arr: Vec<i32>) -> i32 {

    if arr.len() == 0 {
        return 0;
    }
    if arr.len() == 1 {
        return arr[0];
    }
    return arr[0] + sum_array(arr[1..].to_vec());
}

pub fn reverse_string(s: String) -> String {

    if s.len() <= 1 {
        return s;
    }
    return reverse_string(s[1..].to_vec()) + string(s[0]);
}

fn main() {
    println!("{} {}", "Factorial of 5:".to_string(), factorial(5));
    println!("{} {}", "Factorial of 0:".to_string(), factorial(0));
    println!("{}", "Fibonacci sequence:".to_string());
    let mut i = 0;
    while i < 10 {
        print!("fib({}) = {}\n", i, fibonacci(i));
        i += 1;
    }
    println!("{} {}", "GCD of 48 and 18:".to_string(), gcd(48, 18));
    println!("{} {}", "GCD of 17 and 13:".to_string(), gcd(17, 13));
    println!("{} {}", "2^8 =".to_string(), power(2, 8));
    println!("{} {}", "3^4 =".to_string(), power(3, 4));
    println!("{} {}", "5^0 =".to_string(), power(5, 0));
    let mut numbers = vec![1, 2, 3, 4, 5];
    println!("{} {} {} {}", "Sum of".to_string(), numbers, "=".to_string(), sum_array(numbers));
    let mut original = "hello".to_string();
    let mut reversed = reverse_string(original);
    print!("'{}' reversed is '{}'\n", original, reversed);
}