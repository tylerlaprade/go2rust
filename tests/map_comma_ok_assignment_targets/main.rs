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
pub struct Numbers(pub Rc<RefCell<Option<Vec<i32>>>>);

impl Display for Numbers {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice(&self.0))
    }
}


#[derive(Debug, Clone, Default)]
pub struct Index(pub Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>>);

impl Display for Index {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_map(&self.0))
    }
}


#[derive(Debug, Clone)]
pub struct Holder {
    pub nums: Rc<RefCell<Option<Numbers>>>,
    pub plain: Rc<RefCell<Option<Vec<i32>>>>,
}

impl Holder {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.nums.clone();
        let __go_clone_1_0 = self.plain.clone();
        Self {
            nums: __go_clone_0_0,
            plain: __go_clone_1_0,
        }
    }
}


impl Default for Holder {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(Default::default())));
        let __go_default_1_0 = Rc::new(RefCell::new(None));
        Self {
            nums: __go_default_0_0,
            plain: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.nums.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice(&self.plain));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub(crate) struct GoGlobal<T> {
    value: std::cell::UnsafeCell<Option<T>>,
}
unsafe impl<T> Sync for GoGlobal<T> {}
impl<T> GoGlobal<T> {
    pub(crate) const fn new() -> Self {
        Self { value: std::cell::UnsafeCell::new(None) }
    }
    pub(crate) fn borrow(&'static self) -> &'static Option<T> {
        unsafe { &*self.value.get() }
    }
    pub(crate) fn borrow_mut(&'static self) -> &'static mut Option<T> {
        unsafe { &mut *self.value.get() }
    }
    pub(crate) fn clone(&'static self) -> std::rc::Rc<std::cell::RefCell<Option<T>>> where T: Clone {
        std::rc::Rc::new(std::cell::RefCell::new(self.borrow().clone()))
    }
}

pub(crate) static global: GoGlobal<Vec<i32>> = GoGlobal::new();


fn __go_init_globals() {
    *global.borrow_mut() = Some(vec![]);
}


/// "v, ok = m[k]" into an existing target must leave the value in the map,
/// whatever the target is: a named slice, a field, a package variable, an
/// element, or a variable captured by a closure.
fn main() {
    __go_init_all();
    let mut m = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Numbers>>>>::new())));
    let mut src = Rc::new(RefCell::new(Some(Numbers(Rc::new(RefCell::new(Some(vec![0; (2) as usize])))))));
    {
        let __go_lhs_sequence_0 = { let __named_slice = (*src.borrow().as_ref().unwrap()).0.clone(); __named_slice }.clone();
        let __go_lhs_index_0 = (0) as usize;
        let __go_lhs_sequence_1 = { let __named_slice = (*src.borrow().as_ref().unwrap()).0.clone(); __named_slice }.clone();
        let __go_lhs_index_1 = (1) as usize;
        let __tmp_0 = 1;
        let __tmp_1 = 2;
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    { let __map_key = "a".to_string(); let __map_value = src.clone(); (*m.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };

    let mut numbers: Rc<RefCell<Option<Numbers>>> = Rc::new(RefCell::new(Some(Default::default())));
    let mut ok: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));
    { let (__tmp_0, __tmp_1) = { let __map_holder = m.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => ({ let __held = v.borrow().clone(); Rc::new(RefCell::new(__held)) }, true), None => (Rc::new(RefCell::new(Some(Default::default()))), false) } }; let __moved_tmp_0 = { let mut __guard = __tmp_0.borrow_mut(); __guard.take() }; *numbers.borrow_mut() = __moved_tmp_0; *ok.borrow_mut() = Some(__tmp_1); };
    { let __idx = (0) as usize; let __rhs = 9; (*{ let __named_slice = (*numbers.borrow().as_ref().unwrap()).0.clone(); __named_slice }.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {} {} {}", format!("{}", "named slice".to_string()), format!("{}", { let __v = (*ok.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __seq_holder = { let __named_slice = (*{ let __map_holder = m.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Rc::new(RefCell::new(Some(Default::default())))) }.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }), format!("{}", { let __seq_holder = { let __named_slice = (*numbers.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }));
    { let new_val = { let __base = { let __named_slice = (*numbers.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __base_guard = __base.borrow(); let mut __values = __base_guard.as_ref().cloned().unwrap_or_else(Vec::new); drop(__base_guard); __values.push(5); Rc::new(RefCell::new(Some(Numbers(Rc::new(RefCell::new(Some(__values))))))) }; let __moved_val = { let mut __guard = new_val.borrow_mut(); __guard.take() }; *numbers.borrow_mut() = __moved_val; };
    println!("{} {} {}", format!("{}", "named slice, appended to".to_string()), format!("{}", { let __slice_holder = { let __named_slice = (*{ let __map_holder = m.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Rc::new(RefCell::new(Some(Default::default())))) }.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }), format!("{}", { let __slice_holder = { let __named_slice = (*numbers.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }));

    let mut h: Rc<RefCell<Option<Holder>>> = Rc::new(RefCell::new(Some(Default::default())));
    { let (__tmp_0, __tmp_1) = { let __map_holder = m.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => ({ let __held = v.borrow().clone(); Rc::new(RefCell::new(__held)) }, true), None => (Rc::new(RefCell::new(Some(Default::default()))), false) } }; let __moved_tmp_0 = { let mut __guard = __tmp_0.borrow_mut(); __guard.take() }; *(*h.borrow().as_ref().unwrap()).nums.borrow_mut() = __moved_tmp_0; *ok.borrow_mut() = Some(__tmp_1); };
    { let __idx = (1) as usize; let __rhs = 8; (*{ let __named_slice = (*(*h.borrow().as_ref().unwrap()).nums.borrow().as_ref().unwrap()).0.clone(); __named_slice }.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {} {} {}", format!("{}", "named slice field".to_string()), format!("{}", { let __v = (*ok.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __seq_holder = { let __named_slice = (*{ let __map_holder = m.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Rc::new(RefCell::new(Some(Default::default())))) }.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(1) as usize].clone() }), format!("{}", { let __seq_holder = { let __named_slice = (*(*h.borrow().as_ref().unwrap()).nums.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(1) as usize].clone() }));

    let mut lists = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Vec<i32>>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(vec![1, 2]))))]))));
    { let (__tmp_0, __tmp_1) = { let __map_holder = lists.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => ({ let __held = v.borrow().clone(); Rc::new(RefCell::new(__held)) }, true), None => (Default::default(), false) } }; let __moved_tmp_0 = { let mut __guard = __tmp_0.borrow_mut(); __guard.take() }; *global.borrow_mut() = __moved_tmp_0; *ok.borrow_mut() = Some(__tmp_1); };
    println!("{} {} {} {} {} {}", format!("{}", "global".to_string()), format!("{}", { let __v = (*ok.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", format_slice(&{ let __map_holder = lists.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) })), format!("{}", (*global.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", (*global.borrow().as_ref().unwrap())[(0) as usize].clone()), format!("{}", (*global.borrow().as_ref().unwrap())[(1) as usize].clone()));

    let mut dst = Rc::new(RefCell::new(Some(vec![vec![9]])));
    { let (__tmp_0, __tmp_1) = { let __map_holder = lists.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => ({ let __held = v.borrow().clone(); Rc::new(RefCell::new(__held)) }, true), None => (Default::default(), false) } }; { let __idx = (0) as usize; (*dst.borrow_mut().as_mut().unwrap())[__idx] = __tmp_0.borrow_mut().take().unwrap_or_default(); } *ok.borrow_mut() = Some(__tmp_1); };
    println!("{} {} {} {}", format!("{}", "indexed".to_string()), format!("{}", { let __v = (*ok.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", format_slice(&{ let __map_holder = lists.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) })), format!("{}", format_slice_values(&(*dst.borrow().as_ref().unwrap())[(0) as usize].clone())));

    { let (__tmp_0, __tmp_1) = { let __map_holder = lists.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } }; (*h.borrow_mut().as_mut().unwrap()).plain = __tmp_0; *ok.borrow_mut() = Some(__tmp_1); };
    { let __idx = (0) as usize; let __rhs = 70; (*(*h.borrow().as_ref().unwrap()).plain.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {} {} {}", format!("{}", "plain slice field".to_string()), format!("{}", { let __v = (*ok.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", format_slice(&{ let __map_holder = lists.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) })), format!("{}", format_slice_values(&(*(*h.borrow().as_ref().unwrap()).plain.borrow().as_ref().unwrap()).clone())));

    let mut captured: Rc<RefCell<Option<Vec<i32>>>> = Rc::new(RefCell::new(None));
    let mut captured_closure_clone = captured.clone(); let lists_closure_clone = lists.clone(); let mut set = Rc::new(RefCell::new(Some(Box::new(move || -> bool {
        let mut found: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));
        { let (__tmp_0, __tmp_1) = { let __map_holder = lists_closure_clone.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => ({ let __held = v.borrow().clone(); Rc::new(RefCell::new(__held)) }, true), None => (Default::default(), false) } }; let __moved_tmp_0 = { let mut __guard = __tmp_0.borrow_mut(); __guard.take() }; *captured_closure_clone.borrow_mut() = __moved_tmp_0; *found.borrow_mut() = Some(__tmp_1); };
        return (*found.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut() -> bool>)));
    println!("{} {} {} {}", format!("{}", "captured by a closure".to_string()), format!("{}", { let __f_ptr: *mut Box<dyn FnMut() -> bool> = { let mut __f_guard = set.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> bool> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }), format!("{}", format_slice(&{ let __map_holder = lists.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) })), format!("{}", format_slice(&captured)));

    let mut capturedNamed: Rc<RefCell<Option<Numbers>>> = Rc::new(RefCell::new(Some(Default::default())));
    let mut capturedNamed_closure_clone = capturedNamed.clone(); let m_closure_clone = m.clone(); let mut setNamed = Rc::new(RefCell::new(Some(Box::new(move || -> bool {
        let mut found: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));
        { let (__tmp_0, __tmp_1) = { let __map_holder = m_closure_clone.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => ({ let __held = v.borrow().clone(); Rc::new(RefCell::new(__held)) }, true), None => (Rc::new(RefCell::new(Some(Default::default()))), false) } }; let __moved_tmp_0 = { let mut __guard = __tmp_0.borrow_mut(); __guard.take() }; *capturedNamed_closure_clone.borrow_mut() = __moved_tmp_0; *found.borrow_mut() = Some(__tmp_1); };
        return (*found.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut() -> bool>)));
    println!("{} {} {} {}", format!("{}", "named slice captured by a closure".to_string()), format!("{}", { let __f_ptr: *mut Box<dyn FnMut() -> bool> = { let mut __f_guard = setNamed.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> bool> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }), format!("{}", { let __slice_holder = { let __named_slice = (*{ let __map_holder = m.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Rc::new(RefCell::new(Some(Default::default())))) }.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }), format!("{}", { let __slice_holder = { let __named_slice = (*capturedNamed.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }));

    let mut indexes = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Index>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(Index(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::from([("x".to_string(), Rc::new(RefCell::new(Some(1))))])))))))))]))));
    let mut idx: Rc<RefCell<Option<Index>>> = Rc::new(RefCell::new(Some(Index(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::new())))))));
    { let (__tmp_0, __tmp_1) = { let __map_holder = indexes.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => ({ let __held = v.borrow().clone(); Rc::new(RefCell::new(__held)) }, true), None => (Default::default(), false) } }; let __moved_tmp_0 = { let mut __guard = __tmp_0.borrow_mut(); __guard.take() }; *idx.borrow_mut() = __moved_tmp_0; *ok.borrow_mut() = Some(__tmp_1); };
    { let __map_key = "x".to_string(); let __map_value = Rc::new(RefCell::new(Some(2))); (*{ let __named_map = (*idx.borrow().as_ref().unwrap()).0.clone(); __named_map }.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{} {} {} {}", format!("{}", "named map".to_string()), format!("{}", { let __v = (*ok.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __map_holder = { let __named_map = (*{ let __map_holder = indexes.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.borrow().as_ref().unwrap()).0.clone(); __named_map }.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"x".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", { let __map_holder = { let __named_map = (*idx.borrow().as_ref().unwrap()).0.clone(); __named_map }.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"x".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
