use std::cell::{RefCell};
use std::collections::BTreeMap;
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

#[derive(Debug, Default)]
pub struct Counter(pub Rc<RefCell<Option<i32>>>);

impl Clone for Counter {
    fn clone(&self) -> Self {
        let __value = (*self.0.borrow()).clone();
        Counter(Rc::new(RefCell::new(__value)))
    }
}

impl Display for Counter {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for Counter {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for Counter {
    fn eq(&self, other: &i32) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Counter {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for Counter {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Counter> for i32 {
    fn eq(&self, other: &Counter) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<Counter> for i32 {
    fn partial_cmp(&self, other: &Counter) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for Counter {
    type Output = Counter;
    fn add(self, other: Self) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for Counter {
    type Output = Counter;
    fn add(self, other: i32) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Counter> for i32 {
    type Output = Counter;
    fn add(self, other: Counter) -> Counter {
        Counter(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Counter {
    type Output = Counter;
    fn sub(self, other: Self) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for Counter {
    type Output = Counter;
    fn sub(self, other: i32) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Counter> for i32 {
    type Output = Counter;
    fn sub(self, other: Counter) -> Counter {
        Counter(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Counter {
    type Output = Counter;
    fn mul(self, other: Self) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for Counter {
    type Output = Counter;
    fn mul(self, other: i32) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Counter> for i32 {
    type Output = Counter;
    fn mul(self, other: Counter) -> Counter {
        Counter(Rc::new(RefCell::new(Some(self * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Counter {
    type Output = Counter;
    fn div(self, other: Self) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for Counter {
    type Output = Counter;
    fn div(self, other: i32) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Counter> for i32 {
    type Output = Counter;
    fn div(self, other: Counter) -> Counter {
        Counter(Rc::new(RefCell::new(Some(self / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for Counter {
    type Output = Counter;
    fn neg(self) -> Counter {
        Counter(Rc::new(RefCell::new(Some(-*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for Counter {
    type Output = Counter;
    fn rem(self, other: Self) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for Counter {
    type Output = Counter;
    fn rem(self, other: i32) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<Counter> for i32 {
    type Output = Counter;
    fn rem(self, other: Counter) -> Counter {
        Counter(Rc::new(RefCell::new(Some(self % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Counter {
    type Output = Counter;
    fn bitand(self, other: Self) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for Counter {
    type Output = Counter;
    fn bitand(self, other: i32) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Counter> for i32 {
    type Output = Counter;
    fn bitand(self, other: Counter) -> Counter {
        Counter(Rc::new(RefCell::new(Some(self & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Counter {
    type Output = Counter;
    fn bitor(self, other: Self) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for Counter {
    type Output = Counter;
    fn bitor(self, other: i32) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Counter> for i32 {
    type Output = Counter;
    fn bitor(self, other: Counter) -> Counter {
        Counter(Rc::new(RefCell::new(Some(self | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Counter {
    type Output = Counter;
    fn bitxor(self, other: Self) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for Counter {
    type Output = Counter;
    fn bitxor(self, other: i32) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Counter> for i32 {
    type Output = Counter;
    fn bitxor(self, other: Counter) -> Counter {
        Counter(Rc::new(RefCell::new(Some(self ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Counter {
    type Output = Counter;
    fn not(self) -> Counter {
        Counter(Rc::new(RefCell::new(Some(!*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Counter {
    type Output = Counter;
    fn shl(self, other: Counter) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for Counter {
    type Output = Counter;
    fn shl(self, other: i32) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for Counter {
    type Output = Counter;
    fn shl(self, other: i8) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for Counter {
    type Output = Counter;
    fn shl(self, other: i16) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for Counter {
    type Output = Counter;
    fn shl(self, other: i64) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for Counter {
    type Output = Counter;
    fn shl(self, other: u32) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for Counter {
    type Output = Counter;
    fn shl(self, other: u8) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for Counter {
    type Output = Counter;
    fn shl(self, other: u16) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for Counter {
    type Output = Counter;
    fn shl(self, other: u64) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for Counter {
    type Output = Counter;
    fn shl(self, other: usize) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for Counter {
    type Output = Counter;
    fn shr(self, other: Counter) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for Counter {
    type Output = Counter;
    fn shr(self, other: i32) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for Counter {
    type Output = Counter;
    fn shr(self, other: i8) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for Counter {
    type Output = Counter;
    fn shr(self, other: i16) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for Counter {
    type Output = Counter;
    fn shr(self, other: i64) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for Counter {
    type Output = Counter;
    fn shr(self, other: u32) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for Counter {
    type Output = Counter;
    fn shr(self, other: u8) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for Counter {
    type Output = Counter;
    fn shr(self, other: u16) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for Counter {
    type Output = Counter;
    fn shr(self, other: u64) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for Counter {
    type Output = Counter;
    fn shr(self, other: usize) -> Counter {
        Counter(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl Eq for Counter {}

impl Ord for Counter {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug, Default)]
pub struct Celsius(pub Rc<RefCell<Option<f64>>>);

impl Clone for Celsius {
    fn clone(&self) -> Self {
        let __value = (*self.0.borrow()).clone();
        Celsius(Rc::new(RefCell::new(__value)))
    }
}

impl Display for Celsius {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for Celsius {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<f64> for Celsius {
    fn eq(&self, other: &f64) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Celsius {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<f64> for Celsius {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Celsius> for f64 {
    fn eq(&self, other: &Celsius) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<Celsius> for f64 {
    fn partial_cmp(&self, other: &Celsius) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for Celsius {
    type Output = Celsius;
    fn add(self, other: Self) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<f64> for Celsius {
    type Output = Celsius;
    fn add(self, other: f64) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Celsius> for f64 {
    type Output = Celsius;
    fn add(self, other: Celsius) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Celsius {
    type Output = Celsius;
    fn sub(self, other: Self) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<f64> for Celsius {
    type Output = Celsius;
    fn sub(self, other: f64) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Celsius> for f64 {
    type Output = Celsius;
    fn sub(self, other: Celsius) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Celsius {
    type Output = Celsius;
    fn mul(self, other: Self) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<f64> for Celsius {
    type Output = Celsius;
    fn mul(self, other: f64) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Celsius> for f64 {
    type Output = Celsius;
    fn mul(self, other: Celsius) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(self * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Celsius {
    type Output = Celsius;
    fn div(self, other: Self) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div<f64> for Celsius {
    type Output = Celsius;
    fn div(self, other: f64) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Celsius> for f64 {
    type Output = Celsius;
    fn div(self, other: Celsius) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(self / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for Celsius {
    type Output = Celsius;
    fn neg(self) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(-*self.0.borrow().as_ref().unwrap()))))
    }
}


#[derive(Debug, Default)]
pub struct Name(pub Rc<RefCell<Option<String>>>);

impl Clone for Name {
    fn clone(&self) -> Self {
        let __value = (*self.0.borrow()).clone();
        Name(Rc::new(RefCell::new(__value)))
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left == __right
    }
}


#[derive(Debug, Default)]
pub struct Flag(pub Rc<RefCell<Option<bool>>>);

impl Clone for Flag {
    fn clone(&self) -> Self {
        let __value = (*self.0.borrow()).clone();
        Flag(Rc::new(RefCell::new(__value)))
    }
}

impl Display for Flag {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for Flag {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left == __right
    }
}


#[derive(Debug, Clone)]
pub struct Stats {
    pub hits: Rc<RefCell<Option<Counter>>>,
    pub temp: Rc<RefCell<Option<Celsius>>>,
}

impl Stats {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.hits.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.temp.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            hits: __go_clone_0_0,
            temp: __go_clone_1_0,
        }
    }
}


impl Default for Stats {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(Counter(Rc::new(RefCell::new(Some(0)))))));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(Celsius(Rc::new(RefCell::new(Some(0.0)))))));
        Self {
            hits: __go_default_0_0,
            temp: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.hits.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.temp.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl Counter {
    pub fn inc(&mut self) {
        { let mut guard = self.0.borrow_mut(); *guard = Some(guard.as_ref().unwrap().clone() + 1 as i32); }
    }
}

impl Celsius {
    pub fn warm(&mut self) {
        { let __rhs = 1.5; let mut guard = self.0.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
}

impl Name {
    pub fn shout(&mut self) {
        { (*self.0.borrow_mut().as_mut().unwrap()).push_str(&"!".to_string()); };
    }
}

impl Flag {
    pub fn toggle(&mut self) {
        { let new_val = Flag(Rc::new(RefCell::new(Some(!((*self.0.borrow().as_ref().unwrap())))))); *self = new_val; };
    }
}

pub fn bump(c: Rc<RefCell<Option<Counter>>>) -> Rc<RefCell<Option<Counter>>> {
    (*c.borrow_mut().as_mut().unwrap()).inc();
    { let __owned = c.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) }
}

fn main() {
    let mut a = Rc::new(RefCell::new(Some(Counter(Rc::new(RefCell::new(Some(1 as i32)))))));
    let mut b = { let __owned = a.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    (*b.borrow_mut().as_mut().unwrap()).inc();
    println!("{} {} {} {}", format!("{}", { let __v = (*a.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*b.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", (*bump(Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })))).borrow().as_ref().unwrap())), format!("{}", { let __v = (*a.borrow().as_ref().unwrap()).clone(); __v }));

    let mut x = Rc::new(RefCell::new(Some(Celsius(Rc::new(RefCell::new(Some(1 as f64)))))));
    let mut y = { let __owned = x.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    (*y.borrow_mut().as_mut().unwrap()).warm();
    println!("{} {}", format!("{}", { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*y.borrow().as_ref().unwrap()).clone(); __v }));

    let mut n = Rc::new(RefCell::new(Some(Name(Rc::new(RefCell::new(Some("go".to_string())))))));
    let mut m = { let __owned = n.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    (*m.borrow_mut().as_mut().unwrap()).shout();
    println!("{} {}", format!("{}", { let __v = (*n.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*m.borrow().as_ref().unwrap()).clone(); __v }));

    let mut f = Rc::new(RefCell::new(Some(Flag(Rc::new(RefCell::new(Some(false)))))));
    let mut g = { let __owned = f.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    (*g.borrow_mut().as_mut().unwrap()).toggle();
    println!("{} {}", format!("{}", { let __v = (*f.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*g.borrow().as_ref().unwrap()).clone(); __v }));

    let mut s = Rc::new(RefCell::new(Some(Stats { hits: Rc::new(RefCell::new(Some(Counter(Rc::new(RefCell::new(Some(5 as i32))))))), temp: Rc::new(RefCell::new(Some(Celsius(Rc::new(RefCell::new(Some(20 as f64))))))), ..Default::default() })));
    let mut t = { let __owned = s.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) };
    (*(*t.borrow().as_ref().unwrap()).hits.borrow_mut().as_mut().unwrap()).inc();
    (*(*t.borrow().as_ref().unwrap()).temp.borrow_mut().as_mut().unwrap()).warm();
    println!("{} {}", format!("{}", (*s.borrow().as_ref().unwrap())), format!("{}", (*t.borrow().as_ref().unwrap())));

    let mut counters = Rc::new(RefCell::new(Some(vec![Counter(Rc::new(RefCell::new(Some(1 as i32)))), Counter(Rc::new(RefCell::new(Some(2 as i32)))), Counter(Rc::new(RefCell::new(Some(3 as i32))))])));
    let mut first = Rc::new(RefCell::new(Some(Counter(Rc::new(RefCell::new(Some((*(*counters.borrow().as_ref().unwrap())[(0) as usize].clone().0.borrow().as_ref().unwrap()))))))));
    (*first.borrow_mut().as_mut().unwrap()).inc();
    { let __range_holder = counters.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for mut c in __range_values.iter().cloned() {
        c.inc();
    } }
    println!("{} {}", format!("{}", format_slice(&counters)), format!("{}", { let __v = (*first.borrow().as_ref().unwrap()).clone(); __v }));

    let mut p = a.clone();
    (*p.borrow_mut().as_mut().unwrap()).inc();
    println!("{} {}", format!("{}", { let __v = (*a.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*p.borrow().as_ref().unwrap()).clone(); __v }));
    let mut byName = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Counter>>>>::from([("k".to_string(), Rc::new(RefCell::new(Some(Counter(Rc::new(RefCell::new(Some(10 as i32))))))))]))));
    let mut v = Rc::new(RefCell::new(Some(Counter(Rc::new(RefCell::new(Some((*{ let __map_holder = byName.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"k".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| Counter(Rc::new(RefCell::new(Some(0))))) }.0.borrow().as_ref().unwrap()))))))));
    (*v.borrow_mut().as_mut().unwrap()).inc();
    println!("{} {}", format!("{}", { let __map_holder = byName.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"k".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| Counter(Rc::new(RefCell::new(Some(0))))) }), format!("{}", { let __v = (*v.borrow().as_ref().unwrap()).clone(); __v }));
}