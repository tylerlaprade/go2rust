use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default)]
struct AnonymousStruct1 {
    x: Arc<Mutex<Option<i32>>>,
    y: Arc<Mutex<Option<i32>>>,
}


fn main() {
        // Complex arithmetic expressions
    println!("{}", "=== Complex arithmetic expressions ===".to_string());

    let (mut a, mut b, mut c) = (Arc::new(Mutex::new(Some(10))), Arc::new(Mutex::new(Some(20))), Arc::new(Mutex::new(Some(30))));

        // Nested arithmetic
    let mut result1 = Arc::new(Mutex::new(Some(((*a.lock().unwrap().as_mut().unwrap()) + (*b.lock().unwrap().as_mut().unwrap())) * (*c.lock().unwrap().as_mut().unwrap()) - ((*a.lock().unwrap().as_mut().unwrap()) * (*b.lock().unwrap().as_mut().unwrap())) / ((*c.lock().unwrap().as_mut().unwrap()) - (*a.lock().unwrap().as_mut().unwrap())))));
    print!("(a + b) * c - (a * b) / (c - a) = {}\n", (*result1.lock().unwrap().as_mut().unwrap()));

        // Mixed operations with precedence
    let mut result2 = Arc::new(Mutex::new(Some((*a.lock().unwrap().as_mut().unwrap()) + (*b.lock().unwrap().as_mut().unwrap()) * (*c.lock().unwrap().as_mut().unwrap()) / ((*a.lock().unwrap().as_mut().unwrap()) - 5) + (*c.lock().unwrap().as_mut().unwrap()) % (*b.lock().unwrap().as_mut().unwrap()))));
    print!("a + b * c / (a - 5) + c % b = {}\n", (*result2.lock().unwrap().as_mut().unwrap()));

        // Boolean expressions
    println!("{}", "\n=== Complex boolean expressions ===".to_string());

    let (mut x, mut y, mut z) = (Arc::new(Mutex::new(Some(5))), Arc::new(Mutex::new(Some(10))), Arc::new(Mutex::new(Some(15))));

        // Complex boolean logic
    let mut bool1 = Arc::new(Mutex::new(Some(((*x.lock().unwrap().as_mut().unwrap()) < (*y.lock().unwrap().as_mut().unwrap())) && ((*y.lock().unwrap().as_mut().unwrap()) < (*z.lock().unwrap().as_mut().unwrap())) || ((*x.lock().unwrap().as_mut().unwrap()) == 5 && (*z.lock().unwrap().as_mut().unwrap()) > 10))));
    print!("(x < y) && (y < z) || (x == 5 && z > 10) = {}\n", (*bool1.lock().unwrap().as_mut().unwrap()));

    let mut bool2 = Arc::new(Mutex::new(Some((*!((*x.lock().unwrap().as_mut().unwrap()) > (*y.lock().unwrap().as_mut().unwrap())).lock().unwrap().as_ref().unwrap()) && ((*z.lock().unwrap().as_mut().unwrap()) - (*y.lock().unwrap().as_mut().unwrap()) == (*x.lock().unwrap().as_mut().unwrap())) || ((*x.lock().unwrap().as_mut().unwrap()) * 2 == (*y.lock().unwrap().as_mut().unwrap())))));
    print!("!(x > y) && (z-y == x) || (x*2 == y) = {}\n", (*bool2.lock().unwrap().as_mut().unwrap()));

        // Bitwise operations
    println!("{}", "\n=== Complex bitwise expressions ===".to_string());

    let (mut bits1, mut bits2) = (Arc::new(Mutex::new(Some(0b1010))), Arc::new(Mutex::new(Some(0b1100))));

    let mut bitwiseResult = Arc::new(Mutex::new(Some(((*bits1.lock().unwrap().as_mut().unwrap()) & (*bits2.lock().unwrap().as_mut().unwrap())) | ((*bits1.lock().unwrap().as_mut().unwrap()) ^ (*bits2.lock().unwrap().as_mut().unwrap())) << 1)));
    print!("(bits1 & bits2) | (bits1 ^ bits2) << 1 = {:b} ({})\n", (*bitwiseResult.lock().unwrap().as_mut().unwrap()), (*bitwiseResult.lock().unwrap().as_mut().unwrap()));

        // Function calls in expressions
    println!("{}", "\n=== Function calls in expressions ===".to_string());

    let mut getValue = Arc::new(Mutex::new(Some(Box::new(move |n: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<i32>>> {
        return {
            let __tmp_x = (*n.lock().unwrap().as_mut().unwrap());
            let __tmp_y = 2;
            Arc::new(Mutex::new(Some(__tmp_x * __tmp_y)))
        };
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>)));
    let mut getMultiplier = Arc::new(Mutex::new(Some(Box::new(move || -> Arc<Mutex<Option<i32>>> {
        return Arc::new(Mutex::new(Some(3)));
    }) as Box<dyn Fn() -> Arc<Mutex<Option<i32>>> + Send + Sync>)));

    let mut complexResult = Arc::new(Mutex::new(Some((*(*getValue.lock().unwrap().as_ref().unwrap())(a.clone()).lock().unwrap().as_ref().unwrap()) + (*(*getValue.lock().unwrap().as_ref().unwrap())(b.clone()).lock().unwrap().as_ref().unwrap()) * (*(*getMultiplier.lock().unwrap().as_ref().unwrap())().lock().unwrap().as_ref().unwrap()) - (*(*getValue.lock().unwrap().as_ref().unwrap())(c.clone()).lock().unwrap().as_ref().unwrap()) / 2)));
    print!("getValue(a) + getValue(b) * getMultiplier() - getValue(c)/2 = {}\n", (*complexResult.lock().unwrap().as_mut().unwrap()));

        // Array/slice expressions
    println!("{}", "\n=== Array/slice expressions ===".to_string());

    let mut numbers = Arc::new(Mutex::new(Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])));

        // Complex indexing
    let (mut idx1, mut idx2) = (Arc::new(Mutex::new(Some(2))), Arc::new(Mutex::new(Some(7))));
    let mut sliceResult = Arc::new(Mutex::new(Some((*(*Arc::new(Mutex::new(Some((*numbers.lock().unwrap().as_ref().unwrap())[(*idx1.lock().unwrap().as_mut().unwrap()) as usize..(*idx2.lock().unwrap().as_mut().unwrap()) as usize].to_vec()))).lock().unwrap().as_ref().unwrap())[1 as usize].clone().lock().unwrap().as_ref().unwrap()) + (*(*numbers.lock().unwrap().as_ref().unwrap())[(*numbers.lock().unwrap().as_ref().unwrap()).len() - 1 as usize].clone().lock().unwrap().as_ref().unwrap()) - (*(*numbers.lock().unwrap().as_ref().unwrap())[0 as usize].clone().lock().unwrap().as_ref().unwrap()))));
    print!("numbers[idx1:idx2][1] + numbers[len(numbers)-1] - numbers[0] = {}\n", (*sliceResult.lock().unwrap().as_mut().unwrap()));

        // Map expressions
    println!("{}", "\n=== Map expressions ===".to_string());

    let mut data = Arc::new(Mutex::new(Some(HashMap::<String, Arc<Mutex<Option<i32>>>>::from([("alpha".to_string(), Arc::new(Mutex::new(Some(10)))), ("beta".to_string(), Arc::new(Mutex::new(Some(20)))), ("gamma".to_string(), Arc::new(Mutex::new(Some(30))))]))));

    let mut mapResult = Arc::new(Mutex::new(Some((*(*(*data.lock().unwrap().as_ref().unwrap()).get(&"alpha".to_string()).unwrap().lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()) + (*(*(*data.lock().unwrap().as_ref().unwrap()).get(&"beta".to_string()).unwrap().lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()) * 2 - (*(*(*data.lock().unwrap().as_ref().unwrap()).get(&"gamma".to_string()).unwrap().lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()) / 3)));
    print!("data[\"alpha\"] + data[\"beta\"]*2 - data[\"gamma\"]/3 = {}\n", (*mapResult.lock().unwrap().as_mut().unwrap()));

        // Struct field expressions
    println!("{}", "\n=== Struct field expressions ===".to_string());

    type Point = Arc<Mutex<Option<AnonymousStruct1>>>;

    let mut p1 = Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(3))), y: Arc::new(Mutex::new(Some(4))) })));
    let mut p2 = Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(6))), y: Arc::new(Mutex::new(Some(8))) })));

        // Distance calculation (without sqrt for simplicity)
    let mut distanceSquared = Arc::new(Mutex::new(Some(((*(*(*p2.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()) - (*(*(*p1.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap())) * ((*(*(*p2.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()) - (*(*(*p1.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap())) + ((*(*(*p2.lock().unwrap().as_ref().unwrap()).y.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()) - (*(*(*p1.lock().unwrap().as_ref().unwrap()).y.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap())) * ((*(*(*p2.lock().unwrap().as_ref().unwrap()).y.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()) - (*(*(*p1.lock().unwrap().as_ref().unwrap()).y.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap())))));
    print!("Distance squared between points: {}\n", (*distanceSquared.lock().unwrap().as_mut().unwrap()));

        // Pointer expressions
    println!("{}", "\n=== Pointer expressions ===".to_string());

    let mut val = Arc::new(Mutex::new(Some(42)));
    let mut ptr = val.clone();

    let mut ptrResult = Arc::new(Mutex::new(Some((*(*ptr.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()) + ((*(*ptr.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()) * 2) - ((*(*ptr.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()) / 2))));
    print!("*ptr + (*ptr * 2) - (*ptr / 2) = {}\n", (*ptrResult.lock().unwrap().as_mut().unwrap()));

        // Type assertion expressions
    println!("{}", "\n=== Type assertion expressions ===".to_string());

    let mut iface: Arc<Mutex<Option<Box<dyn Any>>>> = Arc::new(Mutex::new(Some(Box::new(100) as Box<dyn Any>)));

    let (mut intVal, mut ok) = ({
        let val = iface.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<i32>() {
                (Arc::new(Mutex::new(Some(typed_val.clone()))), Arc::new(Mutex::new(Some(true))))
            } else {
                (Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(false))))
            }
        } else {
            (Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(false))))
        }
    });
    if (*ok.lock().unwrap().as_mut().unwrap()) {
        let mut assertResult = Arc::new(Mutex::new(Some((*intVal.lock().unwrap().as_mut().unwrap()) * 2 + ((*intVal.lock().unwrap().as_mut().unwrap()) / 5) * 3)));
        print!("Type assertion result: {}\n", (*assertResult.lock().unwrap().as_mut().unwrap()));
    }

        // Channel expressions (non-blocking)
    println!("{}", "\n=== Channel expressions ===".to_string());

    let mut ch = ;
    // TODO: Unhandled statement type: SendStmt
    // TODO: Unhandled statement type: SendStmt
    // TODO: Unhandled statement type: SendStmt

    let mut chanResult = Arc::new(Mutex::new(Some((*<-(*ch.lock().unwrap().as_mut().unwrap()).lock().unwrap().as_ref().unwrap()) + (*<-(*ch.lock().unwrap().as_mut().unwrap()).lock().unwrap().as_ref().unwrap()) * 2 - (*<-(*ch.lock().unwrap().as_mut().unwrap()).lock().unwrap().as_ref().unwrap()) / 2)));
    print!("Channel expression result: {}\n", (*chanResult.lock().unwrap().as_mut().unwrap()));

        // Nested function calls
    println!("{}", "\n=== Nested function calls ===".to_string());

    let mut add = Arc::new(Mutex::new(Some(Box::new(move |a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<i32>>> {
        return {
            let __tmp_x = (*a.lock().unwrap().as_mut().unwrap());
            let __tmp_y = (*b.lock().unwrap().as_mut().unwrap());
            Arc::new(Mutex::new(Some(__tmp_x + __tmp_y)))
        };
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>)));
    let mut multiply = Arc::new(Mutex::new(Some(Box::new(move |a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<i32>>> {
        return {
            let __tmp_x = (*a.lock().unwrap().as_mut().unwrap());
            let __tmp_y = (*b.lock().unwrap().as_mut().unwrap());
            Arc::new(Mutex::new(Some(__tmp_x * __tmp_y)))
        };
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>)));
    let mut subtract = Arc::new(Mutex::new(Some(Box::new(move |a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<i32>>> {
        return {
            let __tmp_x = (*a.lock().unwrap().as_mut().unwrap());
            let __tmp_y = (*b.lock().unwrap().as_mut().unwrap());
            Arc::new(Mutex::new(Some(__tmp_x - __tmp_y)))
        };
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>)));

    let mut nestedResult = (*add.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some((*multiply.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(3))), Arc::new(Mutex::new(Some(4))))))), Arc::new(Mutex::new(Some((*subtract.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(20))), Arc::new(Mutex::new(Some((*multiply.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(2))), Arc::new(Mutex::new(Some(5))))))))))));
    print!("add(multiply(3, 4), subtract(20, multiply(2, 5))) = {}\n", (*nestedResult.lock().unwrap().as_mut().unwrap()));

        // Complex conditional expressions
    println!("{}", "\n=== Complex conditional expressions ===".to_string());

    let mut score = Arc::new(Mutex::new(Some(85)));
    let mut grade: Arc<Mutex<Option<String>>> = String::new();

        // Ternary-like using if-else
    if (*score.lock().unwrap().as_mut().unwrap()) >= 90 {
        { let new_val = "A".to_string(); *grade.lock().unwrap() = Some(new_val); };
    } else if (*score.lock().unwrap().as_mut().unwrap()) >= 80 {
        { let new_val = "B".to_string(); *grade.lock().unwrap() = Some(new_val); };
    } else if (*score.lock().unwrap().as_mut().unwrap()) >= 70 {
        { let new_val = "C".to_string(); *grade.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = "F".to_string(); *grade.lock().unwrap() = Some(new_val); };
    }

    print!("Grade for score {}: {}\n", (*score.lock().unwrap().as_mut().unwrap()), (*grade.lock().unwrap().as_mut().unwrap()));

        // Complex assignment expressions
    println!("{}", "\n=== Complex assignment expressions ===".to_string());

    let mut counter = Arc::new(Mutex::new(Some(0)));
    { let mut guard = counter.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + (5 * 3) - (10 / 2) + (8 % 3)); };
    print!("Complex assignment result: {}\n", (*counter.lock().unwrap().as_mut().unwrap()));

        // Multiple assignment with expressions
    let (mut sum, mut product) = (Arc::new(Mutex::new(Some((*a.lock().unwrap().as_mut().unwrap()) + (*b.lock().unwrap().as_mut().unwrap()) + (*c.lock().unwrap().as_mut().unwrap())))), Arc::new(Mutex::new(Some((*a.lock().unwrap().as_mut().unwrap()) * (*b.lock().unwrap().as_mut().unwrap()) * (*c.lock().unwrap().as_mut().unwrap())))));
    print!("Sum: {}, Product: {}\n", (*sum.lock().unwrap().as_mut().unwrap()), (*product.lock().unwrap().as_mut().unwrap()));

        // Range expressions
    println!("{}", "\n=== Range expressions ===".to_string());

    let mut total = Arc::new(Mutex::new(Some(0)));
    for (i, val) in Arc::new(Mutex::new(Some((*numbers.lock().unwrap().as_ref().unwrap())[..5 as usize].to_vec()))).iter().enumerate() {
        { let mut guard = total.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + i * val + (val % 3)); };
    }
    print!("Complex range calculation: {}\n", (*total.lock().unwrap().as_mut().unwrap()));
}