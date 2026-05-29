use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct Kind(pub Rc<RefCell<Option<i32>>>);

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for Kind {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().as_ref().unwrap() == other.0.borrow().as_ref().unwrap()
    }
}

impl PartialEq<i32> for Kind {
    fn eq(&self, other: &i32) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Kind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl PartialOrd<i32> for Kind {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Kind> for i32 {
    fn eq(&self, other: &Kind) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<Kind> for i32 {
    fn partial_cmp(&self, other: &Kind) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for Kind {
    type Output = Kind;
    fn add(self, other: Self) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for Kind {
    type Output = Kind;
    fn add(self, other: i32) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Kind> for i32 {
    type Output = Kind;
    fn add(self, other: Kind) -> Kind {
        Kind(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Kind {
    type Output = Kind;
    fn sub(self, other: Self) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for Kind {
    type Output = Kind;
    fn sub(self, other: i32) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Kind> for i32 {
    type Output = Kind;
    fn sub(self, other: Kind) -> Kind {
        Kind(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Kind {
    type Output = Kind;
    fn mul(self, other: Self) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for Kind {
    type Output = Kind;
    fn mul(self, other: i32) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Kind> for i32 {
    type Output = Kind;
    fn mul(self, other: Kind) -> Kind {
        Kind(Rc::new(RefCell::new(Some(self * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Kind {
    type Output = Kind;
    fn div(self, other: Self) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for Kind {
    type Output = Kind;
    fn div(self, other: i32) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Kind> for i32 {
    type Output = Kind;
    fn div(self, other: Kind) -> Kind {
        Kind(Rc::new(RefCell::new(Some(self / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for Kind {
    type Output = Kind;
    fn neg(self) -> Kind {
        Kind(Rc::new(RefCell::new(Some(-*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for Kind {
    type Output = Kind;
    fn rem(self, other: Self) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for Kind {
    type Output = Kind;
    fn rem(self, other: i32) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<Kind> for i32 {
    type Output = Kind;
    fn rem(self, other: Kind) -> Kind {
        Kind(Rc::new(RefCell::new(Some(self % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Kind {
    type Output = Kind;
    fn bitand(self, other: Self) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for Kind {
    type Output = Kind;
    fn bitand(self, other: i32) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Kind> for i32 {
    type Output = Kind;
    fn bitand(self, other: Kind) -> Kind {
        Kind(Rc::new(RefCell::new(Some(self & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Kind {
    type Output = Kind;
    fn bitor(self, other: Self) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for Kind {
    type Output = Kind;
    fn bitor(self, other: i32) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Kind> for i32 {
    type Output = Kind;
    fn bitor(self, other: Kind) -> Kind {
        Kind(Rc::new(RefCell::new(Some(self | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Kind {
    type Output = Kind;
    fn bitxor(self, other: Self) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for Kind {
    type Output = Kind;
    fn bitxor(self, other: i32) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Kind> for i32 {
    type Output = Kind;
    fn bitxor(self, other: Kind) -> Kind {
        Kind(Rc::new(RefCell::new(Some(self ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Kind {
    type Output = Kind;
    fn not(self) -> Kind {
        Kind(Rc::new(RefCell::new(Some(!*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Kind {
    type Output = Kind;
    fn shl(self, other: Kind) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for Kind {
    type Output = Kind;
    fn shl(self, other: i32) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for Kind {
    type Output = Kind;
    fn shl(self, other: i8) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for Kind {
    type Output = Kind;
    fn shl(self, other: i16) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for Kind {
    type Output = Kind;
    fn shl(self, other: i64) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for Kind {
    type Output = Kind;
    fn shl(self, other: u32) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for Kind {
    type Output = Kind;
    fn shl(self, other: u8) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for Kind {
    type Output = Kind;
    fn shl(self, other: u16) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for Kind {
    type Output = Kind;
    fn shl(self, other: u64) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for Kind {
    type Output = Kind;
    fn shl(self, other: usize) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for Kind {
    type Output = Kind;
    fn shr(self, other: Kind) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for Kind {
    type Output = Kind;
    fn shr(self, other: i32) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for Kind {
    type Output = Kind;
    fn shr(self, other: i8) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for Kind {
    type Output = Kind;
    fn shr(self, other: i16) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for Kind {
    type Output = Kind;
    fn shr(self, other: i64) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for Kind {
    type Output = Kind;
    fn shr(self, other: u32) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for Kind {
    type Output = Kind;
    fn shr(self, other: u8) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for Kind {
    type Output = Kind;
    fn shr(self, other: u16) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for Kind {
    type Output = Kind;
    fn shr(self, other: u64) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for Kind {
    type Output = Kind;
    fn shr(self, other: usize) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl Eq for Kind {}

impl Ord for Kind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug, Clone)]
pub struct Entry {
    pub kind: Rc<RefCell<Option<Kind>>>,
}

impl Entry {
    pub fn __go_value_clone(&self) -> Self {
        Self { kind: { let __guard = self.kind.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Entry {
    fn default() -> Self {
        Self { kind: Rc::new(RefCell::new(Some(Kind(Rc::new(RefCell::new(Some(0))))))) }
    }
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.kind.borrow().as_ref().unwrap()))
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

pub(crate) static kindIndex: GoGlobal<[u16; 3]> = GoGlobal::new();


fn __go_init_globals() {
    *kindIndex.borrow_mut() = Some(std::array::from_fn(|_| 0));
    *kindIndex.borrow_mut() = Some((*Rc::new(RefCell::new(Some([0 as u16, 2 as u16, 4 as u16]))).borrow().as_ref().unwrap()).clone());
}


impl Kind {
    pub fn method_int(&self) -> i32 {
        (*Rc::new(RefCell::new(Some((*self.0.borrow().as_ref().unwrap()) as i32))).borrow().as_ref().unwrap())
    }

    pub fn method_plus(&self) -> i32 {
        (*Rc::new(RefCell::new(Some((((*self.0.borrow().as_ref().unwrap()) + 1)) as i32))).borrow().as_ref().unwrap())
    }
}

pub fn as_int(k: Rc<RefCell<Option<Kind>>>) -> i32 {
    (*Rc::new(RefCell::new(Some((*(*k.borrow().as_ref().unwrap()).0.borrow().as_ref().unwrap()) as i32))).borrow().as_ref().unwrap())
}

pub fn as_uint64(k: Rc<RefCell<Option<Kind>>>) -> u64 {
    (*Rc::new(RefCell::new(Some((*(*k.borrow().as_ref().unwrap()).0.borrow().as_ref().unwrap()) as u64))).borrow().as_ref().unwrap())
}

pub fn field_as_int(e: Rc<RefCell<Option<Entry>>>) -> i32 {
    (*Rc::new(RefCell::new(Some((*(*(*e.borrow().as_ref().unwrap()).kind.borrow().as_ref().unwrap()).0.borrow().as_ref().unwrap()) as i32))).borrow().as_ref().unwrap())
}

pub fn plus_as_int(k: Rc<RefCell<Option<Kind>>>) -> i32 {
    (*Rc::new(RefCell::new(Some((((*(*k.borrow().as_ref().unwrap()).0.borrow().as_ref().unwrap()) + 1)) as i32))).borrow().as_ref().unwrap())
}

pub fn limit_kind() -> Rc<RefCell<Option<Kind>>> {
    Rc::new(RefCell::new(Some(Kind(Rc::new(RefCell::new(Some(((*kindIndex.borrow().as_ref().unwrap()).len() as i32) - (1 as i32) as i32)))))))
}

pub fn over_limit(k: Rc<RefCell<Option<Kind>>>) -> bool {
    (*k.borrow().as_ref().unwrap()) >= Kind(Rc::new(RefCell::new(Some(((*kindIndex.borrow().as_ref().unwrap()).len() as i32) - (1 as i32) as i32))))
}

fn main() {
    __go_init_all();
    let mut k: Rc<RefCell<Option<Kind>>> = Rc::new(RefCell::new(Some(Kind(Rc::new(RefCell::new(Some(3)))))));
    let mut entry = Rc::new(RefCell::new(Some(Entry { kind: k.clone(), ..Default::default() })));
    println!("{}", format!("{}", as_int(Rc::new(RefCell::new(Some((*k.borrow().as_ref().unwrap()).clone()))))));
    println!("{}", format!("{}", as_uint64(Rc::new(RefCell::new(Some((*k.borrow().as_ref().unwrap()).clone()))))));
    println!("{}", format!("{}", field_as_int(Rc::new(RefCell::new(Some((*entry.borrow().as_ref().unwrap()).clone()))))));
    println!("{}", format!("{}", plus_as_int(Rc::new(RefCell::new(Some((*k.borrow().as_ref().unwrap()).clone()))))));
    println!("{}", format!("{}", (*k.borrow().as_ref().unwrap()).method_int()));
    println!("{}", format!("{}", (*k.borrow().as_ref().unwrap()).method_plus()));
    println!("{}", format!("{}", as_int(limit_kind())));
    println!("{}", format!("{}", over_limit(Rc::new(RefCell::new(Some((*k.borrow().as_ref().unwrap()).clone()))))));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
