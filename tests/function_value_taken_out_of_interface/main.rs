use std::any::Any;
use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc};


fn __go_type_name(val: &dyn Any) -> &'static str {
    if val.is::<i32>() { return "int" }
    if val.is::<i64>() { return "int64" }
    if val.is::<i8>() { return "int8" }
    if val.is::<i16>() { return "int16" }
    if val.is::<u32>() { return "uint" }
    if val.is::<u64>() { return "uint64" }
    if val.is::<u8>() { return "uint8" }
    if val.is::<u16>() { return "uint16" }
    if val.is::<f64>() { return "float64" }
    if val.is::<f32>() { return "float32" }
    if val.is::<bool>() { return "bool" }
    if val.is::<String>() { return "string" }
    if val.is::<Vec<i32>>() { return "[]int" }
    if val.is::<Vec<i64>>() { return "[]int64" }
    if val.is::<Vec<f64>>() { return "[]float64" }
    if val.is::<Vec<String>>() { return "[]string" }
    if val.is::<Vec<bool>>() { return "[]bool" }
    if val.is::<Rc<RefCell<Option<Box<dyn FnMut() -> ()>>>>>() { return "func()" }
    if val.is::<Rc<RefCell<Option<Box<dyn FnMut() -> Rc<RefCell<Option<String>>>>>>>>() { return "func() string" }
    if val.is::<Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> i32>>>>>() { return "func(int) int" }
    std::any::type_name_of_val(val)
}

pub fn run(f: Rc<RefCell<Option<Box<dyn FnMut() -> ()>>>>) {
    { let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = f.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
}

fn main() {
    let mut count = Rc::new(RefCell::new(Some(0)));
    let mut count_closure_clone = count.clone(); let mut x: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(Box::new(move || {
        { let mut guard = count_closure_clone.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }) as Box<dyn FnMut() -> ()>))).clone()) as Box<dyn Any>)));
    let mut g = ({ let val = x.clone(); let guard = val.borrow(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Rc<RefCell<Option<Box<dyn FnMut() -> ()>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } });
    { let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = g.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    let (_, mut ok) = ({
        let val = x.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> ()>>>>>() {
                (typed_val.clone(), true)
            } else {
                (Rc::new(RefCell::new(None::<Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> ()>>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> ()>>)), false)
        }
    });
    println!("{} {}", format!("{}", { let __v = (*count.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", ok));
    let (mut h, mut ok) = ({
        let val = x.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Box<dyn FnMut() -> ()>>>>>() {
                (typed_val.clone(), true)
            } else {
                (Rc::new(RefCell::new(None::<Box<dyn FnMut() -> ()>>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<Box<dyn FnMut() -> ()>>)), false)
        }
    });
    { let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = h.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    println!("{} {}", format!("{}", { let __v = (*count.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", ok));
    run(({ let val = x.clone(); let guard = val.borrow(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Rc<RefCell<Option<Box<dyn FnMut() -> ()>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } }));
    ({
        let __f_holder = { let val = x.clone(); let guard = val.borrow(); match guard.as_ref() { Some(any_val) => any_val.downcast_ref::<Rc<RefCell<Option<Box<dyn FnMut() -> ()>>>>>().expect("type assertion failed").clone(), None => panic!("type assertion on nil interface") } };
        (|| {
            let mut __f_guard = __f_holder.borrow_mut();
            let __f: &mut Box<dyn FnMut() -> ()> = __f_guard.as_mut().expect("call of nil function");
            let __f_result = (*__f)();
            __f_result
        })()
    });
    let mut k: Rc<RefCell<Option<Box<dyn FnMut() -> ()>>>> = Rc::new(RefCell::new(None));
    { let new_val = ({ let val = x.clone(); let guard = val.borrow(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Rc<RefCell<Option<Box<dyn FnMut() -> ()>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } }); k = new_val; };
    { let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = k.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    println!("{}", format!("{}", { let __v = (*count.borrow().as_ref().unwrap()).clone(); __v }));

    let mut handlers = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Box<dyn Any>>>>>::from([("inc".to_string(), Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(Box::new(move |n: Rc<RefCell<Option<i32>>>| -> i32 {
        (*n.borrow().as_ref().unwrap()) + 1
    }) as Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> i32>))).clone()) as Box<dyn Any>))))]))));
    let mut inc = ({ let val = { let __map_holder = handlers.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"inc".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let guard = val.borrow(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> i32>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } });
    println!("{}", format!("{}", { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> i32> = { let mut __f_guard = inc.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> i32> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(41)))) }));

    let mut items = Rc::new(RefCell::new(Some(vec![Box::new(Rc::new(RefCell::new(Some(Box::new(move || -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some("hi".to_string())))
    }) as Box<dyn FnMut() -> Rc<RefCell<Option<String>>>>))).clone()) as Box<dyn Any>])));
    let mut say = ({
        let __idx = (0) as usize;
        let __seq_holder = items.clone();
        let __seq_guard = __seq_holder.borrow();
        let any_val = __seq_guard.as_ref().expect("nil []any in type assertion")[__idx].as_ref();
        any_val.downcast_ref::<Rc<RefCell<Option<Box<dyn FnMut() -> Rc<RefCell<Option<String>>>>>>>>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(any_val), "func() string")).clone()
    });
    println!("{}", format!("{}", (*{ let __f_ptr: *mut Box<dyn FnMut() -> Rc<RefCell<Option<String>>>> = { let mut __f_guard = say.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> Rc<RefCell<Option<String>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }.borrow().as_ref().unwrap())));
    let (mut say2, mut ok) = ({
        let __idx = (0) as usize;
        let __seq_holder = items.clone();
        let __seq_guard = __seq_holder.borrow();
        match __seq_guard.as_ref().expect("nil []any in type assertion")[__idx].downcast_ref::<Rc<RefCell<Option<Box<dyn FnMut() -> Rc<RefCell<Option<String>>>>>>>>() {
            Some(typed_val) => (typed_val.clone(), true),
            None => (Rc::new(RefCell::new(None::<Box<dyn FnMut() -> Rc<RefCell<Option<String>>>>>)), false),
        }
    });
    println!("{} {}", format!("{}", (*{ let __f_ptr: *mut Box<dyn FnMut() -> Rc<RefCell<Option<String>>>> = { let mut __f_guard = say2.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> Rc<RefCell<Option<String>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }.borrow().as_ref().unwrap())), format!("{}", ok));
    { let (__tmp_0, __tmp_1) = ({
        let __idx = (0) as usize;
        let __seq_holder = items.clone();
        let __seq_guard = __seq_holder.borrow();
        match __seq_guard.as_ref().expect("nil []any in type assertion")[__idx].downcast_ref::<Rc<RefCell<Option<Box<dyn FnMut() -> ()>>>>>() {
            Some(typed_val) => (typed_val.clone(), true),
            None => (Rc::new(RefCell::new(None::<Box<dyn FnMut() -> ()>>)), false),
        }
    }); ok = __tmp_1; };
    println!("{}", format!("{}", ok));

    {
    let _ts_subject = x.clone();
    let _ts_guard = _ts_subject.borrow();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| {
        let mut __any = __v.as_ref() as &dyn Any;
        while let Some(__boxed) = __any.downcast_ref::<Box<dyn Any>>() {
            __any = __boxed.as_ref() as &dyn Any;
        }
        __any
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Box<dyn FnMut() -> ()>>>>>()).is_some() {
        let f = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Box<dyn FnMut() -> ()>>>>>()).unwrap().clone();
        drop(_ts_guard);
        { let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = f.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };;
        println!("{} {}", format!("{}", "func".to_string()), format!("{}", { let __v = (*count.borrow().as_ref().unwrap()).clone(); __v }));;
    }
    }
}