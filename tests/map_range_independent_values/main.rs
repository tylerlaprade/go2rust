use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Point {
    pub x: Rc<RefCell<Option<i32>>>,
}

impl Point {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
        }
    }
}


impl Default for Point {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            x: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Point {
    pub fn inc(&mut self) {
        { let __target = self.x.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

fn main() {
    let mut numbers = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(1))))]))));
    for (_, mut v) in { let __range_holder = numbers.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        let v = Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().clone())));
        { let new_val = 5; *v.borrow_mut() = Some(new_val); };
        println!("{} {} {}", format!("{}", "int".to_string()), format!("{}", (*v.borrow_mut().as_mut().unwrap())), format!("{}", { let __map_holder = numbers.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));
    }
    let mut sum = Rc::new(RefCell::new(Some(0)));
    let mut max = Rc::new(RefCell::new(Some(0)));
    for (_, v) in { let __range_holder = numbers.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        let v = Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().clone())));
        { let __rhs = v.borrow().as_ref().unwrap().clone(); let mut guard = sum.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        if (*v.borrow_mut().as_mut().unwrap()) > (*max.borrow().as_ref().unwrap()) {
        { let new_val = v.borrow().as_ref().unwrap().clone(); *max.borrow_mut() = Some(new_val); };
    }
    }
    println!("{} {} {}", format!("{}", "aggregate".to_string()), format!("{}", { let __v = (*sum.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*max.borrow().as_ref().unwrap()).clone(); __v }));

    let mut weights = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<f64>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(1.5)))), ("b".to_string(), Rc::new(RefCell::new(Some(2.25))))]))));
    let mut totalWeight = Rc::new(RefCell::new(Some(0.0)));
    for (_, weight) in { let __range_holder = weights.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        let weight = Rc::new(RefCell::new(Some(weight.borrow().as_ref().unwrap().clone())));
        { let __rhs = weight.borrow().as_ref().unwrap().clone(); let mut guard = totalWeight.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    println!("{} {}", format!("{}", "weight".to_string()), format!("{}", { let __v = (*totalWeight.borrow().as_ref().unwrap()).clone(); __v }));

    let mut words = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<String>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some("one".to_string()))))]))));
    for (_, mut word) in { let __range_holder = words.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        let word = Rc::new(RefCell::new(Some(word.borrow().as_ref().unwrap().clone())));
        { (*word.borrow_mut().as_mut().unwrap()).push_str(&"!".to_string()); };
        println!("{} {} {}", format!("{}", "string".to_string()), format!("{}", (*word.borrow_mut().as_mut().unwrap())), format!("{}", { let __map_holder = words.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()) }));
    }

    let mut points = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Point>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(1))), ..Default::default() }))))]))));
    for (_, p) in { let __range_holder = points.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        let p = Rc::new(RefCell::new(Some(p.borrow().as_ref().unwrap().__go_value_clone())));
        (*p.borrow_mut().as_mut().unwrap()).inc();
        println!("{} {} {}", format!("{}", "struct".to_string()), format!("{}", (*(*p.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())), format!("{}", (*{ let __map_holder = points.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.borrow().as_ref().unwrap().__go_value_clone()).unwrap_or_else(|| Default::default()) }.x.borrow().as_ref().unwrap())));
    }

    let mut lists = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Vec<i32>>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(vec![1]))))]))));
    for (_, list) in { let __range_holder = lists.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        { let __idx = (0) as usize; let __rhs = 2; (*list.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    }
    println!("{} {}", format!("{}", "slice".to_string()), format!("{}", (*{ let __map_holder = lists.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.borrow().as_ref().unwrap())[(0) as usize].clone()));

    let mut n = Rc::new(RefCell::new(Some(3)));
    let mut ptrs = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::from([("a".to_string(), n.clone().clone())]))));
    for (_, p) in { let __range_holder = ptrs.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        { let new_val = 4; *p.borrow_mut() = Some(new_val); };
    }
    println!("{} {}", format!("{}", "pointer".to_string()), format!("{}", { let __v = (*n.borrow().as_ref().unwrap()).clone(); __v }));

    let mut nilPtrs = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::from([("a".to_string(), Rc::new(RefCell::new(None)))]))));
    for (_, p) in { let __range_holder = nilPtrs.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        println!("{} {}", format!("{}", "nil-pointer".to_string()), format!("{}", { let __nil_result = (*p.borrow()).is_none(); __nil_result }));
    }
}