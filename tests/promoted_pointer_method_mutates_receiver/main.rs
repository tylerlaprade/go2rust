use std::any::Any;
use std::cell::{RefCell};
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

thread_local! {
    static GO_EMBEDDED_OWNER_REGISTRY: RefCell<std::collections::HashMap<usize, Box<dyn Any>>> = RefCell::new(std::collections::HashMap::new());
}

fn go_register_embedded_owner<T: 'static>(embedded_key: usize, owner: Rc<RefCell<Option<T>>>) {
    GO_EMBEDDED_OWNER_REGISTRY.with(|registry| {
        registry.borrow_mut().insert(embedded_key, Box::new(owner));
    });
}

fn go_lookup_embedded_owner<T: 'static>(embedded_key: usize, target: &str) -> Rc<RefCell<Option<T>>> {
    GO_EMBEDDED_OWNER_REGISTRY.with(|registry| {
        let registry = registry.borrow();
        let owner = registry.get(&embedded_key).unwrap_or_else(|| panic!("embedded owner registry missing {}", target));
        owner
            .downcast_ref::<Rc<RefCell<Option<T>>>>()
            .unwrap_or_else(|| panic!("embedded owner registry type mismatch for {}", target))
            .clone()
    })
}

#[derive(Debug, Clone)]
pub struct Basic {
    pub id: Rc<RefCell<Option<String>>>,
    pub balance: Rc<RefCell<Option<f64>>>,
    pub history: Rc<RefCell<Option<Vec<String>>>>,
}

impl Basic {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.id.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.balance.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_2_0 = self.history.clone();
        Self {
            id: __go_clone_0_0,
            balance: __go_clone_1_0,
            history: __go_clone_2_0,
        }
    }
}


impl Default for Basic {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0.0)));
        let __go_default_2_0 = Rc::new(RefCell::new(None));
        Self {
            id: __go_default_0_0,
            balance: __go_default_1_0,
            history: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for Basic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.id.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.balance.borrow().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", format_slice(&self.history));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


#[derive(Debug, Clone)]
pub struct Savings {
    pub basic: Rc<RefCell<Option<Basic>>>,
    pub rate: Rc<RefCell<Option<f64>>>,
}

impl Savings {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.basic.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        let __go_clone_1_0 = { let __guard = self.rate.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            basic: __go_clone_0_0,
            rate: __go_clone_1_0,
        }
    }
}


impl Default for Savings {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(Basic::default())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0.0)));
        Self {
            basic: __go_default_0_0,
            rate: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Savings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.basic.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.rate.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct Premium {
    pub savings: Rc<RefCell<Option<Savings>>>,
    pub bonus: Rc<RefCell<Option<f64>>>,
}

impl Premium {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.savings.clone();
        let __go_clone_1_0 = { let __guard = self.bonus.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            savings: __go_clone_0_0,
            bonus: __go_clone_1_0,
        }
    }
}


impl Default for Premium {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(None));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0.0)));
        Self {
            savings: __go_default_0_0,
            bonus: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Premium {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.savings.borrow(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", (*self.bonus.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl Basic {
    pub fn deposit(&mut self, a: Rc<RefCell<Option<f64>>>) {
        { let __target = self.balance.clone(); let __rhs = (*a.borrow().as_ref().unwrap()); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let new_val = { let __append_target = self.history.clone(); let __append_value = (*Rc::new(RefCell::new(Some(format!("+{:.2}", { let __v = (*a.borrow().as_ref().unwrap()).clone(); __v })))).borrow().as_ref().unwrap()).clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; self.history = new_val; };
    }
}

impl Savings {
    pub fn add_interest(&mut self) {
        { let __method_arg0 = Rc::new(RefCell::new(Some((*(*self.basic.borrow().as_ref().unwrap()).balance.borrow().as_ref().unwrap()) * (*self.rate.borrow().as_ref().unwrap())))); self.deposit(__method_arg0) };
    }

    pub fn deposit(&mut self, a: Rc<RefCell<Option<f64>>>) {
        // Forward to embedded type's method
        let embedded = self.basic.clone();
        let mut guard = embedded.borrow_mut();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.deposit(a)
    }
}

impl Premium {
    pub fn reward(&mut self) {
        { let __promoted_recv = self.savings.clone(); let mut __promoted_guard = __promoted_recv.borrow_mut(); let __promoted_ref = __promoted_guard.as_mut().unwrap(); let __result = __promoted_ref.add_interest(); __result };
        { let __method_arg0 = Rc::new(RefCell::new(Some({ let __selector_holder = self.bonus.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))); self.deposit(__method_arg0) };
    }

    pub fn add_interest(&mut self) {
        // Forward to embedded type's method
        let embedded = self.savings.clone();
        let mut guard = embedded.borrow_mut();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.add_interest()
    }

    pub fn deposit(&mut self, a: Rc<RefCell<Option<f64>>>) {
        // Forward to embedded type's method
        let embedded = self.savings.clone();
        let mut guard = embedded.borrow_mut();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.deposit(a)
    }
}

fn main() {
    let mut s = {
        let __owner = Rc::new(RefCell::new(Some(Savings { basic: Rc::new(RefCell::new(Some(Basic { id: Rc::new(RefCell::new(Some("bob".to_string()))), ..Default::default() }))), rate: Rc::new(RefCell::new(Some(0.5))), ..Default::default() })));
        let __embedded = { let __owner_guard = __owner.borrow(); __owner_guard.as_ref().unwrap().basic.clone() };
        let __embedded_key = { let __embedded_guard = __embedded.borrow(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) };
        go_register_embedded_owner(__embedded_key, __owner.clone());
        __owner
    };
    (*s.borrow_mut().as_mut().unwrap()).deposit(Rc::new(RefCell::new(Some(100.0))));
    (*s.borrow_mut().as_mut().unwrap()).add_interest();
    println!("{} {} {}", format!("{}", (*(*(*s.borrow().as_ref().unwrap()).basic.borrow().as_ref().unwrap()).id.borrow().as_ref().unwrap()).clone()), format!("{}", (*(*(*s.borrow().as_ref().unwrap()).basic.borrow().as_ref().unwrap()).balance.borrow().as_ref().unwrap())), format!("{}", format_slice_values(&(*(*(*s.borrow().as_ref().unwrap()).basic.borrow().as_ref().unwrap()).history.borrow().as_ref().unwrap()).clone())));
    let mut p = {
        let __owner = Rc::new(RefCell::new(Some(Premium { savings: {
            let __owner = Rc::new(RefCell::new(Some(Savings { basic: Rc::new(RefCell::new(Some(Basic { id: Rc::new(RefCell::new(Some("ann".to_string()))), ..Default::default() }))), rate: Rc::new(RefCell::new(Some(0.1))), ..Default::default() })));
            let __embedded = { let __owner_guard = __owner.borrow(); __owner_guard.as_ref().unwrap().basic.clone() };
            let __embedded_key = { let __embedded_guard = __embedded.borrow(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) };
            go_register_embedded_owner(__embedded_key, __owner.clone());
            __owner
        }.clone(), bonus: Rc::new(RefCell::new(Some(5.0))), ..Default::default() })));
        let __embedded = { let __owner_guard = __owner.borrow(); __owner_guard.as_ref().unwrap().savings.clone() };
        let __embedded_key = { let __embedded_guard = __embedded.borrow(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) };
        go_register_embedded_owner(__embedded_key, __owner.clone());
        __owner
    };
    (*p.borrow_mut().as_mut().unwrap()).deposit(Rc::new(RefCell::new(Some(50.0))));
    (*p.borrow_mut().as_mut().unwrap()).reward();
    print!("{} {:.2} {}\n", (*(*(*p.borrow().as_ref().unwrap()).savings.borrow().as_ref().unwrap().basic.borrow().as_ref().unwrap()).id.borrow().as_ref().unwrap()).clone(), (*(*(*p.borrow().as_ref().unwrap()).savings.borrow().as_ref().unwrap().basic.borrow().as_ref().unwrap()).balance.borrow().as_ref().unwrap()), format_slice_values(&(*(*(*p.borrow().as_ref().unwrap()).savings.borrow().as_ref().unwrap().basic.borrow().as_ref().unwrap()).history.borrow().as_ref().unwrap()).clone()));
}