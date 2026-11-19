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


/// The variable of a range loop is a copy of the element, also when the loop
/// assigns to a variable declared before it, and for the values of a map.
fn main() {
    let mut s = Rc::new(RefCell::new(Some(vec![Point { x: Rc::new(RefCell::new(Some(1))), ..Default::default() }, Point { x: Rc::new(RefCell::new(Some(2))), ..Default::default() }])));
    let mut p: Rc<RefCell<Option<Point>>> = Rc::new(RefCell::new(Some(Default::default())));
    let mut i: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(0)));
    { let __range_holder = s.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (__range_i, __range_p) in __range_values.iter().enumerate() {
        { let new_val = __range_i as i32; *i.borrow_mut() = Some(new_val); };
        { let new_val = (*__range_p).__go_value_clone(); *p.borrow_mut() = Some(new_val); };
        if (*i.borrow().as_ref().unwrap()) == 0 {
        { let new_val = 9; *(*s.borrow().as_ref().unwrap())[(1) as usize].clone().x.borrow_mut() = Some(new_val); };
    }
        println!("{} {}", format!("{}", "slice into a variable".to_string()), format!("{}", (*(*p.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())));
        { let new_val = 99; *(*p.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    } }
    println!("{} {} {} {}", format!("{}", "slice into a variable, afterwards".to_string()), format!("{}", (*(*s.borrow().as_ref().unwrap())[(0) as usize].clone().x.borrow().as_ref().unwrap())), format!("{}", (*(*s.borrow().as_ref().unwrap())[(1) as usize].clone().x.borrow().as_ref().unwrap())), format!("{}", (*(*p.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())));

    let mut arr = Rc::new(RefCell::new(Some([Point { x: Rc::new(RefCell::new(Some(1))), ..Default::default() }, Point { x: Rc::new(RefCell::new(Some(2))), ..Default::default() }])));
    let mut q: Rc<RefCell<Option<Point>>> = Rc::new(RefCell::new(Some(Default::default())));
    { let __range_holder = arr.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.iter().map(|__e| __e.__go_value_clone()).collect::<Vec<_>>()).unwrap_or_default(); drop(__range_guard); for (__range_i, __range_q) in __range_values.iter().enumerate() {
        { let new_val = __range_i as i32; *i.borrow_mut() = Some(new_val); };
        { let new_val = (*__range_q).__go_value_clone(); *q.borrow_mut() = Some(new_val); };
        if (*i.borrow().as_ref().unwrap()) == 0 {
        { let new_val = 9; *(*arr.borrow().as_ref().unwrap())[(1) as usize].clone().x.borrow_mut() = Some(new_val); };
    }
        println!("{} {}", format!("{}", "array into a variable".to_string()), format!("{}", (*(*q.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())));
        { let new_val = 99; *(*q.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    } }
    println!("{} {} {} {}", format!("{}", "array into a variable, afterwards".to_string()), format!("{}", (*(*arr.borrow().as_ref().unwrap())[(0) as usize].clone().x.borrow().as_ref().unwrap())), format!("{}", (*(*arr.borrow().as_ref().unwrap())[(1) as usize].clone().x.borrow().as_ref().unwrap())), format!("{}", (*(*q.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())));

    let mut r: Rc<RefCell<Option<Point>>> = Rc::new(RefCell::new(Some(Default::default())));
    { let __range_holder = s.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for __range_r in __range_values.iter() {
        { let new_val = (*__range_r).__go_value_clone(); *r.borrow_mut() = Some(new_val); };
        { let __target = (*r.borrow().as_ref().unwrap()).x.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    } }
    println!("{} {} {} {}", format!("{}", "slice into a variable, no index".to_string()), format!("{}", (*(*s.borrow().as_ref().unwrap())[(0) as usize].clone().x.borrow().as_ref().unwrap())), format!("{}", (*(*s.borrow().as_ref().unwrap())[(1) as usize].clone().x.borrow().as_ref().unwrap())), format!("{}", (*(*r.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())));

    { let new_val = Rc::new(RefCell::new(Some(vec![Point { x: Rc::new(RefCell::new(Some(1))), ..Default::default() }, Point { x: Rc::new(RefCell::new(Some(2))), ..Default::default() }]))); s = new_val; };
    { let __range_holder = s.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for e in __range_values.iter() {
        let e = &e.__go_value_clone();
        let mut a = Rc::new(RefCell::new(Some((*e).__go_value_clone())));
        { let new_val = 100; *(*a.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
        let mut b: Rc<RefCell<Option<Point>>> = Rc::new(RefCell::new(Some(Default::default())));
        { let new_val = (*e).__go_value_clone(); *b.borrow_mut() = Some(new_val); };
        { let new_val = 200; *(*b.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
        println!("{} {} {} {}", format!("{}", "from a range variable".to_string()), format!("{}", (*e.x.borrow().as_ref().unwrap())), format!("{}", (*(*a.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())), format!("{}", (*(*b.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())));
    } }

    let mut m = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Point>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(1))), ..Default::default() }))))]))));
    for (_, v) in { let __range_holder = m.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        let v = Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().__go_value_clone())));
        { let new_val = 50; *(*v.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    }
    println!("{} {}", format!("{}", "range over a map".to_string()), format!("{}", (*{ let __map_holder = m.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.borrow().as_ref().unwrap().__go_value_clone()).unwrap_or_else(|| Default::default()) }.x.borrow().as_ref().unwrap())));

    let mut w: Rc<RefCell<Option<Point>>> = Rc::new(RefCell::new(Some(Default::default())));
    for (_, __range_w) in { let __range_holder = m.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        { let new_val = __range_w.borrow().as_ref().unwrap().__go_value_clone(); *w.borrow_mut() = Some(new_val); };
        { let new_val = 60; *(*w.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    }
    println!("{} {} {}", format!("{}", "range over a map into a variable".to_string()), format!("{}", (*{ let __map_holder = m.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.borrow().as_ref().unwrap().__go_value_clone()).unwrap_or_else(|| Default::default()) }.x.borrow().as_ref().unwrap())), format!("{}", (*(*w.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())));

    let mut kept: Rc<RefCell<Option<Vec<Point>>>> = Rc::new(RefCell::new(None));
    { let __range_holder = s.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for e in __range_values.iter() {
        let e = &e.__go_value_clone();
        { let new_val = { let __append_target = kept.clone(); let __append_value = (*e).__go_value_clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; kept = new_val; };
    } }
    { let new_val = 7; *(*kept.borrow().as_ref().unwrap())[(0) as usize].clone().x.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "range variable appended".to_string()), format!("{}", (*(*s.borrow().as_ref().unwrap())[(0) as usize].clone().x.borrow().as_ref().unwrap())), format!("{}", (*(*kept.borrow().as_ref().unwrap())[(0) as usize].clone().x.borrow().as_ref().unwrap())));

    let mut byX = Rc::new(RefCell::new(Some(BTreeMap::<i32, Rc<RefCell<Option<Point>>>>::from([]))));
    { let __range_holder = s.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for e in __range_values.iter() {
        let e = &e.__go_value_clone();
        { let __map_key = (*e.x.borrow().as_ref().unwrap()); let __map_value = Rc::new(RefCell::new(Some((*e).clone()))); (*byX.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    } }
    let mut got = Rc::new(RefCell::new(Some({ let __map_holder = byX.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&1)).map(|__v| __v.borrow().as_ref().unwrap().__go_value_clone()).unwrap_or_else(|| Default::default()) })));
    { let new_val = 70; *(*got.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    println!("{} {} {} {}", format!("{}", "range variable stored in a map".to_string()), format!("{}", (*(*s.borrow().as_ref().unwrap())[(0) as usize].clone().x.borrow().as_ref().unwrap())), format!("{}", (*{ let __map_holder = byX.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&1)).map(|__v| __v.borrow().as_ref().unwrap().__go_value_clone()).unwrap_or_else(|| Default::default()) }.x.borrow().as_ref().unwrap())), format!("{}", (*(*got.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())));

    let mut last: Rc<RefCell<Option<Point>>> = Rc::new(RefCell::new(Some(Default::default())));
    { let __range_holder = s.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for e in __range_values.iter() {
        let e = &e.__go_value_clone();
        { let new_val = (*e).__go_value_clone(); *last.borrow_mut() = Some(new_val); };
    } }
    { let new_val = 90; *(*last.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "range variable kept after the loop".to_string()), format!("{}", (*(*s.borrow().as_ref().unwrap())[(1) as usize].clone().x.borrow().as_ref().unwrap())), format!("{}", (*(*last.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())));
}