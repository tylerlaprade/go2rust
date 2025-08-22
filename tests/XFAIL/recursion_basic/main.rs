use std::cell::{RefCell};
use std::fmt::{Display};
use std::rc::{Rc};

fn format_slice<T>(slice: &Rc<RefCell<Option<Vec<T>>>>) -> String 
where
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

pub fn factorial(n: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>> {

    if (*n.borrow_mut().as_mut().unwrap()) <= 1 {
        return Rc::new(RefCell::new(Some(1)));
    }
    return {
            let __tmp_x = (*n.borrow_mut().as_mut().unwrap());
            let __tmp_y = (*factorial(Rc::new(RefCell::new(Some((*n.borrow_mut().as_mut().unwrap()) - 1)))).borrow().as_ref().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x * __tmp_y)))
        };
}

pub fn fibonacci(n: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>> {

    if (*n.borrow_mut().as_mut().unwrap()) <= 1 {
        return n.clone();
    }
    return Rc::new(RefCell::new(Some((*fibonacci(Rc::new(RefCell::new(Some((*n.borrow_mut().as_mut().unwrap()) - 1)))).borrow().as_ref().unwrap()) + (*fibonacci(Rc::new(RefCell::new(Some((*n.borrow_mut().as_mut().unwrap()) - 2)))).borrow().as_ref().unwrap()))));
}

pub fn gcd(a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>> {

    if (*b.borrow_mut().as_mut().unwrap()) == 0 {
        return a.clone();
    }
    return Rc::new(RefCell::new(Some(gcd(b.clone(), Rc::new(RefCell::new(Some((*a.borrow_mut().as_mut().unwrap()) % (*b.borrow_mut().as_mut().unwrap()))))))));
}

pub fn power(base: Rc<RefCell<Option<i32>>>, exp: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>> {

    if (*exp.borrow_mut().as_mut().unwrap()) == 0 {
        return Rc::new(RefCell::new(Some(1)));
    }
    if (*exp.borrow_mut().as_mut().unwrap()) == 1 {
        return base.clone();
    }
    if (*exp.borrow_mut().as_mut().unwrap()) % 2 == 0 {
        let mut half = power(base.clone(), Rc::new(RefCell::new(Some((*exp.borrow_mut().as_mut().unwrap()) / 2))));
        return {
            let __tmp_x = (*half.borrow_mut().as_mut().unwrap());
            let __tmp_y = (*half.borrow_mut().as_mut().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x * __tmp_y)))
        };
    }
    return {
            let __tmp_x = (*base.borrow_mut().as_mut().unwrap());
            let __tmp_y = (*power(base.clone(), Rc::new(RefCell::new(Some((*exp.borrow_mut().as_mut().unwrap()) - 1)))).borrow().as_ref().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x * __tmp_y)))
        };
}

pub fn sum_array(arr: Rc<RefCell<Option<Vec<i32>>>>) -> Rc<RefCell<Option<i32>>> {

    if (*arr.borrow().as_ref().unwrap()).len() == 0 {
        return Rc::new(RefCell::new(Some(0)));
    }
    if (*arr.borrow().as_ref().unwrap()).len() == 1 {
        return Rc::new(RefCell::new(Some((*arr.borrow().as_ref().unwrap())[0 as usize].clone())));
    }
    return Rc::new(RefCell::new(Some((*(*arr.borrow().as_ref().unwrap())[0 as usize].clone().borrow().as_ref().unwrap()) + (*sum_array(Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some((*arr.borrow().as_ref().unwrap())[1 as usize..].to_vec()))))))).borrow().as_ref().unwrap()))));
}

pub fn reverse_string(s: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {

    if (*s.borrow().as_ref().unwrap()).len() <= 1 {
        return s.clone();
    }
    return Rc::new(RefCell::new(Some((*reverse_string(Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some((*s.borrow().as_ref().unwrap())[1 as usize..].to_vec()))))))).borrow().as_ref().unwrap()) + Rc::new(RefCell::new(Some((*(*(*s.borrow_mut().as_mut().unwrap()).borrow().as_ref().unwrap()).as_bytes()[0 as usize].borrow().as_ref().unwrap()).to_string()))))));
}

fn main() {
        // Factorial
    println!("{} {}", "Factorial of 5:".to_string(), (*factorial(Rc::new(RefCell::new(Some(5)))).borrow().as_ref().unwrap()));
    println!("{} {}", "Factorial of 0:".to_string(), (*factorial(Rc::new(RefCell::new(Some(0)))).borrow().as_ref().unwrap()));

        // Fibonacci
    println!("{}", "Fibonacci sequence:".to_string());
    let mut i = Rc::new(RefCell::new(Some(0)));
    while (*i.borrow_mut().as_mut().unwrap()) < 10 {
        print!("fib({}) = {}\n", (*i.borrow_mut().as_mut().unwrap()), (*fibonacci(i.clone()).borrow().as_ref().unwrap()));
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

        // GCD
    println!("{} {}", "GCD of 48 and 18:".to_string(), (*gcd(Rc::new(RefCell::new(Some(48))), Rc::new(RefCell::new(Some(18)))).borrow().as_ref().unwrap()));
    println!("{} {}", "GCD of 17 and 13:".to_string(), (*gcd(Rc::new(RefCell::new(Some(17))), Rc::new(RefCell::new(Some(13)))).borrow().as_ref().unwrap()));

        // Power
    println!("{} {}", "2^8 =".to_string(), (*power(Rc::new(RefCell::new(Some(2))), Rc::new(RefCell::new(Some(8)))).borrow().as_ref().unwrap()));
    println!("{} {}", "3^4 =".to_string(), (*power(Rc::new(RefCell::new(Some(3))), Rc::new(RefCell::new(Some(4)))).borrow().as_ref().unwrap()));
    println!("{} {}", "5^0 =".to_string(), (*power(Rc::new(RefCell::new(Some(5))), Rc::new(RefCell::new(Some(0)))).borrow().as_ref().unwrap()));

        // Sum array
    let mut numbers = Rc::new(RefCell::new(Some(vec![1, 2, 3, 4, 5])));
    println!("{} {} {} {}", "Sum of".to_string(), format_slice(&numbers), "=".to_string(), (*sum_array(numbers.clone()).borrow().as_ref().unwrap()));

        // Reverse string
    let mut original = Rc::new(RefCell::new(Some("hello".to_string())));
    let mut reversed = reverse_string(original.clone());
    print!("'{}' reversed is '{}'\n", (*original.borrow_mut().as_mut().unwrap()), (*reversed.borrow_mut().as_mut().unwrap()));
}