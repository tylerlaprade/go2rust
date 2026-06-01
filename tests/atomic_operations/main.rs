use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default)]
pub struct namedState(pub Arc<Mutex<Option<u32>>>);

impl Display for namedState {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for namedState {
    fn eq(&self, other: &Self) -> bool {
        self.0.lock().unwrap().as_ref().unwrap() == other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialEq<u32> for namedState {
    fn eq(&self, other: &u32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for namedState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialOrd<u32> for namedState {
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<namedState> for u32 {
    fn eq(&self, other: &namedState) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<namedState> for u32 {
    fn partial_cmp(&self, other: &namedState) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for namedState {
    type Output = namedState;
    fn add(self, other: Self) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u32> for namedState {
    type Output = namedState;
    fn add(self, other: u32) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<namedState> for u32 {
    type Output = namedState;
    fn add(self, other: namedState) -> namedState {
        namedState(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for namedState {
    type Output = namedState;
    fn sub(self, other: Self) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u32> for namedState {
    type Output = namedState;
    fn sub(self, other: u32) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<namedState> for u32 {
    type Output = namedState;
    fn sub(self, other: namedState) -> namedState {
        namedState(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for namedState {
    type Output = namedState;
    fn mul(self, other: Self) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u32> for namedState {
    type Output = namedState;
    fn mul(self, other: u32) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<namedState> for u32 {
    type Output = namedState;
    fn mul(self, other: namedState) -> namedState {
        namedState(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for namedState {
    type Output = namedState;
    fn div(self, other: Self) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u32> for namedState {
    type Output = namedState;
    fn div(self, other: u32) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<namedState> for u32 {
    type Output = namedState;
    fn div(self, other: namedState) -> namedState {
        namedState(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for namedState {
    type Output = namedState;
    fn rem(self, other: Self) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u32> for namedState {
    type Output = namedState;
    fn rem(self, other: u32) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<namedState> for u32 {
    type Output = namedState;
    fn rem(self, other: namedState) -> namedState {
        namedState(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for namedState {
    type Output = namedState;
    fn bitand(self, other: Self) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u32> for namedState {
    type Output = namedState;
    fn bitand(self, other: u32) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<namedState> for u32 {
    type Output = namedState;
    fn bitand(self, other: namedState) -> namedState {
        namedState(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for namedState {
    type Output = namedState;
    fn bitor(self, other: Self) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u32> for namedState {
    type Output = namedState;
    fn bitor(self, other: u32) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<namedState> for u32 {
    type Output = namedState;
    fn bitor(self, other: namedState) -> namedState {
        namedState(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for namedState {
    type Output = namedState;
    fn bitxor(self, other: Self) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u32> for namedState {
    type Output = namedState;
    fn bitxor(self, other: u32) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<namedState> for u32 {
    type Output = namedState;
    fn bitxor(self, other: namedState) -> namedState {
        namedState(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for namedState {
    type Output = namedState;
    fn not(self) -> namedState {
        namedState(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for namedState {
    type Output = namedState;
    fn shl(self, other: namedState) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for namedState {
    type Output = namedState;
    fn shl(self, other: i32) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for namedState {
    type Output = namedState;
    fn shl(self, other: i8) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for namedState {
    type Output = namedState;
    fn shl(self, other: i16) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for namedState {
    type Output = namedState;
    fn shl(self, other: i64) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for namedState {
    type Output = namedState;
    fn shl(self, other: u32) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for namedState {
    type Output = namedState;
    fn shl(self, other: u8) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for namedState {
    type Output = namedState;
    fn shl(self, other: u16) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for namedState {
    type Output = namedState;
    fn shl(self, other: u64) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for namedState {
    type Output = namedState;
    fn shl(self, other: usize) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for namedState {
    type Output = namedState;
    fn shr(self, other: namedState) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for namedState {
    type Output = namedState;
    fn shr(self, other: i32) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for namedState {
    type Output = namedState;
    fn shr(self, other: i8) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for namedState {
    type Output = namedState;
    fn shr(self, other: i16) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for namedState {
    type Output = namedState;
    fn shr(self, other: i64) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for namedState {
    type Output = namedState;
    fn shr(self, other: u32) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for namedState {
    type Output = namedState;
    fn shr(self, other: u8) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for namedState {
    type Output = namedState;
    fn shr(self, other: u16) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for namedState {
    type Output = namedState;
    fn shr(self, other: u64) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for namedState {
    type Output = namedState;
    fn shr(self, other: usize) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for namedState {}

impl Ord for namedState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


fn main() {
    let mut counter: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    Arc::new(Mutex::new(Some({ let __target = counter.clone(); let __delta = 1 as i64; let mut __guard = __target.lock().unwrap(); let __value = __guard.as_mut().unwrap(); *__value += __delta; *__value })));
    Arc::new(Mutex::new(Some({ let __target = counter.clone(); let __delta = 5 as i64; let mut __guard = __target.lock().unwrap(); let __value = __guard.as_mut().unwrap(); *__value += __delta; *__value })));
    let mut value = Arc::new(Mutex::new(Some({ let __target = counter.clone(); let __guard = __target.lock().unwrap(); *__guard.as_ref().unwrap() })));
    println!("{} {}", format!("{}", "Atomic counter:".to_string()), format!("{}", { let __v = (*value.lock().unwrap().as_ref().unwrap()).clone(); __v }));

    let mut state: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));
    let mut next = Arc::new(Mutex::new(Some(namedState(Arc::new(Mutex::new(Some(7 as u32)))))));
    { let __target = state.clone(); let __stored = (*{ let __v = (*next.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u32 as u32; let mut __guard = __target.lock().unwrap(); *__guard.as_mut().unwrap() = __stored; };
    println!("{} {}", format!("{}", "Atomic state:".to_string()), format!("{}", (*Arc::new(Mutex::new(Some({ let __target = state.clone(); let __guard = __target.lock().unwrap(); *__guard.as_ref().unwrap() }))).lock().unwrap().as_ref().unwrap())));
}