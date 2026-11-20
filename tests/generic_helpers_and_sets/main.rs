use std::cell::{RefCell};
use std::collections::BTreeMap;
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

#[derive(Debug, Clone)]
pub struct Pair__string__int {
    pub key: Rc<RefCell<Option<String>>>,
    pub val: Rc<RefCell<Option<i64>>>,
}

impl Pair__string__int {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.key.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.val.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            key: __go_clone_0_0,
            val: __go_clone_1_0,
        }
    }
}


impl Default for Pair__string__int {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            key: __go_default_0_0,
            val: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Pair__string__int {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct Set__string(pub Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<AnonymousStruct1>>>>>>>);


#[derive(Debug, Clone, Default)]
pub struct AnonymousStruct1 {
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
    }
}


impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


impl Pair__string__int {
    pub fn string(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("{}={}", (*self.key.borrow().as_ref().unwrap()), (*self.val.borrow().as_ref().unwrap())))))
    }
}

impl Set__string {
    pub fn add(&self, v: Rc<RefCell<Option<String>>>) {
        let mut __self = self.clone();
        { let __map_key = (*v.borrow().as_ref().unwrap()).clone(); let __map_value = Rc::new(RefCell::new(Some(AnonymousStruct1 {  }))); (*__self.0.clone().borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    }

    pub fn has(&self, v: Rc<RefCell<Option<String>>>) -> bool {
        let (_, mut ok) = { let __map_holder = self.0.clone().clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&(*v.borrow().as_ref().unwrap()).clone())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Rc::new(RefCell::new(Some(Default::default()))), false) } };
        ok
    }

    pub fn len(&self) -> i64 {
        return { let __map_holder = self.0.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64;
    }
}

pub fn keys__string__int(m: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>) -> Rc<RefCell<Option<Vec<String>>>> {
    let mut keys = Rc::new(RefCell::new(Some(Vec::<String>::with_capacity(((*m.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize))));
    for (k, _) in { let __range_holder = m.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        { let new_val = { let __append_target = keys.clone(); let __append_value = k.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; keys = new_val; };
    }
    let keys_closure_clone = keys.clone(); { let __sort_target = keys_closure_clone.clone(); let __sort_less = Rc::new(RefCell::new(Some(Box::new(move |i: Rc<RefCell<Option<i64>>>, j: Rc<RefCell<Option<i64>>>| -> bool {
        return (*keys_closure_clone.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone() < (*keys_closure_clone.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
    }) as Box<dyn FnMut(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> bool>))); let __sort_len = { let __sort_guard = __sort_target.borrow(); __sort_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; for __sort_i in 1..__sort_len { let mut __sort_j = __sort_i; while __sort_j > 0 { let __should_swap = { let mut __less_guard = __sort_less.borrow_mut(); let __less = __less_guard.as_mut().expect("sort.Slice less function is nil"); __less(Rc::new(RefCell::new(Some(__sort_j as i64))), Rc::new(RefCell::new(Some((__sort_j - 1) as i64)))) }; if !__should_swap { break; } { let mut __sort_guard = __sort_target.borrow_mut(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.swap(__sort_j, __sort_j - 1); } } __sort_j -= 1; } } };
    return keys.clone();
}

pub fn keys__int__bool(m: Rc<RefCell<Option<BTreeMap<i64, Rc<RefCell<Option<bool>>>>>>>) -> Rc<RefCell<Option<Vec<i64>>>> {
    let mut keys = Rc::new(RefCell::new(Some(Vec::<i64>::with_capacity(((*m.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize))));
    for (k, _) in { let __range_holder = m.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        { let new_val = { let __append_target = keys.clone(); let __append_value = k; (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; keys = new_val; };
    }
    let keys_closure_clone = keys.clone(); { let __sort_target = keys_closure_clone.clone(); let __sort_less = Rc::new(RefCell::new(Some(Box::new(move |i: Rc<RefCell<Option<i64>>>, j: Rc<RefCell<Option<i64>>>| -> bool {
        return (*keys_closure_clone.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone() < (*keys_closure_clone.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
    }) as Box<dyn FnMut(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> bool>))); let __sort_len = { let __sort_guard = __sort_target.borrow(); __sort_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; for __sort_i in 1..__sort_len { let mut __sort_j = __sort_i; while __sort_j > 0 { let __should_swap = { let mut __less_guard = __sort_less.borrow_mut(); let __less = __less_guard.as_mut().expect("sort.Slice less function is nil"); __less(Rc::new(RefCell::new(Some(__sort_j as i64))), Rc::new(RefCell::new(Some((__sort_j - 1) as i64)))) }; if !__should_swap { break; } { let mut __sort_guard = __sort_target.borrow_mut(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.swap(__sort_j, __sort_j - 1); } } __sort_j -= 1; } } };
    return keys.clone();
}

pub fn max__int(a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) -> i64 {
    if (*a.borrow().as_ref().unwrap()) > (*b.borrow().as_ref().unwrap()) {
        return (*a.borrow().as_ref().unwrap());
    }
    (*b.borrow().as_ref().unwrap())
}

pub fn max__float64(a: Rc<RefCell<Option<f64>>>, b: Rc<RefCell<Option<f64>>>) -> f64 {
    if (*a.borrow().as_ref().unwrap()) > (*b.borrow().as_ref().unwrap()) {
        return (*a.borrow().as_ref().unwrap());
    }
    (*b.borrow().as_ref().unwrap())
}

pub fn max__string(a: Rc<RefCell<Option<String>>>, b: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {
    if (*a.borrow().as_ref().unwrap()) > (*b.borrow().as_ref().unwrap()) {
        return { let __owned = a.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    }
    { let __owned = b.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) }
}

pub fn filter__int(xs: Rc<RefCell<Option<Vec<i64>>>>, keep: Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> bool>>>>) -> Rc<RefCell<Option<Vec<i64>>>> {
    let mut out: Rc<RefCell<Option<Vec<i64>>>> = Rc::new(RefCell::new(None));

    { let __range_holder = xs.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter().copied() {
        if { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> bool> = { let mut __f_guard = keep.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> bool> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(x)))) } {
        { let new_val = { let __append_target = out.clone(); let __append_value = x; (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; out = new_val; };
    }
    } }
    return out.clone();
}

pub fn reduce__int__int(xs: Rc<RefCell<Option<Vec<i64>>>>, init: Rc<RefCell<Option<i64>>>, f: Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>>>>) -> i64 {
    let mut acc = { let __owned = init.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    { let __range_holder = xs.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter().copied() {
        { let new_val = { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> = { let mut __f_guard = f.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(acc.clone(), Rc::new(RefCell::new(Some(x)))) }; *acc.borrow_mut() = Some(new_val); };
    } }
    return (*acc.borrow().as_ref().unwrap());
}

pub fn reduce__string__string(xs: Rc<RefCell<Option<Vec<String>>>>, init: Rc<RefCell<Option<String>>>, f: Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>>>>>) -> Rc<RefCell<Option<String>>> {
    let mut acc = { let __owned = init.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    { let __range_holder = xs.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter() {
        { let new_val = { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>> = { let mut __f_guard = f.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some({ let __arg_holder = acc.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some((*x).clone())))) }; let __moved_val = { let mut __guard = new_val.borrow_mut(); __guard.take() }; *acc.borrow_mut() = __moved_val; };
    } }
    return { let __owned = acc.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
}

pub fn map__int__string(xs: Rc<RefCell<Option<Vec<i64>>>>, f: Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> Rc<RefCell<Option<String>>>>>>>) -> Rc<RefCell<Option<Vec<String>>>> {
    let mut out = Rc::new(RefCell::new(Some(vec!["".to_string(); ((*xs.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize])));
    { let __range_holder = xs.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, x) in __range_values.iter().copied().enumerate() {
        { let __idx = (i) as usize; let __rhs = (*{ let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> Rc<RefCell<Option<String>>>> = { let mut __f_guard = f.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> Rc<RefCell<Option<String>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(x)))) }.borrow().as_ref().unwrap()).clone(); (*out.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    } }
    return out.clone();
}

fn main() {
    let mut ages = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i64>>>>::from([("bob".to_string(), Rc::new(RefCell::new(Some(30i64)))), ("alice".to_string(), Rc::new(RefCell::new(Some(25i64)))), ("carol".to_string(), Rc::new(RefCell::new(Some(35i64))))]))));
    let mut pairs: Rc<RefCell<Option<Vec<Pair__string__int>>>> = Rc::new(RefCell::new(None));
    { let __range_holder = keys__string__int(ages.clone()).clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for k in __range_values.iter() {
        { let new_val = { let __append_target = pairs.clone(); let __append_value = Pair__string__int { key: Rc::new(RefCell::new(Some((*k).clone()))), val: Rc::new(RefCell::new(Some({ let __map_holder = ages.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(k)).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }))), ..Default::default() }; (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; pairs = new_val; };
    } }
    println!("{}", format!("{}", format_slice(&pairs)));
    println!("{} {}", format!("{}", (*pairs.borrow().as_ref().unwrap())[(0) as usize].clone()), format!("{}", (*pairs.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)));
    let mut s = Rc::new(RefCell::new(Some(Set__string(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<AnonymousStruct1>>>>::from([]))))))));
    { let __range_holder = Rc::new(RefCell::new(Some({ let __s = "a b a c b".to_string(); __s.split_whitespace().map(|__part| __part.to_string()).collect::<Vec<String>>() }))).clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for w in __range_values.iter() {
        (*s.borrow().as_ref().unwrap()).add(Rc::new(RefCell::new(Some((*w).clone()))));
    } }
    println!("{} {} {}", format!("{}", (*s.borrow().as_ref().unwrap()).len()), format!("{}", (*s.borrow().as_ref().unwrap()).has(Rc::new(RefCell::new(Some("a".to_string()))))), format!("{}", (*s.borrow().as_ref().unwrap()).has(Rc::new(RefCell::new(Some("z".to_string()))))));
    println!("{} {} {}", format!("{}", max__int(Rc::new(RefCell::new(Some(3))), Rc::new(RefCell::new(Some(7))))), format!("{}", max__float64(Rc::new(RefCell::new(Some(2.5))), Rc::new(RefCell::new(Some(-1.0))))), format!("{}", (*max__string(Rc::new(RefCell::new(Some("go".to_string()))), Rc::new(RefCell::new(Some("rust".to_string())))).borrow().as_ref().unwrap())));
    let mut evens = filter__int(Rc::new(RefCell::new(Some(vec![1i64, 2i64, 3i64, 4i64, 5i64, 6i64]))), Rc::new(RefCell::new(Some(Box::new(move |n: Rc<RefCell<Option<i64>>>| -> bool {
        (*n.borrow().as_ref().unwrap()) % 2 == 0
    }) as Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> bool>))));
    println!("{} {}", format!("{}", format_slice(&evens)), format!("{}", reduce__int__int(evens.clone(), Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(Box::new(move |a: Rc<RefCell<Option<i64>>>, n: Rc<RefCell<Option<i64>>>| -> i64 {
        (*a.borrow().as_ref().unwrap()) + (*n.borrow().as_ref().unwrap())
    }) as Box<dyn FnMut(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>))))));
    let mut words = map__int__string(Rc::new(RefCell::new(Some(vec![1i64, 22i64, 333i64]))), Rc::new(RefCell::new(Some(Box::new(move |n: Rc<RefCell<Option<i64>>>| -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some({ let __s = "x".to_string(); let __count = (*Rc::new(RefCell::new(Some(format!("{}", format!("{}", { let __v = (*n.borrow().as_ref().unwrap()).clone(); __v }))))).borrow().as_ref().unwrap()).len(); __s.repeat(__count as usize) })))
    }) as Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> Rc<RefCell<Option<String>>>>))));
    println!("{} {}", format!("{}", format_slice(&words)), format!("{}", (*reduce__string__string(words.clone(), Rc::new(RefCell::new(Some("".to_string()))), Rc::new(RefCell::new(Some(Box::new(move |a: Rc<RefCell<Option<String>>>, w: Rc<RefCell<Option<String>>>| -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some({
            let mut __s = String::new();
            __s.push_str(&format!("{}", (*a.borrow().as_ref().unwrap())));
            __s.push_str(&format!("{}", (*w.borrow().as_ref().unwrap())));
            __s.push_str(&format!("{}", "|".to_string()));
            __s
        })))
    }) as Box<dyn FnMut(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>>)))).borrow().as_ref().unwrap())));
    let mut ids = keys__int__bool(Rc::new(RefCell::new(Some(BTreeMap::<i64, Rc<RefCell<Option<bool>>>>::from([(3i64, Rc::new(RefCell::new(Some(true)))), (1i64, Rc::new(RefCell::new(Some(false)))), (2i64, Rc::new(RefCell::new(Some(true))))])))));
    println!("{}", format!("{}", format_slice(&ids)));
}