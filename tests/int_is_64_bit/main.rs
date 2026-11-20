use std::any::Any;
use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::rc::{Rc};


fn __go_type_name(val: &dyn Any) -> &'static str {
    if val.is::<i64>() { return "int" }
    if val.is::<i32>() { return "int32" }
    if val.is::<i8>() { return "int8" }
    if val.is::<i16>() { return "int16" }
    if val.is::<u64>() { return "uint" }
    if val.is::<u32>() { return "uint32" }
    if val.is::<u8>() { return "uint8" }
    if val.is::<u16>() { return "uint16" }
    if val.is::<f64>() { return "float64" }
    if val.is::<f32>() { return "float32" }
    if val.is::<bool>() { return "bool" }
    if val.is::<String>() { return "string" }
    if val.is::<Vec<i64>>() { return "[]int" }
    if val.is::<Vec<i32>>() { return "[]int32" }
    if val.is::<Vec<f64>>() { return "[]float64" }
    if val.is::<Vec<String>>() { return "[]string" }
    if val.is::<Vec<bool>>() { return "[]bool" }
    std::any::type_name_of_val(val)
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

pub(crate) static memo: GoGlobal<BTreeMap<i64, Rc<RefCell<Option<i64>>>>> = GoGlobal::new();


fn __go_init_globals() {
    *memo.borrow_mut() = Some(BTreeMap::new());
    {
        let mut __go_map = BTreeMap::<i64, Rc<RefCell<Option<i64>>>>::new();
        *memo.borrow_mut() = Some(__go_map);
    }
}


pub fn fib(n: Rc<RefCell<Option<i64>>>) -> i64 {
    if (*n.borrow().as_ref().unwrap()) < 2 {
        return (*n.borrow().as_ref().unwrap());
    }
    {
        let (mut v, mut ok) = { let __map_holder = memo.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&(*n.borrow().as_ref().unwrap()))) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().clone()))), true), None => (Rc::new(RefCell::new(Some(0))), false) } };;
        if ok {
            return (*v.borrow().as_ref().unwrap());;
        }
    }
    let mut v = Rc::new(RefCell::new(Some(fib(Rc::new(RefCell::new(Some((*n.borrow().as_ref().unwrap()) - 1)))) + fib(Rc::new(RefCell::new(Some((*n.borrow().as_ref().unwrap()) - 2)))))));
    { let __map_key = (*n.borrow().as_ref().unwrap()); let __map_value = Rc::new(RefCell::new(Some((*v.borrow().as_ref().unwrap()).clone()))); (*memo.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    return (*v.borrow().as_ref().unwrap());
}

pub fn factorial(n: Rc<RefCell<Option<i64>>>) -> i64 {
    if (*n.borrow().as_ref().unwrap()) <= 1 {
        return 1;
    }
    (*n.borrow().as_ref().unwrap()) * factorial(Rc::new(RefCell::new(Some((*n.borrow().as_ref().unwrap()) - 1))))
}

fn main() {
    __go_init_all();
    println!("{} {}", format!("{}", fib(Rc::new(RefCell::new(Some(90))))), format!("{}", factorial(Rc::new(RefCell::new(Some(20))))));
    let mut big = Rc::new(RefCell::new(Some(1099511627776i64)));
    println!("{} {} {} {}", format!("{}", { let __v = (*big.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", (*big.borrow().as_ref().unwrap()) * 3), format!("{}", 9223372036854775807i64), format!("{}", i64::MIN));
    let mut total: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));
    let mut i = Rc::new(RefCell::new(Some(0i64)));
    while (*i.borrow().as_ref().unwrap()) < 100000 {
        { let __rhs = { let __bin_i = (*i.borrow().as_ref().unwrap()).clone(); __bin_i * __bin_i }; let mut guard = total.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    println!("{}", format!("{}", { let __v = (*total.borrow().as_ref().unwrap()).clone(); __v }));
    let (mut n, _) = { let __atoi_input = "9000000000".to_string(); match __atoi_input.parse::<i64>() { Ok(n) => (n, Rc::new(RefCell::new(None))), Err(_) => (0 as i64, Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(format!("strconv.Atoi: parsing \"{}\": invalid syntax", __atoi_input))))))) } };
    println!("{} {}", format!("{}", n + 1), format!("{}", (*Rc::new(RefCell::new(Some(((n * 2)).to_string()))).borrow().as_ref().unwrap())));
    let mut wrapped = Rc::new(RefCell::new(Some(9223372036854775807i64)));
    { let mut guard = wrapped.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    println!("{}", format!("{}", (*wrapped.borrow().as_ref().unwrap()) == i64::MIN));
    let mut x: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(7i64) as Box<dyn Any>)));
    let (_, mut isInt) = ({
        let val = x.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<i64>() {
                (Rc::new(RefCell::new(Some(typed_val.clone()))), true)
            } else {
                (Rc::new(RefCell::new(Some(0))), false)
            }
        } else {
            (Rc::new(RefCell::new(Some(0))), false)
        }
    });
    print!("{} {} {}\n", __go_type_name(&**x.borrow().as_ref().unwrap()), isInt, 5000000000i64);
    let mut r: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(('A' as i32))));
    print!("{} {} {}\n", "int32", { let __v = (*r.borrow().as_ref().unwrap()).clone(); __v }, ({ let __v = (*r.borrow().as_ref().unwrap()).clone(); __v }) as u8 as char);
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
