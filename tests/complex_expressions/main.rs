use std::any::Any;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};


struct GoChannel<T> {
    tx: std::sync::Arc<std::sync::Mutex<Option<std::sync::mpsc::SyncSender<T>>>>,
    rx: std::sync::Arc<std::sync::Mutex<std::sync::mpsc::Receiver<T>>>,
}

impl<T> GoChannel<T> {
    fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel(0);
        GoChannel {
            tx: std::sync::Arc::new(std::sync::Mutex::new(Some(tx))),
            rx: std::sync::Arc::new(std::sync::Mutex::new(rx)),
        }
    }

    fn new_buffered(cap: usize) -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel(cap);
        GoChannel {
            tx: std::sync::Arc::new(std::sync::Mutex::new(Some(tx))),
            rx: std::sync::Arc::new(std::sync::Mutex::new(rx)),
        }
    }

    fn send(&self, val: T) {
        if let Some(ref tx) = *self.tx.lock().unwrap() {
            let _ = tx.send(val);
        }
    }

    fn try_send(&self, val: T) -> bool {
        if let Some(ref tx) = *self.tx.lock().unwrap() {
            tx.try_send(val).is_ok()
        } else {
            false
        }
    }

    fn recv(&self) -> Option<T> {
        self.rx.lock().unwrap().recv().ok()
    }

    fn try_recv(&self) -> Option<T> {
        self.rx.lock().unwrap().try_recv().ok()
    }

    fn close(&self) {
        *self.tx.lock().unwrap() = None;
    }
}

impl<T> Clone for GoChannel<T> {
    fn clone(&self) -> Self {
        GoChannel {
            tx: self.tx.clone(),
            rx: self.rx.clone(),
        }
    }
}

impl<T> Iterator for GoChannel<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.recv()
    }
}

#[derive(Debug, Clone, Default)]
struct AnonymousStruct1 {
    x: Arc<Mutex<Option<i32>>>,
    y: Arc<Mutex<Option<i32>>>,
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.x.lock().unwrap().as_ref().unwrap()), (*self.y.lock().unwrap().as_ref().unwrap()))
    }
}


fn main() {
        // Complex arithmetic expressions
    println!("{}", "=== Complex arithmetic expressions ===".to_string());

    let (mut a, mut b, mut c) = (Arc::new(Mutex::new(Some(10))), Arc::new(Mutex::new(Some(20))), Arc::new(Mutex::new(Some(30))));

        // Nested arithmetic
    let mut result1 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = ({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }); let __tmp_y = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let __tmp_y = { let __tmp_x = ({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }); let __tmp_y = ({ let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); __tmp_x / __tmp_y }; __tmp_x - __tmp_y })));
    print!("(a + b) * c - (a * b) / (c - a) = {}\n", { let __v = (*result1.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Mixed operations with precedence
    let mut result2 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let __tmp_y = ({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 5; __tmp_x - __tmp_y }); __tmp_x / __tmp_y }; __tmp_x + __tmp_y }; let __tmp_y = { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x % __tmp_y }; __tmp_x + __tmp_y })));
    print!("a + b * c / (a - 5) + c % b = {}\n", { let __v = (*result2.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Boolean expressions
    println!("{}", "\n=== Complex boolean expressions ===".to_string());

    let (mut x, mut y, mut z) = (Arc::new(Mutex::new(Some(5))), Arc::new(Mutex::new(Some(10))), Arc::new(Mutex::new(Some(15))));

        // Complex boolean logic
    let mut bool1 = Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y }) && ({ let __tmp_x = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*z.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y }) || ({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 5; __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*z.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10; __tmp_x > __tmp_y }))));
    print!("(x < y) && (y < z) || (x == 5 && z > 10) = {}\n", { let __v = (*bool1.lock().unwrap().as_ref().unwrap()).clone(); __v });

    let mut bool2 = Arc::new(Mutex::new(Some(!({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y }) && ({ let __tmp_x = { let __tmp_x = { let __v = (*z.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y }) || ({ let __tmp_x = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x * __tmp_y }; let __tmp_y = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y }))));
    print!("!(x > y) && (z-y == x) || (x*2 == y) = {}\n", { let __v = (*bool2.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Bitwise operations
    println!("{}", "\n=== Complex bitwise expressions ===".to_string());

    let (mut bits1, mut bits2) = (Arc::new(Mutex::new(Some(0b1010))), Arc::new(Mutex::new(Some(0b1100))));

    let mut bitwiseResult = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __v = (*bits1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*bits2.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y }); let __tmp_y = { let __tmp_x = ({ let __tmp_x = { let __v = (*bits1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*bits2.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x ^ __tmp_y }); let __tmp_y = 1; __tmp_x << __tmp_y }; __tmp_x | __tmp_y })));
    print!("(bits1 & bits2) | (bits1 ^ bits2) << 1 = {:b} ({})\n", { let __v = (*bitwiseResult.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*bitwiseResult.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Function calls in expressions
    println!("{}", "\n=== Function calls in expressions ===".to_string());

    let mut getValue = Arc::new(Mutex::new(Some(Box::new(move |n: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<i32>>> {
        return {
            let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v };
            let __tmp_y = 2;
            Arc::new(Mutex::new(Some(__tmp_x * __tmp_y)))
        };
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>)));
    let mut getMultiplier = Arc::new(Mutex::new(Some(Box::new(move || -> Arc<Mutex<Option<i32>>> {
        return Arc::new(Mutex::new(Some(3)));
    }) as Box<dyn Fn() -> Arc<Mutex<Option<i32>>> + Send + Sync>)));

    let mut complexResult = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = (*{ let __f_guard = getValue.lock().unwrap(); let __f = __f_guard.as_ref().unwrap(); (*__f)(a.clone()) }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = (*{ let __f_guard = getValue.lock().unwrap(); let __f = __f_guard.as_ref().unwrap(); (*__f)(b.clone()) }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __f_guard = getMultiplier.lock().unwrap(); let __f = __f_guard.as_ref().unwrap(); (*__f)() }.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }; __tmp_x + __tmp_y }; let __tmp_y = { let __tmp_x = (*{ let __f_guard = getValue.lock().unwrap(); let __f = __f_guard.as_ref().unwrap(); (*__f)(c.clone()) }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 2; __tmp_x / __tmp_y }; __tmp_x - __tmp_y })));
    print!("getValue(a) + getValue(b) * getMultiplier() - getValue(c)/2 = {}\n", { let __v = (*complexResult.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Array/slice expressions
    println!("{}", "\n=== Array/slice expressions ===".to_string());

    let mut numbers = Arc::new(Mutex::new(Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])));

        // Complex indexing
    let (mut idx1, mut idx2) = (Arc::new(Mutex::new(Some(2))), Arc::new(Mutex::new(Some(7))));
    let mut sliceResult = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __seq = { let __seq_holder = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = numbers.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[{ let __v = (*idx1.lock().unwrap().as_ref().unwrap()).clone(); __v } as usize..{ let __v = (*idx2.lock().unwrap().as_ref().unwrap()).clone(); __v } as usize].to_vec() }))).clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[1 as usize].clone() }; let __tmp_y = { let __seq = { let __seq_holder = numbers.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[{ let __tmp_x = (*numbers.lock().unwrap().as_ref().unwrap()).len(); let __tmp_y = 1; __tmp_x - __tmp_y } as usize].clone() }; __tmp_x + __tmp_y }; let __tmp_y = { let __seq = { let __seq_holder = numbers.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[0 as usize].clone() }; __tmp_x - __tmp_y })));
    print!("numbers[idx1:idx2][1] + numbers[len(numbers)-1] - numbers[0] = {}\n", { let __v = (*sliceResult.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Map expressions
    println!("{}", "\n=== Map expressions ===".to_string());

    let mut data = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i32>>>>::from([("alpha".to_string(), Arc::new(Mutex::new(Some(10)))), ("beta".to_string(), Arc::new(Mutex::new(Some(20)))), ("gamma".to_string(), Arc::new(Mutex::new(Some(30))))]))));

    let mut mapResult = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __map = { let __map_holder = data.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = (*__map_guard.as_ref().unwrap()).clone(); drop(__map_guard); __cloned }; __map.get(&"alpha".to_string()).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }; let __tmp_y = { let __tmp_x = { let __map = { let __map_holder = data.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = (*__map_guard.as_ref().unwrap()).clone(); drop(__map_guard); __cloned }; __map.get(&"beta".to_string()).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }; let __tmp_y = 2; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }; let __tmp_y = { let __tmp_x = { let __map = { let __map_holder = data.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = (*__map_guard.as_ref().unwrap()).clone(); drop(__map_guard); __cloned }; __map.get(&"gamma".to_string()).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }; let __tmp_y = 3; __tmp_x / __tmp_y }; __tmp_x - __tmp_y })));
    print!("data[\"alpha\"] + data[\"beta\"]*2 - data[\"gamma\"]/3 = {}\n", { let __v = (*mapResult.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Struct field expressions
    println!("{}", "\n=== Struct field expressions ===".to_string());

    type Point = AnonymousStruct1;

    let mut p1 = Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(3))), y: Arc::new(Mutex::new(Some(4))), ..Default::default() })));
    let mut p2 = Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(6))), y: Arc::new(Mutex::new(Some(8))), ..Default::default() })));

        // Distance calculation (without sqrt for simplicity)
    let mut distanceSquared = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = ({ let __tmp_x = (*{ let __field = (*p2.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*p1.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }); let __tmp_y = ({ let __tmp_x = (*{ let __field = (*p2.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*p1.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }); __tmp_x * __tmp_y }; let __tmp_y = { let __tmp_x = ({ let __tmp_x = (*{ let __field = (*p2.lock().unwrap().as_ref().unwrap()).y.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*p1.lock().unwrap().as_ref().unwrap()).y.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }); let __tmp_y = ({ let __tmp_x = (*{ let __field = (*p2.lock().unwrap().as_ref().unwrap()).y.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*p1.lock().unwrap().as_ref().unwrap()).y.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }); __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));
    print!("Distance squared between points: {}\n", { let __v = (*distanceSquared.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Pointer expressions
    println!("{}", "\n=== Pointer expressions ===".to_string());

    let mut val = Arc::new(Mutex::new(Some(42)));
    let mut ptr = val.clone();

    let mut ptrResult = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*ptr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = { let __v = (*ptr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x * __tmp_y }); __tmp_x + __tmp_y }; let __tmp_y = ({ let __tmp_x = { let __v = (*ptr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x / __tmp_y }); __tmp_x - __tmp_y })));
    print!("*ptr + (*ptr * 2) - (*ptr / 2) = {}\n", { let __v = (*ptrResult.lock().unwrap().as_ref().unwrap()).clone(); __v });

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
    if { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        let mut assertResult = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*intVal.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x * __tmp_y }; let __tmp_y = { let __tmp_x = ({ let __tmp_x = { let __v = (*intVal.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 5; __tmp_x / __tmp_y }); let __tmp_y = 3; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));
        print!("Type assertion result: {}\n", { let __v = (*assertResult.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }

        // Channel expressions (non-blocking)
    println!("{}", "\n=== Channel expressions ===".to_string());

    let mut ch = GoChannel::<i32>::new_buffered(3 as usize);
    ch.send(10);
    ch.send(20);
    ch.send(30);

    let mut chanResult = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = ch.recv().unwrap(); let __tmp_y = { let __tmp_x = ch.recv().unwrap(); let __tmp_y = 2; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }; let __tmp_y = { let __tmp_x = ch.recv().unwrap(); let __tmp_y = 2; __tmp_x / __tmp_y }; __tmp_x - __tmp_y })));
    print!("Channel expression result: {}\n", { let __v = (*chanResult.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Nested function calls
    println!("{}", "\n=== Nested function calls ===".to_string());

    let mut add = Arc::new(Mutex::new(Some(Box::new(move |a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<i32>>> {
        return {
            let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v };
            let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v };
            Arc::new(Mutex::new(Some(__tmp_x + __tmp_y)))
        };
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>)));
    let mut multiply = Arc::new(Mutex::new(Some(Box::new(move |a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<i32>>> {
        return {
            let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v };
            let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v };
            Arc::new(Mutex::new(Some(__tmp_x * __tmp_y)))
        };
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>)));
    let mut subtract = Arc::new(Mutex::new(Some(Box::new(move |a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<i32>>> {
        return {
            let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v };
            let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v };
            Arc::new(Mutex::new(Some(__tmp_x - __tmp_y)))
        };
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>)));

    let mut nestedResult = { let __f_guard = add.lock().unwrap(); let __f = __f_guard.as_ref().unwrap(); (*__f)({ let __f_guard = multiply.lock().unwrap(); let __f = __f_guard.as_ref().unwrap(); (*__f)(Arc::new(Mutex::new(Some(3))), Arc::new(Mutex::new(Some(4)))) }, { let __f_guard = subtract.lock().unwrap(); let __f = __f_guard.as_ref().unwrap(); (*__f)(Arc::new(Mutex::new(Some(20))), { let __f_guard = multiply.lock().unwrap(); let __f = __f_guard.as_ref().unwrap(); (*__f)(Arc::new(Mutex::new(Some(2))), Arc::new(Mutex::new(Some(5)))) }) }) };
    print!("add(multiply(3, 4), subtract(20, multiply(2, 5))) = {}\n", { let __v = (*nestedResult.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Complex conditional expressions
    println!("{}", "\n=== Complex conditional expressions ===".to_string());

    let mut score = Arc::new(Mutex::new(Some(85)));
    let mut grade: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));

        // Ternary-like using if-else
    if { let __tmp_x = { let __v = (*score.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 90; __tmp_x >= __tmp_y } {
        { let new_val = "A".to_string(); *grade.lock().unwrap() = Some(new_val); };
    } else if { let __tmp_x = { let __v = (*score.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 80; __tmp_x >= __tmp_y } {
        { let new_val = "B".to_string(); *grade.lock().unwrap() = Some(new_val); };
    } else if { let __tmp_x = { let __v = (*score.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 70; __tmp_x >= __tmp_y } {
        { let new_val = "C".to_string(); *grade.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = "F".to_string(); *grade.lock().unwrap() = Some(new_val); };
    }

    print!("Grade for score {}: {}\n", { let __v = (*score.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*grade.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Complex assignment expressions
    println!("{}", "\n=== Complex assignment expressions ===".to_string());

    let mut counter = Arc::new(Mutex::new(Some(0)));
    { let mut guard = counter.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + { let __tmp_x = { let __tmp_x = ({ let __tmp_x = 5; let __tmp_y = 3; __tmp_x * __tmp_y }); let __tmp_y = ({ let __tmp_x = 10; let __tmp_y = 2; __tmp_x / __tmp_y }); __tmp_x - __tmp_y }; let __tmp_y = ({ let __tmp_x = 8; let __tmp_y = 3; __tmp_x % __tmp_y }); __tmp_x + __tmp_y }); };
    print!("Complex assignment result: {}\n", { let __v = (*counter.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Multiple assignment with expressions
    let (mut sum, mut product) = (Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let __tmp_y = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }))));
    print!("Sum: {}, Product: {}\n", { let __v = (*sum.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*product.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Range expressions
    println!("{}", "\n=== Range expressions ===".to_string());

    let mut total = Arc::new(Mutex::new(Some(0)));
    for (i, val) in { let __seq = { let __seq_holder = numbers.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[..5 as usize].to_vec() }.iter().copied().enumerate() {
        { let mut guard = total.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + { let __tmp_x = { let __tmp_x = i; let __tmp_y = val; __tmp_x * __tmp_y }; let __tmp_y = ({ let __tmp_x = val; let __tmp_y = 3; __tmp_x % __tmp_y }); __tmp_x + __tmp_y }); };
    }
    print!("Complex range calculation: {}\n", { let __v = (*total.lock().unwrap().as_ref().unwrap()).clone(); __v });
}