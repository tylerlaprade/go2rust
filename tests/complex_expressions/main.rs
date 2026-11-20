use std::any::Any;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};


struct GoChannel<T> {
    tx: std::sync::Arc<std::sync::Mutex<Option<std::sync::mpsc::SyncSender<T>>>>,
    rx: std::sync::Arc<std::sync::Mutex<std::sync::mpsc::Receiver<T>>>,
    is_nil: std::sync::Arc<std::sync::atomic::AtomicBool>,
    len: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    capacity: usize,
}

impl<T> GoChannel<T> {
    fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel(0);
        GoChannel {
            tx: std::sync::Arc::new(std::sync::Mutex::new(Some(tx))),
            rx: std::sync::Arc::new(std::sync::Mutex::new(rx)),
            is_nil: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            len: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            capacity: 0,
        }
    }

    fn new_buffered(cap: usize) -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel(cap);
        GoChannel {
            tx: std::sync::Arc::new(std::sync::Mutex::new(Some(tx))),
            rx: std::sync::Arc::new(std::sync::Mutex::new(rx)),
            is_nil: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            len: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            capacity: cap,
        }
    }

    fn send(&self, val: T) {
        if self.is_nil() {
            return;
        }
        let tx = self.tx.lock().unwrap().clone();
        if let Some(tx) = tx {
            if tx.send(val).is_ok() && self.capacity > 0 {
                self.len.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        }
    }

    fn try_send(&self, val: T) -> bool {
        if self.is_nil() {
            return false;
        }
        let tx = self.tx.lock().unwrap().clone();
        if let Some(tx) = tx {
            if tx.try_send(val).is_ok() {
                if self.capacity > 0 {
                    self.len.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn recv(&self) -> Option<T> {
        if self.is_nil() {
            return None;
        }
        let value = self.rx.lock().unwrap().recv().ok();
        if value.is_some() && self.capacity > 0 {
            let _ = self.len.fetch_update(
                std::sync::atomic::Ordering::SeqCst,
                std::sync::atomic::Ordering::SeqCst,
                |__go_current| __go_current.checked_sub(1),
            );
        }
        value
    }

    fn try_recv(&self) -> Option<T> {
        if self.is_nil() {
            return None;
        }
        let value = self.rx.lock().unwrap().try_recv().ok();
        if value.is_some() && self.capacity > 0 {
            let _ = self.len.fetch_update(
                std::sync::atomic::Ordering::SeqCst,
                std::sync::atomic::Ordering::SeqCst,
                |__go_current| __go_current.checked_sub(1),
            );
        }
        value
    }

    // A select case's receive: Some(Some(v)) for a value, Some(None) when
    // the channel is closed and drained, None when nothing is ready yet.
    fn try_recv_select(&self) -> Option<Option<T>> {
        if self.is_nil() {
            return None;
        }
        match self.rx.lock().unwrap().try_recv() {
            Ok(value) => {
                if self.capacity > 0 {
                    let _ = self.len.fetch_update(
                        std::sync::atomic::Ordering::SeqCst,
                        std::sync::atomic::Ordering::SeqCst,
                        |__go_current| __go_current.checked_sub(1),
                    );
                }
                Some(Some(value))
            }
            Err(std::sync::mpsc::TryRecvError::Disconnected) => Some(None),
            Err(std::sync::mpsc::TryRecvError::Empty) => None,
        }
    }

    fn close(&self) {
        *self.tx.lock().unwrap() = None;
    }

    fn is_nil(&self) -> bool {
        self.is_nil.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn len(&self) -> usize {
        self.len.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn capacity(&self) -> usize {
        self.capacity
    }
}

impl<T> Clone for GoChannel<T> {
    fn clone(&self) -> Self {
        GoChannel {
            tx: self.tx.clone(),
            rx: self.rx.clone(),
            is_nil: self.is_nil.clone(),
            len: self.len.clone(),
            capacity: self.capacity,
        }
    }
}

impl<T> Default for GoChannel<T> {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel(0);
        GoChannel {
            tx: std::sync::Arc::new(std::sync::Mutex::new(Some(tx))),
            rx: std::sync::Arc::new(std::sync::Mutex::new(rx)),
            is_nil: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true)),
            len: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            capacity: 0,
        }
    }
}

impl<T> std::fmt::Debug for GoChannel<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GoChannel")
    }
}

impl<T> Iterator for GoChannel<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.recv()
    }
}

fn main() {
        // Complex arithmetic expressions
    println!("{}", format!("{}", "=== Complex arithmetic expressions ===".to_string()));

    let (mut a, mut b, mut c) = (Arc::new(Mutex::new(Some(10))), Arc::new(Mutex::new(Some(20))), Arc::new(Mutex::new(Some(30))));

        // Nested arithmetic
    let __go_binary_0 = (*a.lock().unwrap().as_ref().unwrap());
let __go_binary_1 = (*b.lock().unwrap().as_ref().unwrap());
let __go_binary_2 = __go_binary_0 + __go_binary_1;
let __go_binary_3 = (*c.lock().unwrap().as_ref().unwrap());
let __go_binary_4 = __go_binary_2 * __go_binary_3;
let __go_binary_5 = (*a.lock().unwrap().as_ref().unwrap());
let __go_binary_6 = (*b.lock().unwrap().as_ref().unwrap());
let __go_binary_7 = __go_binary_5 * __go_binary_6;
let __go_binary_8 = (*c.lock().unwrap().as_ref().unwrap());
let __go_binary_9 = (*a.lock().unwrap().as_ref().unwrap());
let __go_binary_10 = __go_binary_8 - __go_binary_9;
let __go_binary_11 = __go_binary_7 / __go_binary_10;
let __go_binary_12 = __go_binary_4 - __go_binary_11;
let mut result1 = Arc::new(Mutex::new(Some(__go_binary_12)));
    print!("(a + b) * c - (a * b) / (c - a) = {}\n", { let __v = (*result1.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Mixed operations with precedence
    let __go_binary_0 = (*a.lock().unwrap().as_ref().unwrap());
let __go_binary_1 = (*b.lock().unwrap().as_ref().unwrap());
let __go_binary_2 = (*c.lock().unwrap().as_ref().unwrap());
let __go_binary_3 = __go_binary_1 * __go_binary_2;
let __go_binary_4 = (*a.lock().unwrap().as_ref().unwrap());
let __go_binary_5 = 5;
let __go_binary_6 = __go_binary_4 - __go_binary_5;
let __go_binary_7 = __go_binary_3 / __go_binary_6;
let __go_binary_8 = __go_binary_0 + __go_binary_7;
let __go_binary_9 = (*c.lock().unwrap().as_ref().unwrap());
let __go_binary_10 = (*b.lock().unwrap().as_ref().unwrap());
let __go_binary_11 = __go_binary_9 % __go_binary_10;
let __go_binary_12 = __go_binary_8 + __go_binary_11;
let mut result2 = Arc::new(Mutex::new(Some(__go_binary_12)));
    print!("a + b * c / (a - 5) + c % b = {}\n", { let __v = (*result2.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Boolean expressions
    println!("{}", format!("{}", "\n=== Complex boolean expressions ===".to_string()));

    let (mut x, mut y, mut z) = (Arc::new(Mutex::new(Some(5))), Arc::new(Mutex::new(Some(10))), Arc::new(Mutex::new(Some(15))));

        // Complex boolean logic
    let mut bool1 = Arc::new(Mutex::new(Some({
        let __go_cond_0 = {
            let __go_cond_1 = ({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y });
            if __go_cond_1 {
                let __go_cond_2 = ({ let __tmp_x = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*z.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y });
                __go_cond_2
            } else {
                false
            }
        };
        if __go_cond_0 {
            true
        } else {
            let __go_cond_3 = {
                let __go_cond_4 = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 5; __tmp_x == __tmp_y };
                if __go_cond_4 {
                    let __go_cond_5 = { let __tmp_x = { let __v = (*z.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10; __tmp_x > __tmp_y };
                    __go_cond_5
                } else {
                    false
                }
            };
            __go_cond_3
        }
    })));
    print!("(x < y) && (y < z) || (x == 5 && z > 10) = {}\n", { let __v = (*bool1.lock().unwrap().as_ref().unwrap()).clone(); __v });

    let mut bool2 = Arc::new(Mutex::new(Some(!({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y }) && ({ let __tmp_x = { let __tmp_x = { let __v = (*z.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y }) || ({ let __tmp_x = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x * __tmp_y }; let __tmp_y = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y }))));
    print!("!(x > y) && (z-y == x) || (x*2 == y) = {}\n", { let __v = (*bool2.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Bitwise operations
    println!("{}", format!("{}", "\n=== Complex bitwise expressions ===".to_string()));

    let (mut bits1, mut bits2) = (Arc::new(Mutex::new(Some(0b1010))), Arc::new(Mutex::new(Some(0b1100))));

    let __go_binary_0 = (*bits1.lock().unwrap().as_ref().unwrap());
let __go_binary_1 = (*bits2.lock().unwrap().as_ref().unwrap());
let __go_binary_2 = __go_binary_0 & __go_binary_1;
let __go_binary_3 = (*bits1.lock().unwrap().as_ref().unwrap());
let __go_binary_4 = (*bits2.lock().unwrap().as_ref().unwrap());
let __go_binary_5 = __go_binary_3 ^ __go_binary_4;
let __go_binary_6 = 1;
let __go_binary_7 = __go_binary_5 << __go_binary_6;
let __go_binary_8 = __go_binary_2 | __go_binary_7;
let mut bitwiseResult = Arc::new(Mutex::new(Some(__go_binary_8)));
    print!("(bits1 & bits2) | (bits1 ^ bits2) << 1 = {} ({})\n", format!("{:b}", { let __v = (*bitwiseResult.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*bitwiseResult.lock().unwrap().as_ref().unwrap()).clone(); __v }));

        // Function calls in expressions
    println!("{}", format!("{}", "\n=== Function calls in expressions ===".to_string()));

    let mut getValue = Arc::new(Mutex::new(Some(Box::new(move |n: Arc<Mutex<Option<i32>>>| -> i32 {
        return { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x * __tmp_y };
    }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync>)));
    let mut getMultiplier = Arc::new(Mutex::new(Some(Box::new(move || -> i32 {
        3
    }) as Box<dyn FnMut() -> i32 + Send + Sync>)));

    let __go_binary_0 = {
    let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync> = {
        let mut __f_guard = getValue.lock().unwrap();
        __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync>
    };
    let __f = unsafe { &mut *__f_ptr };
    (*__f)(
        a.clone()
    )
};
let __go_binary_1 = {
    let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync> = {
        let mut __f_guard = getValue.lock().unwrap();
        __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync>
    };
    let __f = unsafe { &mut *__f_ptr };
    (*__f)(
        b.clone()
    )
};
let __go_binary_2 = {
    let __f_ptr: *mut Box<dyn FnMut() -> i32 + Send + Sync> = {
        let mut __f_guard = getMultiplier.lock().unwrap();
        __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i32 + Send + Sync>
    };
    let __f = unsafe { &mut *__f_ptr };
    (*__f)(

    )
};
let __go_binary_3 = __go_binary_1 * __go_binary_2;
let __go_binary_4 = __go_binary_0 + __go_binary_3;
let __go_binary_5 = {
    let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync> = {
        let mut __f_guard = getValue.lock().unwrap();
        __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync>
    };
    let __f = unsafe { &mut *__f_ptr };
    (*__f)(
        c.clone()
    )
};
let __go_binary_6 = 2;
let __go_binary_7 = __go_binary_5 / __go_binary_6;
let __go_binary_8 = __go_binary_4 - __go_binary_7;
let mut complexResult = Arc::new(Mutex::new(Some(__go_binary_8)));
    print!("getValue(a) + getValue(b) * getMultiplier() - getValue(c)/2 = {}\n", { let __v = (*complexResult.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Array/slice expressions
    println!("{}", format!("{}", "\n=== Array/slice expressions ===".to_string()));

    let mut numbers = Arc::new(Mutex::new(Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])));

        // Complex indexing
    let (mut idx1, mut idx2) = (Arc::new(Mutex::new(Some(2))), Arc::new(Mutex::new(Some(7))));
    let mut sliceResult = Arc::new(Mutex::new(Some({
        let __tmp_x = {
            let __tmp_x = { let __seq = { let __seq_holder = Arc::new(Mutex::new(Some({
                let __seq_holder = numbers.clone();
                let __seq_guard = __seq_holder.lock().unwrap();
                let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
                let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
                drop(__seq_guard);
                let __low = ({ let __v = (*idx1.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
                let __high = ({ let __v = (*idx2.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
                let __max = __source_cap;
                if __seq.len() < __high { __seq.resize_with(__high, Default::default); }
                let _slice = &__seq[__low..__high];
                let mut _v = Vec::with_capacity((__max - __low) as usize);
                _v.extend_from_slice(_slice);
                _v
            }))).clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() };
            let __tmp_y = { let __seq = { let __seq_holder = numbers.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = ((*numbers.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            __tmp_x + __tmp_y
        };
        let __tmp_y = { let __seq = { let __seq_holder = numbers.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() };
        __tmp_x - __tmp_y
    })));
    print!("numbers[idx1:idx2][1] + numbers[len(numbers)-1] - numbers[0] = {}\n", { let __v = (*sliceResult.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Map expressions
    println!("{}", format!("{}", "\n=== Map expressions ===".to_string()));

    let mut data = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i32>>>>::from([("alpha".to_string(), Arc::new(Mutex::new(Some(10)))), ("beta".to_string(), Arc::new(Mutex::new(Some(20)))), ("gamma".to_string(), Arc::new(Mutex::new(Some(30))))]))));

    let __go_binary_0 = { let __map = { let __map_holder = data.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"alpha".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) };
let __go_binary_1 = { let __map = { let __map_holder = data.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"beta".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) };
let __go_binary_2 = 2;
let __go_binary_3 = __go_binary_1 * __go_binary_2;
let __go_binary_4 = __go_binary_0 + __go_binary_3;
let __go_binary_5 = { let __map = { let __map_holder = data.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"gamma".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) };
let __go_binary_6 = 3;
let __go_binary_7 = __go_binary_5 / __go_binary_6;
let __go_binary_8 = __go_binary_4 - __go_binary_7;
let mut mapResult = Arc::new(Mutex::new(Some(__go_binary_8)));
    print!("data[\"alpha\"] + data[\"beta\"]*2 - data[\"gamma\"]/3 = {}\n", { let __v = (*mapResult.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Struct field expressions
    println!("{}", format!("{}", "\n=== Struct field expressions ===".to_string()));

    type Point = AnonymousStruct1;

    let mut p1 = Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(3))), y: Arc::new(Mutex::new(Some(4))), ..Default::default() })));
    let mut p2 = Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(6))), y: Arc::new(Mutex::new(Some(8))), ..Default::default() })));

        // Distance calculation (without sqrt for simplicity)
    let __go_binary_0 = (*{ let __field = (*p2.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap());
let __go_binary_1 = (*{ let __field = (*p1.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap());
let __go_binary_2 = __go_binary_0 - __go_binary_1;
let __go_binary_3 = (*{ let __field = (*p2.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap());
let __go_binary_4 = (*{ let __field = (*p1.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap());
let __go_binary_5 = __go_binary_3 - __go_binary_4;
let __go_binary_6 = __go_binary_2 * __go_binary_5;
let __go_binary_7 = (*{ let __field = (*p2.lock().unwrap().as_ref().unwrap()).y.clone(); __field }.lock().unwrap().as_ref().unwrap());
let __go_binary_8 = (*{ let __field = (*p1.lock().unwrap().as_ref().unwrap()).y.clone(); __field }.lock().unwrap().as_ref().unwrap());
let __go_binary_9 = __go_binary_7 - __go_binary_8;
let __go_binary_10 = (*{ let __field = (*p2.lock().unwrap().as_ref().unwrap()).y.clone(); __field }.lock().unwrap().as_ref().unwrap());
let __go_binary_11 = (*{ let __field = (*p1.lock().unwrap().as_ref().unwrap()).y.clone(); __field }.lock().unwrap().as_ref().unwrap());
let __go_binary_12 = __go_binary_10 - __go_binary_11;
let __go_binary_13 = __go_binary_9 * __go_binary_12;
let __go_binary_14 = __go_binary_6 + __go_binary_13;
let mut distanceSquared = Arc::new(Mutex::new(Some(__go_binary_14)));
    print!("Distance squared between points: {}\n", { let __v = (*distanceSquared.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Pointer expressions
    println!("{}", format!("{}", "\n=== Pointer expressions ===".to_string()));

    let mut val = Arc::new(Mutex::new(Some(42)));
    let mut ptr = val.clone();

    let __go_binary_0 = { let __v = (*ptr.lock().unwrap().as_ref().unwrap()).clone(); __v };
let __go_binary_1 = { let __v = (*ptr.lock().unwrap().as_ref().unwrap()).clone(); __v };
let __go_binary_2 = 2;
let __go_binary_3 = __go_binary_1 * __go_binary_2;
let __go_binary_4 = __go_binary_0 + __go_binary_3;
let __go_binary_5 = { let __v = (*ptr.lock().unwrap().as_ref().unwrap()).clone(); __v };
let __go_binary_6 = 2;
let __go_binary_7 = __go_binary_5 / __go_binary_6;
let __go_binary_8 = __go_binary_4 - __go_binary_7;
let mut ptrResult = Arc::new(Mutex::new(Some(__go_binary_8)));
    print!("*ptr + (*ptr * 2) - (*ptr / 2) = {}\n", { let __v = (*ptrResult.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Type assertion expressions
    println!("{}", format!("{}", "\n=== Type assertion expressions ===".to_string()));

    let mut iface: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(100) as Box<dyn Any + Send + Sync>)));

    {
        let (mut intVal, mut ok) = ({
        let val = iface.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<i32>() {
                (Arc::new(Mutex::new(Some(typed_val.clone()))), true)
            } else {
                (Arc::new(Mutex::new(Some(0))), false)
            }
        } else {
            (Arc::new(Mutex::new(Some(0))), false)
        }
    });;
        if ok {
            let __go_binary_0 = (*intVal.lock().unwrap().as_ref().unwrap());
let __go_binary_1 = 2;
let __go_binary_2 = __go_binary_0 * __go_binary_1;
let __go_binary_3 = (*intVal.lock().unwrap().as_ref().unwrap());
let __go_binary_4 = 5;
let __go_binary_5 = __go_binary_3 / __go_binary_4;
let __go_binary_6 = 3;
let __go_binary_7 = __go_binary_5 * __go_binary_6;
let __go_binary_8 = __go_binary_2 + __go_binary_7;
let mut assertResult = Arc::new(Mutex::new(Some(__go_binary_8)));;
            print!("Type assertion result: {}\n", { let __v = (*assertResult.lock().unwrap().as_ref().unwrap()).clone(); __v });;
        }
    }

        // Channel expressions (non-blocking)
    println!("{}", format!("{}", "\n=== Channel expressions ===".to_string()));

    let mut ch = GoChannel::<i32>::new_buffered(3 as usize);
    ch.send(10);
    ch.send(20);
    ch.send(30);

    let __go_binary_0 = ch.recv().unwrap_or_default();
let __go_binary_1 = ch.recv().unwrap_or_default();
let __go_binary_2 = 2;
let __go_binary_3 = __go_binary_1 * __go_binary_2;
let __go_binary_4 = __go_binary_0 + __go_binary_3;
let __go_binary_5 = ch.recv().unwrap_or_default();
let __go_binary_6 = 2;
let __go_binary_7 = __go_binary_5 / __go_binary_6;
let __go_binary_8 = __go_binary_4 - __go_binary_7;
let mut chanResult = Arc::new(Mutex::new(Some(__go_binary_8)));
    print!("Channel expression result: {}\n", { let __v = (*chanResult.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Nested function calls
    println!("{}", format!("{}", "\n=== Nested function calls ===".to_string()));

    let mut add = Arc::new(Mutex::new(Some(Box::new(move |a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>| -> i32 {
        return { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y };
    }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync>)));
    let mut multiply = Arc::new(Mutex::new(Some(Box::new(move |a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>| -> i32 {
        return { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y };
    }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync>)));
    let mut subtract = Arc::new(Mutex::new(Some(Box::new(move |a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>| -> i32 {
        return { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y };
    }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync>)));

    let mut nestedResult = {
        let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync> = {
            let mut __f_guard = add.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(
            Arc::new(Mutex::new(Some({
                let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync> = {
                    let mut __f_guard = multiply.lock().unwrap();
                    __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync>
                };
                let __f = unsafe { &mut *__f_ptr };
                (*__f)(
                    Arc::new(Mutex::new(Some(3))),
                    Arc::new(Mutex::new(Some(4)))
                )
            }))),
            Arc::new(Mutex::new(Some({
                let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync> = {
                    let mut __f_guard = subtract.lock().unwrap();
                    __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync>
                };
                let __f = unsafe { &mut *__f_ptr };
                (*__f)(
                    Arc::new(Mutex::new(Some(20))),
                    Arc::new(Mutex::new(Some({
                        let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync> = {
                            let mut __f_guard = multiply.lock().unwrap();
                            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync>
                        };
                        let __f = unsafe { &mut *__f_ptr };
                        (*__f)(
                            Arc::new(Mutex::new(Some(2))),
                            Arc::new(Mutex::new(Some(5)))
                        )
                    })))
                )
            })))
        )
    };
    print!("add(multiply(3, 4), subtract(20, multiply(2, 5))) = {}\n", nestedResult);

        // Complex conditional expressions
    println!("{}", format!("{}", "\n=== Complex conditional expressions ===".to_string()));

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

    print!("Grade for score {}: {}\n", format!("{}", { let __v = (*score.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*grade.lock().unwrap().as_ref().unwrap()).clone(); __v }));

        // Complex assignment expressions
    println!("{}", format!("{}", "\n=== Complex assignment expressions ===".to_string()));

    let mut counter = Arc::new(Mutex::new(Some(0)));
    { let __rhs = 12; let mut guard = counter.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    print!("Complex assignment result: {}\n", { let __v = (*counter.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Multiple assignment with expressions
    let (mut sum, mut product) = (Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let __tmp_y = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }))));
    print!("Sum: {}, Product: {}\n", format!("{}", { let __v = (*sum.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*product.lock().unwrap().as_ref().unwrap()).clone(); __v }));

        // Range expressions
    println!("{}", format!("{}", "\n=== Range expressions ===".to_string()));

    let mut total = Arc::new(Mutex::new(Some(0)));
    for (i, val) in {
        let __seq_holder = numbers.clone();
        let __seq_guard = __seq_holder.lock().unwrap();
        let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
        let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
        drop(__seq_guard);
        let __low = 0;
        let __high = (5) as usize;
        let __max = __source_cap;
        if __seq.len() < __high { __seq.resize_with(__high, Default::default); }
        let _slice = &__seq[__low..__high];
        let mut _v = Vec::with_capacity((__max - __low) as usize);
        _v.extend_from_slice(_slice);
        _v
    }.iter().copied().enumerate() {
        { let __rhs = { let __tmp_x = { let __tmp_x = i as i32; let __tmp_y = val; __tmp_x * __tmp_y }; let __tmp_y = ({ let __tmp_x = val; let __tmp_y = 3; __tmp_x % __tmp_y }); __tmp_x + __tmp_y }; let mut guard = total.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    print!("Complex range calculation: {}\n", { let __v = (*total.lock().unwrap().as_ref().unwrap()).clone(); __v });
}

#[derive(Debug, Clone)]
pub struct AnonymousStruct1 {
    pub x: Arc<Mutex<Option<i32>>>,
    pub y: Arc<Mutex<Option<i32>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.y.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
            y: __go_clone_0_1,
        }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_0_1 = Arc::new(Mutex::new(Some(0)));
        Self {
            x: __go_default_0_0,
            y: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.y.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}
