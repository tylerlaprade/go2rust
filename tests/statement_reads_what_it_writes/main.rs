use std::cell::{RefCell};
use std::cmp::Ord;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

fn format_map<K: Display + Ord + Clone, V>(map: &Rc<RefCell<Option<BTreeMap<K, Rc<RefCell<Option<V>>>>>>>) -> String
where
    V: Display,
{
    let guard = map.borrow();
    if let Some(ref m) = *guard {
        let mut items: Vec<_> = m.iter().collect();
        items.sort_by_key(|(k, _)| (*k).clone());

        let formatted: Vec<String> = items
            .into_iter()
            .map(|(k, v)| {
                let v_guard = v.borrow();
                if let Some(ref val) = *v_guard {
                    format!("{}:{}", k, val)
                } else {
                    format!("{}:<nil>", k)
                }
            })
            .collect();

        format!("map[{}]", formatted.join(" "))
    } else {
        "map[]".to_string()
    }
}
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

#[derive(Debug, Clone, Default)]
pub struct Box_ {
    pub items: Rc<RefCell<Option<Vec<i32>>>>,
}

impl Box_ {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.items.clone();
        Self {
            items: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for Box_ {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_slice(&self.items));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone)]
pub struct Pt {
    pub x: Rc<RefCell<Option<i32>>>,
}

impl Pt {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
        }
    }
}


impl Default for Pt {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            x: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Pt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Box_ {
    pub fn dup(&mut self) {
        { let new_val = { let __append_target = self.items.clone(); let __append_value = (*self.items.borrow().as_ref().unwrap())[((({ let __len_target = { let __field = self.items.clone(); __field }; let __len_guard = __len_target.borrow(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32) - (1 as i32)) as usize].clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; self.items = new_val; };
    }
}

/// A statement may read the slice or map it writes to: in the index, in the
/// value, or in what it appends. The read must be finished before the write
/// takes its lock.
fn main() {
    let mut s = Rc::new(RefCell::new(Some(vec![1, 2, 3])));
    { let __idx = (((*s.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32) - (1 as i32)) as usize; let __rhs = 5; (*s.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {}", format!("{}", "index reads the slice".to_string()), format!("{}", format_slice(&s)));
    { let __idx = (0) as usize; let __rhs = (*s.borrow().as_ref().unwrap())[(1) as usize].clone(); (*s.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {}", format!("{}", "value reads the slice".to_string()), format!("{}", format_slice(&s)));
    { let __idx = ((*s.borrow().as_ref().unwrap())[(0) as usize].clone() - 1) as usize; let __rhs = 7; (*s.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {}", format!("{}", "index is an element".to_string()), format!("{}", format_slice(&s)));
    { let __idx = (0) as usize; let __rhs = (*s.borrow().as_ref().unwrap())[(1) as usize].clone(); let mut __seq_guard = s.borrow_mut(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx] + __rhs; };
    { let __idx = (1) as usize; let mut __seq_guard = s.borrow_mut(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx] + 1; }
    {
        let __go_lhs_sequence_0 = s.clone();
        let __go_lhs_index_0 = (0) as usize;
        let __go_lhs_sequence_1 = s.clone();
        let __go_lhs_index_1 = (2) as usize;
        let __tmp_0 = (*s.borrow().as_ref().unwrap())[(2) as usize].clone();
        let __tmp_1 = (*s.borrow().as_ref().unwrap())[(0) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    println!("{} {}", format!("{}", "compound, increment, swap".to_string()), format!("{}", format_slice(&s)));

    let mut s2 = Rc::new(RefCell::new(Some(vec![1, 2, 3])));
    { let __idx = (((*s2.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32) - (1 as i32)) as usize; let mut __seq_guard = s2.borrow_mut(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx] + 1; }
    { let __idx = (((*s2.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32) - (1 as i32)) as usize; let __rhs = (*s2.borrow().as_ref().unwrap())[(0) as usize].clone(); let mut __seq_guard = s2.borrow_mut(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx] + __rhs; };
    {
        let __go_lhs_sequence_0 = s2.clone();
        let __go_lhs_index_0 = (((*s2.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32) - (1 as i32)) as usize;
        let __go_lhs_sequence_1 = s2.clone();
        let __go_lhs_index_1 = (0) as usize;
        let __tmp_0 = (*s2.borrow().as_ref().unwrap())[(0) as usize].clone();
        let __tmp_1 = (*s2.borrow().as_ref().unwrap())[(((*s2.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32) - (1 as i32)) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    println!("{} {}", format!("{}", "index reads the slice in an increment, a compound assignment and a swap".to_string()), format!("{}", format_slice(&s2)));

    let mut grid = Rc::new(RefCell::new(Some(vec![vec![1, 2], vec![3, 4]])));
    { let __outer_idx = (0) as usize; let __idx = (1) as usize; let __rhs = (*grid.borrow().as_ref().unwrap())[(1) as usize].clone()[(0) as usize].clone(); (*grid.borrow_mut().as_mut().unwrap())[__outer_idx][__idx] = __rhs; };
    { let __outer_idx = (((*grid.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32) - (1 as i32)) as usize; let __idx = (0) as usize; let __rhs = 5; (*grid.borrow_mut().as_mut().unwrap())[__outer_idx][__idx] = __rhs; };
    println!("{} {} {}", format!("{}", "slice of slices".to_string()), format!("{}", (*grid.borrow().as_ref().unwrap())[(0) as usize].clone()[(1) as usize].clone()), format!("{}", (*grid.borrow().as_ref().unwrap())[(1) as usize].clone()[(0) as usize].clone()));

    let mut pts = Rc::new(RefCell::new(Some(vec![Pt { x: Rc::new(RefCell::new(Some(1))), ..Default::default() }, Pt { x: Rc::new(RefCell::new(Some(2))), ..Default::default() }])));
    { let __idx = (((*pts.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32) - (1 as i32)) as usize; let __rhs = (*pts.borrow().as_ref().unwrap())[(0) as usize].clone().__go_value_clone(); (*pts.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {}", format!("{}", "struct element".to_string()), format!("{}", (*(*pts.borrow().as_ref().unwrap())[(1) as usize].clone().x.borrow().as_ref().unwrap())));

    let mut arr = Rc::new(RefCell::new(Some([1, 2, 3])));
    let mut p = arr.clone();
    { let __idx = ((3 as i32) - (1 as i32)) as usize; let __rhs = (*p.borrow().as_ref().unwrap())[(0) as usize].clone(); (*p.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {}", format!("{}", "through a pointer to an array".to_string()), format!("{}", format_slice(&arr)));

    let mut w = Rc::new(RefCell::new(Some(vec!["a".to_string(), "b".to_string()])));
    { let __idx = (((*w.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32) - (1 as i32)) as usize; let __rhs = format!("{}{}", (*w.borrow().as_ref().unwrap())[(0) as usize].clone(), "!".to_string()); (*w.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {}", format!("{}", "strings".to_string()), format!("{}", format_slice(&w)));

    let mut a = Rc::new(RefCell::new(Some([1, 2, 3])));
    { let __idx = (((*a.borrow().as_ref().unwrap()).len() as i32) - (1 as i32)) as usize; let __rhs = (*a.borrow().as_ref().unwrap())[(0) as usize].clone(); (*a.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {}", format!("{}", "array".to_string()), format!("{}", format_slice(&a)));

    let mut b = Rc::new(RefCell::new(Some(Box_ { items: Rc::new(RefCell::new(Some(vec![1, 2]))), ..Default::default() })));
    { let __idx = ((({ let __len_target = { let __field = (*b.borrow().as_ref().unwrap()).items.clone(); __field }; let __len_guard = __len_target.borrow(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32) - (1 as i32)) as usize; let __rhs = 9; (*(*b.borrow().as_ref().unwrap()).items.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    { let __idx = (0) as usize; let __rhs = (*(*b.borrow().as_ref().unwrap()).items.borrow().as_ref().unwrap())[(1) as usize].clone(); (*(*b.borrow().as_ref().unwrap()).items.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {}", format!("{}", "slice in a field".to_string()), format!("{}", format_slice_values(&(*(*b.borrow().as_ref().unwrap()).items.borrow().as_ref().unwrap()).clone())));

    let mut m = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(1)))), ("b".to_string(), Rc::new(RefCell::new(Some(2))))]))));
    { let __map_holder = m.clone(); let __key = "a".to_string(); let __rhs = { let __map_holder = m.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"b".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }; let mut __map_guard = __map_holder.borrow_mut(); let __map = __map_guard.as_mut().unwrap(); let __entry = __map.entry(__key).or_insert_with(|| Rc::new(RefCell::new(Some(0)))); let mut __value = __entry.borrow_mut(); * __value = Some((*__value.as_ref().unwrap()).clone() + __rhs); };
    { let __map_key = "c".to_string(); let __map_value = Rc::new(RefCell::new(Some(({ let __map_holder = m.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) } as i32) + ((*m.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)))); (*m.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{} {}", format!("{}", "map".to_string()), format!("{}", format_map(&m)));

    let mut n = Rc::new(RefCell::new(Some(vec![1, 2])));
    { let new_val = { let __append_target = n.clone(); let __append_value = (*n.borrow().as_ref().unwrap())[(0) as usize].clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; n = new_val; };
    { let new_val = { let __append_target = n.clone(); let __append_value = (*n.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32; (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; n = new_val; };
    { let new_val = { let __append_target = n.clone(); let __append_values = vec![(*n.borrow().as_ref().unwrap())[(1) as usize].clone(), (*n.borrow().as_ref().unwrap())[(2) as usize].clone()]; (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).extend(__append_values); __append_target.clone() }; n = new_val; };
    println!("{} {}", format!("{}", "append reads the slice".to_string()), format!("{}", format_slice(&n)));
    { let new_val = { let __append_target = n.clone(); let __append_values: Vec<_> = { let __slice_holder = n.clone(); let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned().collect(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).extend(__append_values); __append_target.clone() }; n = new_val; };
    println!("{} {} {}", format!("{}", "append the slice to itself".to_string()), format!("{}", format_slice(&n)), format!("{}", (*n.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)));
    { let new_val = { let __append_target = n.clone(); let __append_values: Vec<_> = { let __slice_holder = Rc::new(RefCell::new(Some({ let __seq_holder = n.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (10) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).clone(); let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned().collect(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).extend(__append_values); __append_target.clone() }; n = new_val; };
    println!("{} {} {} {}", format!("{}", "append a part of the slice".to_string()), format!("{}", (*n.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", (*n.borrow().as_ref().unwrap())[(12) as usize].clone()), format!("{}", (*n.borrow().as_ref().unwrap())[(13) as usize].clone()));

    { let new_val = { let __append_target = (*b.borrow().as_ref().unwrap()).items.clone(); let __append_value = (*(*b.borrow().as_ref().unwrap()).items.borrow().as_ref().unwrap())[(0) as usize].clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; (*b.borrow_mut().as_mut().unwrap()).items = new_val; };
    (*b.borrow_mut().as_mut().unwrap()).dup();
    println!("{} {}", format!("{}", "append to a field".to_string()), format!("{}", format_slice_values(&(*(*b.borrow().as_ref().unwrap()).items.borrow().as_ref().unwrap()).clone())));

    let n_closure_clone = n.clone(); let mut total = Rc::new(RefCell::new(Some(Box::new(move || -> i32 {
        return (*n_closure_clone.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32;
    }) as Box<dyn FnMut() -> i32>)));
    { let new_val = { let __append_target = n.clone(); let __append_value = { let __f_ptr: *mut Box<dyn FnMut() -> i32> = { let mut __f_guard = total.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i32> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }; (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; n = new_val; };
    println!("{} {}", format!("{}", "append what a function read from the slice".to_string()), format!("{}", (*n.borrow().as_ref().unwrap())[(((*n.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32) - (1 as i32)) as usize].clone()));

    let mut byKey = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Vec<i32>>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(vec![1]))))]))));
    { let __map_key = "a".to_string(); let __map_value = { let __slice = { let __map_holder = byKey.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().unwrap().get(&"a".to_string()).cloned().unwrap_or_else(|| Rc::new(RefCell::new(None))) }; let __append_value = (*{ let __map_holder = byKey.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.borrow().as_ref().unwrap())[(0) as usize].clone(); (*__slice.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __slice.clone() }; (*byKey.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    { let __map_key = "a".to_string(); let __map_value = { let __slice = { let __map_holder = byKey.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().unwrap().get(&"a".to_string()).cloned().unwrap_or_else(|| Rc::new(RefCell::new(None))) }; let __append_value = (*{ let __map_holder = byKey.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32; (*__slice.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __slice.clone() }; (*byKey.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{} {}", format!("{}", "append to a slice in a map".to_string()), format!("{}", format_slice(&{ let __map_holder = byKey.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) })));
}