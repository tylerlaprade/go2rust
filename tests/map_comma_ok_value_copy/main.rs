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


#[derive(Debug, Clone, Default)]
pub struct Registry {
    pub by_name: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<Point>>>>>>>,
    pub counts: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>>,
}

impl Registry {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.by_name.clone();
        let __go_clone_1_0 = self.counts.clone();
        Self {
            by_name: __go_clone_0_0,
            counts: __go_clone_1_0,
        }
    }
}

impl std::fmt::Display for Registry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_map(&self.by_name));
        let __go_fmt_1 = format!("{}", format_map(&self.counts));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone, Default)]
pub struct Scores(pub Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>>);

impl Display for Scores {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_map(&self.0))
    }
}


/// "v, ok := m[k]" gives v its own copy of the value. Writing to v afterwards
/// must not change the map, and reading into existing variables must not take
/// the value out of the map.
fn main() {
    let mut counts = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(1))))]))));
    let (mut n, mut ok) = { let __map_holder = counts.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().clone()))), true), None => (Rc::new(RefCell::new(Some(0))), false) } };
    { let new_val = 5; *n.borrow_mut() = Some(new_val); };
    println!("{} {} {} {}", format!("{}", "int".to_string()), format!("{}", { let __v = (*n.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", ok), format!("{}", { let __map_holder = counts.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));

    let mut names = Rc::new(RefCell::new(Some(BTreeMap::<i32, Rc<RefCell<Option<String>>>>::from([(1, Rc::new(RefCell::new(Some("one".to_string()))))]))));
    let (mut name, mut ok) = { let __map_holder = names.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&1)) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().clone()))), true), None => (Rc::new(RefCell::new(Some(String::new()))), false) } };
    { (*name.borrow_mut().as_mut().unwrap()).push_str(&"!".to_string()); };
    println!("{} {} {} {}", format!("{}", "string".to_string()), format!("{}", { let __v = (*name.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", ok), format!("{}", { let __map_holder = names.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&1)).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()) }));

    {
        let (mut v, mut ok) = { let __map_holder = counts.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().clone()))), true), None => (Rc::new(RefCell::new(Some(0))), false) } };;
        if ok {
            { let mut guard = v.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); };
            println!("{} {} {}", format!("{}", "in an if".to_string()), format!("{}", { let __v = (*v.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __map_holder = counts.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));;
        }
    }

    let mut m: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(0)));
    let mut found: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));
    { let (__tmp_0, __tmp_1) = { let __map_holder = counts.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().clone()))), true), None => (Rc::new(RefCell::new(Some(0))), false) } }; let __moved_tmp_0 = { let mut __guard = __tmp_0.borrow_mut(); __guard.take() }; *m.borrow_mut() = __moved_tmp_0; *found.borrow_mut() = Some(__tmp_1); };
    { let __rhs = 10; let mut guard = m.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    println!("{} {} {} {} {}", format!("{}", "int into variables".to_string()), format!("{}", { let __v = (*m.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*found.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __map_holder = counts.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", (*counts.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)));

    { let (__tmp_0, __tmp_1) = { let __map_holder = names.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&1)) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().clone()))), true), None => (Rc::new(RefCell::new(Some(String::new()))), false) } }; let __moved_tmp_0 = { let mut __guard = __tmp_0.borrow_mut(); __guard.take() }; *name.borrow_mut() = __moved_tmp_0; ok = __tmp_1; };
    println!("{} {} {} {}", format!("{}", "string into variables".to_string()), format!("{}", { let __v = (*name.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", ok), format!("{}", { let __map_holder = names.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&1)).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()) }));
    { let (__tmp_0, __tmp_1) = { let __map_holder = names.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&2)) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().clone()))), true), None => (Rc::new(RefCell::new(Some(String::new()))), false) } }; let __moved_tmp_0 = { let mut __guard = __tmp_0.borrow_mut(); __guard.take() }; *name.borrow_mut() = __moved_tmp_0; ok = __tmp_1; };
    println!("{} {} {} {}", format!("{}", "missing key into variables".to_string()), format!("{}", (*name.borrow().as_ref().unwrap()).clone() == "".to_string()), format!("{}", ok), format!("{}", { let __map_holder = names.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&1)).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()) }));

    let mut points = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Point>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(1))), ..Default::default() }))))]))));
    let (mut p, mut ok) = { let __map_holder = points.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().__go_value_clone()))), true), None => (Rc::new(RefCell::new(Some(Default::default()))), false) } };
    { let new_val = 9; *(*p.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    println!("{} {} {} {}", format!("{}", "struct".to_string()), format!("{}", ok), format!("{}", (*{ let __map_holder = points.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.borrow().as_ref().unwrap().__go_value_clone()).unwrap_or_else(|| Default::default()) }.x.borrow().as_ref().unwrap())), format!("{}", (*(*p.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())));

    let mut q: Rc<RefCell<Option<Point>>> = Rc::new(RefCell::new(Some(Default::default())));
    { let (__tmp_0, __tmp_1) = { let __map_holder = points.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().__go_value_clone()))), true), None => (Rc::new(RefCell::new(Some(Default::default()))), false) } }; let __moved_tmp_0 = { let mut __guard = __tmp_0.borrow_mut(); __guard.take() }; *q.borrow_mut() = __moved_tmp_0; *found.borrow_mut() = Some(__tmp_1); };
    { let new_val = 8; *(*q.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    println!("{} {} {} {}", format!("{}", "struct into variables".to_string()), format!("{}", { let __v = (*found.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", (*{ let __map_holder = points.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.borrow().as_ref().unwrap().__go_value_clone()).unwrap_or_else(|| Default::default()) }.x.borrow().as_ref().unwrap())), format!("{}", (*(*q.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())));

    {
        let (mut r, mut ok) = { let __map_holder = points.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().__go_value_clone()))), true), None => (Rc::new(RefCell::new(Some(Default::default()))), false) } };;
        if ok {
            { let new_val = 7; *(*r.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };;
            println!("{} {} {}", format!("{}", "struct in an if".to_string()), format!("{}", (*{ let __map_holder = points.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.borrow().as_ref().unwrap().__go_value_clone()).unwrap_or_else(|| Default::default()) }.x.borrow().as_ref().unwrap())), format!("{}", (*(*r.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())));;
        }
    }

    let (mut missing, mut ok) = { let __map_holder = points.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"zzz".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().__go_value_clone()))), true), None => (Rc::new(RefCell::new(Some(Default::default()))), false) } };
    { let new_val = 5; *(*missing.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    println!("{} {} {} {}", format!("{}", "struct, missing key".to_string()), format!("{}", ok), format!("{}", (*(*missing.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())), format!("{}", (*points.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)));

    let mut reg = Rc::new(RefCell::new(Some(Registry { by_name: Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Point>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(1))), ..Default::default() }))))])))), counts: Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(1))))])))), ..Default::default() })));
    let (mut rp, mut ok) = match (*(*reg.borrow().as_ref().unwrap()).by_name.borrow().as_ref().unwrap()).clone().get(&"a".to_string()) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().__go_value_clone()))), true), None => (Rc::new(RefCell::new(Some(Default::default()))), false) };
    { let new_val = 7; *(*rp.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    let (mut c, mut ok2) = match (*(*reg.borrow().as_ref().unwrap()).counts.borrow().as_ref().unwrap()).clone().get(&"a".to_string()) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().clone()))), true), None => (Rc::new(RefCell::new(Some(0))), false) };
    { let mut guard = c.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    println!("{} {} {} {} {} {} {}", format!("{}", "map in a field".to_string()), format!("{}", ok), format!("{}", ok2), format!("{}", (*(*(*reg.borrow().as_ref().unwrap()).by_name.borrow().as_ref().unwrap()).clone().get(&"a".to_string()).map(|__v| __v.borrow().as_ref().unwrap().__go_value_clone()).unwrap_or_else(|| Default::default()).x.borrow().as_ref().unwrap())), format!("{}", (*(*rp.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())), format!("{}", (*(*reg.borrow().as_ref().unwrap()).counts.borrow().as_ref().unwrap()).clone().get(&"a".to_string()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0)), format!("{}", { let __v = (*c.borrow().as_ref().unwrap()).clone(); __v }));

    let mut c2: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(0)));
    { let (__tmp_0, __tmp_1) = match (*(*reg.borrow().as_ref().unwrap()).counts.borrow().as_ref().unwrap()).clone().get(&"a".to_string()) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().clone()))), true), None => (Rc::new(RefCell::new(Some(0))), false) }; let __moved_tmp_0 = { let mut __guard = __tmp_0.borrow_mut(); __guard.take() }; *c2.borrow_mut() = __moved_tmp_0; ok = __tmp_1; };
    { let __rhs = 5; let mut guard = c2.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    println!("{} {} {} {}", format!("{}", "map in a field into variables".to_string()), format!("{}", ok), format!("{}", (*(*reg.borrow().as_ref().unwrap()).counts.borrow().as_ref().unwrap()).clone().get(&"a".to_string()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0)), format!("{}", { let __v = (*c2.borrow().as_ref().unwrap()).clone(); __v }));

    let mut sc = Rc::new(RefCell::new(Some(Scores(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(1))))]))))))));
    let (mut v, mut ok) = { let __map_holder = { let __named_map = (*sc.borrow().as_ref().unwrap()).0.clone(); __named_map }.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().clone()))), true), None => (Rc::new(RefCell::new(Some(0))), false) } };
    { let new_val = 3; *v.borrow_mut() = Some(new_val); };
    println!("{} {} {} {}", format!("{}", "named map type".to_string()), format!("{}", ok), format!("{}", { let __map_holder = { let __named_map = (*sc.borrow().as_ref().unwrap()).0.clone(); __named_map }.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", { let __v = (*v.borrow().as_ref().unwrap()).clone(); __v }));

    let (_, mut has) = { let __map_holder = { let __named_map = (*sc.borrow().as_ref().unwrap()).0.clone(); __named_map }.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"zzz".to_string())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Rc::new(RefCell::new(Some(0))), false) } };
    let (mut z, _) = { let __map_holder = { let __named_map = (*sc.borrow().as_ref().unwrap()).0.clone(); __named_map }.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().clone()))), true), None => (Rc::new(RefCell::new(Some(0))), false) } };
    { let new_val = 4; *z.borrow_mut() = Some(new_val); };
    println!("{} {} {} {}", format!("{}", "blank results".to_string()), format!("{}", has), format!("{}", { let __map_holder = { let __named_map = (*sc.borrow().as_ref().unwrap()).0.clone(); __named_map }.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", { let __v = (*z.borrow().as_ref().unwrap()).clone(); __v }));

    let mut nested = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::from([("x".to_string(), Rc::new(RefCell::new(Some(1))))])))))]))));
    let (mut x, mut ok) = { let __map_holder = { let __map_holder = nested.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"x".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().clone()))), true), None => (Rc::new(RefCell::new(Some(0))), false) } };
    { let new_val = 99; *x.borrow_mut() = Some(new_val); };
    println!("{} {} {} {}", format!("{}", "nested lookup".to_string()), format!("{}", ok), format!("{}", { let __map_holder = { let __map_holder = nested.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"x".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v }));

        // A pointer, a slice and a map are shared with the map entry, as in Go.
    let mut ptrs = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Point>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(1))), ..Default::default() }))).clone())]))));
    let (mut pp, mut ok) = { let __map_holder = ptrs.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } };
    { let new_val = 9; *(*pp.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    println!("{} {} {} {}", format!("{}", "pointer value".to_string()), format!("{}", ok), format!("{}", (*(*{ let __map_holder = ptrs.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())), format!("{}", (*(*pp.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())));

    let mut pq: Rc<RefCell<Option<Point>>> = Rc::new(RefCell::new(None));
    { let (__tmp_0, __tmp_1) = { let __map_holder = ptrs.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } }; pq = __tmp_0.clone(); ok = __tmp_1; };
    { let new_val = 10; *(*pq.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    println!("{} {} {} {}", format!("{}", "pointer value into variables".to_string()), format!("{}", ok), format!("{}", (*(*{ let __map_holder = ptrs.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())), format!("{}", (*(*pq.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())));

    let mut lists = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Vec<i32>>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(vec![1, 2]))))]))));
    let (mut l, mut ok) = { let __map_holder = lists.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } };
    { let __idx = (0) as usize; let __rhs = 50; (*l.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {} {} {}", format!("{}", "slice value".to_string()), format!("{}", ok), format!("{}", format_slice(&{ let __map_holder = lists.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) })), format!("{}", format_slice(&l)));

    let mut l2: Rc<RefCell<Option<Vec<i32>>>> = Rc::new(RefCell::new(None));
    { let (__tmp_0, __tmp_1) = { let __map_holder = lists.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } }; l2 = __tmp_0; ok = __tmp_1; };
    { let __idx = (1) as usize; let __rhs = 60; (*l2.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {} {} {}", format!("{}", "slice value into variables".to_string()), format!("{}", ok), format!("{}", format_slice(&{ let __map_holder = lists.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) })), format!("{}", format_slice(&l2)));

    let (mut inner, mut ok) = { let __map_holder = nested.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } };
    { let __map_key = "x".to_string(); let __map_value = Rc::new(RefCell::new(Some(2))); (*inner.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{} {} {} {}", format!("{}", "map value".to_string()), format!("{}", ok), format!("{}", { let __map_holder = { let __map_holder = nested.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"x".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", { let __map_holder = inner.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"x".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));

    let mut inner2: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>> = Rc::new(RefCell::new(Some(BTreeMap::new())));
    { let (__tmp_0, __tmp_1) = { let __map_holder = nested.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } }; inner2 = __tmp_0; ok = __tmp_1; };
    { let __map_key = "x".to_string(); let __map_value = Rc::new(RefCell::new(Some(3))); (*inner2.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{} {} {} {}", format!("{}", "map value into variables".to_string()), format!("{}", ok), format!("{}", { let __map_holder = { let __map_holder = nested.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"x".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", { let __map_holder = inner2.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"x".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));
}