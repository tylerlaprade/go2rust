use std::cell::{RefCell};
use std::fmt::{Display};
use std::rc::{Rc};

fn format_slice<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[T]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn format_slice_values<T>(slice: &[T]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| v.to_string()).collect();
    format!("[{}]", formatted.join(" "))
}

fn format_slice_wrapped<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[Rc<RefCell<Option<T>>>]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| {
            let inner = v.borrow();
            match inner.as_ref() {
                Some(value) => format!("&{}", value),
                None => "<nil>".to_string(),
            }
        }).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

pub fn factorial(n: Rc<RefCell<Option<i64>>>) -> i64 {
    if (*n.borrow().as_ref().unwrap()) <= 1 {
        return 1;
    }
    (*n.borrow().as_ref().unwrap()) * factorial(Rc::new(RefCell::new(Some((*n.borrow().as_ref().unwrap()) - 1))))
}

pub fn fibonacci(n: Rc<RefCell<Option<i64>>>) -> i64 {
    if (*n.borrow().as_ref().unwrap()) <= 1 {
        return (*n.borrow().as_ref().unwrap());
    }
    fibonacci(Rc::new(RefCell::new(Some((*n.borrow().as_ref().unwrap()) - 1)))) + fibonacci(Rc::new(RefCell::new(Some((*n.borrow().as_ref().unwrap()) - 2))))
}

pub fn gcd(a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) -> i64 {
    if (*b.borrow().as_ref().unwrap()) == 0 {
        return (*a.borrow().as_ref().unwrap());
    }
    gcd(Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) % (*b.borrow().as_ref().unwrap())))))
}

pub fn power(base: Rc<RefCell<Option<i64>>>, exp: Rc<RefCell<Option<i64>>>) -> i64 {
    if (*exp.borrow().as_ref().unwrap()) == 0 {
        return 1;
    }
    if (*exp.borrow().as_ref().unwrap()) == 1 {
        return (*base.borrow().as_ref().unwrap());
    }
    if (*exp.borrow().as_ref().unwrap()) % 2 == 0 {
        let mut half = power(Rc::new(RefCell::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some((*exp.borrow().as_ref().unwrap()) / 2))));
        return half * half;
    }
    (*base.borrow().as_ref().unwrap()) * power(Rc::new(RefCell::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some((*exp.borrow().as_ref().unwrap()) - 1))))
}

pub fn sum_array(arr: Rc<RefCell<Option<Vec<i64>>>>) -> i64 {
    if ((*arr.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i64) == (0 as i64) {
        return 0;
    }
    if ((*arr.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i64) == (1 as i64) {
        return (*arr.borrow().as_ref().unwrap())[(0) as usize].clone();
    }
    (*arr.borrow().as_ref().unwrap())[(0) as usize].clone() + sum_array(Rc::new(RefCell::new(Some({ let __seq_holder = arr.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (1) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))))
}

pub fn reverse_string(s: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {
    if ((*s.borrow().as_ref().unwrap()).len() as i64) <= (1 as i64) {
        return { let __owned = s.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    }
    Rc::new(RefCell::new(Some(format!("{}{}", (*reverse_string(Rc::new(RefCell::new(Some({ let __s = &((*s.borrow().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() })))).borrow().as_ref().unwrap()), (*Rc::new(RefCell::new(Some(({ let __s = &((*s.borrow().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] } as char).to_string()))).borrow().as_ref().unwrap())))))
}

fn main() {
        // Factorial
    println!("{} {}", format!("{}", "Factorial of 5:".to_string()), format!("{}", factorial(Rc::new(RefCell::new(Some(5))))));
    println!("{} {}", format!("{}", "Factorial of 0:".to_string()), format!("{}", factorial(Rc::new(RefCell::new(Some(0))))));

        // Fibonacci
    println!("{}", format!("{}", "Fibonacci sequence:".to_string()));
    let mut i = Rc::new(RefCell::new(Some(0i64)));
    while (*i.borrow().as_ref().unwrap()) < 10 {
        print!("fib({}) = {}\n", { let __v = (*i.borrow().as_ref().unwrap()).clone(); __v }, fibonacci(Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })))));
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

        // GCD
    println!("{} {}", format!("{}", "GCD of 48 and 18:".to_string()), format!("{}", gcd(Rc::new(RefCell::new(Some(48))), Rc::new(RefCell::new(Some(18))))));
    println!("{} {}", format!("{}", "GCD of 17 and 13:".to_string()), format!("{}", gcd(Rc::new(RefCell::new(Some(17))), Rc::new(RefCell::new(Some(13))))));

        // Power
    println!("{} {}", format!("{}", "2^8 =".to_string()), format!("{}", power(Rc::new(RefCell::new(Some(2))), Rc::new(RefCell::new(Some(8))))));
    println!("{} {}", format!("{}", "3^4 =".to_string()), format!("{}", power(Rc::new(RefCell::new(Some(3))), Rc::new(RefCell::new(Some(4))))));
    println!("{} {}", format!("{}", "5^0 =".to_string()), format!("{}", power(Rc::new(RefCell::new(Some(5))), Rc::new(RefCell::new(Some(0))))));

        // Sum array
    let mut numbers = Rc::new(RefCell::new(Some(vec![1i64, 2i64, 3i64, 4i64, 5i64])));
    println!("{} {} {} {}", format!("{}", "Sum of".to_string()), format!("{}", format_slice(&numbers)), format!("{}", "=".to_string()), format!("{}", sum_array(numbers.clone())));

        // Reverse string
    let mut original = Rc::new(RefCell::new(Some("hello".to_string())));
    let mut reversed = reverse_string(Rc::new(RefCell::new(Some({ let __arg_holder = original.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    print!("'{}' reversed is '{}'\n", { let __v = (*original.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*reversed.borrow().as_ref().unwrap()).clone(); __v });
}