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

#[derive(Clone)]
pub struct GoLocalPtrKey<T>(pub Rc<RefCell<Option<T>>>);

impl<T> GoLocalPtrKey<T> {
    pub fn new(value: Rc<RefCell<Option<T>>>) -> Self { GoLocalPtrKey(value) }
    pub fn value(&self) -> Rc<RefCell<Option<T>>> { self.0.clone() }
    fn addr(&self) -> usize { Rc::as_ptr(&self.0) as usize }
}

impl<T> PartialEq for GoLocalPtrKey<T> {
    fn eq(&self, other: &Self) -> bool { self.addr() == other.addr() }
}
impl<T> Eq for GoLocalPtrKey<T> {}
impl<T> PartialOrd for GoLocalPtrKey<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl<T> Ord for GoLocalPtrKey<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.addr().cmp(&other.addr()) }
}
impl<T> std::fmt::Debug for GoLocalPtrKey<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0x{:x}", self.addr()) }
}
impl<T> std::fmt::Display for GoLocalPtrKey<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0x{:x}", self.addr()) }
}

#[derive(Debug, Clone)]
pub struct named {
    pub id: Rc<RefCell<Option<String>>>,
}

impl named {
    pub fn __go_value_clone(&self) -> Self {
        Self { id: { let __guard = self.id.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for named {
    fn default() -> Self {
        Self { id: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for named {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.id.borrow().as_ref().unwrap()))
    }
}


#[derive(Clone)]
pub struct pkgReader {
    pub later_fns: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Box<dyn FnMut() -> ()>>>>>>>>,
    pub later_fors: Rc<RefCell<Option<BTreeMap<GoLocalPtrKey<named>, Rc<RefCell<Option<i32>>>>>>>,
    pub hits: Rc<RefCell<Option<i32>>>,
}

impl pkgReader {
    pub fn __go_value_clone(&self) -> Self {
        Self { later_fns: self.later_fns.clone(), later_fors: self.later_fors.clone(), hits: { let __guard = self.hits.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for pkgReader {
    fn default() -> Self {
        Self { later_fns: Rc::new(RefCell::new(None)), later_fors: Rc::new(RefCell::new(None)), hits: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for pkgReader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", { let __guard = self.later_fns.borrow(); match __guard.as_ref() { Some(__v) => format!("[{}]", std::iter::repeat("<func>").take(__v.len()).collect::<Vec<_>>().join(" ")), None => "[]".to_string() } }, format_map(&self.later_fors), (*self.hits.borrow().as_ref().unwrap()))
    }
}


impl pkgReader {
    pub fn later_for(&mut self, t: Rc<RefCell<Option<named>>>, r#fn: Rc<RefCell<Option<Box<dyn FnMut() -> ()>>>>) {
        if { let __nil_target = self.later_fors.clone(); let __nil_result = (*__nil_target.borrow()).is_none(); __nil_result } {
        { let new_val = Rc::new(RefCell::new(Some(BTreeMap::<GoLocalPtrKey<named>, Rc<RefCell<Option<i32>>>>::new()))); self.later_fors = new_val; };
    }
        { let __map_key = GoLocalPtrKey::new(t.clone()); let __map_value = Rc::new(RefCell::new(Some((*self.later_fns.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32))); (*self.later_fors.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
        { let new_val = { let __append_target = self.later_fns.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(r#fn.clone()); __append_target.clone() }; self.later_fns = new_val; };
    }
}

pub fn schedule(pr: Rc<RefCell<Option<pkgReader>>>, named: Rc<RefCell<Option<named>>>, rhs: Rc<RefCell<Option<named>>>) {
    let mut pk = pr.clone();
    let named_closure_clone = named.clone(); let pk_closure_clone = pk.clone(); let rhs_closure_clone = rhs.clone(); { let __recv = pk_closure_clone.clone(); let __result = (*__recv.borrow_mut().as_mut().unwrap()).later_for(named_closure_clone.clone(), Rc::new(RefCell::new(Some(Box::new(move || {
        { let __map_handle = (*pk_closure_clone.borrow().as_ref().unwrap()).later_fors.clone(); let mut __map_guard = __map_handle.borrow_mut(); __map_guard.as_mut().unwrap().remove(&GoLocalPtrKey::new(named_closure_clone.clone())); };
        {
        let (mut i, mut ok) = match (*(*pk_closure_clone.borrow().as_ref().unwrap()).later_fors.borrow().as_ref().unwrap()).clone().get(&GoLocalPtrKey::new(rhs_closure_clone.clone())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), Rc::new(RefCell::new(Some(true)))), None => (Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(false)))) };;
        if (*ok.borrow().as_ref().unwrap()) {
            let mut f = (*(*pk_closure_clone.borrow().as_ref().unwrap()).later_fns.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone().clone();;
            (*(*pk_closure_clone.borrow().as_ref().unwrap()).later_fns.borrow_mut().as_mut().unwrap())[((*i.borrow().as_ref().unwrap())) as usize] = Rc::new(RefCell::new(Some(Box::new(move || {
    }) as Box<dyn FnMut() -> ()>)));;
            { let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = f.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };;
        }
    }
        { let __target = (*pk_closure_clone.borrow().as_ref().unwrap()).hits.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }) as Box<dyn FnMut() -> ()>)))); __result };
}

fn main() {
    let mut pr = Rc::new(RefCell::new(Some(pkgReader { later_fns: Rc::new(RefCell::new(Some(vec![]))), later_fors: Rc::new(RefCell::new(Some(BTreeMap::new()))), hits: Rc::new(RefCell::new(Some(0))) })));
    let mut a = Rc::new(RefCell::new(Some(named { id: Rc::new(RefCell::new(Some("a".to_string()))), ..Default::default() })));
    let mut b = Rc::new(RefCell::new(Some(named { id: Rc::new(RefCell::new(Some("b".to_string()))), ..Default::default() })));
    let pr_closure_clone = pr.clone(); { let __recv = pr_closure_clone.clone(); let __result = (*__recv.borrow_mut().as_mut().unwrap()).later_for(b.clone(), Rc::new(RefCell::new(Some(Box::new(move || {
        { let __target = (*pr_closure_clone.borrow().as_ref().unwrap()).hits.clone(); let __rhs = 10; let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }) as Box<dyn FnMut() -> ()>)))); __result };
    schedule(pr.clone(), a.clone(), b.clone());
    { let __f_holder = (*(*pr.borrow().as_ref().unwrap()).later_fns.borrow().as_ref().unwrap())[(1) as usize].clone(); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    println!("{}", format!("{}", (*(*pr.borrow().as_ref().unwrap()).hits.borrow().as_ref().unwrap())));
}