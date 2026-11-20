use std::any::Any;
use std::cell::{RefCell};
use std::cmp::Ord;
use std::collections::BTreeMap;
use std::error::Error as StdError;
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

#[derive(Clone, Default)]
pub struct Result_ {
    pub err: Rc<RefCell<Option<Rc<dyn StdError>>>>,
    pub next: Rc<RefCell<Option<Result_>>>,
    pub tags: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>>,
}

impl Result_ {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.err.clone();
        let __go_clone_1_0 = self.next.clone();
        let __go_clone_2_0 = self.tags.clone();
        Self {
            err: __go_clone_0_0,
            next: __go_clone_1_0,
            tags: __go_clone_2_0,
        }
    }
}

impl std::fmt::Display for Result_ {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", match self.err.borrow().as_ref() { Some(__v) => format!("{}", __v), None => "<nil>".to_string() });
        let __go_fmt_1 = format!("{}", { let __guard = self.next.borrow(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_2 = format!("{}", format_map(&self.tags));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


#[derive(Debug, Clone)]
pub struct T {
    pub x: Rc<RefCell<Option<i32>>>,
}

impl T {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
        }
    }
}


impl Default for T {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            x: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for T {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
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

pub(crate) static lastErr: GoGlobal<Rc<dyn StdError>> = GoGlobal::new();

pub(crate) static cache: GoGlobal<BTreeMap<String, Rc<RefCell<Option<i32>>>>> = GoGlobal::new();


fn __go_init_globals() {
    *lastErr.borrow_mut() = None;
    *cache.borrow_mut() = Some(BTreeMap::new());
}


impl Result_ {
    pub fn check(&self) {
        let _ = &self.err;
        let _ = &self.next;
    }

    pub fn check_pointer(&self) {
        let _ = &self.err;
        let _ = &self.tags;
    }
}

pub fn find() -> (Rc<RefCell<Option<T>>>, Rc<RefCell<Option<Rc<dyn StdError>>>>) {
    (Rc::new(RefCell::new(None)), Rc::new(RefCell::new(None)))
}

fn main() {
    __go_init_all();
    let mut err: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(None));
    let _ = &err;
    let mut ptr: Rc<RefCell<Option<T>>> = Rc::new(RefCell::new(None));
    let _ = &ptr;
    let mut m: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>> = Rc::new(RefCell::new(Some(BTreeMap::new())));
    let _ = &m;
    let mut s: Rc<RefCell<Option<Vec<i32>>>> = Rc::new(RefCell::new(None));
    let _ = &s;
    let mut a: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(None));
    let _ = &a;
    let (mut q, mut e2) = find();
    let _ = &q;
    let _ = &e2;
    let mut f: Rc<RefCell<Option<Box<dyn FnMut() -> ()>>>> = Rc::new(RefCell::new(None));
    let _ = &f;

    let _ = &lastErr;
    let _ = &cache;
    let mut r: Rc<RefCell<Option<Result_>>> = Rc::new(RefCell::new(Some(Default::default())));
    let _ = &(*r.borrow().as_ref().unwrap()).err;
    let _ = &(*r.borrow().as_ref().unwrap()).next;
    let _ = &(*r.borrow().as_ref().unwrap()).tags;
    let _ = &((*r.borrow().as_ref().unwrap()).err);
    (*r.borrow().as_ref().unwrap()).check();
    let mut p = Rc::new(RefCell::new(Some(Result_ { err: Default::default(), next: Default::default(), tags: Default::default() })));
    let _ = &(*p.borrow().as_ref().unwrap()).err;
    let _ = &(*p.borrow().as_ref().unwrap()).next;
    (*p.borrow().as_ref().unwrap()).check_pointer();
    let mut inner = Rc::new(RefCell::new(Some(Result_ { next: Rc::new(RefCell::new(Some(Result_ { err: Default::default(), next: Default::default(), tags: Default::default() }))).clone(), ..Default::default() })));
    let _ = &(*(*inner.borrow().as_ref().unwrap()).next.borrow().as_ref().unwrap()).err;

    { let __rhs_holder = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("kept".to_string()))))).clone(); let new_val = (*__rhs_holder.borrow()).clone(); *lastErr.borrow_mut() = new_val; };
    let _ = &lastErr;
    println!("{} {} {}", format!("{}", { let __text = match lastErr.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __nil_target = (*r.borrow().as_ref().unwrap()).err.clone(); let __nil_result = (*__nil_target.borrow()).is_none(); __nil_result }), format!("{}", { let __nil_target = (*p.borrow().as_ref().unwrap()).next.clone(); let __nil_result = (*__nil_target.borrow()).is_none(); __nil_result }));

    let mut count = Rc::new(RefCell::new(Some(0)));
    let mut count_closure_clone = count.clone(); let mut bump = Rc::new(RefCell::new(Some(Box::new(move || -> i32 {
        { let mut guard = count_closure_clone.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return (*count_closure_clone.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut() -> i32>)));
    let _ = { let __f_ptr: *mut Box<dyn FnMut() -> i32> = { let mut __f_guard = bump.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i32> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    let _ = (*Rc::new(RefCell::new(Some(vec![{ let __f_ptr: *mut Box<dyn FnMut() -> i32> = { let mut __f_guard = bump.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i32> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }]))).borrow().as_ref().unwrap())[(0) as usize].clone();
    println!("{}", format!("{}", { let __v = (*count.borrow().as_ref().unwrap()).clone(); __v }));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
