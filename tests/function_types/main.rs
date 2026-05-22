use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
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

/// Function type definitions
pub type BinaryOp = Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>>>>;


pub type UnaryOp = Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>>>>;


pub type Predicate = Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<bool>>>>>>>;


pub type StringProcessor = Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>>>>>;


/// Struct with function fields
#[derive(Clone, Default)]
pub struct Calculator {
    pub add: BinaryOp,
    pub subtract: BinaryOp,
    pub multiply: BinaryOp,
}

impl Calculator {
    pub fn __go_value_clone(&self) -> Self {
        Self { add: self.add.clone(), subtract: self.subtract.clone(), multiply: self.multiply.clone() }
    }
}

impl std::fmt::Display for Calculator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", "<func>", "<func>", "<func>")
    }
}


/// Functions that match the types
pub fn add(a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>> {

    return {
            let __tmp_x = (*a.borrow().as_ref().unwrap());
            let __tmp_y = (*b.borrow().as_ref().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x + __tmp_y)))
        };
}

pub fn multiply(a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>> {

    return {
            let __tmp_x = (*a.borrow().as_ref().unwrap());
            let __tmp_y = (*b.borrow().as_ref().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x * __tmp_y)))
        };
}

pub fn square(x: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>> {

    return {
            let __tmp_x = (*x.borrow().as_ref().unwrap());
            let __tmp_y = (*x.borrow().as_ref().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x * __tmp_y)))
        };
}

pub fn is_even(x: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<bool>>> {

    return Rc::new(RefCell::new(Some((*x.borrow().as_ref().unwrap()) % 2 == 0)));
}

pub fn to_upper(s: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {

    let mut result = Rc::new(RefCell::new(Some("".to_string())));
    for (_, char) in (*s.borrow().as_ref().unwrap()).char_indices() {
        if char >= 'a' && char <= 'z' {
        { (*result.borrow_mut().as_mut().unwrap()).push_str(&{ let __s = Rc::new(RefCell::new(Some(char::from_u32((((char as i32) - 32)) as u32).unwrap().to_string()))); let __value = (*__s.borrow().as_ref().unwrap()).clone(); __value }); };
    } else {
        { (*result.borrow_mut().as_mut().unwrap()).push_str(&{ let __s = Rc::new(RefCell::new(Some(char::from_u32(((char as i32)) as u32).unwrap().to_string()))); let __value = (*__s.borrow().as_ref().unwrap()).clone(); __value }); };
    }
    }
    return Rc::new(RefCell::new(Some(result.borrow().as_ref().unwrap().clone())));
}

/// Higher-order functions
pub fn apply_binary(op: BinaryOp, a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>> {

    return { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>> = { let mut __f_guard = op.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(a.clone(), b.clone()) };
}

pub fn apply_unary(op: UnaryOp, x: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>> {

    return { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>> = { let mut __f_guard = op.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(x.clone()) };
}

pub fn filter(numbers: Rc<RefCell<Option<Vec<i32>>>>, pred: Predicate) -> Rc<RefCell<Option<Vec<i32>>>> {

    let mut result: Rc<RefCell<Option<Vec<i32>>>> = Rc::new(RefCell::new(None));
    { let __range_holder = numbers.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for num in __range_values.iter().copied() {
        if (*{ let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<bool>>>> = { let mut __f_guard = pred.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<bool>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(num)))) }.borrow().as_ref().unwrap()) {
        { let new_val = { let __append_target = result.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(num); __append_target.clone() }; result = new_val; };
    }
    } }
    return result.clone();
}

pub fn transform(numbers: Rc<RefCell<Option<Vec<i32>>>>, op: UnaryOp) -> Rc<RefCell<Option<Vec<i32>>>> {

    let mut result = Rc::new(RefCell::new(Some(vec![0; ((*numbers.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize])));
    { let __range_holder = numbers.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, num) in __range_values.iter().copied().enumerate() {
        (*result.borrow_mut().as_mut().unwrap())[(i) as usize] = (*{ let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>> = { let mut __f_guard = op.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(num)))) }.borrow().as_ref().unwrap()).clone();
    } }
    return result.clone();
}

pub fn process_string(s: Rc<RefCell<Option<String>>>, processor: StringProcessor) -> Rc<RefCell<Option<String>>> {

    return { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>> = { let mut __f_guard = processor.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })))) };
}

/// Function that returns a function
pub fn make_multiplier(factor: Rc<RefCell<Option<i32>>>) -> UnaryOp {

    let factor_closure_clone = factor.clone(); return Rc::new(RefCell::new(Some(Box::new(move |x: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return {
            let __tmp_x = (*x.borrow().as_ref().unwrap());
            let __tmp_y = (*factor_closure_clone.borrow().as_ref().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x * __tmp_y)))
        };
    }) as Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>)));
}

pub fn make_adder(addend: Rc<RefCell<Option<i32>>>) -> BinaryOp {

    let addend_closure_clone = addend.clone(); return Rc::new(RefCell::new(Some(Box::new(move |a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return {
            let __tmp_x = (*a.borrow().as_ref().unwrap()) + (*b.borrow().as_ref().unwrap());
            let __tmp_y = (*addend_closure_clone.borrow().as_ref().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x + __tmp_y)))
        };
    }) as Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>)));
}

fn main() {
        // Basic function type usage
    println!("{}", format!("{}", "=== Basic function types ===".to_string()));

    let mut op: BinaryOp = Rc::new(RefCell::new(None));
    { let new_val = Box::new(move |__arg0: Rc<RefCell<Option<i32>>>, __arg1: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> { add(__arg0, __arg1) }) as Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>; *op.borrow_mut() = Some(new_val); };
    print!("5 + 3 = {}\n", (*{ let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>> = { let mut __f_guard = op.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(5))), Rc::new(RefCell::new(Some(3)))) }.borrow().as_ref().unwrap()));

    { let new_val = Box::new(move |__arg0: Rc<RefCell<Option<i32>>>, __arg1: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> { multiply(__arg0, __arg1) }) as Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>; *op.borrow_mut() = Some(new_val); };
    print!("5 * 3 = {}\n", (*{ let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>> = { let mut __f_guard = op.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(5))), Rc::new(RefCell::new(Some(3)))) }.borrow().as_ref().unwrap()));

        // Higher-order functions
    println!("{}", format!("{}", "\n=== Higher-order functions ===".to_string()));
    let mut result = apply_binary(Rc::new(RefCell::new(Some(Box::new(move |__arg0: Rc<RefCell<Option<i32>>>, __arg1: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> { add(__arg0, __arg1) }) as Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>))), Rc::new(RefCell::new(Some(10))), Rc::new(RefCell::new(Some(20))));
    print!("applyBinary(add, 10, 20) = {}\n", { let __v = (*result.borrow().as_ref().unwrap()).clone(); __v });

    { let new_val = apply_binary(Rc::new(RefCell::new(Some(Box::new(move |__arg0: Rc<RefCell<Option<i32>>>, __arg1: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> { multiply(__arg0, __arg1) }) as Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>))), Rc::new(RefCell::new(Some(4))), Rc::new(RefCell::new(Some(7)))); let __moved_val = { let mut __guard = new_val.borrow_mut(); __guard.take() }; *result.borrow_mut() = __moved_val; };
    print!("applyBinary(multiply, 4, 7) = {}\n", { let __v = (*result.borrow().as_ref().unwrap()).clone(); __v });

    let mut unaryResult = apply_unary(Rc::new(RefCell::new(Some(Box::new(move |__arg0: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> { square(__arg0) }) as Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>))), Rc::new(RefCell::new(Some(6))));
    print!("applyUnary(square, 6) = {}\n", { let __v = (*unaryResult.borrow().as_ref().unwrap()).clone(); __v });

        // Function slices and filtering
    println!("{}", format!("{}", "\n=== Function slices and filtering ===".to_string()));
    let mut numbers = Rc::new(RefCell::new(Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])));

    let mut evens = filter(numbers.clone(), Rc::new(RefCell::new(Some(Box::new(move |__arg0: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<bool>>> { is_even(__arg0) }) as Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<bool>>>>))));
    print!("Even numbers: {}\n", format_slice(&evens));

    let mut odds = filter(numbers.clone(), Rc::new(RefCell::new(Some(Box::new(move |x: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<bool>>> {
        return Rc::new(RefCell::new(Some((*x.borrow().as_ref().unwrap()) % 2 != 0)));
    }) as Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<bool>>>>))));
    print!("Odd numbers: {}\n", format_slice(&odds));

        // Transform with function types
    println!("{}", format!("{}", "\n=== Transform operations ===".to_string()));
    let mut squared = transform(Rc::new(RefCell::new(Some(vec![1, 2, 3, 4, 5]))), Rc::new(RefCell::new(Some(Box::new(move |__arg0: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> { square(__arg0) }) as Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>))));
    print!("Squared: {}\n", format_slice(&squared));

    let mut doubled = transform(Rc::new(RefCell::new(Some(vec![1, 2, 3, 4, 5]))), Rc::new(RefCell::new(Some(Box::new(move |x: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return {
            let __tmp_x = (*x.borrow().as_ref().unwrap());
            let __tmp_y = 2;
            Rc::new(RefCell::new(Some(__tmp_x * __tmp_y)))
        };
    }) as Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>))));
    print!("Doubled: {}\n", format_slice(&doubled));

        // String processing
    println!("{}", format!("{}", "\n=== String processing ===".to_string()));
    let mut text = Rc::new(RefCell::new(Some("hello world".to_string())));
    let mut upper = process_string(Rc::new(RefCell::new(Some({ let __arg_holder = text.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some(Box::new(move |__arg0: Rc<RefCell<Option<String>>>| -> Rc<RefCell<Option<String>>> { to_upper(__arg0) }) as Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>>))));
    print!("'{}' -> '{}'\n", { let __v = (*text.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*upper.borrow().as_ref().unwrap()).clone(); __v });

    let mut reversed = process_string(Rc::new(RefCell::new(Some("hello".to_string()))), Rc::new(RefCell::new(Some(Box::new(move |s: Rc<RefCell<Option<String>>>| -> Rc<RefCell<Option<String>>> {
        let mut runes = Rc::new(RefCell::new(Some(((*s.borrow().as_ref().unwrap())).chars().map(|c| c as i32).collect::<Vec<_>>())));
        let (mut i, mut j) = (Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(((*runes.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32) - (1 as i32)))));
    while (*i.borrow().as_ref().unwrap()) < (*j.borrow().as_ref().unwrap()) {
        { let __tmp_0 = (*runes.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); let __tmp_1 = (*runes.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(); (*runes.borrow_mut().as_mut().unwrap())[((*i.borrow().as_ref().unwrap())) as usize] = __tmp_0; (*runes.borrow_mut().as_mut().unwrap())[((*j.borrow().as_ref().unwrap())) as usize] = __tmp_1; };
        { let __tmp_0 = (*i.borrow().as_ref().unwrap()) + 1; let __tmp_1 = (*j.borrow().as_ref().unwrap()) - 1; *i.borrow_mut() = Some(__tmp_0); *j.borrow_mut() = Some(__tmp_1); };
    }
        return Rc::new(RefCell::new(Some((*runes.borrow().as_ref().unwrap()).iter().map(|&c| char::from_u32(c as u32).unwrap()).collect::<String>())));
    }) as Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>>))));
    print!("Reversed: {}\n", { let __v = (*reversed.borrow().as_ref().unwrap()).clone(); __v });

        // Functions that return functions
    println!("{}", format!("{}", "\n=== Functions returning functions ===".to_string()));
    let mut triple = make_multiplier(Rc::new(RefCell::new(Some(3))));
    print!("triple(4) = {}\n", (*{ let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>> = { let mut __f_guard = triple.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(4)))) }.borrow().as_ref().unwrap()));

    let mut addTen = make_adder(Rc::new(RefCell::new(Some(10))));
    print!("addTen(5, 3) = {}\n", (*{ let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>> = { let mut __f_guard = addTen.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(5))), Rc::new(RefCell::new(Some(3)))) }.borrow().as_ref().unwrap()));

        // Struct with function fields
    println!("{}", format!("{}", "\n=== Struct with function fields ===".to_string()));
    let mut calc = Rc::new(RefCell::new(Some(Calculator { add: Rc::new(RefCell::new(Some(Box::new(move |a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return {
            let __tmp_x = (*a.borrow().as_ref().unwrap());
            let __tmp_y = (*b.borrow().as_ref().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x + __tmp_y)))
        };
    }) as Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>))), subtract: Rc::new(RefCell::new(Some(Box::new(move |a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return {
            let __tmp_x = (*a.borrow().as_ref().unwrap());
            let __tmp_y = (*b.borrow().as_ref().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x - __tmp_y)))
        };
    }) as Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>))), multiply: Rc::new(RefCell::new(Some(Box::new(move |__arg0: Rc<RefCell<Option<i32>>>, __arg1: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> { multiply(__arg0, __arg1) }) as Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>))), ..Default::default() })));

        // Reuse existing function
    print!("calc.Add(10, 5) = {}\n", (*{ let __f_holder = (*calc.borrow().as_ref().unwrap()).add.clone(); let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(10))), Rc::new(RefCell::new(Some(5)))) }.borrow().as_ref().unwrap()));
    print!("calc.Subtract(10, 5) = {}\n", (*{ let __f_holder = (*calc.borrow().as_ref().unwrap()).subtract.clone(); let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(10))), Rc::new(RefCell::new(Some(5)))) }.borrow().as_ref().unwrap()));
    print!("calc.Multiply(10, 5) = {}\n", (*{ let __f_holder = (*calc.borrow().as_ref().unwrap()).multiply.clone(); let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(10))), Rc::new(RefCell::new(Some(5)))) }.borrow().as_ref().unwrap()));

        // Function variables
    println!("{}", format!("{}", "\n=== Function variables ===".to_string()));
    let mut processor: StringProcessor = Rc::new(RefCell::new(None));
    { let new_val = Box::new(move |__arg0: Rc<RefCell<Option<String>>>| -> Rc<RefCell<Option<String>>> { to_upper(__arg0) }) as Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>>; *processor.borrow_mut() = Some(new_val); };
    print!("Using toUpper: {}\n", (*{ let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>> = { let mut __f_guard = processor.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some("test".to_string())))) }.borrow().as_ref().unwrap()));

    { let new_val = Box::new(move |s: Rc<RefCell<Option<String>>>| -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some(format!("{}{}", "processed: ".to_string(), (*s.borrow().as_ref().unwrap())))));
    }) as Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>>; *processor.borrow_mut() = Some(new_val); };
    print!("Using anonymous: {}\n", (*{ let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>> = { let mut __f_guard = processor.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some("test".to_string())))) }.borrow().as_ref().unwrap()));
}