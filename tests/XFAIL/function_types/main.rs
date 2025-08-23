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

/// Function type definitions
#[derive(Debug, Clone)]
struct BinaryOp(Rc<RefCell<Option<Box<dyn Fn(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>>>>);


#[derive(Debug, Clone)]
struct UnaryOp(Rc<RefCell<Option<Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>>>>);


#[derive(Debug, Clone)]
struct Predicate(Rc<RefCell<Option<Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<bool>>>>>>>);


#[derive(Debug, Clone)]
struct StringProcessor(Rc<RefCell<Option<Box<dyn Fn(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>>>>>);


/// Struct with function fields
#[derive(Debug, Clone, Default)]
struct Calculator {
    add: Rc<RefCell<Option<BinaryOp>>>,
    subtract: Rc<RefCell<Option<BinaryOp>>>,
    multiply: Rc<RefCell<Option<BinaryOp>>>,
}

/// Functions that match the types
pub fn add(a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>> {

    return {
            let __tmp_x = (*a.borrow_mut().as_mut().unwrap());
            let __tmp_y = (*b.borrow_mut().as_mut().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x + __tmp_y)))
        };
}

pub fn multiply(a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>> {

    return {
            let __tmp_x = (*a.borrow_mut().as_mut().unwrap());
            let __tmp_y = (*b.borrow_mut().as_mut().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x * __tmp_y)))
        };
}

pub fn square(x: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>> {

    return {
            let __tmp_x = (*x.borrow_mut().as_mut().unwrap());
            let __tmp_y = (*x.borrow_mut().as_mut().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x * __tmp_y)))
        };
}

pub fn is_even(x: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<bool>>> {

    return Rc::new(RefCell::new(Some((*x.borrow_mut().as_mut().unwrap()) % 2 == 0)));
}

pub fn to_upper(s: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {

    let mut result = Rc::new(RefCell::new(Some("".to_string())));
    for (_, char) in (*(*s.borrow_mut().as_mut().unwrap()).borrow().as_ref().unwrap()).chars().enumerate() {
        if char >= ('a' as i32) && char <= ('z' as i32) {
        (*result.borrow_mut().as_mut().unwrap()).push_str(&Rc::new(RefCell::new(Some(char::from_u32((*char - 32.borrow().as_ref().unwrap()) as u32).unwrap().to_string()))));
    } else {
        (*result.borrow_mut().as_mut().unwrap()).push_str(&Rc::new(RefCell::new(Some(char::from_u32((*char.borrow().as_ref().unwrap()) as u32).unwrap().to_string()))));
    }
    }
    return result.clone();
}

/// Higher-order functions
pub fn apply_binary(op: Rc<RefCell<Option<BinaryOp>>>, a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>> {

    return Rc::new(RefCell::new(Some((*op.borrow().as_ref().unwrap())(a.clone(), b.clone()))));
}

pub fn apply_unary(op: Rc<RefCell<Option<UnaryOp>>>, x: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>> {

    return Rc::new(RefCell::new(Some((*op.borrow().as_ref().unwrap())(x.clone()))));
}

pub fn filter(numbers: Rc<RefCell<Option<Vec<i32>>>>, pred: Rc<RefCell<Option<Predicate>>>) -> Rc<RefCell<Option<Vec<i32>>>> {

    let mut result: Rc<RefCell<Option<Vec<i32>>>> = Rc::new(RefCell::new(Some(Default::default())));
    for num in &(*numbers.borrow_mut().as_mut().unwrap()) {
        if (*pred.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(*num)))) {
        {(*result.borrow_mut().as_mut().unwrap()).push(num); result.clone()};
    }
    }
    return result.clone();
}

pub fn transform(numbers: Rc<RefCell<Option<Vec<i32>>>>, op: Rc<RefCell<Option<UnaryOp>>>) -> Rc<RefCell<Option<Vec<i32>>>> {

    let mut result = Rc::new(RefCell::new(Some(vec![0; (*numbers.borrow().as_ref().unwrap()).len()])));
    for (i, num) in (*numbers.borrow_mut().as_mut().unwrap()).iter().enumerate() {
        (*result.borrow_mut().as_mut().unwrap())[i] = (*(*op.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(*num)))).borrow().as_ref().unwrap());
    }
    return result.clone();
}

pub fn process_string(s: Rc<RefCell<Option<String>>>, processor: Rc<RefCell<Option<StringProcessor>>>) -> Rc<RefCell<Option<String>>> {

    return Rc::new(RefCell::new(Some((*processor.borrow().as_ref().unwrap())(s.clone()))));
}

/// Function that returns a function
pub fn make_multiplier(factor: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<UnaryOp>>> {

    return Rc::new(RefCell::new(Some(Box::new(move |x: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return {
            let __tmp_x = (*x.borrow_mut().as_mut().unwrap());
            let __tmp_y = (*factor.borrow_mut().as_mut().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x * __tmp_y)))
        };
    }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>)));
}

pub fn make_adder(addend: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<BinaryOp>>> {

    let addend_closure_clone = addend.clone(); return Rc::new(RefCell::new(Some(Box::new(move |a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return {
            let __tmp_x = (*a.borrow_mut().as_mut().unwrap()) + (*b.borrow_mut().as_mut().unwrap());
            let __tmp_y = (*addend_closure_clone.borrow_mut().as_mut().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x + __tmp_y)))
        };
    }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>)));
}

fn main() {
        // Basic function type usage
    println!("{}", "=== Basic function types ===".to_string());

    let mut op: Rc<RefCell<Option<BinaryOp>>> = Default::default();
    { let new_val = (*add.borrow_mut().as_mut().unwrap()); *op.borrow_mut() = Some(new_val); };
    print!("5 + 3 = {}\n", (*(*op.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(5))), Rc::new(RefCell::new(Some(3)))).borrow().as_ref().unwrap()));

    { let new_val = (*multiply.borrow_mut().as_mut().unwrap()); *op.borrow_mut() = Some(new_val); };
    print!("5 * 3 = {}\n", (*(*op.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(5))), Rc::new(RefCell::new(Some(3)))).borrow().as_ref().unwrap()));

        // Higher-order functions
    println!("{}", "\n=== Higher-order functions ===".to_string());
    let mut result = apply_binary(add.clone(), Rc::new(RefCell::new(Some(10))), Rc::new(RefCell::new(Some(20))));
    print!("applyBinary(add, 10, 20) = {}\n", (*result.borrow_mut().as_mut().unwrap()));

    { let new_val = apply_binary(multiply.clone(), Rc::new(RefCell::new(Some(4))), Rc::new(RefCell::new(Some(7)))); *result.borrow_mut() = Some(new_val); };
    print!("applyBinary(multiply, 4, 7) = {}\n", (*result.borrow_mut().as_mut().unwrap()));

    let mut unaryResult = apply_unary(square.clone(), Rc::new(RefCell::new(Some(6))));
    print!("applyUnary(square, 6) = {}\n", (*unaryResult.borrow_mut().as_mut().unwrap()));

        // Function slices and filtering
    println!("{}", "\n=== Function slices and filtering ===".to_string());
    let mut numbers = Rc::new(RefCell::new(Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])));

    let mut evens = filter(numbers.clone(), isEven.clone());
    print!("Even numbers: {}\n", format_slice(&evens));

    let mut odds = filter(numbers.clone(), Rc::new(RefCell::new(Some(Box::new(move |x: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<bool>>> {
        return Rc::new(RefCell::new(Some((*x.borrow_mut().as_mut().unwrap()) % 2 != 0)));
    }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<bool>>>>))));
    print!("Odd numbers: {}\n", format_slice(&odds));

        // Transform with function types
    println!("{}", "\n=== Transform operations ===".to_string());
    let mut squared = transform(Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some(vec![1, 2, 3, 4, 5])))))), square.clone());
    print!("Squared: {}\n", format_slice(&squared));

    let mut doubled = transform(Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some(vec![1, 2, 3, 4, 5])))))), Rc::new(RefCell::new(Some(Box::new(move |x: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return {
            let __tmp_x = (*x.borrow_mut().as_mut().unwrap());
            let __tmp_y = 2;
            Rc::new(RefCell::new(Some(__tmp_x * __tmp_y)))
        };
    }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>))));
    print!("Doubled: {}\n", format_slice(&doubled));

        // String processing
    println!("{}", "\n=== String processing ===".to_string());
    let mut text = Rc::new(RefCell::new(Some("hello world".to_string())));
    let mut upper = process_string(text.clone(), toUpper.clone());
    print!("'{}' -> '{}'\n", (*text.borrow_mut().as_mut().unwrap()), (*upper.borrow_mut().as_mut().unwrap()));

    let rune_closure_clone = rune.clone(); let mut reversed = process_string(Rc::new(RefCell::new(Some("hello".to_string()))), Rc::new(RefCell::new(Some(Box::new(move |s: Rc<RefCell<Option<String>>>| -> Rc<RefCell<Option<String>>> {
        let mut runes = Rc::new(RefCell::new(Some((*s.borrow().as_ref().unwrap()).chars().map(|c| c as i32).collect::<Vec<_>>())));
        let (mut i, mut j) = (Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some((*runes.borrow().as_ref().unwrap()).len() - 1))));
    while (*i.borrow_mut().as_mut().unwrap()) < (*j.borrow_mut().as_mut().unwrap()) {
        { *(*runes.borrow().as_ref().unwrap())[(*i.borrow_mut().as_mut().unwrap()) as usize].clone().borrow_mut() = Some((*runes.borrow().as_ref().unwrap())[(*j.borrow_mut().as_mut().unwrap()) as usize].clone()); *(*runes.borrow().as_ref().unwrap())[(*j.borrow_mut().as_mut().unwrap()) as usize].clone().borrow_mut() = Some((*runes.borrow().as_ref().unwrap())[(*i.borrow_mut().as_mut().unwrap()) as usize].clone()) };
        { *(*i.borrow_mut().as_mut().unwrap()).borrow_mut() = Some((*i.borrow_mut().as_mut().unwrap()) + 1); *(*j.borrow_mut().as_mut().unwrap()).borrow_mut() = Some((*j.borrow_mut().as_mut().unwrap()) - 1) };
    }
        return Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some((*runes.borrow().as_ref().unwrap()).iter().map(|&c| char::from_u32(c as u32).unwrap()).collect::<String>()))))));
    }) as Box<dyn Fn(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>>))));
    print!("Reversed: {}\n", (*reversed.borrow_mut().as_mut().unwrap()));

        // Functions that return functions
    println!("{}", "\n=== Functions returning functions ===".to_string());
    let mut triple = make_multiplier(Rc::new(RefCell::new(Some(3))));
    print!("triple(4) = {}\n", (*(*triple.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(4)))).borrow().as_ref().unwrap()));

    let mut addTen = make_adder(Rc::new(RefCell::new(Some(10))));
    print!("addTen(5, 3) = {}\n", (*(*addTen.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(5))), Rc::new(RefCell::new(Some(3)))).borrow().as_ref().unwrap()));

        // Struct with function fields
    println!("{}", "\n=== Struct with function fields ===".to_string());
    let mut calc = Rc::new(RefCell::new(Some(Calculator { add: Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some(Box::new(move |a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return {
            let __tmp_x = (*a.borrow_mut().as_mut().unwrap());
            let __tmp_y = (*b.borrow_mut().as_mut().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x + __tmp_y)))
        };
    }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>)))))), subtract: Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some(Box::new(move |a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return {
            let __tmp_x = (*a.borrow_mut().as_mut().unwrap());
            let __tmp_y = (*b.borrow_mut().as_mut().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x - __tmp_y)))
        };
    }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>)))))), multiply: multiply.clone() })));

        // Reuse existing function
    print!("calc.Add(10, 5) = {}\n", (*(*calc.borrow_mut().as_mut().unwrap()).add(Rc::new(RefCell::new(Some(10))), Rc::new(RefCell::new(Some(5)))).borrow().as_ref().unwrap()));
    print!("calc.Subtract(10, 5) = {}\n", (*(*calc.borrow_mut().as_mut().unwrap()).subtract(Rc::new(RefCell::new(Some(10))), Rc::new(RefCell::new(Some(5)))).borrow().as_ref().unwrap()));
    print!("calc.Multiply(10, 5) = {}\n", (*(*calc.borrow_mut().as_mut().unwrap()).multiply(Rc::new(RefCell::new(Some(10))), Rc::new(RefCell::new(Some(5)))).borrow().as_ref().unwrap()));

        // Function variables
    println!("{}", "\n=== Function variables ===".to_string());
    let mut processor: Rc<RefCell<Option<StringProcessor>>> = Default::default();
    { let new_val = (*toUpper.borrow_mut().as_mut().unwrap()); *processor.borrow_mut() = Some(new_val); };
    print!("Using toUpper: {}\n", (*(*processor.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some("test".to_string())))).borrow().as_ref().unwrap()));

    { let new_val = Rc::new(RefCell::new(Some(Box::new(move |s: Rc<RefCell<Option<String>>>| -> Rc<RefCell<Option<String>>> {
        return {
            let __tmp_x = "processed: ".to_string();
            let __tmp_y = (*s.borrow_mut().as_mut().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x + __tmp_y)))
        };
    }) as Box<dyn Fn(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>>))); *processor.borrow_mut() = Some(new_val); };
    print!("Using anonymous: {}\n", (*(*processor.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some("test".to_string())))).borrow().as_ref().unwrap()));
}