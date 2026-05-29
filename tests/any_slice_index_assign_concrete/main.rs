use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};


fn go_any_clone(value: &dyn Any) -> Box<dyn Any> {
    if let Some(v) = value.downcast_ref::<i32>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<i64>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<i8>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<i16>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<u32>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<u64>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<u8>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<u16>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<usize>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<isize>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<f64>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<f32>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<String>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<&'static str>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<bool>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<char>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Pos>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Position>() { return Box::new(v.clone()) as Box<dyn Any>; }

    panic!("go_any_clone: unsupported dynamic type; add typed lowering instead of cloning Box<dyn Any>")
}

#[derive(Debug, Clone, Default)]
pub struct Pos(pub Rc<RefCell<Option<i32>>>);

impl Display for Pos {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().as_ref().unwrap() == other.0.borrow().as_ref().unwrap()
    }
}

impl PartialEq<i32> for Pos {
    fn eq(&self, other: &i32) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl PartialOrd<i32> for Pos {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Pos> for i32 {
    fn eq(&self, other: &Pos) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<Pos> for i32 {
    fn partial_cmp(&self, other: &Pos) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for Pos {
    type Output = Pos;
    fn add(self, other: Self) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for Pos {
    type Output = Pos;
    fn add(self, other: i32) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Pos> for i32 {
    type Output = Pos;
    fn add(self, other: Pos) -> Pos {
        Pos(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Pos {
    type Output = Pos;
    fn sub(self, other: Self) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for Pos {
    type Output = Pos;
    fn sub(self, other: i32) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Pos> for i32 {
    type Output = Pos;
    fn sub(self, other: Pos) -> Pos {
        Pos(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Pos {
    type Output = Pos;
    fn bitand(self, other: Self) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for Pos {
    type Output = Pos;
    fn bitand(self, other: i32) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Pos> for i32 {
    type Output = Pos;
    fn bitand(self, other: Pos) -> Pos {
        Pos(Rc::new(RefCell::new(Some(self & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Pos {
    type Output = Pos;
    fn bitor(self, other: Self) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for Pos {
    type Output = Pos;
    fn bitor(self, other: i32) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Pos> for i32 {
    type Output = Pos;
    fn bitor(self, other: Pos) -> Pos {
        Pos(Rc::new(RefCell::new(Some(self | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Pos {
    type Output = Pos;
    fn bitxor(self, other: Self) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for Pos {
    type Output = Pos;
    fn bitxor(self, other: i32) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Pos> for i32 {
    type Output = Pos;
    fn bitxor(self, other: Pos) -> Pos {
        Pos(Rc::new(RefCell::new(Some(self ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Pos {
    type Output = Pos;
    fn not(self) -> Pos {
        Pos(Rc::new(RefCell::new(Some(!*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Pos {
    type Output = Pos;
    fn shl(self, other: Pos) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for Pos {
    type Output = Pos;
    fn shl(self, other: i32) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for Pos {
    type Output = Pos;
    fn shl(self, other: i8) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for Pos {
    type Output = Pos;
    fn shl(self, other: i16) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for Pos {
    type Output = Pos;
    fn shl(self, other: i64) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for Pos {
    type Output = Pos;
    fn shl(self, other: u32) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for Pos {
    type Output = Pos;
    fn shl(self, other: u8) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for Pos {
    type Output = Pos;
    fn shl(self, other: u16) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for Pos {
    type Output = Pos;
    fn shl(self, other: u64) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for Pos {
    type Output = Pos;
    fn shl(self, other: usize) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for Pos {
    type Output = Pos;
    fn shr(self, other: Pos) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for Pos {
    type Output = Pos;
    fn shr(self, other: i32) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for Pos {
    type Output = Pos;
    fn shr(self, other: i8) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for Pos {
    type Output = Pos;
    fn shr(self, other: i16) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for Pos {
    type Output = Pos;
    fn shr(self, other: i64) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for Pos {
    type Output = Pos;
    fn shr(self, other: u32) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for Pos {
    type Output = Pos;
    fn shr(self, other: u8) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for Pos {
    type Output = Pos;
    fn shr(self, other: u16) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for Pos {
    type Output = Pos;
    fn shr(self, other: u64) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for Pos {
    type Output = Pos;
    fn shr(self, other: usize) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl Eq for Pos {}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug, Clone)]
pub struct Position {
    pub offset: Rc<RefCell<Option<i32>>>,
}

impl Position {
    pub fn __go_value_clone(&self) -> Self {
        Self { offset: { let __guard = self.offset.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Position {
    fn default() -> Self {
        Self { offset: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.offset.borrow().as_ref().unwrap()))
    }
}


pub fn position(p: Rc<RefCell<Option<Pos>>>) -> Rc<RefCell<Option<Position>>> {
    Rc::new(RefCell::new(Some(Position { offset: Rc::new(RefCell::new(Some((*(*p.borrow().as_ref().unwrap()).0.borrow().as_ref().unwrap()) as i32))), ..Default::default() })))
}

pub fn set(args: Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> i32 {
    { let __range_holder = args.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.iter().map(|__e| go_any_clone(__e.as_ref())).collect::<Vec<_>>()).unwrap_or_default(); drop(__range_guard); for (i, mut arg) in __range_values.into_iter().enumerate() {
        {
    let _ts_ref = arg;
    let _ts_is_nil = false;
    let _ts_val: Option<&dyn Any> = Some(_ts_ref.as_ref() as &dyn Any);
    if _ts_val.and_then(|__v| __v.downcast_ref::<Pos>()).is_some() {
        let arg = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<Pos>()).unwrap().clone())));
        (*args.borrow_mut().as_mut().unwrap())[(i) as usize] = Box::new((*position(Rc::new(RefCell::new(Some((*arg.borrow().as_ref().unwrap()).clone())))).borrow().as_ref().unwrap()).clone()) as Box<dyn Any>;;
    }
    }
    } }
    (*args.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32
}

fn main() {
    eprintln!("{}", format!("{}", set(Rc::new(RefCell::new(Some(vec![Box::new(Pos(Rc::new(RefCell::new(Some(42 as i32))))) as Box<dyn Any>, Box::new("a".to_string()) as Box<dyn Any>, Box::new(Pos(Rc::new(RefCell::new(Some(7 as i32))))) as Box<dyn Any>]))))));
}