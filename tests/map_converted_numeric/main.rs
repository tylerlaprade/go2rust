use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub const TICK: i64 = 3;


#[derive(Debug, Default)]
pub struct Span(pub Rc<RefCell<Option<i64>>>);

impl Clone for Span {
    fn clone(&self) -> Self {
        let __value = (*self.0.borrow()).clone();
        Span(Rc::new(RefCell::new(__value)))
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for Span {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i64> for Span {
    fn eq(&self, other: &i64) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Span {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i64> for Span {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Span> for i64 {
    fn eq(&self, other: &Span) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<Span> for i64 {
    fn partial_cmp(&self, other: &Span) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for Span {
    type Output = Span;
    fn add(self, other: Self) -> Span {
        Span(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i64> for Span {
    type Output = Span;
    fn add(self, other: i64) -> Span {
        Span(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Span> for i64 {
    type Output = Span;
    fn add(self, other: Span) -> Span {
        Span(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Span {
    type Output = Span;
    fn sub(self, other: Self) -> Span {
        Span(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i64> for Span {
    type Output = Span;
    fn sub(self, other: i64) -> Span {
        Span(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Span> for i64 {
    type Output = Span;
    fn sub(self, other: Span) -> Span {
        Span(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Span {
    type Output = Span;
    fn mul(self, other: Self) -> Span {
        Span(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i64> for Span {
    type Output = Span;
    fn mul(self, other: i64) -> Span {
        Span(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Span> for i64 {
    type Output = Span;
    fn mul(self, other: Span) -> Span {
        Span(Rc::new(RefCell::new(Some(self * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Span {
    type Output = Span;
    fn div(self, other: Self) -> Span {
        Span(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i64> for Span {
    type Output = Span;
    fn div(self, other: i64) -> Span {
        Span(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Span> for i64 {
    type Output = Span;
    fn div(self, other: Span) -> Span {
        Span(Rc::new(RefCell::new(Some(self / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for Span {
    type Output = Span;
    fn neg(self) -> Span {
        Span(Rc::new(RefCell::new(Some(-*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for Span {
    type Output = Span;
    fn rem(self, other: Self) -> Span {
        Span(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i64> for Span {
    type Output = Span;
    fn rem(self, other: i64) -> Span {
        Span(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<Span> for i64 {
    type Output = Span;
    fn rem(self, other: Span) -> Span {
        Span(Rc::new(RefCell::new(Some(self % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Span {
    type Output = Span;
    fn bitand(self, other: Self) -> Span {
        Span(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i64> for Span {
    type Output = Span;
    fn bitand(self, other: i64) -> Span {
        Span(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Span> for i64 {
    type Output = Span;
    fn bitand(self, other: Span) -> Span {
        Span(Rc::new(RefCell::new(Some(self & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Span {
    type Output = Span;
    fn bitor(self, other: Self) -> Span {
        Span(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i64> for Span {
    type Output = Span;
    fn bitor(self, other: i64) -> Span {
        Span(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Span> for i64 {
    type Output = Span;
    fn bitor(self, other: Span) -> Span {
        Span(Rc::new(RefCell::new(Some(self | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Span {
    type Output = Span;
    fn bitxor(self, other: Self) -> Span {
        Span(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i64> for Span {
    type Output = Span;
    fn bitxor(self, other: i64) -> Span {
        Span(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Span> for i64 {
    type Output = Span;
    fn bitxor(self, other: Span) -> Span {
        Span(Rc::new(RefCell::new(Some(self ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Span {
    type Output = Span;
    fn not(self) -> Span {
        Span(Rc::new(RefCell::new(Some(!*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Span {
    type Output = Span;
    fn shl(self, other: Span) -> Span {
        Span(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for Span {
    type Output = Span;
    fn shl(self, other: i32) -> Span {
        Span(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for Span {
    type Output = Span;
    fn shl(self, other: i8) -> Span {
        Span(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for Span {
    type Output = Span;
    fn shl(self, other: i16) -> Span {
        Span(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for Span {
    type Output = Span;
    fn shl(self, other: i64) -> Span {
        Span(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for Span {
    type Output = Span;
    fn shl(self, other: u32) -> Span {
        Span(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for Span {
    type Output = Span;
    fn shl(self, other: u8) -> Span {
        Span(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for Span {
    type Output = Span;
    fn shl(self, other: u16) -> Span {
        Span(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for Span {
    type Output = Span;
    fn shl(self, other: u64) -> Span {
        Span(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for Span {
    type Output = Span;
    fn shl(self, other: usize) -> Span {
        Span(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for Span {
    type Output = Span;
    fn shr(self, other: Span) -> Span {
        Span(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for Span {
    type Output = Span;
    fn shr(self, other: i32) -> Span {
        Span(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for Span {
    type Output = Span;
    fn shr(self, other: i8) -> Span {
        Span(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for Span {
    type Output = Span;
    fn shr(self, other: i16) -> Span {
        Span(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for Span {
    type Output = Span;
    fn shr(self, other: i64) -> Span {
        Span(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for Span {
    type Output = Span;
    fn shr(self, other: u32) -> Span {
        Span(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for Span {
    type Output = Span;
    fn shr(self, other: u8) -> Span {
        Span(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for Span {
    type Output = Span;
    fn shr(self, other: u16) -> Span {
        Span(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for Span {
    type Output = Span;
    fn shr(self, other: u64) -> Span {
        Span(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for Span {
    type Output = Span;
    fn shr(self, other: usize) -> Span {
        Span(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl Eq for Span {}

impl Ord for Span {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


pub type Code = Rc<RefCell<Option<u16>>>;


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

pub(crate) static units: GoGlobal<BTreeMap<String, Rc<RefCell<Option<u64>>>>> = GoGlobal::new();


fn __go_init_globals() {
    *units.borrow_mut() = Some(BTreeMap::new());
    {
        let mut __go_map = BTreeMap::<String, Rc<RefCell<Option<u64>>>>::new();
        __go_map.insert("tick".to_string(), Rc::new(RefCell::new(Some(TICK as i64 as u64))));
        *units.borrow_mut() = Some(__go_map);
    }
}


fn main() {
    __go_init_all();
    let mut n = Rc::new(RefCell::new(Some(-(4) as i16)));
    let mut signed = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::from([("n".to_string(), Rc::new(RefCell::new(Some((*n.borrow().as_ref().unwrap()) as i32))))]))));
    let mut decimal = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<f64>>>>::from([("tick".to_string(), Rc::new(RefCell::new(Some(TICK as i64 as f64))))]))));
    {
            let __go_print_arg_0 = format!("{}", { let __map_holder = units.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"tick".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) });
            let __go_print_arg_1 = format!("{}", { let __map_holder = signed.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"n".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) });
            let __go_print_arg_2 = format!("{}", (*Rc::new(RefCell::new(Some({ let __map_holder = decimal.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"tick".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0.0) } as i32))).borrow().as_ref().unwrap()));
            eprintln!("{} {} {}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    let mut wide = Rc::new(RefCell::new(Some(270 as i32)));
    let mut codes = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<u16>>>>::from([("x".to_string(), Rc::new(RefCell::new(Some((*wide.borrow().as_ref().unwrap()) as u16))))]))));
    let mut flags = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<bool>>>>::from([("x".to_string(), Rc::new(RefCell::new(Some(true))))]))));
    let mut labels = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<String>>>>::from([("x".to_string(), Rc::new(RefCell::new(Some(char::from_u32((('x' as i32)) as u32).unwrap().to_string()))))]))));
    {
            let __go_print_arg_0 = format!("{}", { let __map_holder = codes.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"x".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) });
            let __go_print_arg_1 = format!("{}", { let __map_holder = flags.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"x".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| false) });
            let __go_print_arg_2 = format!("{}", { let __map_holder = labels.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"x".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()) });
            eprintln!("{} {} {}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
