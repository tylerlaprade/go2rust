#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::cell::{RefCell};
use std::cmp::Ord;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
fn format_map<K: Display + Ord + Clone, V>(map: &Rc<RefCell<Option<BTreeMap<K, Rc<RefCell<Option<V>>>>>>>) -> String
where
    V: Display,
{
    let guard = map.borrow();
    if let Some(ref m) = *guard {
        let mut items: std::vec::Vec<_> = m.iter().collect();
        items.sort_by_key(|(k, _)| (*k).clone());

        let formatted: std::vec::Vec<String> = items
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

pub(crate) const NAMED: i64 = 3;


pub(crate) const OFFSET: i64 = 2;


#[derive(Debug)]
pub struct Index(pub Rc<RefCell<Option<i64>>>);

impl Clone for Index {
    fn clone(&self) -> Self {
        let __value = (*self.0.borrow()).clone();
        Index(Rc::new(RefCell::new(__value)))
    }
}

impl Default for Index {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(Some(Default::default()))))
    }
}

impl Display for Index {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for Index {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i64> for Index {
    fn eq(&self, other: &i64) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Index {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i64> for Index {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Index> for i64 {
    fn eq(&self, other: &Index) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<Index> for i64 {
    fn partial_cmp(&self, other: &Index) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for Index {
    type Output = Index;
    fn add(self, other: Self) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i64> for Index {
    type Output = Index;
    fn add(self, other: i64) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Index> for i64 {
    type Output = Index;
    fn add(self, other: Index) -> Index {
        Index(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Index {
    type Output = Index;
    fn sub(self, other: Self) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i64> for Index {
    type Output = Index;
    fn sub(self, other: i64) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Index> for i64 {
    type Output = Index;
    fn sub(self, other: Index) -> Index {
        Index(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Index {
    type Output = Index;
    fn mul(self, other: Self) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i64> for Index {
    type Output = Index;
    fn mul(self, other: i64) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Index> for i64 {
    type Output = Index;
    fn mul(self, other: Index) -> Index {
        Index(Rc::new(RefCell::new(Some(self * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Index {
    type Output = Index;
    fn div(self, other: Self) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i64> for Index {
    type Output = Index;
    fn div(self, other: i64) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Index> for i64 {
    type Output = Index;
    fn div(self, other: Index) -> Index {
        Index(Rc::new(RefCell::new(Some(self / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for Index {
    type Output = Index;
    fn neg(self) -> Index {
        Index(Rc::new(RefCell::new(Some(-*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for Index {
    type Output = Index;
    fn rem(self, other: Self) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i64> for Index {
    type Output = Index;
    fn rem(self, other: i64) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<Index> for i64 {
    type Output = Index;
    fn rem(self, other: Index) -> Index {
        Index(Rc::new(RefCell::new(Some(self % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Index {
    type Output = Index;
    fn bitand(self, other: Self) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i64> for Index {
    type Output = Index;
    fn bitand(self, other: i64) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Index> for i64 {
    type Output = Index;
    fn bitand(self, other: Index) -> Index {
        Index(Rc::new(RefCell::new(Some(self & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Index {
    type Output = Index;
    fn bitor(self, other: Self) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i64> for Index {
    type Output = Index;
    fn bitor(self, other: i64) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Index> for i64 {
    type Output = Index;
    fn bitor(self, other: Index) -> Index {
        Index(Rc::new(RefCell::new(Some(self | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Index {
    type Output = Index;
    fn bitxor(self, other: Self) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i64> for Index {
    type Output = Index;
    fn bitxor(self, other: i64) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Index> for i64 {
    type Output = Index;
    fn bitxor(self, other: Index) -> Index {
        Index(Rc::new(RefCell::new(Some(self ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Index {
    type Output = Index;
    fn not(self) -> Index {
        Index(Rc::new(RefCell::new(Some(!*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Index {
    type Output = Index;
    fn shl(self, other: Index) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for Index {
    type Output = Index;
    fn shl(self, other: i32) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for Index {
    type Output = Index;
    fn shl(self, other: i8) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for Index {
    type Output = Index;
    fn shl(self, other: i16) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for Index {
    type Output = Index;
    fn shl(self, other: i64) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for Index {
    type Output = Index;
    fn shl(self, other: u32) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for Index {
    type Output = Index;
    fn shl(self, other: u8) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for Index {
    type Output = Index;
    fn shl(self, other: u16) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for Index {
    type Output = Index;
    fn shl(self, other: u64) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for Index {
    type Output = Index;
    fn shl(self, other: usize) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for Index {
    type Output = Index;
    fn shr(self, other: Index) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for Index {
    type Output = Index;
    fn shr(self, other: i32) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for Index {
    type Output = Index;
    fn shr(self, other: i8) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for Index {
    type Output = Index;
    fn shr(self, other: i16) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for Index {
    type Output = Index;
    fn shr(self, other: i64) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for Index {
    type Output = Index;
    fn shr(self, other: u32) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for Index {
    type Output = Index;
    fn shr(self, other: u8) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for Index {
    type Output = Index;
    fn shr(self, other: u16) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for Index {
    type Output = Index;
    fn shr(self, other: u64) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for Index {
    type Output = Index;
    fn shr(self, other: usize) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl Eq for Index {}

impl Ord for Index {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug, Clone, Default)]
pub struct Indexes(pub Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<Index>>>>>>>);

impl Display for Indexes {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_map(&self.0))
    }
}


fn main() {
    ::example_com_mapliteral_dep::__go_init_all();

    let mut local = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Index>>>>::from([("literal".to_string(), Rc::new(RefCell::new(Some(Index(Rc::new(RefCell::new(Some(2 as i64)))))))), ("named".to_string(), Rc::new(RefCell::new(Some(Index(Rc::new(RefCell::new(Some(NAMED as i64))))))))]))));
    println!("{} {}", format!("{}", { let __map_holder = local.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"literal".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| Index(Rc::new(RefCell::new(Some(0))))) }), format!("{}", { let __map_holder = local.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"named".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| Index(Rc::new(RefCell::new(Some(0))))) }));

    let mut indexes = Rc::new(RefCell::new(Some(Indexes(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Index>>>>::from([("computed".to_string(), Rc::new(RefCell::new(Some(Index(Rc::new(RefCell::new(Some((2 + 3) as i64))))))))]))))))));
    println!("{}", format!("{}", { let __map_holder = { let __named_map = (*indexes.borrow().as_ref().unwrap()).0.clone(); __named_map }.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"computed".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| Index(Rc::new(RefCell::new(Some(0))))) }));

    let mut levels = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<example_com_mapliteral_dep::r#mod::Level>>>>::from([("literal".to_string(), Rc::new(RefCell::new(Some(example_com_mapliteral_dep::r#mod::Level(Rc::new(RefCell::new(Some(4 as i64)))))))), ("named".to_string(), Rc::new(RefCell::new(Some(example_com_mapliteral_dep::r#mod::Level(Rc::new(RefCell::new(Some(example_com_mapliteral_dep::HIGH as i64)))))))), ("computed".to_string(), Rc::new(RefCell::new(Some(example_com_mapliteral_dep::r#mod::Level(Rc::new(RefCell::new(Some((1 + 2) as i64)))))))), ("local".to_string(), Rc::new(RefCell::new(Some(example_com_mapliteral_dep::r#mod::Level(Rc::new(RefCell::new(Some(OFFSET as i64)))))))), ("imported".to_string(), Rc::new(RefCell::new(Some(example_com_mapliteral_dep::r#mod::Level(Rc::new(RefCell::new(Some(example_com_mapliteral_dep::OFFSET as i64))))))))]))));
    println!("{} {} {} {} {}", format!("{}", { let __map_holder = levels.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"literal".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| example_com_mapliteral_dep::r#mod::Level(Rc::new(RefCell::new(Some(0))))) }), format!("{}", { let __map_holder = levels.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"named".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| example_com_mapliteral_dep::r#mod::Level(Rc::new(RefCell::new(Some(0))))) }), format!("{}", { let __map_holder = levels.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"computed".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| example_com_mapliteral_dep::r#mod::Level(Rc::new(RefCell::new(Some(0))))) }), format!("{}", { let __map_holder = levels.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"local".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| example_com_mapliteral_dep::r#mod::Level(Rc::new(RefCell::new(Some(0))))) }), format!("{}", { let __map_holder = levels.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"imported".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| example_com_mapliteral_dep::r#mod::Level(Rc::new(RefCell::new(Some(0))))) }));
}