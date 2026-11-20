use std::cell::{RefCell};
use std::collections::BTreeMap;
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

/// Go calls the functions of a statement from left to right: the map or the
/// index of the target first, then the value or what is appended.
fn main() {
    let mut order = Rc::new(RefCell::new(Some("".to_string())));
    let mut m = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i64>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(1i64))))]))));
    let mut order_closure_clone = order.clone(); let mut key = Rc::new(RefCell::new(Some(Box::new(move || -> Rc<RefCell<Option<String>>> {
        { (*order_closure_clone.borrow_mut().as_mut().unwrap()).push_str(&"key ".to_string()); };
        Rc::new(RefCell::new(Some("a".to_string())))
    }) as Box<dyn FnMut() -> Rc<RefCell<Option<String>>>>)));
    let mut order_closure_clone = order.clone(); let mut operand = Rc::new(RefCell::new(Some(Box::new(move || -> i64 {
        { (*order_closure_clone.borrow_mut().as_mut().unwrap()).push_str(&"operand ".to_string()); };
        5
    }) as Box<dyn FnMut() -> i64>)));
    let m_closure_clone = m.clone(); let mut order_closure_clone = order.clone(); let mut pick = Rc::new(RefCell::new(Some(Box::new(move || -> Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>> {
        { (*order_closure_clone.borrow_mut().as_mut().unwrap()).push_str(&"map ".to_string()); };
        return m_closure_clone.clone();
    }) as Box<dyn FnMut() -> Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>)));
    { let __map_holder = { let __f_ptr: *mut Box<dyn FnMut() -> Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>> = { let mut __f_guard = pick.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }.clone(); let __key = { let __map_key_holder = { let __f_ptr: *mut Box<dyn FnMut() -> Rc<RefCell<Option<String>>>> = { let mut __f_guard = key.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> Rc<RefCell<Option<String>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }.clone(); let __map_key_guard = __map_key_holder.borrow(); let __cloned = (*__map_key_guard.as_ref().unwrap()).clone(); drop(__map_key_guard); __cloned }; let __rhs = { let __f_ptr: *mut Box<dyn FnMut() -> i64> = { let mut __f_guard = operand.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i64> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }; let mut __map_guard = __map_holder.borrow_mut(); let __map = __map_guard.as_mut().unwrap(); let __entry = __map.entry(__key).or_insert_with(|| Rc::new(RefCell::new(Some(0)))); let mut __value = __entry.borrow_mut(); * __value = Some((*__value.as_ref().unwrap()).clone() + __rhs); };
    println!("{} {} {}", format!("{}", "compound map assignment:".to_string()), format!("{}", { let __v = (*order.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __map_holder = m.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));

    let mut lists = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Vec<i64>>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(vec![1i64]))))]))));
    let mut order_closure_clone = order.clone(); let mut value = Rc::new(RefCell::new(Some(Box::new(move || -> i64 {
        { (*order_closure_clone.borrow_mut().as_mut().unwrap()).push_str(&"value ".to_string()); };
        2
    }) as Box<dyn FnMut() -> i64>)));
    { let new_val = "".to_string(); *order.borrow_mut() = Some(new_val); };
    { let __map_key = { let __map_key_holder = { let __f_ptr: *mut Box<dyn FnMut() -> Rc<RefCell<Option<String>>>> = { let mut __f_guard = key.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> Rc<RefCell<Option<String>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }.clone(); let __map_key_guard = __map_key_holder.borrow(); let __cloned = (*__map_key_guard.as_ref().unwrap()).clone(); drop(__map_key_guard); __cloned }; let __map_value = { let __slice = { let __map_holder = lists.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().unwrap().get(&{ let __map_key_holder = { let __f_ptr: *mut Box<dyn FnMut() -> Rc<RefCell<Option<String>>>> = { let mut __f_guard = key.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> Rc<RefCell<Option<String>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }.clone(); let __map_key_guard = __map_key_holder.borrow(); let __cloned = (*__map_key_guard.as_ref().unwrap()).clone(); drop(__map_key_guard); __cloned }).cloned().unwrap_or_else(|| Rc::new(RefCell::new(None))) }; let __append_value = { let __f_ptr: *mut Box<dyn FnMut() -> i64> = { let mut __f_guard = value.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i64> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }; (*__slice.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __slice.clone() }; (*lists.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{} {} {}", format!("{}", "append to a slice in a map:".to_string()), format!("{}", { let __v = (*order.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", format_slice(&{ let __map_holder = lists.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) })));

    let mut grid = Rc::new(RefCell::new(Some(vec![vec![1i64]])));
    let mut order_closure_clone = order.clone(); let mut idx = Rc::new(RefCell::new(Some(Box::new(move || -> i64 {
        { (*order_closure_clone.borrow_mut().as_mut().unwrap()).push_str(&"index ".to_string()); };
        0
    }) as Box<dyn FnMut() -> i64>)));
    { let new_val = "".to_string(); *order.borrow_mut() = Some(new_val); };
    { let _ = ({ let __f_ptr: *mut Box<dyn FnMut() -> i64> = { let mut __f_guard = idx.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i64> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }) as usize; { let __append_idx = ({ let __f_ptr: *mut Box<dyn FnMut() -> i64> = { let mut __f_guard = idx.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i64> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }) as usize; let __append_value = { let __f_ptr: *mut Box<dyn FnMut() -> i64> = { let mut __f_guard = value.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i64> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }; (*grid.borrow_mut().as_mut().unwrap())[__append_idx].push(__append_value); (*grid.borrow().as_ref().unwrap())[__append_idx].clone() }; };
    println!("{} {} {}", format!("{}", "append to an element:".to_string()), format!("{}", { let __v = (*order.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", (*grid.borrow().as_ref().unwrap())[(0) as usize].clone().len()));

    let mut s = Rc::new(RefCell::new(Some(vec![1i64, 2i64])));
    { let new_val = "".to_string(); *order.borrow_mut() = Some(new_val); };
    { let __idx = ({ let __f_ptr: *mut Box<dyn FnMut() -> i64> = { let mut __f_guard = idx.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i64> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }) as usize; let __rhs = { let __f_ptr: *mut Box<dyn FnMut() -> i64> = { let mut __f_guard = value.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i64> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }; (*s.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {} {}", format!("{}", "element assignment:".to_string()), format!("{}", { let __v = (*order.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", format_slice(&s)));

    let mut order_closure_clone = order.clone(); let s_closure_clone = s.clone(); let mut get = Rc::new(RefCell::new(Some(Box::new(move || -> Rc<RefCell<Option<Vec<i64>>>> {
        { (*order_closure_clone.borrow_mut().as_mut().unwrap()).push_str(&"slice ".to_string()); };
        return s_closure_clone.clone();
    }) as Box<dyn FnMut() -> Rc<RefCell<Option<Vec<i64>>>>>)));
    { let new_val = "".to_string(); *order.borrow_mut() = Some(new_val); };
    { let __seq = { let __f_ptr: *mut Box<dyn FnMut() -> Rc<RefCell<Option<Vec<i64>>>>> = { let mut __f_guard = get.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> Rc<RefCell<Option<Vec<i64>>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }.clone(); let __idx = ({ let __f_ptr: *mut Box<dyn FnMut() -> i64> = { let mut __f_guard = idx.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i64> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }) as usize; let __rhs = { let __f_ptr: *mut Box<dyn FnMut() -> i64> = { let mut __f_guard = value.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i64> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }; (*__seq.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {} {}", format!("{}", "element of what a function returned:".to_string()), format!("{}", { let __v = (*order.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", format_slice(&s)));

    let mut table = Rc::new(RefCell::new(Some(vec![vec![1i64, 2i64], vec![3i64, 4i64]])));
    let mut order_closure_clone = order.clone(); let table_closure_clone = table.clone(); let mut getTable = Rc::new(RefCell::new(Some(Box::new(move || -> Rc<RefCell<Option<Vec<Vec<i64>>>>> {
        { (*order_closure_clone.borrow_mut().as_mut().unwrap()).push_str(&"table ".to_string()); };
        return table_closure_clone.clone();
    }) as Box<dyn FnMut() -> Rc<RefCell<Option<Vec<Vec<i64>>>>>>)));
    { let new_val = "".to_string(); *order.borrow_mut() = Some(new_val); };
    { let __seq = { let __f_ptr: *mut Box<dyn FnMut() -> Rc<RefCell<Option<Vec<Vec<i64>>>>>> = { let mut __f_guard = getTable.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> Rc<RefCell<Option<Vec<Vec<i64>>>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }.clone(); let __outer_idx = ({ let __f_ptr: *mut Box<dyn FnMut() -> i64> = { let mut __f_guard = idx.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i64> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }) as usize; let __idx = ({ let __f_ptr: *mut Box<dyn FnMut() -> i64> = { let mut __f_guard = idx.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i64> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }) as usize; let __rhs = { let __f_ptr: *mut Box<dyn FnMut() -> i64> = { let mut __f_guard = value.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i64> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }; (*__seq.borrow_mut().as_mut().unwrap())[__outer_idx][__idx] = __rhs; };
    println!("{} {} {}", format!("{}", "element of a slice of slices that a function returned:".to_string()), format!("{}", { let __v = (*order.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", (*table.borrow().as_ref().unwrap())[(0) as usize].clone()[(0) as usize].clone()));

    { let new_val = "".to_string(); *order.borrow_mut() = Some(new_val); };
    { let new_val = { let __append_target = s.clone(); let __append_values = vec![{ let __f_ptr: *mut Box<dyn FnMut() -> i64> = { let mut __f_guard = value.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i64> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }, { let __f_ptr: *mut Box<dyn FnMut() -> i64> = { let mut __f_guard = idx.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i64> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }]; (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).extend(__append_values); __append_target.clone() }; s = new_val; };
    println!("{} {} {}", format!("{}", "append:".to_string()), format!("{}", { let __v = (*order.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", format_slice(&s)));
}